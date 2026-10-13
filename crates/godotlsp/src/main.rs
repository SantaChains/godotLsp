//! `godotlsp` — 独立的、脱离引擎的 GDScript 语言服务器,走 stdio(LSP 3.17)。
//!
//! 分析能力来自 `gdscript-ide`(salsa 增量、场景感知);本二进制负责 JSON-RPC
//! 循环、位置编码转换、项目加载与能力分发。

mod convert;
mod gdscript_docs;
mod globals;
mod state;
mod text;
mod uri;

use std::error::Error;

use gdscript_base::{FileId, FilePosition, TextRange};
use gdscript_ide::WarningOverride;
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use lsp_types::{
    CodeActionParams, ColorPresentationParams, CompletionOptions, CompletionParams,
    ConfigurationItem, ConfigurationParams, DocumentColorParams, DocumentFormattingParams,
    DocumentHighlightParams, DocumentLinkParams, DocumentRangeFormattingParams,
    DocumentSymbolParams, FoldingRangeParams, GotoDefinitionParams, HoverParams, InitializeParams,
    InlayHintParams, PublishDiagnosticsParams, ReferenceParams, RenameParams,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams,
    SemanticTokensServerCapabilities, ServerCapabilities, SignatureHelpOptions,
    SignatureHelpParams, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
    WorkspaceSymbolParams,
};

use crate::convert as cv;
use crate::state::{SECTION, State};

fn main() {
    // `--stdio` 是 vscode-languageclient 等客户端附加的标准传输参数,等价于默认行为。
    match std::env::args().nth(1).as_deref() {
        Some("--version" | "-V") => {
            println!("godotlsp {}", env!("CARGO_PKG_VERSION"));
        }
        Some(other) if other != "--stdio" => {
            eprintln!("[godotlsp] 未知参数:{other}");
            std::process::exit(2);
        }
        _ => {
            if let Err(e) = run() {
                eprintln!("[godotlsp] 致命错误:{e}");
                std::process::exit(1);
            }
        }
    }
}

fn capabilities() -> serde_json::Value {
    let (token_types, token_modifiers) = cv::semantic_tokens_legend();
    let token_types: Vec<lsp_types::SemanticTokenType> = token_types
        .iter()
        .copied()
        .map(lsp_types::SemanticTokenType::new)
        .collect();
    let token_modifiers: Vec<lsp_types::SemanticTokenModifier> = token_modifiers
        .iter()
        .copied()
        .map(lsp_types::SemanticTokenModifier::new)
        .collect();
    let caps = ServerCapabilities {
        position_encoding: Some(lsp_types::PositionEncodingKind::UTF16),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        references_provider: Some(lsp_types::OneOf::Left(true)),
        document_highlight_provider: Some(lsp_types::OneOf::Left(true)),
        document_link_provider: Some(lsp_types::DocumentLinkOptions {
            resolve_provider: Some(false),
            work_done_progress_options: Default::default(),
        }),
        color_provider: Some(lsp_types::ColorProviderCapability::Simple(true)),
        document_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        workspace_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        rename_provider: Some(lsp_types::OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some([".", "@", "$", "%", "/"].into_iter().map(Into::into).collect()),
            ..CompletionOptions::default()
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some([",", "("].into_iter().map(Into::into).collect()),
            retrigger_characters: None,
            work_done_progress_options: Default::default(),
        }),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                work_done_progress_options: Default::default(),
                legend: SemanticTokensLegend {
                    token_types,
                    token_modifiers,
                },
                range: Some(false),
                full: Some(lsp_types::SemanticTokensFullOptions::Bool(true)),
            },
        )),
        inlay_hint_provider: Some(lsp_types::OneOf::Left(true)),
        folding_range_provider: Some(lsp_types::FoldingRangeProviderCapability::Simple(true)),
        code_action_provider: Some(lsp_types::CodeActionProviderCapability::Simple(true)),
        document_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        document_range_formatting_provider: Some(lsp_types::OneOf::Left(true)),
        ..ServerCapabilities::default()
    };
    cv::to_value(&caps)
}

