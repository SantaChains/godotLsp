# MobileVRInterface

**继承**：[XRInterface](class_xrinterface.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MobileVRInterface**

> This is a generic mobile VR implementation where you need to provide details about the phone and HMD used. It does not rely on any existing framework. This is the most basic interface we have. For the best effect, you need a mobile phone with a gyroscope and accelerometer.

<details><summary>官方英文描述（中文精译进行中）</summary>

This is a generic mobile VR implementation where you need to provide details about the phone and HMD used. It does not rely on any existing framework. This is the most basic interface we have. For the best effect, you need a mobile phone with a gyroscope and accelerometer.

Note that even though there is no positional tracking, the camera will assume the headset is at a height of 1.85 meters. You can change this by setting eye_height.

You can initialize this interface as follows:

```gdscript
var interface = XRServer.find_interface("Native mobile")
if interface and interface.initialize():
    get_viewport().use_xr = true
```

Note: For Android, ProjectSettings.input_devices/sensors/enable_accelerometer, ProjectSettings.input_devices/sensors/enable_gravity, ProjectSettings.input_devices/sensors/enable_gyroscope and ProjectSettings.input_devices/sensors/enable_magnetometer must be enabled.

</details>

## 方法

### get_display_to_lens {#get_display_to_lens}

```gdscript
float get_display_to_lens() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_display_width {#get_display_width}

```gdscript
float get_display_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_eye_height {#get_eye_height}

```gdscript
float get_eye_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_iod {#get_iod}

```gdscript
float get_iod() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_k1 {#get_k1}

```gdscript
float get_k1() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_k2 {#get_k2}

```gdscript
float get_k2() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset_rect {#get_offset_rect}

```gdscript
Rect2 get_offset_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_oversample {#get_oversample}

```gdscript
float get_oversample() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vrs_min_radius {#get_vrs_min_radius}

```gdscript
float get_vrs_min_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vrs_strength {#get_vrs_strength}

```gdscript
float get_vrs_strength() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_display_to_lens {#set_display_to_lens}

```gdscript
void set_display_to_lens(display_to_lens: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_display_width {#set_display_width}

```gdscript
void set_display_width(display_width: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_eye_height {#set_eye_height}

```gdscript
void set_eye_height(eye_height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_iod {#set_iod}

```gdscript
void set_iod(iod: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_k1 {#set_k1}

```gdscript
void set_k1(k: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_k2 {#set_k2}

```gdscript
void set_k2(k: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset_rect {#set_offset_rect}

```gdscript
void set_offset_rect(offset_rect: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_oversample {#set_oversample}

```gdscript
void set_oversample(oversample: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_min_radius {#set_vrs_min_radius}

```gdscript
void set_vrs_min_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_strength {#set_vrs_strength}

```gdscript
void set_vrs_strength(strength: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### eye_height {#eye_height}

```gdscript
eye_height: float
```

- void set_eye_height(value: float)
- float get_eye_height()

The height at which the camera is placed in relation to the ground (i.e. XROrigin3D node).

### iod {#iod}

```gdscript
iod: float
```

- void set_iod(value: float)
- float get_iod()

The interocular distance, also known as the interpupillary distance. The distance between the pupils of the left and right eye.

### display_width {#display_width}

```gdscript
display_width: float
```

- void set_display_width(value: float)
- float get_display_width()

The width of the display in centimeters.

### display_to_lens {#display_to_lens}

```gdscript
display_to_lens: float
```

- void set_display_to_lens(value: float)
- float get_display_to_lens()

The distance between the display and the lenses inside of the device in centimeters.

### offset_rect {#offset_rect}

```gdscript
offset_rect: Rect2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### oversample {#oversample}

```gdscript
oversample: float
```

- void set_oversample(value: float)
- float get_oversample()

The oversample setting. Because of the lens distortion we have to render our buffers at a higher resolution then the screen can natively handle. A value between 1.5 and 2.0 often provides good results but at the cost of performance.

### k1 {#k1}

```gdscript
k1: float
```

- void set_k1(value: float)
- float get_k1()

The k1 lens factor is one of the two constants that define the strength of the lens used and directly influences the lens distortion effect.

### k2 {#k2}

```gdscript
k2: float
```

- void set_k2(value: float)
- float get_k2()

The k2 lens factor, see k1.

### vrs_min_radius {#vrs_min_radius}

```gdscript
vrs_min_radius: float
```

- void set_vrs_min_radius(value: float)
- float get_vrs_min_radius()

The minimum radius around the focal point where full quality is guaranteed if VRS is used as a percentage of screen size.

Note: Mobile and Forward+ renderers only. Requires Viewport.vrs_mode to be set to Viewport.VRS_XR.

### vrs_strength {#vrs_strength}

```gdscript
vrs_strength: float
```

- void set_vrs_strength(value: float)
- float get_vrs_strength()

The strength used to calculate the VRS density map. The greater this value, the more noticeable VRS is. This improves performance at the cost of quality.

Note: Mobile and Forward+ renderers only. Requires Viewport.vrs_mode to be set to Viewport.VRS_XR.
