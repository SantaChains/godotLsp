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
use gdscript_syntax::semantic::{analyze_with, ProjectContext};
use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::Uri;
use lsp_types::*;

mod doclinks;

struct State {
    engine: EngineApi,
    /// 真实类名小写全集（doclinks 伪类消歧：@GDScript vs 真类 GDScript）。
    real_lower: HashSet<String>,
    /// 打开文档：uri -> 文本。
    docs: HashMap<Uri, String>,
    /// 项目级符号图（打开根目录后加载）。
    project: Option<Project>,
}

fn main() {
    let (connection, io_threads) = Connection::stdio();
    let server_capabilities = serde_json::to_value(ServerCapabilities {
        position_encoding: Some(PositionEncodingKind::UTF16),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec![".".into()]),
            ..CompletionOptions::default()
        }),
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

    let engine_json = include_str!("../../gdscript-engine/testdata/extension_api.json");
    let engine = EngineApi::from_json(engine_json).expect("解析引擎模型");
    let real_lower: HashSet<String> =
        engine.classes.keys().chain(engine.builtins.keys()).map(|s| s.to_lowercase()).collect();

    let root = initialize_params
        .root_uri
        .or_else(|| initialize_params.workspace_folders.and_then(|f| f.into_iter().next().map(|f| f.uri)))
        .and_then(|u| url_to_path(&u));
    let project = root.and_then(|r| Project::open(&r, Some(engine_json)).ok());

    let mut state = State { engine, real_lower, docs: HashMap::new(), project };

    eprintln!("[godotlsp] 就绪，引擎 {}，项目文件 {}", state.engine.version, state.project.as_ref().map(|p| p.scripts.len()).unwrap_or(0));

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
        _ => serde_json::Value::Null,
    };
    Response::new_ok(id, result)
}

fn handle_notification(notif: Notification, state: &mut State, conn: &Connection) {
    match notif.method.as_str() {
        "textDocument/didOpen" => {
            let p: DidOpenTextDocumentParams = serde_json::from_value(notif.params.clone()).unwrap();
            let uri = p.text_document.uri;
            let src = p.text_document.text;
            state.docs.insert(uri.clone(), src.clone());
            publish_diagnostics(conn, &uri, &src, state);
        }
        "textDocument/didChange" => {
            let p: DidChangeTextDocumentParams = serde_json::from_value(notif.params.clone()).unwrap();
            let uri = p.text_document.uri;
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
                publish_diagnostics(conn, &uri, doc, state);
            }
        }
        "workspace/didChangeConfiguration" => {
            // 扩展把 godotlsp.* 设置推过来；轻量设置暂不影响内核，接收即忽略。
        }
        "workspace/didChangeWatchedFiles" => {
            // 扩展监听 **/*.gd 等；项目级 class_name 缓存由客户端侧重建，内核按需忽略。
        }
        "initialized" => {}
        _ => {}
    }
}

fn publish_diagnostics(conn: &Connection, uri: &Uri, src: &str, state: &State) {
    let (file, _parse_errs) = parse(src);
    let analysis = analyze_with(&file, &state.engine, state.project.as_ref().map(|p| p as &dyn ProjectContext));
    let diags: Vec<Diagnostic> = analysis
        .errors
        .into_iter()
        .map(|e| {
            let (start, end) = e.span;
            Diagnostic {
                range: offset_range(src, start, end),
                severity: Some(DiagnosticSeverity::ERROR),
                message: e.message,
                ..Diagnostic::default()
            }
        })
        .collect();
    let _ = conn.sender.send(Message::Notification(Notification::new(
        "textDocument/publishDiagnostics".into(),
        serde_json::to_value(PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: diags,
            version: None,
        }).unwrap(),
    )));
}

/// 光标处单词的 hover：从引擎模型渲染完整签名，附官方文档链接。
fn hover(state: &State, p: &HoverParams) -> Option<Hover> {
    let uri = &p.text_document_position_params.text_document.uri;
    let src = state.docs.get(uri)?;
    let offset = position_to_offset(src, p.text_document_position_params.position)?;

    // 接收者模式：光标落在 `recv.member` 的 member 上。
    if let Some((recv_ty, member)) = resolve_receiver(state, src, offset) {
        if let Some(md) = member_hover(state, &recv_ty, &member) {
            return Some(md);
        }
    }

    let word = word_at(src, offset)?;
    let md = hover_word(state, &word)?;
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: md }),
        range: None,
    })
}

