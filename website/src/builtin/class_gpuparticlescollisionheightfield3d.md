# GPUParticlesCollisionHeightField3D

**继承**：[GPUParticlesCollision3D](class_gpuparticlescollision3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **GPUParticlesCollisionHeightField3D**

> A real-time heightmap-shaped 3D particle collision shape affecting GPUParticles3D nodes.

<details><summary>官方英文描述（中文精译进行中）</summary>

A real-time heightmap-shaped 3D particle collision shape affecting GPUParticles3D nodes.

Heightmap shapes allow for efficiently representing collisions for convex and concave objects with a single "floor" (such as terrain). This is less flexible than GPUParticlesCollisionSDF3D, but it doesn't require a baking step.

GPUParticlesCollisionHeightField3D can also be regenerated in real-time when it is moved, when the camera moves, or even continuously. This makes GPUParticlesCollisionHeightField3D a good choice for weather effects such as rain and snow and games with highly dynamic geometry. However, this class is limited since heightmaps cannot represent overhangs (e.g. indoors or caves).

Note: ParticleProcessMaterial.collision_mode must be true on the GPUParticles3D's process material for collision to work.

Note: Particle collision only affects GPUParticles3D, not CPUParticles3D.

</details>

## 方法

### get_heightfield_mask {#get_heightfield_mask}

```gdscript
int get_heightfield_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_heightfield_mask_value {#get_heightfield_mask_value}

```gdscript
bool get_heightfield_mask_value(layer_number: int) const
```

Returns true if the specified layer of the heightfield_mask is enabled, given a layer_number between 1 and 20, inclusive.

### get_resolution {#get_resolution}

```gdscript
GPUParticlesCollisionHeightField3D.Resolution get_resolution() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_update_mode {#get_update_mode}

```gdscript
GPUParticlesCollisionHeightField3D.UpdateMode get_update_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_follow_camera_enabled {#is_follow_camera_enabled}

```gdscript
bool is_follow_camera_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_follow_camera_enabled {#set_follow_camera_enabled}

```gdscript
void set_follow_camera_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_heightfield_mask {#set_heightfield_mask}

```gdscript
void set_heightfield_mask(heightfield_mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_heightfield_mask_value {#set_heightfield_mask_value}

```gdscript
void set_heightfield_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the heightfield_mask, given a layer_number between 1 and 20, inclusive.

### set_resolution {#set_resolution}

```gdscript
void set_resolution(resolution: GPUParticlesCollisionHeightField3D.Resolution)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_update_mode {#set_update_mode}

```gdscript
void set_update_mode(update_mode: GPUParticlesCollisionHeightField3D.UpdateMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### resolution {#resolution}

```gdscript
resolution: int
```

- void set_resolution(value: Resolution)
- Resolution get_resolution()

Higher resolutions can represent small details more accurately in large scenes, at the cost of lower performance. If update_mode is UPDATE_MODE_ALWAYS, consider using the lowest resolution possible.

### update_mode {#update_mode}

```gdscript
update_mode: int
```

- void set_update_mode(value: UpdateMode)
- UpdateMode get_update_mode()

The update policy to use for the generated heightmap.

### follow_camera_enabled {#follow_camera_enabled}

```gdscript
follow_camera_enabled: bool
```

- void set_follow_camera_enabled(value: bool)
- bool is_follow_camera_enabled()

If true, the GPUParticlesCollisionHeightField3D will follow the current camera in global space. The GPUParticlesCollisionHeightField3D does not need to be a child of the Camera3D node for this to work.

Following the camera has a performance cost, as it will force the heightmap to update whenever the camera moves. Consider lowering resolution to improve performance if follow_camera_enabled is true.

### heightfield_mask {#heightfield_mask}

```gdscript
heightfield_mask: int
```

- void set_heightfield_mask(value: int)
- int get_heightfield_mask()

The visual layers to account for when updating the heightmap. Only MeshInstance3Ds whose VisualInstance3D.layers match with this heightfield_mask will be included in the heightmap collision update. By default, all 20 user-visible layers are taken into account for updating the heightmap collision.

Note: Since the heightfield_mask allows for 32 layers to be stored in total, there are an additional 12 layers that are only used internally by the engine and aren't exposed in the editor. Setting heightfield_mask using a script allows you to toggle those reserved layers, which can be useful for editor plugins.

To adjust heightfield_mask more easily using a script, use get_heightfield_mask_value() and set_heightfield_mask_value().

## 枚举

### Resolution {#enum-Resolution}

```gdscript
enum Resolution { RESOLUTION_256 = 0, RESOLUTION_512 = 1, RESOLUTION_1024 = 2, RESOLUTION_2048 = 3, RESOLUTION_4096 = 4, RESOLUTION_8192 = 5, RESOLUTION_MAX = 6 }
```

Resolution RESOLUTION_256 = 0

Generate a 256×256 heightmap. Intended for small-scale scenes, or larger scenes with no distant particles.

Resolution RESOLUTION_512 = 1

Generate a 512×512 heightmap. Intended for medium-scale scenes, or larger scenes with no distant particles.

Resolution RESOLUTION_1024 = 2

Generate a 1024×1024 heightmap. Intended for large scenes with distant particles.

Resolution RESOLUTION_2048 = 3

Generate a 2048×2048 heightmap. Intended for very large scenes with distant particles.

Resolution RESOLUTION_4096 = 4

Generate a 4096×4096 heightmap. Intended for huge scenes with distant particles.

Resolution RESOLUTION_8192 = 5

Generate a 8192×8192 heightmap. Intended for gigantic scenes with distant particles.

Resolution RESOLUTION_MAX = 6

Represents the size of the Resolution enum.

#### RESOLUTION_256 {#RESOLUTION_256}

```gdscript
RESOLUTION_256 = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLUTION_512 {#RESOLUTION_512}

```gdscript
RESOLUTION_512 = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLUTION_1024 {#RESOLUTION_1024}

```gdscript
RESOLUTION_1024 = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLUTION_2048 {#RESOLUTION_2048}

```gdscript
RESOLUTION_2048 = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLUTION_4096 {#RESOLUTION_4096}

```gdscript
RESOLUTION_4096 = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLUTION_8192 {#RESOLUTION_8192}

```gdscript
RESOLUTION_8192 = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLUTION_MAX {#RESOLUTION_MAX}

```gdscript
RESOLUTION_MAX = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### UpdateMode {#enum-UpdateMode}

```gdscript
enum UpdateMode { UPDATE_MODE_WHEN_MOVED = 0, UPDATE_MODE_ALWAYS = 1 }
```

UpdateMode UPDATE_MODE_WHEN_MOVED = 0

Only update the heightmap when the GPUParticlesCollisionHeightField3D node is moved, or when the camera moves if follow_camera_enabled is true. An update can be forced by slightly moving the GPUParticlesCollisionHeightField3D in any direction, or by calling RenderingServer.particles_collision_height_field_update().

UpdateMode UPDATE_MODE_ALWAYS = 1

Update the heightmap every frame. This has a significant performance cost. This update should only be used when geometry that particles can collide with changes significantly during gameplay.

#### UPDATE_MODE_WHEN_MOVED {#UPDATE_MODE_WHEN_MOVED}

```gdscript
UPDATE_MODE_WHEN_MOVED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_MODE_ALWAYS {#UPDATE_MODE_ALWAYS}

```gdscript
UPDATE_MODE_ALWAYS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
