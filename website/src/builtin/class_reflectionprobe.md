# ReflectionProbe

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **ReflectionProbe**

> Captures its surroundings as a cubemap, and stores versions of it with increasing levels of blur to simulate different material roughnesses.

<details><summary>官方英文描述（中文精译进行中）</summary>

Captures its surroundings as a cubemap, and stores versions of it with increasing levels of blur to simulate different material roughnesses.

The ReflectionProbe is used to create high-quality reflections at a low performance cost (when update_mode is UPDATE_ONCE). ReflectionProbes can be blended together and with the rest of the scene smoothly. ReflectionProbes can also be combined with VoxelGI, SDFGI (Environment.sdfgi_enabled) and screen-space reflections (Environment.ssr_enabled) to get more accurate reflections in specific areas. ReflectionProbes render all objects within their cull_mask, so updating them can be quite expensive. It is best to update them once with the important static objects and then leave them as-is.

Note: Unlike VoxelGI and SDFGI, ReflectionProbes only source their environment from a WorldEnvironment node. If you specify an Environment resource within a Camera3D node, it will be ignored by the ReflectionProbe. This can lead to incorrect lighting within the ReflectionProbe.

Note: When using the Mobile rendering method, only 8 reflection probes can be displayed on each mesh resource, while the Compatibility rendering method only supports up to 2 reflection probes on each mesh. Attempting to display more than 8 reflection probes on a single mesh resource using the Mobile renderer will result in reflection probes flickering in and out as the camera moves, while the Compatibility renderer will not render any additional probes if more than 2 reflection probes are being used.

Note: When using the Mobile rendering method, reflection probes will only correctly affect meshes whose visibility AABB intersects with the reflection probe's AABB. If using a shader to deform the mesh in a way that makes it go outside its AABB, GeometryInstance3D.extra_cull_margin must be increased on the mesh. Otherwise, the reflection probe may not be visible on the mesh.

</details>

## 方法

### are_shadows_enabled {#are_shadows_enabled}

