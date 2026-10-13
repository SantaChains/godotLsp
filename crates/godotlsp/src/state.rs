//! 服务器状态:分析宿主、文件注册表、内存文档、项目加载。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use gdscript_base::FileId;
use gdscript_ide::{AnalysisHost, Change};
use lsp_types::Uri;

use crate::convert::LineIndex;
use crate::text::apply_content_changes;
use crate::uri::{path_to_uri, uri_to_path};

/// 一个已注册文件的文本及其行索引。
#[derive(Debug, Clone)]
pub struct Doc {
    pub text: String,
    pub index: LineIndex,
    /// 编辑器侧的文档版本;磁盘注册的文件为 `None`。随 `publishDiagnostics` 回传,
    /// 让客户端丢弃迟到的诊断。
    pub version: Option<i32>,
}

/// 客户端配置节名,与编辑器侧 `godotlsp.*` 注册项一一对应。
pub const SECTION: &str = "godotlsp";

/// 全量服务器状态。单线程:请求在主循环线程上顺序处理,因此分析快照不会在处理中途失效。
pub struct State {
    pub host: AnalysisHost,
    pub root: Option<PathBuf>,
    /// 是否显示 inlay 类型提示(由客户端配置驱动)。
    pub inlay_hints: bool,
    /// 客户端要求忽略的诊断码(`UNUSED_VARIABLE` 等 Godot 编辑器同款码)。
    pub diag_ignore: Vec<String>,
    uris: HashMap<FileId, String>,
    ids: HashMap<String, FileId>,
    docs: HashMap<FileId, Doc>,
    next_id: u32,
}

