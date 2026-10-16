# CameraAttributesPractical

**继承**：[CameraAttributes](class_cameraattributes.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CameraAttributesPractical**

> Controls camera-specific attributes such as auto-exposure, depth of field, and exposure override.

<details><summary>官方英文描述</summary>

Controls camera-specific attributes such as auto-exposure, depth of field, and exposure override.

When used in a WorldEnvironment it provides default settings for exposure, auto-exposure, and depth of field that will be used by all cameras without their own CameraAttributes, including the editor camera. When used in a Camera3D it will override any CameraAttributes set in the WorldEnvironment. When used in VoxelGI or LightmapGI, only the exposure settings will be used.

Note: Depth of field blur is only supported in the Forward+ and Mobile rendering methods, not Compatibility.

Note: Auto-exposure is only supported in the Forward+ rendering method, not Mobile or Compatibility.

</details>

## 方法

### get_auto_exposure_max_sensitivity {#get_auto_exposure_max_sensitivity}

```gdscript
float get_auto_exposure_max_sensitivity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_auto_exposure_min_sensitivity {#get_auto_exposure_min_sensitivity}

```gdscript
float get_auto_exposure_min_sensitivity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dof_blur_amount {#get_dof_blur_amount}

```gdscript
float get_dof_blur_amount() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dof_blur_far_distance {#get_dof_blur_far_distance}

```gdscript
float get_dof_blur_far_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dof_blur_far_transition {#get_dof_blur_far_transition}

```gdscript
float get_dof_blur_far_transition() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dof_blur_near_distance {#get_dof_blur_near_distance}

```gdscript
float get_dof_blur_near_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dof_blur_near_transition {#get_dof_blur_near_transition}

```gdscript
float get_dof_blur_near_transition() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_dof_blur_far_enabled {#is_dof_blur_far_enabled}

```gdscript
bool is_dof_blur_far_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_dof_blur_near_enabled {#is_dof_blur_near_enabled}

```gdscript
bool is_dof_blur_near_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_exposure_max_sensitivity {#set_auto_exposure_max_sensitivity}

```gdscript
void set_auto_exposure_max_sensitivity(max_sensitivity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_exposure_min_sensitivity {#set_auto_exposure_min_sensitivity}

```gdscript
void set_auto_exposure_min_sensitivity(min_sensitivity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dof_blur_amount {#set_dof_blur_amount}

```gdscript
void set_dof_blur_amount(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dof_blur_far_distance {#set_dof_blur_far_distance}

```gdscript
void set_dof_blur_far_distance(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dof_blur_far_enabled {#set_dof_blur_far_enabled}

```gdscript
void set_dof_blur_far_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dof_blur_far_transition {#set_dof_blur_far_transition}

```gdscript
void set_dof_blur_far_transition(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dof_blur_near_distance {#set_dof_blur_near_distance}

```gdscript
void set_dof_blur_near_distance(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dof_blur_near_enabled {#set_dof_blur_near_enabled}

```gdscript
void set_dof_blur_near_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dof_blur_near_transition {#set_dof_blur_near_transition}

```gdscript
void set_dof_blur_near_transition(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### dof_blur_far_enabled {#dof_blur_far_enabled}

```gdscript
dof_blur_far_enabled: bool
```

- void set_dof_blur_far_enabled(value: bool)
- bool is_dof_blur_far_enabled()

Enables depth of field blur for objects further than dof_blur_far_distance. Strength of blur is controlled by dof_blur_amount and modulated by dof_blur_far_transition.

Note: Depth of field blur is only supported in the Forward+ and Mobile rendering methods, not Compatibility.

Note: Depth of field blur is not supported on viewports that have a transparent background (where Viewport.transparent_bg is true).

### dof_blur_far_distance {#dof_blur_far_distance}

```gdscript
dof_blur_far_distance: float
```

- void set_dof_blur_far_distance(value: float)
- float get_dof_blur_far_distance()

Objects further from the Camera3D by this amount will be blurred by the depth of field effect. Measured in meters.

### dof_blur_far_transition {#dof_blur_far_transition}

```gdscript
dof_blur_far_transition: float
```

- void set_dof_blur_far_transition(value: float)
- float get_dof_blur_far_transition()

When positive, distance over which (starting from dof_blur_far_distance) blur effect will scale from 0 to dof_blur_amount. When negative, uses physically-based scaling so depth of field effect will scale from 0 at dof_blur_far_distance and will increase in a physically accurate way as objects get further from the Camera3D.

### dof_blur_near_enabled {#dof_blur_near_enabled}

```gdscript
dof_blur_near_enabled: bool
```

- void set_dof_blur_near_enabled(value: bool)
- bool is_dof_blur_near_enabled()

Enables depth of field blur for objects closer than dof_blur_near_distance. Strength of blur is controlled by dof_blur_amount and modulated by dof_blur_near_transition.

Note: Depth of field blur is only supported in the Forward+ and Mobile rendering methods, not Compatibility.

Note: Depth of field blur is not supported on viewports that have a transparent background (where Viewport.transparent_bg is true).

### dof_blur_near_distance {#dof_blur_near_distance}

```gdscript
dof_blur_near_distance: float
```

- void set_dof_blur_near_distance(value: float)
- float get_dof_blur_near_distance()

Objects closer from the Camera3D by this amount will be blurred by the depth of field effect. Measured in meters.

### dof_blur_near_transition {#dof_blur_near_transition}

```gdscript
dof_blur_near_transition: float
```

- void set_dof_blur_near_transition(value: float)
- float get_dof_blur_near_transition()

When positive, distance over which blur effect will scale from 0 to dof_blur_amount, ending at dof_blur_near_distance. When negative, uses physically-based scaling so depth of field effect will scale from 0 at dof_blur_near_distance and will increase in a physically accurate way as objects get closer to the Camera3D.

### dof_blur_amount {#dof_blur_amount}

```gdscript
dof_blur_amount: float
```

- void set_dof_blur_amount(value: float)
- float get_dof_blur_amount()

Sets the maximum amount of blur. When using physically-based blur amounts, will instead act as a multiplier. High values lead to an increased amount of blurriness, but can be much more expensive to calculate. It is best to keep this as low as possible for a given art style.

### auto_exposure_min_sensitivity {#auto_exposure_min_sensitivity}

```gdscript
auto_exposure_min_sensitivity: float
```

- void set_auto_exposure_min_sensitivity(value: float)
- float get_auto_exposure_min_sensitivity()

The minimum sensitivity (in ISO) used when calculating auto exposure. When calculating scene average luminance, color values will be clamped to at least this value. This limits the auto-exposure from exposing above a certain brightness, resulting in a cut off point where the scene will remain dark.

### auto_exposure_max_sensitivity {#auto_exposure_max_sensitivity}

```gdscript
auto_exposure_max_sensitivity: float
```

- void set_auto_exposure_max_sensitivity(value: float)
- float get_auto_exposure_max_sensitivity()

The maximum sensitivity (in ISO) used when calculating auto exposure. When calculating scene average luminance, color values will be clamped to at least this value. This limits the auto-exposure from exposing below a certain brightness, resulting in a cut off point where the scene will remain bright.
