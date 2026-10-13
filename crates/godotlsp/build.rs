//! 构建期从 Godot 官方文档语料派生全局面清单(`SINGLETON_NAMES` / `UTILITY_NAMES` /
//! `GLOBAL_ENUM_NAMES`),写入 `OUT_DIR/gdscript_docs_gen.rs`,由 `gdscript_docs.rs`
//! 通过 `include!` 引入。
//!
//! 设计动机(回应"内嵌模型而非纯代码通信"的改造要求):godotlsp 的全局补全/hover 数据层,
//! 此前是**手维护的语料快照副本**,会随内嵌引擎模型(`gdscript-api` `bundled()`)漂移,
//! 且 `gdscript-api` 不暴露 utility / singleton / global-enum 的遍历接口。
//! 本 build 脚本在**编译期**直接从与模型同源的官方文档语料(`godot-docs` 子模块)重新派生
//! 这三张表,使它们永远是语料的最新投影;运行时仍经模型逐一校验后才进入补全/hover,
//! 准确性由内嵌模型保证。彻底消除"手维护静态表 vs 模型"的漂移根因。
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crate 位于 crates/godotlsp,语料在仓库根的 godot-docs/。
    let corpus = manifest.join("../../godot-docs");
    let gs = corpus.join("gdd_1591_@GlobalScope.md");
    let text = fs::read_to_string(&gs).unwrap_or_else(|e| {
        panic!(
            "无法读取 Godot 文档语料 {}: {}。\n\
             该语料是派生全局清单的权威源,请确认 godot-docs/ 目录存在\n\
             (若以 git 子模块引入,运行 `git submodule update --init`)。",
            gs.display(),
            e
        )
    });

    let (singletons, utilities, enums) = parse_globalscope(&text);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR 未设置"));
    let gen_path = out_dir.join("gdscript_docs_gen.rs");
    let content = format!(
        "/// 构建期由 `build.rs` 从 `godot-docs/gdd_1591_@GlobalScope.md` 自动派生——**勿手改**。\n\
         /// 与 `tools/extract_docs.py` 同源(均解析官方文档语料);运行时再经 gdscript-api\n\
         /// 内嵌模型(`bundled()`)逐一校验后才进入补全/hover,准确性由模型保证。\n\
         pub const SINGLETON_NAMES: &[&str] = &[\n{}\n];\n\n\
         pub const UTILITY_NAMES: &[&str] = &[\n{}\n];\n\n\
         pub const GLOBAL_ENUM_NAMES: &[&str] = &[\n{}\n];\n",
        block(&singletons),
        block(&utilities),
        block(&enums),
    );
    fs::write(&gen_path, content).expect("写入生成的 gdscript_docs_gen.rs 失败");

    println!("cargo:rerun-if-changed={}", gs.display());
    println!("cargo:rerun-if-changed=build.rs");
}

/// 把名字列表渲染为 `r#"name"#` 逗号分隔的 Rust 数组体。
fn block(names: &[String]) -> String {
    names
        .iter()
        .map(|n| format!("    r#\"{}\"#", n))
        .collect::<Vec<_>>()
        .join(",\n")
}

/// 从 `@GlobalScope` 语料解析(单例 / 工具函数 / 全局枚举名)。正则与 `tools/extract_docs.py`
/// 完全一致(同一份官方语料),保证派生结果与语料严格同步。
fn parse_globalscope(text: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let props = section(text, "## Properties");
    let methods = section(text, "## Methods");
    let enums = section(text, "## Enumerations");

    let singleton_re = regex::Regex::new(r"^(?P<n>[A-Za-z_]\w*)\s*\|").unwrap();
    let utility_re = regex::Regex::new(r"^[\w\[\]]+\s*\|\s*(?P<n>[a-z_][a-z0-9_]*)\s*\(").unwrap();
    let enum_re = regex::Regex::new(r"^enum\s+(?P<n>[A-Za-z_]\w*)").unwrap();

    let singletons = props
        .lines()
        .filter_map(|l| singleton_re.captures(l.trim()).map(|c| c["n"].to_string()))
        .collect();
    let utilities = methods
        .lines()
        .filter_map(|l| utility_re.captures(l.trim()).map(|c| c["n"].to_string()))
        .collect();
    let enums = enums
        .lines()
        .filter_map(|l| enum_re.captures(l.trim()).map(|c| c["n"].to_string()))
        .collect();
    (singletons, utilities, enums)
}

/// 截取 `## header` 到下个 `## ` 之间的文本段。
fn section(text: &str, header: &str) -> String {
    let start = match text.find(header) {
        Some(s) => s + header.len(),
        None => return String::new(),
    };
    let rest = &text[start..];
    match rest.find("\n## ") {
        Some(end) => rest[..end].to_string(),
        None => rest.to_string(),
    }
}
