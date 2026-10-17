//! `godotlsp` —— 独立 GDScript 语言服务器（R4 最小真实版）。
//!
//! LSP 3.17 over stdio，分析内核来自 workspace 自研 crate：
//! gdscript-syntax（词法/语法）、gdscript-engine（引擎模型）、gdscript-project（跨文件）。
//!
//! 支持：initialize/shutdown/exit、didOpen/didChange、publishDiagnostics、
//! hover、completion。

use std::collections::{HashMap, HashSet};

use gdscript_engine::{ConstValue, EngineApi};
use gdscript_project::Project;
use gdscript_syntax::ast::{
    expr_to_text, span_slice, BinOp, ClassDecl, ConstDecl, EnumDecl, Expr, File, FuncDecl,
    Literal, SignalDecl, Stmt, UnOp, VarDecl,
};
use gdscript_syntax::infer::{infer_expr_ty, preload_consts, FnRetTable, InferTy};
use gdscript_syntax::parse;
use gdscript_syntax::semantic::{analyze_with, FileInfo, ProjectContext};
use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::Uri;
use lsp_types::*;

mod argopts;
mod doclinks;
mod freq;
mod hoverdata;
mod langdocs;
mod scoring;
mod snippets;

struct State {
    engine: std::sync::Arc<EngineApi>,
    /// 真实类名小写全集（doclinks 伪类消歧：@GDScript vs 真类 GDScript）。
    real_lower: HashSet<String>,
    /// 打开文档：uri -> 文本。
    docs: HashMap<Uri, String>,
    /// 缓存的 AST：uri -> File（didOpen/didChange 后更新，请求时直接用）。
    asts: HashMap<Uri, gdscript_syntax::ast::File>,
    /// 文档版本号（didOpen/didChange 回传，publishDiagnostics.version）。
    doc_versions: HashMap<Uri, i32>,
    /// 已取消请求 ID 集合（$/cancelRequest）。
    cancelled: HashSet<lsp_server::RequestId>,
    /// uid:// -> res:// 相对路径映射（扫描 *.uid 文件）。
    uid_map: HashMap<String, String>,
    /// 项目级符号图（打开根目录后加载）。
    project: Option<std::sync::Arc<Project>>,
    /// 项目根目录（res:// → 文件路径用）。
    root: Option<std::path::PathBuf>,
    /// 后台诊断发送端（150ms 去抖）。
    diag_sender: std::sync::mpsc::Sender<(Uri, String)>,
    /// 客户端声明了 completionItem.snippetSupport（initialize 协商，不可变）。
    snippet_support: bool,
    /// `godotlsp.*` 配置（didChangeConfiguration 推送更新）。
    config: Config,
    /// 编辑器补全设置（`settings.gdscript` 段三键，镜像引擎 EditorSettings
    /// `text_editor/completion/*`，默认全 false；oracle per-fixture 推送）。
    prefs: EditorPrefs,
}

/// 编辑器补全字符串设置（gdscript_editor.cpp:895 `_calculate_string_insertion`）。
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct EditorPrefs {
    /// 引号风格：true → `'`，false → `"`。
    use_single_quotes: bool,
    /// StringName 语义实参加 `&` 前缀。
    add_string_name_literals: bool,
    /// NodePath 语义实参加 `^` 前缀。
    add_node_path_literals: bool,
}

/// 服务器侧配置。来源：`workspace/didChangeConfiguration` 的
/// `settings.godotlsp` 段（VSCode languageclient 在 initialize 后自动全量
/// 推送一次——拉通道因此省略，缺省值在解析时逐键兜底）。
#[derive(Clone, Debug)]
struct Config {
    /// C2 snippet 模板面（与客户端 snippetSupport 协商双闸门）。
    snippets: bool,
    /// C3 本文件词条面。默认关——VSCode 的 wordBasedSuggestions 已提供
    /// 同面补全，默认开会与客户端内置行为重复感。
    file_words: bool,
    /// C4 语料频率加权排序（空前缀热度细分）。
    freq_boost: bool,
    /// 内联类型提示（inlayHints）。
    inlay_type_hints: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { snippets: true, file_words: false, freq_boost: true, inlay_type_hints: true }
    }
}

impl Config {
    /// 从 `settings.godotlsp` 段解析（缺键兜底默认值；非 bool 值忽略）。
    fn from_section(v: &serde_json::Value) -> Config {
        let mut c = Config::default();
        if let Some(b) = v.get("completion").and_then(|s| s.get("snippets")).and_then(|x| x.as_bool()) {
            c.snippets = b;
        }
        if let Some(b) = v.get("completion").and_then(|s| s.get("fileWords")).and_then(|x| x.as_bool()) {
            c.file_words = b;
        }
        if let Some(b) = v.get("completion").and_then(|s| s.get("frequencyBoost")).and_then(|x| x.as_bool()) {
            c.freq_boost = b;
        }
        if let Some(b) = v.get("inlayHints").and_then(|s| s.get("typeHints")).and_then(|x| x.as_bool()) {
            c.inlay_type_hints = b;
        }
        c
    }
}

fn main() {
    let (connection, io_threads) = Connection::stdio();    let server_capabilities = serde_json::to_value(ServerCapabilities {
        position_encoding: Some(PositionEncodingKind::UTF16),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![".".into()]),
            ..CompletionOptions::default()
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec!["(".into(), ",".into()]),
            ..SignatureHelpOptions::default()
        }),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        document_link_provider: Some(DocumentLinkOptions { resolve_provider: Some(false), work_done_progress_options: Default::default() }),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        inlay_hint_provider: Some(OneOf::Left(true)),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        color_provider: Some(ColorProviderCapability::Simple(true)),
        semantic_tokens_provider: Some(
            SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: semtok_legend_types(),
                    token_modifiers: vec![],
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                ..SemanticTokensOptions::default()
            })
        ),
        code_lens_provider: Some(CodeLensOptions { resolve_provider: Some(false) }),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: Default::default(),
        })),
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            ..WorkspaceServerCapabilities::default()
        }),
        ..ServerCapabilities::default()
    })
    .expect("server capabilities");

    // 手动握手，以便在响应里带上 serverInfo（扩展状态栏显示版本）。
    let initialize_value = loop {
        match connection.receiver.recv() {
            Ok(Message::Request(req)) if req.method == "initialize" => {
                let result = serde_json::json!({
                    "capabilities": server_capabilities,
                    "serverInfo": {
                        "name": "godotlsp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                });
                connection
                    .sender
                    .send(Message::Response(Response::new_ok(req.id, result)))
                    .expect("发送 initialize 响应");
                break req.params;
            }
            Ok(_) => continue,
            Err(e) => panic!("等待 initialize 失败: {e}"),
        }
    };
    let initialize_params: InitializeParams = serde_json::from_value(initialize_value).expect("解析 initialize 参数");

    // C2：snippetSupport 协商——客户端不声明则 snippet 面整体不出现。
    let snippet_support = initialize_params
        .capabilities
        .text_document
        .as_ref()
        .and_then(|td| td.completion.as_ref())
        .and_then(|c| c.completion_item.as_ref())
        .and_then(|ci| ci.snippet_support)
        .unwrap_or(false);

    // 根目录：rootUri 优先，否则取首个 workspace folder；多选多根时全部纳入扫描。
    #[allow(deprecated)]
    let root = initialize_params
        .root_uri
        .clone()
        .or_else(|| {
            initialize_params
                .workspace_folders
                .as_ref()
                .and_then(|f| f.first().map(|f| f.uri.clone()))
        })
        .and_then(|u| url_to_path(&u));

    // 多根工作区：把所有 workspace folder 也纳入扫描（rootUri 与首个 folder 去重）。
    let mut scan_roots: Vec<std::path::PathBuf> = Vec::new();
    if let Some(r) = &root {
        scan_roots.push(r.clone());
    }
    if let Some(folders) = &initialize_params.workspace_folders {
        for f in folders {
            if let Some(p) = url_to_path(&f.uri)
                && !scan_roots.iter().any(|e| e == &p) {
                    scan_roots.push(p);
                }
        }
    }

    let embedded_json = include_str!("../../gdscript-engine/testdata/extension_api.json");
    // 引擎模型：扫描每个根目录下的 .godot/extension_api.json（项目自带导出面），取首个命中；否则内嵌出厂面。
    let mut engine_json = embedded_json.to_string();
    let mut engine_source = "embedded(no-root)";
    for r in &scan_roots {
        let candidate = r.join(".godot/extension_api.json");
        match std::fs::read_to_string(&candidate) {
            Ok(text) if serde_json::from_str::<serde_json::Value>(&text).is_ok() => {
                engine_json = text;
                engine_source = "workspace";
                break;
            }
            Ok(_) => {
                engine_source = "embedded(parse-failed)";
            }
            Err(_) => {
                if engine_source == "embedded(no-root)" {
                    engine_source = "embedded";
                }
            }
        }
    }

    let engine = std::sync::Arc::new(EngineApi::from_json(&engine_json).expect("解析引擎模型"));
    let real_lower: HashSet<String> =
        engine.classes.keys().chain(engine.builtins.keys()).map(|s| s.to_lowercase()).collect();

    // 信息日志统一走 window/logMessage（客户端输出面板按 info 级显示）；
    // stderr 只留致命错误——vscode-languageclient 会把 stderr 一律标成 error 级。
    let log_info = |msg: String| {
        let _ = connection.sender.send(Message::Notification(Notification::new(
            "window/logMessage".into(),
            serde_json::json!({ "type": 3, "message": msg }),
        )));
    };

    // 版本对齐：读 project.godot config/features，与 dump 版本比对。
    // 优先定位真实项目根（向上查找 project.godot），否则回退到工作区根。
    let project_root = root.as_ref().and_then(|r| locate_project_root(r)).or(root.clone());
    if let Some(ref r) = project_root
        && let Ok(pg) = std::fs::read_to_string(r.join("project.godot")) {
            for line in pg.lines() {
                if line.starts_with("config/features") {
                    log_info(format!("[godotlsp] 版本对齐: dump={}, project={}", engine.version, line.trim()));
                }
            }
        }

    // uid:// -> res:// 映射：扫描各根下所有 *.uid 文件。
    let mut uid_map: HashMap<String, String> = HashMap::new();
    for r in &scan_roots {
        scan_uid_files(r, r, &mut uid_map);
    }

    // 跨文件符号图：每个根目录分别打开并合并脚本/场景索引（多根/嵌套项目）。
    let mut merged: Option<Project> = None;
    for r in &scan_roots {
        if let Ok(p) = Project::open(r, Some(&engine_json)) {
            merged = Some(match merged {
                None => p,
                Some(mut acc) => {
                    acc.merge(p);
                    acc
                }
            });
        }
    }
    let project = merged.map(std::sync::Arc::new);

    let (diag_tx, diag_rx) = std::sync::mpsc::channel::<(Uri, String)>();
    let mut state = State {
        engine, real_lower, docs: HashMap::new(), asts: HashMap::new(), doc_versions: HashMap::new(),
        cancelled: HashSet::new(), uid_map, project, root, diag_sender: diag_tx,
        snippet_support, config: Config::default(),
        prefs: EditorPrefs::default(),
    };

    log_info(format!(
        "[godotlsp] 就绪，引擎 {} (来源: {})，扫描根 {} 个，项目文件 {}，uid 映射 {}",
        state.engine.version,
        engine_source,
        scan_roots.len(),
        state.project.as_ref().map(|p| p.scripts.len()).unwrap_or(0),
        state.uid_map.len(),
    ));
    for r in &scan_roots {
        log_info(format!("[godotlsp]   扫描根: {}", r.display()));
    }

    // 后台诊断线程：150ms 去抖，击键不直接触发 parse+analyze。
    let conn_sender = connection.sender.clone();
    let engine_arc = state.engine.clone();
    let project_arc = state.project.clone();

    // 诊断线程：收到任务后等 150ms，窗口内到达的任务**按 uri 合并**（同 uri 取最新），
    // 窗口结束逐个分析发布——不能只留最后一条：多文件批量打开（恢复会话/格式化全仓）
    // 会把前面的文件全部静默丢弃（bench_load.py 在 Pixelorama 上实测复现）。
    std::thread::spawn(move || {
        let recv = diag_rx;
        let conn_sender = conn_sender;
        let engine = engine_arc;
        let project = project_arc;
        loop {
            let Ok((uri, src)) = recv.recv() else { break };
            // lsp_types::Uri 内部含可变性,clippy 误报 mutable_key_type;
            // 此处只做插入/覆盖合并不读 key,合入后不再改动 key,安全。
            #[allow(clippy::mutable_key_type)]
            let mut batch: HashMap<Uri, String> = HashMap::new();
            batch.insert(uri, src);
            while let Ok((u, s)) = recv.recv_timeout(std::time::Duration::from_millis(150)) {
                batch.insert(u, s);
            }
            let proj_ctx: Option<&dyn ProjectContext> =
                project.as_ref().map(|p| p.as_ref() as &dyn ProjectContext);
            for (uri, src) in batch {
                // 完整分析：parse errors + 语义错误（未声明标识符、类型错误）。
                let (file, parse_errs) = parse(&src);
                let mut diags: Vec<Diagnostic> = parse_errs.iter().map(|e| Diagnostic {
                    range: offset_range(&src, e.start, e.end),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: e.message.clone(),
                    ..Diagnostic::default()
                }).collect();
                let analysis = analyze_with(&file, &engine, proj_ctx);
                for e in analysis.errors {
                    diags.push(Diagnostic {
                        range: offset_range(&src, e.span.0, e.span.1),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: e.message,
                        ..Diagnostic::default()
                    });
                }
                let _ = conn_sender.send(Message::Notification(Notification::new(
                    "textDocument/publishDiagnostics".into(),
                    serde_json::to_value(PublishDiagnosticsParams {
                        uri,
                        diagnostics: diags,
                        version: None,
                    }).unwrap(),
                )));
            }
        }
    });

    let mut exit = false;
    while let Ok(msg) = connection.receiver.recv() {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).expect("shutdown 处理") {
                    continue;
                }
                // 请求级 panic 隔离：单请求崩溃不拖死进程，也必须给客户端一个应答——
                // 悬空请求会让 VSCode 干等超时（表现为该能力静默失效）。
                let req_id = req.id.clone();
                let req_method = req.method.clone();
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    handle_request(req, &mut state)
                }));
                match result {
                    Ok(resp) => connection.sender.send(Message::Response(resp)).expect("send response"),
                    Err(err) => {
                        let detail = err
                            .downcast_ref::<&str>()
                            .map(|s| s.to_string())
                            .or_else(|| err.downcast_ref::<String>().cloned())
                            .unwrap_or_else(|| "unknown panic".into());
                        eprintln!("[godotlsp] 请求处理 panic，已隔离: {req_method}: {detail}");
                        let _ = connection.sender.send(Message::Notification(Notification::new(
                            "window/logMessage".into(),
                            serde_json::json!({ "type": 1, "message": format!("[godotlsp] 请求 {req_method} 处理异常，已隔离: {detail}") }),
                        )));
                        let _ = connection.sender.send(Message::Response(Response::new_err(
                            req_id,
                            lsp_server::ErrorCode::InternalError as i32,
                            format!("internal error in {req_method}"),
                        )));
                    }
                }
            }
            Message::Notification(notif) => {
                if notif.method == "exit" {
                    exit = true;
                }
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    handle_notification(notif, &mut state, &connection);
                }));
            }
            Message::Response(_) => {}
        }
        if exit {
            break;
        }
    }
    let _ = io_threads.join();
}

fn handle_request(req: Request, state: &mut State) -> Response {
    let id = req.id;
    // 取消三态：已取消的请求直接回 RequestCancelled(-32802)。
    if state.cancelled.remove(&id) {
        return Response::new_err(id, -32802, "RequestCancelled".to_string());
    }
    let params = req.params;
    // P2.8 rename 单独前置：拒绝重命名要回 error（客户端 toast），不能走 new_ok(null)。
    if req.method == "textDocument/rename" {
        let p: RenameParams = serde_json::from_value(params).unwrap();
        return match rename(state, &p) {
            Ok(we) => Response::new_ok(id, serde_json::to_value(we).unwrap()),
            Err(msg) => Response::new_err(id, -32602, msg),
        };
    }
    let result = match req.method.as_str() {
        "textDocument/hover" => {
            let p: HoverParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(hover(state, &p)).unwrap()
        }
        "textDocument/completion" => {
            let p: CompletionParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(completion(state, &p)).unwrap()
        }
        "textDocument/documentSymbol" => {
            let p: DocumentSymbolParams = serde_json::from_value(params).unwrap();
            let syms = document_symbols(state, &p);
            serde_json::to_value(syms).unwrap()
        }
        "textDocument/signatureHelp" => {
            let p: SignatureHelpParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(signature_help(state, &p)).unwrap()
        }
        "textDocument/definition" => {
            let p: GotoDefinitionParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(goto_definition(state, &p)).unwrap()
        }
        "textDocument/references" => {
            let p: ReferenceParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(find_references(state, &p)).unwrap()
        }
        "textDocument/documentLink" => {
            let p: DocumentLinkParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(document_links(state, &p)).unwrap()
        }
        "textDocument/inlayHint" => {
            let p: InlayHintParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(inlay_hints(state, &p)).unwrap()
        }
        "textDocument/foldingRange" => {
            let p: FoldingRangeParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(folding_ranges(state, &p)).unwrap()
        }
        "textDocument/documentHighlight" => {
            let p: DocumentHighlightParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(document_highlight(state, &p)).unwrap()
        }
        "textDocument/documentColor" => {
            let p: DocumentColorParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(document_color(state, &p)).unwrap()
        }
        "textDocument/colorPresentation" => {
            let p: ColorPresentationParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(color_presentation(&p)).unwrap()
        }
        "textDocument/codeLens" => {
            let p: CodeLensParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(code_lens(state, &p)).unwrap()
        }
        "textDocument/prepareRename" => {
            let p: TextDocumentPositionParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(prepare_rename(state, &p)).unwrap()
        }
        "textDocument/semanticTokens/full" => {
            let p: SemanticTokensParams = serde_json::from_value(params).unwrap();
            let src = state.docs.get(&p.text_document.uri).cloned().unwrap_or_default();
            serde_json::to_value(compute_semantic_tokens(&src)).unwrap()
        }
        "workspace/symbol" => {
            let p: WorkspaceSymbolParams = serde_json::from_value(params).unwrap();
            serde_json::to_value(workspace_symbol(state, &p.query)).unwrap()
        }
        _ => serde_json::Value::Null,
    };
    Response::new_ok(id, result)
}

fn handle_notification(notif: Notification, state: &mut State, _conn: &Connection) {
    match notif.method.as_str() {
        "textDocument/didOpen" => {
            let p: DidOpenTextDocumentParams = serde_json::from_value(notif.params.clone()).unwrap();
            let uri = p.text_document.uri;
            let ver = p.text_document.version;
            let src = p.text_document.text;
            let (file, _) = parse(&src);
            state.docs.insert(uri.clone(), src.clone());
            state.asts.insert(uri.clone(), file);
            state.doc_versions.insert(uri.clone(), ver);
            let _ = state.diag_sender.send((uri.clone(), src.clone()));
        }
        "textDocument/didChange" => {
            let p: DidChangeTextDocumentParams = serde_json::from_value(notif.params.clone()).unwrap();
            let uri = p.text_document.uri;
            state.doc_versions.insert(uri.clone(), p.text_document.version);
            if let Some(doc) = state.docs.get_mut(&uri) {
                for change in p.content_changes {
                    if let Some(range) = change.range {
                        apply_range_change(doc, &range, &change.text);
                    } else {
                        *doc = change.text;
                    }
                }
            }
            if let Some(doc) = state.docs.get(&uri) {
                let (file, _) = parse(doc);
                state.asts.insert(uri.clone(), file);
                let _ = state.diag_sender.send((uri.clone(), doc.clone()));
            }
        }
        "workspace/didChangeConfiguration" => {
            // VSCode languageclient 推 `settings: { godotlsp: {...} }`（initialize 后
            // 自动推一次全量 + 用户改动时增量）。逐键解析，缺键兜底默认值。
            let settings = notif.params.get("settings").cloned().unwrap_or_default();
            if let Some(section) = settings.get("godotlsp") {
                state.config = Config::from_section(section);
            }
            // `settings.gdscript` 段：引擎补全设置三键（gdscript_editor.cpp
            // `EDITOR_GET("text_editor/completion/…")` 同名语义，缺省 false）。
            if let Some(gs) = settings.get("gdscript") {
                if let Some(b) = gs.get("use_single_quotes").and_then(|x| x.as_bool()) {
                    state.prefs.use_single_quotes = b;
                }
                if let Some(b) = gs.get("add_string_name_literals").and_then(|x| x.as_bool()) {
                    state.prefs.add_string_name_literals = b;
                }
                if let Some(b) = gs.get("add_node_path_literals").and_then(|x| x.as_bool()) {
                    state.prefs.add_node_path_literals = b;
                }
            }
        }
        "workspace/didChangeWatchedFiles" => {
            // 扩展监听 **/*.gd 等；项目级 class_name 缓存由客户端侧重建，内核按需忽略。
        }
        "$/cancelRequest" => {
            let p: CancelParams = serde_json::from_value(notif.params).unwrap();
            let rid: lsp_server::RequestId = match p.id { lsp_types::NumberOrString::Number(n) => n.into(), lsp_types::NumberOrString::String(s) => s.into() }; state.cancelled.insert(rid);
        }
        "initialized" => {}
        _ => {}
    }
}


/// 光标处单词的 hover：从引擎模型渲染完整签名 + 语料描述 + 文档链接（P1 描述层）。
fn hover(state: &State, p: &HoverParams) -> Option<Hover> {
    let uri = &p.text_document_position_params.text_document.uri;
    let src = state.docs.get(uri)?;
    let offset = position_to_offset(src, p.text_document_position_params.position)?;

    // 精确高亮范围：光标词本身（不含前后空白/符号）。VSCode 对 range=None 的
    // hover 用自己的分词器回退框选,会把前导空白/制表连进来——显式给范围根治。
    let hov_range = word_span(src, offset)
        .map(|(s, w)| offset_range(src, s, s + w.len()));

    // 接收者模式：光标落在 `recv.member` 的 member 上。
    if let Some((recv_ty, member)) = resolve_receiver(state, src, offset)
        && let Some(md) = member_hover(state, &recv_ty, &member)
    {
        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: md }),
            range: hov_range,
        });
    }

    // 单词级：光标词紧贴 '@' 时优先按注解查（@export/@onready/…）。
    let (start, word) = word_span(src, offset)?;
    let is_annotation = start > 0 && src.as_bytes()[start - 1] == b'@';
    let md = hover_word(state, &word, is_annotation)?;
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: md }),
        range: hov_range,
    })
}

/// hover 卡片统一拼装：签名代码块 + 描述段（缺失则省略，不造句）+ 文档链接。
fn hover_md(sig: String, desc: Option<&str>, url: String) -> String {
    let mut md = format!("```gdscript\n{sig}\n```\n");
    if let Some(d) = desc
        && !d.is_empty()
    {
        md.push_str(d);
        md.push_str("\n\n");
    }
    md.push_str(&format!("[文档]({url})"));
    md
}

/// 单词级 hover：注解 > 类 > 内建 > @GDScript 函数 > 全局函数 > 单例 > 全局枚举值 > 全局常量 > @GDScript 常量 > 关键字。
fn hover_word(state: &State, word: &str, is_annotation: bool) -> Option<String> {
    let real = |b: &str| state.real_lower.contains(b);
    let engine = &state.engine;

    // 注解（@export/@onready/…）：锚点 #annotation-<名>，与生成器契约一致。
    if is_annotation
        && let Some(desc) = hoverdata::annotation(word)
    {
        return Some(hover_md(format!("@{word}"), Some(desc), doclinks::annotation_url(word, real)));
    }

    if let Some(cls) = engine.class_by_name(word) {
        let sig = format!(
            "class {} extends {}",
            cls.name,
            cls.inherits.as_deref().unwrap_or("Object")
        );
        return Some(hover_md(sig, hoverdata::class_brief(word), doclinks::class_url(word, real)));
    }
    if engine.builtin_by_name(word).is_some() {
        // C6.5c②：类型名 hover 追加构造重载列表（签名 + 描述，语料序）。
        let ctors = hoverdata::builtin_ctors(word);
        let mut md = format!("```gdscript\n{word}\n```\n");
        if let Some(d) = hoverdata::class_brief(word)
            && !d.is_empty()
        {
            md.push_str(d);
            md.push_str("\n\n");
        }
        if !ctors.is_empty() {
            md.push_str("---\n\n**构造函数**\n\n");
            for (sig, doc) in &ctors {
                md.push_str(&format!("```gdscript\n{sig}\n```\n"));
                if !doc.is_empty() {
                    md.push_str(doc);
                    md.push_str("\n\n");
                }
            }
        }
        md.push_str(&format!("[文档]({})", doclinks::class_url(word, real)));
        return Some(md);
    }
    // @GDScript 内建函数（len/preload/range/…，不在 extension_api.json）。
    if let Some((sig, desc)) = hoverdata::gdscript_fn(word) {
        let url = doclinks::member_url("@GDScript", word, false, real);
        return Some(hover_md(sig, Some(desc.as_str()), url));
    }
    if let Some(f) = engine.utility_function(word) {
        let url = doclinks::member_url("@GlobalScope", word, false, real);
        return Some(hover_md(render_function(f), hoverdata::member("@GlobalScope", word), url));
    }
    if let Some(ty) = engine.singleton_type(word) {
        let sig = format!("const {}: {} (单例)", word, ty);
        return Some(hover_md(sig, hoverdata::class_brief(ty), doclinks::class_url(ty, real)));
    }
    // 全局枚举值裸名（KEY_LEFT / EULER_ORDER_XYZ / …）。
    if let Some((enum_name, value)) = engine.global_enum_value(word) {
        let sig = format!("const {word} = {value} ({enum_name})");
        let url = doclinks::member_url("@GlobalScope", word, false, real);
        return Some(hover_md(sig, hoverdata::member("@GlobalScope", word), url));
    }
    if let Some(c) = engine.global_constants.iter().find(|c| c.name == word) {
        let sig = format!("const {} = {}", c.name, const_value_str(&c.value));
        let url = doclinks::member_url("@GlobalScope", word, false, real);
        return Some(hover_md(sig, hoverdata::member("@GlobalScope", word), url));
    }
    // @GDScript 常量兜底（PI/TAU/INF/NAN，不在 api json 全局常量里）。
    if let Some(md) = langdocs::gdscript_const_hover(&real, word) {
        return Some(md);
    }
    // 语言关键字（func/if/await/…）。
    if let Some(md) = langdocs::keyword_hover(word) {
        return Some(md);
    }
    None
}

/// 成员描述与锚点宿主查找（沿继承链回溯）。
/// `Sprite2D.position` 的描述挂在 Node2D 语料上——链接也必须指向宿主类页
/// （Sprite2D 页没有 `#position` 锚，语料条目与生成页锚点同源同宿主）。
/// 无描述时宿主退回请求类（维持旧 URL 行为）。
fn member_anchor_along<'a>(engine: &'a EngineApi, class: &'a str, member: &str) -> (&'a str, Option<&'static str>) {
    if let Some(d) = hoverdata::member(class, member) {
        return (class, Some(d));
    }
    for cls in engine.ancestor_chain(class) {
        if let Some(d) = hoverdata::member(cls, member) {
            return (cls, Some(d));
        }
    }
    (class, None)
}

/// 接收者成员 hover：`recv.member` → 属性/方法/枚举/枚举值，附语料描述。
fn member_hover(state: &State, recv_ty: &str, member: &str) -> Option<String> {
    let engine = &state.engine;
    let real = |b: &str| state.real_lower.contains(b);
    let md = if let Some(prop) = engine.find_property(recv_ty, member) {
        let sig = format!("{}.{}: {}", recv_ty, prop.name, prop.ty);
        let (host, desc) = member_anchor_along(engine, recv_ty, member);
        hover_md(sig, desc, doclinks::member_url(host, member, false, real))
    } else if let Some((k, host)) = engine.find_const(recv_ty, member) {
        // 常量引用（Vector2.ZERO / Node.NOTIFICATION_READY）；描述与锚点同宿主。
        let sig = format!("const {recv_ty}.{member} = {}", const_value_str(&k.value));
        hover_md(sig, hoverdata::member(host, member), doclinks::member_url(host, member, false, real))
    } else if let Some(f) = engine.find_method(recv_ty, member) {
        let sig = format!("{}.{}", recv_ty, render_function(f));
        let (host, desc) = member_anchor_along(engine, recv_ty, member);
        hover_md(sig, desc, doclinks::member_url(host, member, false, real))
    } else if let Some((e, enum_host)) = engine
        .ancestor_chain(recv_ty)
        .into_iter()
        .find_map(|cls| engine.find_enum(cls, member).map(|e| (e, cls)))
    {
        // 枚举名引用（`Node.ProcessMode` / `Vector3.Axis`）；宿主类 = 声明枚举的祖先。
        let vals = e.values.iter().map(|(n, v)| format!("{n} = {v}")).collect::<Vec<_>>().join(", ");
        let sig = format!("enum {} ({})", member, vals);
        hover_md(sig, hoverdata::member(enum_host, member), doclinks::member_url(enum_host, member, true, real))
    } else if let Some((enum_name, value, host)) = enum_value_in_class(engine, recv_ty, member) {
        // 枚举值引用（`Vector3.AXIS_X` / `Vector3.Axis.X` → member 直接或经枚举名命中）。
        let sig = format!("const {recv_ty}.{member} = {value} ({enum_name})");
        let url = doclinks::member_url(&host, member, false, real);
        hover_md(sig, hoverdata::member(&host, member), url)
    } else {
        return None;
    };
    Some(md)
}

