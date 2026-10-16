# VisualShaderNodeTransformFunc

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeTransformFunc**

> Computes an inverse or transpose function on the provided Transform3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

Computes an inverse or transpose function on the provided Transform3D.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeTransformFunc.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeTransformFunc.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

The function to be computed.

## 枚举

### Function {#enum-Function}

```gdscript
enum Function { FUNC_INVERSE = 0, FUNC_TRANSPOSE = 1, FUNC_MAX = 2 }
```

Function FUNC_INVERSE = 0

Perform the inverse operation on the Transform3D matrix.

Function FUNC_TRANSPOSE = 1

Perform the transpose operation on the Transform3D matrix.

Function FUNC_MAX = 2

Represents the size of the Function enum.

#### FUNC_INVERSE {#FUNC_INVERSE}

```gdscript
FUNC_INVERSE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_TRANSPOSE {#FUNC_TRANSPOSE}

```gdscript
FUNC_TRANSPOSE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
