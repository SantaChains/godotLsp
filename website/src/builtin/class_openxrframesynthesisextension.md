# OpenXRFrameSynthesisExtension

**继承**：[OpenXRExtensionWrapper](class_openxrextensionwrapper.md) → [Object](class_object.md) → **OpenXRFrameSynthesisExtension**

> This class implements the OpenXR Frame synthesis extension [https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_frame_synthesis]. When enabled in the project settings and supported by the XR runtime in use, frame synthesis uses advanced reprojection techniques to inject additional frames so that your XR experience hits the full frame rate of the device.

<details><summary>官方英文描述</summary>

This class implements the OpenXR Frame synthesis extension [https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_frame_synthesis]. When enabled in the project settings and supported by the XR runtime in use, frame synthesis uses advanced reprojection techniques to inject additional frames so that your XR experience hits the full frame rate of the device.

</details>

## 方法

### get_relax_frame_interval {#get_relax_frame_interval}

```gdscript
bool get_relax_frame_interval() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_available {#is_available}

```gdscript
bool is_available() const
```

Returns true if frame synthesis is enabled in the project settings and the current XR runtime supports frame synthesis. The value returned will only be valid once OpenXR has been initialized.

### is_enabled {#is_enabled}

```gdscript
bool is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_relax_frame_interval {#set_relax_frame_interval}

```gdscript
void set_relax_frame_interval(relax_frame_interval: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### skip_next_frame {#skip_next_frame}

```gdscript
void skip_next_frame()
```

Queues the next frame to be skipped when supplying motion vector and depth data. Call this after teleporting your player or a similar action has moved the player to prevent incorrect reprojection results due to this movement.

## 属性

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

Enable frame synthesis. When true motion vector and depth data is provided to the XR runtime.

### relax_frame_interval {#relax_frame_interval}

```gdscript
relax_frame_interval: bool
```

- void set_relax_frame_interval(value: bool)
- bool get_relax_frame_interval()

If true this informs the XR runtime we will be providing frames at a greatly reduced rate. Enable this when you expect your application to run at low framerates and wish to inject multiple reprojected frames.