/// 在类/内建类（含继承链）的枚举值中找 `member`：返回 (枚举名, 值, 描述宿主类)。
/// `Vector3.AXIS_X` → ("Axis", 0, "Vector3")；枚举名引用链（`Vector3.Axis`）走 find_enum 分支。
fn enum_value_in_class<'a>(engine: &'a EngineApi, class: &str, member: &str) -> Option<(&'a str, i64, String)> {
    // 直接枚举值：遍历继承链 + 内建枚举。
    for cls in engine.ancestor_chain(class) {
        if let Some(c) = engine.classes.get(cls) {
            for e in &c.enums {
                if let Some((_, v)) = e.values.iter().find(|(n, _)| n == member) {
                    return Some((e.name.as_str(), *v, c.name.clone()));
                }
            }
        }
    }
    if let Some(b) = engine.builtins.get(class) {
        for e in &b.enums {
            if let Some((_, v)) = e.values.iter().find(|(n, _)| n == member) {
                return Some((e.name.as_str(), *v, b.name.clone()));
            }
        }
    }
    None
}

/// 渲染信号签名：`name(a: T, b)`（信号是通知而非调用，无返回类型）。
fn render_signal(s: &gdscript_engine::EngineSignal) -> String {
    let args = s
        .arguments
        .iter()
        .map(|a| format!("{}: {}", a.name, a.ty))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}({})", s.name, args)
}

/// 渲染函数签名：`name(a: T, b) -> R`。
fn render_function(f: &gdscript_engine::Function) -> String {
    let args = f
        .arguments
        .iter()
        .map(|a| format!("{}: {}", a.name, a.ty))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{}({}) -> {}", f.name, args, f.return_type)
}

fn const_value_str(v: &gdscript_engine::ConstValue) -> String {
    match v {
        gdscript_engine::ConstValue::Int(i) => i.to_string(),
        gdscript_engine::ConstValue::Float(f) => format!("{f}"),
        gdscript_engine::ConstValue::String(s) => format!("\"{s}\""),
        gdscript_engine::ConstValue::Bool(b) => b.to_string(),
        gdscript_engine::ConstValue::Null => "null".into(),
    }
}

/// 解析光标前是否是 `recv.member`：返回 (recv_ty, member)。
/// recv 类型通过扫描当前文件的 var/@onready 声明粗解析。
fn resolve_receiver(state: &State, src: &str, offset: usize) -> Option<(String, String)> {
    let b = src.as_bytes();
    // 向前找成员名起点，必须紧贴 '.'；向后扫完整词（光标可落在词内任意处）。
    let mut i = offset;
    while i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
        i -= 1;
    }
    let mut j = offset.min(b.len());
    while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
        j += 1;
    }
    let member = &src[i..j];
    if member.is_empty() || i == 0 || b[i - 1] != b'.' {
        return None;
    }
    // 再向前找 recv 标识符。
    let mut j = i - 1;
    while j > 0 && (b[j - 1].is_ascii_alphanumeric() || b[j - 1] == b'_') {
        j -= 1;
    }
    let recv = &src[j..i - 1];
    if recv.is_empty() {
        return None;
    }
    let info = FileInfo::from_file(&parse(src).0);
    // 变量 → 推断类型；否则静态接收者（`Node.ProcessMode`/`Vector3.AXIS_X` 的 recv 是类名）。
    let ty = info.resolve_var(src, recv).or_else(|| {
        if state.engine.class_by_name(recv).is_some() || state.engine.builtin_by_name(recv).is_some() {
            Some(recv.to_string())
        } else {
            None
        }
    })?;
    Some((ty, member.to_string()))
}


/// 补全：`.` 后给接收者成员，否则给引擎全局名（类/内建/函数/单例/常量）。
/// C1 fuzzy + C4 频率细分的合成：空前缀时按频率排名细分中性带（热度排序，
/// 受 `godotlsp.completion.frequencyBoost` 开关），有前缀时纯 fuzzy（词面已充分区分）。
/// 频率表以裸 API 名为键（`print`），而 C6.4 起函数面 label 为 `print(…)`——
/// 查表前剥掉 `(…)` 后缀，否则热度细分对函数面整体失效。
fn scored_fuzzy(prefix: &str, label: &str, freq_boost: bool) -> u16 {
    let bare = label.strip_suffix("(…)").unwrap_or(label);
    if prefix.is_empty() && freq_boost {
        freq::rank(bare).map_or(scoring::band::NEUTRAL, scoring::neutral_by_rank)
    } else {
        scoring::fuzzy(prefix, label)
    }
}

/// C6.4 函数补全项：对齐引擎 LSP 适配器的展示/插入约定
/// (源码锚点 `.refs/godot/modules/gdscript/language_server/gdscript_language_protocol.cpp`):
/// label 恒为 `name(…)`;insertText 在客户端 snippetSupport 时为 `name($1)`
/// (Snippet 格式,落标在括号内),否则退化为裸名——上游 `trim_suffix("(")` 同款:
/// 客户端放不进光标就不给括号(插进去也补不齐)。
/// callable-expected 上下文(如 `connect(` 二参)不出括号属语义层判定,未做(见路线图)。
fn function_item(
    name: &str,
    kind: CompletionItemKind,
    detail: Option<String>,
    snippet_support: bool,
) -> CompletionItem {
    let (insert_text, insert_text_format) = if snippet_support {
        (Some(format!("{name}($1)")), Some(InsertTextFormat::SNIPPET))
    } else {
        (Some(name.to_string()), None)
    };
    CompletionItem {
        label: format!("{name}(…)"),
        kind: Some(kind),
        detail,
        insert_text,
        insert_text_format,
        ..CompletionItem::default()
    }
}

/// 光标是否在 `$Path/Seg` / `%Unique/Seg` 取节点路径内。镜像 fa7d645 的向后
/// 字节扫描(无词法器感知,字符串/注释内的误报与内核同级,内核侧还会再以解析树复核)。
/// R4 服务层重写(22882f7)曾丢失本函数,补全在 `$Panel/` 处混入全局面 1662 项
/// (探针实测,其中 1036 个引擎类)——节点路径上下文必须与全局面互斥。
/// 入参 `offset` 是光标偏移(自含词内回扫);调用方传词首亦等价。
fn in_node_path_context(text: &str, offset: usize) -> bool {
    let b = text.as_bytes();
    let is_seg = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = offset.min(b.len());
    while i > 0 && is_seg(b[i - 1]) {
        i -= 1;
    }
    loop {
        match i.checked_sub(1).map(|k| b[k]) {
            Some(b'$') => return true,
            // `%` 兼作取模运算符:前一字符是操作数(标识符/右括号/右方括号)时按取模处理,
            // 不视为路径(内核侧由解析树精确判定,此处仅为合并抑制的保守近似)。
            Some(b'%') => {
                return !i
                    .checked_sub(2)
                    .map(|k| matches!(b[k], b')' | b']') || is_seg(b[k]))
                    .unwrap_or(false);
            }
            Some(b'/') => {
                let mut s = i - 1;
                while s > 0 && is_seg(b[s - 1]) {
                    s -= 1;
                }
                i = s;
            }
            _ => return false,
        }
    }
}

/// C7.2/N2：光标是否位于字符串字面量内；返回内容区间
/// `(开引号后一字节, 闭引号/行尾/文末)`。
/// 前向状态机扫描(词法层近似)：覆盖 `"..."` 与 `"""..."""`、转义 `\"`、
/// `&"..."`/`^"..."` 字面量前缀(前缀字符不影响引号配对)；`#` 注释在字符串
/// 状态外时截断该行(注释内引号不成串)。未闭合字符串按到行尾(单引号对)或
/// 文末(三引号)处理——与引擎错误恢复同向。
pub(crate) fn string_literal_at(src: &str, at: usize) -> Option<(usize, usize)> {
    let b = src.as_bytes();
    let n = b.len();
    let mut i = 0usize;
    while i < n {
        match b[i] {
            b'#' => while i < n && b[i] != b'\n' {
                i += 1;
            },
            b'"' => {
                if b.get(i..i + 3) == Some(b"\"\"\"") {
                    // 多行字符串：内容起于三引号之后，到闭合三引号或文末。
                    let content_start = i + 3;
                    let mut j = content_start;
                    while j < n && b.get(j..j + 3) != Some(b"\"\"\"") {
                        if b[j] == b'\\' {
                            j += 1;
                        }
                        j += 1;
                    }
                    let close = j.min(n);
                    if at >= content_start && at <= close {
                        return Some((content_start, close));
                    }
                    i = j + 3;
                } else {
                    // 单行字符串：内容起于开引号之后，到闭引号/行尾(转义跳过下一字节)。
                    let content_start = i + 1;
                    let mut j = content_start;
                    while j < n && b[j] != b'"' && b[j] != b'\n' {
                        if b[j] == b'\\' {
                            j += 1;
                        }
                        j += 1;
                    }
                    let close = j.min(n);
                    if at >= content_start && at <= close {
                        return Some((content_start, close));
                    }
                    i = j + 1;
                }
            }
            _ => i += 1,
        }
    }
    None
}

/// 字符串是否为 `preload(` / `load(` 的实参(C7.2 路径面前提)。
/// 从开引号左侧回扫：空白 → `(` → 空白 → 标识符 ∈ {`preload`, `load`}。
/// `ResourceLoader.load(` 的 `load` 段同样命中(其首参也是 res:// 路径)。
fn is_load_call_arg(src: &str, quote_pos: usize) -> bool {
    let b = src.as_bytes();
    let mut i = quote_pos;
    while i > 0 && b[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    if i == 0 || b[i - 1] != b'(' {
        return false;
    }
    i -= 1;
    while i > 0 && b[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    let end = i;
    while i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
        i -= 1;
    }
    matches!(&src[i..end], "preload" | "load")
}

/// C7.2：`preload("res://` / `load("res://` 项目文件路径面。
/// 来源 = `project.scripts` + `project.scenes`(Project::open 已建索引,
/// `path` 为 res:// 相对路径);label 为完整 `res://…` 路径,textEdit 替换
/// 字符串内容区(开引号后..光标)——不依赖客户端 wordPattern(各编辑器对
/// `/`/`.` 是否成词约定不一,显式编辑范围是 LSP 标准机制)。
fn preload_path_items(
    state: &State,
    src: &str,
    typed: &str,
    content_start: usize,
    cursor: usize,
) -> Vec<CompletionItem> {
    let Some(proj) = &state.project else {
        return Vec::new();
    };
    let mut paths: Vec<&str> = proj
        .scripts
        .iter()
        .map(|s| s.path.as_str())
        .chain(proj.scenes.iter().map(|s| s.path.as_str()))
        .collect();
    paths.sort_unstable();
    paths.dedup();
    let edit_range = Range::new(
        offset_to_position(src, content_start),
        offset_to_position(src, cursor),
    );
    paths
        .into_iter()
        .filter(|p| p.starts_with(typed))
        .map(|p| {
            let label = format!("res://{p}");
            CompletionItem {
                label: label.clone(),
                kind: Some(CompletionItemKind::FILE),
                detail: Some("项目资源".to_string()),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: edit_range,
                    new_text: label.clone(),
                })),
                sort_text: Some(scoring::sort_text(
                    scoring::bucket::MEMBER,
                    scoring::fuzzy(typed, p),
                )),
                ..CompletionItem::default()
            }
        })
        .collect()
}

fn completion(state: &State, p: &CompletionParams) -> CompletionResponse {
    let uri = &p.text_document_position.text_document.uri;
    let Some(src) = state.docs.get(uri) else {
        return CompletionResponse::Array(vec![]);
    };
    let offset = p.text_document_position.position;
    let b = src.as_bytes();
    // 光标越界防御：客户端可能发超出行数/列数的 position（测试/竞态），clamp 后再切片。
    let at = (line_start_offset(src, offset.line as usize) + offset.character as usize).min(src.len());
    // C1：向前扫到词首——partial 词也正确归面（`Node.po` → 成员面，prefix="po"）。
    let mut word_start = at;
    while word_start > 0 && (b[word_start - 1].is_ascii_alphanumeric() || b[word_start - 1] == b'_') {
        word_start -= 1;
    }
    let prefix = &src[word_start..at];
    let after_dot = word_start > 0 && b[word_start - 1] == b'.';
    let mut items: Vec<CompletionItem> = Vec::new();
    if std::env::var("GODOTLSP_DEBUG_RESOLVE").is_ok() {
        eprintln!("[completion] at={at} word_start={word_start} after_dot={after_dot} prefix={prefix:?}");
    }

    if after_dot {
        // 脚本枚举面兜底:didOpen/didChange 未及或竞态时现解析(与 1199 等处惯用法一致)。
        let ast_owned = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
        // P5 C10：self./super. 接收者 → 当前脚本三层成员面(优先于
        // recv_type_at——后者只做变量/类名解析,不识别 self/super 关键字)。
        let recv_word = receiver_word(src, word_start);
        if matches!(recv_word, Some("self") | Some("super")) {
            items.extend(script_self_members(state, uri, src, recv_word == Some("self")));
        } else if let Some(ty_items) = type_hint_dot_path(src, word_start)
            .and_then(|p| type_member_items(state, uri, &ast_owned, &p))
        {
            // P6 C5（D3）：类型标注位 after_dot（`var x: R.➡`）→ 类型成员面
            // (内部类/枚举/常量名 + 全局枚举名；引擎不出 builtin 与实例成员,
            // oracle types/hints/index_1_* 锁定)。path 无类型源（枚举名/全局
            // 枚举/未知名——py_value/builtin_enum 回归教训）→ None 回落原链。
            items.extend(ty_items);
        } else if enum_value_completion_allowed(src, word_start, at) {
            // 枚举解析优先于成员面(引擎语义:receiver 是枚举则出枚举值,否则出成员;
            // gdscript_editor.cpp `_code_completion` 先解析枚举常量再回落成员)。
            // 否则 `enum TestEnum{}; TestEnum.` 会被 resolve_var 当成变量类型走 class_members 截胡。
            // 类型标注(`var x: Enum.`)与字典键(`{ Enum. = }`)上下文引擎不出枚举值,门控掉。
            if let Some(values) = enum_member_values_at(state, src, word_start, Some(&ast_owned)) {
                // `Class.Enum.<|>` / `Enum.<|>`：接收者是枚举名 → 出该枚举的值面
                // (引擎成员面对 `AStarGrid2D.Heuristic.` / `Key.` / match 位同款;
                // oracle builtin_enum / global_enum_values / enum_values_in_match 桶)。
                items.extend(values);
            } else if let Some(rt) = resolve_recv_type(state, uri, src, at, word_start) {
                items.extend(recv_member_items(state, uri, src, &rt));
            }
        } else if let Some(rt) = resolve_recv_type(state, uri, src, at, word_start) {
            items.extend(recv_member_items(state, uri, src, &rt));
        }
        for i in &mut items {
            let f = scored_fuzzy(prefix, &i.label, state.config.freq_boost);
            i.sort_text = Some(scoring::sort_text(scoring::bucket::MEMBER, f));
        }
    } else if word_start > 0 && b[word_start - 1] == b'@' {
        // 注解面：`@<|` 只出注解（36 个），不与全局面混淆。
        for (name, desc) in hoverdata::annotations() {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(desc.to_string()),
                ..CompletionItem::default()
            });
        }
        for i in &mut items {
            i.sort_text = Some(scoring::sort_text(scoring::bucket::MEMBER, scoring::fuzzy(prefix, &i.label)));
        }
    } else if in_node_path_context(src, word_start) {
        // `$Path/Seg` / `%Unique/Seg` 节点路径上下文(C7.3):全局面合并会污染
        // (fa7d645 探针实测 1662 项,其中 1036 个引擎类)。场景树节点面待
        // C7.3+ 工程面接入(project.scenes → tscn NodeDecl);在此之前不出项——
        // 空列表优于 1662 项噪声,也与引擎"只出场景子节点"同向。
    } else if let Some(arg_items) =
        argopts::argument_options(state, uri, src, at, word_start, state.asts.get(uri))
    {
        // C8：调用实参/注解实参选项面（connect 信号名、InputEvent 输入动作、
        // @export_range/@rpc/@warning_ignore/@export_custom/@export_node_path 参数）。
        // 引擎语义(gdscript_editor.cpp:3432/3474)：调用面零项回落常规面
        // (argument_options 返回 None)，注解参数面 forced 独占(空 Vec)；
        // 命中即只出选项面,不与全局面混合。
        // **先于 N5**——InputEvent 动作面等特判表优先于 Callable 常规面
        // （dont_add_string_name 回归教训）。
        items.extend(arg_items);
    } else if let Some(n5_items) = argopts::callable_arg_items(state, uri, src, at) {
        // N5（P6 C5.3）：实参位形参 Callable → 方法名无括号；本文件方法
        // 非 Callable 位 → 常规 name() 形态。面独占（argopts 同律）。
        for i in n5_items {
            let f = scored_fuzzy(prefix, &i.label, state.config.freq_boost);
            let mut i = i;
            i.sort_text = Some(scoring::sort_text(scoring::bucket::MEMBER, f));
            items.push(i);
        }
    } else if let Some((content_start, content_end)) = string_literal_at(src, at) {
        // N2：字符串字面量内默认不出补全(引擎默认行为;oracle
        // `no_completion_in_string` 锁定——此前因全局面无对应排除项而
        // vacuous pass,抑制后面向才正确)。
        // C7.2 例外:`preload("res://` / `load("res://` → 项目文件路径面。
        if is_load_call_arg(src, content_start - 1) {
            let typed = &src[content_start..at.min(content_end)];
            if let Some(rest) = typed.strip_prefix("res://") {
                items.extend(preload_path_items(state, src, rest, content_start, at));
            }
        }
    } else {
        // 全量返回 + 稳定排序（HashMap 迭代序随机，take(n) 会导致补全时有时无）；
        // 前缀/fuzzy 过滤由客户端负责。
        let mut names: Vec<&str> = state.engine.classes.keys().map(String::as_str).collect();
        names.sort_unstable();
        for name in names {
            items.push(CompletionItem { label: name.to_string(), kind: Some(CompletionItemKind::CLASS), ..CompletionItem::default() });
        }
        let mut names: Vec<&str> = state.engine.builtins.keys().map(String::as_str).collect();
        names.sort_unstable();
        for name in names {
            items.push(CompletionItem { label: name.to_string(), kind: Some(CompletionItemKind::TYPE_PARAMETER), ..CompletionItem::default() });
        }
        for f in &state.engine.utility_functions {
            items.push(function_item(
                &f.name,
                CompletionItemKind::FUNCTION,
                Some(render_function(f)),
                state.snippet_support,
            ));
        }
        for s in &state.engine.singletons {
            items.push(CompletionItem {
                label: s.name.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some(format!("{} (单例)", s.ty)),
                ..CompletionItem::default()
            });
        }
        // C7.1：项目工程面——class_name 脚本类 + autoload 单例
        // (gdscript-project 早已索引,此前只被语义层消费,补全全局面漏掉)。
        // 与引擎类/内建/单例同名时让位引擎(同名注册在 Godot 中即非法,取任一即可)。
        if let Some(proj) = &state.project {
            let mut seen: HashSet<&str> = HashSet::new();
            for s in &proj.scripts {
                if let Some(cn) = &s.ast.class_name
                    && seen.insert(cn.name.as_str())
                    && !state.engine.classes.contains_key(&cn.name)
                    && !state.engine.builtins.contains_key(&cn.name)
                {
                    items.push(CompletionItem {
                        label: cn.name.clone(),
                        kind: Some(CompletionItemKind::CLASS),
                        detail: Some(format!("class_name (res://{})", s.path)),
                        ..CompletionItem::default()
                    });
                }
            }
            for (name, ty) in proj.autoloads() {
                if state.engine.singletons.iter().any(|s| s.name == name) {
                    continue;
                }
                items.push(CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    detail: Some(format!("{ty} (autoload)")),
                    ..CompletionItem::default()
                });
            }
        }
        for c in &state.engine.global_constants {
            items.push(CompletionItem {
                label: c.name.clone(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some(const_value_str(&c.value)),
                ..CompletionItem::default()
            });
        }
        // 全局枚举值展开（Key.LEFT / Error.Ok / EULER_ORDER_XYZ…）。
        for e in &state.engine.global_enums {
            for (n, v) in &e.values {
                items.push(CompletionItem {
                    label: n.clone(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some(format!("{}.{n} = {v}", e.name)),
                    ..CompletionItem::default()
                });
            }
        }
        // C6.6b：全局枚举**类型名**入裸标识符全局面（Key / Error / KeyLocation…22 个，
        // oracle `global_enum_identifier`/`in_type_hint` 两夹具）。`extends <|>` 位
        // 除外——引擎 extends 只出类名（`global_enum_in_extends` exclude 锁定）。
        // extends 位判定含内部类 extends 行（`class X extends ➡`，行内独立词）。
        let line_prefix = &src[line_start_offset(src, offset.line as usize)..word_start];
        let in_extends = line_prefix.split_whitespace().any(|w| w == "extends");
        if !in_extends {
            for e in &state.engine.global_enums {
                items.push(CompletionItem {
                    label: e.name.clone(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some("@GlobalScope 枚举".to_string()),
                    ..CompletionItem::default()
                });
            }
            // P6 C5（D2）：类型标注位（`var x: A➡`）追加 native 类枚举名裸名
            // （ConnectFlags 等 764 个——Object/Xxx 的类枚举，不在 global_enums；
            // oracle types/hints/index_0 锁定）。仅类型位，全局面不膨胀。
            let line_start = line_start_offset(src, offset.line as usize);
            if in_type_hint_position(src, line_start, word_start) {
                let mut enum_names: Vec<&str> = state
                    .engine
                    .classes
                    .values()
                    .flat_map(|c| c.enums.iter().map(|e| e.name.as_str()))
                    .chain(
                        state
                            .engine
                            .builtins
                            .values()
                            .flat_map(|b| b.enums.iter().map(|e| e.name.as_str())),
                    )
                    .collect();
                enum_names.sort_unstable();
                enum_names.dedup();
                for n in enum_names {
                    items.push(CompletionItem {
                        label: n.to_string(),
                        kind: Some(CompletionItemKind::ENUM),
                        detail: Some("类枚举".to_string()),
                        ..CompletionItem::default()
                    });
                }
                // 类型位类作用域类型名（所在内部类的 const/enum/内部类，
                // oracle types/hints/index_0_inner_class：AInner 等仅类内可见）。
                items.extend(class_scope_type_names(state, uri, src, at));
            }
        } else {
            // P5：extends 位脚本内部类（当前文件兄弟 + 脚本父链；引擎类与
            // class_name 脚本类由全局面既有项覆盖）。
            items.extend(extends_inner_class_items(state, uri, src, at, prefix));
        }
        // 编译器内建函数（len/preload/range…，不在 extension_api.json）。
        for (name, bf) in hoverdata::gdscript_fns() {
            items.push(function_item(
                &name,
                CompletionItemKind::FUNCTION,
                Some(bf.0.to_string()),
                state.snippet_support,
            ));
        }
        // @GDScript 常量（PI/TAU/INF/NAN）。
        for name in ["PI", "TAU", "INF", "NAN"] {
            if let Some(desc) = hoverdata::member("@GDScript", name) {
                items.push(CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::CONSTANT),
                    detail: Some(desc.to_string()),
                    ..CompletionItem::default()
                });
            }
        }
        // 语言关键字（func/if/await…）；C2：控制流关键字升级为 snippet 插入（label 不变）。
        for (kw, desc, _) in langdocs::keywords() {
            let mut it = CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(desc.to_string()),
                ..CompletionItem::default()
            };
            let use_snippets = state.snippet_support && state.config.snippets;
            if use_snippets
                && let Some(body) = snippets::keyword_snippet(kw) {
                    it.insert_text = Some(body.to_string());
                    it.insert_text_format = Some(InsertTextFormat::SNIPPET);
                }
            items.push(it);
        }
        // P5：override 签名面（`func ➡` / `static func ➡` 行位）。不依赖
        // snippet 协商——oracle override 夹具裸连接；脚本父链方法签名源码
        // 原样（含尾冒号），static 位只出 static。引擎虚方法模板仍走 C2 闸。
        {
            let line_start = line_start_offset(src, offset.line as usize);
            let line_prefix = &src[line_start..word_start];
            if line_prefix.trim_end().ends_with("func") {
                let want_static = line_prefix.split_whitespace().any(|w| w == "static");
                items.extend(override_signatures(state, uri, src, at, prefix, want_static));
            }
        }
        // P5 C10：当前类三层成员入裸标识符面（引擎 identifiers_in_* 行为：
        // 本脚本自声明 + 脚本父链 + 引擎链）。SELF 桶已随项写入（C1 桶化跳过）。
        items.extend(script_self_members(state, uri, src, true));
        // extends 位：脚本成员面/全局面不得含光标所在类自身（extends_self exclude）。
        if in_extends {
            let file_now = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
            if let Some(sn) = enclosing_class_name(&file_now, at) {
                items.retain(|i| i.label != sn);
            }
        }
        // 形参 + 函数体局部声明名（LOCAL 桶最高优先；同名让位既有面）。
        {
            let ast_for_scope = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
            for name in local_scope_names(&ast_for_scope, src, at) {
                if items.iter().any(|i| i.label == name) {
                    continue;
                }
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    detail: Some("局部".into()),
                    sort_text: Some(scoring::sort_text(
                        scoring::bucket::LOCAL,
                        scoring::fuzzy(prefix, &name),
                    )),
                    ..CompletionItem::default()
                });
            }
        }
        // C2：snippet 面（snippetSupport 协商 × godotlsp.completion.snippets 双闸门）。
        if state.snippet_support && state.config.snippets {
            let line_start = line_start_offset(src, offset.line as usize);
            let line_prefix = &src[line_start..word_start];
            // `func _<|` → 虚方法模板，按宿主继承链过滤（无 extends 默认 RefCounted）。
            if line_prefix.trim_end().ends_with("func") && prefix.starts_with('_') {
                let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
                let host = file.extends.as_ref().map(|e| expr_to_text(src, e)).unwrap_or_else(|| "RefCounted".to_string());
                let chain = state.engine.ancestor_chain(&host);
                items.extend(snippets::virtual_items(&chain, prefix));
            }
            // 语句位（行首空白后）→ 复合模板，仅词面命中项。
            if line_prefix.trim().is_empty() {
                items.extend(snippets::statement_items(prefix));
            }
        }
        // C1：全局面按 kind 归桶（函数/单例 > 类型 > 常量枚举 > 关键字）+ fuzzy 细分。
        for i in &mut items {
            if i.sort_text.is_some() {
                continue; // snippet 项已在 snippets.rs 编码。
            }
            let bucket = match i.kind {
                Some(CompletionItemKind::FUNCTION) | Some(CompletionItemKind::VARIABLE) => scoring::bucket::FN_GLOBAL,
                Some(CompletionItemKind::CLASS) | Some(CompletionItemKind::TYPE_PARAMETER) => scoring::bucket::TYPE,
                Some(CompletionItemKind::CONSTANT) | Some(CompletionItemKind::ENUM_MEMBER) => scoring::bucket::CONST,
                Some(CompletionItemKind::SNIPPET) => scoring::bucket::SNIPPET,
                _ => scoring::bucket::KEYWORD,
            };
            let f = if matches!(i.kind, Some(CompletionItemKind::SNIPPET)) {
                scoring::fuzzy(prefix, &i.label) // snippet 项不吃频率（模板非 API 名）。
            } else {
                scored_fuzzy(prefix, &i.label, state.config.freq_boost)
            };
            i.sort_text = Some(scoring::sort_text(bucket, f));
        }
        // C3：本文件词条面——局部定义（空前缀也出）+ 高频词（有前缀才出，fuzzy>0 过滤）。
        // 排除保留名（引擎类/内建函数/关键字/注解/@GDScript 函数）——
        // 那些已由专门面提供，词条面只出用户自定义标识符，避免同名重复项。
        // `godotlsp.completion.fileWords` 开关（默认关：VSCode wordBasedSuggestions 已提供同面）。
        if state.config.file_words {
            let (locals, freq) = file_words(src, at);
            let reserved: std::collections::HashSet<String> = state
                .real_lower
                .iter()
                .cloned()
                .chain(langdocs::keywords().iter().map(|(k, _, _)| k.to_string()))
                .chain(hoverdata::annotations().iter().map(|(a, _)| a.to_string()))
                .chain(hoverdata::gdscript_fns().iter().map(|(f, _)| f.to_string()))
                .collect();
            let mut emitted: std::collections::HashSet<String> =
                items.iter().map(|i| i.label.clone()).collect();
            for name in &locals {
                if !emitted.contains(name.as_str()) {
                    emitted.insert(name.clone());
                    items.push(CompletionItem {
                        label: name.clone(),
                        kind: Some(CompletionItemKind::VARIABLE),
                        detail: Some("局部变量".into()),
                        sort_text: Some(scoring::sort_text(scoring::bucket::LOCAL, scoring::fuzzy(prefix, name))),
                        ..CompletionItem::default()
                    });
                }
            }
            if !prefix.is_empty() {
                for (w, n) in &freq {
                    if emitted.contains(w.as_str()) || reserved.contains(w.to_lowercase().as_str()) {
                        continue;
                    }
                    let f = scoring::fuzzy(prefix, w);
                    if f > 0 {
                        emitted.insert(w.clone());
                        items.push(CompletionItem {
                            label: w.clone(),
                            kind: Some(CompletionItemKind::VARIABLE),
                            detail: Some(format!("本文件出现 {n} 次")),
                            sort_text: Some(scoring::sort_text(scoring::bucket::LOCAL, f)),
                            ..CompletionItem::default()
                        });
                    }
                }
            }
        }
    }
    CompletionResponse::Array(items)
}

