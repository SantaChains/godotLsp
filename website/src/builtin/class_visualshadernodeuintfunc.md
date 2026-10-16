# VisualShaderNodeUIntFunc

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeUIntFunc**

> Accept an unsigned integer scalar (x) to the input port and transform it according to function.

<details><summary>官方英文描述</summary>

Accept an unsigned integer scalar (x) to the input port and transform it according to function.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeUIntFunc.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeUIntFunc.Function)
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
enum Function { FUNC_NEGATE = 0, FUNC_BITWISE_NOT = 1, FUNC_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_NEGATE {#FUNC_NEGATE}

```gdscript
FUNC_NEGATE = 0
```

Negates the x using -(x).

#### FUNC_BITWISE_NOT {#FUNC_BITWISE_NOT}

```gdscript
FUNC_BITWISE_NOT = 1
```

Returns the result of bitwise NOT operation on the integer. Translates to ~a in the Godot Shader Language.

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 2
```

Represents the size of the Function enum.

---
