//! C8 参数选项面：调用实参位 / 注解实参位的字符串选项补全。
//!
//! 引擎锚点（pinned `.refs/godot` @5b4e0cb0）：
//! - 注解参数表：`modules/gdscript/gdscript_editor.cpp:972` `_find_annotation_arguments`
//! - 调用实参选项：`modules/gdscript/gdscript_editor.cpp:3328` `_find_call_arguments` →
//!   `Object::get_argument_options`（`core/object/core/object/object.cpp:2395`）、
//!   InputEvent action 特判（`modules/gdscript/gdscript_editor.cpp:3181`）
//! - 字符串插入约定：`modules/gdscript/gdscript_editor.cpp:905` `_calculate_string_insertion`
//! - LSP 适配：`modules/gdscript/language_server/gdscript_language_protocol.cpp:554-568`（insertText/text_edit）
//!
//! 语义取舍（与引擎逐条对齐）：
//! - **独占面**：引擎 CALL_ARGUMENTS/ANNOTATION_ARGUMENTS 上下文只出选项、
//!   不出常规标识面；`r_forced = r_result.size() > 0`（gdscript_editor.cpp:3432）
//!   ——调用面零项时回落常规面，注解参数面无条件 forced（:3474，零项也独占）。
//! - **设置 plumbing（P8 C8 二期）**：`workspace/didChangeConfiguration` 的
//!   `settings.gdscript` 段三键（`use_single_quotes`/`add_string_name_literals`/
//!   `add_node_path_literals`，引擎默认全 false）→ State.prefs；
//!   quoted_option 按 expected 语义（StringName→`&`、NodePath→`^`）前缀化。
//! - **既有字面量整串替换**：光标在闭合字符串内时走 textEdit 替换整串
//!   （含引号；`&`/`^` 前缀按引擎 final_code 规则保留），insertText 为裸引号串。
//!
//! 本期未做（引擎有、数据/推断链不在，记入 roadmap）：
//! - `Tween.tween_property` 次参属性面（需实参 0 的局部变量类型推断）
//! - `AnimationPlayer.play` 等动画面（需 tscn AnimationLibrary 解析 + 接收者推断）
//! - `InputMap` 单例方法动作面（`InputMap.has_action(` 等，无 oracle 覆盖）
//! - `@export_tool_button` 图标表（编辑器主题数据，无头不可得——明确不做 #4）

use std::collections::HashSet;

use lsp_types::{CompletionItem, CompletionItemKind, CompletionTextEdit, InsertTextFormat, TextEdit, Uri};

use gdscript_syntax::ast::{Expr, File, FuncDecl, Param, SignalDecl, Stmt};
use gdscript_syntax::semantic::{FileInfo, ProjectContext};

use crate::scoring;
use crate::State;

/// 实参站点的被调方。
enum Callee {
    /// `recv.method(` 或裸 `method(`（receiver=None 即 self）。
    Call { method: String, receiver: Option<String> },
    /// `@annotation(`。
    Annotation(String),
}

/// 光标落在既有字符串字面量内时的跨度信息。
struct LiteralSpan {
    /// 整串起点（含 `&`/`^` 前缀与开引号）。
    full_start: usize,
    /// 整串终点（闭引号之后；未闭合时等于闭引号位置）。
    full_end: usize,
    /// 既有字面量前缀：`&`(StringName) / `^`(NodePath)。
    prefix: Option<char>,
    /// 是否闭合（未闭合不做整串替换）。
    closed: bool,
}

/// 光标处的实参站点（`(` 之后、当前参数内）。
struct ArgSite {
    callee: Callee,
    argidx: usize,
    literal: Option<LiteralSpan>,
    /// 本帧 `(` 的偏移（P8 tween_property 实参 0 文本提取用）。
    open: usize,
}

/// 前向扫描维护的调用/注解括号帧。
struct Frame {
    /// `(` 的字节偏移。
    open: usize,
    method: Option<String>,
    receiver: Option<String>,
    annotation: bool,
    argidx: usize,
}

/// 信号选项的公共名面：`connect`/`is_connected`/`disconnect`/`emit_signal`/`has_signal`
/// 首参（引擎 `Object::get_argument_options`，core/object/object.cpp:2395）。
const SIGNAL_METHODS: &[&str] =
    &["connect", "is_connected", "disconnect", "emit_signal", "has_signal"];

/// `@warning_ignore*` 全量警告名（引擎 `GDScriptWarning::Code` 枚举序，
/// modules/gdscript/gdscript_warning.h:46-95；完成时 `to_lower()`，modules/gdscript/gdscript_editor.cpp:1069）。
/// 三个 DISABLE_DEPRECATED 弃用项按引擎同款剔除。
const WARNING_NAMES: &[&str] = &[
    "unassigned_variable",
    "unassigned_variable_op_assign",
    "unused_variable",
    "unused_local_constant",
    "unused_private_class_variable",
    "unused_parameter",
    "unused_signal",
    "shadowed_variable",
    "shadowed_variable_base_class",
    "shadowed_global_identifier",
    "unreachable_code",
    "unreachable_pattern",
    "standalone_expression",
    "standalone_ternary",
    "incompatible_ternary",
    "untyped_declaration",
    "inferred_declaration",
    "unsafe_property_access",
    "unsafe_method_access",
    "unsafe_cast",
    "unsafe_call_argument",
    "unsafe_void_return",
    "return_value_discarded",
    "static_called_on_instance",
    "missing_tool",
    "redundant_static_unload",
    "redundant_await",
    "missing_await",
    "assert_always_true",
    "assert_always_false",
    "integer_division",
    "narrowing_conversion",
    "int_as_enum_without_cast",
    "int_as_enum_without_match",
    "enum_variable_without_default",
    "empty_file",
    "deprecated_keyword",
    "confusable_identifier",
    "confusable_local_declaration",
    "confusable_local_usage",
    "confusable_capture_reassignment",
    "confusable_temporary_modification",
    "inference_on_variant",
    "native_method_override",
    "get_node_default_without_onready",
    "onready_with_export",
];

// ───────────────────────── 站点检测（纯文本前向扫描） ─────────────────────────

/// 前向跳过字符串字面量，返回下一个待扫下标。
/// 覆盖 `"..."` / `"""..."""` / `'...'`（Godot 4 单引号串）与 `\"` 转义；
/// 单引号对到行尾、三引号对到文末（与 string_literal_at 的错误恢复同向）。
fn skip_string_forward(b: &[u8], i: usize) -> usize {
    let n = b.len();
    let q = b[i];
    if b.get(i..i + 3) == Some(b"\"\"\"") {
        let mut j = i + 3;
        while j < n && b.get(j..j + 3) != Some(b"\"\"\"") {
            if b[j] == b'\\' {
                j += 1;
            }
            j += 1;
        }
        (j + 3).min(n)
    } else {
        let mut j = i + 1;
        while j < n && b[j] != q && b[j] != b'\n' {
            if b[j] == b'\\' {
                j += 1;
            }
            j += 1;
        }
        if j < n && b[j] == q { j + 1 } else { j }
    }
}

