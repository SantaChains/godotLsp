# GradientTexture2D

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GradientTexture2D**

> A 2D texture that obtains colors from a Gradient to fill the texture data. This texture is able to transform a color transition into different patterns such as a linear or a radial gradient. The texture is filled by interpolating colors starting from fill_from to fill_to offsets by default, but the gradient fill can be repeated to cover the entire texture.

<details><summary>官方英文描述</summary>

A 2D texture that obtains colors from a Gradient to fill the texture data. This texture is able to transform a color transition into different patterns such as a linear or a radial gradient. The texture is filled by interpolating colors starting from fill_from to fill_to offsets by default, but the gradient fill can be repeated to cover the entire texture.

The gradient is sampled individually for each pixel so it does not necessarily represent an exact copy of the gradient (see width and height). See also GradientTexture1D, CurveTexture and CurveXYZTexture.

</details>

## 方法

### get_fill {#get_fill}

```gdscript
GradientTexture2D.Fill get_fill() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fill_from {#get_fill_from}

```gdscript
Vector2 get_fill_from() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fill_to {#get_fill_to}

```gdscript
Vector2 get_fill_to() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gradient {#get_gradient}

```gdscript
Gradient get_gradient() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_repeat {#get_repeat}

```gdscript
GradientTexture2D.Repeat get_repeat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_hdr {#is_using_hdr}

```gdscript
bool is_using_hdr() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fill {#set_fill}

```gdscript
void set_fill(fill: GradientTexture2D.Fill)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fill_from {#set_fill_from}

```gdscript
void set_fill_from(fill_from: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fill_to {#set_fill_to}

```gdscript
void set_fill_to(fill_to: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gradient {#set_gradient}

```gdscript
void set_gradient(gradient: Gradient)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_repeat {#set_repeat}

```gdscript
void set_repeat(repeat: GradientTexture2D.Repeat)
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

The number of horizontal color samples that will be obtained from the Gradient, which also represents the texture's width.

### height {#height}

```gdscript
height: int
```

- void set_height(value: int)
- int get_height()

The number of vertical color samples that will be obtained from the Gradient, which also represents the texture's height.

### use_hdr {#use_hdr}

```gdscript
use_hdr: bool
```

- void set_use_hdr(value: bool)
- bool is_using_hdr()

If true, the generated texture will support high dynamic range (Image.FORMAT_RGBAF format). This allows for glow effects to work if Environment.glow_enabled is true. If false, the generated texture will use low dynamic range; overbright colors will be clamped (Image.FORMAT_RGBA8 format).

### fill {#fill}

```gdscript
fill: int
```

- void set_fill(value: Fill)
- Fill get_fill()

The gradient's fill type.

### fill_from {#fill_from}

```gdscript
fill_from: Vector2
```

- void set_fill_from(value: Vector2)
- Vector2 get_fill_from()

The initial offset used to fill the texture specified in UV coordinates.

### fill_to {#fill_to}

```gdscript
fill_to: Vector2
```

- void set_fill_to(value: Vector2)
- Vector2 get_fill_to()

The final offset used to fill the texture specified in UV coordinates.

### repeat {#repeat}

```gdscript
repeat: int
```

- void set_repeat(value: Repeat)
- Repeat get_repeat()

The gradient's repeat type.

## 枚举

### Fill {#enum-Fill}

```gdscript
enum Fill { FILL_LINEAR = 0, FILL_RADIAL = 1, FILL_SQUARE = 2, FILL_CONIC = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILL_LINEAR {#FILL_LINEAR}

```gdscript
FILL_LINEAR = 0
```

The colors are linearly interpolated in a straight line.

#### FILL_RADIAL {#FILL_RADIAL}

```gdscript
FILL_RADIAL = 1
```

The colors are linearly interpolated in a circular pattern.

#### FILL_SQUARE {#FILL_SQUARE}

```gdscript
FILL_SQUARE = 2
```

The colors are linearly interpolated in a square pattern.

#### FILL_CONIC {#FILL_CONIC}

```gdscript
FILL_CONIC = 3
```

The colors are linearly interpolated in a cone pattern.

---

### Repeat {#enum-Repeat}

```gdscript
enum Repeat { REPEAT_NONE = 0, REPEAT = 1, REPEAT_MIRROR = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### REPEAT_NONE {#REPEAT_NONE}

```gdscript
REPEAT_NONE = 0
```

The gradient fill is restricted to the range defined by fill_from to fill_to offsets.

#### REPEAT {#REPEAT}

```gdscript
REPEAT = 1
```

The texture is filled starting from fill_from to fill_to offsets, repeating the same pattern in both directions.

#### REPEAT_MIRROR {#REPEAT_MIRROR}

```gdscript
REPEAT_MIRROR = 2
```

The texture is filled starting from fill_from to fill_to offsets, mirroring the pattern in both directions.

---
