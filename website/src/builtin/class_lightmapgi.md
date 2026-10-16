# LightmapGI

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **LightmapGI**

> The LightmapGI node is used to compute and store baked lightmaps. Lightmaps are used to provide high-quality indirect lighting with very little light leaking. LightmapGI can also provide rough reflections using spherical harmonics if directional is enabled. Dynamic objects can receive indirect lighting thanks to light probes, which can be automatically placed by setting generate_probes_subdiv to a value other than GENERATE_PROBES_DISABLED. Additional lightmap probes can also be added by creating LightmapProbe nodes. The downside is that lightmaps are fully static and cannot be baked in an exported project. Baking a LightmapGI node is also slower compared to VoxelGI.

<details><summary>官方英文描述（中文精译进行中）</summary>

The LightmapGI node is used to compute and store baked lightmaps. Lightmaps are used to provide high-quality indirect lighting with very little light leaking. LightmapGI can also provide rough reflections using spherical harmonics if directional is enabled. Dynamic objects can receive indirect lighting thanks to light probes, which can be automatically placed by setting generate_probes_subdiv to a value other than GENERATE_PROBES_DISABLED. Additional lightmap probes can also be added by creating LightmapProbe nodes. The downside is that lightmaps are fully static and cannot be baked in an exported project. Baking a LightmapGI node is also slower compared to VoxelGI.

Procedural generation: Lightmap baking functionality is only available in the editor. This means LightmapGI is not suited to procedurally generated or user-built levels. For procedurally generated or user-built levels, use VoxelGI or SDFGI instead (see Environment.sdfgi_enabled).

Performance: LightmapGI provides the best possible run-time performance for global illumination. It is suitable for low-end hardware including integrated graphics and mobile devices.

Note: Due to how lightmaps work, most properties only have a visible effect once lightmaps are baked again.

Note: Lightmap baking on CSGShape3Ds and PrimitiveMeshes is not supported, as these cannot store UV2 data required for baking.

Note: If no custom lightmappers are installed, LightmapGI can only be baked from devices that support the Forward+ or Mobile renderers.

Note: The LightmapGI node only bakes light data for child nodes of its parent. Nodes further up the hierarchy of the scene will not be baked.

</details>

## 方法

### get_bake_quality {#get_bake_quality}

