//! 引擎全局面数据层:类 / 内建类型 / 单例 / 工具函数 / GDScript 内建函数 /
//! 全局枚举及值 / 伪常量 / 注解。
//!
//! 名单来自构建期提取的官方文档表([`crate::gdscript_docs`]),签名、枚举值与文档
//! 一律以 `gdscript-api` 模型(`extension_api.json` + `engine_docs.bin`)为权威——
//! 每个名字先经模型验证才进入补全与 hover,文档语料与引擎版本轻微漂移不影响准确性。
//!
//! 补全项以 `z` 前缀 `sort_text` 降权:文档内符号与关键字优先,引擎全局面兜底。

use std::sync::OnceLock;

use gdscript_api::gdscript_layer::{global_consts, LayerTy};
use gdscript_api::{bundled, ElemRef, TyRef};
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::gdscript_docs as docs;

/// 补全项的降权排序前缀:确保引擎全局面排在文档内符号与关键字之后。
const SORT_DEMOTE: &str = "z";

/// 单例常量池(补全与 hover 共用)。
struct Globals {
    items: Vec<CompletionItem>,
}

static GLOBALS: OnceLock<Globals> = OnceLock::new();

/// 全局面补全项(构建一次,常驻复用)。
pub fn completion_items() -> &'static [CompletionItem] {
    &globals().items
}

fn globals() -> &'static Globals {
    GLOBALS.get_or_init(|| Globals {
        items: build_items(),
    })
}

fn item(label: String, kind: CompletionItemKind, detail: String, sort_key: u8) -> CompletionItem {
    CompletionItem {
        label,
        kind: Some(kind),
        detail: Some(detail),
        sort_text: Some(format!("{SORT_DEMOTE}{sort_key}")),
        ..CompletionItem::default()
    }
}

fn build_items() -> Vec<CompletionItem> {
    let api = bundled();
    let mut out = Vec::new();

    // 引擎类(含编辑器专用类,后者在 detail 标注)。
    for c in api.classes() {
        let detail = match c.base.map(|b| api.class(b).name.clone()) {
            Some(base) => format!("引擎类,继承 {base}"),
            None => "引擎类".into(),
        };
        let detail = if c.api_type == gdscript_api::ApiType::Editor {
            format!("{detail}(编辑器专用)")
        } else {
            detail
        };
        out.push(item(c.name.clone(), CompletionItemKind::CLASS, detail, 0));
    }

    // 内建 Variant 类型。
    for b in api.builtins() {
        out.push(item(
            b.name.clone(),
            CompletionItemKind::STRUCT,
            "内建类型".into(),
            0,
        ));
    }

    // 单例(名单经模型验证)。
    for name in docs::SINGLETON_NAMES {
        let Some(class) = api.singleton(name) else {
            continue;
        };
        out.push(item(
            (*name).to_owned(),
            CompletionItemKind::CONSTANT,
            format!("单例({})", api.class(class).name),
            1,
        ));
    }

    // `@GlobalScope` 工具函数(名单经模型验证,签名/文档取自模型)。
    for name in docs::UTILITY_NAMES {
        let Some(u) = api.utility(name) else {
            continue;
        };
        out.push(item(
            u.name.clone(),
            CompletionItemKind::FUNCTION,
            format!("全局函数,{}", signature(api, &u.name, &u.params, &u.return_ty, u.is_vararg)),
            1,
        ));
    }

    // GDScript 内建函数(`preload`/`range`/…;文档与签名来自官方文档表)。
    for (name, md) in docs::BUILTIN_FN_DOCS {
        if api.utility(name).is_none() && api.gdscript_builtin(name).is_none() {
            continue;
        }
        let sig = md
            .strip_prefix("```gdscript\n")
            .and_then(|s| s.split_once("\n```"))
            .map(|(s, _)| s.to_owned())
            .unwrap_or_else(|| (*name).to_owned());
        out.push(item(
            (*name).to_owned(),
            CompletionItemKind::FUNCTION,
            format!("内建函数,{sig}"),
            1,
        ));
    }

    // 全局枚举及其值(值列表取自模型)。
    for name in docs::GLOBAL_ENUM_NAMES {
        let Some(e) = api.global_enum(name) else {
            continue;
        };
        out.push(item(
            e.name.clone(),
            CompletionItemKind::ENUM,
            "全局枚举".into(),
            1,
        ));
        for v in &e.values {
            out.push(item(
                v.name.clone(),
                CompletionItemKind::ENUM_MEMBER,
                format!("{} 枚举值", e.name),
                2,
            ));
        }
    }

    // 伪常量(PI/TAU/INF/NAN)。
    for c in global_consts() {
        out.push(item(
            c.name.to_owned(),
            CompletionItemKind::CONSTANT,
            format!("{} 常量", layer_ty(c.ty)),
            1,
        ));
    }

    // 注解(`@` 上下文由内核处理;这里供 hover,不进按名补全)。
    out
}

