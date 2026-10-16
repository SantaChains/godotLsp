//! hover 描述层数据：从 hoverdata.json 加载。

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BuiltinFunc {
    #[serde(default)]
    d: String,
    #[serde(default)]
    s: String,
}

#[derive(Debug, Deserialize)]
struct HoverData {
    #[serde(default)]
    c: HashMap<String, String>,
    #[serde(default)]
    m: HashMap<String, String>,
    #[serde(default)]
    f: HashMap<String, BuiltinFunc>,
    #[serde(default)]
    a: HashMap<String, String>,
}

static DATA: std::sync::OnceLock<HoverData> = std::sync::OnceLock::new();

fn data() -> &'static HoverData {
    DATA.get_or_init(|| {
        let json = include_str!("hoverdata.json");
        serde_json::from_str(json).expect("parse hoverdata.json")
    })
}

pub fn class_brief(class: &str) -> Option<&'static str> {
    data().c.get(class).map(|s| s.as_str())
}

pub fn member(class: &str, member: &str) -> Option<&'static str> {
    let key = format!("{}.{}", class, member);
    data().m.get(&key).map(|s| s.as_str())
}

pub fn gdscript_fn(name: &str) -> Option<(String, String)> {
    let bf = data().f.get(name)?;
    Some((bf.s.clone(), bf.d.clone()))
}

pub fn annotation(name: &str) -> Option<&'static str> {
    data().a.get(name).map(|s| s.as_str())
}

/// 全部注解，按名排序（completion 注解面消费）。
pub fn annotations() -> Vec<(String, String)> {
    let mut v: Vec<_> = data().a.iter().map(|(k, d)| (k.clone(), d.clone())).collect();
    v.sort();
    v
}

/// 全部 @GDScript 内建函数，(名, (签名, 描述))，按名排序。
pub fn gdscript_fns() -> Vec<(String, (String, String))> {
    let mut v: Vec<_> = data().f.iter().map(|(k, bf)| (k.clone(), (bf.s.clone(), bf.d.clone()))).collect();
    v.sort();
    v
}