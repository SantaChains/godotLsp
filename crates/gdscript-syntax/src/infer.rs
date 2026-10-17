//! 静默类型推断（P6 C5）：补全面专用的单层高置信推断，与 `semantic.rs`
//! 的诊断型分析解耦——推断失败一律返回 `InferTy::Unknown` 静默降级，
//! 与引擎「类型未知不出成员」行为对齐（completion-roadmap §P6 取舍）。
//!
//! 范围（只做单层，不做跨语句数据流、不做集合元素类型下推）：
//! - 字面量 → int/float/String/bool；
//! - 构造调用 `String(x)` / `Vector2(..)` → 引擎类/内建类名；
//! - 内建 utility 函数 `str(b)` → `EngineApi.utility_functions` 返回类型
//!   （+ 调用方外挂表，覆盖 `char()` 等不在 extension_api 的编译器内建）；
//! - `preload("res://…")` / `load("res://…")` → `InferTy::ScriptPath`
//!   （成员面解析由 godotlsp 层沿工程索引做）；
//! - `var`/`const` 声明类型：注解直取，无注解推初始值（收窄规则在
//!   调用方做——子类判断需要工程索引）；
//! - `return <expr>` 返回类型推断（同源推断器）。

use std::collections::HashMap;

use gdscript_engine::EngineApi;

use crate::ast::{Expr, File, FuncDecl, Literal, Stmt, VarDecl};

/// 推断结果：类名（引擎类/内建类型）或 preload 脚本引用。
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum InferTy {
    /// 引擎类 / 内建类型名（`String` / `int` / `Node` / `Vector2` …）。
    Class(String),
    /// preload/load 的脚本 `res://` 路径（godotlsp 层再沿工程索引展开成员）。
    ScriptPath(String),
    #[default]
    Unknown,
}

impl InferTy {
    /// 类名（ScriptPath/Unknown → None）。display/detail 层用。
    pub fn class_name(&self) -> Option<&str> {
        match self {
            InferTy::Class(n) => Some(n),
            _ => None,
        }
    }
}

/// 外挂内建函数返回类型表（`char` → `String` 等 hoverdata 内建函数），
/// 推断器优先查 `EngineApi.utility_functions`，miss 后查此表。
pub type FnRetTable = HashMap<String, String>;

/// 从 hoverdata 风格签名（`"String char(code: int)"`）提取返回类型（首词）。
/// 非该格式（首词不是大写开头类型名）返回 None。
pub fn ret_from_signature(sig: &str) -> Option<String> {
    let first = sig.split_whitespace().next()?;
    // 防御：签名形如 `name(...)`（无返回类型）时首词是函数名，不算类型。
    if first.starts_with(|c: char| c.is_ascii_uppercase()) && first.contains(|c: char| c.is_ascii_alphabetic()) {
        Some(first.to_string())
    } else {
        None
    }
}

/// 单层表达式类型推断。`names` 解析标识符类型（preload 常量 / 内部类名
/// 等——由调用方提供作用域表，推断器自身无状态）。复杂形态（成员方法
/// 调用、二元运算等）一律 Unknown——补全面宁缺毋滥。
pub fn infer_expr_ty(
    src: &str,
    e: &Expr,
    engine: &EngineApi,
    fn_ret: &FnRetTable,
    names: &dyn Fn(&str) -> Option<InferTy>,
) -> InferTy {
    match e {
        Expr::Literal(l) => InferTy::Class(match l {
            Literal::Int(_) => "int".into(),
            Literal::Float(_) => "float".into(),
            Literal::Str(_) => "String".into(),
            Literal::Bool(_) => "bool".into(),
            Literal::Null => return InferTy::Unknown,
        }),
        Expr::Call { callee, args, .. } => infer_call_ty(src, callee, args, engine, fn_ret, names),
        Expr::Array(..) => InferTy::Class("Array".into()),
        Expr::Dict(..) => InferTy::Class("Dictionary".into()),
        Expr::Lambda(..) => InferTy::Class("Callable".into()),
        Expr::Generic(base, _, _) => infer_expr_ty(src, base, engine, fn_ret, names),
        Expr::NodePath { raw, .. } if raw.starts_with('$') => {
            // $Node/%Unique 是节点引用（运行时 Node 子类），具体类型未建模。
            InferTy::Unknown
        }
        Expr::NodePath { .. } => InferTy::Class("NodePath".into()),
        Expr::Ident(id) => names(&id.name).unwrap_or(InferTy::Unknown),
        _ => InferTy::Unknown,
    }
}

