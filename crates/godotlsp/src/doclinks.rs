//! 站内文档链接生成（W2 核心闭环）。
//!
//! 锚点契约（与 `tools/gen_docs.py` 生成器一致，单元测试防漂移）：
//!   类页   `{BASE}builtin/class_<小写去@>.html`
//!   伪类   去@ 后与真实类名冲突（@GDScript vs 真类 GDScript）→ `class_at_<base>`
//!   成员   同页 `#<原名>`；枚举 `#enum-<Enum名>`
//!
//! 链接目标的 md 页由生成器产出并入库（`website/src/builtin/`），
//! 单元测试枚举全部类/成员逐一校验文件与锚点存在，缺一即 `cargo test` 失败。

pub const BASE: &str = "https://santachains.github.io/godotlsp/";

/// 伪类：不在 extension_api.json 中，由生成器合成骨架页。
const PSEUDO_CLASSES: [&str; 2] = ["@GlobalScope", "@GDScript"];

/// 类页 slug：`class_<小写去@>`；伪类与真实类名冲突时加 `at_`。
pub fn class_slug(class: &str, is_real: impl Fn(&str) -> bool) -> String {
    let base = class.to_lowercase().replace('@', "");
    if PSEUDO_CLASSES.contains(&class) && is_real(&base) {
        return format!("class_at_{base}");
    }
    format!("class_{base}")
}

/// 类页相对 URL（相对站点根）。
pub fn class_page(class: &str, is_real: impl Fn(&str) -> bool) -> String {
    format!("builtin/{}.html", class_slug(class, is_real))
}

/// 类页绝对 URL。
pub fn class_url(class: &str, is_real: impl Fn(&str) -> bool) -> String {
    format!("{BASE}{}", class_page(class, is_real))
}

/// 成员页内锚点 URL：方法/属性/信号/常量用原名，枚举用 `enum-<名>`。
pub fn member_url(class: &str, member: &str, is_enum: bool, is_real: impl Fn(&str) -> bool) -> String {
    let anchor = if is_enum { format!("enum-{member}") } else { member.to_string() };
    format!("{}#{anchor}", class_url(class, is_real))
}

/// 注解锚点：`#annotation-<名>`（@GDScript 页注解节，生成器同契约）。
pub fn annotation_url(name: &str, is_real: impl Fn(&str) -> bool) -> String {
    format!("{}#annotation-{name}", class_url("@GDScript", is_real))
}