/// 单词级 hover：类/内建/全局函数/单例/常量。
fn hover_word(state: &State, word: &str) -> Option<String> {
    let real = |b: &str| state.real_lower.contains(b);
    let engine = &state.engine;
    if let Some(cls) = engine.class_by_name(word) {
        let link = doclinks::class_url(word, real);
        return Some(format!(
            "```gdscript\nclass {} extends {}\n```\n[文档]({})",
            cls.name,
            cls.inherits.as_deref().unwrap_or("Object"),
            link
        ));
    }
    if engine.builtin_by_name(word).is_some() {
        let link = doclinks::class_url(word, real);
        return Some(format!("```gdscript\n{}\n```\n[文档]({})", word, link));
    }
    if let Some(f) = engine.utility_function(word) {
        let link = doclinks::member_url("@GlobalScope", word, false, real);
        return Some(format!(
            "```gdscript\n{}\n```\n[文档]({})",
            render_function(f),
            link
        ));
    }
    if let Some(ty) = engine.singleton_type(word) {
        let link = doclinks::class_url(ty, real);
        return Some(format!(
            "```gdscript\nconst {}: {} (单例)\n```\n[文档]({})",
            word, ty, link
        ));
    }
    if let Some(c) = engine.global_constants.iter().find(|c| c.name == word) {
        let link = doclinks::member_url("@GlobalScope", word, false, real);
        return Some(format!(
            "```gdscript\nconst {} = {}\n```\n[文档]({})",
            c.name,
            const_value_str(&c.value),
            link
        ));
    }
    None
}

/// 接收者成员 hover：`recv.member` → 查 recv_ty 类的属性/方法。
fn member_hover(state: &State, recv_ty: &str, member: &str) -> Option<Hover> {
    let engine = &state.engine;
    let real = |b: &str| state.real_lower.contains(b);
    let md = if let Some(prop) = engine.find_property(recv_ty, member) {
        format!(
            "```gdscript\n{}.{}: {}\n```\n[文档]({})",
            recv_ty,
            prop.name,
            prop.ty,
            doclinks::member_url(recv_ty, member, false, real)
        )
    } else {
        let f = engine.find_method(recv_ty, member)?;
        format!(
            "```gdscript\n{}.{}\n```\n[文档]({})",
            recv_ty,
            render_function(f),
            doclinks::member_url(recv_ty, member, false, real)
        )
    };
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: md }),
        range: None,
    })
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
    // 向前找成员名结束处，必须紧贴 '.'。
    let mut i = offset;
    while i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
        i -= 1;
    }
    let member = &src[i..offset];
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
    let ty = infer_var_type(state, src, recv)?;
    Some((ty, member.to_string()))
}

/// 粗解析 `var recv: Type` / `@onready var recv = ...` 的类型。
fn infer_var_type(_state: &State, src: &str, name: &str) -> Option<String> {
    for line in src.lines() {
        let trimmed = line.trim_start();
        // var name: Type
        if let Some(rest) = trimmed.strip_prefix("var ") {
            if let Some(col) = rest.find(':') {
                let lhs = rest[..col].trim();
                if lhs == name {
                    let ty = rest[col + 1..].trim().split('=').next()?.trim().to_string();
                    if !ty.is_empty() {
                        return Some(ty);
                    }
                }
            }
        }
        // @onready var name: Type
        if let Some(rest) = trimmed.strip_prefix("@onready ") {
            if let Some(rest) = rest.strip_prefix("var ") {
                if let Some(col) = rest.find(':') {
                    let lhs = rest[..col].trim();
                    if lhs == name {
                        let ty = rest[col + 1..].trim().split('=').next()?.trim().to_string();
                        if !ty.is_empty() {
                            return Some(ty);
                        }
                    }
                }
            }
        }
    }
    None
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
    infer_var_type_simple(src, recv)
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

/// 粗解析 `var name: Type` / `@onready var name: Type` 的类型。
fn infer_var_type_simple(src: &str, name: &str) -> Option<String> {
    for line in src.lines() {
        let t = line.trim_start();
        for prefix in ["var ", "@onready var "] {
            if let Some(rest) = t.strip_prefix(prefix) {
                if let Some(col) = rest.find(':') {
                    if rest[..col].trim() == name {
                        let ty = rest[col + 1..].trim().split('=').next()?.trim().to_string();
                        if !ty.is_empty() {
                            return Some(ty);
                        }
                    }
                }
            }
        }
    }
    None
}

// ───────────────────────── documentSymbol ─────────────────────────

/// 侧边栏符号树：把文件顶层声明转成嵌套 DocumentSymbol。
fn document_symbols(state: &State, p: &DocumentSymbolParams) -> Vec<DocumentSymbol> {
    let uri = &p.text_document.uri;
    let Some(src) = state.docs.get(uri) else {
        return vec![];
    };
    let (file, _) = parse(src);
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
    let b = src.as_bytes();
    let mut i = offset.min(b.len());
    while i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
        i -= 1;
    }
    let mut j = offset;
    while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
        j += 1;
    }
    if i == j {
        None
    } else {
        Some(src[i..j].to_string())
    }
}

fn apply_range_change(doc: &mut String, range: &Range, text: &str) {
    let start = position_to_offset(doc, range.start).unwrap_or(doc.len());
    let end = position_to_offset(doc, range.end).unwrap_or(doc.len());
    doc.replace_range(start..end, text);
}

