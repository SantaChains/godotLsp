//! `godotlsp` —— 独立 GDScript 语言服务器（R4 最小真实版）。
//!
//! LSP 3.17 over stdio，分析内核来自 workspace 自研 crate：
//! gdscript-syntax（词法/语法）、gdscript-engine（引擎模型）、gdscript-project（跨文件）。
//!
//! 支持：initialize/shutdown/exit、didOpen/didChange、publishDiagnostics、
//! hover、completion。

use std::collections::{HashMap, HashSet};

use gdscript_engine::EngineApi;
use gdscript_project::Project;
use gdscript_syntax::ast::{ClassDecl, ConstDecl, EnumDecl, FuncDecl, SignalDecl, Stmt, VarDecl};
use gdscript_syntax::parse;
use gdscript_syntax::semantic::{analyze_with, FileInfo, ProjectContext};
use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::Uri;
use lsp_types::*;

mod doclinks;
mod hoverdata;
mod langdocs;

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
    };

    eprintln!("[godotlsp] 就绪，引擎 {} (来源: {})，项目文件 {}，uid 映射 {}",
        state.engine.version, engine_source,
        state.project.as_ref().map(|p| p.scripts.len()).unwrap_or(0),
        state.uid_map.len());

    // 后台诊断线程：150ms 去抖，击键不直接触发 parse+analyze。
    let conn_sender = connection.sender.clone();
    let engine_arc = state.engine.clone();
    let project_arc = state.project.clone();

    // 诊断线程：收到任务后等 150ms，期间有新任务就用最新的。
    std::thread::spawn(move || {
        let recv = diag_rx;
        let conn_sender = conn_sender;
        let engine = engine_arc;
        let project = project_arc;
        loop {
            let Ok((uri, src)) = recv.recv() else { break };
            let mut latest = (uri, src);
            while let Ok((u, s)) = recv.recv_timeout(std::time::Duration::from_millis(150)) {
                latest = (u, s);
            }
            let (uri, src) = latest;
            // 完整分析：parse errors + 语义错误（未声明标识符、类型错误）。
            let (file, parse_errs) = parse(&src);
            let mut diags: Vec<Diagnostic> = parse_errs.iter().map(|e| Diagnostic {
                range: offset_range(&src, e.start, e.end),
                severity: Some(DiagnosticSeverity::ERROR),
                message: e.message.clone(),
                ..Diagnostic::default()
            }).collect();
            let proj_ctx: Option<&dyn ProjectContext> = project.as_ref().map(|p| p.as_ref() as &dyn ProjectContext);
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
            // 扩展把 godotlsp.* 设置推过来；轻量设置暂不影响内核，接收即忽略。
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

    // 接收者模式：光标落在 `recv.member` 的 member 上。
    if let Some((recv_ty, member)) = resolve_receiver(state, src, offset)
        && let Some(md) = member_hover(state, &recv_ty, &member)
    {
        return Some(md);
    }

    // 单词级：光标词紧贴 '@' 时优先按注解查（@export/@onready/…）。
    let (start, word) = word_span(src, offset)?;
    let is_annotation = start > 0 && src.as_bytes()[start - 1] == b'@';
    let md = hover_word(state, &word, is_annotation)?;
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: md }),
        range: None,
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
fn member_hover(state: &State, recv_ty: &str, member: &str) -> Option<Hover> {
    let engine = &state.engine;
    let real = |b: &str| state.real_lower.contains(b);
    let md = if let Some(prop) = engine.find_property(recv_ty, member) {
        let sig = format!("{}.{}: {}", recv_ty, prop.name, prop.ty);
        let (host, desc) = member_anchor_along(engine, recv_ty, member);
        hover_md(sig, desc, doclinks::member_url(host, member, false, real))
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
        hover_md(sig, hoverdata::member(&enum_host, member), doclinks::member_url(&enum_host, member, true, real))
    } else if let Some((enum_name, value, host)) = enum_value_in_class(engine, recv_ty, member) {
        // 枚举值引用（`Vector3.AXIS_X` / `Vector3.Axis.X` → member 直接或经枚举名命中）。
        let sig = format!("const {recv_ty}.{member} = {value} ({enum_name})");
        let url = doclinks::member_url(&host, member, false, real);
        hover_md(sig, hoverdata::member(&host, member), url)
    } else {
        return None;
    };
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: md }),
        range: None,
    })
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
fn completion(state: &State, p: &CompletionParams) -> CompletionResponse {
    let uri = &p.text_document_position.text_document.uri;
    let Some(src) = state.docs.get(uri) else {
        return CompletionResponse::Array(vec![]);
    };
    let offset = p.text_document_position.position;
    let b = src.as_bytes();
    let at = line_start_offset(src, offset.line as usize) + offset.character as usize;
    let after_dot = at > 0 && b[at - 1] == b'.';
    let mut items: Vec<CompletionItem> = Vec::new();

    if after_dot {
        if let Some(recv_ty) = recv_type_at(src, at) {
            items.extend(class_members(&state.engine, &recv_ty));
        }
    } else {
        for name in state.engine.classes.keys().take(80) {
            items.push(CompletionItem { label: name.clone(), kind: Some(CompletionItemKind::CLASS), ..CompletionItem::default() });
        }
        for name in state.engine.builtins.keys() {
            items.push(CompletionItem { label: name.clone(), kind: Some(CompletionItemKind::TYPE_PARAMETER), ..CompletionItem::default() });
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
    }
    CompletionResponse::Array(items)
}

/// 在 `recv.<|` 位置向前找 recv 标识符并推断其类型。
fn recv_type_at(src: &str, at: usize) -> Option<String> {
    let b = src.as_bytes();
    let mut j = at;
    while j > 0 && (b[j - 1].is_ascii_alphanumeric() || b[j - 1] == b'_') {
        j -= 1;
    }
    let recv = &src[j..at - 1];
    if recv.is_empty() {
        return None;
    }
    // AST cache 由调用方传入；这里从 src 粗扫，后续改走缓存。
    let info = FileInfo::from_file(&parse(src).0);
    info.resolve_var(src, recv)
}

/// 收集某类（含祖先）的方法与属性补全项。
fn class_members(engine: &EngineApi, class: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();
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

fn collect_word(src: &str, word: &str, uri: &Uri, out: &mut Vec<Location>) {
    let b = src.as_bytes();
    let wb = word.as_bytes();
    let mut i = 0;
    while i + wb.len() <= b.len() {
        if &b[i..i + wb.len()] == wb {
            // 确认前后不是标识符字符（词边界）。
            let before_ok = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
            let after_end = i + wb.len();
            let after_ok = after_end >= b.len() || !(b[after_end].is_ascii_alphanumeric() || b[after_end] == b'_');
            if before_ok && after_ok {
                out.push(Location {
                    uri: uri.clone(),
                    range: offset_range(src, i, after_end),
                });
            }
        }
        i += 1;
    }
}

fn workspace_symbol(state: &State, query: &str) -> Vec<SymbolInformation> {
    let mut syms = Vec::new();
    if let (Some(proj), Some(root)) = (&state.project, &state.root) {
        for script in &proj.scripts {
            if let Some(cn) = &script.ast.class_name {
                if (query.is_empty() || cn.name.to_lowercase().contains(&query.to_lowercase()))
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
            // uid:// → res:// 映射（§8.1 UID 规格）。
            if let Some(uid) = literal.strip_prefix("uid://")
                && let Some(res_path) = state.uid_map.get(uid) {
                    let path = root.join(res_path);
                    if path.exists()
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

// ───────────────────────── inlayHints / foldingRange ─────────────────────────

fn inlay_hints(state: &State, p: &InlayHintParams) -> Vec<InlayHint> {
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
    let mut i = 0;
    while i + wb.len() <= b.len() {
        if &b[i..i + wb.len()] == wb {
            let before_ok = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
            let after_end = i + wb.len();
            let after_ok = after_end >= b.len() || !(b[after_end].is_ascii_alphanumeric() || b[after_end] == b'_');
            if before_ok && after_ok {
                out.push(DocumentHighlight {
                    range: offset_range(src, i, after_end),
                    kind: Some(DocumentHighlightKind::READ),
                });
            }
        }
        i += 1;
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
    }
}

#[cfg(test)]
mod hover_tests {
    use super::*;

    fn test_state() -> State {
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
}
