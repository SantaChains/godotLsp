# VisualShaderNodeFloatFunc

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeFloatFunc**

> Accept a floating-point scalar (x) to the input port and transform it according to function.

<details><summary>官方英文描述（中文精译进行中）</summary>

Accept a floating-point scalar (x) to the input port and transform it according to function.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeFloatFunc.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeFloatFunc.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

A function to be applied to the scalar.

## 枚举

### Function {#enum-Function}

```gdscript
enum Function { FUNC_SIN = 0, FUNC_COS = 1, FUNC_TAN = 2, FUNC_ASIN = 3, FUNC_ACOS = 4, FUNC_ATAN = 5, FUNC_SINH = 6, FUNC_COSH = 7, FUNC_TANH = 8, FUNC_LOG = 9, FUNC_EXP = 10, FUNC_SQRT = 11, FUNC_ABS = 12, FUNC_SIGN = 13, FUNC_FLOOR = 14, FUNC_ROUND = 15, FUNC_CEIL = 16, FUNC_FRACT = 17, FUNC_SATURATE = 18, FUNC_NEGATE = 19, FUNC_ACOSH = 20, FUNC_ASINH = 21, FUNC_ATANH = 22, FUNC_DEGREES = 23, FUNC_EXP2 = 24, FUNC_INVERSE_SQRT = 25, FUNC_LOG2 = 26, FUNC_RADIANS = 27, FUNC_RECIPROCAL = 28, FUNC_ROUNDEVEN = 29, FUNC_TRUNC = 30, FUNC_ONEMINUS = 31, FUNC_MAX = 32 }
```

Function FUNC_SIN = 0

Returns the sine of the parameter. Translates to sin(x) in the Godot Shader Language.

Function FUNC_COS = 1

Returns the cosine of the parameter. Translates to cos(x) in the Godot Shader Language.

Function FUNC_TAN = 2

Returns the tangent of the parameter. Translates to tan(x) in the Godot Shader Language.

Function FUNC_ASIN = 3

Returns the arc-sine of the parameter. Translates to asin(x) in the Godot Shader Language.

Function FUNC_ACOS = 4

Returns the arc-cosine of the parameter. Translates to acos(x) in the Godot Shader Language.

Function FUNC_ATAN = 5

Returns the arc-tangent of the parameter. Translates to atan(x) in the Godot Shader Language.

Function FUNC_SINH = 6

Returns the hyperbolic sine of the parameter. Translates to sinh(x) in the Godot Shader Language.

Function FUNC_COSH = 7

Returns the hyperbolic cosine of the parameter. Translates to cosh(x) in the Godot Shader Language.

Function FUNC_TANH = 8

Returns the hyperbolic tangent of the parameter. Translates to tanh(x) in the Godot Shader Language.

Function FUNC_LOG = 9

Returns the natural logarithm of the parameter. Translates to log(x) in the Godot Shader Language.

Function FUNC_EXP = 10

Returns the natural exponentiation of the parameter. Translates to exp(x) in the Godot Shader Language.

Function FUNC_SQRT = 11

Returns the square root of the parameter. Translates to sqrt(x) in the Godot Shader Language.

Function FUNC_ABS = 12

Returns the absolute value of the parameter. Translates to abs(x) in the Godot Shader Language.

Function FUNC_SIGN = 13

Extracts the sign of the parameter. Translates to sign(x) in the Godot Shader Language.

Function FUNC_FLOOR = 14

Finds the nearest integer less than or equal to the parameter. Translates to floor(x) in the Godot Shader Language.

Function FUNC_ROUND = 15

Finds the nearest integer to the parameter. Translates to round(x) in the Godot Shader Language.

Function FUNC_CEIL = 16

Finds the nearest integer that is greater than or equal to the parameter. Translates to ceil(x) in the Godot Shader Language.

Function FUNC_FRACT = 17

Computes the fractional part of the argument. Translates to fract(x) in the Godot Shader Language.

Function FUNC_SATURATE = 18

Clamps the value between 0.0 and 1.0 using min(max(x, 0.0), 1.0).

Function FUNC_NEGATE = 19

Negates the x using -(x).

Function FUNC_ACOSH = 20

Returns the arc-hyperbolic-cosine of the parameter. Translates to acosh(x) in the Godot Shader Language.

Function FUNC_ASINH = 21

Returns the arc-hyperbolic-sine of the parameter. Translates to asinh(x) in the Godot Shader Language.

Function FUNC_ATANH = 22

