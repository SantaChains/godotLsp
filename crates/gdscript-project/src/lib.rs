//! `gdscript-project` —— 项目级符号图（R3）。
//!
//! 扫描工作区全部 `.gd`，建立跨文件解析所需的索引：
//! - `class_name X` 注册：脚本声明的全局类型名 → 文件；
//! - `project.godot` 的 `[autoload]`：全局单化名 → 脚本/引擎类型；
//! - 文件清单（供后续增量与诊断）。
//!
//! 用法：
//! ```no_run
//! use std::path::Path;
//! let proj = gdscript_project::Project::open(Path::new("."), /* engine */ None).unwrap();
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use gdscript_engine::EngineApi;
use gdscript_syntax::ast::{Expr, File, Stmt};
use gdscript_syntax::semantic::{ProjectContext, ScriptMember};
use gdscript_syntax::parse;

pub mod tscn;

use tscn::SceneFile;

/// 一个已解析的项目脚本。
#[derive(Clone, Debug)]
pub struct Script {
    /// res:// 相对路径（如 "res://player.gd"）。
    pub path: String,
    pub ast: File,
}

/// 项目级符号图。
pub struct Project {
    pub engine: EngineApi,
    pub scripts: Vec<Script>,
    /// class_name → scripts 下标。
    class_names: HashMap<String, usize>,
    /// autoload 单化名 → 类型（脚本路径或引擎类名）。
    autoloads: HashMap<String, String>,
    /// `[input]` 节动作名（C8 参数选项面；排序去重）。
    input_actions: Vec<String>,
    /// tscn/tres 解析结果（P2.7）。
    pub scenes: Vec<SceneFile>,
    /// 自嵌 uid → res:// 路径（tscn/tres 头部 uid）。
    uid_paths: HashMap<String, String>,
    /// 命名枚举全局索引：枚举名 → 条目名列表（跨文件 `Class.EnumName.KEY` 链解析）。
    enum_values: HashMap<String, Vec<String>>,
    /// class_name → 直接子类键列表（家族成员查询：基类型变量可持子类实例）。
    subclass_of: HashMap<String, Vec<String>>,
}

impl Project {
    /// 扫描 root 目录下全部 .gd（递归，跳过 .godot 目录），解析并建索引。
    /// engine_json 为 extension_api.json 文本；None 则用空模型。
    pub fn open(root: &Path, engine_json: Option<&str>) -> Result<Project, std::io::Error> {
        let engine = match engine_json {
            Some(j) => EngineApi::from_json(j).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?,
            None => EngineApi::default(),
        };

        let mut scripts = Vec::new();
        let mut class_names = HashMap::new();
        walk_gd(root, &mut |p| {
            let raw = match std::fs::read_to_string(p) {
                Ok(s) => s,
                Err(_) => return,
            };
            // 对齐 corpus 约定：剥离 Godot 上游光标标记 U+27A1。
            let src: String = raw.chars().filter(|&c| c != '\u{27a1}').collect();
            let (ast, _errs) = parse(&src);
            let rel = path_to_res(p, root);
            if let Some(cn) = &ast.class_name {
                class_names.insert(cn.name.clone(), scripts.len());
            }
            scripts.push(Script { path: rel, ast });
        });

        // 命名枚举全局索引（枚举名在 class_name 作用域下近似唯一，平铺即可）。
        let mut enum_values: HashMap<String, Vec<String>> = HashMap::new();
        // 子类反查索引：extends 基 class_name → 子类键（家族成员查询用）。
        let mut subclass_of: HashMap<String, Vec<String>> = HashMap::new();
        for s in &scripts {
            for st in &s.ast.body {
                if let Stmt::Enum(e) = st
                    && let Some(n) = &e.name
                {
                    enum_values
                        .entry(n.name.clone())
                        .or_default()
                        .extend(e.entries.iter().map(|(id, _)| id.name.clone()));
                }
            }
            if let Some(base) = extends_root(&s.ast.extends)
                && class_names.contains_key(&base)
            {
                subclass_of.entry(base).or_default().push(Self::script_key(s));
            }
        }

        let autoloads = parse_autoload(root.join("project.godot"));
        let mut input_actions = parse_input_actions(root.join("project.godot"));
        input_actions.sort_unstable();
        input_actions.dedup();

        // P2.7：扫描 tscn/tres，建场景索引与自嵌 uid 映射。
        let mut scenes = Vec::new();
        let mut uid_paths = HashMap::new();
        walk_ext(root, &["tscn", "tres"], &mut |p| {
            let Ok(text) = std::fs::read_to_string(p) else { return };
            let rel = path_to_res(p, root);
            let sf = tscn::parse(&rel, &text);
            if let Some((u, p)) = sf.uid_entry() {
                uid_paths.insert(u, p);
            }
            scenes.push(sf);
        });

        // Godot 4.4+ .uid sidecar（`Keychain.gd.uid` 内容单行 uid://xxx）——
        // autoload uid 注册形式经此回查脚本路径。
        walk_ext(root, &["uid"], &mut |p| {
            let Ok(text) = std::fs::read_to_string(p) else { return };
            if let Some(u) = text.trim().strip_prefix("uid://")
                && let Some(res) = path_to_res(p, root).strip_suffix(".uid")
            {
                uid_paths.insert(u.to_string(), res.to_string());
            }
        });

        Ok(Project { engine, scripts, class_names, autoloads, input_actions, scenes, uid_paths, enum_values, subclass_of })
    }

