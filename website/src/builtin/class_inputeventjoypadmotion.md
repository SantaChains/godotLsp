# InputEventJoypadMotion

**继承**：[InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventJoypadMotion**

> Stores information about joystick motions. One InputEventJoypadMotion represents one axis at a time. For gamepad buttons, see InputEventJoypadButton.

<details><summary>官方英文描述</summary>

Stores information about joystick motions. One InputEventJoypadMotion represents one axis at a time. For gamepad buttons, see InputEventJoypadButton.

</details>

## 方法

### get_axis {#get_axis}

```gdscript
JoyAxis get_axis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_axis_value {#get_axis_value}

```gdscript
float get_axis_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_axis {#set_axis}

```gdscript
void set_axis(axis: JoyAxis)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_axis_value {#set_axis_value}

```gdscript
void set_axis_value(axis_value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### axis {#axis}

```gdscript
axis: int
```

- void set_axis(value: JoyAxis)
- JoyAxis get_axis()

Axis identifier.

### axis_value {#axis_value}

```gdscript
axis_value: float
```

- void set_axis_value(value: float)
- float get_axis_value()

Current position of the joystick on the given axis. The value ranges from -1.0 to 1.0. A value of 0 means the axis is in its resting position.
