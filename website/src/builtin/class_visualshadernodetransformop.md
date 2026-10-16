# VisualShaderNodeTransformOp

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeTransformOp**

> Applies operator to two transform (4×4 matrices) inputs.

<details><summary>官方英文描述</summary>

Applies operator to two transform (4×4 matrices) inputs.

</details>

## 方法

### get_operator {#get_operator}

```gdscript
VisualShaderNodeTransformOp.Operator get_operator() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_operator {#set_operator}

```gdscript
void set_operator(op: VisualShaderNodeTransformOp.Operator)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### operator {#operator}

```gdscript
operator: int
```

- void set_operator(value: Operator)
- Operator get_operator()

The type of the operation to be performed on the transforms.

## 枚举

### Operator {#enum-Operator}

```gdscript
enum Operator { OP_AxB = 0, OP_BxA = 1, OP_AxB_COMP = 2, OP_BxA_COMP = 3, OP_ADD = 4, OP_A_MINUS_B = 5, OP_B_MINUS_A = 6, OP_A_DIV_B = 7, OP_B_DIV_A = 8, OP_MAX = 9 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_AxB {#OP_AxB}

```gdscript
OP_AxB = 0
```

Multiplies transform a by the transform b.

#### OP_BxA {#OP_BxA}

```gdscript
OP_BxA = 1
```

Multiplies transform b by the transform a.

#### OP_AxB_COMP {#OP_AxB_COMP}

```gdscript
OP_AxB_COMP = 2
```

Performs a component-wise multiplication of transform a by the transform b.

#### OP_BxA_COMP {#OP_BxA_COMP}

```gdscript
OP_BxA_COMP = 3
```

Performs a component-wise multiplication of transform b by the transform a.

#### OP_ADD {#OP_ADD}

```gdscript
OP_ADD = 4
```

Adds two transforms.

#### OP_A_MINUS_B {#OP_A_MINUS_B}

```gdscript
OP_A_MINUS_B = 5
```

Subtracts the transform a from the transform b.

#### OP_B_MINUS_A {#OP_B_MINUS_A}

```gdscript
OP_B_MINUS_A = 6
```

Subtracts the transform b from the transform a.

#### OP_A_DIV_B {#OP_A_DIV_B}

```gdscript
OP_A_DIV_B = 7
```

Divides the transform a by the transform b.

#### OP_B_DIV_A {#OP_B_DIV_A}

```gdscript
OP_B_DIV_A = 8
```

Divides the transform b by the transform a.

#### OP_MAX {#OP_MAX}

```gdscript
OP_MAX = 9
```

Represents the size of the Operator enum.

---
