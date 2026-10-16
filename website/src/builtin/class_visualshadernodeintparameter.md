# VisualShaderNodeIntParameter

**继承**：[VisualShaderNodeParameter](class_visualshadernodeparameter.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeIntParameter**

> A VisualShaderNodeParameter of type int. Offers additional customization for range of accepted values.

<details><summary>官方英文描述（中文精译进行中）</summary>

A VisualShaderNodeParameter of type int. Offers additional customization for range of accepted values.

</details>

## 方法

### get_default_value {#get_default_value}

```gdscript
int get_default_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enum_names {#get_enum_names}

```gdscript
PackedStringArray get_enum_names() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hint {#get_hint}

```gdscript
VisualShaderNodeIntParameter.Hint get_hint() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max {#get_max}

```gdscript
int get_max() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min {#get_min}

```gdscript
int get_min() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_step {#get_step}

```gdscript
int get_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_default_value_enabled {#is_default_value_enabled}

```gdscript
bool is_default_value_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_value {#set_default_value}

```gdscript
void set_default_value(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_value_enabled {#set_default_value_enabled}

```gdscript
void set_default_value_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enum_names {#set_enum_names}

```gdscript
void set_enum_names(names: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hint {#set_hint}

```gdscript
void set_hint(hint: VisualShaderNodeIntParameter.Hint)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max {#set_max}

```gdscript
void set_max(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min {#set_min}

```gdscript
void set_min(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_step {#set_step}

```gdscript
void set_step(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### hint {#hint}

```gdscript
hint: int
```

- void set_hint(value: Hint)
- Hint get_hint()

Range hint of this node. Use it to customize valid parameter range.

### min {#min}

```gdscript
min: int
```

- void set_min(value: int)
- int get_min()

The minimum value this parameter can take. hint must be either HINT_RANGE or HINT_RANGE_STEP for this to take effect.

### max {#max}

```gdscript
max: int
```

- void set_max(value: int)
- int get_max()

The maximum value this parameter can take. hint must be either HINT_RANGE or HINT_RANGE_STEP for this to take effect.

### step {#step}

```gdscript
step: int
```

- void set_step(value: int)
- int get_step()

The step between parameter's values. Forces the parameter to be a multiple of the given value. hint must be HINT_RANGE_STEP for this to take effect.

### enum_names {#enum_names}

```gdscript
enum_names: PackedStringArray
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### default_value_enabled {#default_value_enabled}

```gdscript
default_value_enabled: bool
```

- void set_default_value_enabled(value: bool)
- bool is_default_value_enabled()

If true, the node will have a custom default value.

### default_value {#default_value}

```gdscript
default_value: int
```

- void set_default_value(value: int)
- int get_default_value()

Default value of this parameter, which will be used if not set externally. default_value_enabled must be enabled; defaults to 0 otherwise.

## 枚举

### Hint {#enum-Hint}

```gdscript
enum Hint { HINT_NONE = 0, HINT_RANGE = 1, HINT_RANGE_STEP = 2, HINT_ENUM = 3, HINT_MAX = 4 }
```

Hint HINT_NONE = 0

The parameter will not constrain its value.

Hint HINT_RANGE = 1

The parameter's value must be within the specified min/max range.

Hint HINT_RANGE_STEP = 2

The parameter's value must be within the specified range, with the given step between values.

Hint HINT_ENUM = 3

The parameter uses an enum to associate preset values to names in the editor.

Hint HINT_MAX = 4

Represents the size of the Hint enum.

#### HINT_NONE {#HINT_NONE}

```gdscript
HINT_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HINT_RANGE {#HINT_RANGE}

```gdscript
HINT_RANGE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HINT_RANGE_STEP {#HINT_RANGE_STEP}

```gdscript
HINT_RANGE_STEP = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HINT_ENUM {#HINT_ENUM}

```gdscript
HINT_ENUM = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HINT_MAX {#HINT_MAX}

```gdscript
HINT_MAX = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