fn infer_call_ty(
    src: &str,
    callee: &Expr,
    args: &[crate::ast::Arg],
    engine: &EngineApi,
    fn_ret: &FnRetTable,
    names: &dyn Fn(&str) -> Option<InferTy>,
) -> InferTy {
    match callee {
        Expr::Ident(id) => {
            let name = id.name.as_str();
            // preload/load("res://…") → 脚本引用。
            if matches!(name, "preload" | "load")
                && let Some(Expr::Literal(Literal::Str(p))) = args.first().map(|a| &a.value)
            {
                return InferTy::ScriptPath(p.clone());
            }
            // 构造调用：名字是引擎类/内建类型 → 该类型。
            if engine.class_by_name(name).is_some() || engine.builtin_by_name(name).is_some() {
                return InferTy::Class(name.to_string());
            }
            // utility 函数返回类型（extension_api），miss 后查外挂表。
            if let Some(f) = engine.utility_functions.iter().find(|f| f.name == name)
                && !f.return_type.is_empty()
                && f.return_type != "void"
            {
                return InferTy::Class(f.return_type.clone());
            }
            if let Some(ret) = fn_ret.get(name) {
                return InferTy::Class(ret.clone());
            }
            // 外部名字表（P8：self 方法名 → 返回类型，供 `get_tree().m()` 链）。
            if let Some(t) = names(name) {
                return t;
            }
            InferTy::Unknown
        }
        // 方法调用：`A.new()` 构造返回自身类型；一般 `recv.m(...)` 在接收者
        // 类型已知（引擎/内建 Class）时查引擎方法表返回类型（仅返回引擎类/
        // 内建名才推断，变体/void/脚本类型静默降级——单层高置信纪律）。
        Expr::Attribute { target, name, span: _ } => {
            if name.name == "new" {
                if let Expr::Ident(id) = target.as_ref() {
                    return names(&id.name).unwrap_or_else(|| {
                        if engine.class_by_name(&id.name).is_some()
                            || engine.builtin_by_name(&id.name).is_some()
                        {
                            InferTy::Class(id.name.clone())
                        } else {
                            InferTy::Unknown
                        }
                    });
                }
                return InferTy::Unknown;
            }
            if let InferTy::Class(c) = infer_expr_ty(src, target, engine, fn_ret, names)
                && let Some(m) = engine.find_method(&c, &name.name)
                && (engine.class_by_name(&m.return_type).is_some()
                    || engine.builtin_by_name(&m.return_type).is_some())
            {
                return InferTy::Class(m.return_type.clone());
            }
            InferTy::Unknown
        }
        _ => {
            let _ = src;
            InferTy::Unknown
        }
    }
}

/// `preload` / `load` 调用的 res:// 实参（工具函数：常量表收集用）。
pub fn preload_arg(e: &Expr) -> Option<&str> {
    if let Expr::Call { callee, args, .. } = e
        && let Expr::Ident(id) = callee.as_ref()
        && matches!(id.name.as_str(), "preload" | "load")
        && let Some(Expr::Literal(Literal::Str(p))) = args.first().map(|a| &a.value)
    {
        return Some(p);
    }
    None
}

