//! `gdscript-engine` —— Godot 引擎 API 模型摄取层（R2 实现）。
//!
//! 数据源：Godot 官方 `godot --dump-extension-api-with-docs` 导出的
//! `extension_api.json`（走 ClassDB，完备-by-construction）。设计见
//! `docs/from-scratch-design.md` §7。本 crate 在运行期零引擎：模型是静态数据。
//!
//! 用法：
//! ```no_run
//! let api = gdscript_engine::EngineApi::from_json(include_str!(
//!     "../../../crates/gdscript-engine/testdata/extension_api.json"
//! ))
//! .expect("valid dump");
//! let node2d = api.class_by_name("Node2D").expect("Node2D");
//! assert_eq!(node2d.inherits.as_deref(), Some("CanvasItem"));
//! let m = api.find_method("Node2D", "get_position");
//! ```

use std::collections::HashMap;

use serde::Deserialize;

// ───────────────────────── 归一化后的查询模型 ─────────────────────────

/// 函数/方法签名。
#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub return_type: String,
    pub is_static: bool,
    pub is_const: bool,
    pub is_vararg: bool,
    pub arguments: Vec<Param>,
}

/// 形参（引擎模型侧，类型为字符串如 `Vector2`/`int`/`String`）。
#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub ty: String,
}

/// 属性。
#[derive(Clone, Debug)]
pub struct Property {
    pub name: String,
    pub ty: String,
    pub setter: String,
    pub getter: String,
}

/// 枚举及其具名值。
#[derive(Clone, Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<(String, i64)>,
}

/// 常量（类级或全局）。
#[derive(Clone, Debug)]
pub struct Constant {
    pub name: String,
    pub value: ConstValue,
}

#[derive(Clone, Debug)]
pub enum ConstValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

/// 具名引擎类（Node2D、Object、...）。
#[derive(Clone, Debug)]
pub struct EngineClass {
    pub name: String,
    pub inherits: Option<String>,
    pub is_instantiable: bool,
    pub methods: Vec<Function>,
    pub properties: Vec<Property>,
    pub enums: Vec<Enum>,
    pub constants: Vec<Constant>,
}

/// 内建值类型（Vector2、int、Array、Dictionary、String...）。
#[derive(Clone, Debug)]
pub struct BuiltinClass {
    pub name: String,
    pub methods: Vec<Function>,
    /// 值成员（Vector2.x / .y）。
    pub members: Vec<Property>,
    /// 内建类自带枚举（Vector3.Axis / Variant.Type 等，extension_api.json builtin_classes.enums）。
    pub enums: Vec<Enum>,
}

/// 单例（autoload 全局节点）。
#[derive(Clone, Debug)]
pub struct Singleton {
    pub name: String,
    pub ty: String,
}

/// 引擎模型顶层。
#[derive(Clone, Debug, Default)]
pub struct EngineApi {
    pub version: String,
    pub classes: HashMap<String, EngineClass>,
    pub builtins: HashMap<String, BuiltinClass>,
    pub global_enums: Vec<Enum>,
    pub global_constants: Vec<Constant>,
    pub utility_functions: Vec<Function>,
    pub singletons: Vec<Singleton>,
}

impl EngineApi {
    /// 从 `extension_api.json` 文本解析。
    pub fn from_json(json: &str) -> serde_json::Result<EngineApi> {
        let raw: RawApi = serde_json::from_str(json)?;
        Ok(raw.into_model())
    }

    pub fn class_by_name(&self, name: &str) -> Option<&EngineClass> {
        self.classes.get(name)
    }

    pub fn builtin_by_name(&self, name: &str) -> Option<&BuiltinClass> {
        self.builtins.get(name)
    }

