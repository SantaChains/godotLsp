//! POD ↔ LSP 转换:字节偏移 ↔ UTF-16 位置,分析器结构 → `lsp-types`。

use std::collections::HashMap;

use gdscript_base::{
    CodeAction as GCodeAction, CompletionItem as GCompletionItem, CompletionKind, Diagnostic,
    DocumentSymbol as GSymbol, FileEdit, FileId, FoldKind, FoldRange, HoverResult, InlayHint,
    InlayHintKind as GInlayKind, NavTarget, Reference, ReferenceKind, SemanticToken,
    SemanticTokenType, Severity, SignatureHelp as GSignatureHelp, SourceChange, SymbolKind,
    TextRange,
};
use lsp_types::{
    CodeAction as LspCodeAction, CodeActionKind, CodeActionOrCommand, CompletionItem,
    CompletionItemKind, Diagnostic as LspDiagnostic, DiagnosticSeverity, DiagnosticTag,
    DocumentSymbol, FoldingRange, FoldingRangeKind, Hover, HoverContents, InlayHint as LspInlayHint,
    InlayHintKind as LspInlayKind, InlayHintLabel, Location, MarkupContent, MarkupKind, Position,
    Range, SemanticToken as LspSemanticToken, SemanticTokens, SignatureHelp as LspSignatureHelp,
    SignatureInformation, SymbolKind as LspSymbolKind, TextEdit, Uri, WorkspaceEdit,
};
use serde::Serialize;

/// 单文件文本之上的字节偏移 ↔ UTF-16 位置索引。
#[derive(Debug, Clone)]
pub struct LineIndex {
    line_starts: Vec<u32>,
    len: u32,
}

impl LineIndex {
    /// 为 `text` 建立索引。
    #[must_use]
    pub fn new(text: &str) -> Self {
        let mut line_starts = vec![0u32];
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                #[allow(clippy::cast_possible_truncation)]
                line_starts.push(i as u32 + 1);
            }
        }
        #[allow(clippy::cast_possible_truncation)]
        let len = text.len() as u32;
        Self { line_starts, len }
    }

    /// 0 起始的行数。
    #[must_use]
    pub fn line_count(&self) -> u32 {
        #[allow(clippy::cast_possible_truncation)]
        {
            self.line_starts.len() as u32
        }
    }

    pub(crate) fn line_span(&self, line: u32) -> (usize, usize) {
        let line = line.min(self.line_count() - 1);
        #[allow(clippy::cast_possible_truncation)]
        let start = self.line_starts[line as usize] as usize;
        #[allow(clippy::cast_possible_truncation)]
        let end = if (line + 1) as usize == self.line_starts.len() {
            self.len as usize
        } else {
            self.line_starts[(line + 1) as usize] as usize
        };
        (start, end)
    }

    /// 总字节长度。
    #[must_use]
    pub fn len(&self) -> u32 {
        self.len
    }

    /// 字节偏移对应的 LSP 位置。
    #[must_use]
    pub fn position(&self, text: &str, offset: u32) -> Position {
        let offset = offset.min(self.len);
        let line = match self.line_starts.binary_search(&offset) {
            Ok(l) => l,
            Err(l) => l - 1,
        };
        #[allow(clippy::cast_possible_truncation)]
        let line = line as u32;
        let (start, end) = self.line_span(line);
        let byte_col = offset as usize - start;
        let units: usize = text[start..(start + byte_col).min(end)]
            .chars()
            .map(char::len_utf16)
            .sum();
        Position {
            line,
            character: u32::try_from(units).unwrap_or(u32::MAX),
        }
    }

    /// LSP 位置对应的字节偏移(夹紧到文档范围内)。
    #[must_use]
    pub fn offset(&self, text: &str, pos: Position) -> u32 {
        let (start, end) = self.line_span(pos.line);
        let line_text = &text[start..end];
        let mut units = 0u32;
        let mut byte_col = 0usize;
        for ch in line_text.chars() {
            if units + ch.len_utf16() as u32 > pos.character {
                break;
            }
            units += ch.len_utf16() as u32;
            byte_col += ch.len_utf8();
        }
        u32::try_from(start + byte_col).unwrap_or(u32::MAX)
    }

    /// 字节范围对应的 LSP 范围。
    #[must_use]
    pub fn range(&self, text: &str, range: TextRange) -> Range {
        Range {
            start: self.position(text, range.start),
            end: self.position(text, range.end),
        }
    }
}

