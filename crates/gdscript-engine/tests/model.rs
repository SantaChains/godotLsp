//! gdscript-engine 模型摄取验证：加载 pinned Godot 4.7 extension_api.json，
//! 校验类继承链、方法/属性沿链查询、builtin 方法、全局函数。

use gdscript_engine::EngineApi;

fn load() -> EngineApi {
    let json = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/testdata/extension_api.json"
    ))
    .expect("read fixture");
    EngineApi::from_json(&json).expect("parse dump")
}

#[test]
fn loads_version_and_counts() {
    let api = load();
    assert_eq!(api.version, "4.7.0");
    assert!(api.classes.len() > 1000, "classes = {}", api.classes.len());
    assert!(api.builtins.len() >= 30, "builtins = {}", api.builtins.len());
}

#[test]
fn inheritance_chain_resolves() {
    let api = load();
    // Node2D -> CanvasItem -> Node -> Object
    let chain = api.ancestor_chain("Node2D");
    assert_eq!(
        chain,
        vec!["Node2D", "CanvasItem", "Node", "Object"],
        "chain = {:?}",
        chain
    );
}

#[test]
fn finds_method_and_property_up_the_chain() {
    let api = load();
    // get_position 定义在 Node2D 自身。
    let m = api.find_method("Node2D", "get_position").expect("get_position");
    assert_eq!(m.return_type, "Vector2");
    // get_child 定义在 Node，沿链可查到。
    let gc = api.find_method("Node2D", "get_child").expect("get_child");
    assert!(!gc.arguments.is_empty());
    // position 属性在 Node2D 自身。
    let p = api.find_property("Node2D", "position").expect("position");
    assert_eq!(p.ty, "Vector2");
    // name 属性在 Object 上，沿链可查到。
    let name = api.find_property("Node2D", "name").expect("name on Object");
    assert_eq!(name.ty, "StringName");
}

#[test]
fn builtin_methods_resolve() {
    let api = load();
    // Vector2.angle() -> float
    let a = api
        .find_method("Vector2", "angle")
        .expect("Vector2.angle");
    assert_eq!(a.return_type, "float");
    // Vector2.direction_to(Vector2) -> Vector2
    let d = api
        .find_method("Vector2", "direction_to")
        .expect("Vector2.direction_to");
    assert_eq!(d.return_type, "Vector2");
    assert_eq!(d.arguments[0].ty, "Vector2");
}

#[test]
fn global_utility_and_singletons() {
    let api = load();
    let sin = api.utility_function("sin").expect("sin");
    assert_eq!(sin.return_type, "float");
    // Engine 是单例。
    assert_eq!(api.singleton_type("Engine"), Some("Engine"));
}

// ───────────────────────── C6.3 虚方法模型化（P7） ─────────────────────────

#[test]
fn is_virtual_count_pinned() {
    let api = load();
    let total: usize = api.classes.values().map(|c| c.methods.iter().filter(|m| m.is_virtual).count()).sum();
    assert_eq!(total, 1437, "is_virtual 计数锚定（extension_api 4.7.0）");
    // 抽查：Node._process 是虚方法；get_child 不是。
    let p = api.find_method("Node", "_process").expect("_process");
    assert!(p.is_virtual);
    let gc = api.find_method("Node", "get_child").expect("get_child");
    assert!(!gc.is_virtual);
}

#[test]
fn chain_virtuals_near_wins() {
    let api = load();
    let chain = api.chain_virtuals("Node");
    // Node 链含 _process/_ready/_input（is_virtual 模型源）。
    let p = chain.iter().find(|m| m.name == "_process").expect("Node._process");
    assert_eq!(p.return_type, "void");
    assert_eq!(p.arguments.len(), 1);
    // 同名先到先得：Node._process(delta: float) -> void，而非 MainLoop._process -> bool。
    assert_eq!(p.return_type, "void", "Node 链上 _process 应取 Node 的（void），非 MainLoop 的（bool）");
    // RefCounted/Object 核心链在 extension_api 里零 is_virtual——核心
    // 虚方法全靠 object_core_virtuals() 手表补（数据源分裂的事实锚点）。
    let refc = api.chain_virtuals("RefCounted");
    assert!(refc.is_empty(), "RefCounted 链不应有 is_virtual 项: {refc:?}");
    // _process/_draw 同名多类分叉收口：CanvasItem 链 _draw 取 CanvasItem 的（零参 void）。
    let d = api.chain_virtuals("CanvasItem").into_iter().find(|m| m.name == "_draw").expect("_draw");
    assert!(d.arguments.is_empty() && d.return_type == "void");
}