/// 从 `(` 处回解析被调方：`(method, receiver, annotation)`。
/// 形态：`name(` / `recv.name(` / `@name(`。
fn parse_callee(src: &str, open: usize) -> (Option<String>, Option<String>, bool) {
    let b = src.as_bytes();
    let mut j = open;
    while j > 0 && b[j - 1].is_ascii_whitespace() {
        j -= 1;
    }
    let end = j;
    while j > 0 && (b[j - 1].is_ascii_alphanumeric() || b[j - 1] == b'_') {
        j -= 1;
    }
    if j == end {
        return (None, None, false);
    }
    let method = src[j..end].to_string();
    // 注解：方法名左侧紧邻 '@'（`@export_range(`；GDScript 不允许其间空白）。
    // 名带 @ 返回，与注解名面（hoverdata::annotations）的键形一致。
    if j >= 1 && b[j - 1] == b'@' {
        return (Some(src[j - 1..end].to_string()), None, true);
    }
    // 接收者：方法名左侧 `.` 再接标识符（只取直接接收者，链式取末段）。
    let mut k = j;
    while k > 0 && b[k - 1].is_ascii_whitespace() {
        k -= 1;
    }
    if k >= 1 && b[k - 1] == b'.' {
        let rend = k - 1;
        let mut s = rend;
        while s > 0 && (b[s - 1].is_ascii_alphanumeric() || b[s - 1] == b'_') {
            s -= 1;
        }
        if s < rend {
            return (Some(method), Some(src[s..rend].to_string()), false);
        }
    }
    (Some(method), None, false)
}

