# Gradient

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Gradient**

> This resource describes a color transition by defining a set of colored points and how to interpolate between them.

<details><summary>官方英文描述（中文精译进行中）</summary>

This resource describes a color transition by defining a set of colored points and how to interpolate between them.

See also Curve which supports more complex easing methods, but does not support colors.

</details>

## 方法

### add_point {#add_point}

```gdscript
void add_point(offset: float, color: Color)
```

Adds the specified color to the gradient, with the specified offset.

### get_color {#get_color}

```gdscript
Color get_color(point: int)
```

Returns the color of the gradient color at index point.

### get_colors {#get_colors}

```gdscript
PackedColorArray get_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_interpolation_color_space {#get_interpolation_color_space}

```gdscript
Gradient.ColorSpace get_interpolation_color_space()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_interpolation_mode {#get_interpolation_mode}

```gdscript
Gradient.InterpolationMode get_interpolation_mode()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset {#get_offset}

```gdscript
float get_offset(point: int)
```

Returns the offset of the gradient color at index point.

### get_offsets {#get_offsets}

```gdscript
PackedFloat32Array get_offsets() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_point_count {#get_point_count}

```gdscript
int get_point_count() const
```

Returns the number of colors in the gradient.

### remove_point {#remove_point}

```gdscript
void remove_point(point: int)
```

Removes the color at index point.

### reverse {#reverse}

```gdscript
void reverse()
```

Reverses/mirrors the gradient.

Note: This method mirrors all points around the middle of the gradient, which may produce unexpected results when interpolation_mode is set to GRADIENT_INTERPOLATE_CONSTANT.

### sample {#sample}

```gdscript
Color sample(offset: float)
```

Returns the interpolated color specified by offset. offset should be between 0.0 and 1.0 (inclusive). Using a value lower than 0.0 will return the same color as 0.0, and using a value higher than 1.0 will return the same color as 1.0. If your input value is not within this range, consider using @GlobalScope.remap() on the input value with output values set to 0.0 and 1.0.

### set_color {#set_color}

```gdscript
void set_color(point: int, color: Color)
```

Sets the color of the gradient color at index point.

### set_colors {#set_colors}

```gdscript
void set_colors(colors: PackedColorArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interpolation_color_space {#set_interpolation_color_space}

```gdscript
void set_interpolation_color_space(interpolation_color_space: Gradient.ColorSpace)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_interpolation_mode {#set_interpolation_mode}

```gdscript
void set_interpolation_mode(interpolation_mode: Gradient.InterpolationMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(point: int, offset: float)
```

Sets the offset for the gradient color at index point.

### set_offsets {#set_offsets}

```gdscript
void set_offsets(offsets: PackedFloat32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### interpolation_mode {#interpolation_mode}

```gdscript
interpolation_mode: int
```

- void set_interpolation_mode(value: InterpolationMode)
- InterpolationMode get_interpolation_mode()

The algorithm used to interpolate between points of the gradient.

### interpolation_color_space {#interpolation_color_space}

```gdscript
interpolation_color_space: int
```

- void set_interpolation_color_space(value: ColorSpace)
- ColorSpace get_interpolation_color_space()

The color space used to interpolate between points of the gradient. It does not affect the returned colors, which will always use nonlinear sRGB encoding.

Note: This setting has no effect when interpolation_mode is set to GRADIENT_INTERPOLATE_CONSTANT.

### offsets {#offsets}

```gdscript
offsets: PackedFloat32Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### colors {#colors}

```gdscript
colors: PackedColorArray
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### InterpolationMode {#enum-InterpolationMode}

```gdscript
enum InterpolationMode { GRADIENT_INTERPOLATE_LINEAR = 0, GRADIENT_INTERPOLATE_CONSTANT = 1, GRADIENT_INTERPOLATE_CUBIC = 2 }
```

InterpolationMode GRADIENT_INTERPOLATE_LINEAR = 0

Linear interpolation.

InterpolationMode GRADIENT_INTERPOLATE_CONSTANT = 1

Constant interpolation, color changes abruptly at each point and stays uniform between. This might cause visible aliasing when used for a gradient texture in some cases.

InterpolationMode GRADIENT_INTERPOLATE_CUBIC = 2

Cubic interpolation.

#### GRADIENT_INTERPOLATE_LINEAR {#GRADIENT_INTERPOLATE_LINEAR}

```gdscript
GRADIENT_INTERPOLATE_LINEAR = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GRADIENT_INTERPOLATE_CONSTANT {#GRADIENT_INTERPOLATE_CONSTANT}

```gdscript
GRADIENT_INTERPOLATE_CONSTANT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GRADIENT_INTERPOLATE_CUBIC {#GRADIENT_INTERPOLATE_CUBIC}

```gdscript
GRADIENT_INTERPOLATE_CUBIC = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ColorSpace {#enum-ColorSpace}

```gdscript
enum ColorSpace { GRADIENT_COLOR_SPACE_SRGB = 0, GRADIENT_COLOR_SPACE_LINEAR_SRGB = 1, GRADIENT_COLOR_SPACE_OKLAB = 2 }
```

ColorSpace GRADIENT_COLOR_SPACE_SRGB = 0

sRGB color space.

ColorSpace GRADIENT_COLOR_SPACE_LINEAR_SRGB = 1

Linear sRGB color space.

ColorSpace GRADIENT_COLOR_SPACE_OKLAB = 2

Oklab [https://bottosson.github.io/posts/oklab/] color space. This color space provides a smooth and uniform-looking transition between colors.

#### GRADIENT_COLOR_SPACE_SRGB {#GRADIENT_COLOR_SPACE_SRGB}

```gdscript
GRADIENT_COLOR_SPACE_SRGB = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GRADIENT_COLOR_SPACE_LINEAR_SRGB {#GRADIENT_COLOR_SPACE_LINEAR_SRGB}

```gdscript
GRADIENT_COLOR_SPACE_LINEAR_SRGB = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GRADIENT_COLOR_SPACE_OKLAB {#GRADIENT_COLOR_SPACE_OKLAB}

```gdscript
GRADIENT_COLOR_SPACE_OKLAB = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
