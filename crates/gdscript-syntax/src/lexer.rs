//! GDScript 2.0 词法分析（两阶段）。
//!
//! 阶段一：把源码切成原始 token（跳过空白与 `#` 注释，字符串/数字/标识符/运算符/标点）。
//! 阶段二：依据缩进注入 `Newline` / `Indent` / `Dedent`，使解析器可据缩进分块。
//!
//! 设计立场（见 `docs/from-scratch-design.md` §4）：R1 先把空白/注释作为 trivia **跳过**，
//! 换取一个简单稳健的 token 流；无损 trivia 一等 token 留待后续升级。

use crate::token::{Token, TokenKind, keyword_kind};
use std::collections::HashSet;

struct Lx<'a> {
    chars: &'a [(usize, char)],
    src: &'a str,
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Lx<'a> {
    fn peek(&self, off: usize) -> Option<char> {
        self.chars.get(self.pos + off).map(|&(_, c)| c)
    }
    fn cur_off(&self) -> usize {
        self.chars.get(self.pos).map(|&(o, _)| o).unwrap_or(self.src.len())
    }
    fn bump(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).map(|&(_, c)| c)?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(c)
    }
}

/// 对外入口：返回含结构 token 的完整流（末尾为 `Eof`）。
pub fn tokenize(source: &str) -> Vec<Token> {
    let chars: Vec<(usize, char)> = source.char_indices().collect();
    let (raw, cont) = lex_raw(&chars, source);
    inject_structure(raw, source, &cont)
}

fn lex_raw(chars: &[(usize, char)], src: &str) -> (Vec<Token>, HashSet<u32>) {
    let mut lx = Lx { chars, src, pos: 0, line: 0, col: 0 };
    let mut out = Vec::new();
    // 记录「行续接」目标行号：这些行不触发 Indent/Dedent/Newline 注入。
    let mut cont: HashSet<u32> = HashSet::new();
    while lx.pos < lx.chars.len() {
        let c = lx.peek(0).unwrap();
        let start = lx.cur_off();
        let line = lx.line;
        let col = lx.col;
        match c {
            ' ' | '\t' | '\r' => {
                lx.bump();
            }
            '\n' => {
                lx.bump();
            }
            '#' => {
                // 行注释，跳过到行尾（不含换行）。
                while let Some(ch) = lx.peek(0) {
                    if ch == '\n' {
                        break;
                    }
                    lx.bump();
                }
            }
            '\\' if lx.peek(1) == Some('\n') || (lx.peek(1) == Some('\r') && lx.peek(2) == Some('\n')) => {
                // 行续接符 `\` + 换行：逻辑行延续，不增行号。
                lx.bump(); // '\'
                if lx.peek(0) == Some('\r') {
                    lx.bump();
                }
                lx.bump(); // '\n'
                // 续接具有传递性（核验自 Godot `gdscript_tokenizer.cpp`：
                // `scan()` 置 `line_continuation` 后调 `_skip_whitespace()`，
                // `check_indent()` 连续吞掉空行与整行注释行，见 GH-89403）。
                // 续接后的物理行号标记为续接行，使 inject_structure 不在此
                // 注入缩进结构；注释/空行本身无 token，标记仅作传递载体。
                cont.insert(lx.line);
                loop {
                    while matches!(lx.peek(0), Some(' ') | Some('\t') | Some('\r')) {
                        lx.bump();
                    }
                    match lx.peek(0) {
                        // 整行注释：吃到行尾并跨过换行，续接传递到下一行。
                        Some('#') => {
                            while !matches!(lx.peek(0), Some('\n') | None) {
                                lx.bump();
                            }
                            if lx.peek(0) == Some('\n') {
                                lx.bump();
                                cont.insert(lx.line);
                            }
                        }
                        // 空行：同样传递。
                        Some('\n') => {
                            lx.bump();
                            cont.insert(lx.line);
                        }
                        // 真正的代码字符：续接到此为止，交回主循环切分。
                        _ => break,
                    }
                }
            }
            '"' | '\'' => {
                let (tok, end) = lex_string(&mut lx, c);
                out.push(Token::new(tok, start, end, line, col));
            }
            '0'..='9' => {
                let (tok, end) = lex_number(&mut lx);
                out.push(Token::new(tok, start, end, line, col));
            }
            // 前导点浮点：`.123` / `.123e4`（点后紧跟数字才进入数字扫描）。
            '.' if lx.peek(1).map(|c| c.is_ascii_digit()).unwrap_or(false) => {
                let (tok, end) = lex_number(&mut lx);
                out.push(Token::new(tok, start, end, line, col));
            }
            // 原始字符串：`r"..."` / `r'...'`（含三引号），内容不处理转义。
            'r' if lx.peek(1) == Some('"') || lx.peek(1) == Some('\'') => {
                let quote = lx.peek(1).unwrap();
                let (tok, end) = lex_raw_string(&mut lx, quote);
                out.push(Token::new(tok, start, end, line, col));
            }
            ch if ch.is_alphabetic() || ch == '_' => {
                // 标识符 / 关键字
                while let Some(ch2) = lx.peek(0) {
                    // 续接：ASCII 字母数字 / `_`，或任意非 ASCII 非空白字符
                    // （覆盖 Unicode 组合记号，如孟加拉/马拉雅拉姆/泰文元音符号）。
                    if ch2.is_alphanumeric()
                        || ch2 == '_'
                        || ((ch2 as u32) >= 0x80 && !ch2.is_whitespace())
                    {
                        lx.bump();
                    } else {
                        break;
                    }
                }
                let end = lx.cur_off();
                let text = &src[start..end];
                let kind = keyword_kind(text).unwrap_or(TokenKind::Ident);
                out.push(Token::new(kind, start, end, line, col));
            }
            _ => {
                if let Some((kind, consumed)) = lex_punct(&mut lx) {
                    let end = lx.cur_off();
                    // consumed 可能 >1，但 cur_off 已随 bump 推进。
                    let _ = consumed;
                    out.push(Token::new(kind, start, end, line, col));
                } else {
                    // 无法识别的字符：记为 Error token，绝不 panic。
                    lx.bump();
                    let end = lx.cur_off();
                    out.push(Token::new(TokenKind::Error, start, end, line, col));
                }
            }
        }
    }
    (out, cont)
}

