# CameraAttributes

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CameraAttributes**

> Controls camera-specific attributes such as depth of field and exposure override.

<details><summary>官方英文描述（中文精译进行中）</summary>

Controls camera-specific attributes such as depth of field and exposure override.

When used in a WorldEnvironment it provides default settings for exposure, auto-exposure, and depth of field that will be used by all cameras without their own CameraAttributes, including the editor camera. When used in a Camera3D it will override any CameraAttributes set in the WorldEnvironment. When used in VoxelGI or LightmapGI, only the exposure settings will be used.

See also Environment for general 3D environment settings.

This is a pure virtual class that is inherited by CameraAttributesPhysical and CameraAttributesPractical.

</details>

## 方法

### get_auto_exposure_scale {#get_auto_exposure_scale}

```gdscript
float get_auto_exposure_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_auto_exposure_speed {#get_auto_exposure_speed}

```gdscript
float get_auto_exposure_speed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_exposure_multiplier {#get_exposure_multiplier}

```gdscript
float get_exposure_multiplier() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_exposure_sensitivity {#get_exposure_sensitivity}

```gdscript
float get_exposure_sensitivity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_auto_exposure_enabled {#is_auto_exposure_enabled}

```gdscript
bool is_auto_exposure_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_exposure_enabled {#set_auto_exposure_enabled}

```gdscript
void set_auto_exposure_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_exposure_scale {#set_auto_exposure_scale}

```gdscript
void set_auto_exposure_scale(exposure_grey: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_exposure_speed {#set_auto_exposure_speed}

```gdscript
void set_auto_exposure_speed(exposure_speed: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exposure_multiplier {#set_exposure_multiplier}

```gdscript
void set_exposure_multiplier(multiplier: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exposure_sensitivity {#set_exposure_sensitivity}

```gdscript
void set_exposure_sensitivity(sensitivity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### exposure_sensitivity {#exposure_sensitivity}

```gdscript
exposure_sensitivity: float
```

- void set_exposure_sensitivity(value: float)
- float get_exposure_sensitivity()

Sensitivity of camera sensors, measured in ISO. A higher sensitivity results in a brighter image.

If auto_exposure_enabled is true, this can be used as a method of exposure compensation, doubling the value will increase the exposure value (measured in EV100) by 1 stop.

Note: Only available when ProjectSettings.rendering/lights_and_shadows/use_physical_light_units is enabled.

### exposure_multiplier {#exposure_multiplier}

```gdscript
exposure_multiplier: float
```

- void set_exposure_multiplier(value: float)
- float get_exposure_multiplier()

Multiplier for the exposure amount. A higher value results in a brighter image.

### auto_exposure_enabled {#auto_exposure_enabled}

```gdscript
auto_exposure_enabled: bool
```

- void set_auto_exposure_enabled(value: bool)
- bool is_auto_exposure_enabled()

If true, enables the tonemapping auto exposure mode of the scene renderer. If true, the renderer will automatically determine the exposure setting to adapt to the scene's illumination and the observed light.

Note: Auto-exposure is only supported in the Forward+ rendering method, not Mobile or Compatibility.

### auto_exposure_scale {#auto_exposure_scale}

```gdscript
auto_exposure_scale: float
```

- void set_auto_exposure_scale(value: float)
- float get_auto_exposure_scale()

The scale of the auto exposure effect. Affects the intensity of auto exposure.

### auto_exposure_speed {#auto_exposure_speed}

```gdscript
auto_exposure_speed: float
```

- void set_auto_exposure_speed(value: float)
- float get_auto_exposure_speed()

The speed of the auto exposure effect. Affects the time needed for the camera to perform auto exposure.
