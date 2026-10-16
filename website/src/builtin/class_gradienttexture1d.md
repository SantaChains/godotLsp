# GradientTexture1D

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GradientTexture1D**

> A 1D texture that obtains colors from a Gradient to fill the texture data. The texture is filled by sampling the gradient for each pixel. Therefore, the texture does not necessarily represent an exact copy of the gradient, as it may miss some colors if there are not enough pixels. See also GradientTexture2D, CurveTexture and CurveXYZTexture.

<details><summary>官方英文描述</summary>

A 1D texture that obtains colors from a Gradient to fill the texture data. The texture is filled by sampling the gradient for each pixel. Therefore, the texture does not necessarily represent an exact copy of the gradient, as it may miss some colors if there are not enough pixels. See also GradientTexture2D, CurveTexture and CurveXYZTexture.

</details>

## 方法

### get_gradient {#get_gradient}

```gdscript
Gradient get_gradient() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_hdr {#is_using_hdr}

```gdscript
bool is_using_hdr() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gradient {#set_gradient}

```gdscript
void set_gradient(gradient: Gradient)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_hdr {#set_use_hdr}

```gdscript
void set_use_hdr(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_width {#set_width}

```gdscript
void set_width(width: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### gradient {#gradient}

```gdscript
gradient: Gradient
```

- void set_gradient(value: Gradient)
- Gradient get_gradient()

The Gradient used to fill the texture.

### width {#width}

```gdscript
width: int
```

- void set_width(value: int)
- int get_width()

The number of color samples that will be obtained from the Gradient.

### use_hdr {#use_hdr}

```gdscript
use_hdr: bool
```

- void set_use_hdr(value: bool)
- bool is_using_hdr()

If true, the generated texture will support high dynamic range (Image.FORMAT_RGBAF format). This allows for glow effects to work if Environment.glow_enabled is true. If false, the generated texture will use low dynamic range; overbright colors will be clamped (Image.FORMAT_RGBA8 format).