/// 在 `recv.<|` 位置解析接收者类型（P6 C5 管线）：
/// ① 方法调用回溯（`a.to_str(10).➡` → 本脚本/内部类方法返回类型推断）；
/// ② `recv` 词作用域解析（局部 → 类级 → preload 常量 → 内部类 →
///    全局类 → 引擎类；`resolve_name_type`）。
fn resolve_recv_type(state: &State, uri: &Uri, src: &str, at: usize, word_start: usize) -> Option<RecvType> {
    if let Some(rt) = method_call_return_type(state, uri, src, word_start) {
        return Some(rt);
    }
    let recv = receiver_word(src, word_start)?;
    resolve_name_type(state, uri, src, at, recv)
}

/// 接收者成员面分流：引擎类 → class_members；脚本类 → 脚本链 + 引擎链。
fn recv_member_items(state: &State, uri: &Uri, src: &str, rt: &RecvType) -> Vec<CompletionItem> {
    match rt {
        RecvType::Engine(c) => class_members(&state.engine, c, state.snippet_support),
        RecvType::Script { .. } => script_class_members(state, uri, src, rt),
    }
}

/// 方法调用返回类型回溯（P6 C5.2 消费）：`recv.method(args).➡`——
/// word_start-1 是 `.`，其前是 `)` 时回溯匹配括号取方法名与接收者词。
/// 只支持一层词接收者；方法限本脚本（顶层 func 或内部类 func，live AST），
/// 返回类型 = 注解直取 / `return <expr>` 推断（func_return_inferred）。
/// 跨脚本方法返回类型推断不做（静默 None）。
fn method_call_return_type(state: &State, uri: &Uri, src: &str, word_start: usize) -> Option<RecvType> {
    let b = src.as_bytes();
    // `.` 前必须紧邻 `)`（`word.` 已由 resolve_name_type 处理）。
    if word_start < 3 || b[word_start - 2] != b')' {
        return None;
    }
    // 括号匹配（忽略字符串内括号——夹具调用形态度，简化逐层计数）。
    let close = word_start - 2;
    let mut depth = 0i32;
    let mut open = None;
    let mut i = close;
    while i > 0 {
        match b[i] {
            b')' => depth += 1,
            b'(' => {
                depth -= 1;
                if depth == 0 {
                    open = Some(i);
                    break;
                }
            }
            _ => {}
        }
        i -= 1;
    }
    let open = open?;
    // `(` 前是方法名标识符。
    let mut m_end = open;
    while m_end > 0 && (b[m_end - 1].is_ascii_alphanumeric() || b[m_end - 1] == b'_') {
        m_end -= 1;
    }
    if m_end == open {
        return None;
    }
    let method = &src[m_end..open];
    // 方法名前是接收者词（或接收者也是调用——不支持，静默）。
    let mut r_end = m_end;
    while r_end > 0 && (b[r_end - 1].is_ascii_alphanumeric() || b[r_end - 1] == b'_') {
        r_end -= 1;
    }
    if r_end == m_end {
        return None; // `(...).method(` 无词接收者
    }
    let recv_name = &src[r_end..m_end];
    if r_end > 0 && (b[r_end - 1] == b'.' || b[r_end - 1] == b')') {
        return None; // 链式/嵌套调用接收者不做
    }
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    // 接收者注解类型（局部/形参/类级 `var a: TestClass`）→ 本文件内部类
    // body 的方法；否则接收者是顶层 func（`my_func(1).➡`）。
    if let Some(ty_name) = local_annotation_text(state, uri, src, r_end, recv_name)
        && let Some(c) = inner_class(&file, &ty_name) {
            for st in &c.body {
                if let Stmt::Func(f) = st
                    && f.name.name == method {
                        return gdscript_syntax::func_return_inferred(src, f, &state.engine, builtin_fn_ret())
                            .map(RecvType::Engine);
                    }
            }
            return None;
        }
    if let Some(Stmt::Func(f)) = file.body.iter().find(|st| matches!(st, Stmt::Func(f) if f.name.name == recv_name)) {
        return gdscript_syntax::func_return_inferred(src, f, &state.engine, builtin_fn_ret())
            .map(RecvType::Engine);
    }
    None
}

/// 名字的类型注解文本（局部 var/形参/类级 var 的 `: T` 注解，光标 at 前）。
/// P6：方法调用回溯用（推断接收者是否本文件内部类）。
fn local_annotation_text(state: &State, uri: &Uri, src: &str, at: usize, name: &str) -> Option<String> {
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    // 最内层 func：形参 + 局部 var。
    let last_idx = file.body.len().saturating_sub(1);
    if let Some(f) = file.body.iter().enumerate().find_map(|(i, st)| match st {
        Stmt::Func(f) if f.span.0 <= at && (at <= f.span.1 || (i == last_idx && at <= src.len())) => Some(f),
        _ => None,
    }) {
        for p in &f.params {
            if p.name.name == name && p.span.1 < at {
                return p.ty.as_ref().map(|t| type_text(src, t));
            }
        }
        let mut blocks: Vec<&gdscript_syntax::ast::Block> = Vec::new();
        collect_blocks(&f.body, &mut blocks);
        for blk in &blocks {
            for st in &blk.stmts {
                if let Stmt::Var(v) = st
                    && v.name.name == name
                    && v.span.1 < at {
                        return v.ty.as_ref().map(|t| type_text(src, t));
                    }
            }
        }
    }
    // 类级 + 顶层 var。
    for stmts in class_scope_stmts(&file, at) {
        for st in stmts {
            if let Stmt::Var(v) = st
                && v.name.name == name
                && v.span.1 < at {
                    return v.ty.as_ref().map(|t| type_text(src, t));
                }
        }
    }
    None
}

// ───────────────────────── P5 C10 脚本成员面 ─────────────────────────

/// uri → res:// 相对路径（工程相对；code_lens 同款换算，供工程索引查询）。
fn uri_res_path(state: &State, uri: &Uri) -> Option<String> {
    let root = state.root.as_ref()?;
    let path = url::Url::parse(uri.as_str()).ok()?.to_file_path().ok()?;
    let rel = path.strip_prefix(root).ok()?;
    Some(format!("res://{}", rel.to_string_lossy().replace('\\', "/")))
}

/// `.` 前接收者词（after_dot 已判定：word_start-1 是 '.'）。
/// 无接收者词（`().` 等）返回 None。
fn receiver_word(src: &str, word_start: usize) -> Option<&str> {
    if word_start < 2 {
        return None;
    }
    let b = src.as_bytes();
    let end = word_start - 1; // '.' 位置
    let mut j = end;
    while j > 0 && (b[j - 1].is_ascii_alphanumeric() || b[j - 1] == b'_') {
        j -= 1;
    }
    if j == end {
        None
    } else {
        Some(&src[j..end])
    }
}

/// 光标所在顶层类名（span 放宽：body 最后一个 stmt 到文件尾——不完整行尾
/// `func `/`extends ` 待补时 at 超过解析器 span 终点）。override 面 enclosing
/// 判定、extends 位自身排除共用。
fn enclosing_class_name(file: &File, at: usize) -> Option<String> {
    let last_idx = file.body.len().saturating_sub(1);
    file.body.iter().enumerate().find_map(|(i, st)| match st {
        Stmt::Class(c) if c.span.0 <= at && (at <= c.span.1 || i == last_idx) => {
            Some(c.name.name.clone())
        }
        _ => None,
    })
}

/// 脚本方法补全项：label 零参 `name()` / 带参 `name(…)`（引擎 display 约定，
/// oracle self.gd/identifiers 夹具锁定）；detail = 签名；insert 同 C6.4 约定。
fn script_func_item(
    name: &str,
    signature: Option<&str>,
    kind: CompletionItemKind,
    snippet_support: bool,
) -> CompletionItem {
    let label = match signature {
        Some(sig) if sig.starts_with(name) && sig.ends_with("()") => format!("{name}()"),
        Some(_) => format!("{name}(…)"),
        None => name.to_string(),
    };
    let (insert_text, insert_text_format) = if snippet_support {
        (Some(format!("{name}($1)")), Some(InsertTextFormat::SNIPPET))
    } else {
        (Some(name.to_string()), None)
    };
    CompletionItem {
        label,
        kind: Some(kind),
        detail: signature.map(str::to_string),
        insert_text,
        insert_text_format,
        ..CompletionItem::default()
    }
}

/// P5 C10：当前脚本成员面（`self.`/`super.` 接收者与裸标识符位共用）。
/// 三层合并（近层同名优先：seen 先到先得，层序即优先级）：
/// ① 本脚本自声明（live AST——避开工程索引时差；`include_live=false` 即
///    `super.` 位，只出父层）；
/// ② 脚本父类链（`extends` Ident/字符串路径 → `script_chain_members`
///    近→远收集，宿主链类枚举/内部类/信号全 kind）；
/// ③ 引擎链（链尾引擎基类 → `class_members`，复用 C6.1/6.2 信号/成员模型）。
/// 输出统一 SELF 桶（局部之后、常规成员之前）。
fn script_self_members(state: &State, uri: &Uri, src: &str, include_live: bool) -> Vec<CompletionItem> {
    use gdscript_project::DeclKind;
    let mut out: Vec<CompletionItem> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // 层①：本脚本自声明（live AST）。
    if include_live {
        let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
        for st in &file.body {
            let item = match st {
                Stmt::Signal(s) => {
                    if !seen.insert(s.name.name.clone()) {
                        continue;
                    }
                    let params = s.params.iter().map(|p| p.name.name.as_str()).collect::<Vec<_>>().join(", ");
                    CompletionItem {
                        label: s.name.name.clone(),
                        kind: Some(CompletionItemKind::EVENT),
                        detail: Some(format!("signal {}({params})", s.name.name)),
                        ..CompletionItem::default()
                    }
                }
                Stmt::Var(v) => {
                    if !seen.insert(v.name.name.clone()) {
                        continue;
                    }
                    let detail = v.ty.as_ref().map(|t| {
                        format!("{}: {}", v.name.name, expr_to_text(src, t))
                    });
                    CompletionItem {
                        label: v.name.name.clone(),
                        kind: Some(CompletionItemKind::PROPERTY),
                        detail,
                        ..CompletionItem::default()
                    }
                }
                Stmt::Const(c) => {
                    if !seen.insert(c.name.name.clone()) {
                        continue;
                    }
                    CompletionItem {
                        label: c.name.name.clone(),
                        kind: Some(CompletionItemKind::CONSTANT),
                        detail: Some(format!("const {}", c.name.name)),
                        ..CompletionItem::default()
                    }
                }
                Stmt::Enum(e) if e.name.is_some() => {
                    let n = e.name.as_ref().expect("checked");
                    if !seen.insert(n.name.clone()) {
                        continue;
                    }
                    CompletionItem {
                        label: n.name.clone(),
                        kind: Some(CompletionItemKind::ENUM),
                        detail: Some(format!(
                            "enum {} ({})",
                            n.name,
                            e.entries.iter().map(|(id, _)| id.name.as_str()).collect::<Vec<_>>().join(", ")
                        )),
                        ..CompletionItem::default()
                    }
                }
                Stmt::Enum(e) if e.name.is_none() => {
                    for (id, _) in &e.entries {
                        if seen.insert(id.name.clone()) {
                            out.push(CompletionItem {
                                label: id.name.clone(),
                                kind: Some(CompletionItemKind::CONSTANT),
                                detail: Some(format!("const {} (匿名枚举条目)", id.name)),
                                ..CompletionItem::default()
                            });
                        }
                    }
                    continue;
                }
                Stmt::Class(c) => {
                    if !seen.insert(c.name.name.clone()) {
                        continue;
                    }
                    CompletionItem {
                        label: c.name.name.clone(),
                        kind: Some(CompletionItemKind::CLASS),
                        detail: Some("内部类".to_string()),
                        ..CompletionItem::default()
                    }
                }
                Stmt::Func(f) => {
                    if !seen.insert(f.name.name.clone()) {
                        continue;
                    }
                    let sig = f.signature_text(src).trim_end_matches(':').to_string();
                    script_func_item(&f.name.name, Some(&sig), CompletionItemKind::METHOD, state.snippet_support)
                }
                _ => continue,
            };
            out.push(item);
        }
    }

    // SELF 桶统一赋值（裸标识符面用；self./super. 位由 after_dot 收尾重写 MEMBER）。
    // 须在层②③的提前 return 之前，否则无工程时层①项漏桶。
    for i in &mut out {
        if i.sort_text.is_none() {
            let f = scored_fuzzy("", &i.label, state.config.freq_boost);
            i.sort_text = Some(scoring::sort_text(scoring::bucket::SELF, f));
        }
    }
    // 层②③：脚本父类链 + 引擎链（工程索引）。
    let Some(proj) = &state.project else { return out };
    let Some(rp) = uri_res_path(state, uri) else { return out };
    if let Some(base) = proj.script_base_key_by_path(&rp) {
        for m in proj.script_chain_members(&base) {
            if !seen.insert(m.name.clone()) {
                continue;
            }
            let item = match m.kind {
                DeclKind::Method => script_func_item(
                    &m.name,
                    m.signature.as_deref(),
                    CompletionItemKind::METHOD,
                    state.snippet_support,
                ),
                DeclKind::Property => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: Some(m.name.clone()),
                    ..CompletionItem::default()
                },
                DeclKind::Const => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::CONSTANT),
                    detail: Some(format!("const {}", m.name)),
                    ..CompletionItem::default()
                },
                DeclKind::Signal => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::EVENT),
                    detail: Some(format!("signal {}", m.name)),
                    ..CompletionItem::default()
                },
                DeclKind::Enum => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::ENUM),
                    detail: Some(format!("enum {}", m.name)),
                    ..CompletionItem::default()
                },
                DeclKind::Class => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::CLASS),
                    detail: Some("内部类（脚本父类）".to_string()),
                    ..CompletionItem::default()
                },
            };
            out.push(item);
        }
    }
    // 层③：引擎链（链尾引擎基类 → 引擎成员模型）。
    if let Some(eb) = proj.script_engine_base_by_path(&rp) {
        out.extend(class_members(&state.engine, &eb, state.snippet_support));
    }
    for i in &mut out {
        if i.sort_text.is_none() {
            let f = scored_fuzzy("", &i.label, state.config.freq_boost);
            i.sort_text = Some(scoring::sort_text(scoring::bucket::SELF, f));
        }
    }
    out
}

// ───────────────────────── P6 C5 语义层类型推断 ─────────────────────────

/// 编译器内建函数返回类型表（`char` → `String` 等，extension_api 外；
/// hoverdata 签名 "String char(code: int)" 首词即返回类型）。进程级惰性。
fn builtin_fn_ret() -> &'static FnRetTable {
    static TABLE: std::sync::OnceLock<FnRetTable> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        hoverdata::gdscript_fns()
            .into_iter()
            .filter_map(|(n, (sig, _))| {
                gdscript_syntax::ret_from_signature(&sig).map(|r| (n, r))
            })
            .collect()
    })
}

/// 接收者类型：补全面消费的统一形态（P6 C5）。
#[derive(Clone, Debug)]
pub(crate) enum RecvType {
    /// 引擎类 / 内建类型名 → class_members 面。
    Engine(String),
    /// 脚本类（res:// 路径 + 可选内部类名）→ 脚本链 + 引擎链成员面。
    /// `inner` 仅支持当前文档内部类（live AST；preload 脚本的内部类
    /// 实例成员不展开——类型位只出类型名，见 type_member_items）。
    Script { res_path: String, inner: Option<String> },
}

/// 推断出的类型（InferTy）→ RecvType（需工程索引把 ScriptPath 落成路径；
/// 无索引时 Unknown/ScriptPath 一律 None 静默降级）。
fn infer_ty_to_recv(state: &State, ty: InferTy) -> Option<RecvType> {
    match ty {
        InferTy::Class(n) => Some(RecvType::Engine(n)),
        InferTy::ScriptPath(p) => {
            let proj = state.project.as_ref()?;
            proj.resolve_preload(&p).map(|_| RecvType::Script { res_path: p, inner: None })
        }
        InferTy::Unknown => None,
    }
}

/// D1 子类收窄：value 类型 is-a 注解类型时用 value（引擎行为——
/// `var test: Node = A.new()` 补全出 A 的成员；oracle typehint_broad 锁定）。
/// 脚本类型（ScriptPath / 工程脚本键）取 extends 链尾引擎基类为根；
/// 引擎类型沿 inherits 链。
fn is_subtype(state: &State, value: &InferTy, ann: &InferTy) -> bool {
    // 类型 → 引擎链根（脚本类型经工程索引解析；引擎类型自身）。
    let root = |ty: &InferTy| -> Option<String> {
        match ty {
            InferTy::Class(n) => {
                let proj = state.project.as_ref()?;
                if let Some(key) = proj.script_path_to_key(n) {
                    let _ = key;
                    return proj.script_engine_base_by_path(n).or_else(|| Some(n.clone()));
                }
                Some(n.clone())
            }
            InferTy::ScriptPath(p) => {
                let proj = state.project.as_ref()?;
                proj.script_engine_base_by_path(p).or_else(|| {
                    proj.resolve_preload(p).map(|_| "Node".to_string())
                })
            }
            InferTy::Unknown => None,
        }
    };
    let (Some(v), Some(a)) = (root(value), root(ann)) else {
        // 至少一方无根（preload 脚本无引擎根等）：同名即同型。
        return value == ann;
    };
    if v == a {
        return true;
    }
    state.engine.ancestor_chain(&v).contains(&a.as_str())
}

/// 声明类型解析（var/const/形参共用）：注解 + 初始值 + D1 收窄。
/// `file` 供 names 表（preload 常量名 → ScriptPath）。
fn decl_infer_ty(state: &State, src: &str, file: &File, ty: Option<&Expr>, value: Option<&Expr>) -> Option<InferTy> {
    let engine = &state.engine;
    let fn_ret = builtin_fn_ret();
    let names = |n: &str| -> Option<InferTy> {
        if let Some((_, p)) = preload_consts(file).into_iter().find(|(k, _)| k == n) {
            return Some(InferTy::ScriptPath(p));
        }
        // self 方法调用（`get_tree()` 等，无接收者词）：本文件方法推断，
        // miss 后引擎链方法返回类型（仅引擎/内建类；变体/void 静默降级）。
        for st in &file.body {
            if let Stmt::Func(f) = st
                && f.name.name == n
                && let Some(ret) = gdscript_syntax::func_return_inferred(src, f, engine, fn_ret)
            {
                return Some(InferTy::Class(ret));
            }
        }
        let root = argopts::script_engine_root(state, file);
        if let Some(m) = engine.find_method(&root, n)
            && engine.class_by_name(&m.return_type).is_some()
        {
            return Some(InferTy::Class(m.return_type.clone()));
        }
        None
    };
    let ann = ty.and_then(|t| gdscript_syntax::type_annotation_ty(t, fn_ret));
    let val = value.map(|e| infer_expr_ty(src, e, engine, fn_ret, &names));
    match (ann, val) {
        (Some(a), Some(v)) => Some(if is_subtype(state, &v, &a) { v } else { a }),
        (Some(a), None) => Some(a),
        (None, Some(v)) if !matches!(v, InferTy::Unknown) => Some(v),
        _ => None,
    }
}

/// 名字 → 类型（P6 C5 单文件作用域解析，静默失败）。解析顺序：
/// 局部作用域（最内层 func：形参 + var/const，光标前已声明）→ 类级
/// （所在内部类 body + 顶层 body）→ preload 常量表 → 本文件内部类名 /
/// class_name → 全局 class_name 类 → 引擎类 / 内建类型名。
pub(crate) fn resolve_name_type(state: &State, uri: &Uri, src: &str, at: usize, name: &str) -> Option<RecvType> {
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    let proj = state.project.as_ref();
    if std::env::var("GODOTLSP_DEBUG_RESOLVE").is_ok() {
        eprintln!("[resolve] name={name} at={at} top_stmts={:?}", file.body.iter().map(|s| match s {
            Stmt::Var(v) => format!("var {} span={:?} ty={:?}", v.name.name, v.span, v.ty.as_ref().map(|t| expr_to_text(src, t))),
            Stmt::Func(f) => format!("func {} span={:?}", f.name.name, f.span),
            _ => "other".into(),
        }).collect::<Vec<_>>());
    }

    // 形参/局部变量声明（嵌套块递归，只收 span 在光标前的声明）。
    let mut scope_ty: Option<InferTy> = None;
    let scan_stmts = |stmts: &[Stmt], out: &mut Option<InferTy>| {
        for st in stmts {
            let (n, sp1, t) = match st {
                Stmt::Var(v) => (
                    &v.name.name,
                    v.span.1,
                    decl_infer_ty(state, src, &file, v.ty.as_ref(), v.value.as_ref()),
                ),
                Stmt::Const(c) => (
                    &c.name.name,
                    c.span.1,
                    decl_infer_ty(state, src, &file, c.ty.as_ref(), Some(&c.value)),
                ),
                _ => continue,
            };
            if n == name && sp1 < at && t.is_some() {
                *out = t;
                return;
            }
        }
    };
    // 最内层 func + 形参。
    let last_idx = file.body.len().saturating_sub(1);
    let func = file.body.iter().enumerate().find_map(|(i, st)| match st {
        Stmt::Func(f) if f.span.0 <= at && (at <= f.span.1 || (i == last_idx && at <= src.len())) => Some(f),
        _ => None,
    });
    if let Some(f) = func {
        for p in &f.params {
            if p.name.name == name && p.span.1 < at {
                if let Some(t) = &p.ty {
                    scope_ty = Some(InferTy::Class(type_text(src, t)));
                }
                break;
            }
        }
        if scope_ty.is_none() {
            let mut blocks: Vec<&gdscript_syntax::ast::Block> = Vec::new();
            collect_blocks(&f.body, &mut blocks);
            for b in &blocks {
                scan_stmts(&b.stmts, &mut scope_ty);
                if scope_ty.is_some() {
                    break;
                }
            }
        }
    }
    if scope_ty.is_none() {
        // 类级 + 顶层（所在内部类优先，遮蔽顶层同名）。
        for stmts in class_scope_stmts(&file, at) {
            scan_stmts(stmts, &mut scope_ty);
            if scope_ty.is_some() {
                break;
            }
        }
    }
    if let Some(t) = scope_ty {
        // 注解是 preload 常量名（var test: A）→ 递归一层解析。
        if let InferTy::Class(n) = &t {
            if let Some((_, path)) = preload_consts(&file).into_iter().find(|(k, _)| k == n) {
                return infer_ty_to_recv(state, InferTy::ScriptPath(path));
            }
            // 注解是本文件内部类名 → 当前脚本 + 内部类。
            if inner_class(&file, n).is_some() {
                let rp = uri_res_path(state, uri).unwrap_or_default();
                return Some(RecvType::Script { res_path: rp, inner: Some(n.clone()) });
            }
        }
        return infer_ty_to_recv(state, t);
    }
    // preload 常量名（A / B）。
    if let Some((_, path)) = preload_consts(&file).into_iter().find(|(k, _)| k == name) {
        return infer_ty_to_recv(state, InferTy::ScriptPath(path));
    }
    // 本文件内部类名 → 当前脚本 + 内部类（`LocalInnerClass.➡` 类型成员面）。
    if inner_class(&file, name).is_some() {
        let rp = uri_res_path(state, uri).unwrap_or_default();
        return Some(RecvType::Script { res_path: rp, inner: Some(name.to_string()) });
    }
    // 全局 class_name 类（B）。
    if let Some(p) = proj.and_then(|p| p.class_script(name)) {
        return Some(RecvType::Script { res_path: format!("res://{}", p.path), inner: None });
    }
    // 引擎类 / 内建类型名。
    if state.engine.class_by_name(name).is_some() || state.engine.builtin_by_name(name).is_some() {
        return Some(RecvType::Engine(name.to_string()));
    }
    None
}

/// 类型注解表达式 → 类型文本（`A` / `Array[int]` → base 名）。
fn type_text(src: &str, e: &Expr) -> String {
    match e {
        Expr::Ident(id) => id.name.clone(),
        Expr::Generic(base, _, _) => type_text(src, base),
        _ => expr_to_text(src, e),
    }
}

/// 递归收集块（含 if/for/while/match 分支体）。显式生命周期收块，
/// 避免闭包 HRTB 推断坑（E0521）。
fn collect_blocks<'a>(blk: &'a gdscript_syntax::ast::Block, out: &mut Vec<&'a gdscript_syntax::ast::Block>) {
    out.push(blk);
    for st in &blk.stmts {
        match st {
            Stmt::If { branches, else_block, .. } => {
                for (_, b) in branches {
                    collect_blocks(b, out);
                }
                if let Some(e) = else_block {
                    collect_blocks(e, out);
                }
            }
            Stmt::For { body, .. } | Stmt::While { body, .. } => collect_blocks(body, out),
            Stmt::Match { arms, .. } => {
                for a in arms {
                    collect_blocks(&a.body, out);
                }
            }
            _ => {}
        }
    }
}

/// 类作用域声明序列（P6）：所在内部类 body 优先，再顶层 body（同名遮蔽）。
fn class_scope_stmts(file: &File, at: usize) -> Vec<&[Stmt]> {
    let mut out = Vec::new();
    if let Some(c) = enclosing_class(file, at) {
        out.push(c.body.as_slice());
    }
    out.push(file.body.as_slice());
    out
}

/// 光标所在内部类（P6：span 放宽同 enclosing_class_name）。
fn enclosing_class(file: &File, at: usize) -> Option<&ClassDecl> {
    let last_idx = file.body.len().saturating_sub(1);
    file.body.iter().enumerate().find_map(|(i, st)| match st {
        Stmt::Class(c) if c.span.0 <= at && (at <= c.span.1 || i == last_idx) => Some(c),
        _ => None,
    })
}

/// 本文件内部类名查找（顶层；嵌套不递归——夹具覆盖顶层形态）。
fn inner_class<'a>(file: &'a File, name: &str) -> Option<&'a ClassDecl> {
    file.body.iter().find_map(|st| match st {
        Stmt::Class(c) if c.name.name == name => Some(c),
        _ => None,
    })
}