fn lex_string(lx: &mut Lx, quote: char) -> (TokenKind, usize) {
    let triple = lx.peek(1) == Some(quote) && lx.peek(2) == Some(quote);
    // 消费起始引号
    lx.bump();
    if triple {
        lx.bump();
        lx.bump();
    }
    let mut content = String::new();
    loop {
        match lx.peek(0) {
            None => break, // 未闭合：留待解析器报错
            Some(c) if c == quote => {
                if triple {
                    if lx.peek(1) == Some(quote) && lx.peek(2) == Some(quote) {
                        lx.bump();
                        lx.bump();
                        lx.bump();
                        break;
                    } else {
                        content.push(c);
                        lx.bump();
                    }
                } else {
                    lx.bump();
                    break;
                }
            }
            Some('\\') => {
                lx.bump();
                if let Some(e) = lx.peek(0) {
                    match e {
                        'n' => content.push('\n'),
                        't' => content.push('\t'),
                        'r' => content.push('\r'),
                        '0' => content.push('\0'),
                        '"' => content.push('"'),
                        '\'' => content.push('\''),
                        '\\' => content.push('\\'),
                        other => content.push(other),
                    }
                    lx.bump();
                }
            }
            Some(other) => {
                content.push(other);
                lx.bump();
            }
        }
    }
    (TokenKind::String, lx.cur_off())
}