/// 前向扫描 [0, at) 维护括号帧栈；返回所有未闭合帧（栈顶 = 最内层）。
fn scan_frames(src: &str, at: usize) -> Vec<Frame> {
    let b = src.as_bytes();
    let n = at.min(b.len());
    let mut frames: Vec<Frame> = Vec::new();
    let mut brackets = 0usize; // `[`/`{` 深度：其中的逗号不属于调用实参分隔
    let mut i = 0usize;
    while i < n {
        match b[i] {
            b'#' => {
                while i < n && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'"' | b'\'' => i = skip_string_forward(b, i),
            b'(' => {
                let (method, receiver, annotation) = parse_callee(src, i);
                frames.push(Frame { open: i, method, receiver, annotation, argidx: 0 });
                i += 1;
            }
            b')' => {
                frames.pop();
                i += 1;
            }
            b'[' | b'{' => {
                brackets += 1;
                i += 1;
            }
            b']' | b'}' => {
                brackets = brackets.saturating_sub(1);
                i += 1;
            }
            b',' if brackets == 0 => {
                if let Some(f) = frames.last_mut() {
                    f.argidx += 1;
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    frames
}

/// 光标处的实参站点；不在任何调用/注解实参内返回 None。
fn arg_site_at(src: &str, at: usize) -> Option<ArgSite> {
    let frames = scan_frames(src, at);
    let frame = frames.last()?;
    let method = frame.method.clone()?;
    let callee = if frame.annotation {
        Callee::Annotation(method)
    } else {
        Callee::Call { method, receiver: frame.receiver.clone() }
    };
    // 光标处的既有字符串字面量：其开引号须在本帧 `(` 之后，才算"本实参内"。
    let literal = crate::string_literal_at(src, at).and_then(|(cs, ce)| {
        if cs < 2 || cs - 1 <= frame.open {
            return None;
        }
        let b = src.as_bytes();
        let (full_start, prefix) = match b.get(cs - 2) {
            Some(b'&') => (cs - 2, Some('&')),
            Some(b'^') => (cs - 2, Some('^')),
            _ => (cs - 1, None),
        };
        let closed = b.get(ce) == Some(&b'"');
        Some(LiteralSpan {
            full_start,
            full_end: if closed { ce + 1 } else { ce },
            prefix,
            closed,
        })
    });
    Some(ArgSite { callee, argidx: frame.argidx, literal, open: frame.open })
}

// ───────────────────────── 选项发射 ─────────────────────────

/// 实参字符串的期望变体类型（引擎 `_calculate_string_insertion` 的 `p_expected`）。
/// 决定设置开启时的字面量前缀：StringName → `&`、NodePath → `^`（引擎
/// `&` 优先于 `^`，gdscript_editor.cpp:912-919）。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum QuoteKind {
    /// 无字符串类型预期（注解 hint 词等）——永不加前缀。
    Str,
    /// StringName 语义（信号名/输入动作名）。
    StringName,
    /// NodePath 语义（tween_property 属性路径等）。
    NodePath,
}

/// 引号选项的发射参数（编辑器设置 + expected 语义聚合,避免超长参数表）。
#[derive(Clone, Copy)]
struct QuoteStyle {
    prefs: crate::EditorPrefs,
    expected: QuoteKind,
}

impl QuoteStyle {
    fn new(prefs: crate::EditorPrefs, expected: QuoteKind) -> Self {
        Self { prefs, expected }
    }
}

/// 引号选项（镜像 `_calculate_string_insertion`，gdscript_editor.cpp:895-957）。
/// - 前缀判定：`(expected==StringName && add_string_name_literals) || 既有 &`，
///   NodePath 同理 `^`；`&` 优先于 `^`。
/// - label/display = 最终码（含前缀）；insertText = 裸引号串，仅在
///   `should && !has_既有前缀` 时才带前缀（引擎 insert_text 分支同款）。
/// - 有闭合字面量：textEdit 替换整串（保留既有 `&`/`^` 前缀）。
/// - use_single_quotes：引号字符换 `'`（corpus 无单引号夹具，单测锁定）。
fn quoted_option(
    src: &str,
    content: &str,
    kind: CompletionItemKind,
    detail: String,
    site: &ArgSite,
    prefix_word: &str,
    qs: QuoteStyle,
) -> CompletionItem {
    let prefs = qs.prefs;
    let expected = qs.expected;
    let q = if prefs.use_single_quotes { '\'' } else { '"' };
    let quoted = format!("{q}{content}{q}");
    let has_sn = site.literal.as_ref().is_some_and(|l| l.prefix == Some('&'));
    let has_np = site.literal.as_ref().is_some_and(|l| l.prefix == Some('^'));
    let should_sn = (expected == QuoteKind::StringName && prefs.add_string_name_literals) || has_sn;
    let should_np = (expected == QuoteKind::NodePath && prefs.add_node_path_literals) || has_np;
    let final_code = if should_sn {
        format!("&{quoted}")
    } else if should_np {
        format!("^{quoted}")
    } else {
        quoted.clone()
    };
    let (label, insert_text, text_edit) = match &site.literal {
        Some(lit) if lit.closed => {
            let range = crate::offset_range(src, lit.full_start, lit.full_end);
            let new_text = final_code.clone();
            (
                final_code,
                quoted,
                Some(CompletionTextEdit::Edit(TextEdit { range, new_text })),
            )
        }
        _ => {
            // 无闭合字面量：insertText 仅 should && !has 时带前缀（引擎
            // `should_have_… && !has_…` 分支；既有前缀未闭合时保留原样）。
            let it = if (should_sn || should_np) && !has_sn && !has_np {
                final_code.clone()
            } else {
                quoted.clone()
            };
            (it.clone(), it, None)
        }
    };
    CompletionItem {
        label,
        kind: Some(kind),
        detail: Some(detail),
        insert_text: Some(insert_text),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        text_edit,
        sort_text: Some(scoring::sort_text(
            scoring::bucket::MEMBER,
            scoring::fuzzy(prefix_word, content),
        )),
        ..CompletionItem::default()
    }
}

/// 无引号裸名选项（`@export_custom` 的 PropertyHint/PropertyUsageFlags，
/// 引擎直接以枚举键名作 display/insert_text，modules/gdscript/gdscript_editor.cpp:1047）。
fn bare_option(name: &str, detail: String, prefix_word: &str) -> CompletionItem {
    CompletionItem {
        label: name.to_string(),
        kind: Some(CompletionItemKind::CONSTANT),
        detail: Some(detail),
        insert_text: Some(name.to_string()),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        sort_text: Some(scoring::sort_text(
            scoring::bucket::MEMBER,
            scoring::fuzzy(prefix_word, name),
        )),
        ..CompletionItem::default()
    }
}

fn quoted_list(
    src: &str,
    names: &[&str],
    kind: CompletionItemKind,
    detail: &str,
    site: &ArgSite,
    prefix_word: &str,
    qs: QuoteStyle,
) -> Vec<CompletionItem> {
    names
        .iter()
        .map(|n| quoted_option(src, n, kind, detail.to_string(), site, prefix_word, qs))
        .collect()
}

// ───────────────────────── 类型解析 ─────────────────────────

/// 类型注解表达式的根类名（`Ident` / `Generic` 递归根）。
fn type_expr_root(e: &Expr) -> Option<String> {
    match e {
        Expr::Ident(i) => Some(i.name.clone()),
        Expr::Generic(base, _, _) => type_expr_root(base),
        _ => None,
    }
}

/// 脚本 `extends` 的根名（引擎类名 / class_name / res:// 路径原文）。
fn extends_root_name(file: &File) -> Option<String> {
    type_expr_root(file.extends.as_ref()?)
}

/// 沿 extends 找最内层含光标的 FuncDecl（供形参类型解析）。
fn innermost_func(file: &File, at: usize) -> Option<&FuncDecl> {
    fn walk<'a>(stmts: &'a [Stmt], at: usize, best: &mut Option<&'a FuncDecl>) {
        for s in stmts {
            match s {
                Stmt::Func(f) => {
                    if f.span.0 <= at && at <= f.span.1 {
                        *best = Some(f);
                        walk(&f.body.stmts, at, best);
                    }
                }
                Stmt::Class(c) => walk(&c.body, at, best),
                Stmt::If { branches, else_block, .. } => {
                    for (_, b) in branches {
                        walk(&b.stmts, at, best);
                    }
                    if let Some(b) = else_block {
                        walk(&b.stmts, at, best);
                    }
                }
                Stmt::For { body, .. } | Stmt::While { body, .. } => walk(&body.stmts, at, best),
                Stmt::Match { arms, .. } => {
                    for a in arms {
                        walk(&a.body.stmts, at, best);
                    }
                }
                _ => {}
            }
        }
    }
    let mut best = None;
    walk(&file.body, at, &mut best);
    best
}

/// AST 绑定解析：外层函数形参（带注解）或类级变量（带注解）。
fn ast_binding_type(file: &File, at: usize, name: &str) -> Option<String> {
    if let Some(f) = innermost_func(file, at) {
        for p in &f.params {
            if p.name.name == name
                && let Some(ty) = &p.ty
            {
                return type_expr_root(ty);
            }
        }
    }
    for s in &file.body {
        if let Stmt::Var(v) = s
            && v.name.name == name
            && let Some(ty) = &v.ty
        {
            return type_expr_root(ty);
        }
    }
    None
}

/// 接收者类型解析（None = 裸调用，即 self）。
/// 顺序：self → 引擎类/内建名 → 工程 class_name/autoload → AST 绑定 →
/// FileInfo 成员表 + 文本扫描兜底（与 recv_type_at 同源，另补形参）。
fn receiver_type(
    state: &State,
    src: &str,
    at: usize,
    file: &File,
    receiver: Option<&str>,
) -> Option<String> {
    let Some(recv) = receiver else {
        return self_type(state, file);
    };
    if recv == "self" {
        return self_type(state, file);
    }
    if state.engine.class_by_name(recv).is_some() || state.engine.builtin_by_name(recv).is_some() {
        return Some(recv.to_string());
    }
    if let Some(proj) = &state.project {
        if proj.class_name_script(recv).is_some() {
            return Some(recv.to_string());
        }
        if let Some(ty) = proj.autoload_type(recv) {
            return Some(ty);
        }
    }
    ast_binding_type(file, at, recv).or_else(|| FileInfo::from_file(file).resolve_var(src, recv))
}

/// self 类型：extends 根（脚本则取其 class_name / res:// 键；无 extends → None）。
fn self_type(state: &State, file: &File) -> Option<String> {
    let root = extends_root_name(file)?;
    if state.engine.class_by_name(&root).is_some() || state.engine.builtin_by_name(&root).is_some() {
        return Some(root);
    }
    // extends 脚本：优先 class_name，否则合成键。
    if let Some(proj) = &state.project
        && let Some(s) = script_by_root(proj, &root)
    {
        return Some(match &s.ast.class_name {
            Some(cn) => cn.name.clone(),
            None => format!("script:{}", s.path),
        });
    }
    Some(root)
}

/// class_name / res:// 路径 → 脚本。
fn script_by_root<'a>(
    proj: &'a gdscript_project::Project,
    root: &str,
) -> Option<&'a gdscript_project::Script> {
    proj.class_name_script(root).or_else(|| proj.resolve_preload(root))
}

/// 脚本的引擎根类名（沿 extends 脚本链走到引擎类；封顶 8 层，缺省 RefCounted）。
/// 引擎类判定走 `state.engine`（不依赖工程在场），脚本解析走工程索引。
pub(crate) fn script_engine_root(state: &State, start: &File) -> String {
    let mut cur = start;
    for _ in 0..8 {
        let Some(root) = extends_root_name(cur) else {
            return "RefCounted".to_string();
        };
        if state.engine.class_by_name(&root).is_some() || state.engine.builtin_by_name(&root).is_some() {
            return root;
        }
        match state.project.as_ref().and_then(|p| script_by_root(p, &root)) {
            Some(s) => cur = &s.ast,
            None => return "RefCounted".to_string(),
        }
    }
    "RefCounted".to_string()
}

/// 脚本链自身信号（本文件 + extends 脚本链，封顶 8 层）→ (名, 签名 detail)。
fn own_signals(state: &State, file: &File) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut cur = file;
    for _ in 0..8 {
        for s in &cur.body {
            if let Stmt::Signal(d) = s
                && seen.insert(d.name.name.clone())
            {
                out.push((d.name.name.clone(), ast_signal_detail(d)));
            }
        }
        let Some(root) = extends_root_name(cur) else { break };
        if state.engine.class_by_name(&root).is_some() || state.engine.builtin_by_name(&root).is_some() {
            break;
        }
        match state.project.as_ref().and_then(|p| script_by_root(p, &root)) {
            Some(s) => cur = &s.ast,
            None => break,
        }
    }
    out
}

/// AST 信号签名：`signal name(a: T, b)`。
fn ast_signal_detail(s: &SignalDecl) -> String {
    let args = s.params.iter().map(param_detail).collect::<Vec<_>>().join(", ");
    format!("signal {}({})", s.name.name, args)
}

/// 引擎信号签名：`signal name(a: T)`（与 main.rs render_signal 同构）。
fn engine_signal_detail(s: &gdscript_engine::EngineSignal) -> String {
    let args = s
        .arguments
        .iter()
        .map(|a| format!("{}: {}", a.name, a.ty))
        .collect::<Vec<_>>()
        .join(", ");
    format!("signal {}({})", s.name, args)
}

fn param_detail(p: &Param) -> String {
    match &p.ty {
        Some(t) => format!("{}: {}", p.name.name, expr_to_type_name(t)),
        None => p.name.name.clone(),
    }
}

