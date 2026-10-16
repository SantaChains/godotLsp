# NoiseTexture3D

**继承**：[Texture3D](class_texture3d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **NoiseTexture3D**

> Uses the FastNoiseLite library or other noise generators to fill the texture data of your desired size.

<details><summary>官方英文描述</summary>

Uses the FastNoiseLite library or other noise generators to fill the texture data of your desired size.

The class uses Threads to generate the texture data internally, so Texture3D.get_data() may return null if the generation process has not completed yet. In that case, you need to wait for the texture to be generated before accessing the image:

```gdscript
var texture = NoiseTexture3D.new()
texture.noise = FastNoiseLite.new()
await texture.changed
var data = texture.get_data()
```

</details>

## 方法

### get_color_ramp {#get_color_ramp}

```gdscript
Gradient get_color_ramp() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_invert {#get_invert}

```gdscript
bool get_invert() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_noise {#get_noise}

```gdscript
Noise get_noise()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_seamless {#get_seamless}

```gdscript
bool get_seamless()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_seamless_blend_skirt {#get_seamless_blend_skirt}

```gdscript
float get_seamless_blend_skirt()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_normalized {#is_normalized}

```gdscript
bool is_normalized() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color_ramp {#set_color_ramp}

```gdscript
void set_color_ramp(gradient: Gradient)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_depth {#set_depth}

```gdscript
void set_depth(depth: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_invert {#set_invert}

```gdscript
void set_invert(invert: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_noise {#set_noise}

```gdscript
void set_noise(noise: Noise)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_normalize {#set_normalize}

```gdscript
void set_normalize(normalize: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_seamless {#set_seamless}

```gdscript
void set_seamless(seamless: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_seamless_blend_skirt {#set_seamless_blend_skirt}

```gdscript
void set_seamless_blend_skirt(seamless_blend_skirt: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_width {#set_width}

```gdscript
void set_width(width: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### width {#width}

```gdscript
width: int
```

- void set_width(value: int)
- int get_width()

Width of the generated texture (in pixels).

### height {#height}

```gdscript
height: int
```

- void set_height(value: int)
- int get_height()

Height of the generated texture (in pixels).

### depth {#depth}

```gdscript
depth: int
```

- void set_depth(value: int)
- int get_depth()

Depth of the generated texture (in pixels).

### noise {#noise}

```gdscript
noise: Noise
```

- void set_noise(value: Noise)
- Noise get_noise()

The instance of the Noise object.

### color_ramp {#color_ramp}

```gdscript
color_ramp: Gradient
```

- void set_color_ramp(value: Gradient)
- Gradient get_color_ramp()

A Gradient which is used to map the luminance of each pixel to a color value.

### seamless {#seamless}

```gdscript
seamless: bool
```

- void set_seamless(value: bool)
- bool get_seamless()

If true, a seamless texture is requested from the Noise resource.

Note: Seamless noise textures may take longer to generate and/or can have a lower contrast compared to non-seamless noise depending on the used Noise resource. This is because some implementations use higher dimensions for generating seamless noise.

Note: The default FastNoiseLite implementation uses the fallback path for seamless generation. If using a width, height or depth lower than the default, you may need to increase seamless_blend_skirt to make seamless blending more effective.

### invert {#invert}

```gdscript
invert: bool
```

- void set_invert(value: bool)
- bool get_invert()

If true, inverts the noise texture. White becomes black, black becomes white.

### normalize {#normalize}

```gdscript
normalize: bool
```

- void set_normalize(value: bool)
- bool is_normalized()

If true, the noise image coming from the noise generator is normalized to the range 0.0 to 1.0.

Turning normalization off can affect the contrast and allows you to generate non repeating tileable noise textures.

### seamless_blend_skirt {#seamless_blend_skirt}

```gdscript
seamless_blend_skirt: float
```

- void set_seamless_blend_skirt(value: float)
- float get_seamless_blend_skirt()

Used for the default/fallback implementation of the seamless texture generation. It determines the distance over which the seams are blended. High values may result in less details and contrast. See Noise for further details.

Note: If using a width, height or depth lower than the default, you may need to increase seamless_blend_skirt to make seamless blending more effective.
