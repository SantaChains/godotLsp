# CurveTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CurveTexture**

> A 1D texture where pixel brightness corresponds to points on a unit Curve resource, either in grayscale or in red. This visual representation simplifies the task of saving curves as image files.

<details><summary>官方英文描述（中文精译进行中）</summary>

A 1D texture where pixel brightness corresponds to points on a unit Curve resource, either in grayscale or in red. This visual representation simplifies the task of saving curves as image files.

If you need to store up to 3 curves within a single texture, use CurveXYZTexture instead. See also GradientTexture1D and GradientTexture2D.

</details>

## 方法

### get_curve {#get_curve}

```gdscript
Curve get_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_mode {#get_texture_mode}

```gdscript
CurveTexture.TextureMode get_texture_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve {#set_curve}

```gdscript
void set_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_mode {#set_texture_mode}

```gdscript
void set_texture_mode(texture_mode: CurveTexture.TextureMode)
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

The width of the texture (in pixels). Higher values make it possible to represent high-frequency data better (such as sudden direction changes), at the cost of increased generation time and memory usage.

### texture_mode {#texture_mode}

```gdscript
texture_mode: int
```

- void set_texture_mode(value: TextureMode)
- TextureMode get_texture_mode()

The format the texture should be generated with. When passing a CurveTexture as an input to a Shader, this may need to be adjusted.

### curve {#curve}

```gdscript
curve: Curve
```

- void set_curve(value: Curve)
- Curve get_curve()

The Curve that is rendered onto the texture. Should be a unit Curve.

## 枚举

### TextureMode {#enum-TextureMode}

```gdscript
enum TextureMode { TEXTURE_MODE_RGB = 0, TEXTURE_MODE_RED = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TEXTURE_MODE_RGB {#TEXTURE_MODE_RGB}

```gdscript
TEXTURE_MODE_RGB = 0
```

Store the curve equally across the red, green and blue channels. This uses more video memory, but is more compatible with shaders that only read the green and blue values.

#### TEXTURE_MODE_RED {#TEXTURE_MODE_RED}

```gdscript
TEXTURE_MODE_RED = 1
```

Store the curve only in the red channel. This saves video memory, but some custom shaders may not be able to work with this.

---