/// 参数类型注解的展示名（Ident/Generic 根名，缺省 Variant）。
fn expr_to_type_name(e: &Expr) -> String {
    type_expr_root(e).unwrap_or_else(|| "Variant".to_string())
}

/// 引擎类继承链信号（链序去重，先声明者胜）。
fn engine_signals(state: &State, class: &str) -> Vec<(String, String)> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for cls in state.engine.ancestor_chain(class) {
        if let Some(c) = state.engine.classes.get(cls) {
            for s in &c.signals {
                if seen.insert(s.name.clone()) {
                    out.push((s.name.clone(), engine_signal_detail(s)));
                }
            }
        }
    }
    out
}

// ───────────────────────── 各面 ─────────────────────────

/// 站点解析的公共上下文（避免面函数参数表过长）。
struct Ctx<'a> {
    state: &'a State,
    /// 当前文档 uri（P8 tween_property 实参 0 类型解析用）。
    uri: &'a Uri,
    src: &'a str,
    /// 光标字节偏移。
    at: usize,
    /// 当前文档 AST（接收者/自身信号解析用）。
    file: &'a File,
    /// 光标前已键入词干（fuzzy 打分用）。
    prefix_word: &'a str,
    /// 编辑器补全设置（`&`/`^` 前缀与引号风格）。
    prefs: crate::EditorPrefs,
}

/// 信号名面：`connect` 等首参 → 接收者信号表（自声明 + 引擎继承链）。
fn signal_items(ctx: &Ctx<'_>, receiver: Option<&str>, site: &ArgSite) -> Vec<CompletionItem> {
    let state = ctx.state;
    let Some(ty) = receiver_type(state, ctx.src, ctx.at, ctx.file, receiver) else {
        return Vec::new();
    };
    let is_self = receiver.is_none() || receiver == Some("self");
    // 工程脚本接收者（self 的 extends 脚本链在 own_signals 内已走）。
    let recv_script = if is_self {
        None
    } else {
        state.project.as_ref().and_then(|p| script_by_root(p, &ty))
    };
    // 自身信号：self 取本文件链；脚本类接收者取其脚本链；引擎类接收者无。
    let own = if is_self {
        own_signals(state, ctx.file)
    } else {
        match recv_script {
            Some(s) => own_signals(state, &s.ast),
            None => Vec::new(),
        }
    };
    // 引擎继承链根：self 走本文件链的引擎根；脚本接收者走其脚本链引擎根。
    let engine_root = if is_self {
        script_engine_root(state, ctx.file)
    } else {
        match recv_script {
            Some(s) => script_engine_root(state, &s.ast),
            None => ty.clone(),
        }
    };
    let mut names = own;
    let mut seen: HashSet<String> = names.iter().map(|(n, _)| n.clone()).collect();
    names.extend(
        engine_signals(state, &engine_root)
            .into_iter()
            .filter(|(n, _)| seen.insert(n.clone())),
    );
    names
        .into_iter()
        .map(|(n, d)| {
            // 信号名是 StringName 语义（引擎 object.cpp:2395 面走
            // _calculate_string_insertion(…, Variant::STRING_NAME)）。
            quoted_option(ctx.src, &n, CompletionItemKind::EVENT, d, site, ctx.prefix_word, QuoteStyle::new(ctx.prefs, QuoteKind::StringName))
        })
        .collect()
}

/// 输入动作面：InputEvent 系接收者的 `*action*` 方法首参 → `[input]` 动作名。
fn input_action_items(ctx: &Ctx<'_>, site: &ArgSite) -> Vec<CompletionItem> {
    let Some(proj) = &ctx.state.project else {
        return Vec::new();
    };
    proj.input_actions()
        .iter()
        .map(|a| {
            // 输入动作名是 StringName 语义（引擎 gdscript_editor.cpp:3188
            // _calculate_string_insertion(…, Variant::STRING_NAME)）。
            quoted_option(
                ctx.src,
                a,
                CompletionItemKind::TEXT,
                "输入动作".to_string(),
                site,
                ctx.prefix_word,
                QuoteStyle::new(ctx.prefs, QuoteKind::StringName),
            )
        })
        .collect()
}

/// 调用实参面。返回 None = 本面不适用（回落常规标识面，引擎 r_forced 语义）。
fn call_options(
    ctx: &Ctx<'_>,
    method: &str,
    receiver: Option<&str>,
    argidx: usize,
    site: &ArgSite,
) -> Option<Vec<CompletionItem>> {
    if argidx == 0 && SIGNAL_METHODS.contains(&method) {
        let items = signal_items(ctx, receiver, site);
        return (!items.is_empty()).then_some(items);
    }
    if argidx == 0
        && method.contains("action")
        && let Some(ty) = receiver_type(ctx.state, ctx.src, ctx.at, ctx.file, receiver)
        && ctx.state.engine.ancestor_chain(&ty).contains(&"InputEvent")
    {
        let items = input_action_items(ctx, site);
        return (!items.is_empty()).then_some(items);
    }
    if argidx == 1
        && method == "tween_property"
        && receiver_is_tween(ctx, receiver)
    {
        let items = tween_property_items(ctx, site);
        return (!items.is_empty()).then_some(items);
    }
    None
}

/// tween_property 接收者是否 Tween 系:常规 receiver_type（注解/类名/单例）
/// miss 后走 P6 resolve（局部 `var tween := get_tree().create_tween()` 的
/// 方法链推断,`create_tween` 返回 Tween）。
fn receiver_is_tween(ctx: &Ctx<'_>, receiver: Option<&str>) -> bool {
    let is_t = |ty: &str| ctx.state.engine.ancestor_chain(ty).contains(&"Tween");
    if let Some(ty) = receiver_type(ctx.state, ctx.src, ctx.at, ctx.file, receiver) {
        return is_t(&ty);
    }
    let Some(recv) = receiver else { return false };
    matches!(
        crate::resolve_name_type(ctx.state, ctx.uri, ctx.src, ctx.at, recv),
        Some(crate::RecvType::Engine(c)) if is_t(&c)
    )
}

/// 提取调用首实参的源码文本（`(` 之后到首个深度 0 逗号；跳过字符串与嵌套
/// 括号）。用于 `tween_property(<obj>, ➡)` 的对象表达式定位。
fn first_arg_text(src: &str, open: usize) -> Option<&str> {
    let b = src.as_bytes();
    let mut i = open + 1;
    let mut depth = 0usize;
    while i < b.len() {
        match b[i] {
            b'"' | b'\'' => i = skip_string_forward(b, i),
            b'(' | b'[' | b'{' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' | b'}' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                i += 1;
            }
            b',' if depth == 0 => {
                let arg = src[open + 1..i].trim();
                return (!arg.is_empty()).then_some(arg);
            }
            _ => i += 1,
        }
    }
    None
}

