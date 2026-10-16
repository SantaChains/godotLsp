//! 多文件 Godot 工程跨文件诊断验证。

use gdscript_project::Project;
use gdscript_syntax::semantic::{analyze_with, ProjectContext};
use gdscript_syntax::parse;
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
    assert_eq!(proj.autoload_type("GameManager").as_deref(), Some("res://game_manager.gd"));

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