    /// class_name → 对应脚本。
    pub fn class_name_script(&self, name: &str) -> Option<&Script> {
        self.class_names.get(name).map(|&i| &self.scripts[i])
    }

    /// autoload 单化名 → 类型（`project.godot` `[autoload]` 节，`*` 前缀已剥）。
    /// 补全全局面/悬停解析共用；HashMap 迭代序不稳定，调用方需自行排序。
    pub fn autoloads(&self) -> impl Iterator<Item = (&str, &str)> {
        self.autoloads.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// 输入动作名（`project.godot` `[input]` 节键名，排序去重）。
    /// C8：`event.is_action_pressed(` 等 InputEvent action 方法实参面消费
    ///（引擎 InputMap::get_argument_options 同源，锚点 core/input/input_map.cpp:90）。
    pub fn input_actions(&self) -> &[String] {
        &self.input_actions
    }

    /// preload("res://x.gd") → 对应脚本。
    pub fn resolve_preload(&self, res_path: &str) -> Option<&Script> {
        let key = res_path.strip_prefix("res://").unwrap_or(res_path);
        self.scripts.iter().find(|s| s.path == key)
    }

    /// 自嵌 uid（剥前缀）→ res:// 路径。
    pub fn uid_to_path(&self, uid: &str) -> Option<&str> {
        self.uid_paths.get(uid).map(|s| s.as_str())
    }

    /// res:// 路径 → 场景/资源文件。
    pub fn scene(&self, res_path: &str) -> Option<&SceneFile> {
        let key = res_path.strip_prefix("res://").unwrap_or(res_path);
        self.scenes.iter().find(|s| s.path == key)
    }

    /// 合并另一个项目的脚本/场景/uid/类名单例索引（多根工作区、子目录嵌套项目用）。
    /// 后者对同名 uid/单例覆盖前者。
    pub fn merge(&mut self, other: Project) {
        let base = self.scripts.len();
        for (i, s) in other.scripts.iter().enumerate() {
            if let Some(cn) = &s.ast.class_name {
                // 合并前 self.scripts.len() 恒为 base，other 脚本按原序追加，
                // 故其在合并后向量中的下标 = base + i（i 即 other 内下标）。
                self.class_names.insert(cn.name.clone(), base + i);
            }
        }
        self.scripts.extend(other.scripts);
        self.scenes.extend(other.scenes);
        self.uid_paths.extend(other.uid_paths);
        self.autoloads.extend(other.autoloads);
        self.input_actions.extend(other.input_actions);
        self.input_actions.sort_unstable();
        self.input_actions.dedup();
        for (n, vals) in other.enum_values {
            self.enum_values.entry(n).or_default().extend(vals);
        }
        for (b, keys) in other.subclass_of {
            self.subclass_of.entry(b).or_default().extend(keys);
        }
    }
}

impl Project {
    /// 脚本代表类型键：class_name 或合成 `script:<相对路径>`。
    fn script_key(script: &Script) -> String {
        match &script.ast.class_name {
            Some(cn) => cn.name.clone(),
            None => format!("script:{}", script.path),
        }
    }

