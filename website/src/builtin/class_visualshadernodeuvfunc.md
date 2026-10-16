# VisualShaderNodeUVFunc

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeUVFunc**

> UV functions are similar to Vector2 functions, but the input port of this node uses the shader's UV value by default.

<details><summary>官方英文描述（中文精译进行中）</summary>

UV functions are similar to Vector2 functions, but the input port of this node uses the shader's UV value by default.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeUVFunc.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeUVFunc.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

A function to be applied to the texture coordinates.

## 枚举

### Function {#enum-Function}

```gdscript
enum Function { FUNC_PANNING = 0, FUNC_SCALING = 1, FUNC_MAX = 2 }
```

Function FUNC_PANNING = 0

Translates uv by using scale and offset values using the following formula: uv = uv + offset * scale. uv port is connected to UV built-in by default.

Function FUNC_SCALING = 1

Scales uv by using scale and pivot values using the following formula: uv = (uv - pivot) * scale + pivot. uv port is connected to UV built-in by default.

Function FUNC_MAX = 2

Represents the size of the Function enum.

#### FUNC_PANNING {#FUNC_PANNING}

```gdscript
FUNC_PANNING = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_SCALING {#FUNC_SCALING}

```gdscript
FUNC_SCALING = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
