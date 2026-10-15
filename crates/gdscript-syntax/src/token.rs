//! 词法 token 定义与关键字表。

/// token 种类。结构类（Newline/Indent/Dedent）由词法第二阶段（缩进处理）注入。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    // 字面量 / 标识符
    Ident,
    Int,
    Float,
    String,
    // 关键字
    If, Elif, Else, For, While, Match,
    Break, Continue, Pass, Return,
    Func, Class, Extends, ClassName, Var, Const, Enum, Signal, Static,
    Await, Yield, Preload, Load,
    SelfKw, Super, Is, As, In, Not, And, Or, Where,
    Null, True, False, Void,
    // 标点
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Comma, Dot, Colon, Semicolon, At, Dollar, Percent,
    // 运算符
    Assign, PlusEq, MinusEq, StarEq, SlashEq, PercentEq, AmpEq, PipeEq, CaretEq,
    Plus, Minus, Star, Slash, PercentOp, Lt, Gt, Bang, Amp, Pipe, Caret, Tilde, Shl, Shr,
    EqEq, NotEq, LtEq, GtEq, AndAnd, PipePipe, Arrow, FatArrow, ColonEq, ShlEq, ShrEq,
    // 结构
    Newline, Indent, Dedent,
    // 特殊
    Error,
    Eof,
}

impl TokenKind {
    /// 是否是语句/声明的起始 token（用于错误恢复同步）。
    pub fn is_stmt_start(self) -> bool {
        matches!(
            self,
            TokenKind::If | TokenKind::Elif | TokenKind::Else | TokenKind::For
                | TokenKind::While | TokenKind::Match | TokenKind::Break
                | TokenKind::Continue | TokenKind::Pass | TokenKind::Return
                | TokenKind::Func | TokenKind::Class | TokenKind::Var
                | TokenKind::Const | TokenKind::Enum | TokenKind::Signal
                | TokenKind::At | TokenKind::Ident | TokenKind::SelfKw
                | TokenKind::Dollar
        )
    }

    /// 硬关键字：不能作普通标识符，但 `.` 之后可作成员名（`dict.if`、`obj.class_name`）。
    pub fn is_hard_keyword(self) -> bool {
        matches!(
            self,
            TokenKind::If | TokenKind::Elif | TokenKind::Else | TokenKind::For
                | TokenKind::While | TokenKind::Match | TokenKind::Break
                | TokenKind::Continue | TokenKind::Pass | TokenKind::Return
                | TokenKind::Func | TokenKind::Class | TokenKind::Extends
                | TokenKind::ClassName | TokenKind::Var | TokenKind::Const
                | TokenKind::Enum | TokenKind::Signal | TokenKind::Static
        )
    }
}

/// 关键字 → token 种类。类型名（int/String/Vector2…）不是保留字，不入表。
pub fn keyword_kind(s: &str) -> Option<TokenKind> {
    Some(match s {
        "if" => TokenKind::If,
        "elif" => TokenKind::Elif,
        "else" => TokenKind::Else,
        "for" => TokenKind::For,
        "while" => TokenKind::While,
        "match" => TokenKind::Match,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "pass" => TokenKind::Pass,
        "return" => TokenKind::Return,
        "func" => TokenKind::Func,
        "class" => TokenKind::Class,
        "extends" => TokenKind::Extends,
        "class_name" => TokenKind::ClassName,
        "var" => TokenKind::Var,
        "const" => TokenKind::Const,
        "enum" => TokenKind::Enum,
        "signal" => TokenKind::Signal,
        "static" => TokenKind::Static,
        "await" => TokenKind::Await,
        "yield" => TokenKind::Yield,
        "preload" => TokenKind::Preload,
        "load" => TokenKind::Load,
        "self" => TokenKind::SelfKw,
        "super" => TokenKind::Super,
        "is" => TokenKind::Is,
        "as" => TokenKind::As,
        "in" => TokenKind::In,
        "not" => TokenKind::Not,
        "and" => TokenKind::And,
        "or" => TokenKind::Or,
        // match 守卫关键字（GDScript 用 `when`；`where` 作为兼容别名保留）。
        "when" => TokenKind::Where,
        "where" => TokenKind::Where,
        "null" => TokenKind::Null,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "void" => TokenKind::Void,
        _ => return None,
    })
}

/// 软关键字：在标识符位置可被当作名字使用的保留字（GDScript 2.0 特性）。
/// 不含 `if`/`for`/`while`/`var`/`const`/`func`/`class` 等硬保留字。
pub fn is_soft_keyword(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::Match
            | TokenKind::Where // when/where 守卫
            | TokenKind::In
            | TokenKind::As
            | TokenKind::Is
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Not
            | TokenKind::Await
            | TokenKind::Yield
            | TokenKind::Preload
            | TokenKind::Load
            | TokenKind::SelfKw
            | TokenKind::Super
            | TokenKind::Void
            | TokenKind::Null
            | TokenKind::True
            | TokenKind::False
    )
}

/// 带位置信息的 token。文本由 `source[start..end]` 恢复。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub col: u32,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize, line: u32, col: u32) -> Self {
        Token { kind, start, end, line, col }
    }

    /// 该 token 所在行的缩进量（首 token 的 col 即缩进）。
    pub fn indent(&self) -> u32 {
        self.col
    }
}
