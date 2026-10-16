# InputEvent

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **InputEvent**

> 输入事件的基类。包含键盘、鼠标、触摸、手柄等事件类型。

<details><summary>官方英文描述</summary>

Abstract base class of all types of input events. See Node._input().

</details>

## 方法

### accumulate {#accumulate}

```gdscript
bool accumulate(with_event: InputEvent)
```

Returns true if the given input event and this input event can be added together (only for events of type InputEventMouseMotion).

The given input event's position, global position and speed will be copied. The resulting relative is a sum of both events. Both events' modifiers have to be identical.

### as_text {#as_text}

```gdscript
String as_text() const
```

Returns a String representation of the event.

### get_action_strength {#get_action_strength}

```gdscript
float get_action_strength(action: StringName, exact_match: bool = false) const
```

Returns a value between 0.0 and 1.0 depending on the given actions' state. Useful for getting the value of events of type InputEventJoypadMotion.

If exact_match is false, it ignores additional input modifiers for InputEventKey and InputEventMouseButton events, and the direction for InputEventJoypadMotion events.

### get_device {#get_device}

```gdscript
int get_device() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_action {#is_action}

```gdscript
bool is_action(action: StringName, exact_match: bool = false) const
```

Returns true if this input event matches a pre-defined action of any type.

If exact_match is false, it ignores additional input modifiers for InputEventKey and InputEventMouseButton events, and the direction for InputEventJoypadMotion events.

### is_action_pressed {#is_action_pressed}

```gdscript
bool is_action_pressed(action: StringName, allow_echo: bool = false, exact_match: bool = false) const
```

Returns true if the given action matches this event and is being pressed (and is not an echo event for InputEventKey events, unless allow_echo is true). Not relevant for events of type InputEventMouseMotion or InputEventScreenDrag.

If exact_match is false, it ignores additional input modifiers for InputEventKey and InputEventMouseButton events, and the direction for InputEventJoypadMotion events.

Note: Due to keyboard ghosting, is_action_pressed() may return false even if one of the action's keys is pressed. See Input examples in the documentation for more information.

### is_action_released {#is_action_released}

```gdscript
bool is_action_released(action: StringName, exact_match: bool = false) const
```

Returns true if the given action matches this event and is released (i.e. not pressed). Not relevant for events of type InputEventMouseMotion or InputEventScreenDrag.

If exact_match is false, it ignores additional input modifiers for InputEventKey and InputEventMouseButton events, and the direction for InputEventJoypadMotion events.

### is_action_type {#is_action_type}

```gdscript
bool is_action_type() const
```

Returns true if this input event's type is one that can be assigned to an input action: InputEventKey, InputEventMouseButton, InputEventJoypadButton, InputEventJoypadMotion, InputEventAction. Returns false for all other input event types.

### is_canceled {#is_canceled}

```gdscript
bool is_canceled() const
```

Returns true if this input event has been canceled.

### is_echo {#is_echo}

```gdscript
bool is_echo() const
```

Returns true if this input event is an echo event (only for events of type InputEventKey). An echo event is a repeated key event sent when the user is holding down the key. Any other event type returns false.

Note: The rate at which echo events are sent is typically around 20 events per second (after holding down the key for roughly half a second). However, the key repeat delay/speed can be changed by the user or disabled entirely in the operating system settings. To ensure your project works correctly on all configurations, do not assume the user has a specific key repeat configuration in your project's behavior.

### is_match {#is_match}

```gdscript
bool is_match(event: InputEvent, exact_match: bool = true) const
```

Returns true if the specified event matches this event. Only valid for action events, which include key (InputEventKey), button (InputEventMouseButton or InputEventJoypadButton), axis InputEventJoypadMotion, and action (InputEventAction) events.

If exact_match is false, the check ignores additional input modifiers for InputEventKey and InputEventMouseButton events, and the direction for InputEventJoypadMotion events.

Note: This method only considers the event configuration (such as the keyboard key or the joypad axis), not state information like is_pressed(), is_released(), is_echo(), or is_canceled().

### is_pressed {#is_pressed}

```gdscript
bool is_pressed() const
```

Returns true if this input event is pressed. Not relevant for events of type InputEventMouseMotion or InputEventScreenDrag.

Note: Due to keyboard ghosting, is_pressed() may return false even if one of the action's keys is pressed. See Input examples in the documentation for more information.

### is_released {#is_released}

```gdscript
bool is_released() const
```

Returns true if this input event is released. Not relevant for events of type InputEventMouseMotion or InputEventScreenDrag.

### set_device {#set_device}

```gdscript
void set_device(device: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### xformed_by {#xformed_by}

```gdscript
InputEvent xformed_by(xform: Transform2D, local_ofs: Vector2 = Vector2(0, 0)) const
```

Returns a copy of the given input event which has been offset by local_ofs and transformed by xform. Relevant for events of type InputEventMouseButton, InputEventMouseMotion, InputEventScreenTouch, InputEventScreenDrag, InputEventMagnifyGesture and InputEventPanGesture.

## 属性

### device {#device}

```gdscript
device: int
```

- void set_device(value: int)
- int get_device()

The event's device ID.

Note: device can be negative for special use cases that don't refer to devices physically present on the system. See DEVICE_ID_EMULATION.

## 常量

### DEVICE_ID_EMULATION {#DEVICE_ID_EMULATION}

```gdscript
const DEVICE_ID_EMULATION = -1
```

Device ID used for emulated mouse input from a touchscreen, or for emulated touch input from a mouse. This can be used to distinguish emulated mouse input from physical mouse input, or emulated touch input from physical touch input.

DEVICE_ID_KEYBOARD = 16 

Device ID used for input from a keyboard. This can be used to distinguish keyboard input events from joypad input events.

DEVICE_ID_MOUSE = 32 

Device ID used for input from a mouse. This can be used to distinguish mouse input events from joypad input events.

### DEVICE_ID_KEYBOARD {#DEVICE_ID_KEYBOARD}

```gdscript
const DEVICE_ID_KEYBOARD = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DEVICE_ID_MOUSE {#DEVICE_ID_MOUSE}

```gdscript
const DEVICE_ID_MOUSE = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
