# GLTFAccessor

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFAccessor**

> GLTFAccessor is a data structure representing a glTF accessor that would be found in the "accessors" array. A buffer is a blob of binary data. A buffer view is a slice of a buffer. An accessor is a typed interpretation of the data in a buffer view.

<details><summary>官方英文描述（中文精译进行中）</summary>

GLTFAccessor is a data structure representing a glTF accessor that would be found in the "accessors" array. A buffer is a blob of binary data. A buffer view is a slice of a buffer. An accessor is a typed interpretation of the data in a buffer view.

Most custom data stored in glTF does not need accessors, only buffer views (see GLTFBufferView). Accessors are for more advanced use cases such as interleaved mesh data encoded for the GPU.

</details>

## 方法

### from_dictionary {#from_dictionary}

```gdscript
static GLTFAccessor from_dictionary(dictionary: Dictionary)
```

Creates a new GLTFAccessor instance by parsing the given Dictionary.

### get_accessor_type {#get_accessor_type}

```gdscript
GLTFAccessor.GLTFAccessorType get_accessor_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_buffer_view {#get_buffer_view}

```gdscript
int get_buffer_view() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_byte_offset {#get_byte_offset}

```gdscript
int get_byte_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_component_type {#get_component_type}

```gdscript
GLTFAccessor.GLTFComponentType get_component_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_count {#get_count}

```gdscript
int get_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max {#get_max}

```gdscript
PackedFloat64Array get_max() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min {#get_min}

```gdscript
PackedFloat64Array get_min() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_normalized {#get_normalized}

```gdscript
bool get_normalized() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sparse_count {#get_sparse_count}

```gdscript
int get_sparse_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sparse_indices_buffer_view {#get_sparse_indices_buffer_view}

```gdscript
int get_sparse_indices_buffer_view() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sparse_indices_byte_offset {#get_sparse_indices_byte_offset}

```gdscript
int get_sparse_indices_byte_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sparse_indices_component_type {#get_sparse_indices_component_type}

```gdscript
GLTFAccessor.GLTFComponentType get_sparse_indices_component_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sparse_values_buffer_view {#get_sparse_values_buffer_view}

```gdscript
int get_sparse_values_buffer_view() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sparse_values_byte_offset {#get_sparse_values_byte_offset}

```gdscript
int get_sparse_values_byte_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_type {#get_type}

```gdscript
int get_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_accessor_type {#set_accessor_type}

```gdscript
void set_accessor_type(accessor_type: GLTFAccessor.GLTFAccessorType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_buffer_view {#set_buffer_view}

```gdscript
void set_buffer_view(buffer_view: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_byte_offset {#set_byte_offset}

```gdscript
void set_byte_offset(byte_offset: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_component_type {#set_component_type}

```gdscript
void set_component_type(component_type: GLTFAccessor.GLTFComponentType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_count {#set_count}

```gdscript
void set_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max {#set_max}

```gdscript
void set_max(max: PackedFloat64Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min {#set_min}

```gdscript
void set_min(min: PackedFloat64Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_normalized {#set_normalized}

```gdscript
void set_normalized(normalized: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sparse_count {#set_sparse_count}

```gdscript
void set_sparse_count(sparse_count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sparse_indices_buffer_view {#set_sparse_indices_buffer_view}

```gdscript
void set_sparse_indices_buffer_view(sparse_indices_buffer_view: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sparse_indices_byte_offset {#set_sparse_indices_byte_offset}

```gdscript
void set_sparse_indices_byte_offset(sparse_indices_byte_offset: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sparse_indices_component_type {#set_sparse_indices_component_type}

```gdscript
void set_sparse_indices_component_type(sparse_indices_component_type: GLTFAccessor.GLTFComponentType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sparse_values_buffer_view {#set_sparse_values_buffer_view}

```gdscript
void set_sparse_values_buffer_view(sparse_values_buffer_view: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sparse_values_byte_offset {#set_sparse_values_byte_offset}

```gdscript
void set_sparse_values_byte_offset(sparse_values_byte_offset: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_type {#set_type}

```gdscript
void set_type(type: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to_dictionary {#to_dictionary}

```gdscript
Dictionary to_dictionary() const
```

Serializes this GLTFAccessor instance into a Dictionary.

## 属性

### buffer_view {#buffer_view}

```gdscript
buffer_view: int
```

- void set_buffer_view(value: int)
- int get_buffer_view()

The index of the buffer view this accessor is referencing. If -1, this accessor is not referencing any buffer view.

### byte_offset {#byte_offset}

```gdscript
byte_offset: int
```

- void set_byte_offset(value: int)
- int get_byte_offset()

The offset relative to the start of the buffer view in bytes.

### component_type {#component_type}

```gdscript
component_type: int
```

- void set_component_type(value: GLTFComponentType)
- GLTFComponentType get_component_type()

The glTF component type as an enum. See GLTFComponentType for possible values. Within the core glTF specification, a value of 5125 or "UNSIGNED_INT" must not be used for any accessor that is not referenced by mesh.primitive.indices.

### normalized {#normalized}

```gdscript
normalized: bool
```

- void set_normalized(value: bool)
- bool get_normalized()

Specifies whether integer data values are normalized before usage.

### count {#count}

```gdscript
count: int
```

- void set_count(value: int)
- int get_count()

The number of elements referenced by this accessor.

### accessor_type {#accessor_type}

```gdscript
accessor_type: int
```

- void set_accessor_type(value: GLTFAccessorType)
- GLTFAccessorType get_accessor_type()

The glTF accessor type, as an enum.

### type {#type}

```gdscript
type: int
```

- void set_type(value: int)
- int get_type()

Deprecated: Use accessor_type instead.

The glTF accessor type, as an int. Possible values are 0 for "SCALAR", 1 for "VEC2", 2 for "VEC3", 3 for "VEC4", 4 for "MAT2", 5 for "MAT3", and 6 for "MAT4".

### min {#min}

```gdscript
min: PackedFloat64Array
```

- void set_min(value: PackedFloat64Array)
- PackedFloat64Array get_min()

Minimum value of each component in this accessor.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedFloat64Array for more details.

### max {#max}

```gdscript
max: PackedFloat64Array
```

- void set_max(value: PackedFloat64Array)
- PackedFloat64Array get_max()

Maximum value of each component in this accessor.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedFloat64Array for more details.

### sparse_count {#sparse_count}

```gdscript
sparse_count: int
```

- void set_sparse_count(value: int)
- int get_sparse_count()

Number of deviating accessor values stored in the sparse array.

### sparse_indices_buffer_view {#sparse_indices_buffer_view}

```gdscript
sparse_indices_buffer_view: int
```

- void set_sparse_indices_buffer_view(value: int)
- int get_sparse_indices_buffer_view()

The index of the buffer view with sparse indices. The referenced buffer view MUST NOT have its target or byteStride properties defined. The buffer view and the optional byteOffset MUST be aligned to the componentType byte length.

### sparse_indices_byte_offset {#sparse_indices_byte_offset}

```gdscript
sparse_indices_byte_offset: int
```

- void set_sparse_indices_byte_offset(value: int)
- int get_sparse_indices_byte_offset()

The offset relative to the start of the buffer view in bytes.

### sparse_indices_component_type {#sparse_indices_component_type}

```gdscript
sparse_indices_component_type: int
```

- void set_sparse_indices_component_type(value: GLTFComponentType)
- GLTFComponentType get_sparse_indices_component_type()

The indices component data type as an enum. Possible values are 5121 for "UNSIGNED_BYTE", 5123 for "UNSIGNED_SHORT", and 5125 for "UNSIGNED_INT".

### sparse_values_buffer_view {#sparse_values_buffer_view}

```gdscript
sparse_values_buffer_view: int
```

- void set_sparse_values_buffer_view(value: int)
- int get_sparse_values_buffer_view()

The index of the bufferView with sparse values. The referenced buffer view MUST NOT have its target or byteStride properties defined.

### sparse_values_byte_offset {#sparse_values_byte_offset}

```gdscript
sparse_values_byte_offset: int
```

- void set_sparse_values_byte_offset(value: int)
- int get_sparse_values_byte_offset()

The offset relative to the start of the bufferView in bytes.

## 枚举

### GLTFAccessorType {#enum-GLTFAccessorType}

```gdscript
enum GLTFAccessorType { TYPE_SCALAR = 0, TYPE_VEC2 = 1, TYPE_VEC3 = 2, TYPE_VEC4 = 3, TYPE_MAT2 = 4, TYPE_MAT3 = 5, TYPE_MAT4 = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_SCALAR {#TYPE_SCALAR}

```gdscript
TYPE_SCALAR = 0
```

Accessor type "SCALAR". For the glTF object model, this can be used to map to a single float, int, or bool value, or a float array.

#### TYPE_VEC2 {#TYPE_VEC2}

```gdscript
TYPE_VEC2 = 1
```

Accessor type "VEC2". For the glTF object model, this maps to "float2", represented in the glTF JSON as an array of two floats.

#### TYPE_VEC3 {#TYPE_VEC3}

```gdscript
TYPE_VEC3 = 2
```

Accessor type "VEC3". For the glTF object model, this maps to "float3", represented in the glTF JSON as an array of three floats.

#### TYPE_VEC4 {#TYPE_VEC4}

```gdscript
TYPE_VEC4 = 3
```

Accessor type "VEC4". For the glTF object model, this maps to "float4", represented in the glTF JSON as an array of four floats.

#### TYPE_MAT2 {#TYPE_MAT2}

```gdscript
TYPE_MAT2 = 4
```

Accessor type "MAT2". For the glTF object model, this maps to "float2x2", represented in the glTF JSON as an array of four floats.

#### TYPE_MAT3 {#TYPE_MAT3}

```gdscript
TYPE_MAT3 = 5
```

Accessor type "MAT3". For the glTF object model, this maps to "float3x3", represented in the glTF JSON as an array of nine floats.

#### TYPE_MAT4 {#TYPE_MAT4}

```gdscript
TYPE_MAT4 = 6
```

Accessor type "MAT4". For the glTF object model, this maps to "float4x4", represented in the glTF JSON as an array of sixteen floats.

---

### GLTFComponentType {#enum-GLTFComponentType}

```gdscript
enum GLTFComponentType { COMPONENT_TYPE_NONE = 0, COMPONENT_TYPE_SIGNED_BYTE = 5120, COMPONENT_TYPE_UNSIGNED_BYTE = 5121, COMPONENT_TYPE_SIGNED_SHORT = 5122, COMPONENT_TYPE_UNSIGNED_SHORT = 5123, COMPONENT_TYPE_SIGNED_INT = 5124, COMPONENT_TYPE_UNSIGNED_INT = 5125, COMPONENT_TYPE_SINGLE_FLOAT = 5126, COMPONENT_TYPE_DOUBLE_FLOAT = 5130, COMPONENT_TYPE_HALF_FLOAT = 5131, COMPONENT_TYPE_SIGNED_LONG = 5134, COMPONENT_TYPE_UNSIGNED_LONG = 5135 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COMPONENT_TYPE_NONE {#COMPONENT_TYPE_NONE}

```gdscript
COMPONENT_TYPE_NONE = 0
```

Component type "NONE". This is not a valid component type, and is used to indicate that the component type is not set.

#### COMPONENT_TYPE_SIGNED_BYTE {#COMPONENT_TYPE_SIGNED_BYTE}

```gdscript
COMPONENT_TYPE_SIGNED_BYTE = 5120
```

Component type "BYTE". The value is 0x1400 which comes from OpenGL. This indicates data is stored in 1-byte or 8-bit signed integers. This is a core part of the glTF specification.

#### COMPONENT_TYPE_UNSIGNED_BYTE {#COMPONENT_TYPE_UNSIGNED_BYTE}

```gdscript
COMPONENT_TYPE_UNSIGNED_BYTE = 5121
```

Component type "UNSIGNED_BYTE". The value is 0x1401 which comes from OpenGL. This indicates data is stored in 1-byte or 8-bit unsigned integers. This is a core part of the glTF specification.

#### COMPONENT_TYPE_SIGNED_SHORT {#COMPONENT_TYPE_SIGNED_SHORT}

```gdscript
COMPONENT_TYPE_SIGNED_SHORT = 5122
```

Component type "SHORT". The value is 0x1402 which comes from OpenGL. This indicates data is stored in 2-byte or 16-bit signed integers. This is a core part of the glTF specification.

#### COMPONENT_TYPE_UNSIGNED_SHORT {#COMPONENT_TYPE_UNSIGNED_SHORT}

```gdscript
COMPONENT_TYPE_UNSIGNED_SHORT = 5123
```

Component type "UNSIGNED_SHORT". The value is 0x1403 which comes from OpenGL. This indicates data is stored in 2-byte or 16-bit unsigned integers. This is a core part of the glTF specification.

#### COMPONENT_TYPE_SIGNED_INT {#COMPONENT_TYPE_SIGNED_INT}

```gdscript
COMPONENT_TYPE_SIGNED_INT = 5124
```

Component type "INT". The value is 0x1404 which comes from OpenGL. This indicates data is stored in 4-byte or 32-bit signed integers. This is NOT a core part of the glTF specification, and may not be supported by all glTF importers. May be used by some extensions including KHR_interactivity.

#### COMPONENT_TYPE_UNSIGNED_INT {#COMPONENT_TYPE_UNSIGNED_INT}

```gdscript
COMPONENT_TYPE_UNSIGNED_INT = 5125
```

Component type "UNSIGNED_INT". The value is 0x1405 which comes from OpenGL. This indicates data is stored in 4-byte or 32-bit unsigned integers. This is a core part of the glTF specification.

#### COMPONENT_TYPE_SINGLE_FLOAT {#COMPONENT_TYPE_SINGLE_FLOAT}

```gdscript
COMPONENT_TYPE_SINGLE_FLOAT = 5126
```

Component type "FLOAT". The value is 0x1406 which comes from OpenGL. This indicates data is stored in 4-byte or 32-bit floating-point numbers. This is a core part of the glTF specification.

#### COMPONENT_TYPE_DOUBLE_FLOAT {#COMPONENT_TYPE_DOUBLE_FLOAT}

```gdscript
COMPONENT_TYPE_DOUBLE_FLOAT = 5130
```

Component type "DOUBLE". The value is 0x140A which comes from OpenGL. This indicates data is stored in 8-byte or 64-bit floating-point numbers. This is NOT a core part of the glTF specification, and may not be supported by all glTF importers. May be used by some extensions including KHR_interactivity.

#### COMPONENT_TYPE_HALF_FLOAT {#COMPONENT_TYPE_HALF_FLOAT}

```gdscript
COMPONENT_TYPE_HALF_FLOAT = 5131
```

Component type "HALF_FLOAT". The value is 0x140B which comes from OpenGL. This indicates data is stored in 2-byte or 16-bit floating-point numbers. This is NOT a core part of the glTF specification, and may not be supported by all glTF importers. May be used by some extensions including KHR_interactivity.

#### COMPONENT_TYPE_SIGNED_LONG {#COMPONENT_TYPE_SIGNED_LONG}

```gdscript
COMPONENT_TYPE_SIGNED_LONG = 5134
```

Component type "LONG". The value is 0x140E which comes from OpenGL. This indicates data is stored in 8-byte or 64-bit signed integers. This is NOT a core part of the glTF specification, and may not be supported by all glTF importers. May be used by some extensions including KHR_interactivity.

#### COMPONENT_TYPE_UNSIGNED_LONG {#COMPONENT_TYPE_UNSIGNED_LONG}

```gdscript
COMPONENT_TYPE_UNSIGNED_LONG = 5135
```

Component type "UNSIGNED_LONG". The value is 0x140F which comes from OpenGL. This indicates data is stored in 8-byte or 64-bit unsigned integers. This is NOT a core part of the glTF specification, and may not be supported by all glTF importers. May be used by some extensions including KHR_interactivity.

---
