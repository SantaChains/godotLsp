# EditorSpinSlider

**继承**：[Range](class_range.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorSpinSlider**

> This Control node is used in the editor's Inspector dock to allow editing of numeric values. Can be used with EditorInspectorPlugin to recreate the same behavior.

<details><summary>官方英文描述（中文精译进行中）</summary>

This Control node is used in the editor's Inspector dock to allow editing of numeric values. Can be used with EditorInspectorPlugin to recreate the same behavior.

If the Range.step value is 1, the EditorSpinSlider will display up/down arrows, similar to SpinBox. If the Range.step value is not 1, a slider will be displayed instead.

</details>

## 方法

### get_control_state {#get_control_state}

```gdscript
EditorSpinSlider.ControlState get_control_state() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_label {#get_label}

```gdscript
String get_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_suffix {#get_suffix}

```gdscript
String get_suffix() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_deferred_drag_mode_enabled {#is_deferred_drag_mode_enabled}

```gdscript
bool is_deferred_drag_mode_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editing_integer {#is_editing_integer}

```gdscript
bool is_editing_integer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_flat {#is_flat}

```gdscript
bool is_flat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hiding_slider {#is_hiding_slider}

```gdscript
bool is_hiding_slider() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_read_only {#is_read_only}

```gdscript
bool is_read_only() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_control_state {#set_control_state}

```gdscript
void set_control_state(state: EditorSpinSlider.ControlState)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deferred_drag_mode_enabled {#set_deferred_drag_mode_enabled}

```gdscript
void set_deferred_drag_mode_enabled(enabled: bool = true)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_editing_integer {#set_editing_integer}

```gdscript
void set_editing_integer(editing_integer: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flat {#set_flat}

```gdscript
void set_flat(flat: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hide_slider {#set_hide_slider}

```gdscript
void set_hide_slider(hide_slider: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_label {#set_label}

```gdscript
void set_label(label: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_read_only {#set_read_only}

```gdscript
void set_read_only(read_only: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_suffix {#set_suffix}

```gdscript
void set_suffix(suffix: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### label {#label}

```gdscript
label: String
```

- void set_label(value: String)
- String get_label()

The text that displays to the left of the value.

### suffix {#suffix}

```gdscript
suffix: String
```

- void set_suffix(value: String)
- String get_suffix()

The suffix to display after the value (in a faded color). This should generally be a plural word. You may have to use an abbreviation if the suffix is too long to be displayed.

### read_only {#read_only}

```gdscript
read_only: bool
```

- void set_read_only(value: bool)
- bool is_read_only()

If true, the slider can't be interacted with.

### flat {#flat}

```gdscript
flat: bool
```

- void set_flat(value: bool)
- bool is_flat()

If true, the slider will not draw background.

### control_state {#control_state}

```gdscript
control_state: bool
```

- void set_control_state(value: ControlState)
- ControlState get_control_state()

The state in which the control used to manipulate the value will be.

### hide_slider {#hide_slider}

```gdscript
hide_slider: bool
```

- void set_hide_slider(value: bool)
- bool is_hiding_slider()

Deprecated: Use control_state instead.

If true, the slider and up/down arrows are hidden.

### editing_integer {#editing_integer}

```gdscript
editing_integer: bool
```

- void set_editing_integer(value: bool)
- bool is_editing_integer()

If true, the EditorSpinSlider is considered to be editing an integer value. If false, the EditorSpinSlider is considered to be editing a floating-point value. This is used to determine whether a slider should be drawn by default. The slider is only drawn for floats; integers use up-down arrows similar to SpinBox instead, unless control_state is set to CONTROL_STATE_PREFER_SLIDER. It will also use EditorSettings.interface/inspector/integer_drag_speed instead of EditorSettings.interface/inspector/float_drag_speed if the slider is available.

### deferred_drag_mode {#deferred_drag_mode}

```gdscript
deferred_drag_mode: bool
```

- void set_deferred_drag_mode_enabled(value: bool)
- bool is_deferred_drag_mode_enabled()

If true, changing via dragging is applied only at the end of the input (for example, when the user releases a mouse button).

## 信号

### grabbed {#grabbed}

```gdscript
signal grabbed()
```

Emitted when the spinner/slider is grabbed.

### ungrabbed {#ungrabbed}

```gdscript
signal ungrabbed()
```

Emitted when the spinner/slider is ungrabbed.

### updown_pressed {#updown_pressed}

```gdscript
signal updown_pressed()
```

Emitted when the updown button is pressed.

### value_focus_entered {#value_focus_entered}

```gdscript
signal value_focus_entered()
```

Emitted when the value form gains focus.

### value_focus_exited {#value_focus_exited}

```gdscript
signal value_focus_exited()
```

Emitted when the value form loses focus.

## 枚举

### ControlState {#enum-ControlState}

```gdscript
enum ControlState { CONTROL_STATE_DEFAULT = 0, CONTROL_STATE_PREFER_SLIDER = 1, CONTROL_STATE_HIDE = 2 }
```

ControlState CONTROL_STATE_DEFAULT = 0

The type of control used will depend on the value of editing_integer. Up-down arrows if true, a slider if false.

ControlState CONTROL_STATE_PREFER_SLIDER = 1

A slider will always be used, even if editing_integer is enabled.

ControlState CONTROL_STATE_HIDE = 2

Neither the up-down arrows nor the slider will be shown.

#### CONTROL_STATE_DEFAULT {#CONTROL_STATE_DEFAULT}

```gdscript
CONTROL_STATE_DEFAULT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONTROL_STATE_PREFER_SLIDER {#CONTROL_STATE_PREFER_SLIDER}

```gdscript
CONTROL_STATE_PREFER_SLIDER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONTROL_STATE_HIDE {#CONTROL_STATE_HIDE}

```gdscript
CONTROL_STATE_HIDE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
