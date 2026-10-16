//! 语义分析（R2）：单文件符号表 + 名称解析 + 表达式类型推断 + 基础诊断。
//!
//! 设计见 `docs/from-scratch-design.md` §6。R2 范围：
//! - 收集文件级声明（var/const/func/signal/enum/class）与函数局部作用域；
//! - 解析裸标识符：局部 → 形参 → 类成员 → self/super → 引擎类/内建类型 →
//!   全局函数/单例/常量；报告未声明标识符；
//! - 推断表达式类型（字面量/注解/沿引擎模型的成员访问）；
//! - 诊断：已知类型上访问未知成员。
//! 跨文件（class_name/preload/autoload）留待 R3。

use std::collections::HashMap;

use gdscript_engine::EngineApi;

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub span: (usize, usize),
}

#[derive(Debug, Default)]
pub struct Analysis {
    pub errors: Vec<SemanticError>,
}

/// 表达式推断出的类型。
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Ty {
    /// 无法确定（不报诊断，交给后续）。
    #[default]
    Unknown,
    Void,
    /// 具名类型：内建（int/float/String/Vector2…）或引擎类（Node2D…）。
    Named(String),
    Callable,
    Signal,
    /// 函数：包其返回类型，供 var x = foo() 推导。
    FuncRet(Box<Ty>),
}

impl Ty {
    fn name(&self) -> Option<&str> {
        match self {
            Ty::Named(n) => Some(n),
            _ => None,
        }
    }
}

/// GDScript 内建函数/常量中不依赖引擎 dump 的保留名。
const BUILTIN_RESERVED: &[&str] = &[
    "print", "prints", "printt", "printerr", "printraw",
    "len", "abs", "min", "max", "clamp", "round", "floor", "ceil", "sign",
    "pow", "sqrt", "posmod", "snapped", "lerp", "lerp_angle", "inverse_lerp",
    "remap", "move_toward", "smoothstep", "is_inf", "is_nan", "range_lerp",
    "PI", "TAU", "INF", "NAN", "SQRT12", "SQRT3",
    "true", "false", "null", "self", "super",
];

/// 项目上下文（由上层 project 层实现，供跨文件解析）。
/// gdscript-syntax 不依赖具体 project crate，靠此接口解耦。
pub trait ProjectContext {
    /// class_name 注册名 → 脚本代表类型（即该 class_name 本身）。
    fn class_name_type(&self, name: &str) -> Option<String>;
    /// autoload 单化名 → 类型。
    fn autoload_type(&self, name: &str) -> Option<String>;
    /// preload("res://x.gd") → 该脚本的 class_name 类型（若声明了）。
    fn preload_type(&self, _res_path: &str) -> Option<String> { None }
}

pub fn analyze(file: &File, engine: &EngineApi) -> Analysis {
    analyze_with(file, engine, None)
}

pub fn analyze_with(
    file: &File,
    engine: &EngineApi,
    project: Option<&dyn ProjectContext>,
) -> Analysis {
    let mut a = Analysis::default();
    let mut ctx = Ctx {
        engine,
        project,
        errors: &mut a.errors,
        members: HashMap::new(),
        locals: Vec::new(),
    };

    // 收集文件级成员名与类型。
    for stmt in &file.body {
        collect_member(stmt, &mut ctx.members);
    }
    if let Some(cn) = &file.class_name {
        ctx.members.insert(cn.name.clone(), Ty::Named(cn.name.clone()));
    }

    for stmt in &file.body {
        analyze_stmt(stmt, &mut ctx);
    }
    a
}

struct Ctx<'a> {
    engine: &'a EngineApi,
    project: Option<&'a dyn ProjectContext>,
    errors: &'a mut Vec<SemanticError>,
    /// 文件级成员名 → 类型。
    members: HashMap<String, Ty>,
    /// 局部作用域栈（每进入一个块压一层）：名 → 类型。
    locals: Vec<HashMap<String, Ty>>,
}

impl<'a> Ctx<'a> {
    fn declare_local(&mut self, name: &str, ty: Ty) {
        if let Some(top) = self.locals.last_mut() {
            top.insert(name.to_string(), ty);
        } else {
            self.locals.push(HashMap::from([(name.to_string(), ty)]));
        }
    }

