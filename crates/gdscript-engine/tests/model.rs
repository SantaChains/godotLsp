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