/// 全文件 preload 常量表：顶层 + 类级 `const A := preload("res://…")` /
/// `const A = preload(…)` → `A → res://…`。同名近层优先（先到先得）。
pub fn preload_consts(file: &File) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut push = |name: &str, path: &str, out: &mut Vec<(String, String)>| {
        if seen.insert(name.to_string()) {
            out.push((name.to_string(), path.to_string()));
        }
    };
    for st in &file.body {
        match st {
            Stmt::Const(c) => {
                if let Some(p) = preload_arg(&c.value) {
                    push(&c.name.name, p, &mut out);
                }
            }
            Stmt::Class(c) => {
                for st in &c.body {
                    if let Stmt::Const(cc) = st
                        && let Some(p) = preload_arg(&cc.value)
                    {
                        push(&cc.name.name, p, &mut out);
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// var 声明类型推断：注解直取（Ident/构造式类型文本），无注解推初始值。
/// 返回 (注解类型, 初始值类型)——收窄判定由调用方做（需工程索引子类链）。
pub fn var_decl_types(
    src: &str,
    v: &VarDecl,
    engine: &EngineApi,
    fn_ret: &FnRetTable,
    names: &dyn Fn(&str) -> Option<InferTy>,
) -> (Option<InferTy>, Option<InferTy>) {
    let ann = v.ty.as_ref().and_then(|t| type_expr_ty(t, fn_ret));
    let val = v.value.as_ref().map(|e| infer_expr_ty(src, e, engine, fn_ret, names));
    (ann, val)
}

/// 类型注解表达式 → 类型（`A` / `Array[int]` 的 base）。公开供 LSP 层
/// 解析形参/返回注解（var_decl_types 内部同源）。
pub fn type_annotation_ty(e: &Expr, fn_ret: &FnRetTable) -> Option<InferTy> {
    type_expr_ty(e, fn_ret)
}

/// 类型注解表达式 → 类型名（`A` / `Array[int]` 的 base / `Node`）。
fn type_expr_ty(e: &Expr, fn_ret: &FnRetTable) -> Option<InferTy> {
    match e {
        Expr::Ident(id) => Some(InferTy::Class(id.name.clone())),
        Expr::Generic(base, _, _) => type_expr_ty(base, fn_ret),
        _ => {
            let _ = fn_ret;
            None
        }
    }
}

/// 函数返回类型：注解直取；无注解收集 `return <expr>` 推断类型——
/// 全部一致 → 该类型；无 return / 不一致 / 含 Unknown → None。
/// 注：推断无作用域表（return 位局部名不解析——单层保守）。
pub fn func_return_inferred(
    src: &str,
    f: &FuncDecl,
    engine: &EngineApi,
    fn_ret: &FnRetTable,
) -> Option<String> {
    if let Some(rt) = &f.ret_ty {
        return type_expr_ty(rt, fn_ret)?.class_name().map(str::to_string);
    }
    let names = |_: &str| -> Option<InferTy> { None };
    let mut found: Option<String> = None;
    let mut visit = |e: &Expr, found: &mut Option<String>| -> bool {
        let ty = infer_expr_ty(src, e, engine, fn_ret, &names);
        let Some(name) = ty.class_name() else { return false };
        match found {
            None => {
                *found = Some(name.to_string());
                true
            }
            Some(prev) => prev == name,
        }
    };
    // 递归收集嵌套块（if/for/while/match 分支）内全部 return。
    fn collect_returns<'a>(
        blk: &'a crate::ast::Block,
        visit: &mut impl FnMut(&'a Expr, &mut Option<String>) -> bool,
        found: &mut Option<String>,
    ) -> bool {
        for st in &blk.stmts {
            let ok = match st {
                Stmt::Return { value: Some(e), .. } => visit(e, found),
                Stmt::If { branches, else_block, .. } => {
                    branches.iter().all(|(_, b)| collect_returns(b, visit, found))
                        && else_block.iter().all(|b| collect_returns(b, visit, found))
                }
                Stmt::For { body, .. } | Stmt::While { body, .. } => collect_returns(body, visit, found),
                Stmt::Match { arms, .. } => arms.iter().all(|a| collect_returns(&a.body, visit, found)),
                _ => true,
            };
            if !ok {
                return false;
            }
        }
        true
    }
    if collect_returns(&f.body, &mut visit, &mut found) {
        found
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;
    use gdscript_engine::EngineApi;

    fn engine() -> EngineApi {
        let json = include_str!("../../gdscript-engine/testdata/extension_api.json");
        EngineApi::from_json(json).expect("解析引擎模型")
    }

    fn table() -> FnRetTable {
        FnRetTable::from([("char".to_string(), "String".to_string())])
    }

    fn last_local_var(file: &File) -> &VarDecl {
        file.body
            .iter()
            .find_map(|st| match st {
                Stmt::Func(f) => f.body.stmts.iter().find_map(|s| match s {
                    Stmt::Var(v) => Some(v),
                    _ => None,
                }),
                _ => None,
            })
            .expect("func 内有 var")
    }

    fn infer_var(src: &str) -> InferTy {
        let (file, _) = parse(src);
        let v = last_local_var(&file);
        let (_, val) = var_decl_types(src, v, &engine(), &table(), &|_| None);
        val.expect("有初始值")
    }

    #[test]
    fn literal_types() {
        assert_eq!(infer_var("func f():\n\tvar x = 1"), InferTy::Class("int".into()));
        assert_eq!(infer_var("func f():\n\tvar x = 1.5"), InferTy::Class("float".into()));
        assert_eq!(infer_var("func f():\n\tvar x = \"s\""), InferTy::Class("String".into()));
        assert_eq!(infer_var("func f():\n\tvar x = true"), InferTy::Class("bool".into()));
    }

    #[test]
    fn constructor_call() {
        assert_eq!(infer_var("func f():\n\tvar t = String(s)"), InferTy::Class("String".into()));
        assert_eq!(infer_var("func f():\n\tvar v = Vector2(1, 2)"), InferTy::Class("Vector2".into()));
    }

    #[test]
    fn script_class_new() {
        // A 为 preload 常量名（调用方 names 表）→ A.new() → ScriptPath。
        let src = "const A := preload(\"res://a.gd\")\nfunc f():\n\tvar t = A.new()";
        let (file, _) = parse(src);
        let v = last_local_var(&file);
        let names = |n: &str| -> Option<InferTy> {
            preload_consts(&file)
                .into_iter()
                .find(|(k, _)| k == n)
                .map(|(_, p)| InferTy::ScriptPath(p))
        };
        let (_, val) = var_decl_types(src, v, &engine(), &table(), &names);
        assert_eq!(val, Some(InferTy::ScriptPath("res://a.gd".into())));
    }

    #[test]
    fn utility_and_builtin_fn() {
        // str() 在 extension_api utility_functions。
        assert_eq!(infer_var("func f(b):\n\tvar x = str(b)"), InferTy::Class("String".into()));
        // char() 不在 extension_api，走外挂表。
        assert_eq!(infer_var("func f(i):\n\tvar t = char(i)"), InferTy::Class("String".into()));
    }

    #[test]
    fn preload_and_load() {
        let src = "func f():\n\tvar t = preload(\"res://a.gd\")";
        assert_eq!(infer_var(src), InferTy::ScriptPath("res://a.gd".into()));
    }

    #[test]
    fn array_dict_callable() {
        assert_eq!(infer_var("func f():\n\tvar x = []"), InferTy::Class("Array".into()));
        assert_eq!(infer_var("func f():\n\tvar x = {}"), InferTy::Class("Dictionary".into()));
        assert_eq!(infer_var("func f():\n\tvar x = func(): pass"), InferTy::Class("Callable".into()));
    }

    #[test]
    fn preload_consts_table() {
        let src = "const A := preload(\"res://completion/class_a.notest.gd\")\nclass C:\n\tconst B = load(\"res://b.gd\")";
        let (file, _) = parse(src);
        let m = preload_consts(&file);
        assert!(m.contains(&("A".into(), "res://completion/class_a.notest.gd".into())));
        assert!(m.contains(&("B".into(), "res://b.gd".into())));
    }

    #[test]
    fn func_return_infer() {
        let src = "class T:\n\tfunc to_str(b: int):\n\t\treturn str(b)";
        let (file, _) = parse(src);
        let f = file.body.iter().find_map(|st| match st {
            Stmt::Class(c) => c.body.iter().find_map(|s| match s {
                Stmt::Func(f) => Some(f),
                _ => None,
            }),
            _ => None,
        });
        let f = f.unwrap();
        assert_eq!(func_return_inferred(src, f, &engine(), &table()), Some("String".into()));
    }

    #[test]
    fn func_return_annotation_wins() {
        let src = "func f() -> int:\n\treturn \"s\"";
        let (file, _) = parse(src);
        let f = match &file.body[0] {
            Stmt::Func(f) => f,
            _ => unreachable!(),
        };
        assert_eq!(func_return_inferred(src, f, &engine(), &table()), Some("int".into()));
    }

    #[test]
    fn func_return_mixed_is_none() {
        let src = "func f():\n\tif true:\n\t\treturn 1\n\treturn \"s\"";
        let (file, _) = parse(src);
        let f = match &file.body[0] {
            Stmt::Func(f) => f,
            _ => unreachable!(),
        };
        assert_eq!(func_return_inferred(src, f, &engine(), &table()), None);
    }

    #[test]
    fn ret_from_signature_parsing() {
        assert_eq!(ret_from_signature("String char(code: int)"), Some("String".into()));
        assert_eq!(ret_from_signature("char(code: int)"), None);
    }

    // ───────────────────── P8：方法调用链返回类型 ─────────────────────

    #[test]
    fn method_chain_return_type() {
        let e = engine();
        // get_tree().create_tween()：names 提供 self 方法 get_tree → SceneTree，
        // 再沿引擎方法表查 create_tween → Tween。
        let names = |n: &str| -> Option<InferTy> {
            (n == "get_tree").then(|| InferTy::Class("SceneTree".into()))
        };
        let (ast, _) = parse("var t := get_tree().create_tween()");
        let val = match ast.body.first() {
            Some(crate::ast::Stmt::Var(v)) => v.value.as_ref().expect("有初始值"),
            _ => panic!("应为 var 声明"),
        };
        let ty = infer_expr_ty("", val, &e, &table(), &names);
        assert_eq!(ty, InferTy::Class("Tween".into()), "方法链推断");
    }

    #[test]
    fn method_chain_variant_return_degrades() {
        let e = engine();
        // 返回 Variant/void 的方法不推断（单层高置信纪律）。
        let names = |n: &str| -> Option<InferTy> {
            (n == "get_tree").then(|| InferTy::Class("SceneTree".into()))
        };
        let (ast, _) = parse("var t := get_tree().create_timer(1.0)");
        let _ = ast;
        // create_timer 返回 Timer（引擎类）——反例用不存在的方法名。
        let (ast2, _) = parse("var t := get_tree().no_such_method()");
        let val = match ast2.body.first() {
            Some(crate::ast::Stmt::Var(v)) => v.value.as_ref().expect("有初始值"),
            _ => panic!("应为 var 声明"),
        };
        assert_eq!(infer_expr_ty("", val, &e, &table(), &names), InferTy::Unknown);
    }
}
