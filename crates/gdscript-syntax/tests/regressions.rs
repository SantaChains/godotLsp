//! 语义锁定回归测试：两处经 Godot 源码核验的行为。
//!
//! 1. `\` 行续接的**传递性**（GH-89403）：续接后连续的空行/整行注释行被一并
//!    吞掉，直到真正的代码行为止；这些物理行全部不参与缩进结构注入。
//!    核验来源：godot `gdscript_tokenizer.cpp` `scan()`（`line_continuation` +
//!    `_skip_whitespace()`）与 `check_indent()`（空行/注释行 continue）。
//! 2. `match` 分支循环吞掉裸 `pass`（不创建分支）。
//!    核验来源：godot `gdscript_parser.cpp` `parse_match()` 的
//!    `if (match(PASS)) { consume(NEWLINE); continue; }`。

use gdscript_syntax::ast::{Block, Stmt};
use gdscript_syntax::{parse, tokenize, TokenKind};

/// GH-89403 语料原文（corpus/godot-engine-tests/parser/features/continuation_lines_comments.bin.gd）。
const CONT_SRC: &str = "\
func test():
\tvar x := 1
\tif x == 0 \\
\t\t\t# Comment.
\t\t\t# Comment.
\t\t\tand (x < 1 or x > 2) \\
\t\t\t# Comment.
\t\t\tand x != 3:
\t\tpass
\tprint(\"Ok\")
";

fn func_body(src: &str) -> Block {
    let (file, errs) = parse(src);
    assert!(errs.is_empty(), "expected clean parse, got {errs:?}");
    match file.body.into_iter().next() {
        Some(Stmt::Func(f)) => f.body,
        other => panic!("expected func, got {other:?}"),
    }
}

#[test]
fn backslash_continuation_spans_comment_and_blank_lines() {
    // 解析级：注释行夹在续接中间，不得产生任何错误。
    let (_ast, errs) = parse(CONT_SRC);
    assert!(errs.is_empty(), "expected clean parse, got {errs:?}");

    // token 级：`if` 与闭合 `:` 之间不得出现任何结构 token ——
    // 续接区间内的 Newline/Indent/Dedent 注入全部被抑制。
    let toks = tokenize(CONT_SRC);
    let if_idx = toks
        .iter()
        .position(|t| t.kind == TokenKind::If)
        .expect("if token");
    let colon_idx = if_idx
        + toks[if_idx..]
            .iter()
            .position(|t| t.kind == TokenKind::Colon)
            .expect("closing colon");
    for t in &toks[if_idx..colon_idx] {
        assert!(
            !matches!(
                t.kind,
                TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
            ),
            "unexpected structural token {:?} inside continuation span",
            t.kind
        );
    }
}

#[test]
fn backslash_continuation_spans_blank_lines() {
    // `\` 之后空行同样传递（Godot check_indent 对空行 continue）。
    let src = "func f():\n\tvar a = 1 + \\\n\n\t\t\t2\n\tprint(a)\n";
    let (_ast, errs) = parse(src);
    assert!(errs.is_empty(), "expected clean parse, got {errs:?}");
    let toks = tokenize(src);
    let var_idx = toks
        .iter()
        .position(|t| t.kind == TokenKind::Var)
        .expect("var token");
    let int_idx = var_idx
        + toks[var_idx..]
            .iter()
            .position(|t| t.kind == TokenKind::Int)
            .expect("int token");
    // `1` 与续接来的 `2` 之间不得有结构 token（同一逻辑行）。
    for t in &toks[var_idx..int_idx] {
        assert!(
            !matches!(
                t.kind,
                TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent
            ),
            "unexpected structural token {:?} inside blank-line continuation span",
            t.kind
        );
    }
}

#[test]
fn backslash_not_at_line_end_stays_error_token() {
    // 反向锁定：行中间的 `\` 不是续接符，须落为 Error token（绝不静默吞掉）。
    let toks = tokenize("var x = 1 \\ 2\n");
    assert!(
        toks.iter().any(|t| t.kind == TokenKind::Error),
        "mid-line backslash should produce an Error token, got {toks:?}"
    );
}

#[test]
fn match_accepts_bare_pass_body() {
    // `match 0:` + `pass` 体（match.gd 语料尾部）：pass 行被吞掉，arms 为空。
    let src = "func test():\n\tmatch 0:\n\t\tpass\n";
    let body = func_body(src);
    match body.stmts.first() {
        Some(Stmt::Match { arms, .. }) => {
            assert!(arms.is_empty(), "bare pass should not create an arm");
        }
        other => panic!("expected match, got {other:?}"),
    }
}

#[test]
fn match_pass_branch_mixed_with_real_arm() {
    // pass 行与真实臂混列：pass 被跳过，真实臂照常解析。
    let src = "func test():\n\tmatch 0:\n\t\tpass\n\t\t_:\n\t\t\tprint(1)\n";
    let body = func_body(src);
    match body.stmts.first() {
        Some(Stmt::Match { arms, .. }) => {
            assert_eq!(arms.len(), 1, "only the wildcard arm should be parsed");
        }
        other => panic!("expected match, got {other:?}"),
    }
}
