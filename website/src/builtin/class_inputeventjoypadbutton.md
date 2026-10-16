# InputEventJoypadButton

**继承**：[InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventJoypadButton**

> Input event type for gamepad buttons. For gamepad analog sticks and joysticks, see InputEventJoypadMotion.

<details><summary>官方英文描述（中文精译进行中）</summary>

Input event type for gamepad buttons. For gamepad analog sticks and joysticks, see InputEventJoypadMotion.

</details>

## 方法

### get_button_index {#get_button_index}

```gdscript
JoyButton get_button_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pressure {#get_pressure}

```gdscript
float get_pressure() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_button_index {#set_button_index}

```gdscript
void set_button_index(button_index: JoyButton)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressed {#set_pressed}

```gdscript
void set_pressed(pressed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pressure {#set_pressure}

```gdscript
void set_pressure(pressure: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### button_index {#button_index}

```gdscript
button_index: int
```

- void set_button_index(value: JoyButton)
- JoyButton get_button_index()

Button identifier. One of the JoyButton button constants.

### pressure {#pressure}

```gdscript
pressure: float
```

- void set_pressure(value: float)
- float get_pressure()

Deprecated: This property is never set by the engine and is always 0.

### pressed {#pressed}

```gdscript
pressed: bool
```

- void set_pressed(value: bool)
- bool is_pressed()

If true, the button's state is pressed. If false, the button's state is released.