    /// 沿 `inherits` 链返回类名序列（含自身），祖先在前。循环继承防御。
    pub fn ancestor_chain<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let mut out = Vec::new();
        let mut cur = Some(name);
        let mut guard = 0;
        while let Some(n) = cur {
            if guard > 64 {
                break; // 循环继承防御
            }
            guard += 1;
            out.push(n);
            cur = self.classes.get(n).and_then(|c| c.inherits.as_deref());
        }
        out
    }

    /// 沿继承链查找方法（自身优先，再祖先）。
    pub fn find_method(&self, class: &str, name: &str) -> Option<&Function> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls) {
                if let Some(m) = c.methods.iter().find(|m| m.name == name) {
                    return Some(m);
                }
            }
        }
        // builtin 兜底（Vector2 等内建类型方法）。
        if let Some(b) = self.builtins.get(class) {
            return b.methods.iter().find(|m| m.name == name);
        }
        None
    }

    /// 沿继承链查找属性；内建类型同时查其值成员（Vector2.x）。
    pub fn find_property(&self, class: &str, name: &str) -> Option<&Property> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls) {
                if let Some(p) = c.properties.iter().find(|p| p.name == name) {
                    return Some(p);
                }
            }
        }
        self.builtins.get(class).and_then(|b| b.members.iter().find(|p| p.name == name))
    }

    /// 沿继承链查找枚举；内建类型兜底（Vector3.Axis）。
    pub fn find_enum(&self, class: &str, name: &str) -> Option<&Enum> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls) {
                if let Some(e) = c.enums.iter().find(|e| e.name == name) {
                    return Some(e);
                }
            }
        }
        self.builtins.get(class).and_then(|b| b.enums.iter().find(|e| e.name == name))
    }

    /// 全局枚举按值名查：返回 (枚举名, 数值)。`KEY_LEFT` → ("Key", 4194305)。
    pub fn global_enum_value(&self, name: &str) -> Option<(&str, i64)> {
        for e in &self.global_enums {
            if let Some((_, v)) = e.values.iter().find(|(n, _)| n == name) {
                return Some((e.name.as_str(), *v));
            }
        }
        None
    }

    pub fn utility_function(&self, name: &str) -> Option<&Function> {
        self.utility_functions.iter().find(|f| f.name == name)
    }

    pub fn singleton_type(&self, name: &str) -> Option<&str> {
        self.singletons.iter().find(|s| s.name == name).map(|s| s.ty.as_str())
    }
}

// ───────────────────────── serde 原始结构 ─────────────────────────

#[derive(Deserialize)]
struct RawApi {
    header: RawHeader,
    #[serde(default)]
    classes: Vec<RawClass>,
    #[serde(default)]
    builtin_classes: Vec<RawBuiltin>,
    #[serde(default)]
    utility_functions: Vec<RawFunction>,
    #[serde(default)]
    global_enums: Vec<RawEnum>,
    #[serde(default)]
    global_constants: Vec<RawConst>,
    #[serde(default)]
    singletons: Vec<RawSingleton>,
}

#[derive(Deserialize)]
struct RawHeader {
    version_major: u32,
    version_minor: u32,
    version_patch: u32,
}

#[derive(Deserialize)]
struct RawClass {
    name: String,
    #[serde(default)]
    inherits: Option<String>,
    #[serde(default)]
    is_instantiable: bool,
    #[serde(default)]
    methods: Vec<RawFunction>,
    #[serde(default)]
    properties: Vec<RawProperty>,
    #[serde(default)]
    enums: Vec<RawEnum>,
    #[serde(default)]
    constants: Vec<RawConst>,
}

#[derive(Deserialize)]
struct RawBuiltin {
    name: String,
    #[serde(default)]
    methods: Vec<RawFunction>,
    #[serde(default)]
    members: Vec<RawMember>,
    #[serde(default)]
    enums: Vec<RawEnum>,
}

#[derive(Deserialize)]
struct RawMember {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Deserialize)]
struct RawFunction {
    name: String,
    /// 返回值：引擎类用 `return_value: { type }`；builtin/utility 用 `return_type` 直填。
    #[serde(default)]
    return_value: Option<RawReturn>,
    #[serde(default)]
    return_type: Option<String>,
    #[serde(default)]
    is_static: bool,
    #[serde(default)]
    is_const: bool,
    #[serde(default)]
    is_vararg: bool,
    #[serde(default)]
    arguments: Vec<RawParam>,
}