/// 任意脚本类成员面（P6：preload 路径 / 工程类 / 当前文件内部类）。
/// 脚本自声明（工程索引链，近→远）+ 引擎链成员；`inner` 走 live AST
/// （当前文档内部类 body + 宿主引擎链）。
fn script_class_members(state: &State, uri: &Uri, src: &str, rt: &RecvType) -> Vec<CompletionItem> {
    use gdscript_project::DeclKind;
    let RecvType::Script { res_path, inner } = rt else {
        return vec![];
    };
    let mut out: Vec<CompletionItem> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let Some(proj) = state.project.as_ref() else { return out };

    // 内部类形态：当前文档 live AST 的该类 body + 宿主引擎链。
    if let Some(iname) = inner {
        let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
        if let Some(c) = inner_class(&file, iname) {
            for st in &c.body {
                let item = match st {
                    Stmt::Signal(s) => {
                        if !seen.insert(s.name.name.clone()) { continue; }
                        CompletionItem {
                            label: s.name.name.clone(),
                            kind: Some(CompletionItemKind::EVENT),
                            detail: Some(format!("signal {}", s.name.name)),
                            ..CompletionItem::default()
                        }
                    }
                    Stmt::Var(v) => {
                        if !seen.insert(v.name.name.clone()) { continue; }
                        CompletionItem {
                            label: v.name.name.clone(),
                            kind: Some(CompletionItemKind::PROPERTY),
                            detail: Some(v.name.name.clone()),
                            ..CompletionItem::default()
                        }
                    }
                    Stmt::Const(cc) => {
                        if !seen.insert(cc.name.name.clone()) { continue; }
                        CompletionItem {
                            label: cc.name.name.clone(),
                            kind: Some(CompletionItemKind::CONSTANT),
                            detail: Some(format!("const {}", cc.name.name)),
                            ..CompletionItem::default()
                        }
                    }
                    Stmt::Enum(e) if e.name.is_some() => {
                        let n = e.name.as_ref().expect("checked");
                        if !seen.insert(n.name.clone()) { continue; }
                        CompletionItem {
                            label: n.name.clone(),
                            kind: Some(CompletionItemKind::ENUM),
                            detail: Some(format!("enum {}", n.name)),
                            ..CompletionItem::default()
                        }
                    }
                    Stmt::Enum(e) if e.name.is_none() => {
                        for (id, _) in &e.entries {
                            if seen.insert(id.name.clone()) {
                                out.push(CompletionItem {
                                    label: id.name.clone(),
                                    kind: Some(CompletionItemKind::CONSTANT),
                                    detail: Some(format!("const {} (匿名枚举条目)", id.name)),
                                    ..CompletionItem::default()
                                });
                            }
                        }
                        continue;
                    }
                    Stmt::Class(cc) => {
                        if !seen.insert(cc.name.name.clone()) { continue; }
                        CompletionItem {
                            label: cc.name.name.clone(),
                            kind: Some(CompletionItemKind::CLASS),
                            detail: Some("内部类".to_string()),
                            ..CompletionItem::default()
                        }
                    }
                    Stmt::Func(f) => {
                        if !seen.insert(f.name.name.clone()) { continue; }
                        let sig = f.signature_text(src).trim_end_matches(':').to_string();
                        script_func_item(&f.name.name, Some(&sig), CompletionItemKind::METHOD, state.snippet_support)
                    }
                    _ => continue,
                };
                out.push(item);
            }
        }
        // 宿主引擎链（当前脚本 extends 链尾）。
        if let Some(rp) = uri_res_path(state, uri)
            && let Some(eb) = proj.script_engine_base_by_path(&rp) {
                out.extend(class_members(&state.engine, &eb, state.snippet_support));
            }
        return out;
    }

    // 脚本链成员（key 寻址，近→远）+ 引擎链。
    if let Some(key) = proj.script_path_to_key(res_path) {
        for m in proj.script_chain_members(&key) {
            if !seen.insert(m.name.clone()) {
                continue;
            }
            let item = match m.kind {
                DeclKind::Method => script_func_item(
                    &m.name,
                    m.signature.as_deref(),
                    CompletionItemKind::METHOD,
                    state.snippet_support,
                ),
                DeclKind::Property => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: Some(m.name.clone()),
                    ..CompletionItem::default()
                },
                DeclKind::Const => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::CONSTANT),
                    detail: Some(format!("const {}", m.name)),
                    ..CompletionItem::default()
                },
                DeclKind::Signal => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::EVENT),
                    detail: Some(format!("signal {}", m.name)),
                    ..CompletionItem::default()
                },
                DeclKind::Enum => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::ENUM),
                    detail: Some(format!("enum {}", m.name)),
                    ..CompletionItem::default()
                },
                DeclKind::Class => CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::CLASS),
                    detail: Some("内部类（脚本父类）".to_string()),
                    ..CompletionItem::default()
                },
            };
            out.push(item);
        }
    }
    if let Some(eb) = proj.script_engine_base_by_path(res_path) {
        out.extend(class_members(&state.engine, &eb, state.snippet_support));
    }
    out
}

/// 裸名位类型标注上下文判定（P6 D2）：行前缀以 `:` / `->` 结尾
/// （`var x: A➡` / `func f(x: C➡` / `-> R➡`）→ 引擎在该位追加
/// native 类枚举名（ConnectFlags 等裸名；oracle types/hints/index_0）。
fn in_type_hint_position(src: &str, line_start: usize, word_start: usize) -> bool {
    if line_start >= word_start || word_start > src.len() {
        return false;
    }
    let t = src[line_start..word_start].trim_end();
    t.ends_with(':') || t.ends_with("->")
}

/// 类型标注位 after_dot 的接收者路径（P6 D3）：`var x: R.➡` → `R`
/// （可两段 `A.B`；`func f() -> R.➡` 同支持）。取 `:` / `->` 中更靠后者
/// 为类型起点；**起点到词首必须同行**（防 `func f():` 块冒号跨行误判）；
/// 校验段只含标识符字符，防字典字面量/字符串误伤。
fn type_hint_dot_path(src: &str, word_start: usize) -> Option<String> {
    let pre = &src[..word_start.saturating_sub(1)]; // 去掉紧邻的 '.'
    // 只在光标所在行内找起点（跨行冒号是块冒号，非类型标注）。
    let line_start = pre.rfind('\n').map(|p| p + 1).unwrap_or(0);
    let t = &src[line_start..word_start.saturating_sub(1)];
    let t = t.trim_end();
    let colon = t.rfind(':');
    let arrow = t.rfind("->");
    let (start, is_arrow) = match (colon, arrow) {
        (Some(c), Some(a)) => {
            if a > c {
                (a, true)
            } else {
                (c, false)
            }
        }
        (Some(c), None) => (c, false),
        (None, Some(a)) => (a, true),
        (None, None) => return None,
    };
    let after = if is_arrow { &t[start + 2..] } else { &t[start + 1..] };
    let rest = after.trim();
    if rest.is_empty() || !rest.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'.') {
        return None;
    }
    let rest = rest.trim_end_matches('.');
    if rest.is_empty() {
        return None;
    }
    Some(rest.to_string())
}

/// 类型成员面（P6 D3）：类型标注位 after_dot 出该类型的**类型名**
/// （内部类 / 命名枚举 / const——含 preload 常量），追加全局枚举名。
/// 引擎语义：不出 builtin 类型名与实例成员（oracle index_1_* exclude int）。
/// path 无类型源（枚举名/未知名）→ None（调用方回落，保 P4 门控行为）。
fn type_member_items(state: &State, uri: &Uri, file: &File, path: &str) -> Option<Vec<CompletionItem>> {
    let proj = state.project.as_ref();
    let segs: Vec<&str> = path.split('.').collect();
    // 逐段下钻。终点源：本文件内部类 / 脚本（preload/工程类）/ 引擎类。
    enum Src<'a> {
        Local(&'a ClassDecl),
        ScriptPath(String),
        Engine(String),
    }
    let mut cur: Option<Src> = None;
    for seg in &segs {
        cur = match cur {
            None => {
                if let Some(c) = inner_class(file, seg) {
                    Some(Src::Local(c))
                } else if let Some((_, p)) = preload_consts(file).into_iter().find(|(k, _)| k == *seg) {
                    Some(Src::ScriptPath(p))
                } else if let Some(s) = proj.and_then(|p| p.class_script(seg)) {
                    Some(Src::ScriptPath(format!("res://{}", s.path)))
                } else if state.engine.class_by_name(seg).is_some()
                    || state.engine.builtin_by_name(seg).is_some() {
                        Some(Src::Engine(seg.to_string()))
                    } else {
                        None
                    }
            }
            Some(Src::Local(c)) => c.body.iter().find_map(|st| match st {
                Stmt::Class(cc) if cc.name.name == *seg => Some(Src::Local(cc)),
                _ => None,
            }),
            // 跨脚本/引擎内部类下钻不做（夹具只覆盖本文件下钻）。
            Some(_) => None,
        };
        cur.as_ref()?;
    }
    let mut out: Vec<CompletionItem> = Vec::new();
    match cur {
        Some(Src::Local(c)) => {
            for st in &c.body {
                match st {
                    Stmt::Class(cc) => out.push(CompletionItem {
                        label: cc.name.name.clone(),
                        kind: Some(CompletionItemKind::CLASS),
                        detail: Some("内部类".to_string()),
                        ..CompletionItem::default()
                    }),
                    Stmt::Enum(e) if e.name.is_some() => {
                        let n = e.name.as_ref().expect("checked");
                        out.push(CompletionItem {
                            label: n.name.clone(),
                            kind: Some(CompletionItemKind::ENUM),
                            detail: Some(format!("enum {}", n.name)),
                            ..CompletionItem::default()
                        });
                    }
                    Stmt::Const(cc) => out.push(CompletionItem {
                        label: cc.name.name.clone(),
                        kind: Some(CompletionItemKind::CONSTANT),
                        detail: Some(format!("const {}", cc.name.name)),
                        ..CompletionItem::default()
                    }),
                    _ => {}
                }
            }
        }
        Some(Src::ScriptPath(p)) => {
            if let Some(proj) = proj
                && let Some(key) = proj.script_path_to_key(&p)
                && let Some(script) = proj.resolve_preload(&p) {
                    let _ = key;
                    for st in &script.ast.body {
                        match st {
                            Stmt::Class(cc) => out.push(CompletionItem {
                                label: cc.name.name.clone(),
                                kind: Some(CompletionItemKind::CLASS),
                                detail: Some("内部类".to_string()),
                                ..CompletionItem::default()
                            }),
                            Stmt::Enum(e) if e.name.is_some() => {
                                let n = e.name.as_ref().expect("checked");
                                out.push(CompletionItem {
                                    label: n.name.clone(),
                                    kind: Some(CompletionItemKind::ENUM),
                                    detail: Some(format!("enum {}", n.name)),
                                    ..CompletionItem::default()
                                });
                            }
                            Stmt::Const(cc) => out.push(CompletionItem {
                                label: cc.name.name.clone(),
                                kind: Some(CompletionItemKind::CONSTANT),
                                detail: Some(format!("const {}", cc.name.name)),
                                ..CompletionItem::default()
                            }),
                            _ => {}
                        }
                    }
                }
        }
        Some(Src::Engine(c)) => {
            // 引擎类沿祖先链收枚举/常量名（BaseButton.→ DrawMode/Anchor/
            // TextureRepeat 分散在 BaseButton/Control/CanvasItem——
            // oracle builtin_enum_in_type_hint include 锁定）。
            for cls in state.engine.ancestor_chain(&c) {
                if let Some(ec) = state.engine.class_by_name(cls) {
                    for e in &ec.enums {
                        out.push(CompletionItem {
                            label: e.name.clone(),
                            kind: Some(CompletionItemKind::ENUM),
                            detail: Some(format!("enum {cls}.{}", e.name)),
                            ..CompletionItem::default()
                        });
                    }
                    for k in &ec.constants {
                        out.push(CompletionItem {
                            label: k.name.clone(),
                            kind: Some(CompletionItemKind::CONSTANT),
                            detail: Some(format!("const {cls}.{}", k.name)),
                            ..CompletionItem::default()
                        });
                    }
                }
            }
        }
        None => {}
    }
    // 枚举名追加（引擎类型位行为）：全局枚举 + native 类枚举裸名
    // （ConnectFlags = Object 类枚举，不在 global_enums——与 D2 同数据源）。
    for e in &state.engine.global_enums {
        out.push(CompletionItem {
            label: e.name.clone(),
            kind: Some(CompletionItemKind::ENUM),
            detail: Some("@GlobalScope 枚举".to_string()),
            ..CompletionItem::default()
        });
    }
    let mut enum_names: Vec<&str> = state
        .engine
        .classes
        .values()
        .flat_map(|c| c.enums.iter().map(|e| e.name.as_str()))
        .chain(
            state
                .engine
                .builtins
                .values()
                .flat_map(|b| b.enums.iter().map(|e| e.name.as_str())),
        )
        .collect();
    enum_names.sort_unstable();
    enum_names.dedup();
    for n in enum_names {
        out.push(CompletionItem {
            label: n.to_string(),
            kind: Some(CompletionItemKind::ENUM),
            detail: Some("类枚举".to_string()),
            ..CompletionItem::default()
        });
    }
    let _ = uri;
    Some(out)
}