fn severity(s: Severity) -> Option<DiagnosticSeverity> {
    Some(match s {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
        Severity::Info => DiagnosticSeverity::INFORMATION,
        Severity::Hint => DiagnosticSeverity::HINT,
    })
}

fn symbol_kind(k: SymbolKind) -> LspSymbolKind {
    match k {
        SymbolKind::Class => LspSymbolKind::CLASS,
        SymbolKind::Function => LspSymbolKind::FUNCTION,
        SymbolKind::Method => LspSymbolKind::METHOD,
        SymbolKind::Variable => LspSymbolKind::VARIABLE,
        SymbolKind::Constant => LspSymbolKind::CONSTANT,
        SymbolKind::Enum => LspSymbolKind::ENUM,
        SymbolKind::EnumMember => LspSymbolKind::ENUM_MEMBER,
        SymbolKind::Signal => LspSymbolKind::EVENT,
    }
}

fn completion_kind(k: CompletionKind) -> CompletionItemKind {
    match k {
        CompletionKind::Keyword => CompletionItemKind::KEYWORD,
        CompletionKind::Annotation => CompletionItemKind::PROPERTY,
        CompletionKind::Function => CompletionItemKind::FUNCTION,
        CompletionKind::Variable => CompletionItemKind::VARIABLE,
        CompletionKind::Constant => CompletionItemKind::CONSTANT,
        CompletionKind::Class => CompletionItemKind::CLASS,
        CompletionKind::Enum => CompletionItemKind::ENUM,
        CompletionKind::Signal => CompletionItemKind::EVENT,
    }
}

/// 分析器诊断 → LSP 诊断。
#[must_use]
pub fn diagnostics(text: &str, index: &LineIndex, diags: &[Diagnostic]) -> Vec<LspDiagnostic> {
    diags
        .iter()
        .map(|d| LspDiagnostic {
            range: index.range(text, d.range),
            severity: severity(d.severity),
            code: Some(lsp_types::NumberOrString::String(d.code.clone())),
            code_description: None,
            source: Some("godotlsp".into()),
            message: d.message.clone(),
            related_information: None,
            tags: if d.tags.is_empty() {
                None
            } else {
                Some(
                    d.tags
                        .iter()
                        .map(|t| {
                            if *t == gdscript_base::DiagnosticTag::Deprecated {
                                DiagnosticTag::DEPRECATED
                            } else {
                                DiagnosticTag::UNNECESSARY
                            }
                        })
                        .collect(),
                )
            },
            data: None,
        })
        .collect()
}

/// 分析器 hover → LSP hover。
#[must_use]
pub fn hover(text: &str, index: &LineIndex, h: HoverResult) -> Hover {
    let mut value = match &h.ty_label {
        Some(ty) => format!("```gdscript\n{ty}\n```"),
        None => String::new(),
    };
    if !h.doc.is_empty() {
        if !value.is_empty() {
            value.push_str("\n\n---\n\n");
        }
        value.push_str(&h.doc);
    }
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }),
        range: Some(index.range(text, h.range)),
    }
}

/// 分析器补全项 → LSP 补全项。
#[must_use]
pub fn completions(items: Vec<GCompletionItem>) -> Vec<CompletionItem> {
    items
        .into_iter()
        .map(|i| CompletionItem {
            label: i.label,
            kind: Some(completion_kind(i.kind)),
            insert_text: i.insert_text,
            detail: i.detail,
            ..CompletionItem::default()
        })
        .collect()
}

#[allow(deprecated)]
fn symbols(text: &str, index: &LineIndex, syms: Vec<GSymbol>) -> Vec<DocumentSymbol> {
    syms.into_iter()
        .map(|s| DocumentSymbol {
            name: s.name,
            detail: s.detail,
            kind: symbol_kind(s.kind),
            range: index.range(text, s.range),
            selection_range: index.range(text, s.selection_range),
            children: Some(symbols(text, index, s.children)),
            tags: None,
            deprecated: None,
        })
        .collect()
}

/// 分析器文档符号 → 嵌套 LSP 文档符号。
#[must_use]
pub fn document_symbols(text: &str, index: &LineIndex, syms: Vec<GSymbol>) -> Vec<DocumentSymbol> {
    symbols(text, index, syms)
}