#[derive(Deserialize)]
struct RawReturn {
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Deserialize)]
struct RawParam {
    name: String,
    #[serde(default)]
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Deserialize)]
struct RawProperty {
    #[serde(rename = "type")]
    ty: String,
    name: String,
    #[serde(default)]
    setter: String,
    #[serde(default)]
    getter: String,
}

#[derive(Deserialize)]
struct RawEnum {
    name: String,
    #[serde(default)]
    values: Vec<RawEnumValue>,
}

#[derive(Deserialize)]
struct RawEnumValue {
    name: String,
    value: i64,
}

#[derive(Deserialize)]
struct RawConst {
    name: String,
    #[serde(default)]
    value: serde_json::Value,
}

#[derive(Deserialize)]
struct RawSingleton {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

impl RawApi {
    fn into_model(self) -> EngineApi {
        let version = format!("{}.{}.{}", self.header.version_major, self.header.version_minor, self.header.version_patch);

        let classes: HashMap<String, EngineClass> = self
            .classes
            .into_iter()
            .map(|c| {
                (
                    c.name.clone(),
                    EngineClass {
                        name: c.name,
                        inherits: c.inherits,
                        is_instantiable: c.is_instantiable,
                        methods: c.methods.into_iter().map(into_fn).collect(),
                        properties: c
                            .properties
                            .into_iter()
                            .map(|p| Property { name: p.name, ty: p.ty, setter: p.setter, getter: p.getter })
                            .collect(),
                        enums: c.enums.into_iter().map(into_enum).collect(),
                        constants: c.constants.into_iter().map(into_const).collect(),
                    },
                )
            })
            .collect();

        let builtins: HashMap<String, BuiltinClass> = self
            .builtin_classes
            .into_iter()
            .map(|b| {
                (
                    b.name.clone(),
                    BuiltinClass {
                        name: b.name,
                        methods: b.methods.into_iter().map(into_fn).collect(),
                        enums: b.enums.into_iter().map(into_enum).collect(),
                        members: b
                            .members
                            .into_iter()
                            .map(|m| Property { name: m.name, ty: m.ty, setter: String::new(), getter: String::new() })
                            .collect(),
                    },
                )
            })
            .collect();

        EngineApi {
            version,
            classes,
            builtins,
            global_enums: self.global_enums.into_iter().map(into_enum).collect(),
            global_constants: self.global_constants.into_iter().map(into_const).collect(),
            utility_functions: self.utility_functions.into_iter().map(into_fn).collect(),
            singletons: self.singletons.into_iter().map(|s| Singleton { name: s.name, ty: s.ty }).collect(),
        }
    }
}

fn into_fn(f: RawFunction) -> Function {
    Function {
        name: f.name,
        return_type: f
            .return_value
            .map(|r| r.ty)
            .or(f.return_type)
            .unwrap_or_else(|| "void".to_string()),
        is_static: f.is_static,
        is_const: f.is_const,
        is_vararg: f.is_vararg,
        arguments: f.arguments.into_iter().map(|p| Param { name: p.name, ty: p.ty }).collect(),
    }
}

fn into_enum(e: RawEnum) -> Enum {
    Enum { name: e.name, values: e.values.into_iter().map(|v| (v.name, v.value)).collect() }
}

fn into_const(c: RawConst) -> Constant {
    let value = match c.value {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                ConstValue::Int(i)
            } else if let Some(f) = n.as_f64() {
                ConstValue::Float(f)
            } else {
                ConstValue::Null
            }
        }
        serde_json::Value::String(s) => ConstValue::String(s),
        serde_json::Value::Bool(b) => ConstValue::Bool(b),
        _ => ConstValue::Null,
    };
    Constant { name: c.name, value }
}
