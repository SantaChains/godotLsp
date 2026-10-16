# VisualShaderNodeVectorFunc

**继承**：[VisualShaderNodeVectorBase](class_visualshadernodevectorbase.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeVectorFunc**

> A visual shader node able to perform different functions using vectors.

<details><summary>官方英文描述</summary>

A visual shader node able to perform different functions using vectors.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeVectorFunc.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeVectorFunc.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

The function to be performed.

## 枚举

### Function {#enum-Function}

```gdscript
enum Function { FUNC_NORMALIZE = 0, FUNC_SATURATE = 1, FUNC_NEGATE = 2, FUNC_RECIPROCAL = 3, FUNC_ABS = 4, FUNC_ACOS = 5, FUNC_ACOSH = 6, FUNC_ASIN = 7, FUNC_ASINH = 8, FUNC_ATAN = 9, FUNC_ATANH = 10, FUNC_CEIL = 11, FUNC_COS = 12, FUNC_COSH = 13, FUNC_DEGREES = 14, FUNC_EXP = 15, FUNC_EXP2 = 16, FUNC_FLOOR = 17, FUNC_FRACT = 18, FUNC_INVERSE_SQRT = 19, FUNC_LOG = 20, FUNC_LOG2 = 21, FUNC_RADIANS = 22, FUNC_ROUND = 23, FUNC_ROUNDEVEN = 24, FUNC_SIGN = 25, FUNC_SIN = 26, FUNC_SINH = 27, FUNC_SQRT = 28, FUNC_TAN = 29, FUNC_TANH = 30, FUNC_TRUNC = 31, FUNC_ONEMINUS = 32, FUNC_MAX = 33 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_NORMALIZE {#FUNC_NORMALIZE}

```gdscript
FUNC_NORMALIZE = 0
```

Normalizes the vector so that it has a length of 1 but points in the same direction.

#### FUNC_SATURATE {#FUNC_SATURATE}

```gdscript
FUNC_SATURATE = 1
```

Clamps the value between 0.0 and 1.0.

#### FUNC_NEGATE {#FUNC_NEGATE}

```gdscript
FUNC_NEGATE = 2
```

Returns the opposite value of the parameter.

#### FUNC_RECIPROCAL {#FUNC_RECIPROCAL}

```gdscript
FUNC_RECIPROCAL = 3
```

Returns 1/vector.

#### FUNC_ABS {#FUNC_ABS}

```gdscript
FUNC_ABS = 4
```

Returns the absolute value of the parameter.

#### FUNC_ACOS {#FUNC_ACOS}

```gdscript
FUNC_ACOS = 5
```

Returns the arc-cosine of the parameter.

#### FUNC_ACOSH {#FUNC_ACOSH}

```gdscript
FUNC_ACOSH = 6
```

Returns the inverse hyperbolic cosine of the parameter.

#### FUNC_ASIN {#FUNC_ASIN}

```gdscript
FUNC_ASIN = 7
```

Returns the arc-sine of the parameter.

#### FUNC_ASINH {#FUNC_ASINH}

```gdscript
FUNC_ASINH = 8
```

Returns the inverse hyperbolic sine of the parameter.

#### FUNC_ATAN {#FUNC_ATAN}

```gdscript
FUNC_ATAN = 9
```

Returns the arc-tangent of the parameter.

#### FUNC_ATANH {#FUNC_ATANH}

```gdscript
FUNC_ATANH = 10
```

Returns the inverse hyperbolic tangent of the parameter.

#### FUNC_CEIL {#FUNC_CEIL}

```gdscript
FUNC_CEIL = 11
```

Finds the nearest integer that is greater than or equal to the parameter.

#### FUNC_COS {#FUNC_COS}

```gdscript
FUNC_COS = 12
```

Returns the cosine of the parameter.

#### FUNC_COSH {#FUNC_COSH}

```gdscript
FUNC_COSH = 13
```

Returns the hyperbolic cosine of the parameter.

#### FUNC_DEGREES {#FUNC_DEGREES}

```gdscript
FUNC_DEGREES = 14
```

Converts a quantity in radians to degrees.

#### FUNC_EXP {#FUNC_EXP}

```gdscript
FUNC_EXP = 15
```

Base-e Exponential.

#### FUNC_EXP2 {#FUNC_EXP2}

```gdscript
FUNC_EXP2 = 16
```

Base-2 Exponential.

#### FUNC_FLOOR {#FUNC_FLOOR}

```gdscript
FUNC_FLOOR = 17
```

Finds the nearest integer less than or equal to the parameter.

#### FUNC_FRACT {#FUNC_FRACT}

```gdscript
FUNC_FRACT = 18
```

Computes the fractional part of the argument.

#### FUNC_INVERSE_SQRT {#FUNC_INVERSE_SQRT}

```gdscript
FUNC_INVERSE_SQRT = 19
```

Returns the inverse of the square root of the parameter.

#### FUNC_LOG {#FUNC_LOG}

```gdscript
FUNC_LOG = 20
```

Natural logarithm.

#### FUNC_LOG2 {#FUNC_LOG2}

```gdscript
FUNC_LOG2 = 21
```

Base-2 logarithm.

#### FUNC_RADIANS {#FUNC_RADIANS}

```gdscript
FUNC_RADIANS = 22
```

Converts a quantity in degrees to radians.

#### FUNC_ROUND {#FUNC_ROUND}

```gdscript
FUNC_ROUND = 23
```

Finds the nearest integer to the parameter.

#### FUNC_ROUNDEVEN {#FUNC_ROUNDEVEN}

```gdscript
FUNC_ROUNDEVEN = 24
```

Finds the nearest even integer to the parameter.

#### FUNC_SIGN {#FUNC_SIGN}

```gdscript
FUNC_SIGN = 25
```

Extracts the sign of the parameter, i.e. returns -1 if the parameter is negative, 1 if it's positive and 0 otherwise.

#### FUNC_SIN {#FUNC_SIN}

```gdscript
FUNC_SIN = 26
```

Returns the sine of the parameter.

#### FUNC_SINH {#FUNC_SINH}

```gdscript
FUNC_SINH = 27
```

Returns the hyperbolic sine of the parameter.

#### FUNC_SQRT {#FUNC_SQRT}

```gdscript
FUNC_SQRT = 28
```

Returns the square root of the parameter.

#### FUNC_TAN {#FUNC_TAN}

```gdscript
FUNC_TAN = 29
```

Returns the tangent of the parameter.

#### FUNC_TANH {#FUNC_TANH}

```gdscript
FUNC_TANH = 30
```

Returns the hyperbolic tangent of the parameter.

#### FUNC_TRUNC {#FUNC_TRUNC}

```gdscript
FUNC_TRUNC = 31
```

Returns a value equal to the nearest integer to the parameter whose absolute value is not larger than the absolute value of the parameter.

#### FUNC_ONEMINUS {#FUNC_ONEMINUS}

```gdscript
FUNC_ONEMINUS = 32
```

Returns 1.0 - vector.

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 33
```

Represents the size of the Function enum.

---
