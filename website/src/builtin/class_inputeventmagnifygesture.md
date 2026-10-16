# InputEventMagnifyGesture

**继承**：[InputEventGesture](class_inputeventgesture.md) → [InputEventWithModifiers](class_inputeventwithmodifiers.md) → [InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventMagnifyGesture**

> Stores the factor of a magnifying touch gesture. This is usually performed when the user pinches the touch screen and used for zooming in/out.

<details><summary>官方英文描述</summary>

Stores the factor of a magnifying touch gesture. This is usually performed when the user pinches the touch screen and used for zooming in/out.

Note: On Android, this requires the ProjectSettings.input_devices/pointing/android/enable_pan_and_scale_gestures project setting to be enabled.

</details>

## 方法

### get_factor {#get_factor}

```gdscript
float get_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_factor {#set_factor}

```gdscript
void set_factor(factor: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### factor {#factor}

```gdscript
factor: float
```

- void set_factor(value: float)
- float get_factor()

The amount (or delta) of the event. This value is closer to 1.0 the slower the gesture is performed.
