# VisualShaderNodeVectorBase

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeVectorBase**

> This is an abstract class. See the derived types for descriptions of the possible operations.

<details><summary>官方英文描述（中文精译进行中）</summary>

This is an abstract class. See the derived types for descriptions of the possible operations.

</details>

## 方法

### get_op_type {#get_op_type}

```gdscript
VisualShaderNodeVectorBase.OpType get_op_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_op_type {#set_op_type}

```gdscript
void set_op_type(type: VisualShaderNodeVectorBase.OpType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### op_type {#op_type}

```gdscript
op_type: int
```

- void set_op_type(value: OpType)
- OpType get_op_type()

A vector type that this operation is performed on.

## 枚举

### OpType {#enum-OpType}

```gdscript
enum OpType { OP_TYPE_VECTOR_2D = 0, OP_TYPE_VECTOR_3D = 1, OP_TYPE_VECTOR_4D = 2, OP_TYPE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_2D {#OP_TYPE_VECTOR_2D}

```gdscript
OP_TYPE_VECTOR_2D = 0
```

A 2D vector type.

#### OP_TYPE_VECTOR_3D {#OP_TYPE_VECTOR_3D}

```gdscript
OP_TYPE_VECTOR_3D = 1
```

A 3D vector type.

#### OP_TYPE_VECTOR_4D {#OP_TYPE_VECTOR_4D}

```gdscript
OP_TYPE_VECTOR_4D = 2
```

A 4D vector type.

#### OP_TYPE_MAX {#OP_TYPE_MAX}

```gdscript
OP_TYPE_MAX = 3
```

Represents the size of the OpType enum.

---