    fn lookup(&self, name: &str) -> Option<Ty> {
        for scope in self.locals.iter().rev() {
            if let Some(t) = scope.get(name) {
                return Some(t.clone());
            }
        }
        self.members.get(name).cloned()
    }

    fn is_known(&self, name: &str) -> bool {
        if self.lookup(name).is_some() {
            return true;
        }
        if BUILTIN_RESERVED.contains(&name) {
            return true;
        }
        if self.engine.class_by_name(name).is_some() || self.engine.builtin_by_name(name).is_some() {
            return true;
        }
        if self.engine.utility_function(name).is_some()
            || self.engine.singleton_type(name).is_some()
            || self.engine.global_constants.iter().any(|c| c.name == name)
            || self.engine.global_enums.iter().any(|e| e.name == name)
        {
            return true;
        }
        // 项目级跨文件符号：class_name 注册与 autoload 单例。
        if let Some(p) = self.project
            && (p.class_name_type(name).is_some() || p.autoload_type(name).is_some()) {
                return true;
            }
        false
    }

    /// 跨文件符号解析为类型：class_name → 该类型；autoload → 其类型。
    fn project_ty(&self, name: &str) -> Option<Ty> {
        let p = self.project?;
        if let Some(t) = p.class_name_type(name) {
            return Some(Ty::Named(t));
        }
        p.autoload_type(name).map(Ty::Named)
    }

    fn err(&mut self, message: impl Into<String>, span: (usize, usize)) {
        self.errors.push(SemanticError { message: message.into(), span });
    }
}

fn collect_member(stmt: &Stmt, out: &mut HashMap<String, Ty>) {
    match stmt {
        Stmt::Var(v) => {
            let ty = v.ty.as_ref().map(type_ref_to_ty).unwrap_or(Ty::Unknown);
            out.insert(v.name.name.clone(), ty);
        }
        Stmt::Const(c) => {
            let ty = c.ty.as_ref().map(type_ref_to_ty).unwrap_or(Ty::Unknown);
            out.insert(c.name.name.clone(), ty);
        }
        Stmt::Func(f) => {
            let ret = f.ret_ty.as_ref().map(type_ref_to_ty).unwrap_or(Ty::Unknown);
            out.insert(f.name.name.clone(), Ty::FuncRet(Box::new(ret)));
        }
        Stmt::Signal(s) => { out.insert(s.name.name.clone(), Ty::Signal); }
        Stmt::Enum(e) => { if let Some(n) = &e.name { out.insert(n.name.clone(), Ty::Named(n.name.clone())); } }
        Stmt::Class(c) => { out.insert(c.name.name.clone(), Ty::Named(c.name.name.clone())); }
        _ => {}
    }
}

/// 把类型注解表达式（`int`/`Vector2`/`Array[int]`）归一成具名类型。
fn type_ref_to_ty(e: &Expr) -> Ty {
    match e {
        Expr::Ident(id) => Ty::Named(id.name.clone()),
        Expr::Generic(base, ..) => type_ref_to_ty(base),
        _ => Ty::Unknown,
    }
}