/// 渲染 `TyRef` 为 GDScript 类型文本。
fn ty_str(api: &gdscript_api::EngineApi, ty: &TyRef) -> String {
    match ty {
        TyRef::Void => "void".into(),
        TyRef::Variant => "Variant".into(),
        TyRef::Builtin(id) => api.builtin(*id).name.clone(),
        TyRef::Class(id) => api.class(*id).name.clone(),
        TyRef::TypedArray(el) => format!("Array[{}]", elem_str(api, el)),
        TyRef::TypedDict(k, v) => format!("Dictionary[{}, {}]", elem_str(api, k), elem_str(api, v)),
        TyRef::Enum { qualified, .. } => qualified.clone(),
    }
}

fn elem_str(api: &gdscript_api::EngineApi, el: &ElemRef) -> String {
    match el {
        ElemRef::Variant => "Variant".into(),
        ElemRef::Builtin(id) => api.builtin(*id).name.clone(),
        ElemRef::Class(id) => api.class(*id).name.clone(),
        ElemRef::Enum { qualified, .. } => qualified.clone(),
    }
}

fn layer_ty(ty: LayerTy) -> &'static str {
    match ty {
        LayerTy::Float => "float",
        LayerTy::Int => "int",
        LayerTy::Bool => "bool",
        LayerTy::Str => "String",
        LayerTy::Array => "Array",
        LayerTy::Dictionary => "Dictionary",
        LayerTy::Color => "Color",
        LayerTy::Variant => "Variant",
        LayerTy::Unknown => "Variant",
        LayerTy::Void => "void",
    }
}

/// `name(a: T, b: U = default) -> R`(可变参以 `…` 收尾)。
fn signature(
    api: &gdscript_api::EngineApi,
    name: &str,
    params: &[gdscript_api::Param],
    ret: &TyRef,
    vararg: bool,
) -> String {
    let mut parts: Vec<String> = params
        .iter()
        .map(|p| match &p.default {
            Some(d) => format!("{}: {} = {}", p.name, ty_str(api, &p.ty), d),
            None => format!("{}: {}", p.name, ty_str(api, &p.ty)),
        })
        .collect();
    if vararg {
        parts.push("…".into());
    }
    format!("{}({}) -> {}", name, parts.join(", "), ty_str(api, ret))
}

// ---- hover ----

/// 官方文档站点链接(zh-CN;`@` 路径转义为 `%40`)。
mod url {
    const BASE: &str = "https://docs.godotengine.org/zh-CN/stable/classes/";

    pub fn class(name: &str) -> String {
        format!("{BASE}class_{}.html", name.to_lowercase())
    }

    pub fn globalscope(kind: &str, name: &str) -> String {
        format!("{BASE}class_%40globalscope.html#class-globalscope-{kind}-{}", name.to_lowercase())
    }

    pub fn gdscript(kind: &str, name: &str) -> String {
        format!("{BASE}class_%40gdscript.html#class-gdscript-{kind}-{}", name.to_lowercase())
    }
}

fn with_link(decl: &str, doc: Option<&str>, link: String) -> String {
    let mut md = format!("```gdscript\n{decl}\n```");
    if let Some(d) = doc.filter(|d| !d.is_empty()) {
        md.push_str("\n\n---\n\n");
        md.push_str(d);
    }
    md.push_str(&format!("\n\n[📖 中文文档]({link})"));
    md
}

