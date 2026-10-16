# Light3D

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Light3D**

> Light3D is the abstract base class for light nodes. As it can't be instantiated, it shouldn't be used directly. Other types of light nodes inherit from it. Light3D contains the common variables and parameters used for lighting.

<details><summary>官方英文描述</summary>

Light3D is the abstract base class for light nodes. As it can't be instantiated, it shouldn't be used directly. Other types of light nodes inherit from it. Light3D contains the common variables and parameters used for lighting.

</details>

## 方法

### get_bake_mode {#get_bake_mode}

```gdscript
Light3D.BakeMode get_bake_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_color {#get_color}

```gdscript
Color get_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_correlated_color {#get_correlated_color}

```gdscript
Color get_correlated_color() const
```

Returns the Color of an idealized blackbody at the given light_temperature. This value is calculated internally based on the light_temperature. This Color is multiplied by light_color before being sent to the RenderingServer.

### get_cull_mask {#get_cull_mask}

```gdscript
int get_cull_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_distance_fade_begin {#get_distance_fade_begin}

```gdscript
float get_distance_fade_begin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_distance_fade_length {#get_distance_fade_length}

```gdscript
float get_distance_fade_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_distance_fade_shadow {#get_distance_fade_shadow}

```gdscript
float get_distance_fade_shadow() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_param {#get_param}

```gdscript
float get_param(param: Light3D.Param) const
```

Returns the value of the specified Param parameter.

### get_projector {#get_projector}

```gdscript
Texture2D get_projector() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadow_caster_mask {#get_shadow_caster_mask}

```gdscript
int get_shadow_caster_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadow_reverse_cull_face {#get_shadow_reverse_cull_face}

```gdscript
bool get_shadow_reverse_cull_face() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_temperature {#get_temperature}

```gdscript
float get_temperature() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_shadow {#has_shadow}

```gdscript
bool has_shadow() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_distance_fade_enabled {#is_distance_fade_enabled}

```gdscript
bool is_distance_fade_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editor_only {#is_editor_only}

```gdscript
bool is_editor_only() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_negative {#is_negative}

```gdscript
bool is_negative() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bake_mode {#set_bake_mode}

```gdscript
void set_bake_mode(bake_mode: Light3D.BakeMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color {#set_color}

```gdscript
void set_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cull_mask {#set_cull_mask}

```gdscript
void set_cull_mask(cull_mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_distance_fade_begin {#set_distance_fade_begin}

```gdscript
void set_distance_fade_begin(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_distance_fade_length {#set_distance_fade_length}

```gdscript
void set_distance_fade_length(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_distance_fade_shadow {#set_distance_fade_shadow}

```gdscript
void set_distance_fade_shadow(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_editor_only {#set_editor_only}

```gdscript
void set_editor_only(editor_only: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_distance_fade {#set_enable_distance_fade}

```gdscript
void set_enable_distance_fade(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_negative {#set_negative}

```gdscript
void set_negative(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_param {#set_param}

```gdscript
void set_param(param: Light3D.Param, value: float)
```

Sets the value of the specified Param parameter.

### set_projector {#set_projector}

```gdscript
void set_projector(projector: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow {#set_shadow}

```gdscript
void set_shadow(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_caster_mask {#set_shadow_caster_mask}

```gdscript
void set_shadow_caster_mask(caster_mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_reverse_cull_face {#set_shadow_reverse_cull_face}

```gdscript
void set_shadow_reverse_cull_face(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_temperature {#set_temperature}

```gdscript
void set_temperature(temperature: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### light_intensity_lumens {#light_intensity_lumens}

```gdscript
light_intensity_lumens: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Used by positional lights (OmniLight3D and SpotLight3D) when ProjectSettings.rendering/lights_and_shadows/use_physical_light_units is true. Sets the intensity of the light source measured in Lumens. Lumens are a measure of luminous flux, which is the total amount of visible light emitted by a light source per unit of time.

For SpotLight3Ds, we assume that the area outside the visible cone is surrounded by a perfect light absorbing material. Accordingly, the apparent brightness of the cone area does not change as the cone increases and decreases in size.

A typical household lightbulb can range from around 600 lumens to 1,200 lumens, a candle is about 13 lumens, while a streetlight can be approximately 60,000 lumens.

### light_intensity_lux {#light_intensity_lux}

```gdscript
light_intensity_lux: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Used by DirectionalLight3Ds when ProjectSettings.rendering/lights_and_shadows/use_physical_light_units is true. Sets the intensity of the light source measured in Lux. Lux is a measure of luminous flux per unit area, it is equal to one lumen per square meter. Lux is the measure of how much light hits a surface at a given time.

On a clear sunny day a surface in direct sunlight may be approximately 100,000 lux, a typical room in a home may be approximately 50 lux, while the moonlit ground may be approximately 0.1 lux.

### light_temperature {#light_temperature}

```gdscript
light_temperature: float
```

- void set_temperature(value: float)
- float get_temperature()

Sets the color temperature of the light source, measured in Kelvin. This is used to calculate a correlated color temperature which tints the light_color.

The sun on a cloudy day is approximately 6500 Kelvin, on a clear day it is between 5500 to 6000 Kelvin, and on a clear day at sunrise or sunset it ranges to around 1850 Kelvin.

### light_color {#light_color}

```gdscript
light_color: Color
```

- void set_color(value: Color)
- Color get_color()

The light's color in nonlinear sRGB encoding. An overbright color can be used to achieve a result equivalent to increasing the light's light_energy.

### light_energy {#light_energy}

```gdscript
light_energy: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

The light's strength multiplier (this is not a physical unit). For OmniLight3D and SpotLight3D, changing this value will only change the light color's intensity, not the light's radius.

### light_indirect_energy {#light_indirect_energy}

```gdscript
light_indirect_energy: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Secondary multiplier used with indirect light (light bounces). Used with VoxelGI and SDFGI (see Environment.sdfgi_enabled).

Note: This property is ignored if light_energy is equal to 0.0, as the light won't be present at all in the GI shader.

### light_volumetric_fog_energy {#light_volumetric_fog_energy}

```gdscript
light_volumetric_fog_energy: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Secondary multiplier multiplied with light_energy then used with the Environment's volumetric fog (if enabled). If set to 0.0, computing volumetric fog will be skipped for this light, which can improve performance for large amounts of lights when volumetric fog is enabled.

Note: To prevent short-lived dynamic light effects from poorly interacting with volumetric fog, lights used in those effects should have light_volumetric_fog_energy set to 0.0 unless Environment.volumetric_fog_temporal_reprojection_enabled is disabled (or unless the reprojection amount is significantly lowered).

### light_projector {#light_projector}

```gdscript
light_projector: Texture2D,-AnimatedTexture,-AtlasTexture,-CameraTexture,-CanvasTexture,-MeshTexture,-Texture2DRD,-ViewportTexture
```

- void set_projector(value: Texture2D)
- Texture2D get_projector()

Texture2D projected by light. shadow_enabled must be on for the projector to work. Light projectors make the light appear as if it is shining through a colored but transparent object, almost like light shining through stained-glass.

Note: Unlike BaseMaterial3D whose filter mode can be adjusted on a per-material basis, the filter mode for light projector textures is set globally with ProjectSettings.rendering/textures/light_projectors/filter.

Note: Light projector textures are only supported in the Forward+ and Mobile rendering methods, not Compatibility.

### light_size {#light_size}

```gdscript
light_size: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

The simulated size of the light in Godot units, affecting shading and shadows. For OmniLight3Ds and SpotLight3Ds, increasing this value simulates a spherical area light, expanding the size of specular highlights. If shadows are enabled, a penumbra is rendered, making shadows appear blurrier. For AreaLight3Ds, only the shadows are affected. Penumbras are simulated with percentage-closer soft shadows, or PCSS, which has a noticeable performance cost for values above 0.0.

Note: light_size is not affected by Node3D.scale (the light's scale or its parent's scale).

Note: PCSS for positional lights is only supported in the Forward+ and Mobile rendering methods, not Compatibility.

### light_angular_distance {#light_angular_distance}

```gdscript
light_angular_distance: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

The light's angular size in degrees. Increasing this will make shadows softer at greater distances (also called percentage-closer soft shadows, or PCSS). Only available for DirectionalLight3Ds. For reference, the Sun from the Earth is approximately 0.5. Increasing this value above 0.0 for lights with shadows enabled will have a noticeable performance cost due to PCSS.

Note: light_angular_distance is not affected by Node3D.scale (the light's scale or its parent's scale).

Note: PCSS for directional lights is only supported in the Forward+ rendering method, not Mobile or Compatibility.

### light_negative {#light_negative}

```gdscript
light_negative: bool
```

- void set_negative(value: bool)
- bool is_negative()

If true, the light's effect is reversed, darkening areas and casting bright shadows.

### light_specular {#light_specular}

```gdscript
light_specular: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

The intensity of the specular blob in objects affected by the light. At 0, the light becomes a pure diffuse light. When not baking emission, this can be used to avoid unrealistic reflections when placing lights above an emissive surface.

### light_bake_mode {#light_bake_mode}

```gdscript
light_bake_mode: int
```

- void set_bake_mode(value: BakeMode)
- BakeMode get_bake_mode()

The light's bake mode. This will affect the global illumination techniques that have an effect on the light's rendering.

Note: Meshes' global illumination mode will also affect the global illumination rendering. See GeometryInstance3D.gi_mode.

### light_cull_mask {#light_cull_mask}

```gdscript
light_cull_mask: int
```

- void set_cull_mask(value: int)
- int get_cull_mask()

The light will affect objects in the selected layers.

Note: The light cull mask is ignored by VoxelGI, SDFGI, LightmapGI, and volumetric fog. These will always render lights in a way that ignores the cull mask. See also VisualInstance3D.layers.

### shadow_enabled {#shadow_enabled}

```gdscript
shadow_enabled: bool
```

- void set_shadow(value: bool)
- bool has_shadow()

If true, the light will cast real-time shadows. This has a significant performance cost. Only enable shadow rendering when it makes a noticeable difference in the scene's appearance, and consider using distance_fade_enabled to hide the light when far away from the Camera3D.

### shadow_bias {#shadow_bias}

```gdscript
shadow_bias: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Used to adjust shadow appearance. Too small a value results in self-shadowing ("shadow acne"), while too large a value causes shadows to separate from casters ("peter-panning"). Adjust as needed.

### shadow_normal_bias {#shadow_normal_bias}

```gdscript
shadow_normal_bias: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Offsets the lookup into the shadow map by the object's normal. This can be used to reduce self-shadowing artifacts without using shadow_bias. In practice, this value should be tweaked along with shadow_bias to reduce artifacts as much as possible.

### shadow_reverse_cull_face {#shadow_reverse_cull_face}

```gdscript
shadow_reverse_cull_face: bool
```

- void set_shadow_reverse_cull_face(value: bool)
- bool get_shadow_reverse_cull_face()

If true, reverses the backface culling of the mesh. This can be useful when you have a flat mesh that has a light behind it. If you need to cast a shadow on both sides of the mesh, set the mesh to use double-sided shadows with GeometryInstance3D.SHADOW_CASTING_SETTING_DOUBLE_SIDED.

### shadow_transmittance_bias {#shadow_transmittance_bias}

```gdscript
shadow_transmittance_bias: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### shadow_opacity {#shadow_opacity}

```gdscript
shadow_opacity: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

The opacity to use when rendering the light's shadow map. Values lower than 1.0 make the light appear through shadows. This can be used to fake global illumination at a low performance cost.

### shadow_blur {#shadow_blur}

```gdscript
shadow_blur: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Blurs the edges of the shadow. Can be used to hide pixel artifacts in low-resolution shadow maps. A high value can impact performance, make shadows appear grainy and can cause other unwanted artifacts. Try to keep as near default as possible.

### shadow_caster_mask {#shadow_caster_mask}

```gdscript
shadow_caster_mask: int
```

- void set_shadow_caster_mask(value: int)
- int get_shadow_caster_mask()

The light will only cast shadows using objects in the selected layers.

### distance_fade_enabled {#distance_fade_enabled}

```gdscript
distance_fade_enabled: bool
```

- void set_enable_distance_fade(value: bool)
- bool is_distance_fade_enabled()

If true, the light will smoothly fade away when far from the active Camera3D starting at distance_fade_begin. This acts as a form of level of detail (LOD). The light will fade out over distance_fade_begin + distance_fade_length, after which it will be culled and not sent to the shader at all. Use this to reduce the number of active lights in a scene and thus improve performance.

Note: Only effective for OmniLight3D and SpotLight3D.

### distance_fade_begin {#distance_fade_begin}

```gdscript
distance_fade_begin: float
```

- void set_distance_fade_begin(value: float)
- float get_distance_fade_begin()

The distance from the camera at which the light begins to fade away (in 3D units).

Note: Only effective for OmniLight3D and SpotLight3D.

### distance_fade_shadow {#distance_fade_shadow}

```gdscript
distance_fade_shadow: float
```

- void set_distance_fade_shadow(value: float)
- float get_distance_fade_shadow()

The distance from the camera at which the light's shadow cuts off (in 3D units). Set this to a value lower than distance_fade_begin + distance_fade_length to further improve performance, as shadow rendering is often more expensive than light rendering itself.

Note: Only effective for OmniLight3D and SpotLight3D, and only when shadow_enabled is true.

### distance_fade_length {#distance_fade_length}

```gdscript
distance_fade_length: float
```

- void set_distance_fade_length(value: float)
- float get_distance_fade_length()

Distance over which the light and its shadow fades. The light's energy and shadow's opacity is progressively reduced over this distance and is completely invisible at the end.

Note: Only effective for OmniLight3D and SpotLight3D.

### editor_only {#editor_only}

```gdscript
editor_only: bool
```

- void set_editor_only(value: bool)
- bool is_editor_only()

If true, the light only appears in the editor and will not be visible at runtime. If true, the light will never be baked in LightmapGI regardless of its light_bake_mode.

## 枚举

### Param {#enum-Param}

```gdscript
enum Param { PARAM_ENERGY = 0, PARAM_INDIRECT_ENERGY = 1, PARAM_VOLUMETRIC_FOG_ENERGY = 2, PARAM_SPECULAR = 3, PARAM_RANGE = 4, PARAM_SIZE = 5, PARAM_ATTENUATION = 6, PARAM_SPOT_ANGLE = 7, PARAM_SPOT_ATTENUATION = 8, PARAM_SHADOW_MAX_DISTANCE = 9, PARAM_SHADOW_SPLIT_1_OFFSET = 10, PARAM_SHADOW_SPLIT_2_OFFSET = 11, PARAM_SHADOW_SPLIT_3_OFFSET = 12, PARAM_SHADOW_FADE_START = 13, PARAM_SHADOW_NORMAL_BIAS = 14, PARAM_SHADOW_BIAS = 15, PARAM_SHADOW_PANCAKE_SIZE = 16, PARAM_SHADOW_OPACITY = 17, PARAM_SHADOW_BLUR = 18, PARAM_TRANSMITTANCE_BIAS = 19, PARAM_INTENSITY = 20, PARAM_MAX = 21 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ENERGY {#PARAM_ENERGY}

```gdscript
PARAM_ENERGY = 0
```

Constant for accessing light_energy.

#### PARAM_INDIRECT_ENERGY {#PARAM_INDIRECT_ENERGY}

```gdscript
PARAM_INDIRECT_ENERGY = 1
```

Constant for accessing light_indirect_energy.

#### PARAM_VOLUMETRIC_FOG_ENERGY {#PARAM_VOLUMETRIC_FOG_ENERGY}

```gdscript
PARAM_VOLUMETRIC_FOG_ENERGY = 2
```

Constant for accessing light_volumetric_fog_energy.

#### PARAM_SPECULAR {#PARAM_SPECULAR}

```gdscript
PARAM_SPECULAR = 3
```

Constant for accessing light_specular.

#### PARAM_RANGE {#PARAM_RANGE}

```gdscript
PARAM_RANGE = 4
```

Constant for accessing OmniLight3D.omni_range or SpotLight3D.spot_range.

#### PARAM_SIZE {#PARAM_SIZE}

```gdscript
PARAM_SIZE = 5
```

Constant for accessing light_size.

#### PARAM_ATTENUATION {#PARAM_ATTENUATION}

```gdscript
PARAM_ATTENUATION = 6
```

Constant for accessing OmniLight3D.omni_attenuation or SpotLight3D.spot_attenuation.

#### PARAM_SPOT_ANGLE {#PARAM_SPOT_ANGLE}

```gdscript
PARAM_SPOT_ANGLE = 7
```

Constant for accessing SpotLight3D.spot_angle.

#### PARAM_SPOT_ATTENUATION {#PARAM_SPOT_ATTENUATION}

```gdscript
PARAM_SPOT_ATTENUATION = 8
```

Constant for accessing SpotLight3D.spot_angle_attenuation.

#### PARAM_SHADOW_MAX_DISTANCE {#PARAM_SHADOW_MAX_DISTANCE}

```gdscript
PARAM_SHADOW_MAX_DISTANCE = 9
```

Constant for accessing DirectionalLight3D.directional_shadow_max_distance.

#### PARAM_SHADOW_SPLIT_1_OFFSET {#PARAM_SHADOW_SPLIT_1_OFFSET}

```gdscript
PARAM_SHADOW_SPLIT_1_OFFSET = 10
```

Constant for accessing DirectionalLight3D.directional_shadow_split_1.

#### PARAM_SHADOW_SPLIT_2_OFFSET {#PARAM_SHADOW_SPLIT_2_OFFSET}

```gdscript
PARAM_SHADOW_SPLIT_2_OFFSET = 11
```

Constant for accessing DirectionalLight3D.directional_shadow_split_2.

#### PARAM_SHADOW_SPLIT_3_OFFSET {#PARAM_SHADOW_SPLIT_3_OFFSET}

```gdscript
PARAM_SHADOW_SPLIT_3_OFFSET = 12
```

Constant for accessing DirectionalLight3D.directional_shadow_split_3.

#### PARAM_SHADOW_FADE_START {#PARAM_SHADOW_FADE_START}

```gdscript
PARAM_SHADOW_FADE_START = 13
```

Constant for accessing DirectionalLight3D.directional_shadow_fade_start.

#### PARAM_SHADOW_NORMAL_BIAS {#PARAM_SHADOW_NORMAL_BIAS}

```gdscript
PARAM_SHADOW_NORMAL_BIAS = 14
```

Constant for accessing shadow_normal_bias.

#### PARAM_SHADOW_BIAS {#PARAM_SHADOW_BIAS}

```gdscript
PARAM_SHADOW_BIAS = 15
```

Constant for accessing shadow_bias.

#### PARAM_SHADOW_PANCAKE_SIZE {#PARAM_SHADOW_PANCAKE_SIZE}

```gdscript
PARAM_SHADOW_PANCAKE_SIZE = 16
```

Constant for accessing DirectionalLight3D.directional_shadow_pancake_size.

#### PARAM_SHADOW_OPACITY {#PARAM_SHADOW_OPACITY}

```gdscript
PARAM_SHADOW_OPACITY = 17
```

Constant for accessing shadow_opacity.

#### PARAM_SHADOW_BLUR {#PARAM_SHADOW_BLUR}

```gdscript
PARAM_SHADOW_BLUR = 18
```

Constant for accessing shadow_blur.

#### PARAM_TRANSMITTANCE_BIAS {#PARAM_TRANSMITTANCE_BIAS}

```gdscript
PARAM_TRANSMITTANCE_BIAS = 19
```

Constant for accessing shadow_transmittance_bias.

#### PARAM_INTENSITY {#PARAM_INTENSITY}

```gdscript
PARAM_INTENSITY = 20
```

Constant for accessing light_intensity_lumens and light_intensity_lux. Only used when ProjectSettings.rendering/lights_and_shadows/use_physical_light_units is true.

#### PARAM_MAX {#PARAM_MAX}

```gdscript
PARAM_MAX = 21
```

Represents the size of the Param enum.

---

### BakeMode {#enum-BakeMode}

```gdscript
enum BakeMode { BAKE_DISABLED = 0, BAKE_STATIC = 1, BAKE_DYNAMIC = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BAKE_DISABLED {#BAKE_DISABLED}

```gdscript
BAKE_DISABLED = 0
```

Light is ignored when baking. This is the fastest mode, but the light will not be taken into account when baking global illumination. This mode should generally be used for dynamic lights that change quickly, as the effect of global illumination is less noticeable on those lights.

Note: Hiding a light does not affect baking LightmapGI. Hiding a light will still affect baking VoxelGI and SDFGI (see Environment.sdfgi_enabled).

#### BAKE_STATIC {#BAKE_STATIC}

```gdscript
BAKE_STATIC = 1
```

Light is taken into account in static baking (VoxelGI, LightmapGI, SDFGI (Environment.sdfgi_enabled)). The light can be moved around or modified, but its global illumination will not update in real-time. This is suitable for subtle changes (such as flickering torches), but generally not large changes such as toggling a light on and off.

Note: The light is not baked in LightmapGI if editor_only is true.

#### BAKE_DYNAMIC {#BAKE_DYNAMIC}

```gdscript
BAKE_DYNAMIC = 2
```

Light is taken into account in dynamic baking (VoxelGI and SDFGI (Environment.sdfgi_enabled) only). The light can be moved around or modified with global illumination updating in real-time. The light's global illumination appearance will be slightly different compared to BAKE_STATIC. This has a greater performance cost compared to BAKE_STATIC. When using SDFGI, the update speed of dynamic lights is affected by ProjectSettings.rendering/global_illumination/sdfgi/frames_to_update_lights.

---
