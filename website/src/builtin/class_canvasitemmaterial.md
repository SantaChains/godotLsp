# CanvasItemMaterial

**继承**：[Material](class_material.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CanvasItemMaterial**

> CanvasItemMaterials provide a means of modifying the textures associated with a CanvasItem. They specialize in describing blend and lighting behaviors for textures. Use a ShaderMaterial to more fully customize a material's interactions with a CanvasItem.

<details><summary>官方英文描述</summary>

CanvasItemMaterials provide a means of modifying the textures associated with a CanvasItem. They specialize in describing blend and lighting behaviors for textures. Use a ShaderMaterial to more fully customize a material's interactions with a CanvasItem.

</details>

## 方法

### get_blend_mode {#get_blend_mode}

```gdscript
CanvasItemMaterial.BlendMode get_blend_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_light_mode {#get_light_mode}

```gdscript
CanvasItemMaterial.LightMode get_light_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_particles_anim_h_frames {#get_particles_anim_h_frames}

```gdscript
int get_particles_anim_h_frames() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_particles_anim_loop {#get_particles_anim_loop}

```gdscript
bool get_particles_anim_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_particles_anim_v_frames {#get_particles_anim_v_frames}

```gdscript
int get_particles_anim_v_frames() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_particles_animation {#get_particles_animation}

```gdscript
bool get_particles_animation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_mode {#set_blend_mode}

```gdscript
void set_blend_mode(blend_mode: CanvasItemMaterial.BlendMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_light_mode {#set_light_mode}

```gdscript
void set_light_mode(light_mode: CanvasItemMaterial.LightMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_particles_anim_h_frames {#set_particles_anim_h_frames}

```gdscript
void set_particles_anim_h_frames(frames: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_particles_anim_loop {#set_particles_anim_loop}

```gdscript
void set_particles_anim_loop(loop: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_particles_anim_v_frames {#set_particles_anim_v_frames}

```gdscript
void set_particles_anim_v_frames(frames: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_particles_animation {#set_particles_animation}

```gdscript
void set_particles_animation(particles_anim: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### blend_mode {#blend_mode}

```gdscript
blend_mode: int
```

- void set_blend_mode(value: BlendMode)
- BlendMode get_blend_mode()

The manner in which a material's rendering is applied to underlying textures.

### light_mode {#light_mode}

```gdscript
light_mode: int
```

- void set_light_mode(value: LightMode)
- LightMode get_light_mode()

The manner in which material reacts to lighting.

### particles_animation {#particles_animation}

```gdscript
particles_animation: bool
```

- void set_particles_animation(value: bool)
- bool get_particles_animation()

If true, enable spritesheet-based animation features when assigned to GPUParticles2D and CPUParticles2D nodes. The ParticleProcessMaterial.anim_speed_max or CPUParticles2D.anim_speed_max should also be set to a positive value for the animation to play.

This property (and other particles_anim_* properties that depend on it) has no effect on other types of nodes.

### particles_anim_h_frames {#particles_anim_h_frames}

```gdscript
particles_anim_h_frames: int
```

- void set_particles_anim_h_frames(value: int)
- int get_particles_anim_h_frames()

The number of columns in the spritesheet assigned as Texture2D for a GPUParticles2D or CPUParticles2D.

Note: This property is only used and visible in the editor if particles_animation is true.

### particles_anim_v_frames {#particles_anim_v_frames}

```gdscript
particles_anim_v_frames: int
```

- void set_particles_anim_v_frames(value: int)
- int get_particles_anim_v_frames()

The number of rows in the spritesheet assigned as Texture2D for a GPUParticles2D or CPUParticles2D.

Note: This property is only used and visible in the editor if particles_animation is true.

### particles_anim_loop {#particles_anim_loop}

```gdscript
particles_anim_loop: bool
```

- void set_particles_anim_loop(value: bool)
- bool get_particles_anim_loop()

If true, the particles animation will loop.

Note: This property is only used and visible in the editor if particles_animation is true.

## 枚举

### BlendMode {#enum-BlendMode}

```gdscript
enum BlendMode { BLEND_MODE_MIX = 0, BLEND_MODE_ADD = 1, BLEND_MODE_SUB = 2, BLEND_MODE_MUL = 3, BLEND_MODE_PREMULT_ALPHA = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BLEND_MODE_MIX {#BLEND_MODE_MIX}

```gdscript
BLEND_MODE_MIX = 0
```

Mix blending mode. Colors are assumed to be independent of the alpha (opacity) value.

#### BLEND_MODE_ADD {#BLEND_MODE_ADD}

```gdscript
BLEND_MODE_ADD = 1
```

Additive blending mode.

#### BLEND_MODE_SUB {#BLEND_MODE_SUB}

```gdscript
BLEND_MODE_SUB = 2
```

Subtractive blending mode.

#### BLEND_MODE_MUL {#BLEND_MODE_MUL}

```gdscript
BLEND_MODE_MUL = 3
```

Multiplicative blending mode.

#### BLEND_MODE_PREMULT_ALPHA {#BLEND_MODE_PREMULT_ALPHA}

```gdscript
BLEND_MODE_PREMULT_ALPHA = 4
```

Mix blending mode. Colors are assumed to be premultiplied by the alpha (opacity) value.

---

### LightMode {#enum-LightMode}

```gdscript
enum LightMode { LIGHT_MODE_NORMAL = 0, LIGHT_MODE_UNSHADED = 1, LIGHT_MODE_LIGHT_ONLY = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_MODE_NORMAL {#LIGHT_MODE_NORMAL}

```gdscript
LIGHT_MODE_NORMAL = 0
```

Render the material using both light and non-light sensitive material properties.

#### LIGHT_MODE_UNSHADED {#LIGHT_MODE_UNSHADED}

```gdscript
LIGHT_MODE_UNSHADED = 1
```

Render the material as if there were no light.

#### LIGHT_MODE_LIGHT_ONLY {#LIGHT_MODE_LIGHT_ONLY}

```gdscript
LIGHT_MODE_LIGHT_ONLY = 2
```

Render the material as if there were only light.

---