Returns the arc-hyperbolic-tangent of the parameter. Translates to atanh(x) in the Godot Shader Language.

Function FUNC_DEGREES = 23

Convert a quantity in radians to degrees. Translates to degrees(x) in the Godot Shader Language.

Function FUNC_EXP2 = 24

Returns 2 raised by the power of the parameter. Translates to exp2(x) in the Godot Shader Language.

Function FUNC_INVERSE_SQRT = 25

Returns the inverse of the square root of the parameter. Translates to inversesqrt(x) in the Godot Shader Language.

Function FUNC_LOG2 = 26

Returns the base 2 logarithm of the parameter. Translates to log2(x) in the Godot Shader Language.

Function FUNC_RADIANS = 27

Convert a quantity in degrees to radians. Translates to radians(x) in the Godot Shader Language.

Function FUNC_RECIPROCAL = 28

Finds reciprocal value of dividing 1 by x (i.e. 1 / x).

Function FUNC_ROUNDEVEN = 29

Finds the nearest even integer to the parameter. Translates to roundEven(x) in the Godot Shader Language.

Function FUNC_TRUNC = 30

Returns a value equal to the nearest integer to x whose absolute value is not larger than the absolute value of x. Translates to trunc(x) in the Godot Shader Language.

Function FUNC_ONEMINUS = 31

Subtracts scalar x from 1 (i.e. 1 - x).

Function FUNC_MAX = 32

Represents the size of the Function enum.

#### FUNC_SIN {#FUNC_SIN}

```gdscript
FUNC_SIN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_COS {#FUNC_COS}

```gdscript
FUNC_COS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_TAN {#FUNC_TAN}

```gdscript
FUNC_TAN = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ASIN {#FUNC_ASIN}

```gdscript
FUNC_ASIN = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ACOS {#FUNC_ACOS}

```gdscript
FUNC_ACOS = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ATAN {#FUNC_ATAN}

```gdscript
FUNC_ATAN = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_SINH {#FUNC_SINH}

```gdscript
FUNC_SINH = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_COSH {#FUNC_COSH}

```gdscript
FUNC_COSH = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_TANH {#FUNC_TANH}

```gdscript
FUNC_TANH = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_LOG {#FUNC_LOG}

```gdscript
FUNC_LOG = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_EXP {#FUNC_EXP}

```gdscript
FUNC_EXP = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_SQRT {#FUNC_SQRT}

```gdscript
FUNC_SQRT = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ABS {#FUNC_ABS}

```gdscript
FUNC_ABS = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_SIGN {#FUNC_SIGN}

```gdscript
FUNC_SIGN = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_FLOOR {#FUNC_FLOOR}

```gdscript
FUNC_FLOOR = 14
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ROUND {#FUNC_ROUND}

```gdscript
FUNC_ROUND = 15
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_CEIL {#FUNC_CEIL}

```gdscript
FUNC_CEIL = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_FRACT {#FUNC_FRACT}

```gdscript
FUNC_FRACT = 17
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_SATURATE {#FUNC_SATURATE}

```gdscript
FUNC_SATURATE = 18
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_NEGATE {#FUNC_NEGATE}

```gdscript
FUNC_NEGATE = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ACOSH {#FUNC_ACOSH}

```gdscript
FUNC_ACOSH = 20
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ASINH {#FUNC_ASINH}

```gdscript
FUNC_ASINH = 21
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ATANH {#FUNC_ATANH}

```gdscript
FUNC_ATANH = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_DEGREES {#FUNC_DEGREES}

```gdscript
FUNC_DEGREES = 23
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_EXP2 {#FUNC_EXP2}

```gdscript
FUNC_EXP2 = 24
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_INVERSE_SQRT {#FUNC_INVERSE_SQRT}

```gdscript
FUNC_INVERSE_SQRT = 25
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_LOG2 {#FUNC_LOG2}

```gdscript
FUNC_LOG2 = 26
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_RADIANS {#FUNC_RADIANS}

```gdscript
FUNC_RADIANS = 27
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_RECIPROCAL {#FUNC_RECIPROCAL}

```gdscript
FUNC_RECIPROCAL = 28
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ROUNDEVEN {#FUNC_ROUNDEVEN}

```gdscript
FUNC_ROUNDEVEN = 29
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_TRUNC {#FUNC_TRUNC}

```gdscript
FUNC_TRUNC = 30
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_ONEMINUS {#FUNC_ONEMINUS}

```gdscript
FUNC_ONEMINUS = 31
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
