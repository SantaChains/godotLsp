# GPUParticlesCollisionSDF3D

**继承**：[GPUParticlesCollision3D](class_gpuparticlescollision3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **GPUParticlesCollisionSDF3D**

> A baked signed distance field 3D particle collision shape affecting GPUParticles3D nodes.

<details><summary>官方英文描述（中文精译进行中）</summary>

A baked signed distance field 3D particle collision shape affecting GPUParticles3D nodes.

Signed distance fields (SDF) allow for efficiently representing approximate collision shapes for convex and concave objects of any shape. This is more flexible than GPUParticlesCollisionHeightField3D, but it requires a baking step.

Baking: The signed distance field texture can be baked by selecting the GPUParticlesCollisionSDF3D node in the editor, then clicking Bake SDF at the top of the 3D viewport. Any visible MeshInstance3Ds within the size will be taken into account for baking, regardless of their GeometryInstance3D.gi_mode.

Note: Baking a GPUParticlesCollisionSDF3D's texture is only possible within the editor, as there is no bake method exposed for use in exported projects. However, it's still possible to load pre-baked Texture3Ds into its texture property in an exported project.

Note: ParticleProcessMaterial.collision_mode must be ParticleProcessMaterial.COLLISION_RIGID or ParticleProcessMaterial.COLLISION_HIDE_ON_CONTACT on the GPUParticles3D's process material for collision to work.

Note: Particle collision only affects GPUParticles3D, not CPUParticles3D.

</details>

## 方法

### get_bake_mask {#get_bake_mask}

```gdscript
int get_bake_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bake_mask_value {#get_bake_mask_value}

```gdscript
bool get_bake_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the bake_mask is enabled, given a layer_number between 1 and 32.

### get_resolution {#get_resolution}

```gdscript
GPUParticlesCollisionSDF3D.Resolution get_resolution() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture3D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_thickness {#get_thickness}

```gdscript
float get_thickness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bake_mask {#set_bake_mask}

```gdscript
void set_bake_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bake_mask_value {#set_bake_mask_value}

```gdscript
void set_bake_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the bake_mask, given a layer_number between 1 and 32.

### set_resolution {#set_resolution}

```gdscript
void set_resolution(resolution: GPUParticlesCollisionSDF3D.Resolution)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_thickness {#set_thickness}

```gdscript
void set_thickness(thickness: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector3
```

- void set_size(value: Vector3)
- Vector3 get_size()

The collision SDF's size in 3D units. To improve SDF quality, the size should be set as small as possible while covering the parts of the scene you need.

### resolution {#resolution}

```gdscript
resolution: int
```

- void set_resolution(value: Resolution)
- Resolution get_resolution()

The bake resolution to use for the signed distance field texture. The texture must be baked again for changes to the resolution property to be effective. Higher resolutions have a greater performance cost and take more time to bake. Higher resolutions also result in larger baked textures, leading to increased VRAM and storage space requirements. To improve performance and reduce bake times, use the lowest resolution possible for the object you're representing the collision of.

### thickness {#thickness}

```gdscript
thickness: float
```

- void set_thickness(value: float)
- float get_thickness()

The collision shape's thickness. Unlike other particle colliders, GPUParticlesCollisionSDF3D is actually hollow on the inside. thickness can be increased to prevent particles from tunneling through the collision shape at high speeds, or when the GPUParticlesCollisionSDF3D is moved.

### bake_mask {#bake_mask}

```gdscript
bake_mask: int
```

- void set_bake_mask(value: int)
- int get_bake_mask()

The visual layers to account for when baking the particle collision SDF. Only MeshInstance3Ds whose VisualInstance3D.layers match with this bake_mask will be included in the generated particle collision SDF. By default, all objects are taken into account for the particle collision SDF baking.

### texture {#texture}

```gdscript
texture: Texture3D
```

- void set_texture(value: Texture3D)
- Texture3D get_texture()

The 3D texture representing the signed distance field.

## 枚举

### Resolution {#enum-Resolution}

```gdscript
enum Resolution { RESOLUTION_16 = 0, RESOLUTION_32 = 1, RESOLUTION_64 = 2, RESOLUTION_128 = 3, RESOLUTION_256 = 4, RESOLUTION_512 = 5, RESOLUTION_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLUTION_16 {#RESOLUTION_16}

```gdscript
RESOLUTION_16 = 0
```

Bake a 16×16×16 signed distance field. This is the fastest option, but also the least precise.

#### RESOLUTION_32 {#RESOLUTION_32}

```gdscript
RESOLUTION_32 = 1
```

Bake a 32×32×32 signed distance field.

#### RESOLUTION_64 {#RESOLUTION_64}

```gdscript
RESOLUTION_64 = 2
```

Bake a 64×64×64 signed distance field.

#### RESOLUTION_128 {#RESOLUTION_128}

```gdscript
RESOLUTION_128 = 3
```

Bake a 128×128×128 signed distance field.

#### RESOLUTION_256 {#RESOLUTION_256}

```gdscript
RESOLUTION_256 = 4
```

Bake a 256×256×256 signed distance field.

#### RESOLUTION_512 {#RESOLUTION_512}

```gdscript
RESOLUTION_512 = 5
```

Bake a 512×512×512 signed distance field. This is the slowest option, but also the most precise.

#### RESOLUTION_MAX {#RESOLUTION_MAX}

```gdscript
RESOLUTION_MAX = 6
```

Represents the size of the Resolution enum.

---
