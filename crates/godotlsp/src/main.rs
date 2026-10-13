//! `godotlsp` — a standalone, engine-free GDScript language server over stdio (LSP 3.17).
//!
//! Analysis comes from `gdscript-ide` (salsa-backed, scene-aware); this binary owns the
//! JSON-RPC loop, position-encoding conversion, project loading, and capability dispatch.

mod convert;
mod state;
mod uri;

use std::error::Error;

use convert::LineIndex;
use gdscript_base::{FileId, FilePosition, TextRange};
use lsp_server::{Connection, Message, Notification, Request, Response};
use lsp_types::{
    CodeActionParams, CompletionOptions, CompletionParams, DocumentFormattingParams,
    DocumentRangeFormattingParams, DocumentSymbolParams, FoldingRangeParams,
    GotoDefinitionParams, HoverParams, InitializeParams, InlayHintParams,
    PublishDiagnosticsParams, ReferenceParams, RenameParams, SemanticTokensLegend,
    SemanticTokensOptions, SemanticTokensParams, SemanticTokensServerCapabilities,
    ServerCapabilities, SignatureHelpOptions, SignatureHelpParams, TextDocumentSyncCapability,
    TextDocumentSyncKind, Uri, WorkspaceSymbolParams,
};

use crate::convert as cv;
use crate::state::State;

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--version" | "-V") => {
            println!("godotlsp {}", env!("CARGO_PKG_VERSION"));
        }
        Some(other) => {
            eprintln!("[godotlsp] unknown argument: {other}");
            std::process::exit(2);
        }
        None => {
            if let Err(e) = run() {
                eprintln!("[godotlsp] fatal: {e}");
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
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        references_provider: Some(lsp_types::OneOf::Left(true)),
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
        "[godotlsp] {} engine model ready",
        gdscript_api::godot_version()
    );
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    // The protocol is over; detached io threads are torn down with the process.
                    return Ok(());
                }
                on_request(&connection, &mut state, req);
            }
            Message::Notification(not) => {
                if not.method == "exit" {
                    std::process::exit(0);
                }
                on_notification(&connection, &mut state, not);
            }
            Message::Response(_) => {}
        }
    }
    io_threads.join()?;
    Ok(())
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

fn on_request(connection: &Connection, state: &mut State, req: Request) {
    let id = req.id.clone();
    let parsed: Result<
        // A tiny internal sum so every handler shares one deserialize path.
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
            let resp = Response::new_err(id, METHOD_NOT_FOUND, format!("unhandled method {}", req.method));
            let _ = connection.sender.send(Message::Response(resp));
            return;
        }
    };
    let params = match parsed {
        Ok(p) => p,
        Err(e) => {
            let resp = Response::new_err(id, INVALID_PARAMS, format!("invalid params: {e}"));
            let _ = connection.sender.send(Message::Response(resp));
            return;
        }
    };
    let result = handle(state, &params);
    let resp = match result {
        Ok(value) => Response::new_ok(id, value),
        Err(msg) => Response::new_err(id, REQUEST_FAILED, msg),
    };
    let _ = connection.sender.send(Message::Response(resp));
}

const METHOD_NOT_FOUND: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;
const REQUEST_FAILED: i32 = -32803;