    /// 按类型键（class_name 或 script:<path>）取脚本。
    fn script_by_key(&self, key: &str) -> Option<&Script> {
        if let Some(&i) = self.class_names.get(key) {
            return Some(&self.scripts[i]);
        }
        let rel = key.strip_prefix("script:")?;
        self.scripts.iter().find(|s| s.path == rel)
    }

    /// 沿工程脚本 extends 链查成员（SymmetryGuide → Guide → 引擎类止步——
    /// 引擎类成员由语义层走引擎模型）。真实工程里子类成员多半在父脚本上，
    /// 只查单文件会漏（Pixelorama SymmetryGuide.add_point 实测）。
    fn walk_member_kind(&self, key: &str, name: &str, depth: u8) -> Option<ScriptMember> {
        if depth > 16 {
            return None; // 继承环防御
        }
        let script = self.script_by_key(key)?;
        for st in &script.ast.body {
            match st {
                Stmt::Func(f) if f.name.name == name => return Some(ScriptMember::Method),
                Stmt::Var(v) if v.name.name == name => return Some(ScriptMember::Property),
                Stmt::Const(c) if c.name.name == name => return Some(ScriptMember::Const),
                Stmt::Signal(s) if s.name.name == name => return Some(ScriptMember::Signal),
                // 命名枚举名按常量成员暴露（TileSetPanel.TileEditingMode → int）。
                Stmt::Enum(e) if e.name.as_ref().is_some_and(|n| n.name == name) => {
                    return Some(ScriptMember::Const)
                }
                // 匿名枚举条目 = 该脚本的文件级 int 常量（Brushes.FILE 跨文件裸用）。
                Stmt::Enum(e)
                    if e.name.is_none()
                        && e.entries.iter().any(|(id, _)| id.name == name) =>
                {
                    return Some(ScriptMember::Const)
                }
                // 内部类名作常量命名空间（Drawer.ColorOp → 类引用）。
                Stmt::Class(c) if c.name.name == name => return Some(ScriptMember::Const),
                _ => {}
            }
        }
        let base = extends_root(&script.ast.extends)?;
        if self.class_names.contains_key(&base) {
            return self.walk_member_kind(&base, name, depth + 1);
        }
        None
    }

    fn walk_member_type(&self, key: &str, name: &str, depth: u8) -> Option<String> {
        if depth > 16 {
            return None;
        }
        let script = self.script_by_key(key)?;
        for st in &script.ast.body {
            let ty = match st {
                Stmt::Func(f) if f.name.name == name => &f.ret_ty,
                Stmt::Var(v) if v.name.name == name => &v.ty,
                Stmt::Const(c) if c.name.name == name => &c.ty,
                Stmt::Enum(e) if e.name.as_ref().is_some_and(|n| n.name == name) => {
                    // 命名枚举是常量命名空间：`Class.EnumName.KEY` 需要枚举名作类型续链。
                    return e.name.as_ref().map(|n| n.name.clone())
                }
                // 匿名枚举条目类型为 int。
                Stmt::Enum(e)
                    if e.name.is_none() && e.entries.iter().any(|(id, _)| id.name == name) =>
                {
                    return Some("int".into())
                }
                // 内部类引用：返回类名自身（ColorOp.new() → 类类型）。
                Stmt::Class(c) if c.name.name == name => return Some(c.name.name.clone()),
                _ => continue,
            };
            return type_root(ty);
        }
        let base = extends_root(&script.ast.extends)?;
        if self.class_names.contains_key(&base) {
            return self.walk_member_type(&base, name, depth + 1);
        }
        None
    }
}

impl ProjectContext for Project {
    fn class_name_type(&self, name: &str) -> Option<String> {
        self.class_names.contains_key(name).then(|| name.to_string())
    }