fn run() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();
    let init: InitializeParams = serde_json::from_value(connection.initialize(capabilities())?)?;
    let mut state = State::new();
    if let Some(root) = params_root(&init) {
        state.load_project(root);
    }
    eprintln!(
        "[godotlsp] {} 引擎模型就绪",
        gdscript_api::godot_version()
    );
    // 拉取式配置:zed/helix 等客户端只回 `workspace/configuration`,不主动推
    // `didChangeConfiguration`。声明了 `workspace.configuration` 能力即主动拉一次,
    // 结果与后续推送走同一条 apply_settings 路径。
    let mut pending_pull: Option<RequestId> =
        pull_configuration(&init).map(|req| {
            let id = req.id.clone();
            let _ = connection.sender.send(Message::Request(req));
            id
        });
    // LSP 3.17 生命周期。注意 lsp-server 0.10 的 `handle_shutdown` 会代收 exit
    // 通知(阻塞至多 30s)并回包,返回 true 时正常终止流程已经完结;
    // exit 在未 shutdown 时到达则属异常终止,按规范以非零码退出。
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    // 协议已结束;分离的 io 线程随进程一并销毁。
                    return Ok(());
                }
                on_request(&connection, &state, req);
            }
            Message::Notification(not) => {
                if not.method == "exit" {
                    eprintln!("[godotlsp] 未收到 shutdown 即 exit,以异常码终止");
                    std::process::exit(1);
                }
                on_notification(&connection, &mut state, not);
            }
            Message::Response(resp) => {
                // 目前服务器只发起配置拉取请求;对不上号的响应一律忽略。
                if pending_pull.as_ref() == Some(&resp.id) {
                    pending_pull = None;
                    apply_pulled_configuration(&mut state, resp);
                }
            }
        }
    }
    io_threads.join()?;
    Ok(())
}

/// 客户端声明了配置拉取能力时,构造一次 `workspace/configuration` 请求。
fn pull_configuration(init: &InitializeParams) -> Option<Request> {
    let supported = init
        .capabilities
        .workspace
        .as_ref()
        .and_then(|w| w.configuration)
        .unwrap_or(false);
    if !supported {
        return None;
    }
    let params = ConfigurationParams {
        items: vec![ConfigurationItem {
            scope_uri: None,
            section: Some(SECTION.into()),
        }],
    };
    Some(Request {
        // 负 id:服务器发起的请求与客户端请求(正 id)空间隔离,避免对撞。
        id: RequestId::from(-1),
        method: "workspace/configuration".into(),
        params: cv::to_value(&params),
    })
}

/// 应用配置拉取的响应载荷(第一个条目即 `godotlsp` 配置节)。
fn apply_pulled_configuration(state: &mut State, resp: Response) {
    let result = match resp.response_result {
        Ok(result) => result,
        Err(err) => {
            eprintln!("[godotlsp] 配置拉取被客户端拒绝:{}", err.message);
            return;
        }
    };
    let Some(first) = result.as_array().and_then(|a| a.first().cloned()) else {
        return;
    };
    apply_settings(state, &first);
}

fn params_root(init: &InitializeParams) -> Option<std::path::PathBuf> {
    let to_path = |u: &Uri| crate::uri::uri_to_path(u);
    init.workspace_folders
        .as_ref()
        .and_then(|f| f.first())
        .and_then(|f| to_path(&f.uri))
        .or_else(|| {
            #[allow(deprecated)]
            init.root_uri.as_ref().and_then(to_path)
        })
}

fn on_request(connection: &Connection, state: &State, req: Request) {
    let id = req.id.clone();
    let method = req.method.clone();
    let parsed: Result<
        // 内部小求和类型,让所有 handler 共用一条反序列化路径。
        RequestKind,
        serde_json::Error,
    > = match req.method.as_str() {
        "textDocument/hover" => serde_json::from_value(req.params.clone()).map(RequestKind::Hover),
        "textDocument/definition" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::Definition)
        }
        "textDocument/completion" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::Completion)
        }
        "textDocument/documentSymbol" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::DocumentSymbol)
        }
        "textDocument/references" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::References)
        }
        "textDocument/documentHighlight" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::DocumentHighlight)
        }
        "textDocument/documentLink" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::DocumentLink)
        }
        "textDocument/documentColor" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::DocumentColor)
        }
        "textDocument/colorPresentation" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::ColorPresentation)
        }
        "textDocument/rename" => serde_json::from_value(req.params.clone()).map(RequestKind::Rename),
        "textDocument/prepareRename" => serde_json::from_value(req.params.clone())
            .map(RequestKind::PrepareRename),
        "workspace/symbol" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::WorkspaceSymbol)
        }
        "textDocument/semanticTokens/full" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::SemanticTokens)
        }
        "textDocument/inlayHint" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::InlayHint)
        }
        "textDocument/foldingRange" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::FoldingRange)
        }
        "textDocument/signatureHelp" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::SignatureHelp)
        }
        "textDocument/codeAction" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::CodeAction)
        }
        "textDocument/formatting" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::Formatting)
        }
        "textDocument/rangeFormatting" => {
            serde_json::from_value(req.params.clone()).map(RequestKind::RangeFormatting)
        }
        _ => {
            let resp = Response::new_err(id, METHOD_NOT_FOUND, format!("未支持的方法 {}", req.method));
            let _ = connection.sender.send(Message::Response(resp));
            return;
        }
    };
    let params = match parsed {
        Ok(p) => p,
        Err(e) => {
            let resp = Response::new_err(id, INVALID_PARAMS, format!("参数无效:{e}"));
            let _ = connection.sender.send(Message::Response(resp));
            return;
        }
    };
    // panic 隔离:单个请求的分析崩溃只回错误响应,不拖垮整个服务器
    // (rust-analyzer 同款防线;分析世界通常仍可服务后续请求)。
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handle(state, &params)));
    let resp = match result {
        Ok(Ok(value)) => Response::new_ok(id, value),
        Ok(Err(msg)) => Response::new_err(id, REQUEST_FAILED, msg),
        Err(_) => {
            eprintln!("[godotlsp] 处理 {method} 时 panic,已隔离");
            Response::new_err(id, REQUEST_FAILED, format!("内部错误:处理 {method} 时服务器异常(已恢复)"))
        }
    };
    let _ = connection.sender.send(Message::Response(resp));
}

