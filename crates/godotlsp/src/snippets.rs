//! C2 snippet 面：上下文感知模板。
//!
//! 分工与插件 16 条手动 snippet（editors/vscode/snippets/gdscript.json）互补：
//! 服务器版多 IDE（nvim/zed/helix）+ 上下文感知（宿主类过滤、语句位判定），
//! 插件版保留作无 LSP 兜底。
//!
//! 全部模板经 `snippetSupport` 能力协商——客户端不声明则整面不出现，
//! 非零误报的"降级为纯文本"不采用（模板没意义就别给）。

use crate::scoring::{self, bucket};
use lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat};

/// GDScript 虚方法（引擎回调）模板表。`base` = 出现所需的最低宿主类
/// （宿主继承链须含它）；`None` = 任意 Object 系（恒可）。
struct VMethod {
    name: &'static str,
    params: &'static str,
    base: Option<&'static str>,
}

const VIRTUALS: &[VMethod] = &[
    VMethod { name: "_init", params: "", base: None },
    VMethod { name: "_notification", params: "what: int", base: Some("Object") },
    VMethod { name: "_enter_tree", params: "", base: Some("Node") },
    VMethod { name: "_ready", params: "", base: Some("Node") },
    VMethod { name: "_exit_tree", params: "", base: Some("Node") },
    VMethod { name: "_process", params: "delta: float", base: Some("Node") },
    VMethod { name: "_physics_process", params: "delta: float", base: Some("Node") },
    VMethod { name: "_input", params: "event: InputEvent", base: Some("Node") },
    VMethod { name: "_unhandled_input", params: "event: InputEvent", base: Some("Node") },
    VMethod { name: "_unhandled_key_input", params: "event: InputEvent", base: Some("Node") },
    VMethod { name: "_draw", params: "", base: Some("CanvasItem") },
    VMethod { name: "_gui_input", params: "event: InputEvent", base: Some("Control") },
];

/// 复合语句模板：(label, 中文 detail, snippet 正文)。
/// label 用 ASCII 蛇形（客户端按 label 前缀过滤），正文避免裸 `$`
/// （LSP snippet 里 `$` 是占位符转义字符，GDScript 节点路径 `$Node` 会炸）。
const COMPOSITES: &[(&str, &str, &str)] = &[
    (
        "instantiate_scene",
        "实例化场景并挂到当前节点",
        "var ${1:node} := preload(${2:\"res://scene.tscn\"}).instantiate()\nadd_child(${1:node})$0",
    ),
    (
        "connect_lambda",
        "信号连接（lambda）",
        "${1:emitter}.${2:signal}.connect(func(${3:args}): $0)",
    ),
    (
        "create_tween",
        "补间动画（tween_property）",
        "var ${1:t} := create_tween()\n${1:t}.tween_property(${2:self}, \"${3:position}\", ${4:Vector2.ZERO}, ${5:0.3})$0",
    ),
    (
        "await_timer",
        "延时等待（场景树计时器）",
        "await get_tree().create_timer(${1:1.0}).timeout$0",
    ),
];

/// 控制流关键字的 snippet 正文（关键字 label 不变，仅升级插入文本）。
pub fn keyword_snippet(kw: &str) -> Option<&'static str> {
    match kw {
        "for" => Some("for ${1:i} in ${2:range(10)}:\n\t$0"),
        "while" => Some("while ${1:cond}:\n\t$0"),
        "if" => Some("if ${1:cond}:\n\t$0"),
        "match" => Some("match ${1:value}:\n\t${2:pattern}:\n\t\t$0"),
        _ => None,
    }
}

fn snippet_item(label: String, detail: String, insert: String, prefix: &str) -> CompletionItem {
    let sort = scoring::sort_text(bucket::SNIPPET, scoring::fuzzy(prefix, &label));
    CompletionItem {
        label,
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some(detail),
        insert_text: Some(insert),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some(sort),
        ..CompletionItem::default()
    }
}

