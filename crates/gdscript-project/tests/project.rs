//! 多文件 Godot 工程跨文件诊断验证。

use gdscript_project::Project;
use gdscript_syntax::semantic::{analyze_with, ProjectContext, ScriptMember};
use std::fs;
use std::path::Path;

fn engine_json() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../gdscript-engine/testdata/extension_api.json"
    ))
    .expect("engine dump")
}

/// 造一个最小工程：project.godot(autoload) + player.gd(class_name) + main.gd(引用二者)。
fn make_project(root: &Path) {
    fs::create_dir_all(root).unwrap();
    fs::write(
        root.join("project.godot"),
        "[autoload]\nGameManager=\"*res://game_manager.gd\"\n",
    )
    .unwrap();
    fs::write(
        root.join("player.gd"),
        "class_name Player\nextends Node2D\n\nvar hp := 10\n",
    )
    .unwrap();
    fs::write(root.join("game_manager.gd"), "extends Node\n\nvar score := 0\n").unwrap();
    fs::write(
        root.join("main.gd"),
        "extends Node2D\n\nfunc _ready():\n    var p := Player.new()\n    print(p.hp)\n    GameManager.score += 1\n",
    )
    .unwrap();
}

#[test]
fn cross_file_symbols_resolve() {
    let tmp = std::env::temp_dir().join("godotlsp_proj_test");
    let _ = fs::remove_dir_all(&tmp);
    make_project(&tmp);

    let proj = Project::open(&tmp, Some(&engine_json())).expect("open project");
    assert!(proj.class_name_script("Player").is_some(), "Player class_name registered");
    assert_eq!(proj.autoload_type("GameManager").as_deref(), Some("script:game_manager.gd"));

    // 分析 main.gd：Player / GameManager 都应跨文件解析，不报未声明。
    let main = proj.scripts.iter().find(|s| s.path == "main.gd").unwrap();
    let analysis = analyze_with(&main.ast, &proj.engine, Some(&proj));
    let undeclared: Vec<&str> = analysis
        .errors
        .iter()
        .filter(|e| e.message.contains("未声明"))
        .map(|e| e.message.as_str())
        .collect();
    assert!(undeclared.is_empty(), "unexpected undeclared: {:?}", undeclared);
}

#[test]
fn still_flags_unknown_cross_file_member() {
    // Player.hp 存在；但 Player 上不存在的成员仍应报错。
    let tmp = std::env::temp_dir().join("godotlsp_proj_test2");
    let _ = fs::remove_dir_all(&tmp);
    make_project(&tmp);
    fs::write(
        tmp.join("main.gd"),
        "extends Node2D\n\nfunc _ready():\n    var p := Player.new()\n    print(p.nope_member)\n",
    )
    .unwrap();

    let proj = Project::open(&tmp, Some(&engine_json())).expect("open project");
    let main = proj.scripts.iter().find(|s| s.path == "main.gd").unwrap();
    let analysis = analyze_with(&main.ast, &proj.engine, Some(&proj));
    let bad: Vec<&str> = analysis
        .errors
        .iter()
        .filter(|e| e.message.contains("nope_member"))
        .map(|e| e.message.as_str())
        .collect();
    assert_eq!(bad.len(), 1, "errors: {:?}", analysis.errors);
}

#[test]
fn g3_preload_const_binds_without_class_name() {
    // G3：无 class_name 的脚本经 const X = preload(...) 绑定，X 不报未声明，X.new() 不误报。
    let tmp = std::env::temp_dir().join("godotlsp_proj_g3");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    fs::write(tmp.join("project.godot"), "[application]\nconfig/name=\"g3\"\n").unwrap();
    fs::write(tmp.join("level.gd"), "extends Node\n\nvar x := 1\n").unwrap();
    fs::write(
        tmp.join("main.gd"),
        "extends Node2D\n\nconst Level = preload(\"res://level.gd\")\n\nfunc _ready():\n    var inst := Level.new()\n    print(inst)\n",
    )
    .unwrap();

    let proj = Project::open(&tmp, Some(&engine_json())).expect("open project");
    let ty = <Project as ProjectContext>::preload_type(&proj, "res://level.gd");
    assert_eq!(ty.as_deref(), Some("script:level.gd"), "synthesized script key");

    let main = proj.scripts.iter().find(|s| s.path == "main.gd").unwrap();
    let analysis = analyze_with(&main.ast, &proj.engine, Some(&proj));
    let undeclared: Vec<&str> = analysis
        .errors
        .iter()
        .filter(|e| e.message.contains("未声明") || e.message.contains("没有方法") || e.message.contains("没有成员"))
        .map(|e| e.message.as_str())
        .collect();
    assert!(undeclared.is_empty(), "unexpected errors: {:?}", undeclared);
}

#[test]
fn g3_load_const_binds_resource() {
    // G3：load(...) 恒绑定 Resource，路径未命中也不报未声明。
    let tmp = std::env::temp_dir().join("godotlsp_proj_g3load");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    fs::write(tmp.join("project.godot"), "[application]\nconfig/name=\"g3load\"\n").unwrap();
    fs::write(
        tmp.join("main.gd"),
        "extends Node2D\n\nconst Res = load(\"res://missing.tscn\")\n\nfunc _ready():\n    print(Res)\n",
    )
    .unwrap();

    let proj = Project::open(&tmp, Some(&engine_json())).expect("open project");
    let main = proj.scripts.iter().find(|s| s.path == "main.gd").unwrap();
    let analysis = analyze_with(&main.ast, &proj.engine, Some(&proj));
    let undeclared: Vec<&str> = analysis
        .errors
        .iter()
        .filter(|e| e.message.contains("未声明"))
        .map(|e| e.message.as_str())
        .collect();
    assert!(undeclared.is_empty(), "unexpected errors: {:?}", undeclared);
}