/// `Tween.tween_property(<obj>, ➡)` 次参属性面（引擎 gdscript_editor.cpp:3093-3162）：
/// 实参 0 的类型 → 脚本链 var（仅 var，不含 const/signal/class）+ 引擎链
/// ClassDB 属性（排除 internal）；expected=NODE_PATH。实参 0 非可解析标识符
/// → 空面回落。
fn tween_property_items(ctx: &Ctx<'_>, site: &ArgSite) -> Vec<CompletionItem> {
    let state = ctx.state;
    // 实参 0 必须是裸标识符（引擎用解析器 datatype,这里用 P6 resolve 对齐;
    // 复杂表达式/字符串路径降级不出面）。
    let Some(obj) = first_arg_text(ctx.src, site.open) else { return Vec::new() };
    let is_ident = obj.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')
        && obj.as_bytes().first().is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_');
    if !is_ident {
        return Vec::new();
    }
    let Some(rt) = crate::resolve_name_type(state, ctx.uri, ctx.src, ctx.at, obj) else {
        return Vec::new();
    };
    let internal = crate::internal_props();
    // (名, 类型 detail) 收集;同名先到先得（脚本链覆盖引擎链）。
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<(String, String)> = Vec::new();
    let push = |name: &str, ty: &str, seen: &mut HashSet<String>, out: &mut Vec<(String, String)>| {
        if seen.insert(name.to_string()) {
            out.push((name.to_string(), ty.to_string()));
        }
    };
    match &rt {
        crate::RecvType::Script { res_path, inner } => {
            let proj = state.project.as_ref();
            // 内部类实例（live AST 类 body var;引擎 CLASS 分支 VARIABLE）。
            if let (Some(inner_name), Some(file)) = (inner, Some(ctx.file)) {
                let cls = file.body.iter().find_map(|st| match st {
                    Stmt::Class(c) if c.name.name == *inner_name => Some(c),
                    _ => None,
                });
                if let Some(c) = cls {
                    for st in &c.body {
                        if let Stmt::Var(v) = st {
                            let ty = v.ty.as_ref().map(expr_to_type_name).unwrap_or_else(|| "Variant".into());
                            push(&v.name.name, &ty, &mut seen, &mut out);
                        }
                    }
                }
            }
            if let Some(proj) = proj {
                let key = proj.script_path_to_key(res_path);
                if let Some(key) = key {
                    for m in proj.script_chain_members(&key) {
                        if m.kind == gdscript_project::DeclKind::Property {
                            push(&m.name, "Variant", &mut seen, &mut out);
                        }
                    }
                }
                // 引擎链 native 属性（SCRIPT 分支后统一 ClassDB::get_property_list）。
                let native = proj.script_engine_base_by_path(res_path);
                if let Some(root) = native {
                    for cls in state.engine.ancestor_chain(&root) {
                        if let Some(c) = state.engine.classes.get(cls) {
                            for p in &c.properties {
                                if !internal.get(cls).is_some_and(|s| s.contains(&p.name)) {
                                    push(&p.name, &p.ty, &mut seen, &mut out);
                                }
                            }
                        }
                    }
                }
            }
        }
        crate::RecvType::Engine(root) => {
            for cls in state.engine.ancestor_chain(root) {
                if let Some(c) = state.engine.classes.get(cls) {
                    for p in &c.properties {
                        if !internal.get(cls).is_some_and(|s| s.contains(&p.name)) {
                            push(&p.name, &p.ty, &mut seen, &mut out);
                        }
                    }
                }
            }
        }
    }
    out.into_iter()
        .map(|(n, ty)| {
            quoted_option(
                ctx.src,
                &n,
                CompletionItemKind::PROPERTY,
                format!("tween 属性: {ty}"),
                site,
                ctx.prefix_word,
                QuoteStyle::new(ctx.prefs, QuoteKind::NodePath),
            )
        })
        .collect()
}

/// 注解参数面（引擎 `_find_annotation_arguments` 表驱动；forced 语义——
/// 注解实参位无条件独占，无项也返回空 Vec）。
fn annotation_options(ctx: &Ctx<'_>, name: &str, argidx: usize, site: &ArgSite) -> Vec<CompletionItem> {
    match name {
        "@export_range" if matches!(argidx, 3..=5) => quoted_list(
            ctx.src,
            &["or_greater", "or_less", "prefer_slider", "hide_control"],
            CompletionItemKind::TEXT,
            "@export_range slider 提示",
            site,
            ctx.prefix_word,
            QuoteStyle::new(ctx.prefs, QuoteKind::Str),
        ),
        "@export_exp_easing" if matches!(argidx, 0..=1) => quoted_list(
            ctx.src,
            &["attenuation", "positive_only"],
            CompletionItemKind::TEXT,
            "@export_exp_easing 提示",
            site,
            ctx.prefix_word,
            QuoteStyle::new(ctx.prefs, QuoteKind::Str),
        ),
        "@rpc" if argidx <= 2 => quoted_list(
            ctx.src,
            &[
                "call_local",
                "call_remote",
                "any_peer",
                "authority",
                "reliable",
                "unreliable",
                "unreliable_ordered",
            ],
            CompletionItemKind::TEXT,
            "@rpc 选项",
            site,
            ctx.prefix_word,
            QuoteStyle::new(ctx.prefs, QuoteKind::Str),
        ),
        "@warning_ignore" | "@warning_ignore_start" | "@warning_ignore_restore" => {
            quoted_list(ctx.src, WARNING_NAMES, CompletionItemKind::TEXT, "GDScript 警告", site, ctx.prefix_word, QuoteStyle::new(ctx.prefs, QuoteKind::Str))
        }
        "@export_custom" if argidx == 0 => global_enum_items(ctx, "PropertyHint"),
        "@export_custom" if argidx == 2 => global_enum_items(ctx, "PropertyUsageFlags"),
        "@export_node_path" if argidx == 0 => node_path_type_items(ctx, site),
        // 引擎对其余注解/参数位同样 forced 空面（gdscript_editor.cpp:3474）。
        _ => Vec::new(),
    }
}

/// `@export_custom` 的全局枚举面（模型 `global_enums`，零手搬）。
fn global_enum_items(ctx: &Ctx<'_>, enum_name: &str) -> Vec<CompletionItem> {
    ctx.state
        .engine
        .global_enums
        .iter()
        .find(|e| e.name == enum_name)
        .map(|e| {
            e.values
                .iter()
                .map(|(n, _)| bare_option(n, format!("{enum_name}.{n}"), ctx.prefix_word))
                .collect()
        })
        .unwrap_or_default()
}

/// `@export_node_path` 首参：`Node` + 引擎 Node 继承树 + 工程 Node 系脚本类
/// （引擎 ClassDB::get_inheriters_from_class + ScriptServer 全局类，:1000）。
fn node_path_type_items(ctx: &Ctx<'_>, site: &ArgSite) -> Vec<CompletionItem> {
    let state = ctx.state;
    let mut names: Vec<String> = vec!["Node".to_string()];
    let mut seen: HashSet<String> = ["Node".to_string()].into_iter().collect();
    let mut cls_names: Vec<&String> = state.engine.classes.keys().collect();
    cls_names.sort_unstable();
    for c in cls_names {
        if state.engine.ancestor_chain(c).contains(&"Node") && seen.insert(c.clone()) {
            names.push(c.clone());
        }
    }
    if let Some(proj) = &state.project {
        let mut script_names: Vec<&str> = proj
            .scripts
            .iter()
            .filter_map(|s| s.ast.class_name.as_ref().map(|cn| cn.name.as_str()))
            .collect();
        script_names.sort_unstable();
        for cn in script_names {
            let Some(s) = proj.class_name_script(cn) else { continue };
            let root = script_engine_root(state, &s.ast);
            if state.engine.ancestor_chain(&root).contains(&"Node")
                && seen.insert(cn.to_string())
            {
                names.push(cn.to_string());
            }
        }
    }
    names
        .into_iter()
        .map(|n| {
            quoted_option(
                ctx.src,
                &n,
                CompletionItemKind::CLASS,
                "Node 类型".to_string(),
                site,
                ctx.prefix_word,
                // 类型名非字符串语义:设置开启也不加前缀(引擎 :1000 面
                // expected 无 STRING 语义)。
                QuoteStyle::new(ctx.prefs, QuoteKind::Str),
            )
        })
        .collect()
}