/// 类型位类作用域类型名（P6）：光标在内部类体内时，该类的 const /
/// 命名枚举 / 内部类名在类型标注位可见（局部遮蔽语义——调用方在全局面
/// 之后追加，同名以类内为准需去重；此处先出，既有面同名项仍并存，
/// oracle 只查 include/exclude 无碍）。
fn class_scope_type_names(state: &State, uri: &Uri, src: &str, at: usize) -> Vec<CompletionItem> {
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    let Some(c) = enclosing_class(&file, at) else {
        return vec![];
    };
    let mut out = Vec::new();
    for st in &c.body {
        match st {
            Stmt::Class(cc) => out.push(CompletionItem {
                label: cc.name.name.clone(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some("内部类".to_string()),
                ..CompletionItem::default()
            }),
            Stmt::Enum(e) if e.name.is_some() => {
                let n = e.name.as_ref().expect("checked");
                out.push(CompletionItem {
                    label: n.name.clone(),
                    kind: Some(CompletionItemKind::ENUM),
                    detail: Some(format!("enum {}", n.name)),
                    ..CompletionItem::default()
                });
            }
            Stmt::Const(cc) => out.push(CompletionItem {
                label: cc.name.name.clone(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some(format!("const {}", cc.name.name)),
                ..CompletionItem::default()
            }),
            _ => {}
        }
    }
    out
}

/// P5：`func ➡` / `static func ➡` 位 override 签名面（脚本父链方法）。
/// display/insert = 签名**源码原样**（含尾冒号与跨行 lambda 缩进——引擎
/// override 行为，oracle override_function_* include 以多行原文锁定）。
/// 光标在内部类内时走该类的 extends（override_function_abstract：顶层
/// `class B extends A`，A 为同文件内部类）；`want_static` = static 位只出
/// static 方法、普通位只出非 static（引擎 override_function_static 语义）。
/// 引擎虚方法（`_get`/`_static_init` 等）归 P7 C6.3 is_virtual 模型化，此处不出。
fn override_signatures(
    state: &State,
    uri: &Uri,
    src: &str,
    at: usize,
    prefix: &str,
    want_static: bool,
) -> Vec<CompletionItem> {
    let mut out = Vec::new();
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    // 光标所在内部类（最内层 span 覆盖光标的顶层 class）。光标在不完整
    // 行尾（`func ` 待补）时 at 可能超过最后声明的 span（解析器 span 止于
    // 最后消费 token）——对 body 最后一个 stmt 放宽到文件尾。
    let enc_name = enclosing_class_name(&file, at);
    let enclosing = file.body.iter().find_map(|st| match st {
        Stmt::Class(c) if enc_name.as_deref() == Some(c.name.name.as_str()) => Some(c.clone()),
        _ => None,
    });
    let mut push_method = |f: &FuncDecl| {
        if f.static_ != want_static {
            return;
        }
        let display = f.signature_text_raw(src);
        out.push(CompletionItem {
            label: display.clone(),
            kind: Some(CompletionItemKind::METHOD),
            detail: Some(if want_static { "override（static，脚本父类）" } else { "override（脚本父类）" }.to_string()),
            insert_text: Some(display),
            sort_text: Some(scoring::sort_text(
                scoring::bucket::SNIPPET,
                scoring::fuzzy(prefix, &f.name.name),
            )),
            ..CompletionItem::default()
        });
    };
    // 当前类（内部类或顶层）已声明的函数名——引擎 override 面跳过
    // （gdscript_editor.cpp:3750 `current_class->has_function(mi.name)`）。
    // **光标所在函数本身排除**（`member != function_node`）：`func _get➡`
    // 时 _get 不算已声明，引擎同名项必须照出。
    let in_current = |s: &(usize, usize)| s.0 <= at && at <= s.1 + 1; // 行尾放宽 1 字节（P5 教训）
    let declared: HashSet<String> = match &enclosing {
        Some(c) => c.body.iter().filter_map(|st| match st {
            Stmt::Func(f) if !in_current(&f.span) => Some(f.name.name.clone()),
            _ => None,
        }).collect(),
        None => file.body.iter().filter_map(|st| match st {
            Stmt::Func(f) if !in_current(&f.span) => Some(f.name.name.clone()),
            _ => None,
        }).collect(),
    };
    if let Some(c) = enclosing {
        // 内部类 override 面：extends 指向同文件内部类 → 收其方法。
        if let Some(Expr::Ident(base)) = &c.extends {
            for st in &file.body {
                if let Stmt::Class(p) = st
                    && p.name.name == base.name
                {
                    for m in &p.body {
                        if let Stmt::Func(f) = m {
                            push_method(f);
                        }
                    }
                }
            }
        }
        // P7 C6.3：引擎虚方法面。内部类 extends 引擎类 → 沿其链；
        // 否则默认 RefCounted（GDScript 无 extends 语义）。core 手表恒叠。
        let host = c
            .extends
            .as_ref()
            .and_then(|e| match e {
                Expr::Ident(id) => Some(id.name.clone()),
                _ => None,
            })
            .filter(|n| state.engine.class_by_name(n).is_some())
            .unwrap_or_else(|| "RefCounted".to_string());
        push_engine_virtuals(state, &host, prefix, want_static, &declared, &mut out);
        return out;
    }
    // 文件级：走工程脚本链。
    let Some(proj) = &state.project else { return out };
    let Some(rp) = uri_res_path(state, uri) else { return out };
    let Some(base) = proj.script_base_key_by_path(&rp) else { return out };
    let mut scripted: HashSet<String> = HashSet::new();
    for m in proj.script_chain_members(&base) {
        if m.kind != gdscript_project::DeclKind::Method {
            continue;
        }
        scripted.insert(m.name.clone());
        if m.is_static != want_static {
            continue;
        }
        let Some(sig) = &m.signature else { continue };
        let display = format!("{sig}:");
        let name = sig.split('(').next().unwrap_or(sig).to_string();
        out.push(CompletionItem {
            label: display.clone(),
            kind: Some(CompletionItemKind::METHOD),
            detail: Some(if want_static { "override（static，脚本父类）" } else { "override（脚本父类）" }.to_string()),
            insert_text: Some(display),
            sort_text: Some(scoring::sort_text(scoring::bucket::SNIPPET, scoring::fuzzy(prefix, &name))),
            ..CompletionItem::default()
        });
    }
    // P7 C6.3：引擎虚方法面（宿主 = 脚本 extends 链尾引擎类；默认 RefCounted）。
    // 脚本父链同名方法覆盖引擎项（近端胜出，与引擎 options.has 语义同向）。
    let host = proj
        .script_engine_base_by_path(&rp)
        .unwrap_or_else(|| "RefCounted".to_string());
    let taken = declared.union(&scripted).cloned().collect::<HashSet<_>>();
    push_engine_virtuals(state, &host, prefix, want_static, &taken, &mut out);
    out
}

/// P7 C6.3：引擎虚方法 override 项（ClassDB is_virtual 模型源 + Object core
/// 手表）。签名形态镜像引擎 `_get_visual_datatype` 渲染：
/// `name(arg: Type, …) -> Ret:`（void 返回恒带 `-> void:`）。
/// `want_static` 位引擎不走 ClassDB——只出特造的 `_static_init`
/// （gdscript_editor.cpp:3735-3739）。`skip` 中的名字跳过（脚本父链 /
/// 当前类已声明胜出，镜像 `options.has(mi.name)` + `has_function`）。
fn push_engine_virtuals(
    state: &State,
    host: &str,
    prefix: &str,
    want_static: bool,
    skip: &HashSet<String>,
    out: &mut Vec<CompletionItem>,
) {
    let mut push = |display: String, detail: &str, name_for_sort: &str| {
        out.push(CompletionItem {
            label: display.clone(),
            kind: Some(CompletionItemKind::METHOD),
            detail: Some(detail.to_string()),
            insert_text: Some(display),
            sort_text: Some(scoring::sort_text(
                scoring::bucket::SNIPPET,
                scoring::fuzzy(prefix, name_for_sort),
            )),
            ..CompletionItem::default()
        });
    };
    if want_static {
        if !skip.contains("_static_init") {
            push("_static_init() -> void:".to_string(), "override（static，引擎）", "_static_init");
        }
        return;
    }
    let detail = "override（引擎虚方法）";
    // 模型源：宿主链 is_virtual 方法（同名先到先得已由 chain_virtuals 收口）。
    for m in state.engine.chain_virtuals(host) {
        if skip.contains(&m.name) {
            continue;
        }
        let args = m
            .arguments
            .iter()
            .map(|p| format!("{}: {}", p.name, p.ty))
            .collect::<Vec<_>>()
            .join(", ");
        push(format!("{}({}) -> {}:", m.name, args, m.return_type), detail, &m.name);
    }
    // Object core 手表：`ClassDB::add_virtual_method` 手工注册（不在
    // extension_api），编辑器构建恒可 override（所有类都有 Object 祖先）。
    for v in gdscript_engine::EngineApi::object_core_virtuals() {
        if skip.contains(v.name) {
            continue;
        }
        let args = v
            .arguments
            .iter()
            .map(|(n, t)| format!("{n}: {t}"))
            .collect::<Vec<_>>()
            .join(", ");
        let ret = v.return_type.unwrap_or("void");
        push(format!("{}({}) -> {ret}:", v.name, args), detail, v.name);
    }
}

/// P5：`extends ➡` 位类型面补充——当前文件兄弟内部类 + 脚本父链内部类
/// （引擎 extends 面出类名但不含脚本内部类；extends_self 排除光标所在类
/// 自身）。引擎类/class_name 脚本类由全局面既有项覆盖。
fn extends_inner_class_items(
    state: &State,
    uri: &Uri,
    src: &str,
    at: usize,
    prefix: &str,
) -> Vec<CompletionItem> {
    use gdscript_project::DeclKind;
    let mut out = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    // 光标所在类自身排除（extends_self）。span 判定与 override 面同款：
    // 最后一个 stmt 放宽到文件尾（不完整行尾 at 超 span.1）。
    let self_name = enclosing_class_name(&file, at);
    // 当前文件兄弟内部类。
    for st in &file.body {
        if let Stmt::Class(c) = st
            && seen.insert(c.name.name.clone())
            && self_name.as_ref() != Some(&c.name.name)
        {
            out.push(CompletionItem {
                label: c.name.name.clone(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some("内部类".to_string()),
                sort_text: Some(scoring::sort_text(
                    scoring::bucket::TYPE,
                    scoring::fuzzy(prefix, &c.name.name),
                )),
                ..CompletionItem::default()
            });
        }
    }
    // 脚本父链内部类（extends_inherited_inner_class：父脚本 class_name 的内部类）。
    if let Some(proj) = &state.project
        && let Some(rp) = uri_res_path(state, uri)
        && let Some(base) = proj.script_base_key_by_path(&rp)
    {
        for m in proj.script_chain_members(&base) {
            if m.kind == DeclKind::Class
                && seen.insert(m.name.clone())
                && self_name.as_ref() != Some(&m.name)
            {
                out.push(CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::CLASS),
                    detail: Some("内部类（脚本父类）".to_string()),
                    sort_text: Some(scoring::sort_text(
                        scoring::bucket::TYPE,
                        scoring::fuzzy(prefix, &m.name),
                    )),
                    ..CompletionItem::default()
                });
            }
        }
    }
    out
}

/// P5：光标所在函数作用域内可直接引用的名字——形参 + 函数体内 var/const
/// 声明（含嵌套块）。LOCAL 桶（最高优先）。
fn local_scope_names(file: &File, src: &str, at: usize) -> Vec<String> {
    let mut out = Vec::new();
    // span 判定放宽（最后一个 stmt 到文件尾——不完整行尾 at 超 span.1）。
    let last_idx = file.body.len().saturating_sub(1);
    let Some(f) = file.body.iter().enumerate().find_map(|(i, st)| match st {
        Stmt::Func(f) if f.span.0 <= at && (at <= f.span.1 || (i == last_idx && at <= src.len())) => Some(f),
        _ => None,
    }) else {
        return out;
    };
    for p in &f.params {
        if !p.name.name.is_empty() {
            out.push(p.name.name.clone());
        }
    }
    fn collect(b: &gdscript_syntax::ast::Block, out: &mut Vec<String>) {
        for st in &b.stmts {
            match st {
                Stmt::Var(v) => out.push(v.name.name.clone()),
                Stmt::Const(c) => out.push(c.name.name.clone()),
                Stmt::If { branches, else_block, .. } => {
                    for (_, blk) in branches {
                        collect(blk, out);
                    }
                    if let Some(e) = else_block {
                        collect(e, out);
                    }
                }
                Stmt::For { body, .. } => collect(body, out),
                Stmt::While { body, .. } => collect(body, out),
                Stmt::Match { arms, .. } => {
                    for a in arms {
                        collect(&a.body, out);
                    }
                }
                _ => {}
            }
        }
    }
    let _ = src;
    collect(&f.body, &mut out);
    out
}

/// `Class.Enum.<|>` / `Enum.<|>`：after_dot 的接收者既不是变量也不是类名时,
/// 按枚举名解析,给出该枚举的值面(label 为裸值名,与引擎成员面一致)。
/// - 两段式 `类.枚举.`:覆盖引擎类与内建类(`Vector2.Axis.X` /
///   `AStarGrid2D.Heuristic.HEURISTIC_MAX`;oracle builtin_enum 桶)。
/// - 单点式 `枚举.`:① 脚本自声明枚举(当前文件顶层 + 类级,遮蔽全局);
///   ② 宿主 extends 链类枚举(`extends Node` → `AutoTranslateMode.` 即
///   Node.AutoTranslateMode,引擎枚举解析沿继承链可见);
///   ③ 引擎全局枚举(`Key.` / `Error.`;oracle `global_enum_values` 与
///   `enum_values_in_match` 两桶——引擎对 match 测试位/分支体无特判,
///   与常规成员位同构)。
///
/// `file` = 当前文档 AST(脚本枚举查找;无则跳过脚本层)。
fn enum_member_values_at(
    state: &State,
    src: &str,
    word_start: usize,
    file: Option<&File>,
) -> Option<Vec<CompletionItem>> {
    let b = src.as_bytes();
    let is_seg = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    if word_start < 2 || b[word_start - 1] != b'.' {
        return None;
    }
    // 枚举名:紧邻 word_start 左侧那个 '.' 前的词。
    let mut e = word_start - 1;
    while e > 0 && is_seg(b[e - 1]) {
        e -= 1;
    }
    let enum_name = &src[e..word_start - 1];
    if enum_name.is_empty() {
        return None;
    }
    let values = if e >= 2 && b[e - 1] == b'.' {
        // 类名:再左侧一个 '.' 前的词。
        let mut c = e - 1;
        while c > 0 && is_seg(b[c - 1]) {
            c -= 1;
        }
        let class_name = &src[c..e - 1];
        let enums = state
            .engine
            .classes
            .get(class_name)
            .map(|k| &k.enums)
            .or_else(|| state.engine.builtins.get(class_name).map(|k| &k.enums))?;
        let en = enums.iter().find(|en| en.name == enum_name)?;
        en.values
            .iter()
            .map(|(n, v)| (n.clone(), *v, format!("const {class_name}.{enum_name}.{n} = {v}")))
            .collect::<Vec<_>>()
    } else {
        // 单点式:脚本自声明枚举 → 宿主链类枚举 → 引擎全局枚举(引擎遮蔽序)。
        if let Some(vals) = file.and_then(|f| script_enum_values(f, enum_name)) {
            vals.into_iter()
                .map(|(n, v)| {
                    let d = format!("const {enum_name}.{n} = {v}");
                    (n, v, d)
                })
                .collect()
        } else if let Some(vals) = file.and_then(|f| host_chain_enum_values(state, f, enum_name)) {
            vals.into_iter()
                .map(|(cls, n, v)| {
                    let d = format!("const {cls}.{enum_name}.{n} = {v}");
                    (n, v, d)
                })
                .collect()
        } else {
            let ge = state.engine.global_enums.iter().find(|en| en.name == enum_name)?;
            ge.values
                .iter()
                .map(|(n, v)| {
                    let d = format!("const {enum_name}.{n} = {v}");
                    (n.clone(), *v, d)
                })
                .collect::<Vec<_>>()
        }
    };
    Some(
        values
            .into_iter()
            .map(|(n, _v, detail)| CompletionItem {
                label: n,
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                detail: Some(detail),
                ..CompletionItem::default()
            })
            .collect(),
    )
}

/// 宿主 extends 链类枚举:`extends Node` 后 `AutoTranslateMode.` 可直接引用
/// (引擎枚举解析沿继承链向上可见;oracle `enum_values_in_match` in_test_1/3)。
/// extends 为脚本路径(`"res://x.gd"`)时当前不追脚本链(N3/P5 面),返回 None。
/// 返回 (定义类名, 值名, 值)。
fn host_chain_enum_values(
    state: &State,
    file: &File,
    name: &str,
) -> Option<Vec<(String, String, i64)>> {
    let host = match &file.extends {
        Some(Expr::Ident(id)) => id.name.clone(),
        _ => return None,
    };
    if state.engine.class_by_name(&host).is_none() && state.engine.builtin_by_name(&host).is_none()
    {
        return None;
    }
    for cls in state.engine.ancestor_chain(&host) {
        let en = if let Some(c) = state.engine.classes.get(cls) {
            c.enums.iter().find(|en| en.name == name)
        } else if let Some(b) = state.engine.builtins.get(cls) {
            b.enums.iter().find(|en| en.name == name)
        } else {
            None
        };
        if let Some(en) = en {
            return Some(
                en.values
                    .iter()
                    .map(|(n, v)| (cls.to_string(), n.clone(), *v))
                    .collect(),
            );
        }
    }
    None
}

/// 枚举条目常量折叠(int 域):字面量 / 一元 ± / 二元算术与位运算。
/// 不可折叠返回 None(调用方跳过该条目,宽容处理)。
fn const_int_value(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::Literal(Literal::Int(v)) => Some(*v),
        Expr::Unary(op, inner, _) => {
            let v = const_int_value(inner)?;
            match op {
                UnOp::Neg => Some(-v),
                UnOp::Pos => Some(v),
                UnOp::BitNot => Some(!v),
                _ => None,
            }
        }
        Expr::Binary(l, op, r, _) => {
            let (a, b) = (const_int_value(l)?, const_int_value(r)?);
            match op {
                BinOp::Add => Some(a + b),
                BinOp::Sub => Some(a - b),
                BinOp::Mul => Some(a * b),
                BinOp::Div => (b != 0).then(|| a / b),
                BinOp::Mod => (b != 0).then(|| a % b),
                BinOp::BitAnd => Some(a & b),
                BinOp::BitOr => Some(a | b),
                BinOp::BitXor => Some(a ^ b),
                BinOp::Shl => u32::try_from(b).ok().and_then(|s| a.checked_shl(s)),
                BinOp::Shr => u32::try_from(b).ok().and_then(|s| a.checked_shr(s)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// 脚本自声明枚举查找:当前文件顶层 `enum X {}` + 类级 `class Y { enum X {} }`
/// (脚本内枚举遮蔽全局;取两层足覆盖 GDScript 惯用法)。值未显式赋值时按
/// 声明序递增值(镜像引擎 EnumDecl 求值)。
fn script_enum_values(file: &File, name: &str) -> Option<Vec<(String, i64)>> {
    fn from_body(body: &[Stmt], name: &str) -> Option<Vec<(String, i64)>> {
        let e = body.iter().find_map(|st| match st {
            Stmt::Enum(e) if e.name.as_ref().map(|n| n.name.as_str()) == Some(name) => Some(e),
            _ => None,
        })?;
        let mut out = Vec::new();
        let mut next = 0i64;
        for (ident, val) in &e.entries {
            match val {
                None => {
                    out.push((ident.name.clone(), next));
                    next += 1;
                }
                Some(expr) => {
                    if let Some(v) = const_int_value(expr) {
                        out.push((ident.name.clone(), v));
                        next = v + 1;
                    }
                }
            }
        }
        Some(out)
    }
    if let Some(v) = from_body(&file.body, name) {
        return Some(v);
    }
    for st in &file.body {
        if let Stmt::Class(c) = st
            && let Some(v) = from_body(&c.body, name)
        {
            return Some(v);
        }
    }
    None
}

/// 枚举值补全是否允许:引擎仅在「值/表达式」上下文出枚举值;
/// **类型标注**(`var x: Enum.`)与**字典键**(`{ Enum. = }`)不出(oracle 两回归
/// 夹具 `index_1_local_enum` / `lua_key_2` 锁定)。返回 false = 抑制枚举值面,
/// 回落 `recv_type_at`/`class_members`(枚举非类,后者为空,与引擎"不出相关项"同向)。
fn enum_value_completion_allowed(src: &str, word_start: usize, at: usize) -> bool {
    let b = src.as_bytes();
    let is_id = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    // 字典键:光标之后首个非空白字符为 '='(`{ Enum. = }`——键赋值)。
    let mut i = at;
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
    }
    if i < b.len() && b[i] == b'=' {
        return false;
    }
    // 类型标注:`var 名 : Enum.` —— 接收者前为 ':' , ':' 前为变量名,变量名前为声明关键字。
    // 接收者起点(跳过紧邻的 '.')。
    let mut r = word_start - 1;
    while r > 0 && is_id(b[r - 1]) {
        r -= 1;
    }
    // r..word_start-1 = 接收者;向左过空白到 ':' 前。
    let mut p = r;
    while p > 0 && (b[p - 1] == b' ' || b[p - 1] == b'\t') {
        p -= 1;
    }
    if p == 0 || b[p - 1] != b':' {
        return true; // 无 ':' → 非类型标注(match/赋值/表达式等)
    }
    // 过 ':' 及其前导空白,取变量名。
    let mut q = p - 1;
    while q > 0 && (b[q - 1] == b' ' || b[q - 1] == b'\t') {
        q -= 1;
    }
    if q == 0 || !is_id(b[q - 1]) {
        return true; // ':' 前非标识符(如字典值 `{"a": Enum.`)→ 仍是值上下文
    }
    let mut v = q;
    while v > 0 && is_id(b[v - 1]) {
        v -= 1;
    }
    let mut c = v;
    while c > 0 && (b[c - 1] == b' ' || b[c - 1] == b'\t') {
        c -= 1;
    }
    if c == 0 {
        return true;
    }
    let mut e = c;
    while e > 0 && is_id(b[e - 1]) {
        e -= 1;
    }
    // 关键词必须是独立词:左侧非标识符(否则 `test_var` 末尾的 `var` 会被误判)。
    let is_standalone_kw = e == 0 || !is_id(b[e - 1]);
    // 命中 `var 名 : 接收者.` 类型标注模式 → 抑制枚举值面(返回 false)。
    let is_type_annotation = is_standalone_kw
        && matches!(
            &src[e..c],
            "var" | "const" | "signal" | "func" | "class" | "enum" | "static"
        );
    !is_type_annotation
}

/// 引擎 internal 属性表（CLASS → [props]）——`PROPERTY_USAGE_INTERNAL`
/// 成员，引擎补全排除（oracle filter/usage_internal：Translation.messages；
/// 源码提取见 tools/gen_internal_props.py，自建派生数据入库）。
pub(crate) fn internal_props() -> &'static HashMap<String, HashSet<String>> {
    static TABLE: std::sync::OnceLock<HashMap<String, HashSet<String>>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        serde_json::from_str::<HashMap<String, Vec<String>>>(include_str!(
            "../../gdscript-engine/testdata/internal_props.json"
        ))
        .unwrap_or_default()
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect()
    })
}

/// 收集某类（含祖先）的方法与属性补全项。方法走 C6.4 `name(…)` 约定。
fn class_members(engine: &EngineApi, class: &str, snippet_support: bool) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();
    let internal = internal_props();
    for cls in engine.ancestor_chain(class) {
        if let Some(c) = engine.classes.get(cls) {
            for m in &c.methods {
                items.push(function_item(
                    &m.name,
                    CompletionItemKind::METHOD,
                    Some(render_function(m)),
                    snippet_support,
                ));
            }
            for p in &c.properties {
                // PROPERTY_USAGE_INTERNAL 成员排除（引擎补全同向，
                // oracle filter/usage_internal 锁定）。
                if internal.get(cls).is_some_and(|s| s.contains(&p.name)) {
                    continue;
                }
                items.push(CompletionItem {
                    label: p.name.clone(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: Some(format!("{}: {}", p.name, p.ty)),
                    ..CompletionItem::default()
                });
            }
            // C6.1：类声明信号（Node.ready / CanvasItem.draw…，dump 503 条，
            // 此前 RawApi 未解析、成员面缺失；引擎成员面同含）。
            for s in &c.signals {
                items.push(CompletionItem {
                    label: s.name.clone(),
                    kind: Some(CompletionItemKind::EVENT),
                    detail: Some(render_signal(s)),
                    ..CompletionItem::default()
                });
            }
            for k in &c.constants {
                items.push(CompletionItem {
                    label: k.name.clone(),
                    kind: Some(CompletionItemKind::CONSTANT),
                    detail: Some(format!("const {}.{} = {}", cls, k.name, const_value_str(&k.value))),
                    ..CompletionItem::default()
                });
            }
            for e in &c.enums {
                items.push(CompletionItem {
                    label: e.name.clone(),
                    kind: Some(CompletionItemKind::ENUM),
                    detail: Some(format!(
                        "enum {} ({})",
                        e.name,
                        e.values.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", ")
                    )),
                    ..CompletionItem::default()
                });
                for (n, v) in &e.values {
                    items.push(CompletionItem {
                        label: n.clone(),
                        kind: Some(CompletionItemKind::ENUM_MEMBER),
                        detail: Some(format!("const {}.{} = {v} ({})", cls, n, e.name)),
                        ..CompletionItem::default()
                    });
                }
            }
        }
    }
    if let Some(b) = engine.builtins.get(class) {
        for m in &b.methods {
            items.push(function_item(
                &m.name,
                CompletionItemKind::METHOD,
                Some(render_function(m)),
                snippet_support,
            ));
        }
        for p in &b.members {
            items.push(CompletionItem {
                label: p.name.clone(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(format!("{}: {}", p.name, p.ty)),
                ..CompletionItem::default()
            });
        }
        for k in &b.constants {
            items.push(CompletionItem {
                label: k.name.clone(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some(format!("const {class}.{} = {}", k.name, const_value_str(&k.value))),
                ..CompletionItem::default()
            });
        }
        for e in &b.enums {
            for (n, v) in &e.values {
                items.push(CompletionItem {
                    label: n.clone(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some(format!("const {class}.{n} = {v} ({})", e.name)),
                    ..CompletionItem::default()
                });
            }
        }
    }
    items
}


// ───────────────────────── documentSymbol ─────────────────────────

/// 侧边栏符号树：把文件顶层声明转成嵌套 DocumentSymbol。
// lsp-types 0.97 将 DocumentSymbol/SymbolInformation 的 deprecated 字段标废但未删除,
// 且结构体无 Default——构造仍需显式 `deprecated: None`。
#[allow(deprecated)]
fn document_symbols(state: &State, p: &DocumentSymbolParams) -> Vec<DocumentSymbol> {
    let uri = &p.text_document.uri;
    let Some(src) = state.docs.get(uri) else {
        return vec![];
    };
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    let body_syms = stmts_symbols(src, &file.body);
    // 若有 class_name，把它包成一个 CLASS 根节点，body 符号作为其 children；
    // 否则直接平铺顶层符号。
    if let Some(cn) = &file.class_name {
        vec![DocumentSymbol {
            name: cn.name.clone(),
            detail: file.extends.as_ref().map(|e| expr_to_text(src, e)),
            kind: SymbolKind::CLASS,
            range: offset_range(src, 0, src.len()),
            selection_range: offset_range(src, cn.span.0, cn.span.1),
            children: Some(body_syms),
            tags: None,
            deprecated: None,
        }]
    } else {
        body_syms
    }
}

fn stmts_symbols(src: &str, body: &[Stmt]) -> Vec<DocumentSymbol> {
    body.iter().filter_map(|s| match s {
        Stmt::Func(f) => Some(func_symbol(src, f)),
        Stmt::Var(v) => Some(var_symbol(src, v)),
        Stmt::Const(c) => Some(const_symbol(src, c)),
        Stmt::Signal(s) => Some(signal_symbol(src, s)),
        Stmt::Enum(e) => Some(enum_symbol(src, e)),
        Stmt::Class(c) => Some(class_symbol(src, c)),
        _ => None,
    }).collect()
}

#[allow(deprecated)]
fn func_symbol(src: &str, f: &FuncDecl) -> DocumentSymbol {
    let params = f.params.iter().map(|p| p.name.name.clone()).collect::<Vec<_>>().join(", ");
    let ret = f.ret_ty.as_ref().map(|r| format!(" -> {}", expr_to_text(src, r))).unwrap_or_default();
    DocumentSymbol {
        name: f.name.name.clone(),
        detail: Some(format!("({params}){ret}")),
        kind: SymbolKind::METHOD,
        range: offset_range(src, f.span.0, f.span.1),
        selection_range: offset_range(src, f.name.span.0, f.name.span.1),
        children: None,
        tags: None,
        deprecated: None,
    }
}

#[allow(deprecated)]
fn var_symbol(src: &str, v: &VarDecl) -> DocumentSymbol {
    let ty = v.ty.as_ref().map(|t| format!(": {}", expr_to_text(src, t))).unwrap_or_default();
    DocumentSymbol {
        name: v.name.name.clone(),
        detail: Some(ty),
        kind: SymbolKind::FIELD,
        range: offset_range(src, v.span.0, v.span.1),
        selection_range: offset_range(src, v.name.span.0, v.name.span.1),
        children: None,
        tags: None,
        deprecated: None,
    }
}

#[allow(deprecated)]
fn const_symbol(src: &str, c: &ConstDecl) -> DocumentSymbol {
    DocumentSymbol {
        name: c.name.name.clone(),
        detail: None,
        kind: SymbolKind::CONSTANT,
        range: offset_range(src, c.span.0, c.span.1),
        selection_range: offset_range(src, c.name.span.0, c.name.span.1),
        children: None,
        tags: None,
        deprecated: None,
    }
}

#[allow(deprecated)]
fn signal_symbol(src: &str, s: &SignalDecl) -> DocumentSymbol {
    let params = s.params.iter().map(|p| p.name.name.clone()).collect::<Vec<_>>().join(", ");
    DocumentSymbol {
        name: s.name.name.clone(),
        detail: Some(format!("({params})")),
        kind: SymbolKind::EVENT,
        range: offset_range(src, s.span.0, s.span.1),
        selection_range: offset_range(src, s.name.span.0, s.name.span.1),
        children: None,
        tags: None,
        deprecated: None,
    }
}

#[allow(deprecated)]
fn enum_symbol(src: &str, e: &EnumDecl) -> DocumentSymbol {
    let children: Vec<DocumentSymbol> = e.entries.iter().map(|(ident, _)| DocumentSymbol {
        name: ident.name.clone(),
        detail: None,
        kind: SymbolKind::ENUM_MEMBER,
        range: offset_range(src, ident.span.0, ident.span.1),
        selection_range: offset_range(src, ident.span.0, ident.span.1),
        children: None,
        tags: None,
        deprecated: None,
    }).collect();
    DocumentSymbol {
        name: e.name.as_ref().map(|n| n.name.clone()).unwrap_or_else(|| "<anonymous enum>".into()),
        detail: None,
        kind: SymbolKind::ENUM,
        range: offset_range(src, e.span.0, e.span.1),
        selection_range: e.name.as_ref().map(|n| offset_range(src, n.span.0, n.span.1)).unwrap_or_else(|| offset_range(src, e.span.0, e.span.1)),
        children: Some(children),
        tags: None,
        deprecated: None,
    }
}

#[allow(deprecated)]
fn class_symbol(src: &str, c: &ClassDecl) -> DocumentSymbol {
    DocumentSymbol {
        name: c.name.name.clone(),
        detail: c.extends.as_ref().map(|e| expr_to_text(src, e)),
        kind: SymbolKind::CLASS,
        range: offset_range(src, c.span.0, c.span.1),
        selection_range: offset_range(src, c.name.span.0, c.name.span.1),
        children: Some(stmts_symbols(src, &c.body)),
        tags: None,
        deprecated: None,
    }
}

// AST/token span → &str 的唯一安全通道：`span_slice`（gdscript-syntax 导出，
// 反转/越界钳制 + UTF-8 字符边界对齐）；表达式粗渲染 `expr_to_text` 同源。
// ───────────────────────── R5.3 signatureHelp ─────────────────────────

fn signature_help(state: &State, p: &SignatureHelpParams) -> Option<SignatureHelp> {
    let uri = &p.text_document_position_params.text_document.uri;
    let src = state.docs.get(uri)?;
    let offset = position_to_offset(src, p.text_document_position_params.position)?;
    let b = src.as_bytes();

    // 向后找最近的未配对 '('。
    let mut depth = 0i32;
    let mut i = offset;
    let open_paren = loop {
        if i == 0 { return None; }
        i -= 1;
        match b[i] {
            b')' => depth += 1,
            b'(' if depth == 0 => break i,
            b'(' => depth -= 1,
            _ => {}
        }
    };

    // '(' 前面的函数调用名（含 recv.method）。
    let mut j = open_paren;
    while j > 0 && (b[j - 1].is_ascii_alphanumeric() || b[j - 1] == b'_' || b[j - 1] == b'.') {
        j -= 1;
    }
    let call = &src[j..open_paren];
    if call.is_empty() {
        return None;
    }

    // 解析 recv.method 或裸名。
    let func = if let Some(dot) = call.rfind('.') {
        let recv = &call[..dot];
        let method = &call[dot + 1..];
        let info = FileInfo::from_file(&parse(src).0);
        let ty = info.resolve_var(src, recv)?;
        state.engine.find_method(&ty, method)?
    } else if state.engine.builtin_by_name(call).is_some() {
        // C6.5c①：内建类型构造调用（`Vector2(` 内）——镜像引擎 call hint
        // （gdscript_editor.cpp:3399-3416：构造重载逐条列出、跳过参数数
        // ≤ 当前参数索引的重载；引擎此处不出补全项）。
        let ctors = state.engine.builtin_constructors(call)?;
        let corpus = hoverdata::builtin_ctors(call);
        let active_param = count_top_commas(&src[open_paren + 1..offset]);
        let signatures: Vec<SignatureInformation> = ctors
            .iter()
            .enumerate()
            .filter(|(_, params)| active_param < params.len())
            .map(|(i, params)| {
                let (label, doc) = corpus
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| (format!("{call}(…)"), String::new()));
                let parameters = if params.is_empty() {
                    None
                } else {
                    Some(
                        params
                            .iter()
                            .map(|p| ParameterInformation {
                                label: lsp_types::ParameterLabel::Simple(format!("{}: {}", p.name, p.ty)),
                                documentation: None,
                            })
                            .collect(),
                    )
                };
                SignatureInformation {
                    label,
                    documentation: if doc.is_empty() { None } else { Some(lsp_types::Documentation::String(doc)) },
                    parameters,
                    active_parameter: Some(active_param as u32),
                }
            })
            .collect();
        if signatures.is_empty() {
            return None;
        }
        return Some(SignatureHelp {
            signatures,
            active_signature: Some(0),
            active_parameter: Some(active_param as u32),
        });
    } else {
        state.engine.utility_function(call)?
    };

    // 统计 open_paren 到光标之间的顶层逗号数 → 当前参数索引。
    let active_param = count_top_commas(&src[open_paren + 1..offset]);
    let label = render_function(func);
    let params = func
        .arguments
        .iter()
        .map(|a| ParameterInformation {
            label: lsp_types::ParameterLabel::Simple(a.name.clone()),
            documentation: None,
        })
        .collect();

    Some(SignatureHelp {
        signatures: vec![SignatureInformation {
            label,
            documentation: None,
            parameters: Some(params),
            active_parameter: Some(active_param as u32),
        }],
        active_signature: Some(0),
        active_parameter: Some(active_param as u32),
    })
}

fn count_top_commas(s: &str) -> usize {
    let mut depth = 0i32;
    let mut count = 0;
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => count += 1,
            _ => {}
        }
    }
    count
}

// ───────────────────────── R6 definition / references / symbol / documentLink ─────────────────────────

fn path_to_uri(path: &std::path::Path) -> Option<Uri> {
    let url = url::Url::from_file_path(path).ok()?;
    url.as_str().parse::<Uri>().ok()
}

fn root_file_uri(root: &std::path::Path, rel: &str) -> Option<Uri> {
    path_to_uri(&root.join(rel))
}

fn goto_definition(state: &State, p: &GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
    let uri = &p.text_document_position_params.text_document.uri;
    let src = state.docs.get(uri)?;
    let offset = position_to_offset(src, p.text_document_position_params.position)?;
    let word = word_at(src, offset)?;

    // 1. 项目 class_name → 跳转脚本文件。
    if let (Some(proj), Some(root)) = (&state.project, &state.root)
        && let Some(script) = proj.class_name_script(&word)
            && let Some(target) = root_file_uri(root, &script.path) {
                return Some(GotoDefinitionResponse::Scalar(Location {
                    uri: target,
                    range: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } },
                }));
            }

    // 2. 当前文件内局部声明（func/var/const/signal/class/enum）。
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    for stmt in &file.body {
        let hit = match stmt {
            Stmt::Func(f) if f.name.name == word => Some((f.name.span.0, f.name.span.1)),
            Stmt::Var(v) if v.name.name == word => Some((v.name.span.0, v.name.span.1)),
            Stmt::Const(c) if c.name.name == word => Some((c.name.span.0, c.name.span.1)),
            Stmt::Signal(s) if s.name.name == word => Some((s.name.span.0, s.name.span.1)),
            Stmt::Class(c) if c.name.name == word => Some((c.name.span.0, c.name.span.1)),
            Stmt::Enum(e) if e.name.as_ref().map(|n| n.name == word).unwrap_or(false) => {
                Some((e.name.as_ref().unwrap().span.0, e.name.as_ref().unwrap().span.1))
            }
            _ => None,
        };
        if let Some((s, e)) = hit {
            return Some(GotoDefinitionResponse::Scalar(Location {
                uri: uri.clone(),
                range: offset_range(src, s, e),
            }));
        }
    }

    // 3. 引擎类/内建/全局函数/单例/常量 → 本地文档站 URL（§8.4-1 修正）。
    let real = |b: &str| state.real_lower.contains(b);
    let doc_url: Option<String> = if state.engine.class_by_name(&word).is_some()
        || state.engine.builtin_by_name(&word).is_some()
    {
        Some(doclinks::class_url(&word, real))
    } else if state.engine.utility_function(&word).is_some() {
        Some(doclinks::member_url("@GlobalScope", &word, false, real))
    } else { state.engine.singleton_type(&word).map(|ty| doclinks::class_url(ty, real)) };
    if let Some(url) = doc_url
        && let Ok(target_uri) = url.parse::<Uri>() {
            return Some(GotoDefinitionResponse::Scalar(Location {
                uri: target_uri,
                range: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } },
            }));
        }
    None
}

fn find_references(state: &State, p: &ReferenceParams) -> Option<Vec<Location>> {
    let uri = &p.text_document_position.text_document.uri;
    let src = state.docs.get(uri)?;
    let offset = position_to_offset(src, p.text_document_position.position)?;
    let word = word_at(src, offset)?;
    let mut locs = Vec::new();

    // 当前文档。
    collect_word(src, &word, uri, &mut locs);

    // 项目其他脚本。
    if let (Some(proj), Some(root)) = (&state.project, &state.root) {
        for script in &proj.scripts {
            let path = root.join(&script.path);
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            if let Some(target) = path_to_uri(&path) {
                collect_word(&text, &word, &target, &mut locs);
            }
        }
    }
    Some(locs)
}

/// 词边界扫描：src 中所有与 word 全词匹配的 (start, end) 偏移。
/// references 与 rename 共用此实现（P2.8：两者必须同源，rename 后跳转结果才一致）。
fn collect_spans(src: &str, word: &str) -> Vec<(usize, usize)> {
    let b = src.as_bytes();
    let wb = word.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + wb.len() <= b.len() {
        if &b[i..i + wb.len()] == wb {
            // 确认前后不是标识符字符（词边界）。
            let before_ok = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
            let after_end = i + wb.len();
            let after_ok = after_end >= b.len() || !(b[after_end].is_ascii_alphanumeric() || b[after_end] == b'_');
            if before_ok && after_ok {
                out.push((i, after_end));
            }
        }
        i += 1;
    }
    out
}

fn collect_word(src: &str, word: &str, uri: &Uri, out: &mut Vec<Location>) {
    for (s, e) in collect_spans(src, word) {
        out.push(Location {
            uri: uri.clone(),
            range: offset_range(src, s, e),
        });
    }
}

#[allow(deprecated)]
fn workspace_symbol(state: &State, query: &str) -> Vec<SymbolInformation> {
    let mut syms = Vec::new();
    if let (Some(proj), Some(root)) = (&state.project, &state.root) {
        for script in &proj.scripts {
            if let Some(cn) = &script.ast.class_name
                && (query.is_empty() || cn.name.to_lowercase().contains(&query.to_lowercase()))
                    && let Some(uri) = root_file_uri(root, &script.path) {
                        syms.push(SymbolInformation {
                            name: cn.name.clone(),
                            kind: SymbolKind::CLASS,
                            location: Location {
                                uri,
                                range: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } },
                            },
                            container_name: None,
                            tags: None,
                            deprecated: None,
                        });
                    }
        }
    }
    syms
}

fn document_links(state: &State, p: &DocumentLinkParams) -> Vec<DocumentLink> {
    let uri = &p.text_document.uri;
    let Some(src) = state.docs.get(uri) else { return vec![] };
    let root = match &state.root { Some(r) => r, None => return vec![] };
    let mut links = Vec::new();
    let b = src.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'"' {
            let start = i;
            i += 1;
            while i < b.len() && b[i] != b'"' { i += 1; }
            let end = i;
            let literal = &src[start + 1..end];
            if let Some(res_path) = literal.strip_prefix("res://") {
                let path = root.join(res_path);
                if path.exists()
                    && let Some(target) = path_to_uri(&path) {
                        links.push(DocumentLink {
                            range: offset_range(src, start, end + 1),
                            target: Some(target),
                            tooltip: None,
                            data: None,
                        });
                    }
            }
            // uid:// → res:// 映射（§8.1 UID 规格）；tscn/tres 头部自嵌 uid 指向自己，跳过。
            // 用路径等价比较而非 Uri 字符串（客户端盘符大小写与 url crate 生成可能不一致）。
            if let Some(uid) = literal.strip_prefix("uid://")
                && let Some(res_path) = state.uid_map.get(uid) {
                    let path = root.join(res_path);
                    let self_path = url::Url::parse(uri.as_str()).ok().and_then(|u| u.to_file_path().ok());
                    if path.exists()
                        && self_path.as_deref() != Some(path.as_path())
                        && let Some(target) = path_to_uri(&path) {
                            links.push(DocumentLink {
                                range: offset_range(src, start, end + 1),
                                target: Some(target),
                                tooltip: Some(format!("uid://{}", uid)),
                                data: None,
                            });
                        }
                }
            i = end + 1;
        } else {
            i += 1;
        }
    }
    links
}

// ───────────────────────── P2.9 documentColor ─────────────────────────

/// 解析 `a, b, c[, d]` 参数串为 RGBA；全部为数字字面量才成功（变量一律不猜，零误报优先）。
fn parse_color_args(args: &str) -> Option<[f32; 4]> {
    let mut comps = Vec::new();
    for a in args.split(',') {
        let t = a.trim().trim_end_matches(',');
        let body = t.strip_prefix(['-', '+']).unwrap_or(t);
        if body.is_empty() || !body.chars().all(|c| c.is_ascii_digit() || c == '.') || body == "." {
            return None;
        }
        comps.push(t.parse::<f32>().ok()?);
    }
    match comps.len() {
        3 => Some([comps[0], comps[1], comps[2], 1.0]),
        4 => Some([comps[0], comps[1], comps[2], comps[3]]),
        _ => None,
    }
}

/// 引擎 Color 命名常量表：NAME -> RGBA（首访构建，值形如 "Color(r, g, b, a)"）。
fn color_named_constants(engine: &EngineApi) -> &'static HashMap<String, [f32; 4]> {
    static TABLE: std::sync::OnceLock<HashMap<String, [f32; 4]>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        let mut m = HashMap::new();
        if let Some(color) = engine.builtins.get("Color") {
            for c in &color.constants {
                if let ConstValue::String(s) = &c.value
                    && let Some(inner) = s.strip_prefix("Color(").and_then(|s| s.strip_suffix(')'))
                    && let Some(rgba) = parse_color_args(inner) {
                        m.insert(c.name.clone(), rgba);
                    }
            }
        }
        m
    })
}

/// 扫描 .gd 文本中的颜色：`Color(r,g,b[,a])` 构造 + `Color.NAME` 命名常量。
/// 词法级扫描（不做 AST 下钻），字符串/注释中的命中一并产出（装饰性，无害）。
fn document_color(state: &State, p: &DocumentColorParams) -> Vec<ColorInformation> {
    let uri = &p.text_document.uri;
    let Some(src) = state.docs.get(uri) else { return vec![] };
    let named = color_named_constants(&state.engine);
    let b = src.as_bytes();
    let mut out = Vec::new();

    let mut i = 0;
    while i < b.len() {
        // 词边界匹配 "Color"
        if b[i] == b'C' && src[i..].starts_with("Color") {
            let boundary_before =
                i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
            if boundary_before {
                let after = &src[i + 5..];
                if let Some(rest) = after.strip_prefix('.') {
                    // Color.NAME 命名常量
                    let name: String =
                        rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
                    if let Some(&rgba) = named.get(&name) {
                        let end = i + 5 + 1 + name.len();
                        out.push(ColorInformation {
                            range: offset_range(src, i, end),
                            color: Color { red: rgba[0], green: rgba[1], blue: rgba[2], alpha: rgba[3] },
                        });
                    }
                } else if let Some(rest) = after.strip_prefix('(') {
                    // Color( 构造调用
                    if let Some(close) = rest.find(')')
                        && let Some(rgba) = parse_color_args(&rest[..close]) {
                            let end = i + 5 + 1 + close + 1;
                            out.push(ColorInformation {
                                range: offset_range(src, i, end),
                                color: Color { red: rgba[0], green: rgba[1], blue: rgba[2], alpha: rgba[3] },
                            });
                        }
                }
            }
            i += 5;
        } else {
            i += 1;
        }
    }
    out
}