/// 原始字符串 `r"..."` / `r'...'`（含三引号）：内容逐字保留，不处理 `\` 转义。
fn lex_raw_string(lx: &mut Lx, quote: char) -> (TokenKind, usize) {
    lx.bump(); // 'r'
    // 此刻 peek(0) 是第一个引号，与 lex_string 一致地检测三引号（需连续三个引号）。
    let triple = lx.peek(0) == Some(quote)
        && lx.peek(1) == Some(quote)
        && lx.peek(2) == Some(quote);
    lx.bump(); // 第一个引号
    if triple {
        lx.bump();
        lx.bump();
    }
    loop {
        match lx.peek(0) {
            None => break, // 未闭合：留待解析器报错
            Some('\\') => {
                // 反斜杠与其后一字符均原样入内容（`\"` 不终止字符串）。
                lx.bump();
                if lx.peek(0).is_some() {
                    lx.bump();
                }
            }
            Some(c) if c == quote => {
                if triple {
                    if lx.peek(1) == Some(quote) && lx.peek(2) == Some(quote) {
                        lx.bump();
                        lx.bump();
                        lx.bump();
                        break;
                    } else {
                        lx.bump(); // 单个引号作为内容
                    }
                } else {
                    lx.bump();
                    break;
                }
            }
            Some(_) => {
                lx.bump(); // 原样字符（不处理转义）
            }
        }
    }
    (TokenKind::String, lx.cur_off())
}

fn lex_number(lx: &mut Lx) -> (TokenKind, usize) {
    let mut is_float = false;
    // 前导点浮点：`.123` / `.123e4`。
    let leading_dot = lx.peek(0) == Some('.');
    if leading_dot {
        is_float = true;
        lx.bump(); // '.'
    }

    // 十六进制 / 二进制 / 八进制（仅当前导无 `.`）。
    let c0 = lx.peek(0).unwrap_or('\0');
    if !leading_dot && c0 == '0' {
        let base = match lx.peek(1) {
            Some('x') | Some('X') => Some(16),
            Some('b') | Some('B') => Some(2),
            Some('o') | Some('O') => Some(8),
            _ => None,
        };
        if let Some(_radix) = base {
            lx.bump();
            lx.bump();
            while let Some(d) = lx.peek(0) {
                if d.is_digit(_radix) || d == '_' {
                    lx.bump();
                } else {
                    break;
                }
            }
            return (TokenKind::Int, lx.cur_off());
        }
    }
    // 十进制整数部分（前导点形式无此部分）。
    if !leading_dot {
        while let Some(d) = lx.peek(0) {
            if d.is_ascii_digit() || d == '_' {
                lx.bump();
            } else {
                break;
            }
        }
    }
    // 小数部分。
    if leading_dot {
        // 前导点后的小数位。
        while let Some(d) = lx.peek(0) {
            if d.is_ascii_digit() || d == '_' {
                lx.bump();
            } else {
                break;
            }
        }
    } else if lx.peek(0) == Some('.') {
        // 尾随点：`123.`、`123.456`、`123.e4`。
        // 仅当点后是数字 / 指数符 / 定界符（非标识字符、非另一 `.`）时才吞为小数点，
        // 以区别于成员访问 `123.foo`（不合法但稳妥）与 `..`。
        let take_dot = match lx.peek(1) {
            Some(d) if d.is_ascii_digit() => true,
            Some('e') | Some('E') => true,
            Some(d) if d.is_alphanumeric() || d == '_' || d == '.' => false,
            Some(_) => true,
            None => true,
        };
        if take_dot {
            is_float = true;
            lx.bump(); // '.'
            while let Some(d) = lx.peek(0) {
                if d.is_ascii_digit() || d == '_' {
                    lx.bump();
                } else {
                    break;
                }
            }
        }
    }
    // 指数部分：`e`/`E` + 可选符号 + 数字。
    if lx.peek(0) == Some('e') || lx.peek(0) == Some('E') {
        is_float = true;
        lx.bump();
        if lx.peek(0) == Some('+') || lx.peek(0) == Some('-') {
            lx.bump();
        }
        while let Some(d) = lx.peek(0) {
            if d.is_ascii_digit() || d == '_' {
                lx.bump();
            } else {
                break;
            }
        }
    }
    (
        if is_float { TokenKind::Float } else { TokenKind::Int },
        lx.cur_off(),
    )
}