fn analyze_stmt(stmt: &Stmt, ctx: &mut Ctx) {
    match stmt {
        Stmt::Var(v) => {
            let inferred = v.value.as_ref().map(|e| infer_expr(e, ctx)).unwrap_or(Ty::Unknown);
            let declared = v.ty.as_ref().map(type_ref_to_ty);
            let ty = declared.unwrap_or(inferred);
            ctx.declare_local(&v.name.name, ty);
        }
        Stmt::Const(c) => {
            infer_expr(&c.value, ctx);
        }
        Stmt::Func(f) => {
            ctx.locals.push(HashMap::new());
            for p in &f.params {
                let ty = p.ty.as_ref().map(type_ref_to_ty).unwrap_or(Ty::Unknown);
                ctx.declare_local(&p.name.name, ty);
                if let Some(d) = &p.default {
                    infer_expr(d, ctx);
                }
            }
            analyze_block(&f.body, ctx);
            ctx.locals.pop();
        }
        Stmt::Class(_c) => {}
        Stmt::If { branches, else_block, .. } => {
            for (cond, body) in branches {
                infer_expr(cond, ctx);
                analyze_block(body, ctx);
            }
            if let Some(b) = else_block {
                analyze_block(b, ctx);
            }
        }
        Stmt::For { vars, iter, body, .. } => {
            infer_expr(iter, ctx);
            ctx.locals.push(HashMap::new());
            for v in vars {
                ctx.declare_local(&v.name, Ty::Unknown);
            }
            analyze_block(body, ctx);
            ctx.locals.pop();
        }
        Stmt::While { cond, body, .. } => {
            infer_expr(cond, ctx);
            analyze_block(body, ctx);
        }
        Stmt::Match { expr, arms, .. } => {
            infer_expr(expr, ctx);
            for arm in arms {
                analyze_block(&arm.body, ctx);
            }
        }
        Stmt::Return { value, .. } => {
            if let Some(e) = value {
                infer_expr(e, ctx);
            }
        }
        Stmt::Assign { target, value, .. } => {
            infer_expr(target, ctx);
            infer_expr(value, ctx);
        }
        Stmt::Expr(e) => { infer_expr(e, ctx); }
        _ => {}
    }
}

fn analyze_block(block: &Block, ctx: &mut Ctx) {
    for s in &block.stmts {
        analyze_stmt(s, ctx);
    }
}

/// 推断表达式类型，顺带产出诊断。
fn infer_expr(expr: &Expr, ctx: &mut Ctx) -> Ty {
    match expr {
        Expr::Ident(id) => {
            if let Some(ty) = ctx.lookup(&id.name) {
                return ty;
            }
            if !ctx.is_known(&id.name) {
                ctx.err(format!("未声明的标识符 '{}'", id.name), id.span);
            }
            // 引擎类/内建名作为类型引用时返回具名类型。
            if ctx.engine.class_by_name(&id.name).is_some() {
                return Ty::Named(id.name.clone());
            }
            // 项目级跨文件类型（class_name / autoload）。
            if let Some(ty) = ctx.project_ty(&id.name) {
                return ty;
            }
            Ty::Unknown
        }
        Expr::Literal(l) => match l {
            Literal::Int(_) => Ty::Named("int".into()),
            Literal::Float(_) => Ty::Named("float".into()),
            Literal::Str(_) => Ty::Named("String".into()),
            Literal::Bool(_) => Ty::Named("bool".into()),
            Literal::Null => Ty::Named("null".into()),
        },
        Expr::Binary(l, op, r, _) => {
            let lt = infer_expr(l, ctx);
            let rt = infer_expr(r, ctx);
            // 比较运算 → bool；算术 → 按操作数。
            use BinOp::*;
            match op {
                Lt | Gt | Le | Ge | Eq | Ne | Is | In => Ty::Named("bool".into()),
                Add | Sub | Mul | Div | Mod | BitAnd | BitOr | BitXor | Shl | Shr => {
                    let lf = matches!(lt, Ty::Named(ref n) if n == "float");
                    let rf = matches!(rt, Ty::Named(ref n) if n == "float");
                    if lf || rf { Ty::Named("float".into()) } else { lt }
                }
                _ => Ty::Unknown,
            }
        }
        Expr::Unary(_, e, _) => infer_expr(e, ctx),
        Expr::Call { callee, args, span } => {
            for a in args {
                infer_expr(&a.value, ctx);
            }
            match callee.as_ref() {
                // Vector2(...) 构造 → Vector2
                Expr::Ident(id) => {
                    if ctx.engine.class_by_name(&id.name).is_some() || ctx.engine.builtin_by_name(&id.name).is_some() {
                        return Ty::Named(id.name.clone());
                    }
                    if let Some(f) = ctx.engine.utility_function(&id.name) {
                        return Ty::Named(f.return_type.clone());
                    }
                    // 普通函数调用：查其返回类型（FuncRet 解包）。
                    if let Some(ty) = ctx.lookup(&id.name) {
                        return match ty {
                            Ty::FuncRet(rt) => *rt,
                            other => other,
                        };
                    }
                    Ty::Unknown
                }
                // obj.method(...)
                Expr::Attribute { target, name, .. } => {
                    let tt = infer_expr(target, ctx);
                    if let Some(cls) = tt.name() {
                        // 构造器 `Type.new()`：引擎 API 不含构造方法，特判为该类型。
                        if name.name == "new" {
                            return Ty::Named(cls.to_string());
                        }
                        if let Some(m) = ctx.engine.find_method(cls, &name.name) {
                            return Ty::Named(m.return_type.clone());
                        }
                        if ctx.engine.find_property(cls, &name.name).is_some() {
                            return Ty::Unknown;
                        }
                        ctx.err(format!("类型 '{}' 上没有方法 '{}'", cls, name.name), *span);
                    }
                    Ty::Unknown
                }
                _ => Ty::Unknown,
            }
        }
        Expr::Index { target, index, .. } => {
            infer_expr(target, ctx);
            infer_expr(index, ctx);
            Ty::Unknown
        }
        Expr::Attribute { target, name, span } => {
            let tt = infer_expr(target, ctx);
            if let Some(cls) = tt.name() {
                if let Some(p) = ctx.engine.find_property(cls, &name.name) {
                    return Ty::Named(p.ty.clone());
                }
                if ctx.engine.find_method(cls, &name.name).is_some() {
                    return Ty::Unknown; // 方法引用（未调用）
                }
                ctx.err(format!("类型 '{}' 上没有成员 '{}'", cls, name.name), *span);
            }
            Ty::Unknown
        }
        Expr::Lambda(lambda, _) => {
            if let Some(rt) = &lambda.ret_ty {
                return type_ref_to_ty(rt);
            }
            Ty::Callable
        }
        Expr::Array(elems, _) => {
            for e in elems { infer_expr(e, ctx); }
            Ty::Named("Array".into())
        }
        Expr::Dict(entries, _) => {
            for (k, v) in entries {
                infer_expr(k, ctx);
                infer_expr(v, ctx);
            }
            Ty::Named("Dictionary".into())
        }
        Expr::Await(e, _) => {
            infer_expr(e, ctx);
            Ty::Unknown
        }
        Expr::Ternary(c, t, f, _) => {
            infer_expr(c, ctx);
            let lt = infer_expr(t, ctx);
            infer_expr(f, ctx);
            lt
        }
        Expr::Generic(base, args, _) => {
            for a in args { infer_expr(a, ctx); }
            type_ref_to_ty(base)
        }
        Expr::NodePath { .. } => Ty::Named("NodePath".into()),
    }
}


