//! GDScript 2.0 抽象语法树（解析器产出，语义层消费）。
//!
//! 设计立场：类型注解（`int`/`Array[String]`/`Vector2`…）在 GDScript 中就是表达式形态，
//! 因此 `TypeRef` 直接复用 `Expr`，避免重复建模类型语法。

/// 字节区间 `(start, end)`。
pub type Span = (usize, usize);

#[derive(Clone, Debug)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new(name: String, span: Span) -> Self {
        Ident { name, span }
    }
}

// ───────────────────────── 表达式 ─────────────────────────

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    /// 二元运算：左、运算符、右。
    Binary(Box<Expr>, BinOp, Box<Expr>, Span),
    /// 一元运算。
    Unary(UnOp, Box<Expr>, Span),
    /// 调用：`callee(args)`。
    Call { callee: Box<Expr>, args: Vec<Arg>, span: Span },
    /// 下标 / 索引：`target[index]`。
    Index { target: Box<Expr>, index: Box<Expr>, span: Span },
    /// 成员访问：`target.name`。
    Attribute { target: Box<Expr>, name: Ident, span: Span },
    /// 内联 lambda / 具名 Callable。
    Lambda(Lambda, Span),
    /// 数组字面量。
    Array(Vec<Expr>, Span),
    /// 字典字面量：`{ key: value, ... }`。
    Dict(Vec<(Expr, Expr)>, Span),
    /// `await expr`。
    Await(Box<Expr>, Span),
    /// 节点路径：`$"path"` / `$Node/Child` / `%Unique`。
    NodePath { raw: String, bracketed: bool, unique: bool, span: Span },
    /// 参数化类型 / 泛型：`Base[T]` 或 `Base[T, U]`（`Array[int]`、`Dictionary[String, int]`）。
    /// 单参数时语义等同索引，但统一建模为泛型以避免与下标歧义。
    Generic(Box<Expr>, Vec<Expr>, Span),
    /// 三元表达式：`a if cond else b`。
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>, Span),
}

#[derive(Clone, Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Lt, Gt, Le, Ge, Eq, Ne,
    And, Or, BitAnd, BitOr, BitXor, Shl, Shr,
    Is, As, In,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnOp {
    Neg, Pos, Not, BitNot, Await,
}

#[derive(Clone, Debug)]
pub struct Arg {
    pub value: Expr,
    /// 具名参数（`name = value`）。
    pub name: Option<Ident>,
}

#[derive(Clone, Debug)]
pub struct Lambda {
    pub params: Vec<Param>,
    pub ret_ty: Option<Box<Expr>>,
    /// 函数体：单表达式（简洁 lambda）或多语句块。
    pub body: LambdaBody,
}

#[derive(Clone, Debug)]
pub enum LambdaBody {
    Expr(Box<Expr>),
    Block(Block),
}

// ───────────────────────── 语句 ─────────────────────────

