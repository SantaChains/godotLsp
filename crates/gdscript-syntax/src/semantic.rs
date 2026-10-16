//! 语义分析（R2）：单文件符号表 + 名称解析 + 表达式类型推断 + 基础诊断。
//!
//! 设计见 `docs/from-scratch-design.md` §6。R2 范围：
//! - 收集文件级声明（var/const/func/signal/enum/class）与函数局部作用域；
//! - 解析裸标识符：局部 → 形参 → 类成员 → self/super → 引擎类/内建类型 →
//!   全局函数/单例/常量；报告未声明标识符；
//! - 推断表达式类型（字面量/注解/沿引擎模型的成员访问）；
//! - 诊断：已知类型上访问未知成员。
//!   跨文件（class_name/preload/autoload）留待 R3。

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
    "true", "false", "null", "self", "super", "Variant", "void",
];

/// @GDScript 语言内建函数（非 @GlobalScope 工具函数）。
/// 名单来自 godot-docs/gdd_1590_@GDScript.md 的 Methods 表；
/// str/typeof/type_convert 在 extension_api utility_functions 内，由引擎模型识别，不列于此。
const GDSCRIPT_BUILTINS: &[&str] = &[
    "Color8", "assert", "char", "convert", "dict_to_inst", "get_stack",
    "inst_to_dict", "is_instance_of", "load", "ord", "preload",
    "print_debug", "print_stack", "range", "type_exists",
];

/// 项目上下文（由上层 project 层实现，供跨文件解析）。
/// gdscript-syntax 不依赖具体 project crate，靠此接口解耦。
pub trait ProjectContext {
    /// class_name 注册名 → 脚本代表类型（即该 class_name 本身）。
    fn class_name_type(&self, name: &str) -> Option<String>;
    /// autoload 单化名 → 解析后的类型（class_name / 引擎类 / 合成 script:<path>）。
    fn autoload_type(&self, name: &str) -> Option<String>;
    /// preload("res://x.gd") → 该脚本的 class_name 类型（若声明了）。
    fn preload_type(&self, _res_path: &str) -> Option<String> { None }
    /// 类型键是否对应已索引脚本（class_name 或合成 script:<path>）。
    fn script_type_exists(&self, _type: &str) -> bool { false }
    /// 脚本自身成员命中种类（method/property/const），不含返回类型推断。
    fn script_member_kind(&self, _type: &str, _name: &str) -> Option<ScriptMember> { None }
    /// G4.6：脚本成员的声明类型（var: T / const: T / func -> T），无注解返回 None。
    fn script_member_type(&self, _type: &str, _name: &str) -> Option<String> { None }
    /// 脚本 `extends` 声明的引擎基类名（Ident/Generic 根），供沿引擎继承链查成员。
    fn script_extends_base(&self, _type: &str) -> Option<String> { None }
    /// 命名枚举条目查询（跨文件）：枚举名 → 条目是否存在。
    fn script_enum_entry(&self, _enum_name: &str, _key: &str) -> bool { false }
    /// 家族成员查询：类型自身或任一传递子类声明该成员（基类型变量可持
    /// 子类实例——GDScript 动态派发下不算静态错误）。
    fn script_family_member(&self, _type: &str, _name: &str) -> bool { false }
}

/// 脚本自身成员种类（G4）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScriptMember {
    Method,
    Property,
    Const,
    Signal,
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
    // self 继承链：extends 根 → 引擎类直接用；工程 class_name 沿脚本链走到引擎类。
    // 裸标识符（queue_redraw/position）按这条链查引擎成员。
    let self_base = extends_root_of(&file.extends).and_then(|root| {
        let mut cur = root.clone();
        for _ in 0..16 {
            if engine.class_by_name(&cur).is_some() || engine.builtin_by_name(&cur).is_some() {
                return Some(cur);
            }
            let p = project?;
            if p.class_name_type(&cur).is_some() {
                cur = p.script_extends_base(&cur)?;
            } else {
                return None;
            }
        }
        None
    });
    let mut ctx = Ctx {
        engine,
        project,
        errors: &mut a.errors,
        members: HashMap::new(),
        locals: Vec::new(),
        enum_members: HashMap::new(),
        self_base,
        extends_root: extends_root_of(&file.extends),
    };

    // 收集文件级成员名与类型。
    for stmt in &file.body {
        collect_member(stmt, &mut ctx.members, &mut ctx.enum_members);
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
    /// 命名枚举名 → 条目名列表（匿名枚举的条目已直接进 members 作 int 常量）。
    enum_members: HashMap<String, Vec<String>>,
    /// 局部作用域栈（每进入一个块压一层）：名 → 类型。
    locals: Vec<HashMap<String, Ty>>,
    /// self 的引擎基类（extends 链走到引擎类为止）：裸标识符按继承成员解析。
    self_base: Option<String>,
    /// 本文件 extends 根（引擎类或工程 class_name）：裸标识符按工程链解析。
    extends_root: Option<String>,
}

