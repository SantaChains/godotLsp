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
use gdscript_syntax::ast::File;
use gdscript_syntax::semantic::ProjectContext;
use gdscript_syntax::parse;

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

        let autoloads = parse_autoload(root.join("project.godot"));

        Ok(Project { engine, scripts, class_names, autoloads })
    }

    /// class_name → 对应脚本。
    pub fn class_name_script(&self, name: &str) -> Option<&Script> {
        self.class_names.get(name).map(|&i| &self.scripts[i])
    }

    /// autoload 单化名 → 类型描述。
    pub fn autoload_type(&self, name: &str) -> Option<&str> {
        self.autoloads.get(name).map(|s| s.as_str())
    }

    /// preload("res://x.gd") → 对应脚本。
    pub fn resolve_preload(&self, res_path: &str) -> Option<&Script> {
        let key = res_path.strip_prefix("res://").unwrap_or(res_path);
        self.scripts.iter().find(|s| s.path == key)
    }
}

impl ProjectContext for Project {
    fn class_name_type(&self, name: &str) -> Option<String> {
        self.class_names.contains_key(name).then(|| name.to_string())
    }

    fn autoload_type(&self, name: &str) -> Option<String> {
        self.autoloads.get(name).cloned()
    }

    fn preload_type(&self, res_path: &str) -> Option<String> {
        let script = self.resolve_preload(res_path)?;
        script.ast.class_name.as_ref().map(|cn| cn.name.clone())
    }
}

fn walk_gd(dir: &Path, f: &mut dyn FnMut(&Path)) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == ".godot" || name.starts_with('.') {
                continue;
            }
            walk_gd(&p, f);
        } else if p.extension().map(|x| x == "gd").unwrap_or(false) {
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
