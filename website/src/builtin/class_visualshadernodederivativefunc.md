# VisualShaderNodeDerivativeFunc

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeDerivativeFunc**

> This node is only available in Fragment and Light visual shaders.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node is only available in Fragment and Light visual shaders.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeDerivativeFunc.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_op_type {#get_op_type}

```gdscript
VisualShaderNodeDerivativeFunc.OpType get_op_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_precision {#get_precision}

```gdscript
VisualShaderNodeDerivativeFunc.Precision get_precision() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeDerivativeFunc.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_op_type {#set_op_type}

```gdscript
void set_op_type(type: VisualShaderNodeDerivativeFunc.OpType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_precision {#set_precision}

```gdscript
void set_precision(precision: VisualShaderNodeDerivativeFunc.Precision)
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

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

A derivative function type.

### precision {#precision}

```gdscript
precision: int
```

- void set_precision(value: Precision)
- Precision get_precision()

Sets the level of precision to use for the derivative function. When using the Compatibility renderer, this setting has no effect.

## 枚举

### OpType {#enum-OpType}

```gdscript
enum OpType { OP_TYPE_SCALAR = 0, OP_TYPE_VECTOR_2D = 1, OP_TYPE_VECTOR_3D = 2, OP_TYPE_VECTOR_4D = 3, OP_TYPE_MAX = 4 }
```

OpType OP_TYPE_SCALAR = 0

A floating-point scalar.

OpType OP_TYPE_VECTOR_2D = 1

A 2D vector type.

OpType OP_TYPE_VECTOR_3D = 2

A 3D vector type.

OpType OP_TYPE_VECTOR_4D = 3

A 4D vector type.

OpType OP_TYPE_MAX = 4

Represents the size of the OpType enum.

#### OP_TYPE_SCALAR {#OP_TYPE_SCALAR}

```gdscript
OP_TYPE_SCALAR = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_2D {#OP_TYPE_VECTOR_2D}

```gdscript
OP_TYPE_VECTOR_2D = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_3D {#OP_TYPE_VECTOR_3D}

```gdscript
OP_TYPE_VECTOR_3D = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_VECTOR_4D {#OP_TYPE_VECTOR_4D}

```gdscript
OP_TYPE_VECTOR_4D = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_TYPE_MAX {#OP_TYPE_MAX}

```gdscript
OP_TYPE_MAX = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Function {#enum-Function}

```gdscript
enum Function { FUNC_SUM = 0, FUNC_X = 1, FUNC_Y = 2, FUNC_MAX = 3 }
```

Function FUNC_SUM = 0

Sum of absolute derivative in x and y.

Function FUNC_X = 1

Derivative in x using local differencing.

Function FUNC_Y = 2

Derivative in y using local differencing.

Function FUNC_MAX = 3

Represents the size of the Function enum.

#### FUNC_SUM {#FUNC_SUM}

```gdscript
FUNC_SUM = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_X {#FUNC_X}

```gdscript
FUNC_X = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_Y {#FUNC_Y}

```gdscript
FUNC_Y = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Precision {#enum-Precision}

```gdscript
enum Precision { PRECISION_NONE = 0, PRECISION_COARSE = 1, PRECISION_FINE = 2, PRECISION_MAX = 3 }
```

Precision PRECISION_NONE = 0

No precision is specified, the GPU driver is allowed to use whatever level of precision it chooses. This is the default option and is equivalent to using dFdx() or dFdy() in text shaders.

Precision PRECISION_COARSE = 1

The derivative will be calculated using the current fragment's neighbors (which may not include the current fragment). This tends to be faster than using PRECISION_FINE, but may not be suitable when more precision is needed. This is equivalent to using dFdxCoarse() or dFdyCoarse() in text shaders.

Precision PRECISION_FINE = 2

The derivative will be calculated using the current fragment and its immediate neighbors. This tends to be slower than using PRECISION_COARSE, but may be necessary when more precision is needed. This is equivalent to using dFdxFine() or dFdyFine() in text shaders.

Precision PRECISION_MAX = 3

Represents the size of the Precision enum.

#### PRECISION_NONE {#PRECISION_NONE}

```gdscript
PRECISION_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRECISION_COARSE {#PRECISION_COARSE}

```gdscript
PRECISION_COARSE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRECISION_FINE {#PRECISION_FINE}

```gdscript
PRECISION_FINE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRECISION_MAX {#PRECISION_MAX}

```gdscript
PRECISION_MAX = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