impl<'a> Ctx<'a> {
    fn declare_local(&mut self, name: &str, ty: Ty) {
        if let Some(top) = self.locals.last_mut() {
            top.insert(name.to_string(), ty);
        } else {
            self.locals.push(HashMap::from([(name.to_string(), ty)]));
        }
    }

    /// 覆盖/登记文件级成员类型（G3：preload/load 绑定）。
    fn bind_member(&mut self, name: &str, ty: Ty) {
        self.members.insert(name.to_string(), ty);
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
        if BUILTIN_RESERVED.contains(&name) || GDSCRIPT_BUILTINS.contains(&name) {
            return true;
        }
        if self.engine.class_by_name(name).is_some() || self.engine.builtin_by_name(name).is_some() {
            return true;
        }
        if self.engine.utility_function(name).is_some()
            || self.engine.singleton_type(name).is_some()
            || self.engine.global_enum_value(name).is_some()
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
        // self 继承链上的引擎成员（queue_redraw/position/delta 等裸写形态）。
        if let Some(base) = &self.self_base {
            for anc in self.engine.ancestor_chain(base) {
                if self.engine.find_method(anc, name).is_some()
                    || self.engine.find_property(anc, name).is_some()
                    || self.engine.find_const(anc, name).is_some()
                    || self.engine.find_signal(anc, name).is_some()
                    || self.engine.find_enum_value(anc, name).is_some()
                {
                    return true;
                }
            }
        }
        // self 沿工程脚本链的成员（Eraser 裸写基类 _drawer/_strength）。
        if let Some(root) = &self.extends_root
            && self.project.is_some_and(|p| p.script_member_kind(root, name).is_some())
        {
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

    /// 是否对缺失成员报错的严格类型：非标量内建（Vector2/Array/String…,
    /// 不可子类化,缺失即真错误）或工程 class_name（配合家族查询）。
    /// 标量（int/float/bool）与引擎类静默——前者方法形式（x.floor()）不在
    /// extension_api 模型,后者可被脚本子类化（downcast 惯用法）。
    fn strict_type(&self, cls: &str) -> bool {
        if matches!(cls, "int" | "float" | "bool") {
            return false;
        }
        self.engine.builtin_by_name(cls).is_some()
            || self.project.is_some_and(|p| p.class_name_type(cls).is_some())
    }

    /// 脚本自身成员命中（G4）。
    fn script_own_member(&self, cls: &str, name: &str) -> Option<ScriptMember> {
        self.project.and_then(|p| p.script_member_kind(cls, name))
    }

    /// 工程类型沿脚本链走到引擎类的根（GroupLayer → BaseLayer → … → Resource）：
    /// 工程类成员查询未命中时,引擎继承链成员（set_meta 等Object 方法）仍要可见。
    fn project_engine_root(&self, cls: &str) -> Option<String> {
        let p = self.project?;
        if p.class_name_type(cls).is_none() && !cls.starts_with("script:") {
            return None;
        }
        let mut cur = cls.to_string();
        for _ in 0..16 {
            let b = p.script_extends_base(&cur)?;
            if self.engine.class_by_name(&b).is_some() || self.engine.builtin_by_name(&b).is_some()
            {
                return Some(b);
            }
            cur = b;
        }
        None
    }

    /// 家族成员命中：自身或任一传递子类声明（动态派发放行）。
    fn script_family_member(&self, cls: &str, name: &str) -> bool {
        self.project.is_some_and(|p| p.script_family_member(cls, name))
    }

    /// G4.6：脚本成员声明类型。
    fn script_own_type(&self, cls: &str, name: &str) -> Option<String> {
        self.project.and_then(|p| p.script_member_type(cls, name))
    }

    /// 脚本 extends 基类（G4.5）。
    fn script_base(&self, cls: &str) -> Option<String> {
        self.project.and_then(|p| p.script_extends_base(cls))
    }

    /// 命名枚举是否含该条目（G5：State.KEY 合法，裸 KEY 不注册）。
    fn enum_entry(&self, en: &str, key: &str) -> bool {
        self.enum_members
            .get(en)
            .is_some_and(|v| v.iter().any(|k| k == key))
            // 跨文件：命名枚举全局索引（BaseLayer.BlendModes.PASS_THROUGH）。
            || self.project.is_some_and(|p| p.script_enum_entry(en, key))
    }


    /// 从 preload/load 调用取资源路径并绑定类型（G3）。
    /// preload 优先取项目脚本类型（class_name），否则回退 Resource；load 恒为 Resource。
    fn bind_load_call(&self, callee: &Expr, args: &[Arg]) -> Option<Ty> {
        let Expr::Ident(id) = callee else { return None };
        if id.name != "preload" && id.name != "load" {
            return None;
        }
        let path = match args.first().map(|a| &a.value)? {
            Expr::Literal(Literal::Str(p)) => p.clone(),
            _ => return None,
        };
        if id.name == "preload" {
            return self.project
                .and_then(|p| p.preload_type(&path))
                .map(Ty::Named)
                .or_else(|| Some(Ty::Named("Resource".into())));
        }
        Some(Ty::Named("Resource".into()))
    }

    fn err(&mut self, message: impl Into<String>, span: (usize, usize)) {
        self.errors.push(SemanticError { message: message.into(), span });
    }
}

fn collect_member(
    stmt: &Stmt,
    out: &mut HashMap<String, Ty>,
    enums: &mut HashMap<String, Vec<String>>,
) {
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
        // G5：匿名枚举（无名字）条目即文件级 int 常量，可裸写；命名枚举是 const Dictionary，
        // 条目只能 Name.KEY 访问，不进全局常量。
        Stmt::Enum(e) => {
            let names: Vec<String> = e.entries.iter().map(|(id, _)| id.name.clone()).collect();
            match &e.name {
                Some(n) => {
                    out.insert(n.name.clone(), Ty::Named(n.name.clone()));
                    enums.insert(n.name.clone(), names);
                }
                None => {
                    for k in names {
                        out.insert(k, Ty::Named("int".to_string()));
                    }
                }
            }
        }
        Stmt::Class(c) => { out.insert(c.name.name.clone(), Ty::Named(c.name.name.clone())); }
        _ => {}
    }
}

/// 把类型注解表达式（`int`/`Vector2`/`Array[int]`）归一成具名类型。
/// extends 表达式取根类名：`Node2D` / `Array[int]` → 根 Ident。
fn extends_root_of(e: &Option<Expr>) -> Option<String> {
    match e {
        Some(Expr::Ident(id)) => Some(id.name.clone()),
        Some(Expr::Generic(base, _, _)) => extends_root_of(&Some((**base).clone())),
        _ => None,
    }
}

fn type_ref_to_ty(e: &Expr) -> Ty {    match e {
        Expr::Ident(id) => Ty::Named(id.name.clone()),
        Expr::Generic(base, ..) => type_ref_to_ty(base),
        _ => Ty::Unknown,
    }
}

fn analyze_stmt(stmt: &Stmt, ctx: &mut Ctx) {
    match stmt {
        Stmt::Var(v) => {
            // G3：无注解且初始值为 preload/load 时，把绑定类型写入成员表。
            if v.ty.is_none()
                && let Some(Expr::Call { callee, args, .. }) = &v.value
                && let Some(ty) = ctx.bind_load_call(callee, args)
            {
                ctx.bind_member(&v.name.name, ty.clone());
                ctx.declare_local(&v.name.name, ty);
            } else {
                let inferred = v.value.as_ref().map(|e| infer_expr(e, ctx)).unwrap_or(Ty::Unknown);
                let declared = v.ty.as_ref().map(type_ref_to_ty);
                let ty = declared.unwrap_or(inferred);
                ctx.declare_local(&v.name.name, ty);
            }
        }
        Stmt::Const(c) => {
            // G3：const X = preload("res://x.gd") / load(...) 绑定资源类型，使 X 可解析。
            if let Expr::Call { callee, args, .. } = &c.value
                && let Some(ty) = ctx.bind_load_call(callee, args)
            {
                ctx.bind_member(&c.name.name, ty);
            }
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
        Stmt::Return { value: Some(e), .. } => {
            infer_expr(e, ctx);
        }
        Stmt::Return { value: None, .. } => {}
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
            // self 继承链成员类型回填（position → Vector2 等）。
            if let Some(base) = &ctx.self_base {
                for anc in ctx.engine.ancestor_chain(base) {
                    if let Some(p) = ctx.engine.find_property(anc, &id.name) {
                        return Ty::Named(p.ty.clone());
                    }
                    if ctx.engine.find_method(anc, &id.name).is_some() {
                        return Ty::Callable;
                    }
                    if ctx.engine.find_signal(anc, &id.name).is_some() {
                        return Ty::Named("Signal".into());
                    }
                    if ctx.engine.find_const(anc, &id.name).is_some() {
                        return Ty::Unknown;
                    }
                }
            }
            // self 沿工程脚本链的成员类型回填（_drawer/_strength 等）。
            if let Some(root) = &ctx.extends_root
                && let Some(kind) = ctx.project.and_then(|p| p.script_member_kind(root, &id.name))
            {
                return match kind {
                    ScriptMember::Method => Ty::Callable,
                    ScriptMember::Signal => Ty::Named("Signal".into()),
                    _ => ctx
                        .project
                        .and_then(|p| p.script_member_type(root, &id.name))
                        .map(Ty::Named)
                        .unwrap_or(Ty::Unknown),
                };
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
                        // G4.5：沿脚本 extends 基类的引擎继承链查方法。
                        let base = ctx.script_base(cls);
                        let base_method = base.as_deref().and_then(|b| ctx.engine.find_method(b, &name.name));
                        if let Some(m) = base_method {
                            return Ty::Named(m.return_type.clone());
                        }
                        let own = ctx.script_own_member(cls, &name.name);
                        let base_prop = base.as_deref().is_some_and(|b| ctx.engine.find_property(b, &name.name).is_some());
                        if own == Some(ScriptMember::Method)
                            || own == Some(ScriptMember::Property)
                            || own == Some(ScriptMember::Const)
                            || ctx.engine.find_property(cls, &name.name).is_some()
                            || base_prop
                        {
                            // G4.6：脚本方法返回类型/成员声明类型回写；无注解回退 Unknown。
                            return ctx
                                .script_own_type(cls, &name.name)
                                .map(Ty::Named)
                                .unwrap_or(Ty::Unknown);
                        }
                        // 工程链走到引擎根：Object 级成员（set_meta/connect/free）仍可见。
                        if let Some(er) = ctx.project_engine_root(cls) {
                            if let Some(m) = ctx.engine.find_method(&er, &name.name) {
                                return Ty::Named(m.return_type.clone());
                            }
                            if ctx.engine.find_property(&er, &name.name).is_some()
                                || ctx.engine.find_signal(&er, &name.name).is_some()
                                || ctx.engine.find_const(&er, &name.name).is_some()
                                || ctx.engine.find_enum_value(&er, &name.name).is_some()
                            {
                                return Ty::Unknown;
                            }
                        }
                        // 缺失方法报错仅限内建值类型（不可子类化,成员缺失即真错误）;
                        // 类类型交给家族查询（自身/任一子类声明即放行——动态派发）。
                        if ctx.strict_type(cls) && !ctx.script_family_member(cls, &name.name) {
                            ctx.err(format!("类型 '{}' 上没有方法 '{}'", cls, name.name), *span);
                        }
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
                // Dictionary 点访问是合法糖（dict.key ≡ dict["key"]，键运行时才知道），静默放过。
                if cls == "Dictionary" {
                    return Ty::Unknown;
                }
                // G5：命名枚举 State.KEY 返回 int 常量，不参与引擎成员链。
                if ctx.enum_entry(cls, &name.name) {
                    return Ty::Named("int".to_string());
                }
                // 类级枚举常量/枚举值/内建常量（Image.FORMAT_RGBA8、Vector2.ZERO、OK）。
                if ctx.engine.find_const(cls, &name.name).is_some()
                    || ctx.engine.find_enum_value(cls, &name.name).is_some()
                {
                    return Ty::Unknown;
                }
                // 引擎信号（CanvasItem.draw / Node.ready / Viewport.size_changed）。
                if ctx.engine.find_signal(cls, &name.name).is_some() {
                    return Ty::Named("Signal".into());
                }
                if let Some(p) = ctx.engine.find_property(cls, &name.name) {
                    return Ty::Named(p.ty.clone());
                }
                // G4.5：沿脚本 extends 基类的引擎继承链查属性。
                let base = ctx.script_base(cls);
                let base_prop = base.as_deref().and_then(|b| ctx.engine.find_property(b, &name.name));
                if let Some(p) = base_prop {
                    return Ty::Named(p.ty.clone());
                }
                let own = ctx.script_own_member(cls, &name.name);
                // 工程脚本信号（CanvasCamera.zoom_changed）→ Signal 类型。
                if own == Some(ScriptMember::Signal)
                    || base.as_deref().is_some_and(|b| ctx.engine.find_signal(b, &name.name).is_some())
                {
                    return Ty::Named("Signal".into());
                }
                if own == Some(ScriptMember::Property) || own == Some(ScriptMember::Const) {
                    // G4.6：声明类型回写；无注解回退 Unknown。
                    return ctx
                        .script_own_type(cls, &name.name)
                        .map(Ty::Named)
                        .unwrap_or(Ty::Unknown);
                }
                let base_method = base.as_deref().is_some_and(|b| ctx.engine.find_method(b, &name.name).is_some());
                if ctx.engine.find_method(cls, &name.name).is_some()
                    || own == Some(ScriptMember::Method)
                    || base_method
                {
                    return Ty::Unknown; // 方法引用（未调用）
                }
                // 工程链走到引擎根：Object 级成员（meta/脚本/信号）仍可见。
                if let Some(er) = ctx.project_engine_root(cls) {
                    if let Some(pr) = ctx.engine.find_property(&er, &name.name) {
                        return Ty::Named(pr.ty.clone());
                    }
                    if ctx.engine.find_method(&er, &name.name).is_some() {
                        return Ty::Unknown; // 方法引用（未调用）
                    }
                    if ctx.engine.find_signal(&er, &name.name).is_some() {
                        return Ty::Named("Signal".into());
                    }
                    if ctx.engine.find_const(&er, &name.name).is_some()
                        || ctx.engine.find_enum_value(&er, &name.name).is_some()
                    {
                        return Ty::Unknown;
                    }
                }
                // 缺失成员报错仅限内建值类型；类类型交给家族查询（动态派发放行）。
                if ctx.strict_type(cls) && !ctx.script_family_member(cls, &name.name) {
                    ctx.err(format!("类型 '{}' 上没有成员 '{}'", cls, name.name), *span);
                }
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
                // 裸标识符 key（{name = 1}）是字典键名字面量,不是变量引用。
                if !matches!(k, Expr::Ident(_)) {
                    infer_expr(k, ctx);
                }
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
        Expr::NodePath { .. } => {
            // $Node/%Unique 是节点引用（运行时返回 Node 子类），不是 NodePath 值;
            // 场景图类型未建模前推断 Unknown（NodePath 会连锁误报 .visible 等）。
            Ty::Unknown
        }
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
        let mut enums = HashMap::new();
        for stmt in &file.body {
            collect_member(stmt, &mut members, &mut enums);
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