const SKIP_DIRS: &[&str] = &[".git", ".godot", "target", "node_modules"];
const EXTS: &[&str] = &["gd", "tscn", "tres"];
const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// 无项目的空状态。
    #[must_use]
    pub fn new() -> Self {
        Self {
            host: AnalysisHost::new(),
            root: None,
            inlay_hints: true,
            diag_ignore: Vec::new(),
            uris: HashMap::new(),
            ids: HashMap::new(),
            docs: HashMap::new(),
            next_id: 0,
        }
    }

    /// `file` 对应的已注册文档。
    #[must_use]
    pub fn doc(&self, file: FileId) -> Option<&Doc> {
        self.docs.get(&file)
    }

    /// `uri` 对应的 `FileId`。
    #[must_use]
    pub fn id_of(&self, uri: &str) -> Option<FileId> {
        self.ids.get(uri).copied()
    }

    /// `file` 对应的注册 URI。
    #[must_use]
    pub fn uri_of(&self, file: FileId) -> Option<&str> {
        self.uris.get(&file).map(String::as_str)
    }

    /// POD→LSP 转换所用的解析回调:为可能已关闭的文件提供 `(uri, text, _len)`。
    pub fn resolver(&self) -> impl Fn(FileId) -> Option<(String, String, u32)> + '_ {
        move |file| {
            let uri = self.uris.get(&file)?;
            let doc = self.docs.get(&file)?;
            Some((uri.clone(), doc.text.to_string(), doc.index.len()))
        }
    }

    fn res_path(&self, path: &Path) -> Option<String> {
        let root = self.root.as_ref()?;
        let rel = path.strip_prefix(root).ok()?;
        let rel = rel.to_string_lossy().replace('\\', "/");
        Some(format!("res://{rel}"))
    }

    fn alloc(&mut self, uri: &str) -> FileId {
        let id = FileId(self.next_id);
        self.next_id += 1;
        self.uris.insert(id, uri.to_string());
        self.ids.insert(uri.to_string(), id);
        id
    }

    /// 把文件文本写入分析世界(新增或替换)。
    pub fn inject(&mut self, uri: &str, res: Option<String>, text: String, version: Option<i32>) -> FileId {
        let file = self.ids.get(uri).copied().unwrap_or_else(|| self.alloc(uri));
        let index = LineIndex::new(&text);
        let mut change = Change::new();
        change.change_file(file, text.as_str());
        if let Some(res) = res {
            change.set_file_path(file, res);
        }
        self.docs.insert(file, Doc { text, index, version });
        self.host.apply_change(change);
        file
    }

    /// `didChange` 增量编辑应用到已注册文档并同步分析世界。编辑时不重发
    /// `res://` 路径(salsa 要求:重发会无谓失效路径注册表)。
    pub fn apply_changes(
        &mut self,
        file: FileId,
        changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
        version: Option<i32>,
    ) -> bool {
        let Some(doc) = self.docs.get_mut(&file) else {
            return false;
        };
        apply_content_changes(&mut doc.text, &mut doc.index, changes);
        doc.version = version;
        let mut change = Change::new();
        change.change_file(file, doc.text.as_str());
        self.host.apply_change(change);
        true
    }

    /// `didOpen` — 以编辑器文本在此 URI 下注册。
    pub fn open(&mut self, uri: &str, text: String, version: Option<i32>) -> FileId {
        let path = uri.parse::<Uri>().ok().and_then(|u| uri_to_path(&u));
        let res = path.as_ref().and_then(|p| self.res_path(p));
        self.inject(uri, res, text, version)
    }

    /// 被监视文件在磁盘上变化:重新加载;若已删除则移除。`project.godot` 走项目配置而非普通文件。
    pub fn watched(&mut self, uri: &str) {
        let Some(path) = uri.parse::<Uri>().ok().and_then(|u| uri_to_path(&u)) else {
            return;
        };
        if let Some(root) = self.root.clone()
            && path.parent() == Some(root.as_path())
            && path.file_name().is_some_and(|n| n == "project.godot")
        {
            if let Ok(text) = std::fs::read_to_string(&path) {
                self.set_project_config(text);
            }
            return;
        }
        match std::fs::read_to_string(&path) {
            Ok(text) => {
                let res = self.res_path(&path);
                self.inject(uri, res, text, None);
            }
            Err(_) => self.remove(uri),
        }
    }

    /// 从注册表与分析世界中移除文件。
    pub fn remove(&mut self, uri: &str) {
        if let Some(file) = self.ids.remove(uri) {
            self.uris.remove(&file);
            self.docs.remove(&file);
            let mut change = Change::new();
            change.remove_file(file);
            self.host.apply_change(change);
        }
    }

    /// 设置(或替换)`project.godot` 配置文本。
    pub fn set_project_config(&mut self, text: String) {
        let mut change = Change::new();
        change.set_project_config(text);
        self.host.apply_change(change);
    }

    /// 遍历 `root`,注册所有 `.gd`/`.tscn`/`.tres` 以及 `project.godot`。
    /// 文件集覆盖整个项目,因此基于缺失的跨文件诊断可用。
    pub fn load_project(&mut self, root: PathBuf) {
        self.root = Some(root.clone());
        let mut paths = Vec::new();
        walk(&root, &mut paths);
        paths.sort();
        let mut change = Change::new();
        for path in &paths {
            let Ok(text) = std::fs::read_to_string(path) else {
                continue;
            };
            let file = self.alloc(&path_to_uri(path).map(|u| u.as_str().to_owned()).unwrap_or_default());
            let index = LineIndex::new(&text);
            self.docs.insert(file, Doc { text, index, version: None });
            change.change_file(file, self.docs[&file].text.clone());
            if let Some(res) = self.res_path(path) {
                change.set_file_path(file, res);
            }
        }
        if let Ok(config) = std::fs::read_to_string(root.join("project.godot")) {
            change.set_project_config(config);
        }
        change.set_workspace_complete(true);
        self.host.apply_change(change);
        eprintln!("[godotlsp] 已加载 {} 个项目文件", paths.len());
    }
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            if path
                .file_name()
                .is_some_and(|n| !SKIP_DIRS.contains(&n.to_string_lossy().as_ref()))
            {
                walk(&path, out);
            }
        } else if path
            .extension()
            .is_some_and(|e| EXTS.contains(&e.to_string_lossy().as_ref()))
            && entry.metadata().map(|m| m.len() <= MAX_FILE_BYTES).unwrap_or(false)
        {
            out.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project() -> State {
        let mut s = State::new();
        s.root = Some(PathBuf::from("/proj"));
        s.inject(
            "file:///proj/markup.gd",
            Some("res://markup.gd".into()),
            "class_name Markup\nfunc parse() -> int:\n\treturn 1\n".into(),
            None,
        );
        s.inject(
            "file:///proj/main.gd",
            Some("res://main.gd".into()),
            "const M = preload(\"res://markup.gd\")\nfunc go():\n\tvar n := M.new().parse()\n\treturn n\n".into(),
            None,
        );
        s
    }

    #[test]
    fn engine_model_is_bundled() {
        assert!(
            !gdscript_api::godot_version().is_empty(),
            "内嵌引擎模型缺失: {}",
            gdscript_api::godot_version()
        );
    }

    #[test]
    fn valid_cross_file_code_has_no_diagnostics() {
        let s = project();
        let main = s.id_of("file:///proj/main.gd").unwrap();
        assert!(s.host.analysis().diagnostics(main).unwrap().is_empty());
    }

    #[test]
    fn syntax_error_is_reported() {
        let mut s = State::new();
        let f = s.inject(
            "file:///p/bad.gd",
            Some("res://bad.gd".into()),
            "func f(:\n\tpass\n".into(),
            None,
        );
        let diags = s.host.analysis().diagnostics(f).unwrap();
        assert!(!diags.is_empty(), "应产生解析诊断");
    }

    #[test]
    fn preload_binding_is_inferred() {
        let s = project();
        let main = s.id_of("file:///proj/main.gd").unwrap();
        let hints = s.host.analysis().inlay_hints(main).unwrap();
        assert!(
            hints.iter().any(|h| h.label.contains("int")),
            "应推断出 `: int`,实际 {hints:?}"
        );
    }

    #[test]
    fn completions_offer_members() {
        let s = project();
        let main = s.id_of("file:///proj/main.gd").unwrap();
        let text = s.doc(main).unwrap().text.to_string();
        let index = &s.doc(main).unwrap().index;
        let offset = index.offset(
            &text,
            lsp_types::Position {
                line: 2,
                character: 16,
            },
        );
        let items = s.host.analysis().completions(gdscript_base::FilePosition { file: main, offset }).unwrap();
        assert!(!items.is_empty());
    }

}
