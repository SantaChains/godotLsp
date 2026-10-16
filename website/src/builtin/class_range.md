# Range

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Range**

> Range is an abstract base class for controls that represent a number within a range, using a configured step and page size. See e.g. ScrollBar and Slider for examples of higher-level nodes using Range.

<details><summary>官方英文描述</summary>

Range is an abstract base class for controls that represent a number within a range, using a configured step and page size. See e.g. ScrollBar and Slider for examples of higher-level nodes using Range.

</details>

## 方法

### _value_changed {#_value_changed}

```gdscript
void _value_changed(new_value: float) virtual
```

Called when the Range's value is changed (following the same conditions as value_changed).

### get_as_ratio {#get_as_ratio}

```gdscript
float get_as_ratio() const
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

### get_page {#get_page}

```gdscript
float get_page() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_step {#get_step}

```gdscript
float get_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_value {#get_value}

```gdscript
float get_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_greater_allowed {#is_greater_allowed}

```gdscript
bool is_greater_allowed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_lesser_allowed {#is_lesser_allowed}

```gdscript
bool is_lesser_allowed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_ratio_exp {#is_ratio_exp}

```gdscript
bool is_ratio_exp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_rounded_values {#is_using_rounded_values}

```gdscript
bool is_using_rounded_values() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_greater {#set_allow_greater}

```gdscript
void set_allow_greater(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_lesser {#set_allow_lesser}

```gdscript
void set_allow_lesser(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_as_ratio {#set_as_ratio}

```gdscript
void set_as_ratio(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exp_ratio {#set_exp_ratio}

```gdscript
void set_exp_ratio(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max {#set_max}

```gdscript
void set_max(maximum: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min {#set_min}

```gdscript
void set_min(minimum: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_page {#set_page}

```gdscript
void set_page(pagesize: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_step {#set_step}

```gdscript
void set_step(step: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_rounded_values {#set_use_rounded_values}

```gdscript
void set_use_rounded_values(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_value {#set_value}

```gdscript
void set_value(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_value_no_signal {#set_value_no_signal}

```gdscript
void set_value_no_signal(value: float)
```

Sets the Range's current value to the specified value, without emitting the value_changed signal.

### share {#share}

```gdscript
void share(with: Node)
```

Binds two Ranges together along with any ranges previously grouped with either of them. When any of range's member variables change, it will share the new value with all other ranges in its group.

### unshare {#unshare}

```gdscript
void unshare()
```

Stops the Range from sharing its member variables with any other.

## 属性

### min_value {#min_value}

```gdscript
min_value: float
```

- void set_min(value: float)
- float get_min()

Minimum value. Range is clamped if value is less than min_value.

### max_value {#max_value}

```gdscript
max_value: float
```

- void set_max(value: float)
- float get_max()

Maximum value. Range is clamped if value is greater than max_value.

### step {#step}

```gdscript
step: float
```

- void set_step(value: float)
- float get_step()

If greater than 0.0, value will always be rounded to a multiple of this property's value above min_value. For example, if min_value is 0.1 and step is 0.2, then value is limited to 0.1, 0.3, 0.5, and so on. If rounded is also true, value will first be rounded to a multiple of this property's value, then rounded to the nearest integer.

### page {#page}

```gdscript
page: float
```

- void set_page(value: float)
- float get_page()

Page size. Used mainly for ScrollBar. A ScrollBar's grabber length is the ScrollBar's size multiplied by page over the difference between min_value and max_value.

### value {#value}

```gdscript
value: float
```

- void set_value(value: float)
- float get_value()

Range's current value. Changing this property (even via code) will trigger value_changed signal. Use set_value_no_signal() if you want to avoid it.

### ratio {#ratio}

```gdscript
ratio: float
```

- void set_as_ratio(value: float)
- float get_as_ratio()

The value mapped between 0 and 1.

### exp_edit {#exp_edit}

```gdscript
exp_edit: bool
```

- void set_exp_ratio(value: bool)
- bool is_ratio_exp()

If true, and min_value is greater or equal to 0, value will be represented exponentially rather than linearly.

### rounded {#rounded}

```gdscript
rounded: bool
```

- void set_use_rounded_values(value: bool)
- bool is_using_rounded_values()

If true, value will always be rounded to the nearest integer.

### allow_greater {#allow_greater}

```gdscript
allow_greater: bool
```

- void set_allow_greater(value: bool)
- bool is_greater_allowed()

If true, value may be greater than max_value.

### allow_lesser {#allow_lesser}

```gdscript
allow_lesser: bool
```

- void set_allow_lesser(value: bool)
- bool is_lesser_allowed()

If true, value may be less than min_value.

## 信号

### value_changed {#value_changed}

```gdscript
signal value_changed(value: float)
```

Emitted when value changes. When used on a Slider, this is called continuously while dragging (potentially every frame). If you are performing an expensive operation in a function connected to value_changed, consider using a debouncing Timer to call the function less often.

Note: Unlike signals such as LineEdit.text_changed, value_changed is also emitted when value is set directly via code.

### changed {#changed}

```gdscript
signal changed()
```

Emitted when min_value, max_value, page, or step change.