// ───────────────────────── 入口 ─────────────────────────

/// C8 参数选项面入口。`Some(items)` = 命中站点（ items 可为空：注解参数面
/// forced 独占）；`None` = 站点不适用/调用面零项，调用方回落后续面。
pub(crate) fn argument_options(
    state: &State,
    uri: &Uri,
    src: &str,
    at: usize,
    word_start: usize,
    file: Option<&File>,
) -> Option<Vec<CompletionItem>> {
    let site = arg_site_at(src, at)?;
    let prefix_word = &src[word_start..at.min(src.len())];
    let parsed;
    let file: &File = match file {
        Some(f) => f,
        None => {
            parsed = gdscript_syntax::parse(src).0;
            &parsed
        }
    };
    let ctx = Ctx { state, uri, src, at, file, prefix_word, prefs: state.prefs };
    match &site.callee {
        // 注解参数面：forced 独占（零项也拦）。
        Callee::Annotation(name) => Some(annotation_options(&ctx, name, site.argidx, &site)),
        // 调用实参面：零项回落（引擎 r_forced = size > 0）。
        Callee::Call { method, receiver } => {
            call_options(&ctx, method, receiver.as_deref(), site.argidx, &site)
        }
    }
}


// ───────────────────────── N5：Callable 实参位无括号面 ─────────────────────────

/// N5（P6 C5.3）：实参位形参类型为 Callable → 本脚本方法名 insert **不带
/// 括号**（`hello_world` 而非 `hello_world()`；oracle
/// no_parenthesis_when_callable_is_expected 桶）。非 Callable 位（或形参无
/// 类型注解）出常规 `name()` 形态（oracle local_method_variant_arg_0）。
/// 面独占（argopts 惯例）；callee/形参不可判定（lambda `func(`、未知方法、
/// 形参越界）→ None 回落常规面。引擎锚点：`_find_call_arguments` 的
/// callable 参数特判（modules/gdscript/gdscript_editor.cpp:3328 起）。
///
/// 已知边界（roadmap）：未闭合调用的 AST 恢复缺失——`sort_custom(h` 会把
/// 后续 `func` 声明吞进实参（builtin_*_braces/native 三夹具），根因在
/// gdscript-syntax 词法器括号计数抑制缩进标记，独立债不属本面。
pub fn callable_arg_items(
    state: &State,
    uri: &lsp_types::Uri,
    src: &str,
    at: usize,
) -> Option<Vec<CompletionItem>> {
    let site = arg_site_at(src, at)?;
    let Callee::Call { method, receiver } = &site.callee else {
        return None; // 注解实参位不走本面
    };
    // 字符串字面量实参位不命中（C8 字符串选项面优先——add_string_name
    // 等 string_literals 夹具回归教训）。
    if site.literal.is_some() {
        return None;
    }
    let file = state.asts.get(uri).cloned().unwrap_or_else(|| crate::parse(src).0);
    // 形参类型解析：位 argidx 的类型根。
    let param_ty: Option<String> = match receiver.as_deref() {
        None | Some("self") => {
            // 裸调用 = 本文件方法（live AST；父链方法签名解析留后续）。
            let f = find_local_func(&file, method)?;
            f.params.get(site.argidx).and_then(|p| p.ty.as_ref()).and_then(type_expr_root)
        }
        Some(recv) => {
            // 接收者类型：常规解析，miss 后特判 utility 调用接收者
            // （`create_tween().tween_callback(` → Tween）。
            let ty = receiver_type(state, src, at, &file, Some(recv)).or_else(|| {
                state
                    .engine
                    .utility_functions
                    .iter()
                    .find(|f| f.name == *recv)
                    .filter(|f| !f.return_type.is_empty() && f.return_type != "void")
                    .map(|f| f.return_type.clone())
            })?;
            let ty = ty.as_str();
            // 引擎类方法 → 内建类方法；都不中（脚本类接收者等）→ 保守回落。
            let args = state
                .engine
                .find_method(ty, method)
                .map(|m| &m.arguments)
                .or_else(|| {
                    state.engine.builtin_by_name(ty).and_then(|b| {
                        b.methods.iter().find(|m| m.name == *method).map(|m| &m.arguments)
                    })
                });
            args?.get(site.argidx).map(|a| a.ty.clone())
        }
    };
    // Callable 判定：显式注解 Callable → 无括号；无注解/其他类型 → 常规。
    let callable = param_ty.as_deref() == Some("Callable");

    // 本脚本方法全集（顶层 + 内部类，同 P5 层①口径）。
    let mut names: Vec<(String, usize)> = Vec::new();
    fn collect(stmts: &[Stmt], out: &mut Vec<(String, usize)>) {
        for st in stmts {
            match st {
                Stmt::Func(f) => out.push((f.name.name.clone(), f.params.len())),
                Stmt::Class(c) => collect(&c.body, out),
                _ => {}
            }
        }
    }
    collect(&file.body, &mut names);
    if names.is_empty() {
        return None;
    }
    let mut out: Vec<CompletionItem> = Vec::new();
    for (name, nparams) in names {
        let item = if callable {
            CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some("Callable 实参（无括号插入）".into()),
                insert_text: Some(name.clone()),
                ..CompletionItem::default()
            }
        } else if nparams == 0 {
            // 常规位零参：引擎 display/insert 都是 `name()`（oracle
            // local_method_variant_arg_0 include 精确锁定）。
            CompletionItem {
                label: format!("{name}()"),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(format!("{name}()")),
                insert_text: Some(format!("{name}()")),
                ..CompletionItem::default()
            }
        } else {
            CompletionItem {
                label: format!("{name}(…)"),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(name.clone()),
                insert_text: Some(format!("{name}($1)")),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..CompletionItem::default()
            }
        };
        out.push(item);
    }
    Some(out)
}

/// 本文件方法查找（顶层 + 内部类，P6 N5 用）。
fn find_local_func<'a>(file: &'a File, name: &str) -> Option<&'a FuncDecl> {
    fn walk<'a>(stmts: &'a [Stmt], name: &str) -> Option<&'a FuncDecl> {
        for st in stmts {
            match st {
                Stmt::Func(f) if f.name.name == name => return Some(f),
                Stmt::Class(c) => {
                    if let Some(f) = walk(&c.body, name) {
                        return Some(f);
                    }
                }
                _ => {}
            }
        }
        None
    }
    walk(&file.body, name)
}

#[cfg(test)]
mod tests {

    #[test]
    fn n5_callable_arg_bare_name() {
        let mut st = crate::hover_tests::test_state();
        let src = "func test(a: Callable, b):
    pass

func _init() -> void:
    test(h➡)
    pass

func hello_world():
	pass
".replace("➡", "");
        let uri: lsp_types::Uri = "file:///n5.gd".parse().unwrap();
        st.docs.insert(uri.clone(), src.clone());
        st.asts.insert(uri.clone(), crate::parse(&src).0);
        let at = src.find("test(h").unwrap() + "test(h".len();
        let items = callable_arg_items(&st, &uri, &src, at).expect("命中");
        let hw = items.iter().find(|i| i.label == "hello_world").expect("裸名项");
        assert_eq!(hw.insert_text.as_deref(), Some("hello_world"));
        // exclude：无带括号变体
        assert!(!items.iter().any(|i| i.insert_text.as_deref() == Some("hello_world(")));
    }

