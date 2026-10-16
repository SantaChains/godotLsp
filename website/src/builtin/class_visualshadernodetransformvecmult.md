# VisualShaderNodeTransformVecMult

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeTransformVecMult**

> A multiplication operation on a transform (4×4 matrix) and a vector, with support for different multiplication operators.

<details><summary>官方英文描述（中文精译进行中）</summary>

A multiplication operation on a transform (4×4 matrix) and a vector, with support for different multiplication operators.

</details>

## 方法

### get_operator {#get_operator}

```gdscript
VisualShaderNodeTransformVecMult.Operator get_operator() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_operator {#set_operator}

```gdscript
void set_operator(op: VisualShaderNodeTransformVecMult.Operator)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### operator {#operator}

```gdscript
operator: int
```

- void set_operator(value: Operator)
- Operator get_operator()

The multiplication type to be performed.

## 枚举

### Operator {#enum-Operator}

```gdscript
enum Operator { OP_AxB = 0, OP_BxA = 1, OP_3x3_AxB = 2, OP_3x3_BxA = 3, OP_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_AxB {#OP_AxB}

```gdscript
OP_AxB = 0
```

Multiplies transform a by the vector b.

#### OP_BxA {#OP_BxA}

```gdscript
OP_BxA = 1
```

Multiplies vector b by the transform a.

#### OP_3x3_AxB {#OP_3x3_AxB}

```gdscript
OP_3x3_AxB = 2
```

Multiplies transform a by the vector b, skipping the last row and column of the transform.

#### OP_3x3_BxA {#OP_3x3_BxA}

```gdscript
OP_3x3_BxA = 3
```

Multiplies vector b by the transform a, skipping the last row and column of the transform.

#### OP_MAX {#OP_MAX}

```gdscript
OP_MAX = 4
```

Represents the size of the Operator enum.

---
