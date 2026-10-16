# VisualShaderNodeIs

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeIs**

> Returns the boolean result of the comparison between INF or NaN and a scalar parameter.

<details><summary>官方英文描述（中文精译进行中）</summary>

Returns the boolean result of the comparison between INF or NaN and a scalar parameter.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeIs.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeIs.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

The comparison function.

## 枚举

### Function {#enum-Function}

```gdscript
enum Function { FUNC_IS_INF = 0, FUNC_IS_NAN = 1, FUNC_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_IS_INF {#FUNC_IS_INF}

```gdscript
FUNC_IS_INF = 0
```

Comparison with INF (Infinity).

#### FUNC_IS_NAN {#FUNC_IS_NAN}

```gdscript
FUNC_IS_NAN = 1
```

Comparison with NaN (Not a Number; indicates invalid numeric results, such as division by zero).

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 2
```

Represents the size of the Function enum.

---