    #[test]
    fn n5_non_callable_arg_regular_form() {
        let mut st = crate::hover_tests::test_state();
        let src = "func test(a, b: Callable):
    pass

func _init() -> void:
    test(h➡)
    pass

func hello_world():
	pass
".replace("➡", "");
        let uri: lsp_types::Uri = "file:///n5b.gd".parse().unwrap();
        st.docs.insert(uri.clone(), src.clone());
        st.asts.insert(uri.clone(), crate::parse(&src).0);
        let at = src.find("test(h").unwrap() + "test(h".len();
        let items = callable_arg_items(&st, &uri, &src, at).expect("命中");
        let hw = items.iter().find(|i| i.label == "hello_world()").expect("常规项");
        assert_eq!(hw.insert_text.as_deref(), Some("hello_world()"));
        assert!(!items.iter().any(|i| i.label == "hello_world" && i.insert_text.as_deref() == Some("hello_world")));
    }

    #[test]
    fn n5_lambda_and_string_fallthrough() {
        let mut st = crate::hover_tests::test_state();
        // lambda 参数位 `func(` 不是方法调用 → None。
        let src = "func _init() -> void:
    x.connect(func(a = p➡): pass
    pass
".replace("➡", "");
        let uri: lsp_types::Uri = "file:///n5c.gd".parse().unwrap();
        st.asts.insert(uri.clone(), crate::parse(&src).0);
        let at = src.find("a = p").unwrap() + 5;
        assert!(callable_arg_items(&st, &uri, &src, at).is_none());
        // 字符串实参位 → None（C8 选项面优先）。
        let src2 = "func _init() -> void:
    x.is_action_pressed(\"a➡\")
    pass
".replace("➡", "");
        let uri2: lsp_types::Uri = "file:///n5d.gd".parse().unwrap();
        st.asts.insert(uri2.clone(), crate::parse(&src2).0);
        let at2 = src2.find("\"a").unwrap() + 2;
        assert!(callable_arg_items(&st, &uri2, &src2, at2).is_none());
    }

    use super::*;
    use crate::hover_tests::test_state;

    fn site_of(src: &str, needle: &str) -> Option<ArgSite> {
        let at = src.find(needle).unwrap() + needle.len();
        arg_site_at(src, at)
    }

    fn at_of(src: &str, needle: &str) -> usize {
        src.find(needle).unwrap() + needle.len()
    }

    /// (line, character) → 字节偏移（测试反查 textEdit 跨度用；仅 \n 行源；
    /// character 为 UTF-16 列，与 LSP Range 语义一致）。
    fn line_char_to_byte(src: &str, line: u32, character: u32) -> usize {
        let mut off = 0usize;
        for (i, l) in src.split('\n').enumerate() {
            if i as u32 == line {
                let mut col = 0u32;
                for (bi, c) in l.char_indices() {
                    if col >= character {
                        return off + bi;
                    }
                    col += c.len_utf16() as u32;
                }
                return off + l.len();
            }
            off += l.len() + 1;
        }
        off
    }

    #[test]
    fn detects_bare_call_and_argidx() {
        let src = "func f():\n\tconnect(➡)\n";
        let site = site_of(src, "➡").unwrap();
        match &site.callee {
            Callee::Call { method, receiver } => {
                assert_eq!(method, "connect");
                assert!(receiver.is_none());
            }
            _ => panic!("应为调用站点"),
        }
        assert_eq!(site.argidx, 0);
    }

    #[test]
    fn counts_argidx_across_nested_and_brackets() {
        // 嵌套调用的逗号归属内层；数组/字典字面量内的逗号不计数。
        let src = "func f():\n\tfoo(a, [1, 2], {\"x\": 1, \"y\": 2}, bar(b, ➡))\n";
        let site = site_of(src, "➡").unwrap();
        match &site.callee {
            Callee::Call { method, .. } => assert_eq!(method, "bar"),
            _ => panic!("应为 bar 调用"),
        }
        assert_eq!(site.argidx, 1);

        let src2 = "func f():\n\tfoo(a, b, ➡)\n";
        assert_eq!(site_of(src2, "➡").unwrap().argidx, 2);
    }

    #[test]
    fn detects_receiver_and_annotation() {
        let src = "func f():\n\tevent.is_action_pressed(➡)\n";
        match &site_of(src, "➡").unwrap().callee {
            Callee::Call { method, receiver } => {
                assert_eq!(method, "is_action_pressed");
                assert_eq!(receiver.as_deref(), Some("event"));
            }
            _ => panic!("应为带接收者的调用"),
        }
        let src2 = "@export_range(1, 2, ➡)\nvar x: int\n";
        match &site_of(src2, "➡").unwrap().callee {
            Callee::Annotation(n) => assert_eq!(n, "@export_range"),
            _ => panic!("应为注解站点"),
        }
        assert_eq!(site_of(src2, "➡").unwrap().argidx, 2);
    }

    #[test]
    fn string_in_argument_is_literal_site() {
        // 闭合串：整串跨度含 & 前缀；insertText 裸引号（引擎默认设置侧）。
        let src = "func f():\n\tevent.is_action_pressed(&\"te➡\")\n";
        let site = site_of(src, "➡").unwrap();
        let lit = site.literal.expect("应识别既有字面量");
        assert_eq!(lit.prefix, Some('&'));
        assert!(lit.closed);
        assert_eq!(&src[lit.full_start..lit.full_end], "&\"te➡\"");
    }

    #[test]
    fn no_site_outside_calls() {
        assert!(site_of("func f():\n\tva➡\n", "➡").is_none());
        assert!(site_of("func f():\n\tx = [1, ➡]\n", "➡").is_none());
    }

    #[test]
    fn signal_face_own_and_chain() {
        let state = test_state();
        let src = "extends Node\n\nsignal signal_a()\n\nfunc _ready():\n\tconnect(➡)\n";
        let at = at_of(src, "➡");
        let file = gdscript_syntax::parse(src).0;
        let items = argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).expect("信号面应命中");
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert!(labels.contains(&"\"signal_a\""), "自声明信号: {labels:?}");
        assert!(labels.contains(&"\"child_entered_tree\""), "Node 链信号: {labels:?}");
        // 引擎语义：选项面独占，不混入全局面。
        assert!(
            labels.iter().all(|l| l.starts_with('"')),
            "不应混入类名面: {labels:?}"
        );
    }

    #[test]
    fn input_action_face_from_project_input_section() {
        let state = test_state();
        let json = include_str!("../../gdscript-engine/testdata/extension_api.json");
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/projface");
        let project = gdscript_project::Project::open(&root, Some(json))
            .ok()
            .map(std::sync::Arc::new);
        let src = "func _input(event: InputEvent) -> void:\n\tevent.is_action_pressed(➡)\n";
        let at = at_of(src, "➡");
        let file = gdscript_syntax::parse(src).0;
        let mut s = state;
        s.project = project;
        let items = argument_options(&s, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).expect("输入动画面应命中");
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert_eq!(labels, vec!["\"test_input_action\""]);
    }

    #[test]
    fn signal_face_falls_through_when_receiver_unknown() {
        // `test_signal` 是信号名不是对象 → 面无（引擎 r_forced=false 回落）。
        let state = test_state();
        let src = "signal test_signal\n\nfunc _init():\n\ttest_signal.connect(h➡)\n";
        let at = at_of(src, "➡");
        let file = gdscript_syntax::parse(src).0;
        assert!(argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).is_none());
    }