#[derive(Clone, Debug)]
pub enum Stmt {
    /// 裸表达式语句。
    Expr(Expr),
    /// 赋值 / 复合赋值。
    Assign { target: Expr, op: AssignOp, value: Expr, span: Span },
    Var(VarDecl),
    Const(ConstDecl),
    Func(FuncDecl),
    Signal(SignalDecl),
    Enum(EnumDecl),
    Class(ClassDecl),
    If { branches: Vec<(Expr, Block)>, else_block: Option<Block>, span: Span },
    For { vars: Vec<Ident>, iter: Expr, body: Block, span: Span },
    While { cond: Expr, body: Block, span: Span },
    Match { expr: Expr, arms: Vec<MatchArm>, span: Span },
    Return { value: Option<Expr>, span: Span },
    Break(Span),
    Continue(Span),
    Pass(Span),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssignOp {
    Assign, Add, Sub, Mul, Div, Mod, BitAnd, BitOr, BitXor, BitShl, BitShr,
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub annotations: Vec<Annotation>,
    pub name: Ident,
    pub ty: Option<Expr>,
    pub value: Option<Expr>,
    /// 属性（getter/setter）块：`var x: int:` 后接 `get:`/`set:`。
    pub property: Option<PropertyBlock>,
    pub span: Span,
}

/// 属性访问器：getter / setter 的两种合法形态。
#[derive(Clone, Debug)]
pub enum Accessor {
    /// 内联语句块：`get:` / `set(value):` 后接缩进块（`param` 为 setter 的值形参名）。
    Inline { param: Option<Ident>, body: Block },
    /// 外部函数引用：`get = fn` / `set = fn`，引用定义在别处的函数（可逗号同行列举）。
    External { func: Ident },
}

/// 属性访问器块：`var x: T:` 后接 `get` / `set`。
#[derive(Clone, Debug)]
pub struct PropertyBlock {
    pub get: Option<Accessor>,
    pub set: Option<Accessor>,
}

#[derive(Clone, Debug)]
pub struct ConstDecl {
    pub annotations: Vec<Annotation>,
    pub name: Ident,
    pub ty: Option<Expr>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct FuncDecl {
    pub annotations: Vec<Annotation>,
    pub static_: bool,
    pub name: Ident,
    pub params: Vec<Param>,
    pub ret_ty: Option<Expr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: Ident,
    pub ty: Option<Expr>,
    pub default: Option<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct SignalDecl {
    pub name: Ident,
    pub params: Vec<Param>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct EnumDecl {
    pub name: Option<Ident>,
    pub entries: Vec<(Ident, Option<Expr>)>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct ClassDecl {
    pub name: Ident,
    pub extends: Option<Expr>,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Annotation {
    pub name: Ident,
    pub args: Vec<Arg>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub guard: Option<Expr>,
    pub body: Block,
}

/// match 模式（R1 覆盖常用形态）。
#[derive(Clone, Debug)]
pub enum MatchPattern {
    Literal(Literal),
    /// 枚举 / 常量路径（`State.IDLE`、`PI`、绑定名）。
    Path(Vec<Ident>),
    /// 绑定变量（`var x`）。
    Bind(Ident),
    /// 数组模式（R1 仅支持嵌套模式占位）。
    Array(Vec<MatchPattern>),
    /// 字典模式（R1 仅支持占位）。
    Dict(Vec<(MatchPattern, MatchPattern)>),
    /// 守卫模式 `pattern where cond` 的 `where` 由 arm.guard 承载。
    Wildcard, // _
    /// 多候选 `a, b`。
    Or(Vec<MatchPattern>),
    /// 数组/字典模式中的占位：`[1, 2, ..]` / `{a: 1, ..}`。
    Rest,
    /// 表达式模式：`1 + 1`、`1 < 2`、`x if y else z` 等（视为常量/布尔表达式）。
    Expr(Box<Expr>),
}

// ───────────────────────── 文件 ─────────────────────────

#[derive(Clone, Debug, Default)]
pub struct File {
    pub class_name: Option<Ident>,
    pub extends: Option<Expr>,
    pub body: Vec<Stmt>,
}

// ───────────────────────── 源切片 / 签名渲染 ─────────────────────────

/// 安全字节切片:反转/越界返回空串,并钳制到字符边界(消除 `&src[a..b]` a>b panic)。
pub fn span_slice(src: &str, a: usize, b: usize) -> &str {
    if a >= b {
        return "";
    }
    let a = a.min(src.len());
    let b = b.min(src.len());
    if a >= b {
        return "";
    }
    let mut a = a;
    while a > 0 && !src.is_char_boundary(a) {
        a -= 1;
    }
    let mut b = b;
    while b < src.len() && !src.is_char_boundary(b) {
        b += 1;
    }
    if a >= b {
        return "";
    }
    &src[a..b]
}

/// 取表达式源区间(各 Expr 变体末位自带 Span 的取之,`Literal` 无源则零长)。
pub fn expr_span(e: &Expr) -> (usize, usize) {
    use Expr::*;
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

/// 用源文件切片把表达式粗渲染成字符串(hover/symbol/签名 detail 用,不求完美)。
pub fn expr_to_text(src: &str, e: &Expr) -> String {
    let (a, b) = expr_span(e);
    span_slice(src, a, b).to_string()
}

/// 把源片段内空白(含换行/制表)折叠为单空格并裁边——用于跨行默认值渲染归一
/// (`func(): var x_of_a = 10` 在源里可能是两行缩进)。
fn collapse_ws(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_ws = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !in_ws && !out.is_empty() {
                out.push(' ');
            }
            in_ws = true;
        } else {
            out.push(c);
            in_ws = false;
        }
    }
    out.trim().to_string()
}

impl FuncDecl {
    /// 渲染方法签名(源码形态,无 body):`name(p: T := def, ...) -> R:`。
    /// 参数逐个按 `Param.span` 源切片 + 空白折叠——Literal 无 span,
    /// 逐字段 expr_to_text 会丢字面量默认值;整段切片天然保真(处理跨行
    /// lambda 默认值如 `call := func(): var x_of_a = 10`)。
    pub fn signature_text(&self, src: &str) -> String {
        self.render_signature(src, true)
    }

    /// 原样签名(保留跨行默认值的换行/缩进)——override 补全面 display 与
    /// 引擎一致(gdscript_editor 多行签名原文;oracle override_function_*
    /// include 以 `call := func():\n\tvar x_of_a = 10` 锁定)。
    pub fn signature_text_raw(&self, src: &str) -> String {
        self.render_signature(src, false)
    }

    fn render_signature(&self, src: &str, collapse: bool) -> String {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|p| {
                let s = span_slice(src, p.span.0, p.span.1);
                if collapse { collapse_ws(s) } else { s.to_string() }
            })
            .collect();
        let mut sig = String::with_capacity(
            8 + params.iter().map(|s| s.len()).sum::<usize>(),
        );
        sig.push_str(&self.name.name);
        sig.push('(');
        sig.push_str(&params.join(", "));
        sig.push(')');
        if let Some(rt) = &self.ret_ty {
            sig.push_str(" -> ");
            sig.push_str(&expr_to_text(src, rt));
        }
        sig.push(':');
        sig
    }
}
