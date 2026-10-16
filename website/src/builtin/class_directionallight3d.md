# DirectionalLight3D

**继承**：[Light3D](class_light3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **DirectionalLight3D**

> A directional light is a type of Light3D node that models an infinite number of parallel rays covering the entire scene. It is used for lights with strong intensity that are located far away from the scene to model sunlight or moonlight.

<details><summary>官方英文描述（中文精译进行中）</summary>

A directional light is a type of Light3D node that models an infinite number of parallel rays covering the entire scene. It is used for lights with strong intensity that are located far away from the scene to model sunlight or moonlight.

Light is emitted in the -Z direction of the node's global basis. For an unrotated light, this means that the light is emitted forwards, illuminating the front side of a 3D model (see Vector3.FORWARD and Vector3.MODEL_FRONT). The position of the node is ignored; only the basis is used to determine light direction.

</details>

## 方法

### get_shadow_mode {#get_shadow_mode}

```gdscript
DirectionalLight3D.ShadowMode get_shadow_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sky_mode {#get_sky_mode}

```gdscript
DirectionalLight3D.SkyMode get_sky_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_blend_splits_enabled {#is_blend_splits_enabled}

```gdscript
bool is_blend_splits_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_splits {#set_blend_splits}

```gdscript
void set_blend_splits(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_mode {#set_shadow_mode}

```gdscript
void set_shadow_mode(mode: DirectionalLight3D.ShadowMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sky_mode {#set_sky_mode}

```gdscript
void set_sky_mode(mode: DirectionalLight3D.SkyMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### directional_shadow_mode {#directional_shadow_mode}

```gdscript
directional_shadow_mode: int
```

- void set_shadow_mode(value: ShadowMode)
- ShadowMode get_shadow_mode()

The light's shadow rendering algorithm.

### directional_shadow_split_1 {#directional_shadow_split_1}

```gdscript
directional_shadow_split_1: float
```

- void set_param(value: float)
- float get_param()

The distance from camera to shadow split 1. Relative to directional_shadow_max_distance. Only used when directional_shadow_mode is SHADOW_PARALLEL_2_SPLITS or SHADOW_PARALLEL_4_SPLITS.

### directional_shadow_split_2 {#directional_shadow_split_2}

```gdscript
directional_shadow_split_2: float
```

- void set_param(value: float)
- float get_param()

The distance from shadow split 1 to split 2. Relative to directional_shadow_max_distance. Only used when directional_shadow_mode is SHADOW_PARALLEL_4_SPLITS.

### directional_shadow_split_3 {#directional_shadow_split_3}

```gdscript
directional_shadow_split_3: float
```

- void set_param(value: float)
- float get_param()

The distance from shadow split 2 to split 3. Relative to directional_shadow_max_distance. Only used when directional_shadow_mode is SHADOW_PARALLEL_4_SPLITS.

### directional_shadow_blend_splits {#directional_shadow_blend_splits}

```gdscript
directional_shadow_blend_splits: bool
```

- void set_blend_splits(value: bool)
- bool is_blend_splits_enabled()

If true, shadow detail is sacrificed in exchange for smoother transitions between splits. Enabling shadow blend splitting also has a moderate performance cost. This is ignored when directional_shadow_mode is SHADOW_ORTHOGONAL.

### directional_shadow_fade_start {#directional_shadow_fade_start}

```gdscript
directional_shadow_fade_start: float
```

- void set_param(value: float)
- float get_param()

Proportion of directional_shadow_max_distance at which point the shadow starts to fade. At directional_shadow_max_distance, the shadow will disappear. The default value is a balance between smooth fading and distant shadow visibility. If the camera moves fast and the directional_shadow_max_distance is low, consider lowering directional_shadow_fade_start below 0.8 to make shadow transitions less noticeable. On the other hand, if you tuned directional_shadow_max_distance to cover the entire scene, you can set directional_shadow_fade_start to 1.0 to prevent the shadow from fading in the distance (it will suddenly cut off instead).

### directional_shadow_max_distance {#directional_shadow_max_distance}

```gdscript
directional_shadow_max_distance: float
```

- void set_param(value: float)
- float get_param()

The maximum distance for shadow splits. Increasing this value will make directional shadows visible from further away, at the cost of lower overall shadow detail and performance (since more objects need to be included in the directional shadow rendering).

### directional_shadow_pancake_size {#directional_shadow_pancake_size}

```gdscript
directional_shadow_pancake_size: float
```

- void set_param(value: float)
- float get_param()

Sets the size of the directional shadow pancake. The pancake offsets the start of the shadow's camera frustum to provide a higher effective depth resolution for the shadow. However, a high pancake size can cause artifacts in the shadows of large objects that are close to the edge of the frustum. Reducing the pancake size can help. Setting the size to 0 turns off the pancaking effect.

### sky_mode {#sky_mode}

```gdscript
sky_mode: int
```

- void set_sky_mode(value: SkyMode)
- SkyMode get_sky_mode()

Whether this DirectionalLight3D is visible in the sky, in the scene, or both in the sky and in the scene.

## 枚举

### ShadowMode {#enum-ShadowMode}

```gdscript
enum ShadowMode { SHADOW_ORTHOGONAL = 0, SHADOW_PARALLEL_2_SPLITS = 1, SHADOW_PARALLEL_4_SPLITS = 2 }
```

ShadowMode SHADOW_ORTHOGONAL = 0

Renders the entire scene's shadow map from an orthogonal point of view. This is the fastest directional shadow mode. May result in blurrier shadows on close objects.

ShadowMode SHADOW_PARALLEL_2_SPLITS = 1

Splits the view frustum in 2 areas, each with its own shadow map. This shadow mode is a compromise between SHADOW_ORTHOGONAL and SHADOW_PARALLEL_4_SPLITS in terms of performance.

ShadowMode SHADOW_PARALLEL_4_SPLITS = 2

Splits the view frustum in 4 areas, each with its own shadow map. This is the slowest directional shadow mode.

#### SHADOW_ORTHOGONAL {#SHADOW_ORTHOGONAL}

```gdscript
SHADOW_ORTHOGONAL = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHADOW_PARALLEL_2_SPLITS {#SHADOW_PARALLEL_2_SPLITS}

```gdscript
SHADOW_PARALLEL_2_SPLITS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHADOW_PARALLEL_4_SPLITS {#SHADOW_PARALLEL_4_SPLITS}

```gdscript
SHADOW_PARALLEL_4_SPLITS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### SkyMode {#enum-SkyMode}

```gdscript
enum SkyMode { SKY_MODE_LIGHT_AND_SKY = 0, SKY_MODE_LIGHT_ONLY = 1, SKY_MODE_SKY_ONLY = 2 }
```

SkyMode SKY_MODE_LIGHT_AND_SKY = 0

Makes the light visible in both scene lighting and sky rendering.

SkyMode SKY_MODE_LIGHT_ONLY = 1

Makes the light visible in scene lighting only (including direct lighting and global illumination). When using this mode, the light will not be visible from sky shaders.

SkyMode SKY_MODE_SKY_ONLY = 2

Makes the light visible to sky shaders only. When using this mode the light will not cast light into the scene (either through direct lighting or through global illumination), but can be accessed through sky shaders. This can be useful, for example, when you want to control sky effects without illuminating the scene (during a night cycle, for example).

#### SKY_MODE_LIGHT_AND_SKY {#SKY_MODE_LIGHT_AND_SKY}

```gdscript
SKY_MODE_LIGHT_AND_SKY = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SKY_MODE_LIGHT_ONLY {#SKY_MODE_LIGHT_ONLY}

```gdscript
SKY_MODE_LIGHT_ONLY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SKY_MODE_SKY_ONLY {#SKY_MODE_SKY_ONLY}

```gdscript
SKY_MODE_SKY_ONLY = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
