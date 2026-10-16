# VisualShaderNodeVectorOp

**继承**：[VisualShaderNodeVectorBase](class_visualshadernodevectorbase.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeVectorOp**

> A visual shader node for use of vector operators. Operates on vector a and vector b.

<details><summary>官方英文描述（中文精译进行中）</summary>

A visual shader node for use of vector operators. Operates on vector a and vector b.

</details>

## 方法

### get_operator {#get_operator}

```gdscript
VisualShaderNodeVectorOp.Operator get_operator() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_operator {#set_operator}

```gdscript
void set_operator(op: VisualShaderNodeVectorOp.Operator)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### operator {#operator}

```gdscript
operator: int
```

- void set_operator(value: Operator)
- Operator get_operator()

The operator to be used.

## 枚举

### Operator {#enum-Operator}

```gdscript
enum Operator { OP_ADD = 0, OP_SUB = 1, OP_MUL = 2, OP_DIV = 3, OP_MOD = 4, OP_POW = 5, OP_MAX = 6, OP_MIN = 7, OP_CROSS = 8, OP_ATAN2 = 9, OP_REFLECT = 10, OP_STEP = 11, OP_ENUM_SIZE = 12 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_ADD {#OP_ADD}

```gdscript
OP_ADD = 0
```

Adds two vectors.

#### OP_SUB {#OP_SUB}

```gdscript
OP_SUB = 1
```

Subtracts a vector from a vector.

#### OP_MUL {#OP_MUL}

```gdscript
OP_MUL = 2
```

Multiplies two vectors.

#### OP_DIV {#OP_DIV}

```gdscript
OP_DIV = 3
```

Divides vector by vector.

#### OP_MOD {#OP_MOD}

```gdscript
OP_MOD = 4
```

Returns the remainder of the two vectors.

#### OP_POW {#OP_POW}

```gdscript
OP_POW = 5
```

Returns the value of the first parameter raised to the power of the second, for each component of the vectors.

#### OP_MAX {#OP_MAX}

```gdscript
OP_MAX = 6
```

Returns the greater of two values, for each component of the vectors.

#### OP_MIN {#OP_MIN}

```gdscript
OP_MIN = 7
```

Returns the lesser of two values, for each component of the vectors.

#### OP_CROSS {#OP_CROSS}

```gdscript
OP_CROSS = 8
```

Calculates the cross product of two vectors.

#### OP_ATAN2 {#OP_ATAN2}

```gdscript
OP_ATAN2 = 9
```

Returns the arc-tangent of the parameters.

#### OP_REFLECT {#OP_REFLECT}

```gdscript
OP_REFLECT = 10
```

Returns the vector that points in the direction of reflection. a is incident vector and b is the normal vector.

#### OP_STEP {#OP_STEP}

```gdscript
OP_STEP = 11
```

Vector step operator. Returns 0.0 if a is smaller than b and 1.0 otherwise.

#### OP_ENUM_SIZE {#OP_ENUM_SIZE}

```gdscript
OP_ENUM_SIZE = 12
```

Represents the size of the Operator enum.

---