```gdscript
bool are_shadows_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ambient_color {#get_ambient_color}

```gdscript
Color get_ambient_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ambient_color_energy {#get_ambient_color_energy}

```gdscript
float get_ambient_color_energy() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ambient_mode {#get_ambient_mode}

```gdscript
ReflectionProbe.AmbientMode get_ambient_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_blend_distance {#get_blend_distance}

```gdscript
float get_blend_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cull_mask {#get_cull_mask}

```gdscript
int get_cull_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_intensity {#get_intensity}

```gdscript
float get_intensity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_distance {#get_max_distance}

```gdscript
float get_max_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh_lod_threshold {#get_mesh_lod_threshold}

```gdscript
float get_mesh_lod_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_origin_offset {#get_origin_offset}

```gdscript
Vector3 get_origin_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_reflection_mask {#get_reflection_mask}

```gdscript
int get_reflection_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_update_mode {#get_update_mode}

```gdscript
ReflectionProbe.UpdateMode get_update_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_box_projection_enabled {#is_box_projection_enabled}

```gdscript
bool is_box_projection_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_set_as_interior {#is_set_as_interior}

```gdscript
bool is_set_as_interior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ambient_color {#set_ambient_color}

```gdscript
void set_ambient_color(ambient: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ambient_color_energy {#set_ambient_color_energy}

```gdscript
void set_ambient_color_energy(ambient_energy: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ambient_mode {#set_ambient_mode}

```gdscript
void set_ambient_mode(ambient: ReflectionProbe.AmbientMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_as_interior {#set_as_interior}

```gdscript
void set_as_interior(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_distance {#set_blend_distance}

```gdscript
void set_blend_distance(blend_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cull_mask {#set_cull_mask}

```gdscript
void set_cull_mask(layers: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_box_projection {#set_enable_box_projection}

```gdscript
void set_enable_box_projection(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_shadows {#set_enable_shadows}

```gdscript
void set_enable_shadows(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_intensity {#set_intensity}

```gdscript
void set_intensity(intensity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_distance {#set_max_distance}

```gdscript
void set_max_distance(max_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh_lod_threshold {#set_mesh_lod_threshold}

```gdscript
void set_mesh_lod_threshold(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_origin_offset {#set_origin_offset}

```gdscript
void set_origin_offset(origin_offset: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_reflection_mask {#set_reflection_mask}

```gdscript
void set_reflection_mask(layers: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_update_mode {#set_update_mode}

```gdscript
void set_update_mode(mode: ReflectionProbe.UpdateMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### update_mode {#update_mode}

```gdscript
update_mode: int
```

- void set_update_mode(value: UpdateMode)
- UpdateMode get_update_mode()

Sets how frequently the ReflectionProbe is updated. Can be UPDATE_ONCE or UPDATE_ALWAYS.

### intensity {#intensity}

```gdscript
intensity: float
```

- void set_intensity(value: float)
- float get_intensity()

Defines the reflection intensity. Intensity modulates the strength of the reflection.

### blend_distance {#blend_distance}

```gdscript
blend_distance: float
```

- void set_blend_distance(value: float)
- float get_blend_distance()

Defines the distance in meters over which a probe blends into the scene.

### max_distance {#max_distance}

```gdscript
max_distance: float
```

- void set_max_distance(value: float)
- float get_max_distance()

The maximum distance away from the ReflectionProbe an object can be before it is culled. Decrease this to improve performance, especially when using the UPDATE_ALWAYS update_mode.

Note: The maximum reflection distance is always at least equal to the probe's extents. This means that decreasing max_distance will not always cull objects from reflections, especially if the reflection probe's box defined by its size is already large.

### size {#size}

```gdscript
size: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### origin_offset {#origin_offset}

```gdscript
origin_offset: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### box_projection {#box_projection}

```gdscript
box_projection: bool
```

- void set_enable_box_projection(value: bool)
- bool is_box_projection_enabled()

If true, enables box projection. This makes reflections look more correct in rectangle-shaped rooms by offsetting the reflection center depending on the camera's location.

Note: To better fit rectangle-shaped rooms that are not aligned to the grid, you can rotate the ReflectionProbe node.

### interior {#interior}

```gdscript
interior: bool
```

- void set_as_interior(value: bool)
- bool is_set_as_interior()

If true, reflections will ignore sky contribution.

### enable_shadows {#enable_shadows}

```gdscript
enable_shadows: bool
```

- void set_enable_shadows(value: bool)
- bool are_shadows_enabled()

If true, computes shadows in the reflection probe. This makes the reflection probe slower to render; you may want to disable this if using the UPDATE_ALWAYS update_mode.

### cull_mask {#cull_mask}

```gdscript
cull_mask: int
```

- void set_cull_mask(value: int)
- int get_cull_mask()

Sets the cull mask which determines what objects are drawn by this probe. Every VisualInstance3D with a layer included in this cull mask will be rendered by the probe. It is best to only include large objects which are likely to take up a lot of space in the reflection in order to save on rendering cost.

This can also be used to prevent an object from reflecting upon itself (for instance, a ReflectionProbe centered on a vehicle).

### reflection_mask {#reflection_mask}

```gdscript
reflection_mask: int
```

- void set_reflection_mask(value: int)
- int get_reflection_mask()

Sets the reflection mask which determines what objects have reflections applied from this probe. Every VisualInstance3D with a layer included in this reflection mask will have reflections applied from this probe. See also cull_mask, which can be used to exclude objects from appearing in the reflection while still making them affected by the ReflectionProbe.

### mesh_lod_threshold {#mesh_lod_threshold}

```gdscript
mesh_lod_threshold: float
```

- void set_mesh_lod_threshold(value: float)
- float get_mesh_lod_threshold()

The automatic LOD bias to use for meshes rendered within the ReflectionProbe (this is analog to Viewport.mesh_lod_threshold). Higher values will use less detailed versions of meshes that have LOD variations generated. If set to 0.0, automatic LOD is disabled. Increase mesh_lod_threshold to improve performance at the cost of geometry detail, especially when using the UPDATE_ALWAYS update_mode.

Note: mesh_lod_threshold does not affect GeometryInstance3D visibility ranges (also known as "manual" LOD or hierarchical LOD).

### ambient_mode {#ambient_mode}

```gdscript
ambient_mode: int
```

- void set_ambient_mode(value: AmbientMode)
- AmbientMode get_ambient_mode()

The ambient color to use within the ReflectionProbe's box defined by its size. The ambient color will smoothly blend with other ReflectionProbes and the rest of the scene (outside the ReflectionProbe's box defined by its size).

### ambient_color {#ambient_color}

```gdscript
ambient_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ambient_color_energy {#ambient_color_energy}

```gdscript
ambient_color_energy: float
```

- void set_ambient_color_energy(value: float)
- float get_ambient_color_energy()

The custom ambient color energy to use within the ReflectionProbe's box defined by its size. Only effective if ambient_mode is AMBIENT_COLOR.

## 枚举

### UpdateMode {#enum-UpdateMode}

```gdscript
enum UpdateMode { UPDATE_ONCE = 0, UPDATE_ALWAYS = 1 }
```

UpdateMode UPDATE_ONCE = 0

Update the probe once on the next frame (recommended for most objects). The corresponding radiance map will be generated over the following six frames. This takes more time to update than UPDATE_ALWAYS, but it has a lower performance cost and can result in higher-quality reflections. The ReflectionProbe is updated when its transform changes, but not when nearby geometry changes. You can force a ReflectionProbe update by moving the ReflectionProbe slightly in any direction.

UpdateMode UPDATE_ALWAYS = 1

Update the probe every frame. This provides better results for fast-moving dynamic objects (such as cars). However, it has a significant performance cost. Due to the cost, it's recommended to only use one ReflectionProbe with UPDATE_ALWAYS at most per scene. For all other use cases, use UPDATE_ONCE.

#### UPDATE_ONCE {#UPDATE_ONCE}

```gdscript
UPDATE_ONCE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_ALWAYS {#UPDATE_ALWAYS}

```gdscript
UPDATE_ALWAYS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### AmbientMode {#enum-AmbientMode}

```gdscript
enum AmbientMode { AMBIENT_DISABLED = 0, AMBIENT_ENVIRONMENT = 1, AMBIENT_COLOR = 2 }
```

AmbientMode AMBIENT_DISABLED = 0

Do not apply any ambient lighting inside the ReflectionProbe's box defined by its size.

AmbientMode AMBIENT_ENVIRONMENT = 1

Apply automatically-sourced environment lighting inside the ReflectionProbe's box defined by its size.

AmbientMode AMBIENT_COLOR = 2

Apply custom ambient lighting inside the ReflectionProbe's box defined by its size. See ambient_color and ambient_color_energy.

#### AMBIENT_DISABLED {#AMBIENT_DISABLED}

```gdscript
AMBIENT_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AMBIENT_ENVIRONMENT {#AMBIENT_ENVIRONMENT}

```gdscript
AMBIENT_ENVIRONMENT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AMBIENT_COLOR {#AMBIENT_COLOR}

```gdscript
AMBIENT_COLOR = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