/// 光标处的裸标识符若命中引擎全局面,返回官方 zh-CN 文档链接(附加到内核 hover)。
#[must_use]
pub fn doc_link(word: &str) -> Option<String> {
    let api = bundled();
    if api.utility(word).is_some() {
        return Some(url::globalscope("method", word));
    }
    if api.gdscript_builtin(word).is_some() {
        return Some(url::gdscript("method", word));
    }
    if api.global_enum_value(word).is_some() {
        return Some(url::globalscope("constant", word));
    }
    if api.global_enum(word).is_some() {
        return Some(url::globalscope("enum", word));
    }
    if api.global_const(word).is_some() {
        return Some(url::gdscript("constant", word));
    }
    if docs::ANNOTATION_DOCS.iter().any(|(n, _)| *n == word) {
        return Some(url::gdscript("annotation", word));
    }
    if let Some(c) = api.singleton(word) {
        return Some(url::class(&api.class(c).name));
    }
    if api.class_by_name(word).is_some() || api.builtin_by_name(word).is_some() {
        return Some(url::class(word));
    }
    None
}

/// 光标处的裸标识符若命中引擎全局面,合成 hover Markdown(内核无结果或无文档时兜底)。
#[must_use]
pub fn hover_markdown(word: &str) -> Option<String> {
    let api = bundled();

    // 工具函数 / 内建函数(签名优先模型,内建函数用文档表)。
    if let Some(u) = api.utility(word) {
        let sig = signature(api, &u.name, &u.params, &u.return_ty, u.is_vararg);
        return Some(with_link(
            &sig,
            u.doc.and_then(|id| api.doc(id)),
            url::globalscope("method", word),
        ));
    }
    if api.gdscript_builtin(word).is_some()
        && let Some((_, md)) = docs::BUILTIN_FN_DOCS.iter().find(|(n, _)| *n == word)
    {
        return Some(format!("{md}\n\n[📖 中文文档]({})", url::gdscript("method", word)));
    }

    // 全局枚举值 → 枚举 → 伪常量。
    if let Some((e, v)) = api.global_enum_value(word) {
        let decl = format!("const {} = {} // {}: {}", v.name, v.value, e.name, v.name);
        return Some(with_link(
            &decl,
            e.doc.and_then(|id| api.doc(id)),
            url::globalscope("constant", word),
        ));
    }
    if let Some(e) = api.global_enum(word) {
        return Some(with_link(
            &format!("enum {}", e.name),
            e.doc.and_then(|id| api.doc(id)),
            url::globalscope("enum", word),
        ));
    }
    if api.global_const(word).is_some()
        && let Some((_, md)) = docs::CONST_DOCS.iter().find(|(n, _)| *n == word)
    {
        let decl = format!(
            "const {}: {}",
            word,
            global_consts()
                .into_iter()
                .find(|c| c.name == word)
                .map(|c| layer_ty(c.ty))
                .unwrap_or("float")
        );
        return Some(format!("{decl}\n\n{md}\n\n[📖 中文文档]({})", url::gdscript("constant", word)));
    }

    // 注解。
    if let Some((_, md)) = docs::ANNOTATION_DOCS.iter().find(|(n, _)| *n == word) {
        return Some(format!("`@{word}`\n\n{md}\n\n[📖 中文文档]({})", url::gdscript("annotation", word)));
    }

    // 单例 → 引擎类 → 内建类型。
    if let Some(class) = api.singleton(word) {
        let c = api.class(class);
        return Some(with_link(
            &format!("var {word}: {} // 单例", c.name),
            c.doc.and_then(|id| api.doc(id)),
            url::class(&c.name),
        ));
    }
    if let Some(id) = api.class_by_name(word) {
        let c = api.class(id);
        let decl = match c.base.map(|b| api.class(b).name.clone()) {
            Some(base) => format!("class {word} extends {base}"),
            None => format!("class {word}"),
        };
        return Some(with_link(
            &decl,
            c.doc.and_then(|id| api.doc(id)),
            url::class(word),
        ));
    }
    if let Some(id) = api.builtin_by_name(word) {
        let b = api.builtin(id);
        return Some(with_link(
            &format!("type {word}(内建类型)"),
            b.doc.and_then(|id| api.doc(id)),
            url::class(word),
        ));
    }
    None
}

// ---- 补全/签名回填(P0-1/2/4) ----

/// `doc` 为空时仅渲染官方文档链接;非空时文档在前。
fn doc_with_link(doc: &str, link: &str) -> String {
    let mut md = String::new();
    if !doc.is_empty() {
        md.push_str(doc);
        md.push_str("\n\n");
    }
    md.push_str(&format!("[📖 中文文档]({link})"));
    md
}

/// 注解补全回填:一行 detail + 完整 Markdown 文档(label 不带 `@`)。
#[must_use]
pub fn annotation_info(name: &str) -> Option<(&'static str, String)> {
    let (_, md) = docs::ANNOTATION_DOCS.iter().find(|(n, _)| *n == name)?;
    let detail = md
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with("```") && !l.starts_with('@'))?;
    Some((detail, format!("{md}\n\n[📖 中文文档]({})", url::gdscript("annotation", name))))
}

