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

use std::collections::{HashMap, HashSet};

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
    /// ClassDB 虚方法（可被脚本 override 的引擎回调，`_process`/`_draw` 等）。
    /// 来源 = extension_api.json `methods[].is_virtual`（C6.3 模型化）。
    pub is_virtual: bool,
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
    /// 类声明的信号（Node.ready / CanvasItem.draw…）。
    pub signals: Vec<EngineSignal>,
}

/// 引擎类信号（extension_api.json class.signals）。
#[derive(Clone, Debug)]
pub struct EngineSignal {
    pub name: String,
    pub arguments: Vec<Param>,
}

/// Object core 手工注册虚方法（`ClassDB::add_virtual_method`，不在
/// extension_api.json——见 `EngineApi::object_core_virtuals`）。
#[derive(Clone, Copy, Debug)]
pub struct CoreVirtual {
    pub name: &'static str,
    pub arguments: &'static [(&'static str, &'static str)],
    /// `None` = void；`"Variant"` = NIL_IS_VARIANT 返回。
    pub return_type: Option<&'static str>,
}

/// object.cpp:1919-1979 逐条对照（`_to_string` 返回 Variant::STRING；
/// `_set`/`_property_can_revert` 返回 bool；`_get`/`_property_get_revert`
/// 返回 NIL_IS_VARIANT → Variant；`_get_property_list` 返回
/// Array[Dictionary]；`_iter_*` 返回 bool/Variant）。`Object.free` 虽
/// add_virtual_method 但 p_virtual=false，不进 override 面。
pub const OBJECT_CORE_VIRTUALS: &[CoreVirtual] = &[
    CoreVirtual { name: "_init", arguments: &[], return_type: None },
    CoreVirtual { name: "_to_string", arguments: &[], return_type: Some("String") },
    CoreVirtual { name: "_notification", arguments: &[("what", "int")], return_type: None },
    CoreVirtual {
        name: "_set",
        arguments: &[("property", "StringName"), ("value", "Variant")],
        return_type: Some("bool"),
    },
    CoreVirtual { name: "_get", arguments: &[("property", "StringName")], return_type: Some("Variant") },
    CoreVirtual { name: "_get_property_list", arguments: &[], return_type: Some("Array[Dictionary]") },
    CoreVirtual { name: "_validate_property", arguments: &[("property", "Dictionary")], return_type: None },
    CoreVirtual { name: "_property_can_revert", arguments: &[("property", "StringName")], return_type: Some("bool") },
    CoreVirtual { name: "_property_get_revert", arguments: &[("property", "StringName")], return_type: Some("Variant") },
    CoreVirtual { name: "_iter_init", arguments: &[("iter", "Array")], return_type: Some("bool") },
    CoreVirtual { name: "_iter_next", arguments: &[("iter", "Array")], return_type: Some("bool") },
    CoreVirtual { name: "_iter_get", arguments: &[("iter", "Variant")], return_type: Some("Variant") },
];

