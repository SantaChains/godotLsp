//! 单文件检查工具（R2）：解析 + 名称解析 + 类型推断，打印所有诊断。
//!
//! 用法（在 crates/gdscript-syntax 下）：
//!   cargo run --example check -- path/to/script.gd

use gdscript_engine::EngineApi;
use gdscript_syntax::{analyze, parse};

fn main() {
    let path = std::env::args().nth(1).expect("用法: cargo run --example check -- <file.gd>");
    let raw = std::fs::read_to_string(&path).expect("读取文件失败");
    // 对齐 corpus 约定：剥离 Godot 上游光标标记 U+27A1。
    let src: String = raw.chars().filter(|&c| c != '\u{27a1}').collect();

    let engine_json = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../gdscript-engine/testdata/extension_api.json"
    ))
    .expect("读取引擎模型失败");
    let engine = EngineApi::from_json(&engine_json).expect("解析引擎模型失败");

    let (file, parse_errs) = parse(&src);
    let analysis = analyze(&file, &engine);

    let mut n = 0;
    for e in &parse_errs {
        n += 1;
        println!("[parse]  {}", e.message);
    }
    for e in &analysis.errors {
        n += 1;
        println!("[sem]    {}", e.message);
    }
    if n == 0 {
        println!("OK: 无诊断");
    } else {
        println!("共 {} 条诊断", n);
    }
}