/// 多字符优先的运算符/标点识别。
fn lex_punct(lx: &mut Lx) -> Option<(TokenKind, usize)> {
    let a = lx.peek(0)?;
    let b = lx.peek(1);
    let c = lx.peek(2);
    // 三字符
    if let (Some(b), Some(c)) = (b, c) {
        let t = match (a, b, c) {
            ('<', '<', '=') => Some(TokenKind::ShlEq),
            ('>', '>', '=') => Some(TokenKind::ShrEq),
            _ => None,
        };
        if let Some(k) = t {
            lx.bump();
            lx.bump();
            lx.bump();
            return Some((k, 3));
        }
    }
    // 两字符
    if let Some(b) = b {
        let t = match (a, b) {
            ('-', '>') => Some(TokenKind::Arrow),
            ('=', '>') => Some(TokenKind::FatArrow),
            (':', '=') => Some(TokenKind::ColonEq),
            ('+', '=') => Some(TokenKind::PlusEq),
            ('-', '=') => Some(TokenKind::MinusEq),
            ('*', '=') => Some(TokenKind::StarEq),
            ('/', '=') => Some(TokenKind::SlashEq),
            ('%', '=') => Some(TokenKind::PercentEq),
            ('&', '=') => Some(TokenKind::AmpEq),
            ('|', '=') => Some(TokenKind::PipeEq),
            ('^', '=') => Some(TokenKind::CaretEq),
            ('=', '=') => Some(TokenKind::EqEq),
            ('!', '=') => Some(TokenKind::NotEq),
            ('<', '=') => Some(TokenKind::LtEq),
            ('>', '=') => Some(TokenKind::GtEq),
            ('&', '&') => Some(TokenKind::AndAnd),
            ('|', '|') => Some(TokenKind::PipePipe),
            ('<', '<') => Some(TokenKind::Shl),
            ('>', '>') => Some(TokenKind::Shr),
            _ => None,
        };
        if let Some(k) = t {
            lx.bump();
            lx.bump();
            return Some((k, 2));
        }
    }
    // 单字符
    let t = match a {
        '+' => Some(TokenKind::Plus),
        '-' => Some(TokenKind::Minus),
        '*' => Some(TokenKind::Star),
        '/' => Some(TokenKind::Slash),
        '%' => Some(TokenKind::PercentOp),
        '<' => Some(TokenKind::Lt),
        '>' => Some(TokenKind::Gt),
        '!' => Some(TokenKind::Bang),
        '&' => Some(TokenKind::Amp),
        '|' => Some(TokenKind::Pipe),
        '^' => Some(TokenKind::Caret),
        '~' => Some(TokenKind::Tilde),
        '=' => Some(TokenKind::Assign),
        '.' => Some(TokenKind::Dot),
        ':' => Some(TokenKind::Colon),
        ',' => Some(TokenKind::Comma),
        ';' => Some(TokenKind::Semicolon),
        '(' => Some(TokenKind::LParen),
        ')' => Some(TokenKind::RParen),
        '[' => Some(TokenKind::LBracket),
        ']' => Some(TokenKind::RBracket),
        '{' => Some(TokenKind::LBrace),
        '}' => Some(TokenKind::RBrace),
        '@' => Some(TokenKind::At),
        '$' => Some(TokenKind::Dollar),
        _ => None,
    };
    if let Some(k) = t {
        lx.bump();
        Some((k, 1))
    } else {
        None
    }
}