/// 成员补全回填:接收者类型名(内核 hover 的 `ty_label`)+ 成员名 →
/// (完整签名/类型 detail, 文档 Markdown)。命中引擎类或内建类型成员。
#[must_use]
pub fn member_info(ty_name: &str, member: &str) -> Option<(String, String)> {
    let api = bundled();
    if let Some(cid) = api.class_by_name(ty_name) {
        let link = url::class(ty_name);
        return match api.lookup_member(cid, member) {
            Some(gdscript_api::MemberRef::Method(sig)) => {
                let detail = signature(api, member, &sig.params, &sig.return_ty, sig.is_vararg);
                let doc = sig.doc.and_then(|id| api.doc(id)).unwrap_or_default();
                Some((detail, doc_with_link(doc, &link)))
            }
            Some(gdscript_api::MemberRef::Property(p)) => {
                let detail = format!("{}: {}", p.name, ty_str(api, &p.ty));
                let doc = p.doc.and_then(|id| api.doc(id)).unwrap_or_default();
                Some((detail, doc_with_link(doc, &link)))
            }
            Some(gdscript_api::MemberRef::Const(c)) => {
                let detail = format!("const {}", c.name);
                let doc = c.doc.and_then(|id| api.doc(id)).unwrap_or_default();
                Some((detail, doc_with_link(doc, &link)))
            }
            Some(gdscript_api::MemberRef::Signal(s)) => {
                let detail = format!("signal {}", s.name);
                let doc = s.doc.and_then(|id| api.doc(id)).unwrap_or_default();
                Some((detail, doc_with_link(doc, &link)))
            }
            _ => None,
        };
    }
    let bid = api.builtin_by_name(ty_name)?;
    let link = url::class(ty_name);
    if let Some(sig) = api.builtin_method(bid, member) {
        let detail = signature(api, member, &sig.params, &sig.return_ty, sig.is_vararg);
        let doc = sig.doc.and_then(|id| api.doc(id)).unwrap_or_default();
        return Some((detail, doc_with_link(doc, &link)));
    }
    let m = api.builtin_member(bid, member)?;
    let detail = format!("{}: {}", m.name, ty_str(api, &m.ty));
    Some((detail, doc_with_link("", &link)))
}

/// 成员方法调用文档(`recv.method(` 的 signature help 回填)。
#[must_use]
pub fn member_method_doc(ty_name: &str, method: &str) -> Option<String> {
    let api = bundled();
    let link = url::class(ty_name);
    if let Some(cid) = api.class_by_name(ty_name)
        && let Some(gdscript_api::MemberRef::Method(sig)) = api.lookup_member(cid, method)
    {
        let doc = sig.doc.and_then(|id| api.doc(id)).unwrap_or_default();
        return Some(doc_with_link(doc, &link));
    }
    let bid = api.builtin_by_name(ty_name)?;
    let sig = api.builtin_method(bid, method)?;
    let doc = sig.doc.and_then(|id| api.doc(id)).unwrap_or_default();
    Some(doc_with_link(doc, &link))
}

/// 全局调用文档回填(`sin(`/`preload(` 等 signature help):工具函数取模型,
/// GDScript 内建函数取文档表。
#[must_use]
pub fn signature_doc(name: &str) -> Option<String> {
    let api = bundled();
    if let Some(u) = api.utility(name) {
        let doc = u.doc.and_then(|id| api.doc(id)).unwrap_or_default();
        return Some(doc_with_link(doc, &url::globalscope("method", name)));
    }
    if api.gdscript_builtin(name).is_some()
        && let Some((_, md)) = docs::BUILTIN_FN_DOCS.iter().find(|(n, _)| *n == name)
    {
        return Some(format!("{md}\n\n[📖 中文文档]({})", url::gdscript("method", name)));
    }
    None
}

// ---- 关键字 hover(P0-3) ----

/// GDScript reference 在线文档(zh-CN)。
const GDSCRIPT_BASICS_URL: &str =
    "https://docs.godotengine.org/zh-CN/stable/tutorials/scripting/gdscript/gdscript_basics.html";

