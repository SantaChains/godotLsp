# VisualShaderNodeFloatParameter

**继承**：[VisualShaderNodeParameter](class_visualshadernodeparameter.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeFloatParameter**

> Translated to uniform float in the shader language.

<details><summary>官方英文描述（中文精译进行中）</summary>

Translated to uniform float in the shader language.

</details>

## 方法

### get_default_value {#get_default_value}

```gdscript
float get_default_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hint {#get_hint}

```gdscript
VisualShaderNodeFloatParameter.Hint get_hint() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max {#get_max}

```gdscript
float get_max() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min {#get_min}

```gdscript
float get_min() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_step {#get_step}

```gdscript
float get_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_default_value_enabled {#is_default_value_enabled}

```gdscript
bool is_default_value_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_value {#set_default_value}

```gdscript
void set_default_value(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_value_enabled {#set_default_value_enabled}

```gdscript
void set_default_value_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hint {#set_hint}

```gdscript
void set_hint(hint: VisualShaderNodeFloatParameter.Hint)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max {#set_max}

```gdscript
void set_max(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min {#set_min}

```gdscript
void set_min(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_step {#set_step}

```gdscript
void set_step(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### hint {#hint}

```gdscript
hint: int
```

- void set_hint(value: Hint)
- Hint get_hint()

A hint applied to the uniform, which controls the values it can take when set through the Inspector.

### min {#min}

```gdscript
min: float
```

- void set_min(value: float)
- float get_min()

Maximum value for range hints. Used if hint is set to HINT_RANGE or HINT_RANGE_STEP.

### max {#max}

```gdscript
max: float
```

- void set_max(value: float)
- float get_max()

Minimum value for range hints. Used if hint is set to HINT_RANGE or HINT_RANGE_STEP.

### step {#step}

```gdscript
step: float
```

- void set_step(value: float)
- float get_step()

Step (increment) value for the range hint with step. Used if hint is set to HINT_RANGE_STEP.

### default_value_enabled {#default_value_enabled}

```gdscript
default_value_enabled: bool
```

- void set_default_value_enabled(value: bool)
- bool is_default_value_enabled()

Enables usage of the default_value.

### default_value {#default_value}

```gdscript
default_value: float
```

- void set_default_value(value: float)
- float get_default_value()

A default value to be assigned within the shader.

## 枚举

### Hint {#enum-Hint}

```gdscript
enum Hint { HINT_NONE = 0, HINT_RANGE = 1, HINT_RANGE_STEP = 2, HINT_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HINT_NONE {#HINT_NONE}

```gdscript
HINT_NONE = 0
```

No hint used.

#### HINT_RANGE {#HINT_RANGE}

```gdscript
HINT_RANGE = 1
```

A range hint for scalar value, which limits possible input values between min and max. Translated to hint_range(min, max) in shader code.

#### HINT_RANGE_STEP {#HINT_RANGE_STEP}

```gdscript
HINT_RANGE_STEP = 2
```

A range hint for scalar value with step, which limits possible input values between min and max, with a step (increment) of step). Translated to hint_range(min, max, step) in shader code.

#### HINT_MAX {#HINT_MAX}

```gdscript
HINT_MAX = 3
```

Represents the size of the Hint enum.

---