/// colorPresentation：把色值回写为 GDScript 构造文本。
fn color_presentation(p: &ColorPresentationParams) -> Vec<ColorPresentation> {
    let c = p.color;
    let label = format!("Color({}, {}, {}, {})", c.red, c.green, c.blue, c.alpha);
    vec![ColorPresentation {
        label,
        text_edit: Some(TextEdit {
            range: p.range,
            new_text: format!("Color({:.6}, {:.6}, {:.6}, {:.6})", c.red, c.green, c.blue, c.alpha),
        }),
        additional_text_edits: None,
    }]
}

// ───────────────────────── P2.10 codeLens（场景信号连接） ─────────────────────────

/// tscn [connection] → 脚本方法声明行上方的连接数 lens。
/// 仅统计 ext_resource 引用本脚本且四字段完整的 connection；命令跳转 tscn 连接行。
fn code_lens(state: &State, p: &CodeLensParams) -> Vec<CodeLens> {
    let uri = &p.text_document.uri;
    let Some(src) = state.docs.get(uri) else { return vec![] };
    let (Some(root), Some(project)) = (&state.root, &state.project) else { return vec![] };

    // uri → res:// 相对路径。
    let Some(path) = url::Url::parse(uri.as_str()).ok().and_then(|u| u.to_file_path().ok()) else {
        return vec![];
    };
    let Ok(rel) = path.strip_prefix(root) else { return vec![] };
    let res_path = format!("res://{}", rel.to_string_lossy().replace('\\', "/"));

    // method 名 → (场景, 连接) 列表（只看引用了本脚本的场景）。
    let mut by_method: HashMap<&str, Vec<(&gdscript_project::tscn::SceneFile, &gdscript_project::tscn::Connection)>> = HashMap::new();
    for scene in &project.scenes {
        let refs_me = scene.ext_resources.iter().any(|e| e.path == res_path);
        if !refs_me {
            continue;
        }
        for c in &scene.connections {
            if c.complete {
                by_method.entry(c.method.as_str()).or_default().push((scene, c));
            }
        }
    }
    if by_method.is_empty() {
        return vec![];
    }

    // 顶层 func 声明行 → lens。
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| parse(src).0);
    let mut lenses = Vec::new();
    for stmt in &file.body {
        let Stmt::Func(f) = stmt else { continue };
        let Some(conns) = by_method.get(f.name.name.as_str()) else { continue };
        let line = offset_to_position(src, f.span.0).line;
        let (scene, conn) = conns[0];
        let target = path_to_uri(&root.join(&scene.path));
        lenses.push(CodeLens {
            range: Range {
                start: Position { line, character: 0 },
                end: Position { line, character: 0 },
            },
            command: target.map(|t| Command {
                title: if conns.len() == 1 {
                    "1 个场景连接".into()
                } else {
                    format!("{} 个场景连接", conns.len())
                },
                command: "godotlsp.openConnection".into(),
                arguments: Some(vec![
                    serde_json::Value::String(t.as_str().to_string()),
                    serde_json::json!(conn.line),
                ]),
            }),
            data: None,
        });
    }
    lenses
}

// ───────────────────────── P2.8 rename（跨文件,词法级与 references 同源） ─────────────────────────

/// 词属于引擎/语言保留命名空间 → 不可 rename。
fn is_reserved_name(state: &State, word: &str) -> bool {
    state.engine.class_by_name(word).is_some()
        || state.engine.builtin_by_name(word).is_some()
        || state.engine.singletons.iter().any(|s| s.name == word)
        || state.engine.utility_function(word).is_some()
        || state.engine.global_enum_value(word).is_some()
        || state.engine.global_constants.iter().any(|c| c.name == word)
        || langdocs::keywords().iter().any(|(k, _, _)| *k == word)
        || hoverdata::gdscript_fn(word).is_some()
        || hoverdata::annotation(word).is_some()
}

/// 新名是否为合法 GDScript 标识符（PascalCase/SnakeCase 由调用方自便，只卡字符集）。
fn is_valid_ident(s: &str) -> bool {
    let mut ch = s.chars();
    matches!(ch.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && ch.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// prepareRename：光标词可 rename 时返回词范围 + placeholder。
/// 保留名（引擎类/单例/内建函数/关键字/注解/@GDScript 函数/全局常量枚举）返回 None，
/// 客户端即不触发 rename UI——第一道零误报闸门。
fn prepare_rename(state: &State, p: &TextDocumentPositionParams) -> Option<PrepareRenameResponse> {
    let src = state.docs.get(&p.text_document.uri)?;
    let offset = position_to_offset(src, p.position)?;
    let (start, word) = word_span(src, offset)?;
    if is_reserved_name(state, &word) {
        return None;
    }
    Some(PrepareRenameResponse::RangeWithPlaceholder {
        range: offset_range(src, start, start + word.len()),
        placeholder: word,
    })
}

/// rename：重命名打开文档 + 项目全部 .gd 中的全词匹配（词法级，与 references 同源）。
/// 只改 .gd——tscn/tres 引用脚本靠 path 而非符号名，改类型字段反而破坏资源文件。
/// 已知限制（词法级）:同名不同作用域符号会被一并改写（与 find_references 一致）。
fn rename(state: &State, p: &RenameParams) -> Result<WorkspaceEdit, String> {
    let uri = &p.text_document_position.text_document.uri;
    let src = state.docs.get(uri).ok_or_else(|| "文档未打开".to_string())?;
    let offset = position_to_offset(src, p.text_document_position.position)
        .ok_or_else(|| "光标不在标识符上".to_string())?;
    let (_, old) = word_span(src, offset).ok_or_else(|| "光标不在标识符上".to_string())?;
    if is_reserved_name(state, &old) {
        return Err(format!("\"{old}\" 是引擎/语言保留名，不可重命名"));
    }
    if !is_valid_ident(&p.new_name) {
        return Err(format!("\"{}\" 不是合法的 GDScript 标识符", p.new_name));
    }
    if p.new_name == old {
        return Err("新名与旧名相同".to_string());
    }
    if is_reserved_name(state, &p.new_name) {
        return Err(format!("\"{}\" 与引擎/语言保留名冲突", p.new_name));
    }

    // 同上:Uri 作 key 的 mutable_key_type 为 lsp-types 内部表示误报。
    #[allow(clippy::mutable_key_type)]
    let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();
    // 打开文档：以编辑器缓冲（state.docs）为准。
    let self_path = url::Url::parse(uri.as_str()).ok().and_then(|u| u.to_file_path().ok());
    #[allow(clippy::mutable_key_type)]
    let push_edits = |text: &str, u: &Uri, changes: &mut HashMap<Uri, Vec<TextEdit>>| {
        let edits: Vec<TextEdit> = collect_spans(text, &old)
            .into_iter()
            .map(|(s, e)| TextEdit { range: offset_range(text, s, e), new_text: p.new_name.clone() })
            .collect();
        if !edits.is_empty() {
            changes.insert(u.clone(), edits);
        }
    };
    push_edits(src, uri, &mut changes);
    // 项目内其他 .gd：磁盘文本。
    if let (Some(proj), Some(root)) = (&state.project, &state.root) {
        for script in &proj.scripts {
            let path = root.join(&script.path);
            if self_path.as_deref() == Some(path.as_path()) {
                continue;
            }
            if let Ok(text) = std::fs::read_to_string(&path)
                && let Some(target) = path_to_uri(&path) {
                    push_edits(&text, &target, &mut changes);
                }
        }
    }
    Ok(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
}

// ───────────────────────── inlayHints / foldingRange ─────────────────────────

fn inlay_hints(state: &State, p: &InlayHintParams) -> Vec<InlayHint> {
    if !state.config.inlay_type_hints {
        return vec![];
    }
    let uri = &p.text_document.uri;
    let Some(file) = state.asts.get(uri) else { return vec![] };
    let src = state.docs.get(uri).map(|s| s.as_str()).unwrap_or("");
    let mut hints = Vec::new();
    // 对每个顶层 var，如果没有类型注解但有初始值，推断类型并显示。
    for stmt in &file.body {
        if let gdscript_syntax::ast::Stmt::Var(v) = stmt
            && v.ty.is_none()
                && let Some(value) = &v.value {
                    let ty = infer_lit_type(value);
                    if !ty.is_empty() {
                        let pos = offset_to_position(src, v.name.span.1);
                        hints.push(InlayHint {
                            position: pos,
                            label: InlayHintLabel::String(format!(": {}", ty)),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        });
                    }
                }
    }
    hints
}

fn infer_lit_type(e: &gdscript_syntax::ast::Expr) -> &'static str {
    use gdscript_syntax::ast::*;
    match e {
        Expr::Literal(Literal::Int(_)) => "int",
        Expr::Literal(Literal::Float(_)) => "float",
        Expr::Literal(Literal::Str(_)) => "String",
        Expr::Literal(Literal::Bool(_)) => "bool",
        Expr::Array(_, _) => "Array",
        Expr::Dict(_, _) => "Dictionary",
        _ => "",
    }
}

fn folding_ranges(state: &State, p: &FoldingRangeParams) -> Vec<FoldingRange> {
    let uri = &p.text_document.uri;
    let Some(file) = state.asts.get(uri) else { return vec![] };
    let src = state.docs.get(uri).map(|s| s.as_str()).unwrap_or("");
    let mut ranges = Vec::new();
    for stmt in &file.body {
        let span = match stmt {
            gdscript_syntax::ast::Stmt::Func(f) => Some(f.span),
            gdscript_syntax::ast::Stmt::Class(c) => Some(c.span),
            gdscript_syntax::ast::Stmt::If { branches, .. } => {
                branches.iter().map(|(_, b)| (b.span.0, b.span.1)).next()
            }
            gdscript_syntax::ast::Stmt::For { body, .. } => Some((body.span.0, body.span.1)),
            gdscript_syntax::ast::Stmt::While { body, .. } => Some((body.span.0, body.span.1)),
            gdscript_syntax::ast::Stmt::Match { arms, .. } => {
                arms.iter().map(|a| (a.body.span.0, a.body.span.1)).next()
            }
            _ => None,
        };
        if let Some((start, end)) = span {
            let start_pos = offset_to_position(src, start);
            let end_pos = offset_to_position(src, end);
            ranges.push(FoldingRange {
                start_line: start_pos.line,
                start_character: Some(start_pos.character),
                end_line: end_pos.line,
                end_character: Some(end_pos.character),
                kind: Some(FoldingRangeKind::Region),
                collapsed_text: None,
            });
        }
    }
    ranges
}

// ───────────────────────── documentHighlight ─────────────────────────

fn document_highlight(state: &State, p: &DocumentHighlightParams) -> Vec<DocumentHighlight> {
    let uri = &p.text_document_position_params.text_document.uri;
    let Some(src) = state.docs.get(uri) else { return vec![] };
    let offset = position_to_offset(src, p.text_document_position_params.position).unwrap_or(0);
    let Some(word) = word_at(src, offset) else { return vec![] };
    let mut out = Vec::new();
    let b = src.as_bytes();
    let wb = word.as_bytes();
    // 赋值算符集合（`=` 与复合赋值;`==`/`!=`/`<=` 等比较不算写）。
    let is_assign_at = |i: usize| -> bool {
        let rest = &src[i..];
        for op in ["=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>="] {
            if rest.starts_with(op) {
                // `==` 是比较:排除。
                if op == "=" && rest[1..].starts_with('=') {
                    return false;
                }
                return true;
            }
        }
        false
    };
    // 按字符推进(避免切进多字节字符 panic);逐词匹配 + 词边界判定。
    let mut i = 0;
    while i + wb.len() <= b.len() {
        if &b[i..i + wb.len()] == wb {
            let before_ok = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
            let after_end = i + wb.len();
            let after_ok = after_end >= b.len() || !(b[after_end].is_ascii_alphanumeric() || b[after_end] == b'_');
            if before_ok && after_ok {
                let mut j = after_end;
                while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
                    j += 1;
                }
                let kind = if j < b.len() && is_assign_at(j) {
                    DocumentHighlightKind::WRITE
                } else {
                    DocumentHighlightKind::READ
                };
                out.push(DocumentHighlight {
                    range: offset_range(src, i, after_end),
                    kind: Some(kind),
                });
            }
        }
        match src[i..].chars().next() {
            Some(c) => i += c.len_utf8(),
            None => break,
        }
    }
    out
}

// ───────────────────────── 语义着色（符号颜色匹配/区分） ─────────────────────────
// 服务器下发语义 token，按标准 LSP 语义类型上色，使函数/类型/变量/常量/参数等
// 在任意主题下都有稳定且可区分的颜色。TextMate 语法只能给粗略作用域、且无法按
// 语义区分「成员类型 vs 局部变量」，语义 token 填补这一层；括号彩虹色由编辑器基于
// language-configuration 的 brackets 负责，这里不覆盖运算符/标点，避免冲突。

#[derive(Clone, Copy, PartialEq, Eq)]
enum SemKind {
    Keyword,
    Function,
    Method,
    Class,
    Type,
    Variable,
    Parameter,
    Property,
    EnumMember,
    Constant,
    String,
    Number,
    Comment,
    Operator,
    Macro,
}

/// 图例顺序即 token 类型下标，必须稳定。标准 LSP 类型名，主题天然着色。
const SEM_LEGEND: &[(SemKind, &str)] = &[
    (SemKind::Keyword, "keyword"),
    (SemKind::Function, "function"),
    (SemKind::Method, "method"),
    (SemKind::Class, "class"),
    (SemKind::Type, "type"),
    (SemKind::Variable, "variable"),
    (SemKind::Parameter, "parameter"),
    (SemKind::Property, "property"),
    (SemKind::EnumMember, "enumMember"),
    (SemKind::Constant, "constant"),
    (SemKind::String, "string"),
    (SemKind::Number, "number"),
    (SemKind::Comment, "comment"),
    (SemKind::Operator, "operator"),
    (SemKind::Macro, "macro"),
];

fn semtok_legend_types() -> Vec<SemanticTokenType> {
    SEM_LEGEND.iter().map(|(_, s)| SemanticTokenType::new(s)).collect()
}

fn semtok_index(k: SemKind) -> u32 {
    SEM_LEGEND.iter().position(|(kk, _)| *kk == k).unwrap() as u32
}

/// 计算某文档的全量语义 token。基于词法 token + AST 声明名区间上色，再 delta 编码。
fn compute_semantic_tokens(src: &str) -> SemanticTokens {
    use gdscript_syntax::token::TokenKind as K;
    let tokens = gdscript_syntax::tokenize(src);
    let (file, _errs) = gdscript_syntax::parse(src);

    // 声明名区间 -> 精确语义类型，覆盖词法启发式。
    let mut special: HashMap<(usize, usize), SemKind> = HashMap::new();
    collect_decl_spans(&file, &mut special);

    let mut rows: Vec<(u32, u32, u32, u32)> = Vec::new(); // (line, col_utf16, len_utf16, type_idx)
    for (i, t) in tokens.iter().enumerate() {
        match t.kind {
            K::Newline | K::Indent | K::Dedent | K::Eof | K::Error => continue,
            _ => {}
        }
        let kind = if let Some(sk) = special.get(&(t.start, t.end)) {
            *sk
        } else {
            classify_token(t, i, &tokens, src)
        };
        // 运算符/标点不覆盖编辑器括号彩虹色与 TextMate 着色。
        if kind == SemKind::Operator {
            continue;
        }
        let (line, col) = utf16_line_col(src, t.start);
        let len = utf16_len(src, t.start, t.end);
        rows.push((line, col, len, semtok_index(kind)));
    }

    // 按 (line, col) 排序后做相对增量编码（LSP 要求）。
    rows.sort_by_key(|a| (a.0, a.1));
    let mut data = Vec::with_capacity(rows.len());
    let mut prev_line = 0u32;
    let mut prev_col = 0u32;
    for (line, col, len, ti) in rows {
        let delta_line = line - prev_line;
        let delta_col = if delta_line == 0 { col - prev_col } else { col };
        data.push(SemanticToken {
            delta_line,
            delta_start: delta_col,
            length: len,
            token_type: ti,
            token_modifiers_bitset: 0,
        });
        prev_line = line;
        prev_col = col;
    }
    SemanticTokens { result_id: None, data }
}

fn classify_token(
    t: &gdscript_syntax::token::Token,
    i: usize,
    tokens: &[gdscript_syntax::token::Token],
    src: &str,
) -> SemKind {
    use gdscript_syntax::token::TokenKind as K;
    match t.kind {
        K::String => SemKind::String,
        K::Int | K::Float => SemKind::Number,
        K::Ident => {
            let text = &src[t.start..t.end];
            let next = tokens.get(i + 1).map(|x| x.kind);
            let prev = if i > 0 { tokens.get(i - 1).map(|x| x.kind) } else { None };
            // 调用：标识符后紧跟 `(`，`.x(` 视为方法，裸 `x(` 视为函数。
            if matches!(next, Some(K::LParen)) {
                return if matches!(prev, Some(K::Dot)) { SemKind::Method } else { SemKind::Function };
            }
            // 类型：PascalCase（首字母大写且含小写）。
            if is_pascal_case(text) {
                return SemKind::Type;
            }
            // 常量：全大写 + 含 `_` 或纯大写长度 > 1。
            if is_upper_case(text) {
                return SemKind::Constant;
            }
            SemKind::Variable
        }
        _ => SemKind::Operator,
    }
}

fn is_pascal_case(s: &str) -> bool {
    let mut has_lower = false;
    let mut has_upper = false;
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            has_upper = true;
        } else if c.is_ascii_lowercase() {
            has_lower = true;
        } else if !c.is_ascii_alphanumeric() {
            return false;
        }
    }
    has_upper && has_lower
}

fn is_upper_case(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
        && s.chars().any(|c| c.is_ascii_uppercase())
}

fn collect_decl_spans(file: &gdscript_syntax::ast::File, out: &mut HashMap<(usize, usize), SemKind>) {
    if let Some(cn) = &file.class_name {
        out.insert(cn.span, SemKind::Class);
    }
    for s in &file.body {
        collect_stmt(s, out);
    }
}

fn collect_stmt(s: &gdscript_syntax::ast::Stmt, out: &mut HashMap<(usize, usize), SemKind>) {
    use gdscript_syntax::ast::Stmt;
    match s {
        Stmt::Func(f) => {
            out.insert(f.name.span, SemKind::Function);
            for p in &f.params {
                out.insert(p.name.span, SemKind::Parameter);
            }
            collect_block(&f.body, out);
        }
        Stmt::Var(v) => {
            out.insert(v.name.span, SemKind::Variable);
            if let Some(pb) = &v.property {
                if let Some(acc) = &pb.get {
                    collect_accessor(acc, out);
                }
                if let Some(acc) = &pb.set {
                    collect_accessor(acc, out);
                }
            }
        }
        Stmt::Const(c) => {
            out.insert(c.name.span, SemKind::Constant);
        }
        Stmt::Signal(sg) => {
            out.insert(sg.name.span, SemKind::Method);
        }
        Stmt::Enum(e) => {
            if let Some(name) = &e.name {
                out.insert(name.span, SemKind::EnumMember);
            }
            for (name, _) in &e.entries {
                out.insert(name.span, SemKind::EnumMember);
            }
        }
        Stmt::Class(c) => {
            out.insert(c.name.span, SemKind::Class);
            for st in &c.body {
                collect_stmt(st, out);
            }
        }
        Stmt::If { branches, else_block, .. } => {
            for (_, b) in branches {
                collect_block(b, out);
            }
            if let Some(b) = else_block {
                collect_block(b, out);
            }
        }
        Stmt::For { vars, body, .. } => {
            for v in vars {
                out.insert(v.span, SemKind::Variable);
            }
            collect_block(body, out);
        }
        Stmt::While { body, .. } => collect_block(body, out),
        Stmt::Match { arms, .. } => {
            for a in arms {
                collect_block(&a.body, out);
            }
        }
        Stmt::Expr(e) => collect_expr(e, out),
        _ => {}
    }
}

fn collect_accessor(acc: &gdscript_syntax::ast::Accessor, out: &mut HashMap<(usize, usize), SemKind>) {
    match acc {
        gdscript_syntax::ast::Accessor::Inline { param, body } => {
            if let Some(p) = param {
                out.insert(p.span, SemKind::Parameter);
            }
            collect_block(body, out);
        }
        gdscript_syntax::ast::Accessor::External { func } => {
            out.insert(func.span, SemKind::Function);
        }
    }
}

fn collect_block(b: &gdscript_syntax::ast::Block, out: &mut HashMap<(usize, usize), SemKind>) {
    for s in &b.stmts {
        collect_stmt(s, out);
    }
}

fn collect_expr(_e: &gdscript_syntax::ast::Expr, _out: &mut HashMap<(usize, usize), SemKind>) {
    // 表达式内的标识符已由词法启发式覆盖（调用/类型/常量/变量），不递归以免重复误判。
}

/// 字节偏移 -> (行号, UTF-16 列)，与 position_encoding=UTF16 对齐。
fn utf16_line_col(src: &str, byte: usize) -> (u32, u32) {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, c) in src.char_indices() {
        if i >= byte {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += c.len_utf16() as u32;
        }
    }
    (line, col)
}

fn utf16_len(src: &str, start: usize, end: usize) -> u32 {
    span_slice(src, start, end).chars().map(|c| c.len_utf16() as u32).sum()
}

// ───────────────────────── 文本工具 ─────────────────────────

fn url_to_path(u: &Uri) -> Option<std::path::PathBuf> {
    let s = u.as_str();
    let path = s.strip_prefix("file://")?;
    let path = path.strip_prefix('/').unwrap_or(path); // Windows: /D:/… → D:/…
    // URL-decode：部分 LSP 客户端会把 `:`（%3A）、空格（%20）等编码进路径，
    // 不解会令 Windows 路径无效 → read_dir 静默失败 → 扫描到 0 文件。
    Some(std::path::PathBuf::from(percent_decode(path)))
}

/// 对路径做最小化的 URL 百分号解码（%XX → 字节），非法序列原样保留。
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len()
            && let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'A'..=b'F' => Some(b - b'A' + 10),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}

/// 向上查找包含 `project.godot` 的最近祖先目录（含自身）；找不到返回 None。
/// 用于「打开的是项目子目录/父目录」时仍能定位真实项目根，加载引擎模型与版本对齐。
fn locate_project_root(start: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut cur = Some(start.to_path_buf());
    while let Some(p) = cur {
        if p.join("project.godot").is_file() {
            return Some(p);
        }
        cur = p.parent().map(|pp| pp.to_path_buf());
    }
    None
}

fn line_start_offset(src: &str, line: usize) -> usize {
    src.char_indices().filter(|(_, c)| *c == '\n').take(line).map(|(i, _)| i + 1).last().unwrap_or(0)
}

fn offset_to_position(src: &str, offset: usize) -> Position {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, c) in src.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    Position { line, character: col }
}

pub(crate) fn offset_range(src: &str, start: usize, end: usize) -> Range {
    Range { start: offset_to_position(src, start), end: offset_to_position(src, end) }
}

fn position_to_offset(src: &str, pos: Position) -> Option<usize> {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, c) in src.char_indices() {
        if line == pos.line && col == pos.character {
            return Some(i);
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    None
}

fn word_at(src: &str, offset: usize) -> Option<String> {
    word_span(src, offset).map(|(_, w)| w)
}

/// 同 word_at，另返回词起点 offset（hover 的 '@' 注解上下文判断用）。
fn word_span(src: &str, offset: usize) -> Option<(usize, String)> {
    let b = src.as_bytes();
    let mut i = offset.min(b.len());
    while i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
        i -= 1;
    }
    let mut j = offset.min(b.len());
    while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
        j += 1;
    }
    if i == j {
        None
    } else {
        Some((i, src[i..j].to_string()))
    }
}

/// C3：本文件自定义标识符提取。
///
/// 返回 (局部定义, 高频词)。局部定义 = 光标行之前（含同行）词法扫出的
/// `var X` / `const X` / `for X in` / 函数参数名；高频词 = 文件内出现
/// ≥2 次的标识符（跳过与光标词重叠的 token——半截词不计）。
/// 排序：locals 按声明顺序（作用域就近），高频词按出现次数降序。
fn file_words(src: &str, cursor: usize) -> (Vec<String>, Vec<(String, u32)>) {
    let b = src.as_bytes();
    // 光标词区间：扫描统计时跳过与之重叠的 token（输入中的半截词不计入频次）。
    let (cs, ce) = match word_span(src, cursor) {
        Some((s, w)) => (s, s + w.len()),
        None => (usize::MAX, 0),
    };

    // 局部定义：逐行扫 var/const/for/func 参数（词法级，行号 ≤ 光标行）。
    let mut locals: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let cursor_line = src[..cursor.min(src.len())].bytes().filter(|&c| c == b'\n').count();
    for (ln, line) in src.lines().enumerate() {
        if ln > cursor_line {
            break;
        }
        let t = line.trim_start();
        let push = |name: &str, locals: &mut Vec<String>, seen: &mut std::collections::HashSet<String>| {
            if !name.is_empty()
                && (name.as_bytes()[0].is_ascii_alphabetic() || name.as_bytes()[0] == b'_')
                && seen.insert(name.to_string())
            {
                locals.push(name.to_string());
            }
        };
        if let Some(rest) = t.strip_prefix("var ").or_else(|| t.strip_prefix("const ")) {
            let name: String = rest.trim().chars().take_while(|&c| c.is_ascii_alphanumeric() || c == '_').collect();
            push(&name, &mut locals, &mut seen);
        } else if let Some(rest) = t.strip_prefix("for ") {
            // for i in ... / for i: int in ...
            let name: String = rest.trim().chars().take_while(|&c| c.is_ascii_alphanumeric() || c == '_').collect();
            push(&name, &mut locals, &mut seen);
        } else if let Some(rest) = t.strip_prefix("func ") {
            // func f(a, b: int = 1) -> 逐参数取冒号/等号前的名字。
            if let Some(lp) = rest.find('(') {
                let params = rest[lp..].trim_start_matches('(');
                for p in params.split(',') {
                    let name: String = p
                        .trim()
                        .split(':')
                        .next()
                        .unwrap_or("")
                        .split('=')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .chars()
                        .take_while(|&c| c.is_ascii_alphanumeric() || c == '_')
                        .collect();
                    push(&name, &mut locals, &mut seen);
                }
            }
        }
    }

    // 高频词：全文标识符计数（跳过光标词区间与字符串字面量外不做区分——词法级）。
    let mut counts: HashMap<String, u32> = HashMap::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_alphabetic() || b[i] == b'_' {
            let s = i;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            // 与光标词区间重叠的 token 跳过（含正在输入的半截词）。
            if s < ce && i > cs {
                continue;
            }
            let w = &src[s..i];
            if w.len() >= 2 {
                *counts.entry(w.to_string()).or_insert(0) += 1;
            }
        } else {
            i += 1;
        }
    }
    let mut freq: Vec<(String, u32)> = counts.into_iter().filter(|(_, n)| *n >= 2).collect();
    freq.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    (locals, freq)
}

fn apply_range_change(doc: &mut String, range: &Range, text: &str) {
    let start = position_to_offset(doc, range.start).unwrap_or(doc.len());
    let end = position_to_offset(doc, range.end).unwrap_or(doc.len());
    doc.replace_range(start..end, text);
}



/// 递归扫描 root 下所有 *.uid 文件，建立 uid:// -> res_path 映射。
/// Godot 4.4+ 为每个资源写一个 `<resource>.uid` 侧写文件，内容是 uid 字符串。
fn scan_uid_files(base: &std::path::Path, dir: &std::path::Path, map: &mut HashMap<String, String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // 跳过 .godot/.git。
            if let Some(name) = path.file_name().and_then(|n| n.to_str())
                && (name == ".godot" || name == ".git") { continue; }
            scan_uid_files(base, &path, map);
        } else if path.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("uid"))
            && let Ok(uid_str) = std::fs::read_to_string(&path) {
                let uid = uid_str.trim().trim_start_matches("uid://").to_string();
                if let Ok(rel) = path.strip_prefix(base) {
                    let res_path = rel.with_extension(""); // 去掉 .uid
                    let res_path = res_path.to_string_lossy().replace('\\', "/");
                    map.insert(uid, res_path);
                }
            }
        // P2.7：tscn/tres 头部自嵌 uid（[gd_scene uid="uid://..."]），只读头部。
        else if let Some(ext) = path.extension().and_then(|e| e.to_str())
            && (ext.eq_ignore_ascii_case("tscn") || ext.eq_ignore_ascii_case("tres"))
            && let Ok(mut f) = std::fs::File::open(&path) {
                use std::io::Read;
                let mut buf = [0u8; 1024];
                // 头部限 1KB：uid 必在首行附近；读多了浪费（.tres 可数 MB）。
                let n = f.read(&mut buf).unwrap_or(0);
                let head = String::from_utf8_lossy(&buf[..n]);
                if let Some(uid) = gdscript_project::tscn::scan_header_uid(&head)
                    && let Ok(rel) = path.strip_prefix(base) {
                        let res_path = rel.to_string_lossy().replace('\\', "/");
                        map.entry(uid).or_insert_with(|| res_path);
                    }
            }
    }
}

