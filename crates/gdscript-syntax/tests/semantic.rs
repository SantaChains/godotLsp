//! 语义层测试：单文件符号表 + 名称解析 + 未声明标识符诊断。

use gdscript_engine::EngineApi;
use gdscript_syntax::{analyze, parse};

fn engine() -> EngineApi {
    let json = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../gdscript-engine/testdata/extension_api.json"
    ))
    .expect("read engine dump");
    EngineApi::from_json(&json).expect("parse")
}

fn analyze_src(src: &str) -> Vec<String> {
    let (file, _parse_errs) = parse(src);
    let a = analyze(&file, &engine());
    a.errors.into_iter().map(|e| e.message).collect()
}

#[test]
fn declares_and_resolves_locals() {
    let src = r#"
extends Node2D

var speed := 10.0

func _ready():
    var v := speed * 2.0
    print(v)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "unexpected errors: {:?}", errs);
}

#[test]
fn reports_undeclared_identifier() {
    let src = r#"
extends Node2D

func _ready():
    print(nope_thing)
"#;
    let errs = analyze_src(src);
    assert_eq!(errs.len(), 1, "errors: {:?}", errs);
    assert!(errs[0].contains("nope_thing"), "msg: {}", errs[0]);
}

#[test]
fn recognizes_engine_types_and_globals() {
    // Vector2、print、PI、Node2D 都应被识别，不报未声明。
    let src = r#"
extends Node2D

func _ready():
    var v := Vector2(1.0, 2.0)
    print(v, PI)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "unexpected errors: {:?}", errs);
}

#[test]
fn flags_unknown_member_on_known_type() {
    // v 推断为 Vector2，访问不存在的成员应报诊断。
    let src = r#"
extends Node2D

func _ready():
    var v := Vector2(1.0, 2.0)
    print(v.nonexistent_member)
"#;
    let errs = analyze_src(src);
    assert_eq!(errs.len(), 1, "errors: {:?}", errs);
    assert!(errs[0].contains("nonexistent_member"), "msg: {}", errs[0]);
}

#[test]
fn known_member_access_is_clean() {
    // Vector2.x 是真实属性，Node2D.get_position() 是真实方法，不应报错。
    let src = r#"
extends Node2D

func _ready():
    var v := Vector2(1.0, 2.0)
    print(v.x)
    var p := get_position()
    print(p)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "unexpected errors: {:?}", errs);
}
