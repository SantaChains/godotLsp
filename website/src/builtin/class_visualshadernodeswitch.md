# VisualShaderNodeSwitch

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeSwitch**

> Returns an associated value of the op_type type if the provided boolean value is true or false.

<details><summary>官方英文描述</summary>

Returns an associated value of the op_type type if the provided boolean value is true or false.

</details>

## 方法

### get_op_type {#get_op_type}

```gdscript
VisualShaderNodeSwitch.OpType get_op_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_op_type {#set_op_type}

```gdscript
void set_op_type(type: VisualShaderNodeSwitch.OpType)
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
enum OpType { OP_TYPE_FLOAT = 0, OP_TYPE_INT = 1, OP_TYPE_UINT = 2, OP_TYPE_VECTOR_2D = 3, OP_TYPE_VECTOR_3D = 4, OP_TYPE_VECTOR_4D = 5, OP_TYPE_BOOLEAN = 6, OP_TYPE_TRANSFORM = 7, OP_TYPE_MAX = 8 }
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

#### OP_TYPE_BOOLEAN {#OP_TYPE_BOOLEAN}

```gdscript
OP_TYPE_BOOLEAN = 6
```

A boolean type.

#### OP_TYPE_TRANSFORM {#OP_TYPE_TRANSFORM}

```gdscript
OP_TYPE_TRANSFORM = 7
```

A transform type.

#### OP_TYPE_MAX {#OP_TYPE_MAX}

```gdscript
OP_TYPE_MAX = 8
```

Represents the size of the OpType enum.

---