// ───────────────────────── 公开查询 API（LSP 层用） ─────────────────────────

/// 文件级类型信息：LSP 查询变量/函数类型时用，不再自己扫文本。
#[derive(Debug, Default, Clone)]
pub struct FileInfo {
    pub members: HashMap<String, Ty>,
}

impl FileInfo {
    pub fn from_file(file: &File) -> Self {
        let mut members = HashMap::new();
        for stmt in &file.body {
            collect_member(stmt, &mut members);
        }
        if let Some(cn) = &file.class_name {
            members.insert(cn.name.clone(), Ty::Named(cn.name.clone()));
        }
        FileInfo { members }
    }

    pub fn resolve_var(&self, src: &str, name: &str) -> Option<String> {
        if let Some(ty) = self.members.get(name) {
            return match ty {
                Ty::Named(n) => Some(n.clone()),
                Ty::FuncRet(rt) => match &**rt {
                    Ty::Named(n) => Some(n.clone()),
                    _ => None,
                },
                _ => None,
            };
        }
        for line in src.lines() {
            let t = line.trim_start();
            for prefix in ["var ", "@onready var "] {
                if let Some(rest) = t.strip_prefix(prefix)
                    && let Some(col) = rest.find(':')
                        && rest[..col].trim() == name {
                            let ty = rest[col + 1..].trim().split('=').next()?.trim();
                            if !ty.is_empty() {
                                return Some(ty.to_string());
                            }
                        }
            }
        }
        None
    }

    pub fn func_return(&self, name: &str) -> Option<String> {
        match self.members.get(name)? {
            Ty::FuncRet(rt) => match &**rt {
                Ty::Named(n) => Some(n.clone()),
                _ => None,
            },
            _ => None,
        }
    }
}
