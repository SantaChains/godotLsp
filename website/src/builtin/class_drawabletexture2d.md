# DrawableTexture2D

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **DrawableTexture2D**

> A 2D texture that can be modified via blit calls, copying from a target texture to itself. Primarily intended to be managed in code, a user must call setup() to initialize the state before drawing. Each blit_rect() call takes at least a rectangle, the area to draw to, and another texture, what to be drawn. The draw calls use a Texture_Blit Shader to process and calculate the result, pixel by pixel. Users can supply their own ShaderMaterial with custom Texture_Blit shaders for more complex behaviors.

<details><summary>官方英文描述</summary>

A 2D texture that can be modified via blit calls, copying from a target texture to itself. Primarily intended to be managed in code, a user must call setup() to initialize the state before drawing. Each blit_rect() call takes at least a rectangle, the area to draw to, and another texture, what to be drawn. The draw calls use a Texture_Blit Shader to process and calculate the result, pixel by pixel. Users can supply their own ShaderMaterial with custom Texture_Blit shaders for more complex behaviors.

</details>

## 方法

### blit_rect {#blit_rect}

```gdscript
void blit_rect(rect: Rect2i, source: Texture2D, modulate: Color = Color(1, 1, 1, 1), mipmap: int = 0, material: Material = null)
```

Experimental: This method may be changed or removed in future versions.

Draws to given rect on this texture by copying from the given source. A modulate color can be passed in for the shader to use, but defaults to White. The mipmap value can specify a draw to a lower mipmap level. The material parameter can take a ShaderMaterial with a TextureBlit Shader for custom drawing behavior.

### blit_rect_multi {#blit_rect_multi}

```gdscript
void blit_rect_multi(rect: Rect2i, sources: typedarray::Texture2D, extra_targets: typedarray::DrawableTexture2D, modulate: Color = Color(1, 1, 1, 1), mipmap: int = 0, material: Material = null)
```

Experimental: This method may be changed or removed in future versions.

Draws to the given rect on this texture, as well as on up to 3 DrawableTexture extra_targets. All extra_targets must be the same size and DrawableFormat as the original target, otherwise the Shader may fail. Expects up to 4 Texture sources, but will replace missing sources with default Black Textures.

### generate_mipmaps {#generate_mipmaps}

```gdscript
void generate_mipmaps()
```

Re-calculates the mipmaps for this texture on demand.

### get_use_mipmaps {#get_use_mipmaps}

```gdscript
bool get_use_mipmaps() const
```

Returns true if mipmaps are set to be used on this DrawableTexture.

### set_format {#set_format}

```gdscript
void set_format(format: DrawableTexture2D.DrawableFormat)
```

Sets the format of this DrawableTexture.

### set_use_mipmaps {#set_use_mipmaps}

```gdscript
void set_use_mipmaps(mipmaps: bool)
```

Sets if mipmaps should be used on this DrawableTexture.

### setup {#setup}

```gdscript
void setup(width: int, height: int, format: DrawableTexture2D.DrawableFormat, color: Color = Color(1, 1, 1, 1), use_mipmaps: bool = false)
```

Experimental: This method may be changed or removed in future versions.

Initializes the DrawableTexture to a White texture of the given width, height, and format.

## 枚举

### DrawableFormat {#enum-DrawableFormat}

```gdscript
enum DrawableFormat { DRAWABLE_FORMAT_RGBA8 = 0, DRAWABLE_FORMAT_RGBA8_SRGB = 1, DRAWABLE_FORMAT_RGBAH = 2, DRAWABLE_FORMAT_RGBAF = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DRAWABLE_FORMAT_RGBA8 {#DRAWABLE_FORMAT_RGBA8}

```gdscript
DRAWABLE_FORMAT_RGBA8 = 0
```

OpenGL texture format RGBA with four components, each with a bitdepth of 8.

#### DRAWABLE_FORMAT_RGBA8_SRGB {#DRAWABLE_FORMAT_RGBA8_SRGB}

```gdscript
DRAWABLE_FORMAT_RGBA8_SRGB = 1
```

OpenGL texture format RGBA with four components, each with a bitdepth of 8.

When drawn to, an sRGB to linear color space conversion is performed.

#### DRAWABLE_FORMAT_RGBAH {#DRAWABLE_FORMAT_RGBAH}

```gdscript
DRAWABLE_FORMAT_RGBAH = 2
```

OpenGL texture format GL_RGBA16F where there are four components, each a 16-bit "half-precision" floating-point value.

#### DRAWABLE_FORMAT_RGBAF {#DRAWABLE_FORMAT_RGBAF}

```gdscript
DRAWABLE_FORMAT_RGBAF = 3
```

OpenGL texture format GL_RGBA32F where there are four components, each a 32-bit floating-point value.

---