fn with_docs<R>(
    targets: impl IntoIterator<Item = (FileId, TextRange)>,
    get: &dyn Fn(FileId) -> Option<(String, String, u32)>,
    build: impl Fn(Location) -> R,
) -> Vec<R> {
    // 按文件分组:每文件只取一次文本、建一次行索引,避免逐 range 重建
    // (references 在单文件可有数百命中,O(ranges × 文件大小) 会放大成卡顿)。
    let mut order: Vec<FileId> = Vec::new();
    let mut grouped: HashMap<FileId, Vec<TextRange>> = HashMap::new();
    for (f, r) in targets {
        match grouped.get_mut(&f) {
            Some(v) => v.push(r),
            None => {
                grouped.insert(f, vec![r]);
                order.push(f);
            }
        }
    }
    let mut out = Vec::new();
    for f in order {
        let Some(ranges) = grouped.remove(&f) else {
            continue;
        };
        let Some((uri, text, _)) = get(f) else {
            continue;
        };
        let index = LineIndex::new(&text);
        let Ok(uri) = uri.parse::<Uri>() else {
            continue;
        };
        for r in ranges {
            out.push(build(Location {
                uri: uri.clone(),
                range: index.range(&text, r),
            }));
        }
    }
    out
}

/// 解析回调的形状:每个 `FileId` 提供 `(uri, text, _len)`。
pub type DocResolver<'a> = &'a dyn Fn(FileId) -> Option<(String, String, u32)>;

/// 导航目标 → 位置(跳转定义)。
#[must_use]
pub fn nav_targets(targets: Vec<NavTarget>, get: DocResolver) -> Vec<Location> {
    with_docs(
        targets.into_iter().map(|t| (t.file, t.focus_range)),
        get,
        |l| l,
    )
}

/// 引用 → 位置,可选排除声明本身。
#[must_use]
pub fn references(refs: Vec<Reference>, include_decl: bool, get: DocResolver) -> Vec<Location> {
    with_docs(
        refs.into_iter()
            .filter(|r| include_decl || r.kind != ReferenceKind::Declaration)
            .map(|r| (r.file, r.range)),
        get,
        |l| l,
    )
}

/// 跨文件源码变更 → `WorkspaceEdit`。
#[must_use]
#[allow(clippy::mutable_key_type)] // lsp-types 的 `WorkspaceEdit` 以 `Uri` 为键,类型即如此
pub fn workspace_edit(change: SourceChange, get: DocResolver) -> Option<WorkspaceEdit> {
    let mut changes: HashMap<Uri, Vec<TextEdit>> = HashMap::new();
    for FileEdit { file, edits } in change.edits {
        let (uri, text, _) = get(file)?;
        let index = LineIndex::new(&text);
        let uri = uri.parse::<Uri>().ok()?;
        changes
            .entry(uri)
            .or_default()
            .extend(edits.into_iter().map(|e| TextEdit {
                range: index.range(&text, e.range),
                new_text: e.new_text,
            }));
    }
    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
}

/// 字节偏移处的可重命名单词(用于 `prepareRename` 交互)。
#[must_use]
pub fn word_range_at(text: &str, offset: u32) -> Option<TextRange> {
    let b = text.as_bytes();
    let at = offset as usize;
    if at >= b.len() || !is_word(b[at]) {
        return None;
    }
    let mut start = at;
    while start > 0 && is_word(b[start - 1]) {
        start -= 1;
    }
    let mut end = at;
    while end < b.len() && is_word(b[end]) {
        end += 1;
    }
    #[allow(clippy::cast_possible_truncation)]
    Some(TextRange::new(start as u32, end as u32))
}

fn is_word(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

/// 语义 token 图例:token 类型与修饰符,按线上传输顺序。
#[must_use]
pub fn semantic_tokens_legend() -> (&'static [&'static str], &'static [&'static str]) {
    (
        &[
            "function", "method", "variable", "parameter", "property", "class", "enum",
            "enumMember", "type", "decorator", "number", "string", "comment", "event", "constant",
        ],
        &["declaration", "readonly", "static", "defaultLibrary"],
    )
}

fn token_type_index(t: SemanticTokenType) -> u32 {
    match t {
        SemanticTokenType::Function => 0,
        SemanticTokenType::Method => 1,
        SemanticTokenType::Variable => 2,
        SemanticTokenType::Parameter => 3,
        SemanticTokenType::Property => 4,
        SemanticTokenType::Class => 5,
        SemanticTokenType::Enum => 6,
        SemanticTokenType::EnumMember => 7,
        SemanticTokenType::Type => 8,
        SemanticTokenType::Decorator => 9,
        SemanticTokenType::Number => 10,
        SemanticTokenType::String => 11,
        SemanticTokenType::Comment => 12,
        SemanticTokenType::Signal => 13,
        SemanticTokenType::Constant => 14,
    }
}

