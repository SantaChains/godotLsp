# VisualShaderNodeIntOp

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeIntOp**

> Applies operator to two integer inputs: a and b.

<details><summary>官方英文描述（中文精译进行中）</summary>

Applies operator to two integer inputs: a and b.

</details>

## 方法

### get_operator {#get_operator}

```gdscript
VisualShaderNodeIntOp.Operator get_operator() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_operator {#set_operator}

```gdscript
void set_operator(op: VisualShaderNodeIntOp.Operator)
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
enum Operator { OP_ADD = 0, OP_SUB = 1, OP_MUL = 2, OP_DIV = 3, OP_MOD = 4, OP_MAX = 5, OP_MIN = 6, OP_BITWISE_AND = 7, OP_BITWISE_OR = 8, OP_BITWISE_XOR = 9, OP_BITWISE_LEFT_SHIFT = 10, OP_BITWISE_RIGHT_SHIFT = 11, OP_ENUM_SIZE = 12 }
```

Operator OP_ADD = 0

Sums two numbers using a + b.

Operator OP_SUB = 1

Subtracts two numbers using a - b.

Operator OP_MUL = 2

Multiplies two numbers using a * b.

Operator OP_DIV = 3

Divides two numbers using a / b.

Operator OP_MOD = 4

Calculates the remainder of two numbers using a % b.

Operator OP_MAX = 5

Returns the greater of two numbers. Translates to max(a, b) in the Godot Shader Language.

Operator OP_MIN = 6

Returns the lesser of two numbers. Translates to max(a, b) in the Godot Shader Language.

Operator OP_BITWISE_AND = 7

Returns the result of bitwise AND operation on the integer. Translates to a & b in the Godot Shader Language.

Operator OP_BITWISE_OR = 8

Returns the result of bitwise OR operation for two integers. Translates to a | b in the Godot Shader Language.

Operator OP_BITWISE_XOR = 9

Returns the result of bitwise XOR operation for two integers. Translates to a ^ b in the Godot Shader Language.

Operator OP_BITWISE_LEFT_SHIFT = 10

Returns the result of bitwise left shift operation on the integer. Translates to a << b in the Godot Shader Language.

Operator OP_BITWISE_RIGHT_SHIFT = 11

Returns the result of bitwise right shift operation on the integer. Translates to a >> b in the Godot Shader Language.

Operator OP_ENUM_SIZE = 12

Represents the size of the Operator enum.

#### OP_ADD {#OP_ADD}

```gdscript
OP_ADD = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_SUB {#OP_SUB}

```gdscript
OP_SUB = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_MUL {#OP_MUL}

```gdscript
OP_MUL = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_DIV {#OP_DIV}

```gdscript
OP_DIV = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_MOD {#OP_MOD}

```gdscript
OP_MOD = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_MAX {#OP_MAX}

```gdscript
OP_MAX = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_MIN {#OP_MIN}

```gdscript
OP_MIN = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_BITWISE_AND {#OP_BITWISE_AND}

```gdscript
OP_BITWISE_AND = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_BITWISE_OR {#OP_BITWISE_OR}

```gdscript
OP_BITWISE_OR = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_BITWISE_XOR {#OP_BITWISE_XOR}

```gdscript
OP_BITWISE_XOR = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_BITWISE_LEFT_SHIFT {#OP_BITWISE_LEFT_SHIFT}

```gdscript
OP_BITWISE_LEFT_SHIFT = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_BITWISE_RIGHT_SHIFT {#OP_BITWISE_RIGHT_SHIFT}

```gdscript
OP_BITWISE_RIGHT_SHIFT = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_ENUM_SIZE {#OP_ENUM_SIZE}

```gdscript
OP_ENUM_SIZE = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
