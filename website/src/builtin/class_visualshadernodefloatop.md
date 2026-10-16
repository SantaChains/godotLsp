# VisualShaderNodeFloatOp

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeFloatOp**

> Applies operator to two floating-point inputs: a and b.

<details><summary>官方英文描述</summary>

Applies operator to two floating-point inputs: a and b.

</details>

## 方法

### get_operator {#get_operator}

```gdscript
VisualShaderNodeFloatOp.Operator get_operator() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_operator {#set_operator}

```gdscript
void set_operator(op: VisualShaderNodeFloatOp.Operator)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### operator {#operator}

```gdscript
operator: int
```

- void set_operator(value: Operator)
- Operator get_operator()

An operator to be applied to the inputs.

## 枚举

### Operator {#enum-Operator}

```gdscript
enum Operator { OP_ADD = 0, OP_SUB = 1, OP_MUL = 2, OP_DIV = 3, OP_MOD = 4, OP_POW = 5, OP_MAX = 6, OP_MIN = 7, OP_ATAN2 = 8, OP_STEP = 9, OP_ENUM_SIZE = 10 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_ADD {#OP_ADD}

```gdscript
OP_ADD = 0
```

Sums two numbers using a + b.

#### OP_SUB {#OP_SUB}

```gdscript
OP_SUB = 1
```

Subtracts two numbers using a - b.

#### OP_MUL {#OP_MUL}

```gdscript
OP_MUL = 2
```

Multiplies two numbers using a * b.

#### OP_DIV {#OP_DIV}

```gdscript
OP_DIV = 3
```

Divides two numbers using a / b.

#### OP_MOD {#OP_MOD}

```gdscript
OP_MOD = 4
```

Calculates the remainder of two numbers. Translates to mod(a, b) in the Godot Shader Language.

#### OP_POW {#OP_POW}

```gdscript
OP_POW = 5
```

Raises the a to the power of b. Translates to pow(a, b) in the Godot Shader Language.

#### OP_MAX {#OP_MAX}

```gdscript
OP_MAX = 6
```

Returns the greater of two numbers. Translates to max(a, b) in the Godot Shader Language.

#### OP_MIN {#OP_MIN}

```gdscript
OP_MIN = 7
```

Returns the lesser of two numbers. Translates to min(a, b) in the Godot Shader Language.

#### OP_ATAN2 {#OP_ATAN2}

```gdscript
OP_ATAN2 = 8
```

Returns the arc-tangent of the parameters. Translates to atan(a, b) in the Godot Shader Language.

#### OP_STEP {#OP_STEP}

```gdscript
OP_STEP = 9
```

Generates a step function by comparing b(x) to a(edge). Returns 0.0 if x is smaller than edge and otherwise 1.0. Translates to step(a, b) in the Godot Shader Language.

#### OP_ENUM_SIZE {#OP_ENUM_SIZE}

```gdscript
OP_ENUM_SIZE = 10
```

Represents the size of the Operator enum.

---