```gdscript
LightmapGI.BakeQuality get_bake_quality() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bias {#get_bias}

```gdscript
float get_bias() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bounce_indirect_energy {#get_bounce_indirect_energy}

```gdscript
float get_bounce_indirect_energy() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bounces {#get_bounces}

```gdscript
int get_bounces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_camera_attributes {#get_camera_attributes}

```gdscript
CameraAttributes get_camera_attributes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_denoiser_range {#get_denoiser_range}

```gdscript
int get_denoiser_range() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_denoiser_strength {#get_denoiser_strength}

```gdscript
float get_denoiser_strength() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_environment_custom_color {#get_environment_custom_color}

```gdscript
Color get_environment_custom_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_environment_custom_energy {#get_environment_custom_energy}

```gdscript
float get_environment_custom_energy() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_environment_custom_sky {#get_environment_custom_sky}

```gdscript
Sky get_environment_custom_sky() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_environment_mode {#get_environment_mode}

```gdscript
LightmapGI.EnvironmentMode get_environment_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_generate_probes {#get_generate_probes}

```gdscript
LightmapGI.GenerateProbes get_generate_probes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_light_data {#get_light_data}

```gdscript
LightmapGIData get_light_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_texture_size {#get_max_texture_size}

```gdscript
int get_max_texture_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadowmask_mode {#get_shadowmask_mode}

```gdscript
LightmapGIData.ShadowmaskMode get_shadowmask_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_supersampling_factor {#get_supersampling_factor}

```gdscript
float get_supersampling_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texel_scale {#get_texel_scale}

```gdscript
float get_texel_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_directional {#is_directional}

```gdscript
bool is_directional() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_interior {#is_interior}

```gdscript
bool is_interior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_supersampling_enabled {#is_supersampling_enabled}

```gdscript
bool is_supersampling_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_denoiser {#is_using_denoiser}

```gdscript
bool is_using_denoiser() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_texture_for_bounces {#is_using_texture_for_bounces}

```gdscript
bool is_using_texture_for_bounces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bake_quality {#set_bake_quality}

```gdscript
void set_bake_quality(bake_quality: LightmapGI.BakeQuality)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bias {#set_bias}

```gdscript
void set_bias(bias: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bounce_indirect_energy {#set_bounce_indirect_energy}

```gdscript
void set_bounce_indirect_energy(bounce_indirect_energy: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bounces {#set_bounces}

```gdscript
void set_bounces(bounces: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_camera_attributes {#set_camera_attributes}

```gdscript
void set_camera_attributes(camera_attributes: CameraAttributes)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_denoiser_range {#set_denoiser_range}

```gdscript
void set_denoiser_range(denoiser_range: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_denoiser_strength {#set_denoiser_strength}

```gdscript
void set_denoiser_strength(denoiser_strength: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_directional {#set_directional}

```gdscript
void set_directional(directional: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment_custom_color {#set_environment_custom_color}

```gdscript
void set_environment_custom_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment_custom_energy {#set_environment_custom_energy}

```gdscript
void set_environment_custom_energy(energy: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment_custom_sky {#set_environment_custom_sky}

```gdscript
void set_environment_custom_sky(sky: Sky)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment_mode {#set_environment_mode}

```gdscript
void set_environment_mode(mode: LightmapGI.EnvironmentMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_generate_probes {#set_generate_probes}

```gdscript
void set_generate_probes(subdivision: LightmapGI.GenerateProbes)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interior {#set_interior}

```gdscript
void set_interior(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_light_data {#set_light_data}

```gdscript
void set_light_data(data: LightmapGIData)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_texture_size {#set_max_texture_size}

```gdscript
void set_max_texture_size(max_texture_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadowmask_mode {#set_shadowmask_mode}

```gdscript
void set_shadowmask_mode(mode: LightmapGIData.ShadowmaskMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_supersampling_enabled {#set_supersampling_enabled}

```gdscript
void set_supersampling_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_supersampling_factor {#set_supersampling_factor}

```gdscript
void set_supersampling_factor(factor: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texel_scale {#set_texel_scale}

```gdscript
void set_texel_scale(texel_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_denoiser {#set_use_denoiser}

```gdscript
void set_use_denoiser(use_denoiser: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_texture_for_bounces {#set_use_texture_for_bounces}

```gdscript
void set_use_texture_for_bounces(use_texture_for_bounces: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### quality {#quality}

```gdscript
quality: int
```

- void set_bake_quality(value: BakeQuality)
- BakeQuality get_bake_quality()

The quality preset to use when baking lightmaps. This affects bake times, but output file sizes remain mostly identical across quality levels.

To further speed up bake times, decrease bounces, disable use_denoiser and/or decrease texel_scale.

To further increase quality, enable supersampling and/or increase texel_scale.

### supersampling {#supersampling}

```gdscript
supersampling: bool
```

- void set_supersampling_enabled(value: bool)
- bool is_supersampling_enabled()

If true, lightmaps are baked with the texel scale multiplied with supersampling_factor and downsampled before saving the lightmap (so the effective texel density is identical to having supersampling disabled).

Supersampling provides increased lightmap quality with less noise, smoother shadows and better shadowing of small-scale features in objects. However, it may result in significantly increased bake times and memory usage while baking lightmaps. Padding is automatically adjusted to avoid increasing light leaking.

### supersampling_factor {#supersampling_factor}

```gdscript
supersampling_factor: float
```

- void set_supersampling_factor(value: float)
- float get_supersampling_factor()

The factor by which the texel density is multiplied for supersampling. For best results, use an integer value. While fractional values are allowed, they can result in increased light leaking and a blurry lightmap.

Higher values may result in better quality, but also increase bake times and memory usage while baking.

See supersampling for more information.

### bounces {#bounces}

```gdscript
bounces: int
```

- void set_bounces(value: int)
- int get_bounces()

Number of light bounces that are taken into account during baking. Higher values result in brighter, more realistic lighting, at the cost of longer bake times. If set to 0, only environment lighting, direct light and emissive lighting is baked.

### bounce_indirect_energy {#bounce_indirect_energy}

```gdscript
bounce_indirect_energy: float
```

- void set_bounce_indirect_energy(value: float)
- float get_bounce_indirect_energy()

The energy multiplier for each bounce. Higher values will make indirect lighting brighter. A value of 1.0 represents physically accurate behavior, but higher values can be used to make indirect lighting propagate more visibly when using a low number of bounces. This can be used to speed up bake times by lowering the number of bounces then increasing bounce_indirect_energy.

Note: bounce_indirect_energy only has an effect if bounces is set to a value greater than or equal to 1.

### directional {#directional}

```gdscript
directional: bool
```

- void set_directional(value: bool)
- bool is_directional()

If true, bakes lightmaps to contain directional information as spherical harmonics. This results in more realistic lighting appearance, especially with normal mapped materials and for lights that have their direct light baked (Light3D.light_bake_mode set to Light3D.BAKE_STATIC and with Light3D.editor_only set to false). The directional information is also used to provide rough reflections for static and dynamic objects. This has a small run-time performance cost as the shader has to perform more work to interpret the direction information from the lightmap. Directional lightmaps also take longer to bake and result in larger file sizes.

Note: The property's name has no relationship with DirectionalLight3D. directional works with all light types.

### shadowmask_mode {#shadowmask_mode}

```gdscript
shadowmask_mode: int
```

- void set_shadowmask_mode(value: ShadowmaskMode)
- ShadowmaskMode get_shadowmask_mode()

Experimental: This property may be changed or removed in future versions.

The shadowmasking policy to use for directional shadows on static objects that are baked with this LightmapGI instance.

Shadowmasking allows DirectionalLight3D nodes to cast shadows even outside the range defined by their DirectionalLight3D.directional_shadow_max_distance property. This is done by baking a texture that contains a shadowmap for the directional light, then using this texture according to the current shadowmask mode.

Note: The shadowmask texture is only created if shadowmask_mode is not LightmapGIData.SHADOWMASK_MODE_NONE. To see a difference, you need to bake lightmaps again after switching from LightmapGIData.SHADOWMASK_MODE_NONE to any other mode.

### use_texture_for_bounces {#use_texture_for_bounces}

```gdscript
use_texture_for_bounces: bool
```

- void set_use_texture_for_bounces(value: bool)
- bool is_using_texture_for_bounces()

If true, a texture with the lighting information will be generated to speed up the generation of indirect lighting at the cost of some accuracy. The geometry might exhibit extra light leak artifacts when using low resolution lightmaps or UVs that stretch the lightmap significantly across surfaces. Leave use_texture_for_bounces at its default value of true if unsure.

Note: use_texture_for_bounces only has an effect if bounces is set to a value greater than or equal to 1.

### interior {#interior}

```gdscript
interior: bool
```

- void set_interior(value: bool)
- bool is_interior()

If true, ignore environment lighting when baking lightmaps.

### use_denoiser {#use_denoiser}

```gdscript
use_denoiser: bool
```

- void set_use_denoiser(value: bool)
- bool is_using_denoiser()

If true, uses a GPU-based denoising algorithm on the generated lightmap. This eliminates most noise within the generated lightmap at the cost of longer bake times. File sizes are generally not impacted significantly by the use of a denoiser, although lossless compression may do a better job at compressing a denoised image.

### denoiser_strength {#denoiser_strength}

```gdscript
denoiser_strength: float
```

- void set_denoiser_strength(value: float)
- float get_denoiser_strength()

The strength of denoising step applied to the generated lightmaps. Only effective if use_denoiser is true and ProjectSettings.rendering/lightmapping/denoising/denoiser is set to JNLM.

### denoiser_range {#denoiser_range}

```gdscript
denoiser_range: int
```

- void set_denoiser_range(value: int)
- int get_denoiser_range()

The distance in pixels from which the denoiser samples. Lower values preserve more details, but may give blotchy results if the lightmap quality is not high enough. Only effective if use_denoiser is true and ProjectSettings.rendering/lightmapping/denoising/denoiser is set to JNLM.

### bias {#bias}

```gdscript
bias: float
```

- void set_bias(value: float)
- float get_bias()

The bias to use when computing shadows. Increasing bias can fix shadow acne on the resulting baked lightmap, but can introduce peter-panning (shadows not connecting to their casters). Real-time Light3D shadows are not affected by this bias property.

### texel_scale {#texel_scale}

```gdscript
texel_scale: float
```

- void set_texel_scale(value: float)
- float get_texel_scale()

Scales the lightmap texel density of all meshes for the current bake. This is a multiplier that builds upon the existing lightmap texel size defined in each imported 3D scene, along with the per-mesh density multiplier (which is designed to be used when the same mesh is used at different scales). Lower values will result in faster bake times.

For example, doubling texel_scale doubles the lightmap texture resolution for all objects on each axis, so it will quadruple the texel count.

### max_texture_size {#max_texture_size}

```gdscript
max_texture_size: int
```

- void set_max_texture_size(value: int)
- int get_max_texture_size()

The maximum texture size for the generated texture atlas. Higher values will result in fewer slices being generated, but may not work on all hardware as a result of hardware limitations on texture sizes. Leave max_texture_size at its default value of 16384 if unsure.

### environment_mode {#environment_mode}

```gdscript
environment_mode: int
```

- void set_environment_mode(value: EnvironmentMode)
- EnvironmentMode get_environment_mode()

The environment mode to use when baking lightmaps.

### environment_custom_sky {#environment_custom_sky}

```gdscript
environment_custom_sky: Sky
```

- void set_environment_custom_sky(value: Sky)
- Sky get_environment_custom_sky()

The sky to use as a source of environment lighting. Only effective if environment_mode is ENVIRONMENT_MODE_CUSTOM_SKY.

### environment_custom_color {#environment_custom_color}

```gdscript
environment_custom_color: Color
```

- void set_environment_custom_color(value: Color)
- Color get_environment_custom_color()

The color to use for environment lighting. Only effective if environment_mode is ENVIRONMENT_MODE_CUSTOM_COLOR.

### environment_custom_energy {#environment_custom_energy}

```gdscript
environment_custom_energy: float
```

- void set_environment_custom_energy(value: float)
- float get_environment_custom_energy()

The color multiplier to use for environment lighting. Only effective if environment_mode is ENVIRONMENT_MODE_CUSTOM_COLOR.

### camera_attributes {#camera_attributes}

```gdscript
camera_attributes: CameraAttributesPractical,CameraAttributesPhysical
```

- void set_camera_attributes(value: CameraAttributes)
- CameraAttributes get_camera_attributes()

The CameraAttributes resource that specifies exposure levels to bake at. Auto-exposure and non exposure properties will be ignored. Exposure settings should be used to reduce the dynamic range present when baking. If exposure is too high, the LightmapGI will have banding artifacts or may have over-exposure artifacts.

### generate_probes_subdiv {#generate_probes_subdiv}

```gdscript
generate_probes_subdiv: int
```

- void set_generate_probes(value: GenerateProbes)
- GenerateProbes get_generate_probes()

The level of subdivision to use when automatically generating LightmapProbes for dynamic object lighting. Higher values result in more accurate indirect lighting on dynamic objects, at the cost of longer bake times and larger file sizes.

Note: Automatically generated LightmapProbes are not visible as nodes in the Scene tree dock, and cannot be modified this way after they are generated.

Note: Regardless of generate_probes_subdiv, direct lighting on dynamic objects is always applied using Light3D nodes in real-time.

### light_data {#light_data}

```gdscript
light_data: LightmapGIData
```

- void set_light_data(value: LightmapGIData)
- LightmapGIData get_light_data()

The LightmapGIData associated to this LightmapGI node. This resource is automatically created after baking, and is not meant to be created manually.

## 枚举

### BakeQuality {#enum-BakeQuality}

```gdscript
enum BakeQuality { BAKE_QUALITY_LOW = 0, BAKE_QUALITY_MEDIUM = 1, BAKE_QUALITY_HIGH = 2, BAKE_QUALITY_ULTRA = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BAKE_QUALITY_LOW {#BAKE_QUALITY_LOW}

```gdscript
BAKE_QUALITY_LOW = 0
```

Low bake quality (fastest bake times). The quality of this preset can be adjusted by changing ProjectSettings.rendering/lightmapping/bake_quality/low_quality_ray_count and ProjectSettings.rendering/lightmapping/bake_quality/low_quality_probe_ray_count.

#### BAKE_QUALITY_MEDIUM {#BAKE_QUALITY_MEDIUM}

```gdscript
BAKE_QUALITY_MEDIUM = 1
```

Medium bake quality (fast bake times). The quality of this preset can be adjusted by changing ProjectSettings.rendering/lightmapping/bake_quality/medium_quality_ray_count and ProjectSettings.rendering/lightmapping/bake_quality/medium_quality_probe_ray_count.

#### BAKE_QUALITY_HIGH {#BAKE_QUALITY_HIGH}

```gdscript
BAKE_QUALITY_HIGH = 2
```

High bake quality (slow bake times). The quality of this preset can be adjusted by changing ProjectSettings.rendering/lightmapping/bake_quality/high_quality_ray_count and ProjectSettings.rendering/lightmapping/bake_quality/high_quality_probe_ray_count.

#### BAKE_QUALITY_ULTRA {#BAKE_QUALITY_ULTRA}

```gdscript
BAKE_QUALITY_ULTRA = 3
```

Highest bake quality (slowest bake times). The quality of this preset can be adjusted by changing ProjectSettings.rendering/lightmapping/bake_quality/ultra_quality_ray_count and ProjectSettings.rendering/lightmapping/bake_quality/ultra_quality_probe_ray_count.

---

### GenerateProbes {#enum-GenerateProbes}

```gdscript
enum GenerateProbes { GENERATE_PROBES_DISABLED = 0, GENERATE_PROBES_SUBDIV_4 = 1, GENERATE_PROBES_SUBDIV_8 = 2, GENERATE_PROBES_SUBDIV_16 = 3, GENERATE_PROBES_SUBDIV_32 = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GENERATE_PROBES_DISABLED {#GENERATE_PROBES_DISABLED}

```gdscript
GENERATE_PROBES_DISABLED = 0
```

Don't generate lightmap probes for lighting dynamic objects.

#### GENERATE_PROBES_SUBDIV_4 {#GENERATE_PROBES_SUBDIV_4}

```gdscript
GENERATE_PROBES_SUBDIV_4 = 1
```

Lowest level of subdivision (fastest bake times, smallest file sizes).

#### GENERATE_PROBES_SUBDIV_8 {#GENERATE_PROBES_SUBDIV_8}

```gdscript
GENERATE_PROBES_SUBDIV_8 = 2
```

Low level of subdivision (fast bake times, small file sizes).

#### GENERATE_PROBES_SUBDIV_16 {#GENERATE_PROBES_SUBDIV_16}

```gdscript
GENERATE_PROBES_SUBDIV_16 = 3
```

High level of subdivision (slow bake times, large file sizes).

#### GENERATE_PROBES_SUBDIV_32 {#GENERATE_PROBES_SUBDIV_32}

```gdscript
GENERATE_PROBES_SUBDIV_32 = 4
```

Highest level of subdivision (slowest bake times, largest file sizes).

---

### BakeError {#enum-BakeError}

```gdscript
enum BakeError { BAKE_ERROR_OK = 0, BAKE_ERROR_NO_SCENE_ROOT = 1, BAKE_ERROR_FOREIGN_DATA = 2, BAKE_ERROR_NO_LIGHTMAPPER = 3, BAKE_ERROR_NO_SAVE_PATH = 4, BAKE_ERROR_NO_MESHES = 5, BAKE_ERROR_MESHES_INVALID = 6, BAKE_ERROR_CANT_CREATE_IMAGE = 7, BAKE_ERROR_USER_ABORTED = 8, BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL = 9, BAKE_ERROR_LIGHTMAP_TOO_SMALL = 10, BAKE_ERROR_ATLAS_TOO_SMALL = 11 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BAKE_ERROR_OK {#BAKE_ERROR_OK}

```gdscript
BAKE_ERROR_OK = 0
```

Lightmap baking was successful.

#### BAKE_ERROR_NO_SCENE_ROOT {#BAKE_ERROR_NO_SCENE_ROOT}

```gdscript
BAKE_ERROR_NO_SCENE_ROOT = 1
```

Lightmap baking failed because the root node for the edited scene could not be accessed.

#### BAKE_ERROR_FOREIGN_DATA {#BAKE_ERROR_FOREIGN_DATA}

```gdscript
BAKE_ERROR_FOREIGN_DATA = 2
```

Lightmap baking failed as the lightmap data resource is embedded in a foreign resource.

#### BAKE_ERROR_NO_LIGHTMAPPER {#BAKE_ERROR_NO_LIGHTMAPPER}

```gdscript
BAKE_ERROR_NO_LIGHTMAPPER = 3
```

Lightmap baking failed as there is no lightmapper available in this Godot build.

#### BAKE_ERROR_NO_SAVE_PATH {#BAKE_ERROR_NO_SAVE_PATH}

```gdscript
BAKE_ERROR_NO_SAVE_PATH = 4
```

Lightmap baking failed as the LightmapGIData save path isn't configured in the resource.

#### BAKE_ERROR_NO_MESHES {#BAKE_ERROR_NO_MESHES}

```gdscript
BAKE_ERROR_NO_MESHES = 5
```

Lightmap baking failed as there are no meshes whose GeometryInstance3D.gi_mode is GeometryInstance3D.GI_MODE_STATIC and with valid UV2 mapping in the current scene. You may need to select 3D scenes in the Import dock and change their global illumination mode accordingly.

#### BAKE_ERROR_MESHES_INVALID {#BAKE_ERROR_MESHES_INVALID}

```gdscript
BAKE_ERROR_MESHES_INVALID = 6
```

Lightmap baking failed as the lightmapper failed to analyze some of the meshes marked as static for baking.

#### BAKE_ERROR_CANT_CREATE_IMAGE {#BAKE_ERROR_CANT_CREATE_IMAGE}

```gdscript
BAKE_ERROR_CANT_CREATE_IMAGE = 7
```

Lightmap baking failed as the resulting image couldn't be saved or imported by Godot after it was saved.

#### BAKE_ERROR_USER_ABORTED {#BAKE_ERROR_USER_ABORTED}

```gdscript
BAKE_ERROR_USER_ABORTED = 8
```

The user aborted the lightmap baking operation (typically by clicking the Cancel button in the progress dialog).

#### BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL {#BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL}

```gdscript
BAKE_ERROR_TEXTURE_SIZE_TOO_SMALL = 9
```

Lightmap baking failed as the maximum texture size is too small to fit some of the meshes marked for baking.

#### BAKE_ERROR_LIGHTMAP_TOO_SMALL {#BAKE_ERROR_LIGHTMAP_TOO_SMALL}

```gdscript
BAKE_ERROR_LIGHTMAP_TOO_SMALL = 10
```

Lightmap baking failed as the lightmap is too small.

#### BAKE_ERROR_ATLAS_TOO_SMALL {#BAKE_ERROR_ATLAS_TOO_SMALL}

```gdscript
BAKE_ERROR_ATLAS_TOO_SMALL = 11
```

Lightmap baking failed as the lightmap was unable to fit into an atlas.

---

### EnvironmentMode {#enum-EnvironmentMode}

```gdscript
enum EnvironmentMode { ENVIRONMENT_MODE_DISABLED = 0, ENVIRONMENT_MODE_SCENE = 1, ENVIRONMENT_MODE_CUSTOM_SKY = 2, ENVIRONMENT_MODE_CUSTOM_COLOR = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENVIRONMENT_MODE_DISABLED {#ENVIRONMENT_MODE_DISABLED}

```gdscript
ENVIRONMENT_MODE_DISABLED = 0
```

Ignore environment lighting when baking lightmaps.

#### ENVIRONMENT_MODE_SCENE {#ENVIRONMENT_MODE_SCENE}

```gdscript
ENVIRONMENT_MODE_SCENE = 1
```

Use the scene's environment lighting when baking lightmaps.

Note: If baking lightmaps in a scene with no WorldEnvironment node, this will act like ENVIRONMENT_MODE_DISABLED. The editor's preview sky and sun is not taken into account by LightmapGI when baking lightmaps.

#### ENVIRONMENT_MODE_CUSTOM_SKY {#ENVIRONMENT_MODE_CUSTOM_SKY}

```gdscript
ENVIRONMENT_MODE_CUSTOM_SKY = 2
```

Use environment_custom_sky as a source of environment lighting when baking lightmaps.

#### ENVIRONMENT_MODE_CUSTOM_COLOR {#ENVIRONMENT_MODE_CUSTOM_COLOR}

```gdscript
ENVIRONMENT_MODE_CUSTOM_COLOR = 3
```

Use environment_custom_color multiplied by environment_custom_energy as a constant source of environment lighting when baking lightmaps.

---
