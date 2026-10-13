//! 服务器状态:分析宿主、文件注册表、内存文档、项目加载。
//!
//! P2 并发模型(P2 §6.7 第 1 步,参照 rust-analyzer `GlobalState`/`GlobalStateSnapshot`
//! 字段划分):[`State`] 是单写者可变世界,由主循环独占持有;[`Snapshot`] 是派生的
//! 只读快照 —— `Analysis` 为 salsa 克隆句柄(廉价、`Send`),注册表经 `Arc` 共享,
//! 文档表按 `Arc<Doc>` 逐项共享。请求被派发到阻塞线程池时携带快照,主循环继续
//! 处理通知(变更);并发 `apply_change` 会让在途快照的查询在边界处返回
//! `Err(Cancelled)`,由协议层映射为三态取消响应。

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gdscript_base::FileId;
use gdscript_ide::{Analysis, AnalysisHost, Change};
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

/// 全量服务器状态(单写者)。主循环独占;请求计算只读 [`State::snapshot`]。
///
/// salsa 0.27 硬约束:写路径(`zalsa_mut`)要求全部快照句柄消亡
/// (`cancel_others` 等待 clone 计数归 1)。因此写操作前,主循环必须等待
/// 在飞任务完成(见 main.rs 的 quiesce);[`revision`](State::revision)
/// 随每次写推进,快照携带派发时的 revision,过期结果由协议层比对识别。
pub struct State {
    pub host: AnalysisHost,
    pub root: Option<PathBuf>,
    /// 是否显示 inlay 类型提示(由客户端配置驱动)。
    pub inlay_hints: bool,
    /// 客户端要求忽略的诊断码(`UNUSED_VARIABLE` 等 Godot 编辑器同款码)。
    pub diag_ignore: Arc<Vec<String>>,
    /// 世界修订号:每次写(`apply_change`)后 +1。快照携带派发时的值,
    /// 任务结果回到主循环时与当前值比对 —— 不一致即结果过期。
    pub revision: u64,
    uris: Arc<HashMap<FileId, String>>,
    ids: Arc<HashMap<String, FileId>>,
    docs: HashMap<FileId, Arc<Doc>>,
    next_id: u32,
}

/// 只读快照:阻塞线程池上请求计算所持的一致视图。
///
/// 构造成本 = 一次 salsa 句柄克隆 + 数次 `Arc` 克隆 + 文档表逐项指针拷贝,
/// 与项目文件数线性但远低于文本量;生成后不再变化。注意:快照存活期间
/// 主循环不得写世界(salsa 0.27 写路径要求句柄消亡,见 `State` 文档)。
pub struct Snapshot {
    pub analysis: Analysis,
    pub root: Option<PathBuf>,
    pub inlay_hints: bool,
    pub diag_ignore: Arc<Vec<String>>,
    /// 派发时的世界修订号;结果回主循环时与 `State::revision` 比对。
    pub revision: u64,
    uris: Arc<HashMap<FileId, String>>,
    ids: Arc<HashMap<String, FileId>>,
    docs: HashMap<FileId, Arc<Doc>>,
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
            diag_ignore: Arc::new(Vec::new()),
            revision: 0,
            uris: Arc::new(HashMap::new()),
            ids: Arc::new(HashMap::new()),
            docs: HashMap::new(),
            next_id: 0,
        }
    }

    /// 派生只读快照(见模块文档)。仅在无在飞任务时调用(见 `State` 文档)。
    #[must_use]
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            analysis: self.host.analysis(),
            root: self.root.clone(),
            inlay_hints: self.inlay_hints,
            diag_ignore: Arc::clone(&self.diag_ignore),
            revision: self.revision,
            uris: Arc::clone(&self.uris),
            ids: Arc::clone(&self.ids),
            docs: self.docs.clone(),
        }
    }

    /// `file` 对应的已注册文档。
    #[cfg(test)]
    #[must_use]
    pub fn doc(&self, file: FileId) -> Option<&Doc> {
        self.docs.get(&file).map(Arc::as_ref)
    }

    /// `uri` 对应的 `FileId`。
    #[must_use]
    pub fn id_of(&self, uri: &str) -> Option<FileId> {
        self.ids.get(uri).copied()
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
        // Arc 化注册表:快照在飞时写侧克隆(写时复制),否则原地写。
        Arc::make_mut(&mut self.uris).insert(id, uri.to_string());
        Arc::make_mut(&mut self.ids).insert(uri.to_string(), id);
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
        self.docs.insert(file, Arc::new(Doc { text, index, version }));
        self.host.apply_change(change);
        self.revision += 1;
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
        let doc = Arc::make_mut(doc);
        apply_content_changes(&mut doc.text, &mut doc.index, changes);
        doc.version = version;
        let mut change = Change::new();
        change.change_file(file, doc.text.as_str());
        self.host.apply_change(change);
        self.revision += 1;
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
        if let Some(file) = Arc::make_mut(&mut self.ids).remove(uri) {
            Arc::make_mut(&mut self.uris).remove(&file);
            self.docs.remove(&file);
            let mut change = Change::new();
            change.remove_file(file);
            self.host.apply_change(change);
            self.revision += 1;
        }
    }

    /// 设置(或替换)`project.godot` 配置文本。
    pub fn set_project_config(&mut self, text: String) {
        let mut change = Change::new();
        change.set_project_config(text);
        self.host.apply_change(change);
        self.revision += 1;
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
            change.change_file(file, text.as_str());
            self.docs.insert(file, Arc::new(Doc { text, index, version: None }));
            if let Some(res) = self.res_path(path) {
                change.set_file_path(file, res);
            }
        }
        if let Ok(config) = std::fs::read_to_string(root.join("project.godot")) {
            change.set_project_config(config);
        }
        change.set_workspace_complete(true);
        self.host.apply_change(change);
        self.revision += 1;
        eprintln!("[godotlsp] 已加载 {} 个项目文件", paths.len());
    }
}

impl Snapshot {
    /// `file` 对应的已注册文档。
    #[must_use]
    pub fn doc(&self, file: FileId) -> Option<&Doc> {
        self.docs.get(&file).map(Arc::as_ref)
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

    /// revision 推进与快照一致性:snapshot 携带派发时的 revision,写后推进,
    /// 新快照可见变更。注意:salsa 0.27 写路径要求快照句柄先消亡,因此本测试
    /// 在 apply 前显式 `drop(snap)` —— 与主循环的 quiesce 等待同一约束。
    #[test]
    fn revision_advances_and_snapshot_is_consistent() {
        let mut s = State::new();
        let f = s.inject(
            "file:///p/a.gd",
            Some("res://a.gd".into()),
            "var x = 1\n".into(),
            Some(1),
        );
        let snap = s.snapshot();
        assert_eq!(snap.revision, 1);
        assert_eq!(snap.doc(f).unwrap().text, "var x = 1\n");
        assert_eq!(snap.doc(f).unwrap().version, Some(1));
        drop(snap); // salsa 0.27:写前必须放弃全部句柄
        s.apply_changes(f, vec![lsp_types::TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "var x = 2222\n".into(),
        }], Some(2));
        assert_eq!(s.revision, 2);
        let snap2 = s.snapshot();
        assert_eq!(snap2.revision, 2);
        assert_eq!(snap2.doc(f).unwrap().text, "var x = 2222\n");
        assert_eq!(snap2.doc(f).unwrap().version, Some(2));
    }
}
