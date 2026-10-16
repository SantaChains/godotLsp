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

#[test]
fn g1_recognizes_gdscript_builtins() {
    // G1：@GDScript 语言内建（非 @GlobalScope 工具函数）不得误报未声明。
    let src = r#"
extends Node2D

func _ready():
    var arr := range(3)
    var c := char(65)
    var o := ord("A")
    assert(true)
    var inst := preload("res://x.gd")
    var res := load("res://y.tscn")
    var ok := is_instance_of(self, Node)
    var exists := type_exists("Node")
    var stack := get_stack()
    print_stack()
    print_debug("debug")
    var d := dict_to_inst({})
    var dd := inst_to_dict(self)
    var col := Color8(255, 0, 0)
    var n := convert(1, TYPE_INT)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "unexpected errors: {:?}", errs);
}

#[test]
fn g2_recognizes_global_enum_values() {
    // G2：裸写全局枚举值（KEY_*/ERR_*/MOUSE_BUTTON_*）不得误报未声明。
    // 注意 Error 枚举前两项为 OK/FAILED，其余为 ERR_ 前缀，不存在 ERR_OK。
    let src = r#"
extends Node2D

func _ready():
    var k := KEY_ENTER
    var e := OK
    var b := MOUSE_BUTTON_LEFT
    var s := KEY_SPACE
    var f := ERR_FILE_NOT_FOUND
    print(k, e, b, s, f)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "unexpected errors: {:?}", errs);
}

#[test]
fn g2_unknown_identifier_still_reported() {
    // 边界：枚举值识别不得放过真正未声明的名字。
    let src = r#"
extends Node2D

func _ready():
    print(NOT_A_REAL_CONSTANT)
"#;
    let errs = analyze_src(src);
    assert_eq!(errs.len(), 1, "errors: {:?}", errs);
    assert!(errs[0].contains("NOT_A_REAL_CONSTANT"), "msg: {}", errs[0]);
}

#[test]
fn g5_named_enum_and_signal_ok() {
    // G5：命名枚举 State.KEY 合法；信号 emit/connect 合法；内部类实例化静默。
    let src = r#"
extends Node2D

enum State { IDLE, RUNNING }
signal hp_changed(value)

class Inner:
    var label := "x"

func _ready():
    var s := State.IDLE
    if s == State.RUNNING:
        pass
    hp_changed.emit(10)
    hp_changed.connect(_on_hp)
    var i := Inner.new()
    print(i.label, s, State.keys())

func _on_hp(v):
    pass
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "unexpected errors: {:?}", errs);
}

#[test]
fn g5_anonymous_enum_bare_constant_ok() {
    // 官方规则：匿名枚举（无名字）条目即文件级 int 常量，可裸写。
    let src = r#"
extends Node2D

enum { TILE_BRICK, TILE_FLOOR, TILE_SPIKE }

func _ready():
    var t := TILE_BRICK
    print(TILE_FLOOR, TILE_SPIKE, t)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "unexpected errors: {:?}", errs);
}

#[test]
fn g5_bare_named_enum_key_still_unknown() {
    // 边界：命名枚举的键不是全局常量，裸写 IDLE 仍应报未声明。
    let src = r#"
extends Node2D

enum State { IDLE, RUNNING }

func _ready():
    print(IDLE)
"#;
    let errs = analyze_src(src);
    assert_eq!(errs.len(), 1, "errors: {:?}", errs);
    assert!(errs[0].contains("IDLE"), "msg: {}", errs[0]);
}

// ───────────────────────── self 继承链 / 类常量 / Dictionary 点访问 ─────────────────────────

#[test]
fn self_chain_bare_engine_member_ok() {
    // extends Node2D → CanvasItem：queue_redraw/position 裸写沿链解析，不报未声明。
    let src = r#"
extends Node2D

func _process(delta):
    queue_redraw()
    position += Vector2.ONE * delta
    print(scale)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn self_chain_reserved_word_param_delta_ok() {
    // delta 是 _process 形参（局部），position 走 self 链——两者都不报。
    let src = r#"
extends Node2D

func _process(delta: float) -> void:
    rotation += delta
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn self_chain_unknown_still_reported() {
    // self 链上确实不存在的名字仍要报（不能矫枉过正变哑巴）。
    let src = r#"
extends Node2D

func _ready():
    print(not_a_real_member_xyz)
"#;
    let errs = analyze_src(src);
    assert!(errs.iter().any(|e| e.contains("not_a_real_member_xyz")), "errors: {:?}", errs);
}

#[test]
fn engine_class_enum_constant_ok() {
    // 类级枚举常量 Image.FORMAT_RGBA8：find_const 覆盖，不报没有成员。
    let src = r#"
extends Node

func f(img: Image) -> void:
    img.convert(Image.FORMAT_RGBA8)
    print(Image.MAX_WIDTH, OK, ERR_CANT_OPEN)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn builtin_constant_ok() {
    // 内建类常量 Vector2.ZERO 沿 find_const 内建兜底。
    let src = r#"
extends Node2D

func _ready():
    position = Vector2.ZERO
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn dictionary_dot_access_ok() {
    // dict.key 是合法糖（键运行时才知），未知成员静默放过。
    let src = r#"
extends Node

func _ready():
    var d := {"name": "a"}
    print(d.name, d.anything_else)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn engine_signal_ok() {
    // 引擎信号沿继承链解析：ready/draw 都是 signal；裸方法引用作 Callable。
    let src = r#"
extends Control

func _on_ready() -> void:
    pass

func _redraw() -> void:
    pass

func _ready():
    ready.connect(_on_ready)
    draw.connect(_redraw)
    assert(ready is Signal)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn engine_signal_unknown_still_reported() {
    // 引擎类上不存在的信号名仍要报。
    let src = r#"
extends Node

func _ready():
    not_a_real_signal_xyz.connect(_f)
"#;
    let errs = analyze_src(src);
    assert!(errs.iter().any(|e| e.contains("not_a_real_signal_xyz")), "errors: {:?}", errs);
}

#[test]
fn node_path_expr_is_node_reference_not_nodepath() {
    // $Node 是节点引用,连锁 .visible 不误报;%"Unique" 同理。
    let src = r#"
extends Node2D

func _ready():
    $Child.visible = false
    %"UniqueName".queue_redraw()
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn dict_bare_ident_key_not_identifier() {
    // {source_index = 1} 的裸 key 是字典键名字面量,不查符号表。
    let src = r#"
extends Node

func _ready():
    var d := {source_index = 1, name = "x"}
    print(d)
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}

#[test]
fn scalar_receiver_silent() {
    // float/int 的方法形式（x.floor()）不在模型,标量接收者静默。
    let src = r#"
extends Node2D

func _ready():
    var x := 1.5
    print(x.floor(), x.ceil(), (3).abs())
"#;
    let errs = analyze_src(src);
    assert!(errs.is_empty(), "errors: {:?}", errs);
}
