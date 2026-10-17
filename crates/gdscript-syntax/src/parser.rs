//! GDScript 2.0 解析器：递归下降 + Pratt，带 panic-free错误恢复。
//!
//! 设计立场（见 `docs/from-scratch-design.md` §5）：
//! - 产出结构化 AST（见 `ast.rs`）；
//! - 错误恢复靠「同步 token」（Newline / Indent / Dedent / 语句起始关键字）重置，
//!   保证每个文件错误有界、绝不 panic；
//! - 缩进分块由词法的 Indent/Dedent 驱动。

use crate::ast::*;
use crate::ast::Accessor;
use crate::error::ParseError;
use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

const EOF: Token = Token {
    kind: TokenKind::Eof,
    start: 0,
    end: 0,
    line: 0,
    col: 0,
};

pub struct Parser<'a> {
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
    last_end: usize,
    errors: Vec<ParseError>,
    step: usize,
    watchdog_tripped: bool,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            src,
            toks: tokenize(src),
            pos: 0,
            last_end: 0,
            errors: Vec::new(),
            step: 0,
            watchdog_tripped: false,
        }
    }

    // ── 词法访问器 ──
    fn peek(&self) -> Token {
        self.toks.get(self.pos).copied().unwrap_or(EOF)
    }
    fn peek_at(&self, n: usize) -> Token {
        self.toks.get(self.pos + n).copied().unwrap_or(EOF)
    }
    fn advance(&mut self) -> Token {
        // 看门狗：单文件推进次数远超合理上限即判定为死循环，
        // 强制跳到 EOF 让所有解析循环优雅终止（绝不挂起进程）。
        self.step += 1;
        if self.step > 500_000 {
            if !self.watchdog_tripped {
                self.watchdog_tripped = true;
                self.errors.push(ParseError::new(
                    "解析看门狗触发：疑似死循环，已截断",
                    self.cur_start(),
                    self.cur_start(),
                    0,
                ));
            }
            self.pos = self.toks.len();
            return EOF;
        }
        let t = self.peek();
        if self.pos < self.toks.len() {
            self.pos += 1;
            self.last_end = t.end;
        }
        t
    }
    fn at(&self, k: TokenKind) -> bool {
        self.peek().kind == k
    }
    fn at_any(&self, ks: &[TokenKind]) -> bool {
        ks.contains(&self.peek().kind)
    }
    /// 前一个 token 是否为块边界（Newline / Dedent）——用于区分三元 `x if .. else ..`
    /// 与紧随其后的 `if` 语句（悬挂 if，如块体 lambda 之后换行的 `if`）。
    fn prev_is_boundary(&self) -> bool {
        self.pos
            .checked_sub(1)
            .and_then(|p| self.toks.get(p))
            .map(|t| matches!(t.kind, TokenKind::Newline | TokenKind::Dedent))
            .unwrap_or(false)
    }
    fn bump_if(&mut self, k: TokenKind) -> bool {
        if self.at(k) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn expect(&mut self, k: TokenKind) -> Option<Token> {
        if self.at(k) {
            Some(self.advance())
        } else if self.at(TokenKind::Eof) {
            // 截断输入容错：文件在期望 token 处结束，静默恢复不报错（LSP 增量解析硬需求）。
            None
        } else {
            self.error(format!("期望 {:?}", k), self.peek());
            None
        }
    }
    fn text(&self, t: Token) -> &str {
        &self.src[t.start..t.end]
    }
    fn span_of(&self, t: Token) -> Span {
        (t.start, t.end)
    }
    fn cur_start(&self) -> usize {
        self.toks.get(self.pos).map(|t| t.start).unwrap_or(self.src.len())
    }
    fn error(&mut self, msg: impl Into<String>, t: Token) {
        self.errors.push(ParseError::new(msg, t.start, t.end, t.line));
    }

    /// 错误恢复：推进到下一个「语句起始」或结构 token。
    fn sync(&mut self) {
        while !self.at_any(&[TokenKind::Newline, TokenKind::Indent, TokenKind::Dedent, TokenKind::Eof])
            && !self.peek().kind.is_stmt_start()
        {
            if self.at(TokenKind::Eof) {
                break;
            }
            self.advance();
        }
        if self.at(TokenKind::Newline) {
            self.advance();
        }
    }

    // ── 顶层 ──
    pub fn parse_file(&mut self) -> (File, Vec<ParseError>) {
        let mut file = File::default();
        loop {
            let _anns = self.parse_annotations();
            // 注解单独成行（如 `@tool` 后换行再 `extends`）：吞掉换行继续。
            if self.at(TokenKind::Newline) {
                self.advance();
                continue;
            }
            if self.at(TokenKind::ClassName) {
                self.advance();
                if let Some(t) = self.expect(TokenKind::Ident) {
                    file.class_name = Some(Ident::new(self.text(t).to_string(), self.span_of(t)));
                }
                if self.at(TokenKind::Extends) {
                    self.advance();
                    file.extends = Some(self.parse_expr());
                }
                self.bump_if(TokenKind::Newline);
            } else if self.at(TokenKind::Extends) {
                self.advance();
                file.extends = Some(self.parse_expr());
                self.bump_if(TokenKind::Newline);
            } else {
                break;
            }
        }
        let body = if self.at(TokenKind::Indent) {
            self.advance();
            let b = self.parse_items(&[TokenKind::Dedent]);
            self.bump_if(TokenKind::Dedent);
            b
        } else {
            self.parse_items(&[])
        };
        file.body = body;
        (file, std::mem::take(&mut self.errors))
    }

    fn parse_items(&mut self, stop: &[TokenKind]) -> Vec<Stmt> {
        let mut items = Vec::new();
        loop {
            if self.at(TokenKind::Eof) || self.at_any(stop) {
                break;
            }
            if self.at(TokenKind::Newline) {
                self.advance();
                continue;
            }
            // 可选语句终止符 `;`（GDScript 允许 `stmt;` / `a; b; c`）。
            if self.at(TokenKind::Semicolon) {
                self.advance();
                continue;
            }
            // 容错：意外的缩进/反缩进（多由前序错误导致结构错位）。
            // 跳过整段缩进块，避免级联报错淹没真实问题。
            if self.at(TokenKind::Indent) {
                self.skip_indented_block();
                continue;
            }
            if self.at(TokenKind::Dedent) {
                // 多出的 Dedent（配对失衡）：不在 stop 集中则跳过。
                self.advance();
                continue;
            }
            // 容错：块级 stray Comma 只在 lambda 体作实参分隔时出现
            // (`reduce(func(a,b): ... \n return accum,\n {})`),
            // 跳过并让上层 parse_args 消费。
            if self.at(TokenKind::Comma) {
                self.advance();
                continue;
            }
            match self.parse_item() {
                Some(s) => items.push(s),
                None => self.sync(),
            }
        }
        items
    }

    /// 跳过一段缩进块（当前应位于 Indent 上）：消费 Indent 及到匹配 Dedent 的全部 token。
    fn skip_indented_block(&mut self) {
        if self.at(TokenKind::Indent) {
            self.advance();
        }
        let mut depth: i32 = 1;
        while !self.at(TokenKind::Eof) {
            if self.at(TokenKind::Indent) {
                depth += 1;
            } else if self.at(TokenKind::Dedent) {
                depth -= 1;
                self.advance();
                if depth == 0 {
                    break;
                }
                continue;
            }
            self.advance();
        }
    }

    /// 解析一个代码块：可选 Newline + Indent … Dedent；或同行单语句（内联块）。
    fn parse_block(&mut self) -> Block {
        let start = self.cur_start();
        if self.at(TokenKind::Newline) {
            // 缩进块。
            self.advance();
            self.bump_if(TokenKind::Indent);
            // 括号/逗号终止:lambda 体缩进块会被外层实参的 `)`/`,` 终结
            // (词法已保证该行注入 Dedent,但 stray 括号仍可能先行到达)。
            let stmts = self.parse_items(&[
                TokenKind::Dedent,
                TokenKind::RParen,
                TokenKind::RBracket,
                TokenKind::RBrace,
                TokenKind::Comma,
            ]);
            self.bump_if(TokenKind::Dedent);
            Block {
                stmts,
                span: (start, self.last_end),
            }
        } else {
            // 同行内联块：单语句/声明，或 `;` 分隔的多条；止于行尾/闭合括号。
            // （`get: return x`、`func f(): return 1`、`if c: stmt`、`b=func(): print(10)`。）
            let mut stmts = Vec::new();
            loop {
                if self.at_any(&[
                    TokenKind::Newline,
                    TokenKind::Dedent,
                    TokenKind::Eof,
                    TokenKind::RParen,
                    TokenKind::RBracket,
                    TokenKind::RBrace,
                    TokenKind::Comma, // lambda 内联体后作实参分隔
                ]) {
                    break;
                }
                if self.at(TokenKind::Semicolon) {
                    self.advance();
                    continue;
                }
                let before = self.pos;
                if let Some(s) = self.parse_item() {
                    stmts.push(s);
                }
                if self.pos == before {
                    self.advance();
                    break;
                }
                // 仅当紧跟 `;` 时才继续解析下一条，否则内联块到此为止。
                if !self.at(TokenKind::Semicolon) {
                    break;
                }
            }
            Block {
                stmts,
                span: (start, self.last_end),
            }
        }
    }

    // ── 项 / 声明 / 语句 ──
    fn parse_item(&mut self) -> Option<Stmt> {
        let anns = self.parse_annotations();
        // 注解附着于紧随的成员/语句（可跨换行）；吞掉其间换行。
        while self.at(TokenKind::Newline) {
            self.advance();
        }
        // 纯注解行（后接块尾/文件尾，无声明可附着）：安全跳过（不触发 sync 误吞下一行）。
        if self.at_any(&[TokenKind::Dedent, TokenKind::Eof]) {
            return None;
        }
        let mut static_ = false;
        if self.at(TokenKind::Static) {
            self.advance();
            static_ = true;
        }
        match self.peek().kind {
            TokenKind::Var => Some(self.parse_var(anns)),
            TokenKind::Const => Some(self.parse_const(anns)),
            TokenKind::Func => Some(self.parse_func_decl(anns, static_)),
            TokenKind::Signal => Some(self.parse_signal(anns)),
            TokenKind::Enum => Some(self.parse_enum(anns)),
            TokenKind::Class => Some(self.parse_class_decl(anns)),
            // 其余（含语句关键字与表达式）统一走 parse_stmt。
            _ => Some(self.parse_stmt()),
        }
    }

    /// 解析单条语句（表达式 / 赋值 / 控制流 / return 等，不含声明）。
    fn parse_stmt(&mut self) -> Stmt {
        match self.peek().kind {
            TokenKind::If => self.parse_if(),
            TokenKind::For => self.parse_for(),
            TokenKind::While => self.parse_while(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Break => {
                let t = self.advance();
                Stmt::Break(self.span_of(t))
            }
            TokenKind::Continue => {
                let t = self.advance();
                Stmt::Continue(self.span_of(t))
            }
            TokenKind::Pass => {
                let t = self.advance();
                Stmt::Pass(self.span_of(t))
            }
            // `extends` 亦可出现在类体首行（内层类体的继承声明）。
            TokenKind::Extends => {
                self.advance();
                let e = self.parse_expr();
                Stmt::Expr(e)
            }
            _ => {
                let lhs = self.parse_expr();
                if let Some(aop) = self.assign_op() {
                    self.advance();
                    // 截断赋值容错：`x = ` 后直接行尾/缩进边界/EOF，RHS 补空（LSP 增量解析）。
                    let rhs = if matches!(
                        self.peek().kind,
                        TokenKind::Newline | TokenKind::Dedent | TokenKind::Eof
                    ) {
                        Expr::Literal(Literal::Null)
                    } else {
                        self.parse_expr()
                    };
                    let span = (self.cur_start(), self.last_end);
                    Stmt::Assign {
                        target: lhs,
                        op: aop,
                        value: rhs,
                        span,
                    }
                } else {
                    Stmt::Expr(lhs)
                }
            }
        }
    }

    fn parse_annotations(&mut self) -> Vec<Annotation> {
        let mut out = Vec::new();
        while self.at(TokenKind::At) {
            let start = self.cur_start();
            self.advance();
            let name = if self.at(TokenKind::Ident) {
                let t = self.advance();
                Ident::new(self.text(t).to_string(), self.span_of(t))
            } else {
                let t = self.peek();
                self.error("注解期望标识符", t);
                Ident::new(String::new(), (t.start, t.end))
            };
            let mut args = Vec::new();
            if self.at(TokenKind::LParen) {
                args = self.parse_args();
            }
            out.push(Annotation {
                name,
                args,
                span: (start, self.last_end),
            });
            // 吞掉注解之间的换行（多个注解可各自成行）。
            while self.at(TokenKind::Newline) {
                self.advance();
            }
        }
        out
    }

    fn parse_var(&mut self, anns: Vec<Annotation>) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Var
        let name = self.parse_ident();
        let mut ty = None;
        let mut value = None;
        let mut property = None;
        if self.at(TokenKind::ColonEq) {
            // `var x := expr` 推断类型。
            self.advance();
            value = Some(self.parse_expr());
        } else if self.at(TokenKind::Colon) {
            let after = self.peek_at(1).kind;
            if after == TokenKind::Assign {
                // `var x : = expr` 带空格的推断写法（容忍）。
                self.advance();
                self.advance();
                value = Some(self.parse_expr());
            } else if after == TokenKind::Newline || after == TokenKind::Indent {
                // `var x:` 无类型属性块（`:` 即属性块引导，非类型分隔）。
                self.advance();
                property = Some(self.parse_property_block());
            } else {
                // 有类型注解：`var x: T` / `var x: T = v` / `var x: T:`（属性块）。
                self.advance();
                ty = Some(self.parse_expr());
                if self.at(TokenKind::Assign) {
                    self.advance();
                    value = Some(self.parse_expr());
                }
            }
        } else if self.at(TokenKind::Assign) {
            self.advance();
            value = Some(self.parse_expr());
        }
        // 尾随属性块：`var x := v:` / `var x: T:` 形态（第二个 `:`）。
        if property.is_none() && self.at(TokenKind::Colon) {
            self.advance();
            property = Some(self.parse_property_block());
        }
        Stmt::Var(VarDecl {
            annotations: anns,
            name,
            ty,
            value,
            property,
            span: (start, self.last_end),
        })
    }

    /// 解析属性访问器块：`get` / `set`，支持三种形态：
    /// - 内联块无参：`get:` / `set:` 后接缩进语句块；
    /// - 内联块带参：`set(value):` 后接缩进语句块；
    /// - 外部函数引用：`get = fn` / `set = fn`，可逗号同行列举（`set = a, get = b`）。
    fn parse_property_block(&mut self) -> PropertyBlock {
        self.bump_if(TokenKind::Newline);
        self.bump_if(TokenKind::Indent);
        let mut get = None;
        let mut set = None;
        while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
            let before = self.pos;
            if self.at(TokenKind::Newline) {
                self.advance();
            } else if self.at(TokenKind::Ident) {
                let t = self.advance();
                let kw = self.text(t).to_string();
                if kw == "get" || kw == "set" {
                    if self.bump_if(TokenKind::Assign) {
                        // 外部函数引用形式。
                        let fname = self.parse_ident();
                        if kw == "get" {
                            get = Some(Accessor::External { func: fname });
                        } else {
                            set = Some(Accessor::External { func: fname });
                        }
                        // 可能逗号同行列举另一访问器（`set = a, get = b`）。
                        self.bump_if(TokenKind::Comma);
                    } else {
                        // 内联块形式：`get:` / `get():` / `set:` / `set(value):`。
                        let param = if self.at(TokenKind::LParen) {
                            self.advance();
                            // 空括号 `get()`/`set()` 无参；否则取值形参名。
                            let p = if self.at(TokenKind::RParen) {
                                None
                            } else {
                                Some(self.parse_ident())
                            };
                            self.bump_if(TokenKind::RParen);
                            p
                        } else {
                            None
                        };
                        if self.expect(TokenKind::Colon).is_some() {
                            let body = self.parse_block();
                            if kw == "get" {
                                get = Some(Accessor::Inline { param: None, body });
                            } else {
                                set = Some(Accessor::Inline { param, body });
                            }
                        } else {
                            // 缺少 `:`：同步恢复，避免级联。
                            self.sync();
                        }
                    }
                } else {
                    // 非 get/set 标识符（错位）：同步恢复。
                    self.sync();
                }
            } else {
                // 非预期的 token（如 Indent 错位）：同步恢复。
                self.sync();
            }
            // 防御：若整轮未推进（sync 在同步 token 上不前进），强制前进，杜绝死循环。
            if self.pos == before && !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
                self.advance();
            }
        }
        self.bump_if(TokenKind::Dedent);
        PropertyBlock { get, set }
    }

    fn parse_const(&mut self, anns: Vec<Annotation>) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Const
        let name = self.parse_ident();
        let mut ty = None;
        let mut assign_consumed = false;
        if self.at(TokenKind::ColonEq) {
            // `const X := expr` 推断类型（`=` 已含在 := 中）。
            self.advance();
            assign_consumed = true;
        } else if self.at(TokenKind::Colon) {
            self.advance();
            if self.at(TokenKind::Assign) {
                // `const X : = expr` 带空格（容忍，`=` 已含）。
                self.advance();
                assign_consumed = true;
            } else {
                ty = Some(self.parse_expr());
            }
        }
        let value = if assign_consumed || self.expect(TokenKind::Assign).is_some() {
            self.parse_expr()
        } else {
            Expr::Literal(Literal::Null)
        };
        Stmt::Const(ConstDecl {
            annotations: anns,
            name,
            ty,
            value,
            span: (start, self.last_end),
        })
    }

    fn parse_func_decl(&mut self, anns: Vec<Annotation>, static_: bool) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Func
        // 截断函数声明容错：`func ` 后直接行尾/缩进边界/EOF，按空声明返回。
        if matches!(self.peek().kind, TokenKind::Newline | TokenKind::Dedent | TokenKind::Eof) {
            return Stmt::Func(FuncDecl {
                annotations: anns,
                static_,
                name: Ident::new(String::new(), (self.cur_start(), self.cur_start())),
                params: Vec::new(),
                ret_ty: None,
                body: Block { stmts: Vec::new(), span: (start, self.last_end) },
                span: (start, self.last_end),
            });
        }
        let name = self.parse_ident();
        self.expect(TokenKind::LParen);
        let params = self.parse_params();
        self.bump_if(TokenKind::RParen);
        let mut ret_ty = None;
        if self.at(TokenKind::Arrow) {
            self.advance();
            ret_ty = Some(self.parse_expr());
        }
        // 抽象函数可无函数体（直接结束，或 `;` 终止）。
        let body = if self.at(TokenKind::Colon) {
            self.advance();
            self.parse_block()
        } else {
            Block {
                stmts: Vec::new(),
                span: (self.cur_start(), self.last_end),
            }
        };
        self.bump_if(TokenKind::Semicolon); // 抽象声明可能以 `;` 结束
        Stmt::Func(FuncDecl {
            annotations: anns,
            static_,
            name,
            params,
            ret_ty,
            body,
            span: (start, self.last_end),
        })
    }

    fn parse_signal(&mut self, _anns: Vec<Annotation>) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Signal
        let name = self.parse_ident();
        let mut params = Vec::new();
        if self.at(TokenKind::LParen) {
            self.advance();
            params = self.parse_params();
            self.bump_if(TokenKind::RParen);
        }
        Stmt::Signal(SignalDecl {
            name,
            params,
            span: (start, self.last_end),
        })
    }

    fn parse_enum(&mut self, _anns: Vec<Annotation>) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Enum
        let name = if self.at(TokenKind::Ident) {
            let t = self.advance();
            Some(Ident::new(self.text(t).to_string(), self.span_of(t)))
        } else {
            None
        };
        let mut entries = Vec::new();
        if self.expect(TokenKind::LBrace).is_some() {
            while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
                let ename = self.parse_ident();
                let val = if self.at(TokenKind::Assign) {
                    self.advance();
                    Some(self.parse_expr())
                } else {
                    None
                };
                entries.push((ename, val));
                if !self.bump_if(TokenKind::Comma) {
                    break;
                }
            }
            self.bump_if(TokenKind::RBrace);
        }
        Stmt::Enum(EnumDecl {
            name,
            entries,
            span: (start, self.last_end),
        })
    }

    fn parse_class_decl(&mut self, anns: Vec<Annotation>) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Class
        let name = self.parse_ident();
        let mut extends = None;
        if self.at(TokenKind::Extends) {
            self.advance();
            extends = Some(self.parse_expr());
        }
        self.expect(TokenKind::Colon);
        let body = self.parse_block();
        let _ = anns;
        Stmt::Class(ClassDecl {
            name,
            extends,
            body: body.stmts,
            span: (start, self.last_end),
        })
    }

    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        while !self.at(TokenKind::RParen) && !self.at(TokenKind::Eof) {
            let before = self.pos;
            // 参数自身区间起点（含 `...` 变长前缀）。
            let start = self.peek().start;
            // 变长参数：`...args`（R1 容错消费前缀，仅取名字）。
            if self.at(TokenKind::Dot)
                && self.peek_at(1).kind == TokenKind::Dot
                && self.peek_at(2).kind == TokenKind::Dot
            {
                self.advance();
                self.advance();
                self.advance();
            }
            let name = self.parse_ident();
            let mut ty = None;
            let mut default = None;
            if self.at(TokenKind::ColonEq) {
                // `name := value`：推断类型并带默认值（函数形参/默认值类型推断）。
                self.advance();
                default = Some(self.parse_expr());
            } else {
                if self.at(TokenKind::Colon) {
                    self.advance();
                    ty = Some(self.parse_expr());
                }
                if self.at(TokenKind::Assign) {
                    self.advance();
                    default = Some(self.parse_expr());
                }
            }
            // span 覆盖参数自身（name 起、最后消费 token 止）。旧实现取
            // span_of(self.peek()) 指向下一 token（`,`/`)`），并非参数区间；
            // 无消费者依赖旧语义（高亮/类型推断均走 name.span），P5 签名渲染
            // 首次消费真实区间。
            let end = self.last_end.max(start);
            params.push(Param {
                name,
                ty,
                default,
                span: (start, end),
            });
            if !self.bump_if(TokenKind::Comma) {
                break;
            }
            // 防御：若本轮未推进（遇意外 token），强制前进避免死循环。
            if self.pos == before {
                self.advance();
            }
        }
        params
    }

    fn parse_ident(&mut self) -> Ident {
        let k = self.peek().kind;
        if k == TokenKind::Ident || crate::token::is_soft_keyword(k) {
            let t = self.advance();
            Ident::new(self.text(t).to_string(), self.span_of(t))
        } else if let Some(t) = self.expect(TokenKind::Ident) {
            Ident::new(self.text(t).to_string(), self.span_of(t))
        } else {
            let t = self.peek();
            Ident::new(String::new(), (t.start, t.end))
        }
    }

    /// 成员名（`.` 之后）：可为任意关键字或标识符（`dict.if`、`obj.self`、`x.class_name`）。
    fn parse_member_name(&mut self) -> Ident {
        let k = self.peek().kind;
        if k == TokenKind::Ident
            || k.is_hard_keyword()
            || crate::token::is_soft_keyword(k)
        {
            let t = self.advance();
            Ident::new(self.text(t).to_string(), self.span_of(t))
        } else {
            self.parse_ident()
        }
    }

    fn parse_if(&mut self) -> Stmt {
        let start = self.cur_start();
        self.advance(); // If
        let cond = self.parse_expr();
        self.expect(TokenKind::Colon);
        let mut branches = vec![(cond, self.parse_block())];
        let mut else_block = None;
        // elif/else 可跟在内联体之后的下一行(`if c: stmt\nelif d: ...`),
        // 链检查前跳过行尾 Newline(多余 Newline 由 parse_items 容错)。
        self.bump_if(TokenKind::Newline);
        while self.at(TokenKind::Elif) {
            self.advance();
            let c = self.parse_expr();
            self.expect(TokenKind::Colon);
            branches.push((c, self.parse_block()));
            self.bump_if(TokenKind::Newline);
        }
        if self.at(TokenKind::Else) {
            self.advance();
            self.expect(TokenKind::Colon);
            else_block = Some(self.parse_block());
        }
        Stmt::If {
            branches,
            else_block,
            span: (start, self.last_end),
        }
    }

    fn parse_for(&mut self) -> Stmt {
        let start = self.cur_start();
        self.advance(); // For
        let mut vars = vec![self.parse_for_var()];
        while self.at(TokenKind::Comma) {
            self.advance();
            vars.push(self.parse_for_var());
        }
        self.expect(TokenKind::In);
        let iter = self.parse_expr();
        self.expect(TokenKind::Colon);
        let body = self.parse_block();
        Stmt::For {
            vars,
            iter,
            body,
            span: (start, self.last_end),
        }
    }

    /// `for` 循环变量，支持可选类型注解：`for e: float in ...`。
    /// 类型注解用 `parse_postfix` 解析（不吞 `in` 等中缀运算符——`in` 属于循环头）。
    fn parse_for_var(&mut self) -> Ident {
        let name = self.parse_ident();
        if self.at(TokenKind::Colon) {
            self.advance();
            let _ty = self.parse_postfix(); // R1 容错消费类型注解（语义层再校验）
        }
        name
    }

    fn parse_while(&mut self) -> Stmt {
        let start = self.cur_start();
        self.advance(); // While
        let cond = self.parse_expr();
        self.expect(TokenKind::Colon);
        let body = self.parse_block();
        Stmt::While {
            cond,
            body,
            span: (start, self.last_end),
        }
    }

    fn parse_match(&mut self) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Match
        let expr = self.parse_expr();
        // 截断 match 容错：主题后直接行尾/缩进边界/EOF，不期望冒号。
        if !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Dedent | TokenKind::Eof) {
            self.expect(TokenKind::Colon);
        }
        self.bump_if(TokenKind::Newline);
        self.bump_if(TokenKind::Indent);
        let mut arms = Vec::new();
        while !self.at(TokenKind::Dedent) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::Newline) {
                self.advance();
                continue;
            }
            // 裸 `pass` 臂：核验自 Godot `gdscript_parser.cpp` 的 `parse_match` ——
            // 分支循环中 `match(PASS)` 直接吞掉该 token（要求后跟换行）并 continue，
            // 不创建任何分支。`match 0:` + `pass` 体即依赖此路径（match.gd 语料）。
            if self.at(TokenKind::Pass) {
                self.advance();
                self.bump_if(TokenKind::Newline);
                continue;
            }
            // match 臂前可有注解（如 `@warning_ignore("unreachable_pattern")`）。
            let _anns = self.parse_annotations();
            if self.at(TokenKind::Newline) {
                self.advance();
                continue;
            }
            let pattern = self.parse_match_pattern();
            let guard = if self.at(TokenKind::Where) {
                self.advance();
                Some(self.parse_expr())
            } else {
                None
            };
            // 不完整分支容错：模式后直接换行/缩进边界/EOF（光标停在模式后，如 `match t: AutoTranslateMode.`），
            // 不期望冒号、按空分支体处理。
            let body = if matches!(
                self.peek().kind,
                TokenKind::Newline | TokenKind::Dedent | TokenKind::Eof
            ) {
                Block { stmts: Vec::new(), span: (self.cur_start(), self.last_end) }
            } else {
                self.expect(TokenKind::Colon);
                self.parse_block()
            };
            arms.push(MatchArm {
                pattern,
                guard,
                body,
            });
        }
        self.bump_if(TokenKind::Dedent);
        Stmt::Match {
            expr,
            arms,
            span: (start, self.last_end),
        }
    }

    fn parse_match_pattern(&mut self) -> MatchPattern {
        let mut pats = vec![self.parse_one_pattern()];
        // 多候选仅以 `,` 分隔（GDScript 规范）；`or` 在表达式模式内部作布尔运算符，
        // 不再作为模式分隔符，以免与 `1 or 2 and 1` 这类表达式模式冲突。
        while self.at(TokenKind::Comma) {
            self.advance();
            pats.push(self.parse_one_pattern());
        }
        if pats.len() == 1 {
            pats.into_iter().next().unwrap()
        } else {
            MatchPattern::Or(pats)
        }
    }

    /// 解析一个 match 模式。
    /// - 结构模式：数组 `[...]`、字典 `{...}`、`var` 绑定——专门解析（支持 `..` 占位、嵌套绑定）。
    /// - 其余一律按表达式模式：字面量、`a.b` 路径、算术/比较/布尔(`or`/`and`)/位运算、三元，
    ///   以及裸 `_`（转为通配符）。逗号分隔的多个模式由 `parse_match_pattern` 汇总为 `Or`。
    fn parse_one_pattern(&mut self) -> MatchPattern {
        match self.peek().kind {
            TokenKind::LBracket => self.parse_array_pattern(),
            TokenKind::LBrace => self.parse_dict_pattern(),
            TokenKind::Var => {
                self.advance();
                MatchPattern::Bind(self.parse_ident())
            }
            _ => {
                let e = self.parse_expr();
                // 裸 `_` 视为通配符（单一标识符且名为 `_`）。
                if let Expr::Ident(id) = &e
                    && id.name == "_"
                {
                    return MatchPattern::Wildcard;
                }
                MatchPattern::Expr(Box::new(e))
            }
        }
    }

    /// 数组模式：`[a, b, ..]`，支持 `..` 占位（忽略剩余元素）。
    fn parse_array_pattern(&mut self) -> MatchPattern {
        self.advance(); // [
        let mut items = Vec::new();
        while !self.at(TokenKind::RBracket) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::Dot) && self.peek_at(1).kind == TokenKind::Dot {
                self.advance();
                self.advance();
                items.push(MatchPattern::Rest);
            } else {
                items.push(self.parse_one_pattern());
            }
            if !self.bump_if(TokenKind::Comma) {
                break;
            }
        }
        self.bump_if(TokenKind::RBracket);
        MatchPattern::Array(items)
    }

    /// 字典模式：`{a: 1, b: 2, ..}`，支持 `..` 占位。
    fn parse_dict_pattern(&mut self) -> MatchPattern {
        self.advance(); // {
        let mut entries = Vec::new();
        while !self.at(TokenKind::RBrace) && !self.at(TokenKind::Eof) {
            let key = if self.at(TokenKind::Dot) && self.peek_at(1).kind == TokenKind::Dot {
                self.advance();
                self.advance();
                MatchPattern::Rest
            } else {
                self.parse_one_pattern()
            };
            if self.bump_if(TokenKind::Colon) {
                let value = if self.at(TokenKind::Dot) && self.peek_at(1).kind == TokenKind::Dot {
                    self.advance();
                    self.advance();
                    MatchPattern::Rest
                } else {
                    self.parse_one_pattern()
                };
                entries.push((key, value));
            } else if !matches!(key, MatchPattern::Rest) {
                entries.push((key, MatchPattern::Wildcard));
            }
            if !self.bump_if(TokenKind::Comma) {
                break;
            }
        }
        self.bump_if(TokenKind::RBrace);
        MatchPattern::Dict(entries)
    }

    fn parse_return(&mut self) -> Stmt {
        let start = self.cur_start();
        self.advance(); // Return
        let value = if self.at_any(&[TokenKind::Newline, TokenKind::Dedent, TokenKind::Eof]) {
            None
        } else {
            Some(self.parse_expr())
        };
        Stmt::Return {
            value,
            span: (start, self.last_end),
        }
    }

    fn parse_args(&mut self) -> Vec<Arg> {
        self.expect(TokenKind::LParen);
        let mut args = Vec::new();
        // 括号内 lambda 体恢复结构注入后,参数列表中会出现 Newline/Dedent
        // (多行调用的单行 lambda 后、多行 lambda 体结束后、参数续行):
        // 全程容錯跳过,与 GDScript 括号内换行无意义语义一致。
        // (Dedent 只可能来自括号内 lambda 注入,跳过安全。)
        loop {
            while self.bump_if(TokenKind::Newline) || self.bump_if(TokenKind::Dedent) {}
            if self.at(TokenKind::RParen) || self.at(TokenKind::Eof) {
                break;
            }
            // 具名参数：`name = value`
            if self.at(TokenKind::Ident)
                && (self.peek_at(1).kind == TokenKind::Assign)
            {
                let nt = self.advance();
                self.advance(); // =
                let value = self.parse_expr();
                args.push(Arg {
                    value,
                    name: Some(Ident::new(self.text(nt).to_string(), self.span_of(nt))),
                });
            } else {
                args.push(Arg {
                    value: self.parse_expr(),
                    name: None,
                });
            }
            let mut ate_comma = self.bump_if(TokenKind::Comma);
            while self.bump_if(TokenKind::Newline) || self.bump_if(TokenKind::Dedent) {
                ate_comma = true;
            }
            if !ate_comma {
                break;
            }
        }
        while self.bump_if(TokenKind::Newline) || self.bump_if(TokenKind::Dedent) {}
        self.bump_if(TokenKind::RParen);
        args
    }

    fn assign_op(&self) -> Option<AssignOp> {
        Some(match self.peek().kind {
            TokenKind::Assign => AssignOp::Assign,
            TokenKind::PlusEq => AssignOp::Add,
            TokenKind::MinusEq => AssignOp::Sub,
            TokenKind::StarEq => AssignOp::Mul,
            TokenKind::SlashEq => AssignOp::Div,
            TokenKind::PercentEq => AssignOp::Mod,
            TokenKind::AmpEq => AssignOp::BitAnd,
            TokenKind::PipeEq => AssignOp::BitOr,
            TokenKind::CaretEq => AssignOp::BitXor,
            TokenKind::ShlEq => AssignOp::BitShl,
            TokenKind::ShrEq => AssignOp::BitShr,
            _ => return None,
        })
    }

    // ── 表达式：Pratt ──
    fn parse_expr(&mut self) -> Expr {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Expr {
        let mut lhs = self.parse_unary();
        loop {
            // 三元表达式：`lhs if cond else rhs`（GDScript 最低优先级）。
            // 仅当前一个 token 不是块边界（Newline/Dedent）时才认定为三元，
            // 以区别于悬挂的 `if` 语句（如块体 lambda 之后换行的 `if cond:`）。
            if self.at(TokenKind::If) && !self.prev_is_boundary() {
                let t = self.advance();
                let cond = self.parse_expr_bp(0);
                if self.bump_if(TokenKind::Else) {
                    let rhs = self.parse_expr_bp(min_bp);
                    let span = (self.cur_start(), self.last_end);
                    lhs = Expr::Ternary(Box::new(lhs), Box::new(cond), Box::new(rhs), span);
                } else {
                    self.error("三元表达式缺少 else", t);
                }
                continue;
            }
            let op = self.peek().kind;
            // `x not in y` ≡ `not (x in y)`：Not 在二元位且后跟 In 时构成 not-in 链
            //（GDScript 官方语法,Pixelorama 等真实工程高频使用）。
            if op == TokenKind::Not && self.peek_at(1).kind == TokenKind::In {
                self.advance();
                self.advance();
                let rhs = self.parse_expr_bp(6);
                let span = (self.cur_start(), self.last_end);
                lhs = Expr::Unary(
                    UnOp::Not,
                    Box::new(Expr::Binary(Box::new(lhs), BinOp::In, Box::new(rhs), span)),
                    span,
                );
                continue;
            }
            let (lbp, rbp) = match self.infix_bp(op) {
                Some(bp) => bp,
                None => break,
            };
            if lbp < min_bp {
                break;
            }
            self.advance();
            let rhs = self.parse_expr_bp(rbp);
            let span = (self.cur_start(), self.last_end);
            lhs = match self.binop_of(op) {
                Some(b) => Expr::Binary(Box::new(lhs), b, Box::new(rhs), span),
                None => lhs, // 防御
            };
        }
        lhs
    }

    fn infix_bp(&self, k: TokenKind) -> Option<(u8, u8)> {
        let bp = match k {
            TokenKind::Or | TokenKind::PipePipe => 3,
            TokenKind::And | TokenKind::AndAnd => 4,
            TokenKind::Is | TokenKind::As | TokenKind::In
            | TokenKind::EqEq | TokenKind::NotEq | TokenKind::Lt | TokenKind::Gt
            | TokenKind::LtEq | TokenKind::GtEq => 5,
            TokenKind::Pipe => 6,
            TokenKind::Caret => 7,
            TokenKind::Amp => 8,
            TokenKind::Shl | TokenKind::Shr => 9,
            TokenKind::Plus | TokenKind::Minus => 10,
            TokenKind::Star | TokenKind::Slash | TokenKind::PercentOp => 11,
            _ => return None,
        };
        Some((bp, bp))
    }

    fn binop_of(&self, k: TokenKind) -> Option<BinOp> {
        Some(match k {
            TokenKind::Or | TokenKind::PipePipe => BinOp::Or,
            TokenKind::And | TokenKind::AndAnd => BinOp::And,
            TokenKind::Is => BinOp::Is,
            TokenKind::As => BinOp::As,
            TokenKind::In => BinOp::In,
            TokenKind::EqEq => BinOp::Eq,
            TokenKind::NotEq => BinOp::Ne,
            TokenKind::Lt => BinOp::Lt,
            TokenKind::Gt => BinOp::Gt,
            TokenKind::LtEq => BinOp::Le,
            TokenKind::GtEq => BinOp::Ge,
            TokenKind::Pipe => BinOp::BitOr,
            TokenKind::Caret => BinOp::BitXor,
            TokenKind::Amp => BinOp::BitAnd,
            TokenKind::Shl => BinOp::Shl,
            TokenKind::Shr => BinOp::Shr,
            TokenKind::Plus => BinOp::Add,
            TokenKind::Minus => BinOp::Sub,
            TokenKind::Star => BinOp::Mul,
            TokenKind::Slash => BinOp::Div,
            TokenKind::PercentOp => BinOp::Mod,
            _ => return None,
        })
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek().kind {
            TokenKind::Minus => {
                let t = self.advance();
                // 负号折叠进整数字面量：正确表达 i64::MIN（`-9223372036854775808`）。
                if self.at(TokenKind::Int) {
                    let it = self.advance();
                    if let Some(m) = int_magnitude(self.text(it)) {
                        let v = if m <= (i64::MAX as u128) + 1 {
                            (-(m as i128)) as i64
                        } else {
                            i64::MIN // 超范围饱和
                        };
                        return Expr::Literal(Literal::Int(v));
                    }
                    return Expr::Literal(Literal::Int(0));
                }
                Expr::Unary(UnOp::Neg, Box::new(self.parse_unary()), self.span_of(t))
            }
            TokenKind::Plus => {
                let t = self.advance();
                Expr::Unary(UnOp::Pos, Box::new(self.parse_unary()), self.span_of(t))
            }
            TokenKind::Tilde => {
                let t = self.advance();
                Expr::Unary(UnOp::BitNot, Box::new(self.parse_unary()), self.span_of(t))
            }
            TokenKind::Bang => {
                let t = self.advance();
                Expr::Unary(UnOp::Not, Box::new(self.parse_unary()), self.span_of(t))
            }
            TokenKind::Not => {
                let t = self.advance();
                Expr::Unary(UnOp::Not, Box::new(self.parse_unary()), self.span_of(t))
            }
            TokenKind::Await => {
                let t = self.advance();
                Expr::Await(Box::new(self.parse_unary()), self.span_of(t))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut e = self.parse_primary();
        loop {
            match self.peek().kind {
                TokenKind::Dot => {
                    self.advance();
                    // 悬空成员访问容错（LSP 增量解析硬需求）：`.` 后紧跟行尾 / 闭合符 / EOF，
                    // 说明用户光标正停在此处输入（`obj.`、`$Node.`、`match X.`）。
                    // 视作不完整表达式：不报错，产空名 Attribute 并停止 postfix。
                    if matches!(
                        self.peek().kind,
                        TokenKind::Newline
                            | TokenKind::Dedent
                            | TokenKind::Eof
                            | TokenKind::RParen
                            | TokenKind::RBracket
                            | TokenKind::RBrace
                            | TokenKind::Comma
                            | TokenKind::Colon
                            | TokenKind::Assign
                    ) {
                        let span = (self.cur_start(), self.last_end);
                        e = Expr::Attribute {
                            target: Box::new(e),
                            name: Ident::new(String::new(), span),
                            span,
                        };
                        break;
                    }
                    let name = self.parse_member_name();
                    let span = (self.cur_start(), self.last_end);
                    e = Expr::Attribute {
                        target: Box::new(e),
                        name,
                        span,
                    };
                }
                TokenKind::LParen => {
                    let span = (self.cur_start(), self.last_end);
                    let args = self.parse_args();
                    e = Expr::Call {
                        callee: Box::new(e),
                        args,
                        span,
                    };
                }
                TokenKind::LBracket => {
                    self.advance();
                    let first = self.parse_expr();
                    if self.bump_if(TokenKind::Comma) {
                        // 参数化类型（多参数）：`Dictionary[int, int]`。
                        let mut args = vec![first];
                        while !self.at(TokenKind::RBracket) && !self.at(TokenKind::Eof) {
                            args.push(self.parse_expr());
                            if !self.bump_if(TokenKind::Comma) {
                                break;
                            }
                        }
                        self.expect(TokenKind::RBracket);
                        let span = (self.cur_start(), self.last_end);
                        e = Expr::Generic(Box::new(e), args, span);
                    } else {
                        self.expect(TokenKind::RBracket);
                        let span = (self.cur_start(), self.last_end);
                        e = Expr::Index {
                            target: Box::new(e),
                            index: Box::new(first),
                            span,
                        };
                    }
                }
                _ => break,
            }
        }
        e
    }

    fn parse_primary(&mut self) -> Expr {
        let t = self.peek();
        match t.kind {
            TokenKind::Int => {
                self.advance();
                // 宽容解析：超出 i64 也绝不报错，饱和到边界（R1）。
                let v = match int_magnitude(self.text(t)) {
                    Some(m) if m <= i64::MAX as u128 => m as i64,
                    Some(m) if m <= u64::MAX as u128 => m as i64, // 高位环绕
                    _ => i64::MAX,
                };
                Expr::Literal(Literal::Int(v))
            }
            TokenKind::Float => {
                self.advance();
                match self.text(t).replace('_', "").parse::<f64>() {
                    Ok(v) => Expr::Literal(Literal::Float(v)),
                    Err(_) => Expr::Literal(Literal::Float(0.0)),
                }
            }
            TokenKind::String => {
                self.advance();
                Expr::Literal(Literal::Str(self.unquote(self.text(t))))
            }
            TokenKind::True => {
                self.advance();
                Expr::Literal(Literal::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Expr::Literal(Literal::Bool(false))
            }
            TokenKind::Null => {
                self.advance();
                Expr::Literal(Literal::Null)
            }
            TokenKind::Void => {
                // void 当作类型名（标识符）
                self.advance();
                Expr::Ident(Ident::new("void".to_string(), self.span_of(t)))
            }
            TokenKind::Ident | TokenKind::SelfKw | TokenKind::Super => {
                self.advance();
                Expr::Ident(Ident::new(self.text(t).to_string(), self.span_of(t)))
            }
            TokenKind::Match | TokenKind::Where | TokenKind::In | TokenKind::As
            | TokenKind::Is | TokenKind::And | TokenKind::Or | TokenKind::Not => {
                // 软关键字在表达式位置作标识符（如 `print(when)`、`var match = ...`）。
                self.advance();
                Expr::Ident(Ident::new(self.text(t).to_string(), self.span_of(t)))
            }
            TokenKind::Caret => {
                // `^"path"` 是唯一节点路径字面量（与 `%"path"` 同义）。
                if self.peek_at(1).kind == TokenKind::String {
                    self.advance();
                    self.parse_node_path(true)
                } else {
                    let t = self.advance();
                    self.error("意外的 token ^", t);
                    Expr::Literal(Literal::Null)
                }
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.parse_expr();
                self.bump_if(TokenKind::RParen);
                inner
            }
            TokenKind::LBracket => self.parse_array(),
            TokenKind::LBrace => self.parse_dict(),
            TokenKind::Dollar => {
                self.advance();
                self.parse_node_path(false)
            }
            TokenKind::PercentOp => {
                // `%Unique` 唯一节点路径（词法把 `%` 记为 PercentOp）。
                self.advance();
                self.parse_node_path(true)
            }
            TokenKind::Amp => {
                if self.peek_at(1).kind == TokenKind::String {
                    self.advance();
                    self.parse_node_path(false)
                } else {
                    let t = self.advance();
                    self.error("意外的 &", t);
                    Expr::Literal(Literal::Null)
                }
            }
            TokenKind::Preload | TokenKind::Load => {
                let t = self.advance();
                let name = if t.kind == TokenKind::Preload {
                    "preload"
                } else {
                    "load"
                };
                let args = if self.at(TokenKind::LParen) {
                    self.parse_args()
                } else {
                    Vec::new()
                };
                Expr::Call {
                    callee: Box::new(Expr::Ident(Ident::new(name.to_string(), self.span_of(t)))),
                    args,
                    span: self.span_of(t),
                }
            }
            TokenKind::Func => self.parse_lambda(),
            // 截断输入容错：表达式位置遇 EOF，静默补 Null，不报错。
            TokenKind::Eof => Expr::Literal(Literal::Null),
            _ => {
                self.error(format!("意外的 token {:?}", t.kind), t);
                self.advance();
                Expr::Literal(Literal::Null)
            }
        }
    }

    fn parse_array(&mut self) -> Expr {
        let start = self.cur_start();
        self.advance(); // [
        let mut elems = Vec::new();
        loop {
            self.bump_if(TokenKind::Newline);
            if self.at(TokenKind::RBracket) || self.at(TokenKind::Eof) {
                break;
            }
            elems.push(self.parse_expr());
            let mut ate_comma = self.bump_if(TokenKind::Comma);
            while self.bump_if(TokenKind::Newline) {
                ate_comma = true;
            }
            if !ate_comma {
                break;
            }
        }
        while self.bump_if(TokenKind::Newline) {}
        self.bump_if(TokenKind::RBracket);
        Expr::Array(elems, (start, self.last_end))
    }

    fn parse_dict(&mut self) -> Expr {
        let start = self.cur_start();
        self.advance(); // {
        let mut entries = Vec::new();
        loop {
            self.bump_if(TokenKind::Newline);
            if self.at(TokenKind::RBrace) || self.at(TokenKind::Eof) {
                break;
            }
            // 字典条目：`key: value` 或 Lua 风格 `key = value`。
            let key = self.parse_expr();
            if self.bump_if(TokenKind::Colon) || self.bump_if(TokenKind::Assign) {
                let val = self.parse_expr();
                entries.push((key, val));
            } else {
                // 坏条目（无键值分隔符）：错误恢复——跳到下一个 Comma / RBrace，
                // 把坏条目 containment 在字典内，避免级联到语句层。
                entries.push((key, Expr::Literal(Literal::Null)));
                while !matches!(self.peek().kind, TokenKind::Comma | TokenKind::RBrace | TokenKind::Eof) {
                    self.advance();
                }
            }
            let mut ate_comma = self.bump_if(TokenKind::Comma);
            while self.bump_if(TokenKind::Newline) {
                ate_comma = true;
            }
            if !ate_comma {
                break;
            }
        }
        while self.bump_if(TokenKind::Newline) {}
        self.bump_if(TokenKind::RBrace);
        Expr::Dict(entries, (start, self.last_end))
    }

    /// 解析节点路径：`$`/`%`/`^` 前缀之后的部分。
    /// 路径字符为标识符、字符串、`/` 分隔符，以及 `%`（唯一段前缀，如 `$%Unique`、`%a/%b`）。
    /// 采用极大口咬（maximal munch），遇运算符/`.`/`,`/`)` 等即止——`.` 不计入（成员访问须留给 postfix）。
    fn parse_node_path(&mut self, mut unique: bool) -> Expr {
        let start = self.cur_start();
        let mut raw = String::new();
        let mut bracketed = false;
        // 前导 `%`：`$%Unique` 形态（`%Unique` 已在 parse_primary 消费前缀）。
        if self.at(TokenKind::PercentOp) {
            self.advance();
            unique = true;
        }
        loop {
            if self.at(TokenKind::String) {
                let t = self.advance();
                raw.push_str(&self.unquote(self.text(t)));
                bracketed = true;
            } else if self.at(TokenKind::PercentOp) {
                // 路径中段的唯一段前缀：`%Hey/%Howdy`。
                self.advance();
                raw.push('%');
            } else if self.at_any(&[TokenKind::Ident, TokenKind::Slash]) {
                let t = self.advance();
                raw.push_str(self.text(t));
            } else {
                break;
            }
        }
        Expr::NodePath {
            raw,
            bracketed,
            unique,
            span: (start, self.last_end),
        }
    }

    fn parse_lambda(&mut self) -> Expr {
        let start = self.cur_start();
        self.advance(); // Func
        // 具名 lambda：`func name():`（名字可选）。
        if self.at(TokenKind::Ident) {
            self.advance();
        }
        self.expect(TokenKind::LParen);
        let params = self.parse_params();
        self.bump_if(TokenKind::RParen);
        let mut ret_ty = None;
        if self.at(TokenKind::Arrow) {
            self.advance();
            ret_ty = Some(Box::new(self.parse_expr()));
        }
        self.expect(TokenKind::Colon);
        // lambda 体：支持调用括号内的多行多语句（`other(func(): print(1) print(2))`、
        // `(func(): return true).call()`）。括号内不注入 Newline/Indent token，
        // 故以 Newline/Dedent/EOF 及闭合括号/Comma 界定体边界，允许多语句无分号串联。
        let block = if self.at(TokenKind::Newline) {
            self.parse_block()
        } else {
            let mut stmts = Vec::new();
            loop {
                if self.at_any(&[
                    TokenKind::Newline,
                    TokenKind::Dedent,
                    TokenKind::Eof,
                    TokenKind::RParen,
                    TokenKind::RBracket,
                    TokenKind::RBrace,
                    TokenKind::Comma,
                ]) {
                    break;
                }
                let before = self.pos;
                if let Some(s) = self.parse_item() {
                    stmts.push(s);
                }
                if self.pos == before {
                    self.advance();
                    break;
                }
                if self.at(TokenKind::Semicolon) {
                    self.advance();
                }
            }
            Block { stmts, span: (start, self.last_end) }
        };
        let body = LambdaBody::Block(block);
        Expr::Lambda(
            Lambda {
                params,
                ret_ty,
                body,
            },
            (start, self.last_end),
        )
    }

    /// 去除字符串首尾引号（调用方已保证是 String token）。
    /// 兼容原始字符串 `r"..."` / `r'...'` 的 `r` 前缀。
    fn unquote(&self, s: &str) -> String {
        let s = s.strip_prefix('r').unwrap_or(s);
        let bytes = s.as_bytes();
        if bytes.len() >= 6 && bytes[0] == b'"' && bytes[1] == b'"' && bytes[2] == b'"'
            && bytes[bytes.len() - 1] == b'"' && bytes[bytes.len() - 2] == b'"'
            && bytes[bytes.len() - 3] == b'"'
        {
            s[3..s.len() - 3].to_string()
        } else if bytes.len() >= 2 && (bytes[0] == b'"' || bytes[0] == b'\'')
            && bytes[bytes.len() - 1] == bytes[0]
        {
            s[1..s.len() - 1].to_string()
        } else {
            s.to_string()
        }
    }
}

/// 把整数字面量文本解析为无符号量级（支持 `0x`/`0b`/`0o` 与 `_` 分隔）。
/// 失败（非法位/空）返回 `None`。
fn int_magnitude(txt: &str) -> Option<u128> {
    let s = txt.replace('_', "");
    let (radix, digits) = if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        (16u32, h)
    } else if let Some(b) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        (2, b)
    } else if let Some(o) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        (8, o)
    } else {
        (10, s.as_str())
    };
    if digits.is_empty() {
        return None;
    }
    u128::from_str_radix(digits, radix).ok()
}

/// 解析入口：返回 (AST, 错误列表)。保证不 panic。
pub fn parse(source: &str) -> (File, Vec<ParseError>) {
    let mut p = Parser::new(source);
    p.parse_file()
}
