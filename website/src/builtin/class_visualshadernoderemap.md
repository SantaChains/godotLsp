# VisualShaderNodeRemap

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeRemap**

> Remap will transform the input range into output range, e.g. you can change a 0..1 value to -2..2 etc. See @GlobalScope.remap() for more details.

<details><summary>官方英文描述（中文精译进行中）</summary>

Remap will transform the input range into output range, e.g. you can change a 0..1 value to -2..2 etc. See @GlobalScope.remap() for more details.

</details>

## 方法

### get_op_type {#get_op_type}

```gdscript
VisualShaderNodeRemap.OpType get_op_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_op_type {#set_op_type}

```gdscript
void set_op_type(op_type: VisualShaderNodeRemap.OpType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### op_type {#op_type}

```gdscript
op_type: int
```

- void set_op_type(value: OpType)
- OpType get_op_type()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

## 枚举

### OpType {#enum-OpType}

```gdscript
enum OpType { OP_TYPE_SCALAR = 0, OP_TYPE_VECTOR_2D = 1, OP_TYPE_VECTOR_2D_SCALAR = 2, OP_TYPE_VECTOR_3D = 3, OP_TYPE_VECTOR_3D_SCALAR = 4, OP_TYPE_VECTOR_4D = 5, OP_TYPE_VECTOR_4D_SCALAR = 6, OP_TYPE_MAX = 7 }
```

OpType OP_TYPE_SCALAR = 0

A floating-point scalar type.

OpType OP_TYPE_VECTOR_2D = 1

A 2D vector type.

OpType OP_TYPE_VECTOR_2D_SCALAR = 2

The value port uses a 2D vector type, while the input min, input max, output min, and output max ports use a floating-point scalar type.

OpType OP_TYPE_VECTOR_3D = 3

A 3D vector type.

OpType OP_TYPE_VECTOR_3D_SCALAR = 4

The value port uses a 3D vector type, while the input min, input max, output min, and output max ports use a floating-point scalar type.

OpType OP_TYPE_VECTOR_4D = 5

A 4D vector type.

OpType OP_TYPE_VECTOR_4D_SCALAR = 6

The value port uses a 4D vector type, while the input min, input max, output min, and output max ports use a floating-point scalar type.

OpType OP_TYPE_MAX = 7

Represents the size of the OpType enum.

#### OP_TYPE_SCALAR {#OP_TYPE_SCALAR}

```gdscript
OP_TYPE_SCALAR = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_2D {#OP_TYPE_VECTOR_2D}

```gdscript
OP_TYPE_VECTOR_2D = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_2D_SCALAR {#OP_TYPE_VECTOR_2D_SCALAR}

```gdscript
OP_TYPE_VECTOR_2D_SCALAR = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_3D {#OP_TYPE_VECTOR_3D}

```gdscript
OP_TYPE_VECTOR_3D = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_3D_SCALAR {#OP_TYPE_VECTOR_3D_SCALAR}

```gdscript
OP_TYPE_VECTOR_3D_SCALAR = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_4D {#OP_TYPE_VECTOR_4D}

```gdscript
OP_TYPE_VECTOR_4D = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_4D_SCALAR {#OP_TYPE_VECTOR_4D_SCALAR}

```gdscript
OP_TYPE_VECTOR_4D_SCALAR = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_MAX {#OP_TYPE_MAX}

```gdscript
OP_TYPE_MAX = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
