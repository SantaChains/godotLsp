# InputEventPanGesture

**继承**：[InputEventGesture](class_inputeventgesture.md) → [InputEventWithModifiers](class_inputeventwithmodifiers.md) → [InputEventFromWindow](class_inputeventfromwindow.md) → [InputEvent](class_inputevent.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEventPanGesture**

> Stores information about pan gestures. A pan gesture is performed when the user swipes the touch screen with two fingers. It's typically used for panning/scrolling.

<details><summary>官方英文描述</summary>

Stores information about pan gestures. A pan gesture is performed when the user swipes the touch screen with two fingers. It's typically used for panning/scrolling.

Note: On Android, this requires the ProjectSettings.input_devices/pointing/android/enable_pan_and_scale_gestures project setting to be enabled.

</details>

## 方法

### get_delta {#get_delta}

```gdscript
Vector2 get_delta() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_delta {#set_delta}

```gdscript
void set_delta(delta: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### delta {#delta}

```gdscript
delta: Vector2
```

- void set_delta(value: Vector2)
- Vector2 get_delta()

Panning amount since last pan event.