    fn autoload_type(&self, name: &str) -> Option<String> {
        let raw = self.autoloads.get(name)?;
        // `*res://x.gd` → 解析脚本为 class_name 或合成键；裸类名 → class_name/引擎类。
        if let Some(_rel) = raw.strip_prefix("res://") {
            let script = self.resolve_preload(raw)?;
            return Some(Self::script_key(script));
        }
        // Godot 4.4+ uid 注册形式：`Keychain="*uid://xxx"` → uid_paths 回查 res 路径。
        if let Some(uid) = raw.strip_prefix("uid://") {
            let rel = self.uid_paths.get(uid)?;
            let script = self.scripts.iter().find(|s| s.path == rel.as_str())?;
            return Some(Self::script_key(script));
        }
        if self.class_names.contains_key(raw) || self.engine.class_by_name(raw).is_some() {
            return Some(raw.clone());
        }
        None
    }

    fn preload_type(&self, res_path: &str) -> Option<String> {
        let script = self.resolve_preload(res_path)?;
        // 有 class_name 用全局类型名；无则合成稳定脚本键，供后续脚本成员模型挂载。
        Some(Self::script_key(script))
    }

    fn script_type_exists(&self, key: &str) -> bool {
        self.script_by_key(key).is_some()
    }

    fn script_member_kind(&self, key: &str, name: &str) -> Option<ScriptMember> {
        self.walk_member_kind(key, name, 0)
    }

    /// G4.6：返回 var/const 注解类型或 func 返回类型的根类名；无注解 None。
    fn script_member_type(&self, key: &str, name: &str) -> Option<String> {
        self.walk_member_type(key, name, 0)
    }

    fn script_extends_base(&self, key: &str) -> Option<String> {
        let script = self.script_by_key(key)?;
        extends_root(&script.ast.extends)
    }

    fn script_enum_entry(&self, enum_name: &str, key: &str) -> bool {
        self.enum_values
            .get(enum_name)
            .is_some_and(|vals| vals.iter().any(|v| v == key))
    }

    fn script_family_member(&self, base: &str, name: &str) -> bool {
        if self.script_member_kind(base, name).is_some() {
            return true;
        }
        // 传递子类闭包 BFS（继承环由 seen 防御）。
        let mut stack: Vec<&str> = vec![base];
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        while let Some(b) = stack.pop() {
            if !seen.insert(b) {
                continue;
            }
            if let Some(kids) = self.subclass_of.get(b) {
                for k in kids {
                    if self.walk_member_kind(k, name, 0).is_some() {
                        return true;
                    }
                    stack.push(k.as_str());
                }
            }
        }
        false
    }
}

/// 从类型表达式提取根类名：`Node2D`、`Array[int]` 取根 Ident；
/// `preload(...)` / 字符串路径 / 无注解返回 None。
fn type_root(e: &Option<Expr>) -> Option<String> {
    match e {
        Some(Expr::Ident(id)) => Some(id.name.clone()),
        Some(Expr::Generic(base, _, _)) => type_root(&Some((**base).clone())),
        _ => None,
    }
}

/// 从 `extends` 表达式提取根类名（G4.5）。
fn extends_root(ext: &Option<Expr>) -> Option<String> {
    type_root(ext)
}

fn walk_gd(dir: &Path, f: &mut dyn FnMut(&Path)) {    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == ".godot" || name.starts_with('.') {
                continue;
            }
            walk_gd(&p, f);
        } else if p
            .extension()
            .and_then(|x| x.to_str())
            .is_some_and(|x| x.eq_ignore_ascii_case("gd"))
        {
            f(&p);
        }
    }
}

