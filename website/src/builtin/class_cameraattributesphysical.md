# CameraAttributesPhysical

**继承**：[CameraAttributes](class_cameraattributes.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CameraAttributesPhysical**

> CameraAttributesPhysical is used to set rendering settings based on a physically-based camera's settings. It is responsible for exposure, auto-exposure, and depth of field.

<details><summary>官方英文描述</summary>

CameraAttributesPhysical is used to set rendering settings based on a physically-based camera's settings. It is responsible for exposure, auto-exposure, and depth of field.

When used in a WorldEnvironment it provides default settings for exposure, auto-exposure, and depth of field that will be used by all cameras without their own CameraAttributes, including the editor camera. When used in a Camera3D it will override any CameraAttributes set in the WorldEnvironment and will override the Camera3Ds Camera3D.far, Camera3D.near, Camera3D.fov, and Camera3D.keep_aspect properties. When used in VoxelGI or LightmapGI, only the exposure settings will be used.

The default settings are intended for use in an outdoor environment, tips for settings for use in an indoor environment can be found in each setting's documentation.

Note: Depth of field blur is only supported in the Forward+ and Mobile rendering methods, not Compatibility.

Note: Auto-exposure is only supported in the Forward+ rendering method, not Mobile or Compatibility.

</details>

## 方法

### get_aperture {#get_aperture}

```gdscript
float get_aperture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_auto_exposure_max_exposure_value {#get_auto_exposure_max_exposure_value}

```gdscript
float get_auto_exposure_max_exposure_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_auto_exposure_min_exposure_value {#get_auto_exposure_min_exposure_value}

```gdscript
float get_auto_exposure_min_exposure_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_far {#get_far}

```gdscript
float get_far() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_focal_length {#get_focal_length}

```gdscript
float get_focal_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_focus_distance {#get_focus_distance}

```gdscript
float get_focus_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fov {#get_fov}

```gdscript
float get_fov() const
```

Returns the vertical field of view that corresponds to the frustum_focal_length. This value is calculated internally whenever frustum_focal_length is changed.

### get_near {#get_near}

```gdscript
float get_near() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shutter_speed {#get_shutter_speed}

```gdscript
float get_shutter_speed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_aperture {#set_aperture}

```gdscript
void set_aperture(aperture: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_exposure_max_exposure_value {#set_auto_exposure_max_exposure_value}

```gdscript
void set_auto_exposure_max_exposure_value(exposure_value_max: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_exposure_min_exposure_value {#set_auto_exposure_min_exposure_value}

```gdscript
void set_auto_exposure_min_exposure_value(exposure_value_min: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_far {#set_far}

```gdscript
void set_far(far: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_focal_length {#set_focal_length}

```gdscript
void set_focal_length(focal_length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_focus_distance {#set_focus_distance}

```gdscript
void set_focus_distance(focus_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_near {#set_near}

```gdscript
void set_near(near: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shutter_speed {#set_shutter_speed}

```gdscript
void set_shutter_speed(shutter_speed: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### frustum_focus_distance {#frustum_focus_distance}

```gdscript
frustum_focus_distance: float
```

- void set_focus_distance(value: float)
- float get_focus_distance()

Distance from camera of object that will be in focus, measured in meters. Internally this will be clamped to be at least 1 millimeter larger than frustum_focal_length.

### frustum_focal_length {#frustum_focal_length}

```gdscript
frustum_focal_length: float
```

- void set_focal_length(value: float)
- float get_focal_length()

Distance between camera lens and camera aperture, measured in millimeters. Controls field of view and depth of field. A larger focal length will result in a smaller field of view and a narrower depth of field meaning fewer objects will be in focus. A smaller focal length will result in a wider field of view and a larger depth of field meaning more objects will be in focus. When attached to a Camera3D as its Camera3D.attributes, it will override the Camera3D.fov property and the Camera3D.keep_aspect property.

### frustum_near {#frustum_near}

```gdscript
frustum_near: float
```

- void set_near(value: float)
- float get_near()

Override value for Camera3D.near. Used internally when calculating depth of field. When attached to a Camera3D as its Camera3D.attributes, it will override the Camera3D.near property.

### frustum_far {#frustum_far}

```gdscript
frustum_far: float
```

- void set_far(value: float)
- float get_far()

Override value for Camera3D.far. Used internally when calculating depth of field. When attached to a Camera3D as its Camera3D.attributes, it will override the Camera3D.far property.

### exposure_aperture {#exposure_aperture}

```gdscript
exposure_aperture: float
```

- void set_aperture(value: float)
- float get_aperture()

Size of the aperture of the camera, measured in f-stops. An f-stop is a unitless ratio between the focal length of the camera and the diameter of the aperture. A high aperture setting will result in a smaller aperture which leads to a dimmer image and sharper focus. A low aperture results in a wide aperture which lets in more light resulting in a brighter, less-focused image. Default is appropriate for outdoors at daytime (i.e. for use with a default DirectionalLight3D), for indoor lighting, a value between 2 and 4 is more appropriate.

Only available when ProjectSettings.rendering/lights_and_shadows/use_physical_light_units is enabled.

### exposure_shutter_speed {#exposure_shutter_speed}

```gdscript
exposure_shutter_speed: float
```

- void set_shutter_speed(value: float)
- float get_shutter_speed()

Time for shutter to open and close, evaluated as 1 / shutter_speed seconds. A higher value will allow less light (leading to a darker image), while a lower value will allow more light (leading to a brighter image).

Only available when ProjectSettings.rendering/lights_and_shadows/use_physical_light_units is enabled.

### auto_exposure_min_exposure_value {#auto_exposure_min_exposure_value}

```gdscript
auto_exposure_min_exposure_value: float
```

- void set_auto_exposure_min_exposure_value(value: float)
- float get_auto_exposure_min_exposure_value()

The minimum luminance (in EV100) used when calculating auto exposure. When calculating scene average luminance, color values will be clamped to at least this value. This limits the auto-exposure from exposing above a certain brightness, resulting in a cut off point where the scene will remain dark.

### auto_exposure_max_exposure_value {#auto_exposure_max_exposure_value}

```gdscript
auto_exposure_max_exposure_value: float
```

- void set_auto_exposure_max_exposure_value(value: float)
- float get_auto_exposure_max_exposure_value()

The maximum luminance (in EV100) used when calculating auto exposure. When calculating scene average luminance, color values will be clamped to at least this value. This limits the auto-exposure from exposing below a certain brightness, resulting in a cut off point where the scene will remain bright.