#[test]
fn g4_autoload_resolves_and_knows_own_members() {
    // G4：autoload 解析为脚本键；自身 var/func 命中不报，缺失成员静默（extends 基类未建模）。
    let tmp = std::env::temp_dir().join("godotlsp_proj_g4");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    fs::write(
        tmp.join("project.godot"),
        "[autoload]\nBus=\"*res://bus.gd\"\n",
    )
    .unwrap();
    fs::write(
        tmp.join("bus.gd"),
        "extends Node\n\nvar energy := 0\n\nfunc emit_pulse():\n    pass\n",
    )
    .unwrap();
    fs::write(
        tmp.join("main.gd"),
        "extends Node2D\n\nfunc _ready():\n    Bus.energy += 1\n    Bus.emit_pulse()\n    print(Bus.maybe_unknown_member)\n",
    )
    .unwrap();

    let proj = Project::open(&tmp, Some(&engine_json())).expect("open project");
    assert_eq!(proj.autoload_type("Bus").as_deref(), Some("script:bus.gd"));
    assert!(proj.script_type_exists("script:bus.gd"));
    assert_eq!(
        <Project as ProjectContext>::script_member_kind(&proj, "script:bus.gd", "energy"),
        Some(ScriptMember::Property)
    );
    assert_eq!(
        <Project as ProjectContext>::script_member_kind(&proj, "script:bus.gd", "emit_pulse"),
        Some(ScriptMember::Method)
    );

    let main = proj.scripts.iter().find(|s| s.path == "main.gd").unwrap();
    let analysis = analyze_with(&main.ast, &proj.engine, Some(&proj));
    let bad: Vec<&str> = analysis
        .errors
        .iter()
        .filter(|e| e.message.contains("未声明") || e.message.contains("没有成员") || e.message.contains("没有方法"))
        .map(|e| e.message.as_str())
        .collect();
    assert!(bad.is_empty(), "unexpected errors: {:?}", bad);
}

#[test]
fn g45_extends_base_members_resolve() {
    // G4.5：class_name 与 autoload 脚本访问 extends 基类引擎成员，不报未声明/没有成员。
    let tmp = std::env::temp_dir().join("godotlsp_proj_g45");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    fs::write(
        tmp.join("project.godot"),
        "[autoload]\nBus=\"*res://bus.gd\"\n",
    )
    .unwrap();
    fs::write(
        tmp.join("player.gd"),
        "class_name Player\nextends Node2D\n\nvar hp := 10\n",
    )
    .unwrap();
    fs::write(tmp.join("bus.gd"), "extends Node\n\nvar energy := 0\n").unwrap();
    fs::write(
        tmp.join("main.gd"),
        "extends Node2D\n\nfunc _ready():\n    var p := Player.new()\n    print(p.hp, p.position)\n    print(Bus.energy, Bus.name)\n",
    )
    .unwrap();

    let proj = Project::open(&tmp, Some(&engine_json())).expect("open project");
    assert_eq!(
        <Project as ProjectContext>::script_extends_base(&proj, "Player"),
        Some("Node2D".to_string())
    );
    assert_eq!(
        <Project as ProjectContext>::script_extends_base(&proj, "script:bus.gd"),
        Some("Node".to_string())
    );

    let main = proj.scripts.iter().find(|s| s.path == "main.gd").unwrap();
    let analysis = analyze_with(&main.ast, &proj.engine, Some(&proj));
    let bad: Vec<&str> = analysis
        .errors
        .iter()
        .filter(|e| e.message.contains("未声明") || e.message.contains("没有成员") || e.message.contains("没有方法"))
        .map(|e| e.message.as_str())
        .collect();
    assert!(bad.is_empty(), "unexpected errors: {:?}", bad);
}

#[test]
fn g46_script_member_declared_types_write_back() {
    // G4.6：var/const 注解类型与 func 返回类型回写；无注解 None；未知成员 None。
    let tmp = std::env::temp_dir().join("godotlsp_proj_g46");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    fs::write(tmp.join("project.godot"), "[application]\nconfig/name=\"g46\"\n").unwrap();
    fs::write(
        tmp.join("player.gd"),
        "class_name Player\nextends Node2D\n\nvar hp: int = 10\nconst MAX: int = 100\nfunc label() -> String:\n    return \"x\"\nfunc loose():\n    pass\n",
    )
    .unwrap();
    fs::write(
        tmp.join("main.gd"),
        "extends Node2D\n\nfunc _ready():\n    var p := Player.new()\n    var h: int = p.hp\n    var s: String = p.label()\n    print(h, s, Player.MAX)\n",
    )
    .unwrap();

    let proj = Project::open(&tmp, Some(&engine_json())).expect("open project");
    let ty = |m: &str| <Project as ProjectContext>::script_member_type(&proj, "Player", m);
    assert_eq!(ty("hp").as_deref(), Some("int"));
    assert_eq!(ty("MAX").as_deref(), Some("int"));
    assert_eq!(ty("label").as_deref(), Some("String"));
    assert_eq!(ty("loose"), None, "无返回注解不臆造类型");
    assert_eq!(ty("nope"), None);

    let main = proj.scripts.iter().find(|s| s.path == "main.gd").unwrap();
    let analysis = analyze_with(&main.ast, &proj.engine, Some(&proj));
    let bad: Vec<&str> = analysis
        .errors
        .iter()
        .filter(|e| e.message.contains("未声明") || e.message.contains("没有成员") || e.message.contains("没有方法"))
        .map(|e| e.message.as_str())
        .collect();
    assert!(bad.is_empty(), "unexpected errors: {:?}", bad);
}