const METHOD_NOT_FOUND: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;
const REQUEST_FAILED: i32 = -32803;

/// 每个支持的请求一个变体,分发表因此收敛为单个 match。
enum RequestKind {
    Hover(HoverParams),
    Definition(GotoDefinitionParams),
    Completion(CompletionParams),
    DocumentSymbol(DocumentSymbolParams),
    References(ReferenceParams),
    DocumentHighlight(DocumentHighlightParams),
    DocumentLink(DocumentLinkParams),
    DocumentColor(DocumentColorParams),
    ColorPresentation(ColorPresentationParams),
    Rename(RenameParams),
    PrepareRename(lsp_types::TextDocumentPositionParams),
    WorkspaceSymbol(WorkspaceSymbolParams),
    SemanticTokens(SemanticTokensParams),
    InlayHint(InlayHintParams),
    FoldingRange(FoldingRangeParams),
    SignatureHelp(SignatureHelpParams),
    CodeAction(CodeActionParams),
    Formatting(DocumentFormattingParams),
    RangeFormatting(DocumentRangeFormattingParams),
}

/// 处理请求。`Ok(serde_json::Value::Null)` 渲染为协议层的 `null`。
#[allow(deprecated)] // SymbolInformation.deprecated 仍是 LSP 3.17 的组成部分
fn handle(state: &State, req: &RequestKind) -> Result<serde_json::Value, String> {
    let analysis = state.host.analysis();
    let result: Option<serde_json::Value> = match req {
        RequestKind::Hover(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            let doc = state.doc(file).expect("hover 文件已注册");
            // 内核 hover;引擎无文档时,回扫声明名上方的 `##` 文档注释(与 Godot 4.4+ 悬停行为对齐)。
            let mut kernel = analysis.hover(FilePosition { file, offset }).ok().flatten();
            if let Some(h) = kernel.as_mut()
                && h.doc.is_empty()
            {
                let line = doc.index.position(&doc.text, h.range.start).line;
                if let Some(md) = text::doc_comment_above(&doc.text, &doc.index, line) {
                    h.doc = md;
                }
            }
            // 内核无结果或仍无文档:引擎全局面兜底(类名/单例/工具函数/常量/枚举值/注解),
            // 再退语言关键字;内核有文档时,追加官方 zh-CN 文档链接(文档跳转)。
            if kernel.as_ref().is_none_or(|h| h.doc.is_empty()) {
                let fallback = globals_hover_at(&doc.text, offset)
                    .or_else(|| keyword_hover_at(&doc.text, offset));
                if let Some(md) = fallback {
                    let word = cv::word_range_at(&doc.text, offset);
                    return Ok(cv::to_value(&lsp_types::Hover {
                        contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: md,
                        }),
                        range: word.map(|r| doc.index.range(&doc.text, r)),
                    }));
                }
            } else if let Some(h) = kernel.as_mut()
                && let Some(link) = globals_hover_link(&doc.text, offset)
            {
                h.doc.push_str(&format!("\n\n[📖 中文文档]({link})"));
            }
            let Some(h) = kernel else {
                return Ok(serde_json::Value::Null);
            };
            Some(cv::to_value(&cv::hover(&doc.text, &doc.index, h)))
        }
        RequestKind::Definition(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            analysis
                .goto_definition(FilePosition { file, offset })
                .ok()
                .map(|targets| cv::to_value(&cv::nav_targets(targets, &state.resolver())))
        }
        RequestKind::Completion(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position) else {
                return Ok(serde_json::Value::Null);
            };
            analysis
                .completions(FilePosition { file, offset })
                .ok()
                .map(|items| {
                    let mut items = cv::completions(items);
                    // 按名上下文(前字符非 `.`/`@`)合并引擎全局面:
                    // 类、内建类型、单例、工具/内建函数、全局枚举及值、常量。
                    let doc = state.doc(file).expect("completion 文件已注册");
                    if by_name_context(&doc.text, offset) {
                        items.extend(globals::completion_items().iter().cloned());
                    }
                    enrich_completions(state, file, &doc.text, &doc.index, offset, &mut items);
                    cv::to_value(&lsp_types::CompletionResponse::Array(items))
                })
        }
        RequestKind::DocumentSymbol(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.document_symbols(file).ok().map(|syms| {
                let doc = state.doc(file).expect("symbol 文件已注册");
                cv::to_value(&lsp_types::DocumentSymbolResponse::Nested(
                    cv::document_symbols(&doc.text, &doc.index, syms),
                ))
            })
        }
        RequestKind::References(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position) else {
                return Ok(serde_json::Value::Null);
            };
            analysis
                .find_references(FilePosition { file, offset })
                .ok()
                .map(|refs| {
                    cv::to_value(&cv::references(
                        refs,
                        p.context.include_declaration,
                        &state.resolver(),
                    ))
                })
        }
        RequestKind::DocumentHighlight(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = state.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.find_references(FilePosition { file, offset }).ok().map(|refs| {
                // 只取本文档内的出现位置,编辑器据此高亮光标符号。
                let highlights: Vec<lsp_types::DocumentHighlight> = refs
                    .iter()
                    .filter(|r| r.file == file)
                    .map(|r| lsp_types::DocumentHighlight {
                        range: doc.index.range(&doc.text, r.range),
                        kind: Some(lsp_types::DocumentHighlightKind::TEXT),
                    })
                    .collect();
                cv::to_value(&highlights)
            })
        }
        RequestKind::DocumentLink(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = state.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(root) = state.root.clone() else {
                return Ok(serde_json::Value::Null);
            };
            Some(cv::to_value(&text::document_links(&doc.text, &root)))
        }
        RequestKind::DocumentColor(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = state.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            Some(cv::to_value(&text::document_colors(&doc.text)))
        }
        RequestKind::ColorPresentation(p) => {
            Some(cv::to_value(&text::color_presentations(&p.color)))
        }
        RequestKind::Rename(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position) else {
                return Ok(serde_json::Value::Null);
            };
            match analysis.rename(FilePosition { file, offset }, &p.new_name) {
                Ok(Ok(change)) => match cv::workspace_edit(change, &state.resolver()) {
                    Some(edit) => Some(cv::to_value(&edit)),
                    None => return Err("重命名编辑无法映射".into()),
                },
                Ok(Err(e)) => return Err(format!("无法重命名:{e:?}")),
                Err(_) => return Err("重命名被一次编辑取消".into()),
            }
        }
        RequestKind::PrepareRename(p) => {
            let Some((file, offset)) = position(state, p) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = state.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            cv::word_range_at(&doc.text, offset)
                .map(|r| {
                    let placeholder = doc.text[r.start as usize..r.end as usize].to_string();
                    cv::to_value(&lsp_types::PrepareRenameResponse::RangeWithPlaceholder {
                        range: doc.index.range(&doc.text, r),
                        placeholder,
                    })
                })
                .or(Some(serde_json::Value::Null))
        }
        RequestKind::WorkspaceSymbol(p) => {
            analysis.workspace_symbols(&p.query).ok().map(|targets| {
                // 直接读注册表:按 FileId 逐个克隆全文再建行索引是 O(n·文件大小) 的浪费。
                let infos: Vec<lsp_types::SymbolInformation> = targets
                    .iter()
                    .filter_map(|t| {
                        let uri = state.uri_of(t.file)?.parse::<Uri>().ok()?;
                        let doc = state.doc(t.file)?;
                        Some(lsp_types::SymbolInformation {
                            name: t.name.clone(),
                            kind: symbol_kind_of(t.kind),
                            tags: None,
                            deprecated: None,
                            location: lsp_types::Location {
                                uri,
                                range: doc.index.range(&doc.text, t.focus_range),
                            },
                            container_name: None,
                        })
                    })
                    .collect();
                cv::to_value(&infos)
            })
        }
        RequestKind::SemanticTokens(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.semantic_tokens(file).ok().map(|tokens| {
                let doc = state.doc(file).expect("token 文件已注册");
                cv::to_value(&cv::semantic_tokens(&doc.text, &doc.index, tokens))
            })
        }
        RequestKind::InlayHint(p) => {
            if !state.inlay_hints {
                return Ok(serde_json::Value::Null);
            }
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.inlay_hints(file).ok().map(|hints| {
                let doc = state.doc(file).expect("hint 文件已注册");
                cv::to_value(&cv::inlay_hints(&doc.text, &doc.index, hints))
            })
        }
        RequestKind::FoldingRange(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.folding_ranges(file).ok().map(|ranges| {
                let doc = state.doc(file).expect("fold 文件已注册");
                cv::to_value(&cv::folding_ranges(&doc.text, &doc.index, ranges))
            })
        }
        RequestKind::SignatureHelp(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(mut help) = analysis
                .signature_help(FilePosition { file, offset })
                .ok()
                .flatten()
            else {
                return Ok(serde_json::Value::Null);
            };
            let doc = state.doc(file).expect("signature help 文件已注册");
            enrich_signature_docs(state, file, &doc.text, offset, &mut help);
            Some(cv::to_value(&cv::signature_help(help)))
        }
        RequestKind::CodeAction(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = state.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            let offset = doc.index.offset(&doc.text, p.range.start);
            analysis
                .code_actions(FilePosition { file, offset })
                .ok()
                .map(|actions| cv::to_value(&cv::code_actions(actions, &state.resolver())))
        }
        RequestKind::Formatting(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.format(file).ok().flatten().map(|new_text| {
                let doc = state.doc(file).expect("format 文件已注册");
                cv::to_value(&cv::format_edits(&doc.text, &doc.index, new_text))
            })
        }
        RequestKind::RangeFormatting(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            let Some(doc) = state.doc(file) else {
                return Ok(serde_json::Value::Null);
            };
            let start = doc.index.offset(&doc.text, p.range.start);
            let end = doc.index.offset(&doc.text, p.range.end);
            analysis
                .format_range(file, start, end)
                .ok()
                .flatten()
                .map(|(s, e, new_text)| {
                    let edits = vec![lsp_types::TextEdit {
                        range: doc.index.range(&doc.text, TextRange::new(s, e)),
                        new_text,
                    }];
                    cv::to_value(&edits)
                })
        }
    };
    Ok(result.unwrap_or(serde_json::Value::Null))
}

