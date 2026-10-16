# GLTFObjectModelProperty

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFObjectModelProperty**

> GLTFObjectModelProperty defines a mapping between a property in the glTF object model and a NodePath in the Godot scene tree. This can be used to animate properties in a glTF file using the KHR_animation_pointer extension, or to access them through an engine-agnostic script such as a behavior graph as defined by the KHR_interactivity extension.

<details><summary>官方英文描述（中文精译进行中）</summary>

GLTFObjectModelProperty defines a mapping between a property in the glTF object model and a NodePath in the Godot scene tree. This can be used to animate properties in a glTF file using the KHR_animation_pointer extension, or to access them through an engine-agnostic script such as a behavior graph as defined by the KHR_interactivity extension.

The glTF property is identified by JSON pointer(s) stored in json_pointers, while the Godot property it maps to is defined by node_paths. In most cases json_pointers and node_paths will each only have one item, but in some cases a single glTF JSON pointer will map to multiple Godot properties, or a single Godot property will be mapped to multiple glTF JSON pointers, or it might be a many-to-many relationship.

Expression objects can be used to define conversions between the data, such as when glTF defines an angle in radians and Godot uses degrees. The object_model_type property defines the type of data stored in the glTF file as defined by the object model, see GLTFObjectModelType for possible values.

</details>

## 方法

### append_node_path {#append_node_path}

```gdscript
void append_node_path(node_path: NodePath)
```

Appends a NodePath to node_paths. This can be used by GLTFDocumentExtension classes to define how a glTF object model property maps to a Godot property, or multiple Godot properties. Prefer using append_path_to_property() for simple cases. Be sure to also call set_types() once (the order does not matter).

### append_path_to_property {#append_path_to_property}

```gdscript
void append_path_to_property(node_path: NodePath, prop_name: StringName)
```

High-level wrapper over append_node_path() that handles the most common cases. It constructs a new NodePath using node_path as a base and appends prop_name to the subpath. Be sure to also call set_types() once (the order does not matter).

### get_accessor_type {#get_accessor_type}

```gdscript
GLTFAccessor.GLTFAccessorType get_accessor_type() const
```

The GLTF accessor type associated with this property's object_model_type. See GLTFAccessor.accessor_type for possible values, and see GLTFObjectModelType for how the object model type maps to accessor types.

### get_gltf_to_godot_expression {#get_gltf_to_godot_expression}

