# OpenXRDpadBindingModifier

**继承**：[OpenXRIPBindingModifier](class_openxripbindingmodifier.md) → [OpenXRBindingModifier](class_openxrbindingmodifier.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRDpadBindingModifier**

> The DPad binding modifier converts an axis input to a dpad output, emulating a DPad. New input paths for each dpad direction will be added to the interaction profile. When bound to actions the DPad emulation will be activated. You should not combine dpad inputs with normal inputs in the same action set for the same control, this will result in an error being returned when suggested bindings are submitted to OpenXR.

<details><summary>官方英文描述（中文精译进行中）</summary>

The DPad binding modifier converts an axis input to a dpad output, emulating a DPad. New input paths for each dpad direction will be added to the interaction profile. When bound to actions the DPad emulation will be activated. You should not combine dpad inputs with normal inputs in the same action set for the same control, this will result in an error being returned when suggested bindings are submitted to OpenXR.

See XR_EXT_dpad_binding [https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_dpad_binding] for in-depth details.

Note: If the DPad binding modifier extension is enabled, all dpad binding paths will be available in the action map. Adding the modifier to an interaction profile allows you to further customize the behavior.

</details>

## 方法

### get_action_set {#get_action_set}

```gdscript
OpenXRActionSet get_action_set() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_center_region {#get_center_region}

```gdscript
float get_center_region() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_input_path {#get_input_path}

```gdscript
String get_input_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_is_sticky {#get_is_sticky}

```gdscript
bool get_is_sticky() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_off_haptic {#get_off_haptic}

```gdscript
OpenXRHapticBase get_off_haptic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_on_haptic {#get_on_haptic}

```gdscript
OpenXRHapticBase get_on_haptic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_threshold {#get_threshold}

```gdscript
float get_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_threshold_released {#get_threshold_released}

```gdscript
float get_threshold_released() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wedge_angle {#get_wedge_angle}

```gdscript
float get_wedge_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_action_set {#set_action_set}

```gdscript
void set_action_set(action_set: OpenXRActionSet)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_center_region {#set_center_region}

```gdscript
void set_center_region(center_region: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_path {#set_input_path}

```gdscript
void set_input_path(input_path: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_is_sticky {#set_is_sticky}

```gdscript
void set_is_sticky(is_sticky: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_off_haptic {#set_off_haptic}

```gdscript
void set_off_haptic(haptic: OpenXRHapticBase)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_on_haptic {#set_on_haptic}

```gdscript
void set_on_haptic(haptic: OpenXRHapticBase)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_threshold {#set_threshold}

```gdscript
void set_threshold(threshold: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_threshold_released {#set_threshold_released}

```gdscript
void set_threshold_released(threshold_released: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wedge_angle {#set_wedge_angle}

```gdscript
void set_wedge_angle(wedge_angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### action_set {#action_set}

```gdscript
action_set: OpenXRActionSet
```

- void set_action_set(value: OpenXRActionSet)
- OpenXRActionSet get_action_set()

Action set for which this dpad binding modifier is active.

### input_path {#input_path}

```gdscript
input_path: String
```

- void set_input_path(value: String)
- String get_input_path()

Input path for this dpad binding modifier.

### threshold {#threshold}

```gdscript
threshold: float
```

- void set_threshold(value: float)
- float get_threshold()

When our input value is equal or larger than this value, our dpad in that direction becomes true. It stays true until it falls under the threshold_released value.

### threshold_released {#threshold_released}

```gdscript
threshold_released: float
```

- void set_threshold_released(value: float)
- float get_threshold_released()

When our input value falls below this, our output becomes false.

### center_region {#center_region}

```gdscript
center_region: float
```

- void set_center_region(value: float)
- float get_center_region()

Center region in which our center position of our dpad return true.

### wedge_angle {#wedge_angle}

```gdscript
wedge_angle: float
```

- void set_wedge_angle(value: float)
- float get_wedge_angle()

The angle of each wedge that identifies the 4 directions of the emulated dpad.

### is_sticky {#is_sticky}

```gdscript
is_sticky: bool
```

- void set_is_sticky(value: bool)
- bool get_is_sticky()

If false, when the joystick enters a new dpad zone this becomes true.

If true, when the joystick remains in active dpad zone, this remains true even if we overlap with another zone.

### on_haptic {#on_haptic}

```gdscript
on_haptic: OpenXRHapticBase
```

- void set_on_haptic(value: OpenXRHapticBase)
- OpenXRHapticBase get_on_haptic()

Haptic pulse to emit when the user presses the input.

### off_haptic {#off_haptic}

```gdscript
off_haptic: OpenXRHapticBase
```

- void set_off_haptic(value: OpenXRHapticBase)
- OpenXRHapticBase get_off_haptic()

Haptic pulse to emit when the user releases the input.