#[allow(clippy::needless_pass_by_value)]
fn symbol_kind_of(k: gdscript_base::SymbolKind) -> lsp_types::SymbolKind {
    match k {
        gdscript_base::SymbolKind::Class => lsp_types::SymbolKind::CLASS,
        gdscript_base::SymbolKind::Function => lsp_types::SymbolKind::FUNCTION,
        gdscript_base::SymbolKind::Method => lsp_types::SymbolKind::METHOD,
        gdscript_base::SymbolKind::Variable => lsp_types::SymbolKind::VARIABLE,
        gdscript_base::SymbolKind::Constant => lsp_types::SymbolKind::CONSTANT,
        gdscript_base::SymbolKind::Enum => lsp_types::SymbolKind::ENUM,
        gdscript_base::SymbolKind::EnumMember => lsp_types::SymbolKind::ENUM_MEMBER,
        gdscript_base::SymbolKind::Signal => lsp_types::SymbolKind::EVENT,
    }
}

fn file_id(state: &State, uri: &Uri) -> Option<FileId> {
    state.id_of(uri.as_str())
}

/// 文本文档位置对应的 `(FileId, 字节偏移)`,未注册则 `None`。
fn position(state: &State, p: &lsp_types::TextDocumentPositionParams) -> Option<(FileId, u32)> {
    let file = file_id(state, &p.text_document.uri)?;
    let doc = state.doc(file)?;
    let offset = doc.index.offset(&doc.text, p.position);
    Some((file, offset))
}