/// 绝对 token 范围 → LSP 差分编码 token。
#[must_use]
pub fn semantic_tokens(text: &str, index: &LineIndex, tokens: Vec<SemanticToken>) -> SemanticTokens {
    let mut prev_line = 0u32;
    let mut prev_char = 0u32;
    let data: Vec<LspSemanticToken> = tokens
        .into_iter()
        .map(|t| {
            let start = index.position(text, t.range.start);
            let length = t.range.end - t.range.start;
            let line = start.line;
            let character = start.character;
            let delta_line = line - prev_line;
            let delta_start = if line == prev_line {
                character - prev_char
            } else {
                character
            };
            prev_line = line;
            prev_char = character;
            LspSemanticToken {
                delta_line,
                delta_start,
                length,
                token_type: token_type_index(t.token_type),
                token_modifiers_bitset: t.modifiers,
            }
        })
        .collect();
    SemanticTokens {
        result_id: None,
        data,
    }
}

/// 分析器 inlay hint → LSP inlay hint。
#[must_use]
pub fn inlay_hints(text: &str, index: &LineIndex, hints: Vec<InlayHint>) -> Vec<LspInlayHint> {
    hints
        .into_iter()
        .map(|h| LspInlayHint {
            position: index.position(text, h.offset),
            label: InlayHintLabel::String(h.label),
            kind: Some(match h.kind {
                GInlayKind::Type => LspInlayKind::TYPE,
                GInlayKind::Parameter => LspInlayKind::PARAMETER,
            }),
            text_edits: None,
            tooltip: None,
            padding_left: Some(false),
            padding_right: Some(false),
            data: None,
        })
        .collect()
}

/// 分析器折叠范围 → LSP 折叠范围。
#[must_use]
pub fn folding_ranges(text: &str, index: &LineIndex, ranges: Vec<FoldRange>) -> Vec<FoldingRange> {
    ranges
        .into_iter()
        .map(|r| {
            let start = index.position(text, r.range.start);
            let end = index.position(text, r.range.end);
            FoldingRange {
                start_line: start.line,
                start_character: Some(start.character),
                end_line: end.line,
                end_character: Some(end.character),
                kind: (r.kind == FoldKind::Region).then_some(FoldingRangeKind::Region),
                collapsed_text: None,
            }
        })
        .collect()
}

fn signature_information(s: gdscript_base::SignatureInfo) -> SignatureInformation {
    let doc = |value: String| {
        (!value.is_empty()).then_some(lsp_types::Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }))
    };
    SignatureInformation {
        label: s.label,
        documentation: doc(s.doc),
        parameters: Some(
            s.params
                .into_iter()
                .map(|p| lsp_types::ParameterInformation {
                    label: lsp_types::ParameterLabel::Simple(p.label),
                    documentation: doc(p.doc),
                })
                .collect(),
        ),
        active_parameter: None,
    }
}

/// 分析器签名帮助 → LSP 签名帮助。
#[must_use]
pub fn signature_help(help: GSignatureHelp) -> LspSignatureHelp {
    LspSignatureHelp {
        signatures: help.signatures.into_iter().map(signature_information).collect(),
        active_signature: Some(help.active_signature),
        active_parameter: Some(help.active_parameter),
    }
}

/// 分析器 code action → LSP code action。引擎当前只产出"补类型注解"类修复,
/// 稳定作为首选动作呈现。
#[must_use]
pub fn code_actions(actions: Vec<GCodeAction>, get: DocResolver) -> Vec<CodeActionOrCommand> {
    actions
        .into_iter()
        .filter_map(|a| {
            Some(CodeActionOrCommand::CodeAction(LspCodeAction {
                title: a.title,
                kind: a.kind.map(CodeActionKind::from),
                diagnostics: None,
                edit: Some(workspace_edit(a.edit, get)?),
                command: None,
                data: None,
                is_preferred: Some(true),
                disabled: None,
            }))
        })
        .collect()
}

/// 整文档格式化 → 单个全文编辑。
#[must_use]
pub fn format_edits(text: &str, index: &LineIndex, new_text: String) -> Vec<TextEdit> {
    vec![TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: index.position(text, index.len()),
        },
        new_text,
    }]
}

/// 把 `value` 序列化为线上传输的 JSON 值。
pub(crate) fn to_value<T: Serialize>(value: &T) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}
