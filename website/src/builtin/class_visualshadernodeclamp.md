# VisualShaderNodeClamp

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeClamp**

> Constrains a value to lie between min and max values.

<details><summary>官方英文描述</summary>

Constrains a value to lie between min and max values.

</details>

## 方法

### get_op_type {#get_op_type}

```gdscript
VisualShaderNodeClamp.OpType get_op_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_op_type {#set_op_type}

```gdscript
void set_op_type(op_type: VisualShaderNodeClamp.OpType)
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
enum OpType { OP_TYPE_FLOAT = 0, OP_TYPE_INT = 1, OP_TYPE_UINT = 2, OP_TYPE_VECTOR_2D = 3, OP_TYPE_VECTOR_3D = 4, OP_TYPE_VECTOR_4D = 5, OP_TYPE_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_FLOAT {#OP_TYPE_FLOAT}

```gdscript
OP_TYPE_FLOAT = 0
```

A floating-point scalar.

#### OP_TYPE_INT {#OP_TYPE_INT}

```gdscript
OP_TYPE_INT = 1
```

An integer scalar.

#### OP_TYPE_UINT {#OP_TYPE_UINT}

```gdscript
OP_TYPE_UINT = 2
```

An unsigned integer scalar.

#### OP_TYPE_VECTOR_2D {#OP_TYPE_VECTOR_2D}

```gdscript
OP_TYPE_VECTOR_2D = 3
```

A 2D vector type.

#### OP_TYPE_VECTOR_3D {#OP_TYPE_VECTOR_3D}

```gdscript
OP_TYPE_VECTOR_3D = 4
```

A 3D vector type.

#### OP_TYPE_VECTOR_4D {#OP_TYPE_VECTOR_4D}

```gdscript
OP_TYPE_VECTOR_4D = 5
```

A 4D vector type.

#### OP_TYPE_MAX {#OP_TYPE_MAX}

```gdscript
OP_TYPE_MAX = 6
```

Represents the size of the OpType enum.

---
