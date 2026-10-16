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
use gdscript_syntax::ast::{ClassDecl, ConstDecl, EnumDecl, FuncDecl, SignalDecl, Stmt, VarDecl};
use gdscript_syntax::parse;
use gdscript_syntax::semantic::{analyze_with, FileInfo, ProjectContext};
use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::Uri;
use lsp_types::*;

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

    // 五级摄取梯度（简化版）：workspace .godot/extension_api.json -> 内嵌出厂面。
    #[allow(deprecated)]
    let root = initialize_params
        .root_uri
        .or_else(|| initialize_params.workspace_folders.and_then(|f| f.into_iter().next().map(|f| f.uri)))
        .and_then(|u| url_to_path(&u));

    let embedded_json = include_str!("../../gdscript-engine/testdata/extension_api.json");
    let (engine_json, engine_source) = if let Some(ref r) = root {
        let candidate = r.join(".godot/extension_api.json");
        if let Ok(text) = std::fs::read_to_string(&candidate) {
            if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                (text, "workspace")
            } else {
                (embedded_json.to_string(), "embedded(parse-failed)")
            }
        } else {
            (embedded_json.to_string(), "embedded")
        }
    } else {
        (embedded_json.to_string(), "embedded(no-root)")
    };

    let engine = std::sync::Arc::new(EngineApi::from_json(&engine_json).expect("解析引擎模型"));
    let real_lower: HashSet<String> =
        engine.classes.keys().chain(engine.builtins.keys()).map(|s| s.to_lowercase()).collect();

    // 版本对齐：读 project.godot config/features，与 dump 版本比对。
    if let Some(ref r) = root
        && let Ok(pg) = std::fs::read_to_string(r.join("project.godot")) {
            for line in pg.lines() {
                if line.starts_with("config/features") {
                    eprintln!("[godotlsp] 版本对齐: dump={}, project={}", engine.version, line.trim());
                }
            }
        }

    // uid:// -> res:// 映射：扫描项目根下所有 *.uid 文件。
    let mut uid_map: HashMap<String, String> = HashMap::new();
    if let Some(ref r) = root {
        scan_uid_files(r, r, &mut uid_map);
    }

    let project = root.as_deref().and_then(|r| Project::open(r, Some(&engine_json)).ok()).map(std::sync::Arc::new);

    let (diag_tx, diag_rx) = std::sync::mpsc::channel::<(Uri, String)>();
    let mut state = State {
        engine, real_lower, docs: HashMap::new(), asts: HashMap::new(), doc_versions: HashMap::new(),
        cancelled: HashSet::new(), uid_map, project, root, diag_sender: diag_tx,
        snippet_support, config: Config::default(),
    };

    eprintln!("[godotlsp] 就绪，引擎 {} (来源: {})，项目文件 {}，uid 映射 {}",
        state.engine.version, engine_source,
        state.project.as_ref().map(|p| p.scripts.len()).unwrap_or(0),
        state.uid_map.len());

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
                // 请求级 panic 隔离：单请求崩溃不拖死进程。
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    handle_request(req, &mut state)
                }));
                match result {
                    Ok(resp) => connection.sender.send(Message::Response(resp)).expect("send response"),
                    Err(_) => eprintln!("[godotlsp] 请求处理 panic，已隔离"),
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
        return Some(hover_md(word.to_string(), hoverdata::class_brief(word), doclinks::class_url(word, real)));
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
fn scored_fuzzy(prefix: &str, label: &str, freq_boost: bool) -> u16 {
    if prefix.is_empty() && freq_boost {
        freq::rank(label).map_or(scoring::band::NEUTRAL, scoring::neutral_by_rank)
    } else {
        scoring::fuzzy(prefix, label)
    }
}

fn completion(state: &State, p: &CompletionParams) -> CompletionResponse {    let uri = &p.text_document_position.text_document.uri;
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

    if after_dot {
        if let Some(recv_ty) = recv_type_at(state, src, word_start) {
            items.extend(class_members(&state.engine, &recv_ty));
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
            items.push(CompletionItem {
                label: f.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(render_function(f)),
                ..CompletionItem::default()
            });
        }
        for s in &state.engine.singletons {
            items.push(CompletionItem {
                label: s.name.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some(format!("{} (单例)", s.ty)),
                ..CompletionItem::default()
            });
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
        // 编译器内建函数（len/preload/range…，不在 extension_api.json）。
        for (name, bf) in hoverdata::gdscript_fns() {
            items.push(CompletionItem {
                label: name,
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(bf.0),
                ..CompletionItem::default()
            });
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

/// 在 `recv.<|` 位置向前找 recv 标识符并推断其类型。
/// 光标在 `.` 之后（after_dot 已判定），扫描从 `.` 前开始；
/// 变量（AST 语义解析）→ 静态类名 / 内建类名回退（`Node.ProcessMode` 补全）。
fn recv_type_at(state: &State, src: &str, at: usize) -> Option<String> {
    let b = src.as_bytes();
    let mut j = at - 1; // 跳过紧邻的 '.'
    while j > 0 && (b[j - 1].is_ascii_alphanumeric() || b[j - 1] == b'_') {
        j -= 1;
    }
    let recv = &src[j..at - 1];
    if recv.is_empty() {
        return None;
    }
    let info = FileInfo::from_file(&parse(src).0);
    info.resolve_var(src, recv).or_else(|| {
        if state.engine.class_by_name(recv).is_some() || state.engine.builtin_by_name(recv).is_some() {
            Some(recv.to_string())
        } else {
            None
        }
    })
}

/// 收集某类（含祖先）的方法与属性补全项。
fn class_members(engine: &EngineApi, class: &str) -> Vec<CompletionItem> {
    let mut items: Vec<CompletionItem> = Vec::new();
    for cls in engine.ancestor_chain(class) {
        if let Some(c) = engine.classes.get(cls) {
            for m in &c.methods {
                items.push(CompletionItem {
                    label: m.name.clone(),
                    kind: Some(CompletionItemKind::METHOD),
                    detail: Some(render_function(m)),
                    ..CompletionItem::default()
                });
            }
            for p in &c.properties {
                items.push(CompletionItem {
                    label: p.name.clone(),
                    kind: Some(CompletionItemKind::PROPERTY),
                    detail: Some(format!("{}: {}", p.name, p.ty)),
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
            items.push(CompletionItem {
                label: m.name.clone(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(render_function(m)),
                ..CompletionItem::default()
            });
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

/// 用源文件切片把类型表达式粗渲染成字符串（hover/symbol detail 用，不求完美）。
fn expr_to_text(src: &str, e: &gdscript_syntax::ast::Expr) -> String {
    let (a, b) = expr_span(e);
    src[a.min(src.len())..b.min(src.len())].to_string()
}

fn expr_span(e: &gdscript_syntax::ast::Expr) -> (usize, usize) {
    use gdscript_syntax::ast::Expr::*;
    match e {
        Literal(_) => (0, 0),
        Ident(i) => i.span,
        Binary(_, _, _, sp) => *sp,
        Unary(_, _, sp) => *sp,
        Call { span, .. } => *span,
        Index { span, .. } => *span,
        Attribute { span, .. } => *span,
        Lambda(_, sp) => *sp,
        Array(_, sp) => *sp,
        Dict(_, sp) => *sp,
        Await(_, sp) => *sp,
        NodePath { span, .. } => *span,
        Generic(_, _, sp) => *sp,
        Ternary(_, _, _, sp) => *sp,
    }
}

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
    let doc_url: Option<String> = if state.engine.class_by_name(&word).is_some() {
        Some(doclinks::class_url(&word, real))
    } else if state.engine.builtin_by_name(&word).is_some() {
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
                    if let Some(close) = rest.find(')') {
                        if let Some(rgba) = parse_color_args(&rest[..close]) {
                            let end = i + 5 + 1 + close + 1;
                            out.push(ColorInformation {
                                range: offset_range(src, i, end),
                                color: Color { red: rgba[0], green: rgba[1], blue: rgba[2], alpha: rgba[3] },
                            });
                        }
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

    let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();
    // 打开文档：以编辑器缓冲（state.docs）为准。
    let self_path = url::Url::parse(uri.as_str()).ok().and_then(|u| u.to_file_path().ok());
    let mut push_edits = |text: &str, u: &Uri, changes: &mut HashMap<Uri, Vec<TextEdit>>| {
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

// ───────────────────────── 文本工具 ─────────────────────────

fn url_to_path(u: &Uri) -> Option<std::path::PathBuf> {
    let s = u.as_str();
    let path = s.strip_prefix("file://")?;
    let path = path.strip_prefix('/').unwrap_or(path); // Windows: /D:/… → D:/…
    Some(std::path::PathBuf::from(path))
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

fn offset_range(src: &str, start: usize, end: usize) -> Range {
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
        let mut push = |name: &str, locals: &mut Vec<String>, seen: &mut std::collections::HashSet<String>| {
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
        } else if path.extension().and_then(|e| e.to_str()) == Some("uid")
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
            && (ext == "tscn" || ext == "tres")
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
            config: Config::default(),
        }
    }

    fn state_with(state: &State, src: &str) -> State {
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
            config: Config::default(),
        }
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
        for need in ["len", "preload", "PI", "func", "KEY_LEFT", "Sprite2D", "Input"] {
            assert!(all.iter().any(|l| l == need), "全局面应含 {need}: 命中数 {}", all.len());
        }

        // @ 触发：注解面只出注解，不混类名，且恰好全量。
        let annos = labels("@\n", 0, 1);
        assert!(annos.iter().any(|l| l == "export"), "注解面应含 export: {annos:?}");
        assert!(!annos.iter().any(|l| l == "Node2D"), "注解面不应混入类名");
        assert_eq!(annos.len(), hoverdata::annotations().len(), "注解面应恰好是全量注解");

        // 成员面（内建）：Vector3. → 枚举值 / 常量 / 方法。
        let mem = labels("var v := Vector3.\n", 0, 17);
        for need in ["AXIS_X", "ZERO", "normalized"] {
            assert!(mem.iter().any(|l| l == need), "Vector3 成员面应含 {need}: 命中数 {}", mem.len());
        }

        // 成员面（静态类名回退）：Node. → ProcessMode 枚举名。
        let mem = labels("var x = Node.\n", 0, 13);
        assert!(mem.iter().any(|l| l == "ProcessMode"), "Node 成员面应含枚举名 ProcessMode: 命中数 {}", mem.len());
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
        assert!(items.iter().any(|i| i.label == "normalized"), "成员面应识别 partial 词");
        let st = |l: &str| {
            items.iter().find(|i| i.label == l).map(|i| i.sort_text.clone().unwrap()).unwrap_or_else(|| panic!("缺 {l}"))
        };
        assert!(st("normalized") < st("ONE"), "前缀分应排序: normalized={} ONE={}", st("normalized"), st("ONE"));
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
            config: Config::default(),
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
            config: Config::default(),
        };
        // 模拟 didOpen：路径须在 root 之下（code_lens 做 strip_prefix）。
        let gd = root.join("main.gd");
        let uri = path_to_uri(&gd).unwrap();
        let src = std::fs::read_to_string(&gd).unwrap();
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
            config: Config::default(),
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
