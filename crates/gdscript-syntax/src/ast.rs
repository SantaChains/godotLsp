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