/// 阶段二：依据缩进注入 Newline/Indent/Dedent。
/// `cont` 为行续接（`\` + 换行）目标行号集合：这些行不触发结构注入。
fn inject_structure(raw: Vec<Token>, src: &str, cont: &HashSet<u32>) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::with_capacity(raw.len() + 8);
    let mut indent_stack: Vec<u32> = vec![0];
    // 括号内多行 lambda:depth>0 本不注入结构 token,但 lambda 体是真正的
    // 缩进块(对齐 Godot gdscript_tokenizer 对表达式内 lambda 的缩进处理)。
    // 以 lambda 头行为基准维护独立缩进栈;头行基准深度以下的嵌套括号续行
    // (depth > base_depth)不注入——那是括号续行,不是 lambda 体行。
    let mut lambda: Option<(i32, u32, Vec<u32>)> = None; // (基准 depth, 头行行首缩进, 体缩进栈)
    let mut prev_line: Option<u32> = None;
    let mut had_prev_token = false;
    let mut depth: i32 = 0;
    // 当前行首 token 列号(mid-line lambda 的头缩进判定用:lambda 头可以在
    // 链式调用 `.filter(func(...):` 中间,Func token 的 col 不是行缩进)。
    let mut cur_line_start_col: u32 = 0;
    let mut seen_line: Option<u32> = None;

    for tok in raw {
        if seen_line != Some(tok.line) {
            seen_line = Some(tok.line);
            cur_line_start_col = tok.col;
        }
        if let Some(pl) = prev_line
            && tok.line != pl
            && !cont.contains(&tok.line)
        {
            let had = had_prev_token;
            if had && depth == 0 {
                out.push(Token::new(TokenKind::Newline, tok.start, tok.start, pl, 0));
                let ind = tok.indent();
                loop {
                    let top = *indent_stack.last().unwrap();
                    if ind > top {
                        indent_stack.push(ind);
                        out.push(Token::new(TokenKind::Indent, tok.start, tok.start, tok.line, 0));
                        break;
                    } else if ind < top {
                        indent_stack.pop();
                        out.push(Token::new(TokenKind::Dedent, tok.start, tok.start, tok.line, 0));
                    } else {
                        break;
                    }
                }
            } else if had && depth > 0 {
                if let Some((base, header_ind, stack)) = lambda.as_mut() {
                    if depth == *base {
                        // lambda 体块行:按体缩进栈注入。
                        // 闭括号行(RParen 等)无条件终结体——闭括号的缩进
                        // 在真实工程里可以是任意值(常与体对齐)。
                        let closes = matches!(
                            tok.kind,
                            TokenKind::RParen
                                | TokenKind::RBracket
                                | TokenKind::RBrace
                        );
                        out.push(Token::new(TokenKind::Newline, tok.start, tok.start, pl, 0));
                        if closes {
                            for _ in stack.drain(..) {
                                out.push(Token::new(TokenKind::Dedent, tok.start, tok.start, tok.line, 0));
                            }
                            lambda = None;
                        } else {
                            let ind = tok.indent();
                            if stack.is_empty() && ind <= *header_ind {
                                // 体已结束(缩进回到头行及以下)。
                                lambda = None;
                            } else {
                                loop {
                                    let Some(&top) = stack.last() else {
                                        stack.push(ind);
                                        out.push(Token::new(TokenKind::Indent, tok.start, tok.start, tok.line, 0));
                                        break;
                                    };
                                    if ind > top {
                                        stack.push(ind);
                                        out.push(Token::new(TokenKind::Indent, tok.start, tok.start, tok.line, 0));
                                        break;
                                    } else if ind < top {
                                        stack.pop();
                                        out.push(Token::new(TokenKind::Dedent, tok.start, tok.start, tok.line, 0));
                                        if stack.is_empty() {
                                            lambda = None;
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    // depth > base:lambda 体内嵌套括号的续行,不注入。
                }
            }
        }
        prev_line = Some(tok.line);
        match tok.kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                depth -= 1;
                if depth <= 0 {
                    lambda = None;
                } else if let Some((base, _, _)) = lambda
                    && depth < base
                {
                    // 闭括号使深度降到 lambda 基准之下:体不可能再继续
                    //(`}}},` 同行收束多层 dict 的真实形态),立即终结。
                    lambda = None;
                }
            }
            TokenKind::Func if depth > 0 => {
                if lambda.is_none() {
                    lambda = Some((depth, cur_line_start_col, Vec::new()));
                }
            }
            _ => {}
        }
        out.push(tok);
        had_prev_token = true;
    }
    while indent_stack.len() > 1 {
        indent_stack.pop();
        let last = out.last().map(|t| t.end).unwrap_or(src.len());
        out.push(Token::new(TokenKind::Dedent, last, last, 0, 0));
    }
    out.push(Token::new(TokenKind::Eof, src.len(), src.len(), 0, 0));
    out
}
