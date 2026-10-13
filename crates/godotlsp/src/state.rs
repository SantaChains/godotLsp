//! Server state: the analysis host, file registry, in-memory documents, project loading.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gdscript_base::FileId;
use gdscript_ide::{AnalysisHost, Change};
use lsp_types::Uri;

use crate::convert::LineIndex;
use crate::uri::{path_to_uri, uri_to_path};

/// One registered file's text plus its line index.
#[derive(Debug, Clone)]
pub struct Doc {
    pub text: Arc<str>,
    pub index: LineIndex,
}

/// The whole server state. Single-threaded: requests are handled sequentially on the
/// main loop thread, so an `Analysis` snapshot is only stale across awaits we don't have.
pub struct State {
    pub host: AnalysisHost,
    pub root: Option<PathBuf>,
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
    /// An empty state with no project.
    #[must_use]
    pub fn new() -> Self {
        Self {
            host: AnalysisHost::new(),
            root: None,
            uris: HashMap::new(),
            ids: HashMap::new(),
            docs: HashMap::new(),
            next_id: 0,
        }
    }

    /// The registered document for `file`.
    #[must_use]
    pub fn doc(&self, file: FileId) -> Option<&Doc> {
        self.docs.get(&file)
    }

    /// The `FileId` registered for `uri`.
    #[must_use]
    pub fn id_of(&self, uri: &str) -> Option<FileId> {
        self.ids.get(uri).copied()
    }

    /// The resolver used by POD→LSP conversions for files that may be closed.
    #[must_use]
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

    /// Add or replace a file's text in the analysis world.
    pub fn inject(&mut self, uri: &str, res: Option<String>, text: String) -> FileId {
        let file = self.ids.get(uri).copied().unwrap_or_else(|| self.alloc(uri));
        let index = LineIndex::new(&text);
        let text: Arc<str> = Arc::from(text.as_str());
        self.docs.insert(file, Doc {
            text: text.clone(),
            index,
        });
        let mut change = Change::new();
        change.change_file(file, text);
        if let Some(res) = res {
            change.set_file_path(file, res);
        }
        self.host.apply_change(change);
        file
    }

    /// `didOpen` — register editor text under this URI.
    pub fn open(&mut self, uri: &str, text: String) -> FileId {
        let path = uri.parse::<Uri>().ok().and_then(|u| uri_to_path(&u));
        let res = path.as_ref().and_then(|p| self.res_path(p));
        self.inject(uri, res, text)
    }

    /// A watched file changed on disk; reload it, or drop it if deleted. `project.godot`
    /// is re-applied as the project config instead of a file.
    pub fn watched(&mut self, uri: &str) {
        let Some(path) = uri.parse::<Uri>().ok().and_then(|u| uri_to_path(&u)) else {
            return;
        };
        if let Some(root) = self.root.clone() {
            if path.parent() == Some(root.as_path()) && path.file_name().is_some_and(|n| n == "project.godot") {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    self.set_project_config(text);
                }
                return;
            }
        }
        match std::fs::read_to_string(&path) {
            Ok(text) => {
                let res = self.res_path(&path);
                self.inject(uri, res, text);
            }
            Err(_) => self.remove(uri),
        }
    }

    /// Remove a file from the registry and the analysis world.
    pub fn remove(&mut self, uri: &str) {
        if let Some(file) = self.ids.remove(uri) {
            self.uris.remove(&file);
            self.docs.remove(&file);
            let mut change = Change::new();
            change.remove_file(file);
            self.host.apply_change(change);
        }
    }

    /// Set (or replace) the `project.godot` config text.
    pub fn set_project_config(&mut self, text: String) {
        let mut change = Change::new();
        change.set_project_config(text);
        self.host.apply_change(change);
    }

    /// Walk `root` and register every `.gd`/`.tscn`/`.tres` plus `project.godot`.
    /// The file set is the whole project, so absence-based diagnostics are enabled.
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
            let text: Arc<str> = Arc::from(text.as_str());
            self.docs.insert(file, Doc {
                text: text.clone(),
                index,
            });
            change.change_file(file, text);
            if let Some(res) = self.res_path(path) {
                change.set_file_path(file, res);
            }
        }
        match std::fs::read_to_string(root.join("project.godot")) {
            Ok(config) => change.set_project_config(config),
            Err(_) => {}
        }
        change.set_workspace_complete(true);
        self.host.apply_change(change);
        eprintln!("[godotlsp] loaded {} project files", paths.len());
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
        );
        s.inject(
            "file:///proj/main.gd",
            Some("res://main.gd".into()),
            "const M = preload(\"res://markup.gd\")\nfunc go():\n\tvar n := M.new().parse()\n\treturn n\n".into(),
        );
        s
    }

    #[test]
    fn engine_model_is_bundled() {
        assert!(
            !gdscript_api::godot_version().is_empty(),
            "bundled engine model missing: {}",
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
        );
        let diags = s.host.analysis().diagnostics(f).unwrap();
        assert!(!diags.is_empty(), "expected a parse diagnostic");
    }

    #[test]
    fn preload_binding_is_inferred() {
        let s = project();
        let main = s.id_of("file:///proj/main.gd").unwrap();
        let hints = s.host.analysis().inlay_hints(main).unwrap();
        assert!(
            hints.iter().any(|h| h.label.contains("int")),
            "expected an inferred `: int`, got {hints:?}"
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