/// `func _<|` 上下文的虚方法模板。`chain` = 宿主继承链（engine 视角）。
pub fn virtual_items(chain: &[&str], prefix: &str) -> Vec<CompletionItem> {
    VIRTUALS
        .iter()
        .filter(|v| v.base.is_none_or(|b| chain.contains(&b)))
        .map(|v| {
            let insert = format!("{}({}) -> void:\n\t$0", v.name, v.params);
            let detail = match v.base {
                Some(b) => format!("虚方法模板（{b} 系）"),
                None => "虚方法模板".to_string(),
            };
            snippet_item(v.name.to_string(), detail, insert, prefix)
        })
        .collect()
}

/// 语句位复合模板。只对与输入有词面关系的项出现（空前缀不泛滥）。
pub fn statement_items(prefix: &str) -> Vec<CompletionItem> {
    COMPOSITES
        .iter()
        .filter(|(label, ..)| scoring::is_match(scoring::fuzzy(prefix, label)))
        .map(|(label, detail, insert)| snippet_item(label.to_string(), detail.to_string(), insert.to_string(), prefix))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtuals_filtered_by_chain() {
        let node_chain = ["Node", "Object"];
        let names: Vec<String> = virtual_items(&node_chain, "_").into_iter().map(|i| i.label).collect::<Vec<String>>();
        assert!(names.iter().any(|l| l == "_ready") && names.iter().any(|l| l == "_process"), "{names:?}");
        assert!(!names.iter().any(|l| l == "_draw"), "Node 系不应出 _draw: {names:?}");
        assert!(!names.iter().any(|l| l == "_gui_input"), "Node 系不应出 _gui_input");

        let canvas_chain = ["Sprite2D", "CanvasItem", "Node", "Object"];
        let names: Vec<String> = virtual_items(&canvas_chain, "_").into_iter().map(|i| i.label).collect::<Vec<String>>();
        assert!(names.iter().any(|l| l == "_draw"), "CanvasItem 系应出 _draw");
        assert!(!names.iter().any(|l| l == "_gui_input"), "CanvasItem 非 Control 不出 _gui_input");

        // GDScript 无 extends 默认 RefCounted——只出 _init + _notification（Object 系恒可）。
        let refc = ["RefCounted", "Object"];
        let names: Vec<String> = virtual_items(&refc, "_").into_iter().map(|i| i.label).collect::<Vec<String>>();
        assert_eq!(names, vec!["_init", "_notification"], "RefCounted 系只出 Object 级虚方法: {names:?}");
    }

    #[test]
    fn virtual_item_shape() {
        let items = virtual_items(&["Node", "Object"], "_re");
        let ready = items.iter().find(|i| i.label == "_ready").unwrap();
        assert_eq!(ready.insert_text.as_deref(), Some("_ready() -> void:\n\t$0"));
        assert_eq!(ready.insert_text_format, Some(InsertTextFormat::SNIPPET));
        assert!(ready.detail.as_deref().unwrap().contains("Node"));
        // sortText 已编码（SNIPPET 桶 + 取反 fuzzy）。
        assert!(ready.sort_text.as_deref().unwrap().starts_with("11"));
    }

    #[test]
    fn composites_gated_by_word_match() {
        assert!(statement_items("").is_empty(), "空前缀不泛滥");
        let hit = statement_items("inst");
        assert_eq!(hit.len(), 1, "inst 只应命中 instantiate_scene: {:?}", hit.iter().map(|i| &i.label).collect::<Vec<_>>());
        assert_eq!(hit[0].label, "instantiate_scene");
        // 模板正文不得含裸 $（LSP 占位符转义）——$ 只允许出现在 ${n:...} 与 $n 中。
        for (.., insert) in COMPOSITES {
            let mut chars = insert.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '$' {
                    let next = chars.peek().copied();
                    assert!(next == Some('{') || next.is_some_and(|n| n.is_ascii_digit()), "裸 $ 于 {insert:?}");
                }
            }
        }
        // tween 模板里 "position" 的引号没破坏占位符。
        let tw = statement_items("tween");
        assert!(tw[0].insert_text.as_deref().unwrap().contains("${3:position}"));
    }

    #[test]
    fn keyword_snippets_control_flow() {
        assert!(keyword_snippet("for").unwrap().starts_with("for ${1:i}"));
        assert!(keyword_snippet("match").unwrap().contains("${2:pattern}"));
        assert_eq!(keyword_snippet("func"), None, "func 关键字不出模板（虚方法模板另有上下文）");
        assert_eq!(keyword_snippet("extends"), None);
    }
}