/// 按扩展名白名单递归遍历（跳过 .godot/.git 等隐藏目录）。
fn walk_ext(dir: &Path, exts: &[&str], f: &mut dyn FnMut(&Path)) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == ".godot" || name.starts_with('.') {
                continue;
            }
            walk_ext(&p, exts, f);
        } else if p
            .extension()
            .and_then(|x| x.to_str())
            .is_some_and(|x| exts.iter().any(|e| e.eq_ignore_ascii_case(x)))
        {
            f(&p);
        }
    }
}

fn path_to_res(p: &Path, root: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("/")
}

/// 解析 project.godot 的 [autoload] 段：`Name="*res://path.gd"` 或 `Name="*ClassName"`。
fn parse_autoload(project_godot: PathBuf) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let Ok(text) = std::fs::read_to_string(project_godot) else { return map };
    let mut in_autoload = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') && t.ends_with(']') {
            in_autoload = t == "[autoload]";
            continue;
        }
        if !in_autoload || t.is_empty() {
            continue;
        }
        // 形如:  GameManager="*res://game_manager.gd"
        let Some((name, value)) = t.split_once('=') else { continue };
        let value = value.trim().trim_matches('"');
        let value = value.strip_prefix('*').unwrap_or(value);
        map.insert(name.trim().to_string(), value.to_string());
    }
    map
}

/// 解析 project.godot 的 [input] 段动作名（C8 输入动画面数据源）。
/// 形如：
/// ```text
/// [input]
///
/// test_input_action={
/// "deadzone": 0.2,
/// "events": []
/// }
/// ```
/// 键行 = 标识符 + `=`；值跨行时内部行（`"deadzone": …`）不含 `=` 或键段
/// 非标识符，天然被过滤。单行形式 `name={...}` 同样命中。
fn parse_input_actions(project_godot: PathBuf) -> Vec<String> {
    let mut names = Vec::new();
    let Ok(text) = std::fs::read_to_string(project_godot) else { return names };
    let mut in_input = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') && t.ends_with(']') {
            in_input = t == "[input]";
            continue;
        }
        if !in_input || t.is_empty() {
            continue;
        }
        let Some((key, _)) = t.split_once('=') else { continue };
        let key = key.trim();
        if !key.is_empty()
            && key.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
            && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            names.push(key.to_string());
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_project(dir: &std::path::Path, text: &str) -> std::path::PathBuf {
        let _ = std::fs::remove_dir_all(dir);
        std::fs::create_dir_all(dir).unwrap();
        let pg = dir.join("project.godot");
        std::fs::write(&pg, text).unwrap();
        pg
    }

    #[test]
    fn parse_input_actions_multiline_and_inline() {
        let pg = write_project(
            &std::env::temp_dir().join("godotlsp_input_actions_test"),
            "config_version=5\n\n[application]\nconfig/name=\"t\"\n\n[input]\n\nmove_left={\n\"deadzone\": 0.2,\n\"events\": []\n}\njump={\"deadzone\":0.5,\"events\":[]}\n\n[autoload]\nA=\"*res://a.gd\"\n",
        );
        let actions = parse_input_actions(pg);
        // 文档序（排序去重发生在 Project::open 内）。
        assert_eq!(actions, vec!["move_left".to_string(), "jump".to_string()]);
        // [autoload] 段的 `=` 不得混入。
        assert!(!actions.iter().any(|a| a.contains("res://")));
    }

    #[test]
    fn parse_input_actions_absent_section() {
        let pg = write_project(
            &std::env::temp_dir().join("godotlsp_input_actions_test2"),
            "config_version=5\n",
        );
        assert!(parse_input_actions(pg).is_empty());
        // 文件缺失同样返回空（不 panic）。
        assert!(parse_input_actions(std::env::temp_dir().join("godotlsp_nope.godot")).is_empty());
    }
}
