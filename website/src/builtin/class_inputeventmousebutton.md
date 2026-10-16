# InputEventMouseButton

**继承**：[InputEventMouse](class_inputeventmouse.md) → [InputEventWithModifiers](class_inputeventwithmodifiers.md) → [InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventMouseButton**

> Stores information about mouse click events. See Node._input().

<details><summary>官方英文描述（中文精译进行中）</summary>

Stores information about mouse click events. See Node._input().

Note: On Wear OS devices, rotary input is mapped to @GlobalScope.MOUSE_BUTTON_WHEEL_UP and @GlobalScope.MOUSE_BUTTON_WHEEL_DOWN. This can be changed to @GlobalScope.MOUSE_BUTTON_WHEEL_LEFT and @GlobalScope.MOUSE_BUTTON_WHEEL_RIGHT with the ProjectSettings.input_devices/pointing/android/rotary_input_scroll_axis setting.

</details>

## 方法

### get_button_index {#get_button_index}

```gdscript
MouseButton get_button_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_factor {#get_factor}

```gdscript
float get_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_double_click {#is_double_click}

```gdscript
bool is_double_click() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_button_index {#set_button_index}

```gdscript
void set_button_index(button_index: MouseButton)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_canceled {#set_canceled}

```gdscript
void set_canceled(canceled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_double_click {#set_double_click}

```gdscript
void set_double_click(double_click: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_factor {#set_factor}

```gdscript
void set_factor(factor: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressed {#set_pressed}

```gdscript
void set_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### factor {#factor}

```gdscript
factor: float
```

- void set_factor(value: float)
- float get_factor()

The amount (or delta) of the event. When used for high-precision scroll events, this indicates the scroll amount (vertical or horizontal). This is only supported on some platforms; the reported sensitivity varies depending on the platform. May be 0 if not supported.

### button_index {#button_index}

```gdscript
button_index: int
```

- void set_button_index(value: MouseButton)
- MouseButton get_button_index()

The mouse button identifier, one of the MouseButton button or button wheel constants.

### canceled {#canceled}

```gdscript
canceled: bool
```

- void set_canceled(value: bool)
- bool is_canceled()

If true, the mouse button event has been canceled.

### pressed {#pressed}

```gdscript
pressed: bool
```

- void set_pressed(value: bool)
- bool is_pressed()

If true, the mouse button's state is pressed. If false, the mouse button's state is released.

### double_click {#double_click}

```gdscript
double_click: bool
```

- void set_double_click(value: bool)
- bool is_double_click()

If true, the mouse button's state is a double-click.