#[cfg(test)]
mod hover_tests {
    use super::*;

    pub(super) fn test_state() -> State {
        let json = include_str!("../../gdscript-engine/testdata/extension_api.json");
        let engine = std::sync::Arc::new(EngineApi::from_json(json).unwrap());
        let real_lower: HashSet<String> =
            engine.classes.keys().chain(engine.builtins.keys()).map(|s| s.to_lowercase()).collect();
        let (tx, _rx) = std::sync::mpsc::channel();
        State {
            engine,
            real_lower,
            docs: HashMap::new(),
            asts: HashMap::new(),
            doc_versions: HashMap::new(),
            cancelled: HashSet::new(),
            uid_map: HashMap::new(),
            project: None,
            root: None,
            diag_sender: tx,
            snippet_support: false,
            config: Config::default(), prefs: EditorPrefs::default(),
        }
    }

    pub(crate) fn state_with(state: &State, src: &str) -> State {
        // Uri 作 key 的 mutable_key_type 为 lsp-types 内部表示误报(仅插入不改 key)。
        #[allow(clippy::mutable_key_type)]
        let mut docs = HashMap::new();
        docs.insert("file:///tmp/t.gd".parse::<Uri>().unwrap(), src.to_string());
        State {
            engine: state.engine.clone(),
            real_lower: state.real_lower.clone(),
            docs,
            asts: HashMap::new(),
            doc_versions: HashMap::new(),
            cancelled: HashSet::new(),
            uid_map: HashMap::new(),
            project: None,
            root: None,
            diag_sender: state.diag_sender.clone(),
            snippet_support: false,
            config: Config::default(), prefs: EditorPrefs::default(),
        }
    }

    /// 带工程面的 state：从磁盘 fixture 打开 Project（C7.1/C7.2 测试用）。
    fn state_with_project(state: &State, src: &str, fixture: &str) -> State {
        let json = include_str!("../../gdscript-engine/testdata/extension_api.json");
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(fixture);
        let project = Project::open(&root, Some(json)).ok().map(std::sync::Arc::new);
        let mut s = state_with(state, src);
        s.project = project;
        s.root = Some(root);
        s
    }

    fn hover_md_of(state: &State, src: &str, needle: &str) -> Option<String> {
        let off = src.find(needle).unwrap() + needle.len() / 2;
        let mut line = 0u32;
        let mut col = 0u32;
        for (i, c) in src.char_indices() {
            if i >= off {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let p: HoverParams = serde_json::from_value(serde_json::json!({
            "textDocument": {"uri": uri},
            "position": {"line": line, "character": col}
        }))
        .unwrap();
        hover(state, &p).map(|h| match h.contents {
            HoverContents::Markup(m) => m.value,
            _ => String::new(),
        })
    }

    /// P1 描述层端到端回归：成员属性 / 内建枚举值 / 类枚举名 hover 必须命中。
    #[test]
    fn hover_member_and_enum() {
        let base = test_state();
        let src = "var sprite: Sprite2D
func _ready() -> void:
	sprite.position
	var ax := Vector3.AXIS_X
	var pm = Node.ProcessMode
";
        let state = state_with(&base, src);

        // 引擎数据面断言（防模型回归）。
        assert!(base.engine.find_property("Sprite2D", "position").is_some(), "Sprite2D.position 必须可查");
        assert!(base.engine.find_enum("Node", "ProcessMode").is_some(), "Node.ProcessMode 必须可查");
        assert!(
            base.engine.builtins.get("Vector3").unwrap().enums.iter().any(|e| e.name == "Axis"),
            "Vector3.Axis 内建枚举必须加载"
        );

        // hover 端到端断言。
        let m = hover_md_of(&state, src, "position").unwrap_or_else(|| panic!("sprite.position hover 缺失"));
        assert!(m.contains("position"), "成员 hover 应含签名: {m}");
        // P0 中文补丁：Node2D.position 描述已汉化；URL 必须指向描述宿主类页（Sprite2D 页无此锚）。
        assert!(m.contains("位置"), "成员 hover 应沿继承链取到 Node2D 中文描述: {m}");
        assert!(m.contains("class_node2d.html#position"), "成员 hover 链接应指向描述宿主类页: {m}");
        let e = hover_md_of(&state, src, "AXIS_X").unwrap_or_else(|| panic!("Vector3.AXIS_X hover 缺失"));
        assert!(e.contains("X axis"), "枚举值 hover 应含描述: {e}");
        let en = hover_md_of(&state, src, "ProcessMode").unwrap_or_else(|| panic!("Node.ProcessMode hover 缺失"));
        assert!(en.contains("enum ProcessMode"), "枚举名 hover 应含签名: {en}");
    }

    /// completion 面覆盖：全局面（内建函数/常量/关键字/全局枚举值）+
    /// @ 触发注解面 + 成员面（内建枚举值/常量/类枚举名）。
    #[test]
    fn completion_surfaces() {
        let base = test_state();

        // labels：对给定 src 在 (line, ch) 取 completion 标签集。
        let labels = |src: &str, line: u32, ch: u32| -> Vec<String> {
            let state = state_with(&base, src);
            let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
            let p: CompletionParams = serde_json::from_value(serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": line, "character": ch}
            }))
            .unwrap();
            match completion(&state, &p) {
                CompletionResponse::Array(items) => items.into_iter().map(|i| i.label).collect(),
                _ => vec![],
            }
        };

        // 全局面：内建函数 / @GDScript 常量 / 关键字 / 全局枚举值 / 类 / 单例。
        let all = labels("# comment\n", 0, 2);
        for need in ["len(…)", "preload(…)", "PI", "func", "KEY_LEFT", "Sprite2D", "Input"] {
            assert!(all.iter().any(|l| l == need), "全局面应含 {need}: 命中数 {}", all.len());
        }

        // @ 触发：注解面只出注解，不混类名，且恰好全量。
        let annos = labels("@\n", 0, 1);
        assert!(annos.iter().any(|l| l == "export"), "注解面应含 export: {annos:?}");
        assert!(!annos.iter().any(|l| l == "Node2D"), "注解面不应混入类名");
        assert_eq!(annos.len(), hoverdata::annotations().len(), "注解面应恰好是全量注解");

        // 成员面（内建）：Vector3. → 枚举值 / 常量 / 方法。
        let mem = labels("var v := Vector3.\n", 0, 17);
        for need in ["AXIS_X", "ZERO", "normalized(…)"] {
            assert!(mem.iter().any(|l| l == need), "Vector3 成员面应含 {need}: 命中数 {}", mem.len());
        }

        // 成员面（静态类名回退）：Node. → ProcessMode 枚举名。
        let mem = labels("var x = Node.\n", 0, 13);
        assert!(mem.iter().any(|l| l == "ProcessMode"), "Node 成员面应含枚举名 ProcessMode: 命中数 {}", mem.len());

        // 枚举接收者：`AStarGrid2D.Heuristic.` → 该枚举的值面(裸值名)。
        let ev = labels("var h := AStarGrid2D.Heuristic.\n", 0, 29);
        for need in ["HEURISTIC_MAX", "HEURISTIC_OCTILE"] {
            assert!(ev.iter().any(|l| l == need), "枚举值面应含 {need}: {:?}", ev);
        }
    }

    /// C7.3 节点路径上下文识别(镜像 fa7d645 语义;R4 重写回归补测)。
    #[test]
    fn node_path_context_detection() {
        let t = "var p = $Panel/\nvar q = %Sprite\nvar r = 7 % 3\nvar s = a%b\n";
        let l2 = t.find('%').unwrap();
        let l3 = t.find("7 % 3").unwrap();
        let l4 = t.find("a%b").unwrap();
        // $ 路径:"var p = $Panel/"
        assert!(in_node_path_context(t, 9), "$ 后");
        assert!(in_node_path_context(t, 12), "$Pa 中");
        assert!(in_node_path_context(t, 14), "$Panel 尾");
        assert!(in_node_path_context(t, 15), "$Panel/ 后");
        assert!(!in_node_path_context(t, 8), "$ 之前");
        assert!(!in_node_path_context(t, 4), "普通标识符 var");
        // % unique:"var q = %Sprite"
        assert!(in_node_path_context(t, l2 + 1), "% 后");
        // 取模:"7 % 3" — 光标在结果操作数之后,不得视为路径
        assert!(!in_node_path_context(t, l3 + 5), "取模结果后");
        // a%b:% 前是操作数 → 取模
        assert!(!in_node_path_context(t, l4 + 3), "a%b 的 b 后");
    }

    /// C7.3 回归锁:`$Panel/` / `%Sprite` 上下文不再合并全局面
    /// (fa7d645 探针实测污染 1662 项);取模上下文不受影响。
    #[test]
    fn completion_node_path_no_global_pollution() {
        let base = test_state();
        let labels = |src: &str, line: u32, ch: u32| -> Vec<String> {
            let state = state_with(&base, src);
            let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
            let p: CompletionParams = serde_json::from_value(serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": line, "character": ch}
            }))
            .unwrap();
            match completion(&state, &p) {
                CompletionResponse::Array(items) => items.into_iter().map(|i| i.label).collect(),
                _ => vec![],
            }
        };
        // `$Panel/` 词尾:不得出现任何全局面项(类/函数/关键字)。
        let dollar = labels("func _ready():\n\tvar p = $Panel/\n", 1, 15);
        assert!(dollar.is_empty(), "$Panel/ 应不出全局面: 前 12 项 {:?}", &dollar[..dollar.len().min(12)]);
        // `%Sprite` 词尾:同上。
        let unique = labels("func _ready():\n\tvar q = %Sprite\n", 1, 14);
        assert!(unique.is_empty(), "%Sprite 应不出全局面: 前 12 项 {:?}", &unique[..unique.len().min(12)]);
        // 取模上下文:`7 % 3` 之后照常出全局面(抑制不得误伤)。
        let modulo = labels("func _ready():\n\tvar r = 7 % 3\n", 1, 15);
        assert!(modulo.iter().any(|l| l == "print(…)"), "取模后应仍出全局面: {}", modulo.len());
    }

    /// C6.4 函数面 insertText 约定(引擎 LSP 适配器同款):snippet 协商时
    /// `name($1)` + Snippet 格式;无协商时退化为裸名(上游 trim_suffix("("))。
    #[test]
    fn completion_function_insert_text_convention() {
        let base = test_state();
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let ask = |state: &State, line: u32, ch: u32| -> Vec<CompletionItem> {
            let p: CompletionParams = serde_json::from_value(serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": line, "character": ch}
            }))
            .unwrap();
            match completion(state, &p) {
                CompletionResponse::Array(items) => items,
                _ => vec![],
            }
        };
        // snippet 协商 + 开启:全局面 print → label "print(…)" / insertText "print($1)"。
        let mut st = state_with(&base, "print\n");
        st.snippet_support = true;
        let items = ask(&st, 0, 5);
        let print = items.iter().find(|i| i.label == "print(…)").expect("应出 print(…)");
        assert_eq!(print.insert_text.as_deref(), Some("print($1)"));
        assert_eq!(print.insert_text_format, Some(InsertTextFormat::SNIPPET));
        // 无 snippet 协商:insertText 退化为裸名(不放括号,客户端补不进光标)。
        let st2 = state_with(&base, "print\n");
        let items2 = ask(&st2, 0, 5);
        let print2 = items2.iter().find(|i| i.label == "print(…)").expect("应出 print(…)");
        assert_eq!(print2.insert_text.as_deref(), Some("print"));
        assert_eq!(print2.insert_text_format, None);
        // 成员面同约定:Vector3.normalized → "normalized($1)"(协商时)。
        let mut st3 = state_with(&base, "var v := Vector3.normalized\n");
        st3.snippet_support = true;
        let items3 = ask(&st3, 0, 27);
        let norm = items3
            .iter()
            .find(|i| i.label == "normalized(…)")
            .expect("成员面应出 normalized(…)");
        assert_eq!(norm.insert_text.as_deref(), Some("normalized($1)"));
    }

    /// C1 sortText：partial 词归面（`Vector3.no` → 成员面）+ 桶/fuzzy 编码。
    #[test]
    fn completion_sort_text() {
        let base = test_state();
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let ask = |state: &State, ch: u32| -> Vec<CompletionItem> {
            let p: CompletionParams = serde_json::from_value(serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": 0, "character": ch}
            }))
            .unwrap();
            match completion(state, &p) {
                CompletionResponse::Array(items) => items,
                _ => vec![],
            }
        };

        // 全局面 partial 词："Col" → 类/内建（TYPE 桶）前缀分压过不匹配者与关键字桶。
        let state = state_with(&base, "var x = Col\n");
        let items = ask(&state, 11);
        assert!(items.iter().all(|i| i.sort_text.is_some()), "全局面每项都应有 sortText");
        let st = |l: &str| {
            items.iter().find(|i| i.label == l).map(|i| i.sort_text.clone().unwrap()).unwrap_or_else(|| panic!("缺 {l}"))
        };
        assert!(st("Color") < st("AudioServer"), "同桶内前缀分: Color={} AudioServer={}", st("Color"), st("AudioServer"));
        assert!(st("Color") < st("class_name"), "TYPE 桶压过 KEYWORD 桶: {} vs {}", st("Color"), st("class_name"));

        // 成员面 partial 词：`Vector3.no`（旧逻辑只有 `Vector3.` 紧邻点才归成员面）。
        let state = state_with(&base, "var v := Vector3.no\n");
        let items = ask(&state, 19);
        assert!(items.iter().any(|i| i.label == "normalized(…)"), "成员面应识别 partial 词");
        let st = |l: &str| {
            items.iter().find(|i| i.label == l).map(|i| i.sort_text.clone().unwrap()).unwrap_or_else(|| panic!("缺 {l}"))
        };
        assert!(st("normalized(…)") < st("ONE"), "前缀分应排序: normalized={} ONE={}", st("normalized(…)"), st("ONE"));
    }

    /// C2 snippet 面：协商开关两侧 + 上下文感知（宿主过滤/语句位）。
    #[test]
    fn completion_snippet_surfaces() {
        let base = test_state();
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let ask = |state: &State, line: u32, ch: u32| -> Vec<CompletionItem> {
            let p: CompletionParams = serde_json::from_value(serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": line, "character": ch}
            }))
            .unwrap();
            match completion(state, &p) {
                CompletionResponse::Array(items) => items,
                _ => vec![],
            }
        };

        // 开启：`extends Node` + `func _re` → _ready/_process 模板,无 _draw（Node 非 CanvasItem）。
        let mut st = state_with(&base, "extends Node\nfunc _re\n");
        st.snippet_support = true;
        st.config.snippets = true;
        let items = ask(&st, 1, 8);
        let ready = items.iter().find(|i| i.label == "_ready").expect("Node 系应出 _ready 模板");
        assert_eq!(ready.insert_text.as_deref(), Some("_ready() -> void:\n\t$0"));
        assert_eq!(ready.insert_text_format, Some(InsertTextFormat::SNIPPET));
        assert!(!items.iter().any(|i| i.label == "_draw"), "Node 系不应出 _draw");
        assert!(ready.sort_text.as_deref().unwrap().starts_with("11"), "SNIPPET 桶编码");

        // 无 extends → RefCounted 默认 → 不出 _ready（Object 级 _notification 不匹配前缀 _re）。
        let mut st2 = state_with(&base, "func _re\n");
        st2.snippet_support = true;
        st2.config.snippets = true;
        let items2 = ask(&st2, 0, 8);
        assert!(!items2.iter().any(|i| i.label == "_ready"), "RefCounted 默认不出 _ready");

        // 语句位复合模板：`inst` → instantiate_scene；非语句位（跟在标识符后）不出。
        let mut st3 = state_with(&base, "inst\n");
        st3.snippet_support = true;
        st3.config.snippets = true;
        let items3 = ask(&st3, 0, 4);
        let comp = items3.iter().find(|i| i.label == "instantiate_scene").expect("语句位应出 instantiate_scene");
        assert_eq!(comp.insert_text_format, Some(InsertTextFormat::SNIPPET));
        assert!(comp.insert_text.as_deref().unwrap().starts_with("var ${1:node}"));
        let st4 = state_with(&base, "var inst\n");
        let items4 = ask(&st4, 0, 8);
        assert!(!items4.iter().any(|i| i.label == "instantiate_scene"), "非语句位不出复合模板");

        // 关闭：整套模板不存在,控制流关键字保持纯文本。
        let st5 = state_with(&base, "for\n");
        let items5 = ask(&st5, 0, 3);
        let for_kw = items5.iter().find(|i| i.label == "for").expect("关键字 for 应在");
        assert!(for_kw.insert_text.is_none(), "snippetSupport=false 时关键字不应升级");
        assert!(!items5.iter().any(|i| i.label == "instantiate_scene"));

        // 开启：`for` 关键字升级为 snippet 插入（label 不变）。
        let mut st6 = state_with(&base, "for\n");
        st6.snippet_support = true;
        st6.config.snippets = true;
        let items6 = ask(&st6, 0, 3);
        let for_kw6 = items6.iter().find(|i| i.label == "for").unwrap();
        assert_eq!(for_kw6.insert_text_format, Some(InsertTextFormat::SNIPPET));
        assert!(for_kw6.insert_text.as_deref().unwrap().starts_with("for ${1:i}"));
    }

    /// C4 频率细分：空前缀热 API 排前,有前缀不受频率影响。
    #[test]
    fn freq_hot_first_on_empty_prefix() {
        let hot = scored_fuzzy("", "print", true);
        let cold = scored_fuzzy("", "zzz_not_an_api_zzz", true);
        assert_eq!(cold, scoring::band::NEUTRAL, "未收录项中性分");
        assert!(hot > cold, "热 API 应细分排前: hot={hot:#06x} cold={cold:#06x}");
        // 有前缀时纯 fuzzy——频率只在空前缀生效。
        assert_eq!(scored_fuzzy("pri", "print", true), scoring::fuzzy("pri", "print"));
        // freq_boost=false 时空前缀回退纯中性。
        assert_eq!(scored_fuzzy("", "print", false), scoring::band::NEUTRAL);
    }

    /// C3 本文件词条面：locals 空前缀也出 + 高频词有前缀才出 + 保留名排除 + LOCAL 桶最前。
    #[test]
    fn completion_file_words() {
        let base = test_state();
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let ask = |state: &State, line: u32, ch: u32| -> Vec<CompletionItem> {
            let p: CompletionParams = serde_json::from_value(serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": line, "character": ch}
            }))
            .unwrap();
            match completion(state, &p) {
                CompletionResponse::Array(items) => items,
                _ => vec![],
            }
        };
        let is_word_entry = |i: &CompletionItem, detail: &str| i.detail.as_deref() == Some(detail);

        // locals：var/const/for/参数——空前缀也出，LOCAL 桶（"00"）最前。
        let src = "extends Node\nfunc tick(delta: float, n := 0) -> void:\n\tvar hp := 10\n\tfor i in range(3):\n\t\tpass\n\nfunc other() -> void:\n\t";
        let mut st = state_with(&base, src);
        st.config.file_words = true; // 配置开关默认关，测试显式开。
        let items = ask(&st, 7, 1);
        for name in ["hp", "delta", "n", "i"] {
            assert!(
                items.iter().any(|i| i.label == name && is_word_entry(i, "局部变量")),
                "{name} 应以局部变量出: {:?}",
                items.iter().filter(|i| is_word_entry(i, "局部变量")).map(|i| &i.label).collect::<Vec<_>>()
            );
        }
        let hp = items.iter().find(|i| i.label == "hp").unwrap();
        assert!(hp.sort_text.as_deref().unwrap().starts_with("00"), "LOCAL 桶编码: {}", hp.sort_text.as_deref().unwrap());

        // 高频词：非声明的重复标识符，有前缀才出；保留名（Sprite2D）不重复出。
        let src2 = "var t := Sprite2D.new()\nvar u := Sprite2D.new()\n\nmove_and_slide()\nmove_and_slide()\nmov";
        let mut st2 = state_with(&base, src2);
        st2.config.file_words = true;
        let items2 = ask(&st2, 5, 3);
        let mas = items2
            .iter()
            .find(|i| i.label == "move_and_slide" && is_word_entry(i, "本文件出现 2 次"))
            .expect("高频词 move_and_slide 应出");
        assert!(mas.sort_text.as_deref().unwrap().starts_with("00"));
        assert!(
            !items2.iter().any(|i| i.label == "Sprite2D" && is_word_entry(i, "本文件出现 2 次")),
            "保留名不该以词条身份重复出现"
        );

        // 空前缀不出高频词（避免淹没全局面）。
        let st3 = state_with(&base, "move_and_slide()\nmove_and_slide()\n\n");
        let items3 = ask(&st3, 2, 0);
        assert!(
            !items3.iter().any(|i| i.label == "move_and_slide" && is_word_entry(i, "本文件出现 2 次")),
            "空前缀不出高频词"
        );
    }

    /// C6.1：成员面含类声明信号（dump 503 条，此前 RawApi 丢弃、成员面缺失）。
    #[test]
    fn completion_member_signals() {
        let base = test_state();
        let state = state_with(&base, "var n: CanvasItem\nn.\n");
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let p: CompletionParams = serde_json::from_value(serde_json::json!({
            "textDocument": {"uri": uri},
            "position": {"line": 1, "character": 2}
        }))
        .unwrap();
        let CompletionResponse::Array(items) = completion(&state, &p) else { panic!("应返回数组") };
        let draw = items.iter().find(|i| i.label == "draw").unwrap_or_else(|| panic!("CanvasItem 成员面应含信号 draw: {:?}", items.len()));
        assert_eq!(draw.kind, Some(CompletionItemKind::EVENT));
        assert!(draw.detail.as_deref().is_some_and(|d| d.starts_with("draw(")), "信号 detail 应为签名: {:?}", draw.detail);
        // 引擎模型面断言（防解析回归）。
        assert!(base.engine.find_signal("CanvasItem", "draw").is_some(), "CanvasItem.draw 必须可查");
    }

    /// C7.1：全局面含项目 class_name 脚本类与 autoload 单例（工程面接入）。
    #[test]
    fn completion_project_global_face() {
        let base = test_state();
        let state = state_with_project(&base, "# c\n", "projface");
        assert!(state.project.is_some(), "fixture 工程应可打开");
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let p: CompletionParams = serde_json::from_value(serde_json::json!({
            "textDocument": {"uri": uri},
            "position": {"line": 0, "character": 2}
        }))
        .unwrap();
        let CompletionResponse::Array(items) = completion(&state, &p) else { panic!("应返回数组") };
        let player = items.iter().find(|i| i.label == "Player").unwrap_or_else(|| panic!("全局面应含 class_name Player"));
        assert_eq!(player.kind, Some(CompletionItemKind::CLASS));
        assert!(player.detail.as_deref().is_some_and(|d| d.contains("player.gd")), "detail 应指脚本: {:?}", player.detail);
        let gm = items.iter().find(|i| i.label == "GameManager").unwrap_or_else(|| panic!("全局面应含 autoload GameManager"));
        assert_eq!(gm.kind, Some(CompletionItemKind::VARIABLE));
        assert!(gm.detail.as_deref().is_some_and(|d| d.contains("autoload")), "detail 应标 autoload: {:?}", gm.detail);
        // 引擎面不得被工程面污染：Node 仍恰好一项（class_name 同名让位引擎）。
        assert_eq!(items.iter().filter(|i| i.label == "Node").count(), 1, "引擎类不得重复");
    }

    /// N2 + C7.2：普通字符串内不出补全；`preload("res://`/`load("res://` 出项目
    /// 文件路径面（label 完整 res:// 路径 + textEdit 替换内容区）。
    #[test]
    fn completion_string_context() {
        let base = test_state();
        let src = "extends Node\nfunc _ready():\n\tvar s := preload(\"res://pla\n\tvar t := load(\"res://game_man\n\tvar u := \"plain string\"\n\tvar v := \"esc \\\" quote\"\n";
        let state = state_with_project(&base, src, "projface");
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let at = |line: u32, ch: u32| -> Vec<CompletionItem> {
            let p: CompletionParams = serde_json::from_value(serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": line, "character": ch}
            }))
            .unwrap();
            match completion(&state, &p) {
                CompletionResponse::Array(items) => items,
                _ => vec![],
            }
        };
        // preload 路径面：命中 player.gd，textEdit 覆盖开引号后..光标。
        let pl = at(2, 28);
        let item = pl.iter().find(|i| i.label == "res://player.gd").unwrap_or_else(|| panic!("preload 路径面应含 player.gd: {:?}", pl.len()));
        assert_eq!(item.kind, Some(CompletionItemKind::FILE));
        let Some(CompletionTextEdit::Edit(e)) = &item.text_edit else { panic!("路径项应带 textEdit") };
        assert_eq!(e.range.start, Position { line: 2, character: 19 }, "替换区应起于开引号后");
        assert_eq!(e.new_text, "res://player.gd");
        // load 路径面：game_manager.gd；level.tscn 也应在库（前缀 pla 不命中它）。
        assert!(at(3, 30).iter().any(|i| i.label == "res://game_manager.gd"), "load 路径面应含 game_manager.gd");
        assert!(at(2, 28).iter().all(|i| i.label != "res://level.tscn"), "前缀 pla 不应命中 level.tscn");
        // N2：普通字符串与转义引号字符串内一律空。
        assert!(at(4, 15).is_empty(), "普通字符串内应无补全");
        assert!(at(5, 21).is_empty(), "转义引号字符串内应无补全");
        // 字符串外的全局面不受影响。
        assert!(!at(1, 14).is_empty(), "func _ready(): 行外全局面应照常");
    }

    /// string_literal_at 状态机：单行/多行/转义/注释/未闭合。
    #[test]
    fn string_literal_at_scanner() {
        let s = "x = \"ab\"\n";
        assert_eq!(string_literal_at(s, 5), Some((5, 7)), "单行字符串内容区");
        assert_eq!(string_literal_at(s, 7), Some((5, 7)), "闭引号前一格仍算串内");
        assert_eq!(string_literal_at(s, 8), None, "闭引号后不算");
        assert_eq!(string_literal_at(s, 0), None, "串前代码不算");
        // 注释内引号不成串。
        let c = "x = 1 # \"not a string\"\n";
        assert_eq!(string_literal_at(c, 12), None, "注释内引号不成串");
        // 多行字符串。
        let m = "a = \"\"\"line1\nline2➡\n\"\"\"\n";
        let at = m.find('➡').unwrap();
        assert_eq!(string_literal_at(m, at), Some((7, 22)), "多行字符串内容区");
        // 转义引号不闭合字符串。
        let e = "a = \"x \\\" y\"\n";
        let inner = e.find('y').unwrap();
        assert_eq!(string_literal_at(e, inner), Some((5, 11)), "转义引号仍在串内");
        // 未闭合字符串：到行尾。
        let u = "a = \"oops\nb = 1\n";
        assert_eq!(string_literal_at(u, 7), Some((5, 9)), "未闭合按到行尾");
        assert_eq!(string_literal_at(u, 13), None, "下一行不受未闭合影响");
        // NodePath/StringName 字面量前缀不影响配对。
        let np = "a = &\"path\"\n";
        assert_eq!(string_literal_at(np, 8), Some((6, 10)));
    }

    /// is_load_call_arg：preload/load 判定与回扫边界（quote_pos = 开引号下标）。
    #[test]
    fn is_load_call_arg_scanner() {
        assert!(is_load_call_arg("preload(\"res://x.gd\")", 8), "preload 直调");
        assert!(is_load_call_arg("load ( \"res://x.gd\")", 7), "load 带空白");
        assert!(is_load_call_arg("ResourceLoader.load(\"res://x.gd\")", 20), "限定 load");
        assert!(!is_load_call_arg("myload(\"res://x.gd\")", 7), "前缀不同名");
        assert!(!is_load_call_arg("x = \"res://x.gd\"", 4), "赋值非调用实参");
        assert!(!is_load_call_arg("f(g(\"res://x.gd\"))", 4), "内层调用按 f 判否");
    }

}

#[cfg(test)]
mod color_codelens_tests {
    use super::*;

    #[test]
    fn parse_color_args_basics() {
        assert_eq!(parse_color_args("0.5, 0.25, 1.0"), Some([0.5, 0.25, 1.0, 1.0]));
        assert_eq!(parse_color_args("1, 0, 0"), Some([1.0, 0.0, 0.0, 1.0]));
        assert_eq!(parse_color_args("0.5,0.25,1.0,0.8"), Some([0.5, 0.25, 1.0, 0.8]));
        assert_eq!(parse_color_args("-0.1, 0, 0"), Some([-0.1, 0.0, 0.0, 1.0]));
        // 变量 / 空 / 缺参 / 表达式一律拒绝（零误报）。
        assert_eq!(parse_color_args("r, 0, 0"), None);
        assert_eq!(parse_color_args("0.5, 0.25"), None);
        assert_eq!(parse_color_args(""), None);
        assert_eq!(parse_color_args("a+b, 0, 0"), None);
    }

