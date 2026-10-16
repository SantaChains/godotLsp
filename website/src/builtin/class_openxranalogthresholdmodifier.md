# OpenXRAnalogThresholdModifier

**继承**：[OpenXRActionBindingModifier](class_openxractionbindingmodifier.md) → [OpenXRBindingModifier](class_openxrbindingmodifier.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRAnalogThresholdModifier**

> The analog threshold binding modifier can modify a float input to a boolean input with specified thresholds.

<details><summary>官方英文描述（中文精译进行中）</summary>

The analog threshold binding modifier can modify a float input to a boolean input with specified thresholds.

See XR_VALVE_analog_threshold [https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_VALVE_analog_threshold] for in-depth details.

</details>

## 方法

### get_off_haptic {#get_off_haptic}

```gdscript
OpenXRHapticBase get_off_haptic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_off_threshold {#get_off_threshold}

```gdscript
float get_off_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_on_haptic {#get_on_haptic}

```gdscript
OpenXRHapticBase get_on_haptic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_on_threshold {#get_on_threshold}

```gdscript
float get_on_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_off_haptic {#set_off_haptic}

```gdscript
void set_off_haptic(haptic: OpenXRHapticBase)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_off_threshold {#set_off_threshold}

```gdscript
void set_off_threshold(off_threshold: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_on_haptic {#set_on_haptic}

```gdscript
void set_on_haptic(haptic: OpenXRHapticBase)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_on_threshold {#set_on_threshold}

```gdscript
void set_on_threshold(on_threshold: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### on_threshold {#on_threshold}

```gdscript
on_threshold: float
```

- void set_on_threshold(value: float)
- float get_on_threshold()

When our input value is equal or larger than this value, our output becomes true. It stays true until it falls under the off_threshold value.

### off_threshold {#off_threshold}

```gdscript
off_threshold: float
```

- void set_off_threshold(value: float)
- float get_off_threshold()

When our input value falls below this, our output becomes false.

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