/// 内建值类型（Vector2、int、Array、Dictionary、String...）。
#[derive(Clone, Debug)]
pub struct BuiltinClass {
    pub name: String,
    pub methods: Vec<Function>,
    /// 值成员（Vector2.x / .y）。
    pub members: Vec<Property>,
    /// 内建类自带枚举（Vector3.Axis / Variant.Type 等，extension_api.json builtin_classes.enums）。
    pub enums: Vec<Enum>,
    /// 内建类常量（Vector2.ZERO / ONE / LEFT…，值为字符串表达式）。
    pub constants: Vec<Constant>,
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
            if let Some(c) = self.classes.get(cls)
                && let Some(m) = c.methods.iter().find(|m| m.name == name) {
                    return Some(m);
                }
        }
        // builtin 兜底（Vector2 等内建类型方法）。
        if let Some(b) = self.builtins.get(class) {
            return b.methods.iter().find(|m| m.name == name);
        }
        None
    }

    /// 沿继承链收集虚方法（可 override 的引擎回调）。镜像引擎
    /// `ClassDB::get_virtual_methods`（链式）+ 补全面的名字去重
    /// （gdscript_editor.cpp:3746 `options.has(mi.name)`）：同名先到先得，
    /// 近类覆盖远类（Node._process 胜过 MainLoop._process）。
    pub fn chain_virtuals(&self, class: &str) -> Vec<&Function> {
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls) {
                for m in c.methods.iter().filter(|m| m.is_virtual) {
                    if seen.insert(m.name.clone()) {
                        out.push(m);
                    }
                }
            }
        }
        out
    }

    /// `ClassDB::add_virtual_method` 手工注册的 Object core 虚方法
    /// （object.cpp:1919-1979，`#ifdef TOOLS_ENABLED`——不在
    /// extension_api.json，编辑器构建恒可 override）。
    /// `(name, [(arg, ty)], Option<ret>)`；ret None = void。
    pub fn object_core_virtuals() -> &'static [CoreVirtual] {
        OBJECT_CORE_VIRTUALS
    }

    /// 沿继承链查找属性；内建类型同时查其值成员（Vector2.x）。
    pub fn find_property(&self, class: &str, name: &str) -> Option<&Property> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls)
                && let Some(p) = c.properties.iter().find(|p| p.name == name) {
                    return Some(p);
                }
        }
        self.builtins.get(class).and_then(|b| b.members.iter().find(|p| p.name == name))
    }

    /// 常量查找：类级（含继承链）+ 内建类常量兜底（Vector2.ZERO / ONE）。
    /// 返回 (常量, 声明宿主类)——描述与文档锚点都挂宿主。
    pub fn find_const(&self, class: &str, name: &str) -> Option<(&Constant, &str)> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls)
                && let Some(k) = c.constants.iter().find(|k| k.name == name) {
                    return Some((k, c.name.as_str()));
                }
        }
        self.builtins.get(class).and_then(|b| {
            b.constants.iter().find(|k| k.name == name).map(|k| (k, b.name.as_str()))
        })
    }

    /// 沿继承链查找枚举；内建类型兜底（Vector3.Axis）。
    pub fn find_enum(&self, class: &str, name: &str) -> Option<&Enum> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls)
                && let Some(e) = c.enums.iter().find(|e| e.name == name) {
                    return Some(e);
                }
        }
        self.builtins.get(class).and_then(|b| b.enums.iter().find(|e| e.name == name))
    }

    /// 沿继承链查找信号（Node.ready / CanvasItem.draw / Viewport.size_changed）。
    pub fn find_signal(&self, class: &str, name: &str) -> Option<&EngineSignal> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls)
                && let Some(s) = c.signals.iter().find(|s| s.name == name) {
                    return Some(s);
                }
        }
        None
    }

    /// 枚举值查找：类级枚举值在 `enums[].values`（不在 constants），沿继承链 +
    /// 内建类枚举兜底（Image.FORMAT_RGBA8 / Vector2.Axis.X）。返回 (枚举名, 值, 宿主类)。
    pub fn find_enum_value(&self, class: &str, name: &str) -> Option<(&str, i64, &str)> {
        for cls in self.ancestor_chain(class) {
            if let Some(c) = self.classes.get(cls) {
                for e in &c.enums {
                    if let Some((_, v)) = e.values.iter().find(|(n, _)| *n == name) {
                        return Some((e.name.as_str(), *v, c.name.as_str()));
                    }
                }
            }
        }
        if let Some(b) = self.builtins.get(class) {
            for e in &b.enums {
                if let Some((_, v)) = e.values.iter().find(|(n, _)| *n == name) {
                    return Some((e.name.as_str(), *v, b.name.as_str()));
                }
            }
        }
        None
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
    #[serde(default)]
    signals: Vec<RawSignal>,
}

#[derive(Deserialize)]
struct RawSignal {
    name: String,
    #[serde(default)]
    arguments: Vec<RawParam>,
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
    #[serde(default)]
    constants: Vec<RawConst>,
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
    is_virtual: bool,
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
                        signals: c
                            .signals
                            .into_iter()
                            .map(|s| EngineSignal {
                                name: s.name,
                                arguments: s
                                    .arguments
                                    .into_iter()
                                    .map(|p| Param { name: p.name, ty: p.ty })
                                    .collect(),
                            })
                            .collect(),
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
                        constants: b.constants.into_iter().map(into_const).collect(),
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
        is_virtual: f.is_virtual,
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
