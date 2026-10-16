# FogMaterial

**继承**：[Material](class_material.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **FogMaterial**

> A Material resource that can be used by FogVolumes to draw volumetric effects.

<details><summary>官方英文描述（中文精译进行中）</summary>

A Material resource that can be used by FogVolumes to draw volumetric effects.

If you need more advanced effects, use a custom fog shader.

</details>

## 方法

### get_albedo {#get_albedo}

```gdscript
Color get_albedo() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_density {#get_density}

```gdscript
float get_density() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_density_texture {#get_density_texture}

```gdscript
Texture3D get_density_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_edge_fade {#get_edge_fade}

```gdscript
float get_edge_fade() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission {#get_emission}

```gdscript
Color get_emission() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_height_falloff {#get_height_falloff}

```gdscript
float get_height_falloff() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_albedo {#set_albedo}

```gdscript
void set_albedo(albedo: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_density {#set_density}

```gdscript
void set_density(density: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_density_texture {#set_density_texture}

```gdscript
void set_density_texture(density_texture: Texture3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edge_fade {#set_edge_fade}

```gdscript
void set_edge_fade(edge_fade: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission {#set_emission}

```gdscript
void set_emission(emission: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height_falloff {#set_height_falloff}

```gdscript
void set_height_falloff(height_falloff: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### density {#density}

```gdscript
density: float
```

- void set_density(value: float)
- float get_density()

The density of the FogVolume. Denser objects are more opaque, but may suffer from under-sampling artifacts that look like stripes. Negative values can be used to subtract fog from other FogVolumes or global volumetric fog.

Note: Due to limited precision, density values between -0.001 and 0.001 (exclusive) act like 0.0. This does not apply to Environment.volumetric_fog_density.

### albedo {#albedo}

```gdscript
albedo: Color
```

- void set_albedo(value: Color)
- Color get_albedo()

The single-scattering Color of the FogVolume. Internally, albedo is converted into single-scattering, which is additively blended with other FogVolumes and the Environment.volumetric_fog_albedo.

### emission {#emission}

```gdscript
emission: Color
```

- void set_emission(value: Color)
- Color get_emission()

The Color of the light emitted by the FogVolume. Emitted light will not cast light or shadows on other objects, but can be useful for modulating the Color of the FogVolume independently from light sources.

### height_falloff {#height_falloff}

```gdscript
height_falloff: float
```

- void set_height_falloff(value: float)
- float get_height_falloff()

The rate by which the height-based fog decreases in density as height increases in world space. A high falloff will result in a sharp transition, while a low falloff will result in a smoother transition. A value of 0.0 results in uniform-density fog. The height threshold is determined by the height of the associated FogVolume.

### edge_fade {#edge_fade}

```gdscript
edge_fade: float
```

- void set_edge_fade(value: float)
- float get_edge_fade()

The hardness of the edges of the FogVolume. A higher value will result in softer edges, while a lower value will result in harder edges.

### density_texture {#density_texture}

```gdscript
density_texture: Texture3D
```

- void set_density_texture(value: Texture3D)
- Texture3D get_density_texture()

The 3D texture that is used to scale the density of the FogVolume. This can be used to vary fog density within the FogVolume with any kind of static pattern. For animated effects, consider using a custom fog shader.