/// 语言关键字/伪常量别名的一行 hover(内核与全局面均未命中时兜底)。
#[must_use]
pub fn keyword_hover(word: &str) -> Option<String> {
    let (_, desc) = docs::KEYWORD_DOCS.iter().find(|(n, _)| *n == word)?;
    Some(format!("`{word}` — {desc}\n\n[📖 GDScript 基础]({GDSCRIPT_BASICS_URL})"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completions_cover_all_global_layers() {
        let items = completion_items();
        let has = |n: &str| items.iter().any(|i| i.label == n);
        assert!(has("Node"), "引擎类缺失");
        assert!(has("Vector2"), "内建类型缺失");
        assert!(has("Input"), "单例缺失");
        assert!(has("sin"), "工具函数缺失");
        assert!(has("print"), "工具函数缺失");
        assert!(has("preload"), "内建函数缺失");
        assert!(has("PI"), "伪常量缺失");
        assert!(has("Error"), "全局枚举缺失");
        assert!(has("KEY_A"), "枚举值缺失");
        // 降权排序:全局项永远带 z 前缀 sortText。
        assert!(items.iter().all(|i| i.sort_text.as_deref().is_some_and(|s| s.starts_with('z'))));
        // 名单中不存在的名字已被模型验证过滤(如语料多出的条目)。
        assert!(!has("NotARealSingleton"));
    }

    #[test]
    fn member_and_signature_enrichment_hits_model() {
        // 引擎类方法:完整签名 + 文档链接。
        let (detail, doc) = member_info("Input", "is_action_pressed").unwrap();
        assert!(detail.starts_with("is_action_pressed("), "{detail}");
        assert!(doc.contains("docs.godotengine.org/zh-CN/"), "{doc}");

        // 引擎类属性。
        let (detail, _) = member_info("Node2D", "position").unwrap();
        assert_eq!(detail, "position: Vector2");

        // 内建类型方法与属性。
        let (detail, _) = member_info("String", "to_int").unwrap();
        assert!(detail.starts_with("to_int("), "{detail}");
        let (detail, _) = member_info("Vector2", "x").unwrap();
        assert_eq!(detail, "x: float");

        // 全局/内建函数签名文档。
        assert!(signature_doc("sin").is_some_and(|d| d.contains("中文文档")));
        assert!(signature_doc("preload").is_some_and(|d| d.contains("Resource preload")));
        assert!(signature_doc("not_a_global_fn").is_none());
        assert!(member_method_doc("Input", "is_action_pressed").is_some());

        // 未命中。
        assert!(member_info("NotAClass", "x").is_none());
        assert!(member_info("Node", "not_a_member").is_none());
    }

    #[test]
    fn keyword_hover_and_annotation_detail() {
        // 关键字一行 hover + 官方链接。
        let h = keyword_hover("await").unwrap();
        assert!(h.contains("Waits for a signal"), "{h}");
        assert!(h.contains("docs.godotengine.org/zh-CN/"));
        assert!(keyword_hover("definitely_not_a_keyword").is_none());

        // 注解补全回填:一行 detail + 完整文档。
        let (detail, doc) = annotation_info("export_range").unwrap();
        assert!(!detail.is_empty() && !detail.starts_with("```"), "{detail}");
        assert!(doc.contains("@export_range") && doc.contains("中文文档"), "{doc}");
        assert!(annotation_info("not_an_annotation").is_none());
    }

    #[test]
    fn hover_covers_classes_singletons_fns_and_annotations() {
        // 类:extends Node 场景的文档跳转兜底。
        let h = hover_markdown("Node").unwrap();
        assert!(h.contains("class Node"), "{h}");
        assert!(h.contains("docs.godotengine.org/zh-CN/"), "缺文档链接:{h}");

        // 单例。
        let h = hover_markdown("Input").unwrap();
        assert!(h.contains("单例"), "{h}");

        // 工具函数:签名 + 文档。
        let h = hover_markdown("sin").unwrap();
        assert!(h.contains("sin(angle_rad: float) -> float"), "{h}");

        // 内建函数:签名来自文档表。
        let h = hover_markdown("preload").unwrap();
        assert!(h.contains("Resource preload(path: String)"), "{h}");

        // 枚举值 / 常量 / 注解。
        assert!(hover_markdown("KEY_A").is_some());
        assert!(hover_markdown("PI").unwrap().contains("PI"));
        assert!(hover_markdown("export_range").unwrap().contains("@export_range"));

        // 未命中返回 None。
        assert!(hover_markdown("definitely_not_a_global").is_none());
    }
}
