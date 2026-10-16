# VisualShaderNodeCompare

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeCompare**

> Compares a and b of type by function. Returns a boolean scalar. Translates to if instruction in shader code.

<details><summary>官方英文描述（中文精译进行中）</summary>

Compares a and b of type by function. Returns a boolean scalar. Translates to if instruction in shader code.

</details>

## 方法

### get_comparison_type {#get_comparison_type}

```gdscript
VisualShaderNodeCompare.ComparisonType get_comparison_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_condition {#get_condition}

```gdscript
VisualShaderNodeCompare.Condition get_condition() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_function {#get_function}

```gdscript
VisualShaderNodeCompare.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_comparison_type {#set_comparison_type}

```gdscript
void set_comparison_type(type: VisualShaderNodeCompare.ComparisonType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_condition {#set_condition}

```gdscript
void set_condition(condition: VisualShaderNodeCompare.Condition)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeCompare.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### type {#type}

```gdscript
type: int
```

- void set_comparison_type(value: ComparisonType)
- ComparisonType get_comparison_type()

The type to be used in the comparison.

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

A comparison function.

### condition {#condition}

```gdscript
condition: int
```

- void set_condition(value: Condition)
- Condition get_condition()

Extra condition which is applied if type is set to CTYPE_VECTOR_3D.

## 枚举

### ComparisonType {#enum-ComparisonType}

```gdscript
enum ComparisonType { CTYPE_SCALAR = 0, CTYPE_SCALAR_INT = 1, CTYPE_SCALAR_UINT = 2, CTYPE_VECTOR_2D = 3, CTYPE_VECTOR_3D = 4, CTYPE_VECTOR_4D = 5, CTYPE_BOOLEAN = 6, CTYPE_TRANSFORM = 7, CTYPE_MAX = 8 }
```

ComparisonType CTYPE_SCALAR = 0

A floating-point scalar.

ComparisonType CTYPE_SCALAR_INT = 1

An integer scalar.

ComparisonType CTYPE_SCALAR_UINT = 2

An unsigned integer scalar.

ComparisonType CTYPE_VECTOR_2D = 3

A 2D vector type.

ComparisonType CTYPE_VECTOR_3D = 4

A 3D vector type.

ComparisonType CTYPE_VECTOR_4D = 5

A 4D vector type.

ComparisonType CTYPE_BOOLEAN = 6

A boolean type.

ComparisonType CTYPE_TRANSFORM = 7

A transform (mat4) type.

ComparisonType CTYPE_MAX = 8

Represents the size of the ComparisonType enum.

#### CTYPE_SCALAR {#CTYPE_SCALAR}

```gdscript
CTYPE_SCALAR = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_SCALAR_INT {#CTYPE_SCALAR_INT}

```gdscript
CTYPE_SCALAR_INT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_SCALAR_UINT {#CTYPE_SCALAR_UINT}

```gdscript
CTYPE_SCALAR_UINT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_VECTOR_2D {#CTYPE_VECTOR_2D}

```gdscript
CTYPE_VECTOR_2D = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_VECTOR_3D {#CTYPE_VECTOR_3D}

```gdscript
CTYPE_VECTOR_3D = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_VECTOR_4D {#CTYPE_VECTOR_4D}

```gdscript
CTYPE_VECTOR_4D = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_BOOLEAN {#CTYPE_BOOLEAN}

```gdscript
CTYPE_BOOLEAN = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_TRANSFORM {#CTYPE_TRANSFORM}

```gdscript
CTYPE_TRANSFORM = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CTYPE_MAX {#CTYPE_MAX}

```gdscript
CTYPE_MAX = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Function {#enum-Function}

```gdscript
enum Function { FUNC_EQUAL = 0, FUNC_NOT_EQUAL = 1, FUNC_GREATER_THAN = 2, FUNC_GREATER_THAN_EQUAL = 3, FUNC_LESS_THAN = 4, FUNC_LESS_THAN_EQUAL = 5, FUNC_MAX = 6 }
```

Function FUNC_EQUAL = 0

Comparison for equality (a == b).

Function FUNC_NOT_EQUAL = 1

Comparison for inequality (a != b).

Function FUNC_GREATER_THAN = 2

Comparison for greater than (a > b). Cannot be used if type set to CTYPE_BOOLEAN or CTYPE_TRANSFORM.

Function FUNC_GREATER_THAN_EQUAL = 3

Comparison for greater than or equal (a >= b). Cannot be used if type set to CTYPE_BOOLEAN or CTYPE_TRANSFORM.

Function FUNC_LESS_THAN = 4

Comparison for less than (a < b). Cannot be used if type set to CTYPE_BOOLEAN or CTYPE_TRANSFORM.

Function FUNC_LESS_THAN_EQUAL = 5

Comparison for less than or equal (a <= b). Cannot be used if type set to CTYPE_BOOLEAN or CTYPE_TRANSFORM.

Function FUNC_MAX = 6

Represents the size of the Function enum.

#### FUNC_EQUAL {#FUNC_EQUAL}

```gdscript
FUNC_EQUAL = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_NOT_EQUAL {#FUNC_NOT_EQUAL}

```gdscript
FUNC_NOT_EQUAL = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_GREATER_THAN {#FUNC_GREATER_THAN}

```gdscript
FUNC_GREATER_THAN = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_GREATER_THAN_EQUAL {#FUNC_GREATER_THAN_EQUAL}

```gdscript
FUNC_GREATER_THAN_EQUAL = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_LESS_THAN {#FUNC_LESS_THAN}

```gdscript
FUNC_LESS_THAN = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_LESS_THAN_EQUAL {#FUNC_LESS_THAN_EQUAL}

```gdscript
FUNC_LESS_THAN_EQUAL = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Condition {#enum-Condition}

```gdscript
enum Condition { COND_ALL = 0, COND_ANY = 1, COND_MAX = 2 }
```

Condition COND_ALL = 0

The result will be true if all components in the vector satisfy the comparison condition.

Condition COND_ANY = 1

The result will be true if any component in the vector satisfies the comparison condition.

Condition COND_MAX = 2

Represents the size of the Condition enum.

#### COND_ALL {#COND_ALL}

```gdscript
COND_ALL = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COND_ANY {#COND_ANY}

```gdscript
COND_ANY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COND_MAX {#COND_MAX}

```gdscript
COND_MAX = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