/// 光标是否处于"按名补全"上下文:标识符之前不是 `.`(成员)、`@`(注解),
/// 也不在 `$`/`%` 取节点路径内(节点路径补全由内核独占,合并全局面会污染)。
fn by_name_context(text: &str, offset: u32) -> bool {
    let b = text.as_bytes();
    let mut ws = (offset as usize).min(b.len());
    while ws > 0 && (b[ws - 1].is_ascii_alphanumeric() || b[ws - 1] == b'_') {
        ws -= 1;
    }
    !matches!(ws.checked_sub(1).map(|i| b[i]), Some(b'.') | Some(b'@'))
        && !in_node_path_context(text, offset)
}

/// 光标是否在 `$Path/Seg` / `%Unique/Seg` 取节点路径内。镜像内核
/// `dollar_path_prefix`/`unique_path_prefix` 的向后字节扫描(无词法器感知,
/// 字符串/注释内的误报与内核同级,内核侧还会再以解析树复核)。
fn in_node_path_context(text: &str, offset: u32) -> bool {
    let b = text.as_bytes();
    let is_seg = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = (offset as usize).min(b.len());
    while i > 0 && is_seg(b[i - 1]) {
        i -= 1;
    }
    loop {
        match i.checked_sub(1).map(|k| b[k]) {
            Some(b'$') => return true,
            // `%` 兼作取模运算符:前一字符是操作数(标识符/右括号)时按取模处理,
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

/// 光标处裸标识符的引擎全局面 hover(单例/类/工具函数/常量/枚举值/注解)。
fn globals_hover_at(text: &str, offset: u32) -> Option<String> {
    let range = cv::word_range_at(text, offset)?;
    globals::hover_markdown(&text[range.start as usize..range.end as usize])
}

/// 光标处裸标识符的官方文档链接(命中引擎全局面时)。
fn globals_hover_link(text: &str, offset: u32) -> Option<String> {
    let range = cv::word_range_at(text, offset)?;
    globals::doc_link(&text[range.start as usize..range.end as usize])
}

/// 光标处裸标识符的语言关键字 hover(内核与全局面均未命中时兜底)。
fn keyword_hover_at(text: &str, offset: u32) -> Option<String> {
    let range = cv::word_range_at(text, offset)?;
    globals::keyword_hover(&text[range.start as usize..range.end as usize])
}

/// 补全项增强(服务器侧,不动内核):
/// - 注解项:一行说明 + 完整官方文档;
/// - 成员上下文:接收者类型经引擎模型回填完整签名/类型与文档;
/// - 仍缺 detail 的项(自有成员):声明行原文 + 声明上方 `##` 文档注释。
fn enrich_completions(
    state: &State,
    file: FileId,
    text: &str,
    index: &cv::LineIndex,
    offset: u32,
    items: &mut [lsp_types::CompletionItem],
) {
    // 注解(内核 label 不带 `@`,kind 映射为 PROPERTY 且 detail 恒空)。
    for it in items.iter_mut() {
        if it.detail.is_none()
            && it.kind == Some(lsp_types::CompletionItemKind::PROPERTY)
            && let Some((detail, doc)) = globals::annotation_info(&it.label)
        {
            it.detail = Some(detail.to_owned());
            it.documentation = Some(markdown_doc(doc));
        }
    }

    // 成员上下文:接收者类型 → 完整签名/类型 + 文档。
    if let Some(ty) = member_receiver_type(state, file, text, offset) {
        for it in items.iter_mut() {
            if let Some((detail, doc)) = globals::member_info(&ty, &it.label) {
                it.detail = Some(detail);
                it.documentation = Some(markdown_doc(doc));
            }
        }
    }

    // 自有成员兜底:声明行原文 + 上方 `##` 文档注释。
    if items.iter().any(|it| it.detail.is_none())
        && let Ok(syms) = state.host.analysis().document_symbols(file)
    {
        let mut decls: Vec<(&str, u32)> = Vec::new();
        collect_decls(&syms, &mut decls);
        for it in items.iter_mut() {
            if it.detail.is_some() {
                continue;
            }
            if let Some(&(_, start)) = decls.iter().find(|(n, _)| *n == it.label) {
                // 用 selection_range(名字)而非 range:内核 decl range 可能回吞脚本头。
                let line = index.position(text, start).line;
                let (s, e) = index.line_span(line);
                let raw = text[s..e].trim();
                // 函数/类声明行以 `:` 收尾,去掉以呈现签名本体。
                it.detail = Some(raw.strip_suffix(':').unwrap_or(raw).trim_end().to_owned());
                if let Some(md) = text::doc_comment_above(text, index, line) {
                    it.documentation = Some(markdown_doc(md));
                }
            }
        }
    }
}

/// 递归收集文档符号的 (名字, 声明行锚点=名字起点)。
fn collect_decls<'a>(syms: &'a [gdscript_base::DocumentSymbol], out: &mut Vec<(&'a str, u32)>) {
    for s in syms {
        out.push((&s.name, s.selection_range.start));
        collect_decls(&s.children, out);
    }
}

fn markdown_doc(value: String) -> lsp_types::Documentation {
    lsp_types::Documentation::MarkupContent(lsp_types::MarkupContent {
        kind: lsp_types::MarkupKind::Markdown,
        value,
    })
}

/// 成员上下文 `recv.` 的接收者类型名(内核 hover 的 `ty_label`)。
/// `offset` 为补全位置(紧跟 `.` 之后)。
fn member_receiver_type(state: &State, file: FileId, text: &str, offset: u32) -> Option<String> {
    let bytes = text.as_bytes();
    let mut i = (offset as usize).min(bytes.len());
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    // 须紧跟 `.` 且点前有表达式字符(排除行首点与 `..`)。
    if i < 2 || bytes[i - 1] != b'.' || bytes[i - 2] == b'.' {
        return None;
    }
    // hover 接收者表达式的末字符,取其类型标签。
    let h = state
        .host
        .analysis()
        .hover(gdscript_base::FilePosition {
            file,
            offset: i as u32 - 2,
        })
        .ok()
        .flatten()?;
    h.ty_label
}

/// 签名帮助文档回填(P0-2):内核签名恒无 doc;按调用名回填模型/文档表,
/// 成员调用(`recv.method(`)经接收者 hover 定位类后取成员文档。
fn enrich_signature_docs(
    state: &State,
    file: FileId,
    text: &str,
    offset: u32,
    help: &mut gdscript_base::SignatureHelp,
) {
    for sig in &mut help.signatures {
        if !sig.doc.is_empty() {
            continue;
        }
        let Some(name) = sig.label.split('(').next().map(str::trim) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        if let Some(md) =
            callee_member_doc(state, file, text, offset, name).or_else(|| globals::signature_doc(name))
        {
            sig.doc = md;
        }
    }
}

/// `recv.name(` 形态的成员调用文档;非成员调用返回 None。
fn callee_member_doc(
    state: &State,
    file: FileId,
    text: &str,
    offset: u32,
    name: &str,
) -> Option<String> {
    let bytes = text.as_bytes();
    let cur = (offset as usize).min(bytes.len());
    // 回溯光标前最内层未闭合的 `(`。
    let mut depth = 0i32;
    let mut open = None;
    for i in (0..cur).rev() {
        match bytes[i] {
            b')' => depth += 1,
            b'(' => {
                if depth == 0 {
                    open = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let open = open?;
    // `(` 前恰为调用名,且调用名前是 `.`(成员调用)。
    if !text[..open].ends_with(name) {
        return None;
    }
    let dot = open.checked_sub(name.len() + 1)?;
    if dot == 0 || bytes[dot] != b'.' {
        return None;
    }
    // 接收者末字符 hover → 类型 → 成员文档。
    let h = state
        .host
        .analysis()
        .hover(gdscript_base::FilePosition {
            file,
            offset: dot as u32 - 1,
        })
        .ok()
        .flatten()?;
    let ty = h.ty_label?;
    globals::member_method_doc(&ty, name)
}

fn on_notification(connection: &Connection, state: &mut State, not: Notification) {
    match not.method.as_str() {
        "textDocument/didOpen" => {
            if let Ok(p) = serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(not.params)
            {
                let uri = p.text_document.uri.to_string();
                state.open(&uri, p.text_document.text, Some(p.text_document.version));
                publish_diagnostics(connection, state, &uri);
            }
        }
        "textDocument/didChange" => {
            if let Ok(p) =
                serde_json::from_value::<lsp_types::DidChangeTextDocumentParams>(not.params)
            {
                let uri = p.text_document.uri.to_string();
                let version = Some(p.text_document.version);
                if p.content_changes.is_empty() {
                    return;
                }
                if let Some(file) = state.id_of(&uri) {
                    // 已注册:增量应用 range 编辑。
                    if !state.apply_changes(file, p.content_changes, version) {
                        return;
                    }
                } else {
                    // 未注册的兜底:取最后一个全文变更按打开处理。
                    let Some(text) = p.content_changes.into_iter().next_back().map(|c| c.text)
                    else {
                        return;
                    };
                    state.open(&uri, text, version);
                }
                publish_diagnostics(connection, state, &uri);
            }
        }
        "textDocument/didClose" => {
            if let Ok(p) = serde_json::from_value::<lsp_types::DidCloseTextDocumentParams>(not.params)
            {
                let uri = p.text_document.uri.to_string();
                // 关闭后以磁盘内容为准刷新(未保存的编辑丢弃),保持项目视图一致;
                // 文件已从磁盘消失则移出分析世界。
                state.watched(&uri);
                publish_diagnostics(connection, state, &uri);
            }
        }
        "workspace/didChangeConfiguration" => {
            apply_settings(state, &not.params);
        }
        "workspace/didChangeWatchedFiles" => {
            if let Ok(p) =
                serde_json::from_value::<lsp_types::DidChangeWatchedFilesParams>(not.params)
            {
                for event in p.changes {
                    let uri = event.uri.to_string();
                    state.watched(&uri);
                    // 磁盘变化同步诊断:仍注册的 .gd 重算推送;已删除的推空清单,
                    // 让客户端清掉残留标记。
                    if uri.ends_with(".gd") {
                        if state.id_of(&uri).is_some() {
                            publish_diagnostics(connection, state, &uri);
                        } else {
                            publish(connection, &uri, Vec::new(), None);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// 客户端配置(与 VSCode `godotlsp.*` 注册项一一对应)。
#[derive(serde::Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ClientSettings {
    diagnostics: Option<DiagSettings>,
    inlay_hints: Option<InlayHintSettings>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct DiagSettings {
    strict: Option<bool>,
    /// 忽略的诊断码列表(Godot 编辑器同款,如 `UNUSED_VARIABLE`)。
    ignore: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct InlayHintSettings {
    type_hints: Option<bool>,
}

/// 应用 `didChangeConfiguration` 载荷。`vscode-languageclient` 以
/// `{ godotlsp: {…} }` 形态推送;兼容把配置节直接内嵌的客户端。
/// 注意:警告严格度变化不立即重推全项目诊断,由后续编辑/打开自然刷新。
fn apply_settings(state: &mut State, params: &serde_json::Value) {
    let settings = params.get("settings").unwrap_or(params);
    let scoped = settings.get(SECTION).unwrap_or(settings);
    let Ok(s) = serde_json::from_value::<ClientSettings>(scoped.clone()) else {
        return;
    };
    if let Some(d) = s.diagnostics {
        if let Some(strict) = d.strict {
            state.host.set_warning_override(if strict {
                WarningOverride::Strict
            } else {
                WarningOverride::None
            });
        }
        if let Some(ignore) = d.ignore {
            state.diag_ignore = ignore;
        }
    }
    if let Some(i) = s.inlay_hints
        && let Some(t) = i.type_hints
    {
        state.inlay_hints = t;
    }
}

fn publish_diagnostics(connection: &Connection, state: &State, uri: &str) {
    let Some(file) = state.id_of(uri) else {
        return;
    };
    let Some(doc) = state.doc(file) else {
        return;
    };
    let diags = match state.host.analysis().diagnostics(file) {
        Ok(d) => d,
        Err(_) => return,
    };
    // 客户端忽略表:按 Godot 编辑器同款诊断码(`UNUSED_VARIABLE` 等)过滤。
    let diags = if state.diag_ignore.is_empty() {
        diags
    } else {
        diags
            .into_iter()
            .filter(|d| !state.diag_ignore.iter().any(|c| c == &d.code))
            .collect()
    };
    let lsp_diags = cv::diagnostics(&doc.text, &doc.index, &diags);
    publish(connection, uri, lsp_diags, doc.version);
}

fn publish(
    connection: &Connection,
    uri: &str,
    diags: Vec<lsp_types::Diagnostic>,
    version: Option<i32>,
) {
    let Ok(url) = uri.parse::<Uri>() else {
        return;
    };
    let note = Notification::new(
        "textDocument/publishDiagnostics".to_string(),
        PublishDiagnosticsParams {
            uri: url,
            diagnostics: diags,
            version,
        },
    );
    let _ = connection.sender.send(Message::Notification(note));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_path_context_detection() {
        let t = "var p = $Panel/
var q = %Sprite
var r = 7 % 3
var s = a%b
";
        let l2 = t.find("%").unwrap();
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
        assert!(in_node_path_context(t, (l2 + 1) as u32), "% 后");
        // 取模:"7 % 3" — 光标在结果操作数之后,不得视为路径
        assert!(!in_node_path_context(t, (l3 + 5) as u32), "取模结果后");
        // a%b:% 前是操作数 → 取模
        assert!(!in_node_path_context(t, (l4 + 3) as u32), "a%b 的 b 后");
    }
}