    #[test]
    fn document_color_literal_and_named() {
        let state = super::hover_tests::test_state();
        // Uri 作 key 的 mutable_key_type 为 lsp-types 内部表示误报。
        #[allow(clippy::mutable_key_type)]
        let mut docs = HashMap::new();
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        docs.insert(uri.clone(), "var a := Color(0.5, 0.25, 1.0, 0.8)\nvar b := Color.RED\nvar c := Color(1, 0, 0)\nvar bad := Color(x, 0, 0)\nvar not_color := XColor(1, 0, 0)\n".into());
        // 直接构造一个带 docs 的 state（test_state 返回空 docs，手动填充）。
        let st = State {
            engine: state.engine.clone(),
            real_lower: state.real_lower.clone(),
            docs,
            asts: HashMap::new(),
            doc_versions: HashMap::new(),
            cancelled: HashSet::new(),
            uid_map: HashMap::new(),
            project: None,
            root: None,
            diag_sender: state.diag_sender.clone(),
            snippet_support: false,
            config: Config::default(), prefs: EditorPrefs::default(),
        };
        let colors = document_color(&st, &DocumentColorParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        });
        assert_eq!(colors.len(), 3, "应命中构造×2 + 命名×1, 变量与 XColor 不算: {colors:?}");
        // 第一条 = 构造调用。
        assert!((colors[0].color.red - 0.5).abs() < 1e-6);
        assert!((colors[0].color.alpha - 0.8).abs() < 1e-6);
        // 第二条 = Color.RED。
        assert!(colors[1].color.red > 0.8 && colors[1].color.green < 0.2);
    }

    #[test]
    fn color_presentation_roundtrip() {
        let p = ColorPresentationParams {
            text_document: TextDocumentIdentifier { uri: "file:///tmp/t.gd".parse().unwrap() },
            color: Color { red: 1.0, green: 0.5, blue: 0.0, alpha: 1.0 },
            range: Range::default(),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let pres = color_presentation(&p);
        assert_eq!(pres.len(), 1);
        assert_eq!(pres[0].text_edit.as_ref().unwrap().new_text, "Color(1.000000, 0.500000, 0.000000, 1.000000)");
    }

    #[test]
    fn code_lens_scene_connections() {
        // 用 uidscene 夹具：main.tscn [connection] method="_ready" 引用 main.gd。
        let json = include_str!("../../gdscript-engine/testdata/extension_api.json");
        let engine = std::sync::Arc::new(EngineApi::from_json(json).unwrap());
        let real_lower: HashSet<String> =
            engine.classes.keys().chain(engine.builtins.keys()).map(|s| s.to_lowercase()).collect();
        let (tx, _rx) = std::sync::mpsc::channel();
        let manifest = env!("CARGO_MANIFEST_DIR");
        let root = std::path::PathBuf::from(manifest).join("tests/fixtures/uidscene");
        let project = Project::open(&root, Some(json)).unwrap();
        let st = State {
            engine,
            real_lower,
            docs: HashMap::new(),
            asts: HashMap::new(),
            doc_versions: HashMap::new(),
            cancelled: HashSet::new(),
            uid_map: HashMap::new(),
            project: Some(std::sync::Arc::new(project)),
            root: Some(root.clone()),
            diag_sender: tx,
            snippet_support: false,
            config: Config::default(), prefs: EditorPrefs::default(),
        };
        // 模拟 didOpen：路径须在 root 之下（code_lens 做 strip_prefix）。
        let gd = root.join("main.gd");
        let uri = path_to_uri(&gd).unwrap();
        let src = std::fs::read_to_string(&gd).unwrap();
        #[allow(clippy::mutable_key_type)]
        let mut docs = HashMap::new();
        docs.insert(uri.clone(), src);
        let mut st = st;
        st.docs = docs;

        let lenses = code_lens(&st, &CodeLensParams {
            text_document: TextDocumentIdentifier { uri },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        });
        assert_eq!(lenses.len(), 1, "func _ready 应有 1 条 lens: {lenses:?}");
        let cmd = lenses[0].command.as_ref().unwrap();
        assert_eq!(cmd.title, "1 个场景连接");
        assert_eq!(cmd.command, "godotlsp.openConnection");
        let args = cmd.arguments.as_ref().unwrap();
        assert!(args[0].as_str().unwrap().ends_with("main.tscn"));
    }
}

#[cfg(test)]
mod rename_tests {
    use super::*;

    fn rename_state(dir: &std::path::Path, open: &str, src: &str) -> (State, Uri) {
        let json = include_str!("../../gdscript-engine/testdata/extension_api.json");
        let engine = std::sync::Arc::new(EngineApi::from_json(json).unwrap());
        let real_lower: HashSet<String> =
            engine.classes.keys().chain(engine.builtins.keys()).map(|s| s.to_lowercase()).collect();
        let (tx, _rx) = std::sync::mpsc::channel();
        let project = Project::open(dir, Some(json)).unwrap();
        let uri = path_to_uri(&dir.join(open)).unwrap();
        let mut st = State {
            engine,
            real_lower,
            docs: HashMap::new(),
            asts: HashMap::new(),
            doc_versions: HashMap::new(),
            cancelled: HashSet::new(),
            uid_map: HashMap::new(),
            project: Some(std::sync::Arc::new(project)),
            root: Some(dir.to_path_buf()),
            diag_sender: tx,
            snippet_support: false,
            config: Config::default(), prefs: EditorPrefs::default(),
        };
        st.docs.insert(uri.clone(), src.to_string());
        (st, uri)
    }

    #[test]
    fn rename_rejects_reserved_invalid_same() {
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let mk = |st_docs: &mut HashMap<Uri, String>, new_name: &str, src: &str, pos: Position| {
            st_docs.insert(uri.clone(), src.to_string());
            RenameParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: pos,
                },
                new_name: new_name.to_string(),
                work_done_progress_params: Default::default(),
            }
        };
        // 保留名（引擎类）→ prepareRename None（第一道闸门）。
        let mut st2 = super::hover_tests::test_state();
        st2.docs.insert(uri.clone(), "var Sprite2D := 1\n".to_string());
        assert!(prepare_rename(&st2, &TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position { line: 0, character: 5 },
        }).is_none());

        let mut st = super::hover_tests::test_state();
        let p = mk(&mut st.docs, "1abc", "var health := 10\n", Position { line: 0, character: 5 });
        let err = rename(&st, &p).unwrap_err();
        assert!(err.contains("标识符"), "{err}");
        let p = mk(&mut st.docs, "health", "var health := 10\n", Position { line: 0, character: 5 });
        let err = rename(&st, &p).unwrap_err();
        assert!(err.contains("相同"), "{err}");
        let p = mk(&mut st.docs, "Node2D", "var health := 10\n", Position { line: 0, character: 5 });
        let err = rename(&st, &p).unwrap_err();
        assert!(err.contains("冲突"), "{err}");
    }

    #[test]
    fn rename_cross_file_lexical() {
        let dir = std::env::temp_dir().join(format!("godotlsp_rename_t{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let a = dir.join("a.gd");
        let b = dir.join("b.gd");
        std::fs::write(&a, "extends Node\nfunc do_work() -> void:\n\tdo_work()\n").unwrap();
        std::fs::write(&b, "extends Node\nvar f := do_work\n").unwrap();

        let (st, uri) = rename_state(&dir, "a.gd", &std::fs::read_to_string(&a).unwrap());
        let we = rename(&st, &RenameParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position { line: 1, character: 6 }, // do_work 词中
            },
            new_name: "do_task".to_string(),
            work_done_progress_params: Default::default(),
        }).unwrap();
        // 同上:Uri 作 key 的 mutable_key_type 为 lsp-types 内部表示误报。
        #[allow(clippy::mutable_key_type)]
        let changes = we.changes.unwrap();
        assert_eq!(changes.len(), 2, "a.gd（声明+调用）与 b.gd（引用）都应被改: {changes:?}");
        // 按 uri 精确取（HashMap 无序，勿用 values().find）。
        let a_edits = changes.get(&uri).unwrap();
        assert_eq!(a_edits.len(), 2, "a.gd 声明+递归调用: {a_edits:?}");
        let total: usize = changes.values().map(|v| v.len()).sum();
        assert_eq!(total, 3, "b.gd 引用一处: {changes:?}");
        // prepareRename 返回词范围 + placeholder。
        let pr = prepare_rename(&st, &TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position { line: 1, character: 6 },
        }).unwrap();
        assert_eq!(pr, PrepareRenameResponse::RangeWithPlaceholder {
            range: Range {
                start: Position { line: 1, character: 5 },
                end: Position { line: 1, character: 12 },
            },
            placeholder: "do_work".into(),
        });
        let _ = std::fs::remove_dir_all(&dir);
    }
}

// ───────────────────── span 巡检（反转 span 一次性排查） ─────────────────────
#[cfg(test)]
mod span_sanity_tests {
    use super::*;
    use gdscript_syntax::ast::{Expr, MatchArm, MatchPattern, Stmt};

    fn check_span(sp: (usize, usize), ctx: &str, bad: &mut Vec<String>) {
        if sp.0 > sp.1 {
            bad.push(format!("{ctx}: span ({}, {}) inverted", sp.0, sp.1));
        }
    }

    fn walk_expr(e: &Expr, ctx: &str, bad: &mut Vec<String>) {
        let sp = match e {
            Expr::Literal(_) | Expr::Ident(_) => return,
            Expr::Binary(_, _, _, sp)
            | Expr::Unary(_, _, sp)
            | Expr::Call { span: sp, .. }
            | Expr::Index { span: sp, .. }
            | Expr::Attribute { span: sp, .. }
            | Expr::Lambda(_, sp)
            | Expr::Array(_, sp)
            | Expr::Dict(_, sp)
            | Expr::Await(_, sp)
            | Expr::NodePath { span: sp, .. }
            | Expr::Generic(_, _, sp)
            | Expr::Ternary(_, _, _, sp) => *sp,
        };
        check_span(sp, ctx, bad);
        match e {
            Expr::Binary(a, _, b, _) => {
                walk_expr(a, ctx, bad);
                walk_expr(b, ctx, bad);
            }
            Expr::Unary(_, a, _) => walk_expr(a, ctx, bad),
            Expr::Call { callee, args, .. } => {
                walk_expr(callee, ctx, bad);
                for a in args {
                    walk_expr(&a.value, ctx, bad);
                }
            }
            Expr::Index { target, index, .. } => {
                walk_expr(target, ctx, bad);
                walk_expr(index, ctx, bad);
            }
            Expr::Attribute { target, .. } => walk_expr(target, ctx, bad),
            Expr::Array(items, _) => {
                for it in items {
                    walk_expr(it, ctx, bad);
                }
            }
            Expr::Dict(pairs, _) => {
                for (k, v) in pairs {
                    walk_expr(k, ctx, bad);
                    walk_expr(v, ctx, bad);
                }
            }
            Expr::Await(a, _) => walk_expr(a, ctx, bad),
            Expr::Generic(base, args, _) => {
                walk_expr(base, ctx, bad);
                for a in args {
                    walk_expr(a, ctx, bad);
                }
            }
            Expr::Ternary(a, b, c, _) => {
                walk_expr(a, ctx, bad);
                walk_expr(b, ctx, bad);
                walk_expr(c, ctx, bad);
            }
            _ => {}
        }
    }

    /// MatchPattern 各变体均无 span 字段，仅做嵌套遍历占位（当前无可查内容）。
    fn walk_pat(p: &MatchPattern) {
        match p {
            MatchPattern::Array(ps) => {
                for x in ps {
                    walk_pat(x);
                }
            }
            MatchPattern::Dict(ps) => {
                for (a, b) in ps {
                    walk_pat(a);
                    walk_pat(b);
                }
            }
            MatchPattern::Or(ps) => {
                for x in ps {
                    walk_pat(x);
                }
            }
            _ => {}
        }
    }

    fn walk_stmt(s: &Stmt, ctx: &str, bad: &mut Vec<String>) {
        match s {
            Stmt::Expr(e) => walk_expr(e, ctx, bad),
            Stmt::Assign { target, value, span, .. } => {
                walk_expr(target, ctx, bad);
                walk_expr(value, ctx, bad);
                check_span(*span, ctx, bad);
            }
            Stmt::Var(v) => {
                check_span(v.span, ctx, bad);
                if let Some(t) = &v.ty {
                    walk_expr(t, ctx, bad);
                }
                if let Some(val) = &v.value {
                    walk_expr(val, ctx, bad);
                }
            }
            Stmt::Const(c) => {
                check_span(c.span, ctx, bad);
                if let Some(t) = &c.ty {
                    walk_expr(t, ctx, bad);
                }
                walk_expr(&c.value, ctx, bad);
            }
            Stmt::Func(f) => {
                check_span(f.span, ctx, bad);
                if let Some(r) = &f.ret_ty {
                    walk_expr(r, ctx, bad);
                }
                for st in &f.body.stmts {
                    walk_stmt(st, ctx, bad);
                }
            }
            Stmt::Class(c) => {
                check_span(c.span, ctx, bad);
                if let Some(e) = &c.extends {
                    walk_expr(e, ctx, bad);
                }
                for st in &c.body {
                    walk_stmt(st, ctx, bad);
                }
            }
            Stmt::If { branches, else_block, span, .. } => {
                check_span(*span, ctx, bad);
                for (cond, blk) in branches {
                    walk_expr(cond, ctx, bad);
                    for st in &blk.stmts {
                        walk_stmt(st, ctx, bad);
                    }
                }
                if let Some(blk) = else_block {
                    for st in &blk.stmts {
                        walk_stmt(st, ctx, bad);
                    }
                }
            }
            Stmt::For { iter, body, span, .. } => {
                check_span(*span, ctx, bad);
                walk_expr(iter, ctx, bad);
                for st in &body.stmts {
                    walk_stmt(st, ctx, bad);
                }
            }
            Stmt::While { cond, body, span } => {
                check_span(*span, ctx, bad);
                walk_expr(cond, ctx, bad);
                for st in &body.stmts {
                    walk_stmt(st, ctx, bad);
                }
            }
            Stmt::Match { expr, arms, span } => {
                check_span(*span, ctx, bad);
                walk_expr(expr, ctx, bad);
                for MatchArm { pattern, guard, body, .. } in arms {
                    walk_pat(pattern);
                    if let Some(g) = guard {
                        walk_expr(g, ctx, bad);
                    }
                    for st in &body.stmts {
                        walk_stmt(st, ctx, bad);
                    }
                }
            }
            Stmt::Return { value: Some(e), span } => {
                check_span(*span, ctx, bad);
                walk_expr(e, ctx, bad);
            }
            _ => {}
        }
    }

    fn probe_dir(dir: &std::path::Path, bad: &mut Vec<String>) {
        let Ok(rd) = std::fs::read_dir(dir) else { return };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                probe_dir(&p, bad);
            } else if p.extension().is_some_and(|x| x == "gd") {
                let Ok(src) = std::fs::read_to_string(&p) else { continue };
                let (file, _errs) = parse(&src);
                if let Some(e) = &file.extends {
                    walk_expr(e, &p.display().to_string(), bad);
                }
                for st in &file.body {
                    walk_stmt(st, &p.display().to_string(), bad);
                }
            }
        }
    }

    #[test]
    fn no_inverted_spans_in_local_corpus() {
        let mut bad = vec![];
        probe_dir(std::path::Path::new("../godot-test"), &mut bad);
        probe_dir(std::path::Path::new("../corpus/feature-showcase"), &mut bad);
        probe_dir(std::path::Path::new("../corpus/bench/pixelorama"), &mut bad);
        assert!(bad.is_empty(), "found {} inverted spans:\n{}", bad.len(), bad.join("\n"));
    }

    /// 模拟编辑器缓冲：前缀截断（用户打到一半）也必须产出良构 span。
    /// 只跑 godot-test 小语料，O(n²) 可承受；发现反转立即 panic 报文件+偏移。
    #[test]
    fn no_inverted_spans_in_truncated_buffers() {
        let mut bad = vec![];
        let mut files = vec![];
        collect_gd(std::path::Path::new("../godot-test"), &mut files);
        for f in files {
            let src = std::fs::read_to_string(&f).unwrap();
            for (cut, _) in src.char_indices().skip(1) {
                let ctx = format!("{}@cut{}", f.display(), cut);
                let (file, _errs) = parse(&src[..cut]);
                if let Some(e) = &file.extends {
                    walk_expr(e, &ctx, &mut bad);
                }
                for st in &file.body {
                    walk_stmt(st, &ctx, &mut bad);
                }
                if !bad.is_empty() {
                    break;
                }
            }
        }
        assert!(bad.is_empty(), "found {} inverted spans in truncated buffers:\n{}", bad.len(), bad.join("\n"));
    }

    fn collect_gd(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(rd) = std::fs::read_dir(dir) else { return };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                collect_gd(&p, out);
            } else if p.extension().is_some_and(|x| x == "gd") {
                out.push(p);
            }
        }
    }

    #[test]
    fn span_slice_rejects_inverted_and_out_of_bounds() {
        let src = "var a = 1"; // ASCII 基准
        assert_eq!(span_slice(src, 0, 3), "var");
        assert_eq!(span_slice(src, 5, 2), "", "反转 span 必须返回空串而非 panic");
        assert_eq!(span_slice(src, 0, 999), src, "越界 end 钳制到 len");
        assert_eq!(span_slice(src, 999, 9999), "");
        let uni = "var 位置 = Vector2.ZERO"; // 含多字节
        assert_eq!(span_slice(uni, 4, 10), "位置", "切片须对齐字符边界");
    }
}
// ───────────────────────── P5 C10 脚本成员面单测 ─────────────────────────

#[cfg(test)]
mod p5_script_member_tests {
    use super::*;
    use crate::hover_tests::{state_with, test_state};

    fn ask(state: &State, line: u32, ch: u32) -> Vec<CompletionItem> {
        let uri: Uri = "file:///tmp/t.gd".parse().unwrap();
        let p: CompletionParams = serde_json::from_value(serde_json::json!({
            "textDocument": {"uri": uri},
            "position": {"line": line, "character": ch}
        }))
        .unwrap();
        let CompletionResponse::Array(items) = completion(state, &p) else {
            panic!("应返回数组");
        };
        items
    }

    /// 层①（本脚本自声明）：self. 位出信号/属性/方法，kind 与 display 约定对齐。
    #[test]
    fn p5_self_layer1_kinds_and_display() {
        let base = test_state();
        let src = "extends Node
signal sig_a(a)
var prop_a
func func_a(t: int) -> void:
	pass
func func_b():
	pass
func _init():
	self.
";
        let state = state_with(&base, src);
        let items = ask(&state, 8, 6); // "	self." 词尾
        let sig = items.iter().find(|i| i.label == "sig_a").unwrap_or_else(|| panic!("self. 面应含信号 sig_a"));
        assert_eq!(sig.kind, Some(CompletionItemKind::EVENT));
        let prop = items.iter().find(|i| i.label == "prop_a").unwrap_or_else(|| panic!("self. 面应含属性 prop_a"));
        assert_eq!(prop.kind, Some(CompletionItemKind::PROPERTY));
        let fa = items.iter().find(|i| i.label == "func_a(…)").unwrap_or_else(|| panic!("带参方法 display 应为 name(…): {:?}", items.iter().filter(|i| i.label.starts_with("func")).map(|i| &i.label).collect::<Vec<_>>()));
        assert_eq!(fa.kind, Some(CompletionItemKind::METHOD));
        assert!(items.iter().any(|i| i.label == "func_b()"), "零参方法 display 应为 name()");
        // self. 位统一 MEMBER 桶（after_dot 分支收尾重写；SELF 桶只用于裸标识符面）。
        assert!(fa.sort_text.as_deref().is_some_and(|s| s.starts_with("10")), "self. 位应入 MEMBER 桶: {:?}", fa.sort_text);
    }

    /// 层①+形参/局部：裸标识符位出本类成员 + 光标函数形参与局部声明。
    #[test]
    fn p5_bare_face_locals_and_params() {
        let base = test_state();
        let src = "var member_v
func _ready(a_param: int):
	var local_v := 1
	local_v
";
        let state = state_with(&base, src);
        let items = ask(&state, 3, 3); // "	local_v" 词尾
        assert!(items.iter().any(|i| i.label == "a_param"), "裸面应含形参");
        assert!(items.iter().any(|i| i.label == "local_v"), "裸面应含局部声明");
        let mv = items.iter().find(|i| i.label == "member_v").unwrap();
        assert!(mv.sort_text.as_deref().is_some_and(|s| s.starts_with("08")), "裸面脚本成员应入 SELF 桶(0x08): {:?}", mv.sort_text);
    }

    /// override 面：内部类 extends 链（光标在类尾不完整 func 行,span 放宽），
    /// static 位只出 static 方法；签名 label = 源码形态含尾冒号。
    #[test]
    fn p5_override_inner_class_static_split() {
        let base = test_state();
        let src = "class Base:
	func m_a() -> void:
		pass
	static func m_s(p: int):
		pass

class Derived extends Base:
	func 
";
        let state = state_with(&base, src);
        let items = ask(&state, 7, 7); // "	func " 尾
        let ma = items.iter().find(|i| i.label == "m_a() -> void:").unwrap_or_else(|| panic!("override 面应含 m_a() -> void:（span 放宽）: {:?}", items.iter().filter(|i| i.label.starts_with('m')).map(|i| &i.label).collect::<Vec<_>>()));
        assert_eq!(ma.insert_text.as_deref(), Some("m_a() -> void:"), "override insert = 签名原样");
        assert!(
            !items.iter().any(|i| i.label == "m_s(p: int):"),
            "普通 func 位不得出 static 方法"
        );
        // static 位
        let src_s = "class Base:
	func m_a() -> void:
		pass
	static func m_s(p: int):
		pass

class Derived extends Base:
	static func 
";
        let state_s = state_with(&base, src_s);
        let items_s = ask(&state_s, 7, 14);
        assert!(items_s.iter().any(|i| i.label == "m_s(p: int):"), "static func 位应出 static 方法");
        assert!(!items_s.iter().any(|i| i.label == "m_a() -> void:"), "static 位不得出非 static 方法");
    }

    /// extends 位：出兄弟内部类、排除光标所在类自身（extends_self）。
    #[test]
    fn p5_extends_inner_class_self_exclude() {
        let base = test_state();
        let src = "class A:
	pass

class B extends 
";
        let state = state_with(&base, src);
        let items = ask(&state, 3, 16); // 行3 "class B extends " 尾
        assert!(items.iter().any(|i| i.label == "A" && i.kind == Some(CompletionItemKind::CLASS)), "extends 位应出兄弟内部类 A");
        assert!(!items.iter().any(|i| i.label == "B"), "extends 位不得出自身 B");
    }

    /// 跨行 lambda 默认值签名**原样**保留（override 面 display 引擎行为）。
    #[test]
    fn p5_signature_raw_multiline_lambda() {
        let base = test_state();
        let src = "class Base:
	func cb(call := func():
		var x = 10):
		pass

class Derived extends Base:
	func 
";
        let state = state_with(&base, src);
        let items = ask(&state, 6, 7);
        let want = "cb(call := func():
		var x = 10):";
        assert!(
            items.iter().any(|i| i.label == want),
            "override display 应保留多行 lambda 原文: {:?}",
            items.iter().filter(|i| i.label.starts_with("cb")).map(|i| &i.label).collect::<Vec<_>>()
        );
    }

    /// P7 C6.3：override 面引擎虚方法——内部类 extends Node → 引擎链
    /// is_virtual（_process）+ Object core 手表（_init/_get/…）。
    #[test]
    fn p7_override_engine_virtuals() {
        let base = test_state();
        let src = "class Inner extends Node:
	func 
";
        let state = state_with(&base, src);
        let items = ask(&state, 1, 7);
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(
            labels.contains(&"_process(delta: float) -> void:"),
            "引擎链 _process 签名: {labels:?}"
        );
        assert!(
            labels.contains(&"_get(property: StringName) -> Variant:"),
            "core 手表 _get（extension_api 之外的编辑器侧虚方法）: {labels:?}"
        );
        assert!(labels.contains(&"_init() -> void:"), "core _init: {labels:?}");
        assert!(
            !labels.contains(&"_draw() -> void:"),
            "Node 链无 _draw（CanvasItem 系）"
        );
        assert!(
            labels.contains(&"_get_property_list() -> Array[Dictionary]:"),
            "core _get_property_list 应出"
        );
    }

    /// P7 C6.3：static 位只出 `_static_init() -> void:`（引擎 static 分支
    /// 不走 ClassDB）。
    #[test]
    fn p7_override_static_static_init() {
        let base = test_state();
        let src = "class Inner extends Node:
	static func 
";
        let state = state_with(&base, src);
        let items = ask(&state, 1, 14);
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(
            labels.contains(&"_static_init() -> void:"),
            "static 位应出 _static_init: {labels:?}"
        );
        assert!(
            !labels.contains(&"_process(delta: float) -> void:"),
            "static 位不得出引擎非 static 虚方法"
        );
    }

    /// P7 C6.3：当前类已声明同名方法 → 引擎项跳过（has_function 语义）。
    #[test]
    fn p7_override_skip_declared() {
        let base = test_state();
        let src = "class Inner extends Node:
	func _process(delta: float):
		pass
	func 
";
        let state = state_with(&base, src);
        let items = ask(&state, 3, 7);
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(
            !labels.contains(&"_process(delta: float) -> void:"),
            "已声明 _process 不得再出引擎项: {labels:?}"
        );
        assert!(
            labels.contains(&"_get(property: StringName) -> Variant:"),
            "未声明的 core 项仍应出: {labels:?}"
        );
    }

    // ── P9 C6.5：builtin 构造 signatureHelp + hover 构造重载 ──

    fn sig_help_at(state: &State, src: &str, needle: &str) -> Option<SignatureHelp> {
        let off = src.find(needle).unwrap() + needle.len();
        let p = SignatureHelpParams {
            context: None,
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: "file:///tmp/t.gd".parse::<Uri>().unwrap() },
                position: offset_to_position(src, off),
            },
            work_done_progress_params: Default::default(),
        };
        signature_help(state, &p)
    }

    /// P9 C6.5c①：`Vector2(` 内出构造重载 signatureHelp（引擎 call hint 同款）。
    #[test]
    fn p9_sig_help_builtin_ctor() {
        let base = test_state();
        let src = "func f():
	var v = Vector2(1, 2)
";
        let state = state_with(&base, src);
        // 光标在 `Vector2(1, ` 之后：argidx=1
        let help = sig_help_at(&state, src, "Vector2(1, ").expect("Vector2(1, 应出构造签名帮助");
        // argidx=1：镜像引擎过滤——0/1 参重载跳过，2 参重载保留
        let labels: Vec<&str> = help.signatures.iter().map(|s| s.label.as_str()).collect();
        assert_eq!(labels, vec!["Vector2(x: float, y: float)"], "argidx=1 只剩 2 参重载: {labels:?}");
        assert_eq!(help.active_parameter, Some(1));
        // 首条签名带语料描述
        assert!(
            help.signatures[0]
                .documentation
                .as_ref()
                .is_some_and(|d| matches!(d, lsp_types::Documentation::String(s) if !s.is_empty())),
            "构造重载应带描述"
        );
    }

    /// P9：`Vector2(` 光标在第一参（argidx=0）→ 默认构造（0 参）仍被过滤（引擎
    /// `p_argidx >= E.arguments.size()` 同款），其余 3 条重载全出。
    #[test]
    fn p9_sig_help_ctor_argidx_zero() {
        let base = test_state();
        let src = "func f():
	var v = Vector2()
";
        let state = state_with(&base, src);
        let help = sig_help_at(&state, src, "Vector2(").expect("构造签名帮助");
        let labels: Vec<&str> = help.signatures.iter().map(|s| s.label.as_str()).collect();
        assert_eq!(labels.len(), 3, "argidx=0 剩 3 条带参重载: {labels:?}");
        assert_eq!(help.active_parameter, Some(0));
    }

    /// P9：未知裸名不出签名帮助（utility/builtin 都不是）。
    #[test]
    fn p9_sig_help_unknown_none() {
        let base = test_state();
        let src = "func f():
	frobnicate(1)
";
        let state = state_with(&base, src);
        assert!(sig_help_at(&state, src, "frobnicate(").is_none());
    }

    /// P9 C6.5c②：builtin 类型名 hover 追加构造重载列表。
    #[test]
    fn p9_hover_builtin_ctor_overloads() {
        let base = test_state();
        let src = "var v = Vector2.ZERO
";
        let st = state_with(&base, src);
        let md = hover_word(
            &st,
            "Vector2",
            false,
        )
        .expect("Vector2 hover");
        assert!(md.contains("构造函数"), "hover 应含构造函数节: {md}");
        assert!(md.contains("Vector2(x: float, y: float)"), "hover 应含 2 参重载: {md}");
        assert!(md.contains("Vector2(from: Vector2i)"), "hover 应含 Vector2i 重载: {md}");
    }
}
