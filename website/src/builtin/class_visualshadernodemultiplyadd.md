# VisualShaderNodeMultiplyAdd

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeMultiplyAdd**

> Uses three operands to compute (a * b + c) expression.

<details><summary>官方英文描述（中文精译进行中）</summary>

Uses three operands to compute (a * b + c) expression.

</details>

## 方法

### get_op_type {#get_op_type}

```gdscript
VisualShaderNodeMultiplyAdd.OpType get_op_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_op_type {#set_op_type}

```gdscript
void set_op_type(type: VisualShaderNodeMultiplyAdd.OpType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### op_type {#op_type}

```gdscript
op_type: int
```

- void set_op_type(value: OpType)
- OpType get_op_type()

A type of operands and returned value.

## 枚举

### OpType {#enum-OpType}

```gdscript
enum OpType { OP_TYPE_SCALAR = 0, OP_TYPE_VECTOR_2D = 1, OP_TYPE_VECTOR_3D = 2, OP_TYPE_VECTOR_4D = 3, OP_TYPE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_SCALAR {#OP_TYPE_SCALAR}

```gdscript
OP_TYPE_SCALAR = 0
```

A floating-point scalar type.

#### OP_TYPE_VECTOR_2D {#OP_TYPE_VECTOR_2D}

```gdscript
OP_TYPE_VECTOR_2D = 1
```

A 2D vector type.

#### OP_TYPE_VECTOR_3D {#OP_TYPE_VECTOR_3D}

```gdscript
OP_TYPE_VECTOR_3D = 2
```

A 3D vector type.

#### OP_TYPE_VECTOR_4D {#OP_TYPE_VECTOR_4D}

```gdscript
OP_TYPE_VECTOR_4D = 3
```

A 4D vector type.

#### OP_TYPE_MAX {#OP_TYPE_MAX}

```gdscript
OP_TYPE_MAX = 4
```

Represents the size of the OpType enum.

---