/// 语法篇 URL：`slug` 或 `slug#kw-xxx`（锚点契约见规划文档 §五之三）。
pub fn syntax_url(target: &str) -> String {
    match target.split_once('#') {
        Some((slug, anchor)) => format!("{BASE}syntax/{slug}.html#{anchor}"),
        None => format!("{BASE}syntax/{target}.html"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// builtin 生成页目录（仓库内，产物入库）。
    fn builtin_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../website/src/builtin")
    }

    fn api_json() -> &'static serde_json::Value {
        static CACHE: std::sync::OnceLock<serde_json::Value> = std::sync::OnceLock::new();
        CACHE.get_or_init(|| {
            let text = include_str!("../../gdscript-engine/testdata/extension_api.json");
            serde_json::from_str(text).expect("解析 extension_api.json")
        })
    }

    fn is_real_lower(base: &str) -> bool {
        real_names(api_json()).contains(base)
    }

    fn real_names(api: &serde_json::Value) -> &'static std::collections::HashSet<String> {
        static CACHE: std::sync::OnceLock<std::collections::HashSet<String>> = std::sync::OnceLock::new();
        CACHE.get_or_init(|| {
            api["classes"]
                .as_array()
                .unwrap()
                .iter()
                .chain(api["builtin_classes"].as_array().unwrap())
                .map(|c| c["name"].as_str().unwrap().to_lowercase())
                .collect()
        })
    }

    /// 断言 md 页存在且含显式锚点 `{#anchor}`。
    fn assert_anchor(class: &str, anchor: &str) {
        let path = builtin_dir().join(format!("{}.md", class_slug(class, is_real_lower)));
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("类页缺失 {}: {e}", path.display()));
        assert!(
            text.contains(&format!("{{#{anchor}}}")),
            "锚点缺失: {path:?} 中无 {{#{anchor}}}"
        );
    }

    #[test]
    fn slug_disambiguation() {
        assert_eq!(class_slug("@GlobalScope", is_real_lower), "class_globalscope");
        assert_eq!(class_slug("@GDScript", is_real_lower), "class_at_gdscript");
        assert_eq!(class_slug("Node", is_real_lower), "class_node");
        assert_eq!(class_slug("Vector2", is_real_lower), "class_vector2");
    }

    #[test]
    fn every_class_page_exists() {
        let api = api_json();
        let names: Vec<&str> = api["classes"]
            .as_array()
            .unwrap()
            .iter()
            .chain(api["builtin_classes"].as_array().unwrap())
            .map(|c| c["name"].as_str().unwrap())
            .collect();
        for name in &names {
            let path = builtin_dir().join(format!("{}.md", class_slug(name, is_real_lower)));
            assert!(path.exists(), "类页缺失: {}", path.display());
        }
        // 伪类页
        for name in PSEUDO_CLASSES {
            let path = builtin_dir().join(format!("{}.md", class_slug(name, is_real_lower)));
            assert!(path.exists(), "伪类页缺失: {}", path.display());
        }
    }

    #[test]
    fn every_member_anchor_exists() {
        let api = api_json();
        for c in api["classes"].as_array().unwrap().iter().chain(api["builtin_classes"].as_array().unwrap()) {
            let name = c["name"].as_str().unwrap();
            // 方法（builtin 也在 methods）
            for m in c["methods"].as_array().unwrap_or(&vec![]) {
                let mn = m["name"].as_str().unwrap();
                assert_anchor(name, mn);
            }
            // 属性（类 properties / builtin members）
            for p in c["properties"].as_array().or_else(|| c["members"].as_array()).unwrap_or(&vec![]) {
                assert_anchor(name, p["name"].as_str().unwrap());
            }
            // 信号
            for s in c["signals"].as_array().unwrap_or(&vec![]) {
                assert_anchor(name, s["name"].as_str().unwrap());
            }
            // 枚举
            for e in c["enums"].as_array().unwrap_or(&vec![]) {
                assert_anchor(name, &format!("enum-{}", e["name"].as_str().unwrap()));
                for v in e["values"].as_array().unwrap_or(&vec![]) {
                    assert_anchor(name, v["name"].as_str().unwrap());
                }
            }
            // 常量
            for k in c["constants"].as_array().unwrap_or(&vec![]) {
                assert_anchor(name, k["name"].as_str().unwrap());
            }
        }
        // @GlobalScope：工具函数 + 全局枚举 + 全局常量
        for f in api["utility_functions"].as_array().unwrap() {
            assert_anchor("@GlobalScope", f["name"].as_str().unwrap());
        }
        for e in api["global_enums"].as_array().unwrap() {
            assert_anchor("@GlobalScope", &format!("enum-{}", e["name"].as_str().unwrap()));
            for v in e["values"].as_array().unwrap_or(&vec![]) {
                assert_anchor("@GlobalScope", v["name"].as_str().unwrap());
            }
        }
        for k in api["global_constants"].as_array().unwrap() {
            assert_anchor("@GlobalScope", k["name"].as_str().unwrap());
        }
    }

    #[test]
    fn url_shapes() {
        assert_eq!(
            member_url("Node", "add_child", false, is_real_lower),
            "https://santachains.github.io/godotlsp/builtin/class_node.html#add_child"
        );
        assert_eq!(
            member_url("Node", "ProcessMode", true, is_real_lower),
            "https://santachains.github.io/godotlsp/builtin/class_node.html#enum-ProcessMode"
        );
        assert_eq!(
            member_url("@GlobalScope", "sin", false, is_real_lower),
            "https://santachains.github.io/godotlsp/builtin/class_globalscope.html#sin"
        );
    }

    /// §五之三：语法篇目标支持 `slug#kw-x` 锚点形态。
    #[test]
    fn syntax_url_with_anchor() {
        assert_eq!(syntax_url("12-await"), format!("{BASE}syntax/12-await.html"));
        assert_eq!(
            syntax_url("12-await#kw-await"),
            format!("{BASE}syntax/12-await.html#kw-await")
        );
    }
}
