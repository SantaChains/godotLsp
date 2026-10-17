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

/// 内建构造/运算符重载条目（C6.5，sig 键——重载并列同名会撞）。
#[derive(Debug, Deserialize)]
struct OverloadSig {
    #[serde(default)]
    d: String,
    #[serde(default)]
    s: String,
}

#[derive(Debug, Deserialize)]
struct BuiltinSigs {
    #[serde(default)]
    c: Vec<OverloadSig>,
    /// 运算符条目（数据已入 hoverdata.json；消费面 per-type 运算符 hover
    /// 需二元表达式类型推断，属开放问题——C6.5d 式备未来）。
    #[serde(default)]
    #[allow(dead_code)]
    o: Vec<OverloadSig>,
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
    #[serde(default)]
    k: HashMap<String, BuiltinSigs>,
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

/// 内建类型构造重载，(签名, 描述)，语料序（默认构造在前）。
/// 语料无该类型（如 Nil）返回空。
pub fn builtin_ctors(ty: &str) -> Vec<(String, String)> {
    data().k
        .get(ty)
        .map(|b| b.c.iter().map(|e| (e.s.clone(), e.d.clone())).collect())
        .unwrap_or_default()
}

/// 内建类型运算符条目，(签名, 描述)，语料序。
/// 当前无消费面（per-type 运算符 hover 属开放问题），随数据一并备未来。
#[allow(dead_code)]
pub fn builtin_ops(ty: &str) -> Vec<(String, String)> {
    data().k
        .get(ty)
        .map(|b| b.o.iter().map(|e| (e.s.clone(), e.d.clone())).collect())
        .unwrap_or_default()
}