    #[test]
    fn annotation_tables() {
        let state = test_state();
        let file = gdscript_syntax::parse("var x: int\n").0;

        let src = "@export_range(1, 2, 3, ➡)\nvar x: int\n";
        let at = at_of(src, "➡");
        let items = argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).unwrap();
        let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
        assert_eq!(
            labels,
            vec!["\"or_greater\"", "\"or_less\"", "\"prefer_slider\"", "\"hide_control\""]
        );

        // @export_range 数值参位：引擎 forced 空面。
        let src = "@export_range(➡)\nvar x: int\n";
        let at = at_of(src, "➡");
        assert!(argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).unwrap().is_empty());

        let src = "@rpc(➡)\nfunc f():\n\tpass\n";
        let at = at_of(src, "➡");
        let items = argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).unwrap();
        assert_eq!(items.len(), 7);
        assert!(items.iter().any(|i| i.label == "\"any_peer\""));

        let src = "@warning_ignore(➡)\nfunc f():\n\tpass\n";
        let at = at_of(src, "➡");
        let items = argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).unwrap();
        assert!(items.iter().any(|i| i.label == "\"unused_variable\""));
        assert!(
            !items.iter().any(|i| i.label.contains("property_used_as_function")),
            "弃用项剔除"
        );

        // @export_custom 走模型全局枚举（裸名，无引号）。
        let src = "@export_custom(➡)\nvar x: int\n";
        let at = at_of(src, "➡");
        let items = argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).unwrap();
        assert!(items.iter().any(|i| i.label == "PROPERTY_HINT_RANGE"));
        assert!(items.iter().all(|i| !i.label.starts_with('"')), "@export_custom 为裸名面");

        // @export_node_path：Node + 引擎继承树 + 脚本类，引号面。
        let src = "@export_node_path(➡)\nvar x: NodePath\n";
        let at = at_of(src, "➡");
        let items = argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).unwrap();
        assert!(items.iter().any(|i| i.label == "\"Node\""));
        assert!(items.iter().any(|i| i.label == "\"Sprite2D\""));
    }

    #[test]
    fn literal_replacement_edit_spans_whole_string() {
        let state = test_state();
        let src = "extends Node\n\nsignal s()\n\nfunc _ready():\n\tconnect(\"➡\")\n";
        let at = at_of(src, "➡");
        let file = gdscript_syntax::parse(src).0;
        let items = argument_options(&state, &"file:///t.gd".parse::<lsp_types::Uri>().unwrap(), src, at, at, Some(&file)).unwrap();
        let item = items.iter().find(|i| i.label == "\"s\"").expect("自信号");
        let Some(CompletionTextEdit::Edit(edit)) = &item.text_edit else {
            panic!("既有字面量应走 textEdit 整串替换");
        };
        let start = line_char_to_byte(src, edit.range.start.line, edit.range.start.character);
        let end = line_char_to_byte(src, edit.range.end.line, edit.range.end.character);
        assert_eq!(&src[start..end], "\"➡\"");
        assert_eq!(edit.new_text, "\"s\"");
        // insertText 仍是裸引号串（引擎 insert_text 语义）。
        assert_eq!(item.insert_text.as_deref(), Some("\"s\""));
    }

    // ───────────────────── P8 C8 二期：设置 plumbing ─────────────────────

    fn signal_setup() -> (State, String, lsp_types::Uri, usize) {
        let mut st = crate::hover_tests::test_state();
        let src = "extends Node

signal s

func _init() -> void:
    self.connect(➡)
    pass
".replace("➡", "");
        let uri: lsp_types::Uri = "file:///p8a.gd".parse().unwrap();
        st.docs.insert(uri.clone(), src.clone());
        st.asts.insert(uri.clone(), crate::parse(&src).0);
        let at = src.find("self.connect(").unwrap() + "self.connect(".len();
        (st, src, uri, at)
    }

    #[test]
    fn p8_add_string_name_prefix() {
        let (mut st, src, uri, at) = signal_setup();
        st.prefs.add_string_name_literals = true;
        let items = argument_options(&st, &uri, &src, at, at, st.asts.get(&uri))
            .expect("信号面命中");
        let s = items.iter().find(|i| i.label == "&\"s\"").expect("& 前缀项");
        assert_eq!(s.insert_text.as_deref(), Some("&\"s\""));
        // exclude：裸版不得同时出现（独占面）。
        assert!(!items.iter().any(|i| i.label == "\"s\""));
    }

    #[test]
    fn p8_dont_add_bare_default() {
        let (st, src, uri, at) = signal_setup();
        let items = argument_options(&st, &uri, &src, at, at, st.asts.get(&uri))
            .expect("信号面命中");
        assert!(items.iter().any(|i| i.label == "\"s\""), "默认设置出裸版");
        assert!(!items.iter().any(|i| i.label == "&\"s\""), "默认设置无 & 前缀");
    }

    #[test]
    fn p8_use_single_quotes() {
        let (mut st, src, uri, at) = signal_setup();
        st.prefs.use_single_quotes = true;
        let items = argument_options(&st, &uri, &src, at, at, st.asts.get(&uri))
            .expect("信号面命中");
        let s = items.iter().find(|i| i.label == "'s'").expect("单引号项");
        assert_eq!(s.insert_text.as_deref(), Some("'s'"));
    }

    #[test]
    fn p8_closed_literal_replacement_keeps_prefix() {
        let mut st = crate::hover_tests::test_state();
        st.prefs.add_string_name_literals = true;
        // 既有裸字面量（无 & 前缀）→ textEdit 替换整串为 &"s"（引擎 final_code），
        // insertText 保持裸引号串（should && has → 不加前缀）。
        let src = "extends Node

signal s

func _init() -> void:
    self.connect(\"➡\")
    pass
".replace("➡", "");
        let uri: lsp_types::Uri = "file:///p8b.gd".parse().unwrap();
        st.docs.insert(uri.clone(), src.clone());
        st.asts.insert(uri.clone(), crate::parse(&src).0);
        let at = src.find("self.connect(\"").unwrap() + "self.connect(\"".len();
        let items = argument_options(&st, &uri, &src, at, at, st.asts.get(&uri))
            .expect("信号面命中");
        let s = items.iter().find(|i| i.label == "&\"s\"").expect("替换文本带前缀");
        assert_eq!(s.insert_text.as_deref(), Some("\"s\""), "insertText 不加前缀（has 既有字面量）");
    }

    #[test]
    fn p8_first_arg_text_cases() {
        let src = "tween_property(a, )";
        assert_eq!(first_arg_text(src, src.find('(').unwrap()), Some("a"));
        // 嵌套括号内逗号不切分。
        let src2 = "tween_property(get_node(\"X\")[0], )";
        assert_eq!(first_arg_text(src2, src2.find('(').unwrap()), Some("get_node(\"X\")[0]"));
        // 字符串内逗号不切分。
        let src3 = "tween_property(m.get(\"a,b\"), )";
        assert_eq!(first_arg_text(src3, src3.find('(').unwrap()), Some("m.get(\"a,b\")"));
        // 空实参 → None。
        let src4 = "tween_property(, )";
        assert_eq!(first_arg_text(src4, src4.find('(').unwrap()), None);
    }
}
