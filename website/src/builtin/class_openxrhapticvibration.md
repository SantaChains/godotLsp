# OpenXRHapticVibration

**继承**：[OpenXRHapticBase](class_openxrhapticbase.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRHapticVibration**

> This haptic feedback resource makes it possible to define a vibration based haptic feedback pulse that can be triggered through actions in the OpenXR action map.

<details><summary>官方英文描述</summary>

This haptic feedback resource makes it possible to define a vibration based haptic feedback pulse that can be triggered through actions in the OpenXR action map.

</details>

## 方法

### get_amplitude {#get_amplitude}

```gdscript
float get_amplitude() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_duration {#get_duration}

```gdscript
int get_duration() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_frequency {#get_frequency}

```gdscript
float get_frequency() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_amplitude {#set_amplitude}

```gdscript
void set_amplitude(amplitude: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_duration {#set_duration}

```gdscript
void set_duration(duration: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_frequency {#set_frequency}

```gdscript
void set_frequency(frequency: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### duration {#duration}

```gdscript
duration: int
```

- void set_duration(value: int)
- int get_duration()

The duration of the pulse in nanoseconds. Use -1 for a minimum duration pulse for the current XR runtime.

### frequency {#frequency}

```gdscript
frequency: float
```

- void set_frequency(value: float)
- float get_frequency()

The frequency of the pulse in Hz. 0.0 will let the XR runtime chose an optimal frequency for the device used.

### amplitude {#amplitude}

```gdscript
amplitude: float
```

- void set_amplitude(value: float)
- float get_amplitude()

The amplitude of the pulse between 0.0 and 1.0.
