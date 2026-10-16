# InputEventWithModifiers

**继承**：[InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventWithModifiers**

> Stores information about mouse, keyboard, and touch gesture input events. This includes information about which modifier keys are pressed, such as Shift or Alt. See Node._input().

<details><summary>官方英文描述（中文精译进行中）</summary>

Stores information about mouse, keyboard, and touch gesture input events. This includes information about which modifier keys are pressed, such as Shift or Alt. See Node._input().

Note: Modifier keys are considered modifiers only when used in combination with another key. As a result, their corresponding member variables, such as ctrl_pressed, will return false if the key is pressed on its own.

</details>

## 方法

### get_modifiers_mask {#get_modifiers_mask}

```gdscript
bitfield::KeyModifierMask get_modifiers_mask() const
```

Returns the keycode combination of modifier keys.

### is_alt_pressed {#is_alt_pressed}

```gdscript
bool is_alt_pressed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_command_or_control_autoremap {#is_command_or_control_autoremap}

```gdscript
bool is_command_or_control_autoremap() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_command_or_control_pressed {#is_command_or_control_pressed}

```gdscript
bool is_command_or_control_pressed() const
```

On macOS, returns true if Meta (Cmd) is pressed.

On other platforms, returns true if Ctrl is pressed.

### is_ctrl_pressed {#is_ctrl_pressed}

```gdscript
bool is_ctrl_pressed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_meta_pressed {#is_meta_pressed}

```gdscript
bool is_meta_pressed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_shift_pressed {#is_shift_pressed}

```gdscript
bool is_shift_pressed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alt_pressed {#set_alt_pressed}

```gdscript
void set_alt_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_command_or_control_autoremap {#set_command_or_control_autoremap}

```gdscript
void set_command_or_control_autoremap(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ctrl_pressed {#set_ctrl_pressed}

```gdscript
void set_ctrl_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_meta_pressed {#set_meta_pressed}

```gdscript
void set_meta_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shift_pressed {#set_shift_pressed}

```gdscript
void set_shift_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### command_or_control_autoremap {#command_or_control_autoremap}

```gdscript
command_or_control_autoremap: bool
```

- void set_command_or_control_autoremap(value: bool)
- bool is_command_or_control_autoremap()

Automatically use Meta (Cmd) on macOS and Ctrl on other platforms. If true, ctrl_pressed and meta_pressed cannot be set.

### alt_pressed {#alt_pressed}

```gdscript
alt_pressed: bool
```

- void set_alt_pressed(value: bool)
- bool is_alt_pressed()

State of the Alt modifier.

### shift_pressed {#shift_pressed}

```gdscript
shift_pressed: bool
```

- void set_shift_pressed(value: bool)
- bool is_shift_pressed()

State of the Shift modifier.

### ctrl_pressed {#ctrl_pressed}

```gdscript
ctrl_pressed: bool
```

- void set_ctrl_pressed(value: bool)
- bool is_ctrl_pressed()

State of the Ctrl modifier.

### meta_pressed {#meta_pressed}

```gdscript
meta_pressed: bool
```

- void set_meta_pressed(value: bool)
- bool is_meta_pressed()

State of the Meta modifier. On Windows and Linux, this represents the Windows key (sometimes called "meta" or "super" on Linux). On macOS, this represents the Command key.