```gdscript
Expression get_gltf_to_godot_expression() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_godot_to_gltf_expression {#get_godot_to_gltf_expression}

```gdscript
Expression get_godot_to_gltf_expression() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_json_pointers {#get_json_pointers}

```gdscript
typedarray::PackedStringArray get_json_pointers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_node_paths {#get_node_paths}

```gdscript
typedarray::NodePath get_node_paths() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_object_model_type {#get_object_model_type}

```gdscript
GLTFObjectModelProperty.GLTFObjectModelType get_object_model_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_variant_type {#get_variant_type}

```gdscript
Variant.Type get_variant_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_json_pointers {#has_json_pointers}

```gdscript
bool has_json_pointers() const
```

Returns true if json_pointers is not empty. This is used during export to determine if a GLTFObjectModelProperty can handle converting a Godot property to a glTF object model property.

### has_node_paths {#has_node_paths}

```gdscript
bool has_node_paths() const
```

Returns true if node_paths is not empty. This is used during import to determine if a GLTFObjectModelProperty can handle converting a glTF object model property to a Godot property.

### set_gltf_to_godot_expression {#set_gltf_to_godot_expression}

```gdscript
void set_gltf_to_godot_expression(gltf_to_godot_expr: Expression)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_godot_to_gltf_expression {#set_godot_to_gltf_expression}

```gdscript
void set_godot_to_gltf_expression(godot_to_gltf_expr: Expression)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_json_pointers {#set_json_pointers}

```gdscript
void set_json_pointers(json_pointers: typedarray::PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_node_paths {#set_node_paths}

```gdscript
void set_node_paths(node_paths: typedarray::NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_object_model_type {#set_object_model_type}

```gdscript
void set_object_model_type(type: GLTFObjectModelProperty.GLTFObjectModelType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_types {#set_types}

```gdscript
void set_types(variant_type: Variant.Type, obj_model_type: GLTFObjectModelProperty.GLTFObjectModelType)
```

Sets the variant_type and object_model_type properties. This is a convenience method to set both properties at once, since they are almost always known at the same time. This method should be called once. Calling it again with the same values will have no effect.

### set_variant_type {#set_variant_type}

```gdscript
void set_variant_type(variant_type: Variant.Type)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### gltf_to_godot_expression {#gltf_to_godot_expression}

```gdscript
gltf_to_godot_expression: Expression
```

- void set_gltf_to_godot_expression(value: Expression)
- Expression get_gltf_to_godot_expression()

If set, this Expression will be used to convert the property value from the glTF object model to the value expected by the Godot property. This is useful when the glTF object model uses a different unit system, or when the data needs to be transformed in some way. If null, the value will be copied as-is.

### godot_to_gltf_expression {#godot_to_gltf_expression}

```gdscript
godot_to_gltf_expression: Expression
```

- void set_godot_to_gltf_expression(value: Expression)
- Expression get_godot_to_gltf_expression()

If set, this Expression will be used to convert the property value from the Godot property to the value expected by the glTF object model. This is useful when the glTF object model uses a different unit system, or when the data needs to be transformed in some way. If null, the value will be copied as-is.

### node_paths {#node_paths}

```gdscript
node_paths: Array
```

- void set_node_paths(value: Array[NodePath])
- Array[NodePath] get_node_paths()

An array of NodePaths that point to a property, or multiple properties, in the Godot scene tree. On import, this will either be set by GLTFDocument, or by a GLTFDocumentExtension class. For simple cases, use append_path_to_property() to add properties to this array.

In most cases node_paths will only have one item, but in some cases a single glTF JSON pointer will map to multiple Godot properties. For example, a GLTFCamera or GLTFLight used on multiple glTF nodes will be represented by multiple Godot nodes.

### object_model_type {#object_model_type}

```gdscript
object_model_type: int
```

- void set_object_model_type(value: GLTFObjectModelType)
- GLTFObjectModelType get_object_model_type()

The type of data stored in the glTF file as defined by the object model. This is a superset of the available accessor types, and determines the accessor type.

### json_pointers {#json_pointers}

```gdscript
json_pointers: PackedStringArray
```

- void set_json_pointers(value: Array[PackedStringArray])
- Array[PackedStringArray] get_json_pointers()

The glTF object model JSON pointers used to identify the property in the glTF object model. In most cases, there will be only one item in this array, but specific cases may require multiple pointers. The items are themselves arrays which represent the JSON pointer split into its components.

### variant_type {#variant_type}

```gdscript
variant_type: int
```

- void set_variant_type(value: Variant.Type)
- Variant.Type get_variant_type()

The type of data stored in the Godot property. This is the type of the property that the node_paths point to.

## 枚举

### GLTFObjectModelType {#enum-GLTFObjectModelType}

```gdscript
enum GLTFObjectModelType { GLTF_OBJECT_MODEL_TYPE_UNKNOWN = 0, GLTF_OBJECT_MODEL_TYPE_BOOL = 1, GLTF_OBJECT_MODEL_TYPE_FLOAT = 2, GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY = 3, GLTF_OBJECT_MODEL_TYPE_FLOAT2 = 4, GLTF_OBJECT_MODEL_TYPE_FLOAT3 = 5, GLTF_OBJECT_MODEL_TYPE_FLOAT4 = 6, GLTF_OBJECT_MODEL_TYPE_FLOAT2X2 = 7, GLTF_OBJECT_MODEL_TYPE_FLOAT3X3 = 8, GLTF_OBJECT_MODEL_TYPE_FLOAT4X4 = 9, GLTF_OBJECT_MODEL_TYPE_INT = 10 }
```

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_UNKNOWN = 0

Unknown or not set object model type. If the object model type is set to this value, the real type still needs to be determined.

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_BOOL = 1

Object model type "bool". Represented in the glTF JSON as a boolean, and encoded in a GLTFAccessor as "SCALAR". When encoded in an accessor, a value of 0 is false, and any other value is true.

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT = 2

Object model type "float". Represented in the glTF JSON as a number, and encoded in a GLTFAccessor as "SCALAR".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY = 3

Object model type "float[]". Represented in the glTF JSON as an array of numbers, and encoded in a GLTFAccessor as "SCALAR".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT2 = 4

Object model type "float2". Represented in the glTF JSON as an array of two numbers, and encoded in a GLTFAccessor as "VEC2".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT3 = 5

Object model type "float3". Represented in the glTF JSON as an array of three numbers, and encoded in a GLTFAccessor as "VEC3".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT4 = 6

Object model type "float4". Represented in the glTF JSON as an array of four numbers, and encoded in a GLTFAccessor as "VEC4".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT2X2 = 7

Object model type "float2x2". Represented in the glTF JSON as an array of four numbers, and encoded in a GLTFAccessor as "MAT2".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT3X3 = 8

Object model type "float3x3". Represented in the glTF JSON as an array of nine numbers, and encoded in a GLTFAccessor as "MAT3".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_FLOAT4X4 = 9

Object model type "float4x4". Represented in the glTF JSON as an array of sixteen numbers, and encoded in a GLTFAccessor as "MAT4".

GLTFObjectModelType GLTF_OBJECT_MODEL_TYPE_INT = 10

Object model type "int". Represented in the glTF JSON as a number, and encoded in a GLTFAccessor as "SCALAR". The range of values is limited to signed integers. For KHR_interactivity, only 32-bit integers are supported.

#### GLTF_OBJECT_MODEL_TYPE_UNKNOWN {#GLTF_OBJECT_MODEL_TYPE_UNKNOWN}

```gdscript
GLTF_OBJECT_MODEL_TYPE_UNKNOWN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_BOOL {#GLTF_OBJECT_MODEL_TYPE_BOOL}

```gdscript
GLTF_OBJECT_MODEL_TYPE_BOOL = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT {#GLTF_OBJECT_MODEL_TYPE_FLOAT}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY {#GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT2 {#GLTF_OBJECT_MODEL_TYPE_FLOAT2}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT2 = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT3 {#GLTF_OBJECT_MODEL_TYPE_FLOAT3}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT3 = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT4 {#GLTF_OBJECT_MODEL_TYPE_FLOAT4}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT4 = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT2X2 {#GLTF_OBJECT_MODEL_TYPE_FLOAT2X2}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT2X2 = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT3X3 {#GLTF_OBJECT_MODEL_TYPE_FLOAT3X3}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT3X3 = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_FLOAT4X4 {#GLTF_OBJECT_MODEL_TYPE_FLOAT4X4}

```gdscript
GLTF_OBJECT_MODEL_TYPE_FLOAT4X4 = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLTF_OBJECT_MODEL_TYPE_INT {#GLTF_OBJECT_MODEL_TYPE_INT}

```gdscript
GLTF_OBJECT_MODEL_TYPE_INT = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