#[test]
fn object_core_virtuals_pinned() {
    let core = gdscript_engine::EngineApi::object_core_virtuals();
    assert_eq!(core.len(), 12, "object.cpp BIND_OBJ_CORE_METHOD 全集（含 _iter_* 3 条）");
    let get = core.iter().find(|v| v.name == "_get").expect("_get");
    assert_eq!(get.arguments, &[("property", "StringName")]);
    assert_eq!(get.return_type, Some("Variant"));
    let tostr = core.iter().find(|v| v.name == "_to_string").expect("_to_string");
    assert_eq!(tostr.return_type, Some("String"));
    // extension_api 里不存在这些名字（数据源分裂的事实锚点）。
    let api = load();
    for v in core {
        assert!(
            api.find_method("Object", v.name).is_none(),
            "{} 不应出现在 ClassDB methods 表",
            v.name
        );
    }
}

// ───────────────────────── C6.5a builtin 构造/运算符（P9） ─────────────────────────

#[test]
fn builtin_ctors_ops_pinned() {
    let api = load();
    // 38/38 内建类全有构造与运算符数据。
    assert_eq!(api.builtins.len(), 38, "builtin 类计数锚定");
    for (name, b) in &api.builtins {
        assert!(!b.constructors.is_empty(), "{name} 应有构造重载");
        assert!(!b.operators.is_empty(), "{name} 应有运算符");
    }
    // 计数锚定：Vector2 4 构造；int 5 构造 64 运算符；Array 13 构造 12 运算符。
    assert_eq!(api.builtin_constructors("Vector2").unwrap().len(), 4);
    assert_eq!(api.builtin_constructors("int").unwrap().len(), 5);
    assert_eq!(api.builtin_operators("int").unwrap().len(), 64);
    assert_eq!(api.builtin_constructors("Array").unwrap().len(), 13);
    assert_eq!(api.builtin_operators("Array").unwrap().len(), 12);
    // 抽查：Vector2(x: float, y: float) 重载；默认构造零参。
    let v2 = api.builtin_constructors("Vector2").unwrap();
    let xy = v2.iter().find(|c| c.len() == 2).expect("Vector2(x, y) 重载");
    assert_eq!(&xy[0].name, "x");
    assert_eq!(&xy[0].ty, "float");
    assert!(v2.iter().any(|c| c.is_empty()), "默认构造零参");
    // 抽查：int 运算符 `*`(right: int) -> int；一元运算符省略 right_type（空串）。
    let mul = api
        .builtin_operators("int")
        .unwrap()
        .iter()
        .find(|o| o.name == "*" && o.right_type == "int")
        .expect("int * int");
    assert_eq!(mul.return_type, "int");
    // JSON 是全超集：一元运算符在（right_type 空串），且含语言级 and/in/not。
    let ops = api.builtin_operators("int").unwrap();
    let unary_plus = ops.iter().find(|o| o.name == "unary+").expect("JSON 应含 unary+");
    assert_eq!(unary_plus.right_type, "", "一元运算符 right_type 应为空串");
    assert_eq!(unary_plus.return_type, "int");
    assert!(ops.iter().any(|o| o.name == "in"), "JSON 应含语言级 in");
    assert!(ops.iter().any(|o| o.name == "not"), "JSON 应含语言级 not");
}

#[test]
fn builtin_keyed_indexing_parsed() {
    let api = load();
    // C6.5d 只解析：is_keyed 仅 Dictionary；indexing_return_type 24 类。
    assert!(api.builtins["Dictionary"].is_keyed);
    assert_eq!(api.builtins["Dictionary"].indexing_return_type.as_deref(), Some("Variant"));
    assert_eq!(api.builtins["Array"].indexing_return_type.as_deref(), Some("Variant"));
    assert_eq!(api.builtins["Vector2"].indexing_return_type.as_deref(), Some("float"));
    assert!(!api.builtins["int"].is_keyed);
    let n_indexing = api.builtins.values().filter(|b| b.indexing_return_type.is_some()).count();
    assert_eq!(n_indexing, 24, "indexing_return_type 类计数锚定");
}