/// One variant per supported request, so the dispatch table is a single match.
enum RequestKind {
    Hover(HoverParams),
    Definition(GotoDefinitionParams),
    Completion(CompletionParams),
    DocumentSymbol(DocumentSymbolParams),
    References(ReferenceParams),
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

/// Handle a request. `Ok(serde_json::Value::Null)` renders as a protocol `null`.
#[allow(deprecated)] // SymbolInformation.deprecated is still part of LSP 3.17
fn handle(state: &mut State, req: &RequestKind) -> Result<serde_json::Value, String> {
    let analysis = state.host.analysis();
    let result: Option<serde_json::Value> = match req {
        RequestKind::Hover(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.hover(FilePosition { file, offset }).ok().flatten().map(|h| {
                let doc = state.doc(file).expect("hover file registered");
                cv::to_value(&cv::hover(&doc.text, &doc.index, h))
            })
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
                    cv::to_value(&lsp_types::CompletionResponse::Array(cv::completions(items)))
                })
        }
        RequestKind::DocumentSymbol(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.document_symbols(file).ok().map(|syms| {
                let doc = state.doc(file).expect("symbol file registered");
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
        RequestKind::Rename(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position) else {
                return Ok(serde_json::Value::Null);
            };
            match analysis.rename(FilePosition { file, offset }, &p.new_name) {
                Ok(Ok(change)) => match cv::workspace_edit(change, &state.resolver()) {
                    Some(edit) => Some(cv::to_value(&edit)),
                    None => return Err("rename edit could not be mapped".into()),
                },
                Ok(Err(e)) => return Err(format!("cannot rename: {e:?}")),
                Err(_) => return Err("rename cancelled by an edit".into()),
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
                let infos: Vec<lsp_types::SymbolInformation> = targets
                    .iter()
                    .filter_map(|t| {
                        let (uri, text, _) = (state.resolver())(t.file)?;
                        let index = LineIndex::new(&text);
                        Some(lsp_types::SymbolInformation {
                            name: t.name.clone(),
                            kind: symbol_kind_of(t.kind),
                            tags: None,
                            deprecated: None,
                            location: lsp_types::Location {
                                uri: uri.parse::<Uri>().ok()?,
                                range: index.range(&text, t.focus_range),
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
                let doc = state.doc(file).expect("token file registered");
                cv::to_value(&cv::semantic_tokens(&doc.text, &doc.index, tokens))
            })
        }
        RequestKind::InlayHint(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.inlay_hints(file).ok().map(|hints| {
                let doc = state.doc(file).expect("hint file registered");
                cv::to_value(&cv::inlay_hints(&doc.text, &doc.index, hints))
            })
        }
        RequestKind::FoldingRange(p) => {
            let Some(file) = file_id(state, &p.text_document.uri) else {
                return Ok(serde_json::Value::Null);
            };
            analysis.folding_ranges(file).ok().map(|ranges| {
                let doc = state.doc(file).expect("fold file registered");
                cv::to_value(&cv::folding_ranges(&doc.text, &doc.index, ranges))
            })
        }
        RequestKind::SignatureHelp(p) => {
            let Some((file, offset)) = position(state, &p.text_document_position_params) else {
                return Ok(serde_json::Value::Null);
            };
            analysis
                .signature_help(FilePosition { file, offset })
                .ok()
                .flatten()
                .map(|h| cv::to_value(&cv::signature_help(h)))
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
                let doc = state.doc(file).expect("format file registered");
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

/// `(FileId, byte offset)` for a text-document position, or `None` if unregistered.
fn position(state: &State, p: &lsp_types::TextDocumentPositionParams) -> Option<(FileId, u32)> {
    let file = file_id(state, &p.text_document.uri)?;
    let doc = state.doc(file)?;
    let offset = doc.index.offset(&doc.text, p.position);
    Some((file, offset))
}

fn on_notification(connection: &Connection, state: &mut State, not: Notification) {
    match not.method.as_str() {
        "textDocument/didOpen" => {
            if let Ok(p) = serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(not.params)
            {
                let uri = p.text_document.uri.to_string();
                state.open(&uri, p.text_document.text);
                publish_diagnostics(connection, state, &uri);
            }
        }
        "textDocument/didChange" => {
            if let Ok(p) =
                serde_json::from_value::<lsp_types::DidChangeTextDocumentParams>(not.params)
            {
                let uri = p.text_document.uri.to_string();
                if let Some(text) = p.content_changes.into_iter().next_back().map(|c| c.text) {
                    state.open(&uri, text);
                    publish_diagnostics(connection, state, &uri);
                }
            }
        }
        "textDocument/didClose" => {
            if let Ok(p) = serde_json::from_value::<lsp_types::DidCloseTextDocumentParams>(not.params)
            {
                let uri = p.text_document.uri.to_string();
                publish_empty(connection, &uri);
            }
        }
        "workspace/didChangeWatchedFiles" => {
            if let Ok(p) =
                serde_json::from_value::<lsp_types::DidChangeWatchedFilesParams>(not.params)
            {
                for event in p.changes {
                    let uri = event.uri.to_string();
                    state.watched(&uri);
                }
            }
        }
        _ => {}
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
    let lsp_diags = cv::diagnostics(&doc.text, &doc.index, &diags);
    publish(connection, uri, lsp_diags);
}

fn publish_empty(connection: &Connection, uri: &str) {
    publish(connection, uri, Vec::new());
}

fn publish(connection: &Connection, uri: &str, diags: Vec<lsp_types::Diagnostic>) {
    let Ok(url) = uri.parse::<Uri>() else {
        return;
    };
    let note = Notification::new(
        "textDocument/publishDiagnostics".to_string(),
        PublishDiagnosticsParams {
            uri: url,
            diagnostics: diags,
            version: None,
        },
    );
    let _ = connection.sender.send(Message::Notification(note));
}
