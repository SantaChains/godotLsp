# StyleBoxFlat

**继承**：[StyleBox](class_stylebox.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StyleBoxFlat**

> By configuring various properties of this style box, you can achieve many common looks without the need of a texture. This includes optionally rounded borders, antialiasing, shadows, and skew.

<details><summary>官方英文描述</summary>

By configuring various properties of this style box, you can achieve many common looks without the need of a texture. This includes optionally rounded borders, antialiasing, shadows, and skew.

Setting corner radius to high values is allowed. As soon as corners overlap, the stylebox will switch to a relative system:

```
height = 30
corner_radius_top_left = 50
corner_radius_bottom_left = 100
```

The relative system now would take the 1:2 ratio of the two left corners to calculate the actual corner width. Both corners added will never be more than the height. Result:

```
corner_radius_top_left: 10
corner_radius_bottom_left: 20
```

</details>

## 方法

### get_aa_size {#get_aa_size}

```gdscript
float get_aa_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bg_color {#get_bg_color}

```gdscript
Color get_bg_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_border_blend {#get_border_blend}

```gdscript
bool get_border_blend() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_border_color {#get_border_color}

```gdscript
Color get_border_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_border_width {#get_border_width}

```gdscript
int get_border_width(margin: Side) const
```

Returns the specified Side's border width.

### get_border_width_min {#get_border_width_min}

```gdscript
int get_border_width_min() const
```

Returns the smallest border width out of all four borders.

### get_corner_detail {#get_corner_detail}

```gdscript
int get_corner_detail() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_corner_radius {#get_corner_radius}

```gdscript
int get_corner_radius(corner: Corner) const
```

Returns the given corner's radius.

### get_expand_margin {#get_expand_margin}

```gdscript
float get_expand_margin(margin: Side) const
```

Returns the size of the specified Side's expand margin.

### get_shadow_color {#get_shadow_color}

```gdscript
Color get_shadow_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadow_offset {#get_shadow_offset}

```gdscript
Vector2 get_shadow_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadow_size {#get_shadow_size}

```gdscript
int get_shadow_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skew {#get_skew}

```gdscript
Vector2 get_skew() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_anti_aliased {#is_anti_aliased}

```gdscript
bool is_anti_aliased() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draw_center_enabled {#is_draw_center_enabled}

```gdscript
bool is_draw_center_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_aa_size {#set_aa_size}

```gdscript
void set_aa_size(size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_anti_aliased {#set_anti_aliased}

```gdscript
void set_anti_aliased(anti_aliased: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bg_color {#set_bg_color}

```gdscript
void set_bg_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_border_blend {#set_border_blend}

```gdscript
void set_border_blend(blend: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_border_color {#set_border_color}

```gdscript
void set_border_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_border_width {#set_border_width}

```gdscript
void set_border_width(margin: Side, width: int)
```

Sets the specified Side's border width to width pixels.

### set_border_width_all {#set_border_width_all}

```gdscript
void set_border_width_all(width: int)
```

Sets the border width to width pixels for all sides.

### set_corner_detail {#set_corner_detail}

```gdscript
void set_corner_detail(detail: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_corner_radius {#set_corner_radius}

```gdscript
void set_corner_radius(corner: Corner, radius: int)
```

Sets the corner radius to radius pixels for the given corner.

### set_corner_radius_all {#set_corner_radius_all}

```gdscript
void set_corner_radius_all(radius: int)
```

Sets the corner radius to radius pixels for all corners.

### set_draw_center {#set_draw_center}

```gdscript
void set_draw_center(draw_center: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_expand_margin {#set_expand_margin}

```gdscript
void set_expand_margin(margin: Side, size: float)
```

Sets the expand margin to size pixels for the specified Side.

### set_expand_margin_all {#set_expand_margin_all}

```gdscript
void set_expand_margin_all(size: float)
```

Sets the expand margin to size pixels for all sides.

### set_shadow_color {#set_shadow_color}

```gdscript
void set_shadow_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_offset {#set_shadow_offset}

```gdscript
void set_shadow_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_size {#set_shadow_size}

```gdscript
void set_shadow_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skew {#set_skew}

```gdscript
void set_skew(skew: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### bg_color {#bg_color}

```gdscript
bg_color: Color
```

- void set_bg_color(value: Color)
- Color get_bg_color()

The background color of the stylebox.

### draw_center {#draw_center}

```gdscript
draw_center: bool
```

- void set_draw_center(value: bool)
- bool is_draw_center_enabled()

Toggles drawing of the inner part of the stylebox.

### skew {#skew}

```gdscript
skew: Vector2
```

- void set_skew(value: Vector2)
- Vector2 get_skew()

If set to a non-zero value on either axis, skew distorts the StyleBox horizontally and/or vertically. This can be used for "futuristic"-style UIs. Positive values skew the StyleBox towards the right (X axis) and upwards (Y axis), while negative values skew the StyleBox towards the left (X axis) and downwards (Y axis).

Note: To ensure text does not touch the StyleBox's edges, consider increasing the StyleBox's content margin (see StyleBox.content_margin_bottom). It is preferable to increase the content margin instead of the expand margin (see expand_margin_bottom), as increasing the expand margin does not increase the size of the clickable area for Controls.

### border_width_left {#border_width_left}

```gdscript
border_width_left: int
```

- void set_border_width(margin: Side, width: int)
- int get_border_width(margin: Side) const

Border width for the left border.

### border_width_top {#border_width_top}

```gdscript
border_width_top: int
```

- void set_border_width(margin: Side, width: int)
- int get_border_width(margin: Side) const

Border width for the top border.

### border_width_right {#border_width_right}

```gdscript
border_width_right: int
```

- void set_border_width(margin: Side, width: int)
- int get_border_width(margin: Side) const

Border width for the right border.

### border_width_bottom {#border_width_bottom}

```gdscript
border_width_bottom: int
```

- void set_border_width(margin: Side, width: int)
- int get_border_width(margin: Side) const

Border width for the bottom border.

### border_color {#border_color}

```gdscript
border_color: Color
```

- void set_border_color(value: Color)
- Color get_border_color()

Sets the color of the border.

### border_blend {#border_blend}

```gdscript
border_blend: bool
```

- void set_border_blend(value: bool)
- bool get_border_blend()

If true, the border will fade into the background color.

### corner_radius_top_left {#corner_radius_top_left}

```gdscript
corner_radius_top_left: int
```

- void set_corner_radius(corner: Corner, radius: int)
- int get_corner_radius(corner: Corner) const

The top-left corner's radius. If 0, the corner is not rounded.

### corner_radius_top_right {#corner_radius_top_right}

```gdscript
corner_radius_top_right: int
```

- void set_corner_radius(corner: Corner, radius: int)
- int get_corner_radius(corner: Corner) const

The top-right corner's radius. If 0, the corner is not rounded.

### corner_radius_bottom_right {#corner_radius_bottom_right}

```gdscript
corner_radius_bottom_right: int
```

- void set_corner_radius(corner: Corner, radius: int)
- int get_corner_radius(corner: Corner) const

The bottom-right corner's radius. If 0, the corner is not rounded.

### corner_radius_bottom_left {#corner_radius_bottom_left}

```gdscript
corner_radius_bottom_left: int
```

- void set_corner_radius(corner: Corner, radius: int)
- int get_corner_radius(corner: Corner) const

The bottom-left corner's radius. If 0, the corner is not rounded.

### corner_detail {#corner_detail}

```gdscript
corner_detail: int
```

- void set_corner_detail(value: int)
- int get_corner_detail()

This sets the number of vertices used for each corner. Higher values result in rounder corners but take more processing power to compute. When choosing a value, you should take the corner radius (set_corner_radius_all()) into account.

For corner radii less than 10, 4 or 5 should be enough. For corner radii less than 30, values between 8 and 12 should be enough.

A corner detail of 1 will result in chamfered corners instead of rounded corners, which is useful for some artistic effects.

### expand_margin_left {#expand_margin_left}

```gdscript
expand_margin_left: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the stylebox outside of the control rect on the left edge. Useful in combination with border_width_left to draw a border outside the control rect.

Note: Unlike StyleBox.content_margin_left, expand_margin_left does not affect the size of the clickable area for Controls. This can negatively impact usability if used wrong, as the user may try to click an area of the StyleBox that cannot actually receive clicks.

### expand_margin_top {#expand_margin_top}

```gdscript
expand_margin_top: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the stylebox outside of the control rect on the top edge. Useful in combination with border_width_top to draw a border outside the control rect.

Note: Unlike StyleBox.content_margin_top, expand_margin_top does not affect the size of the clickable area for Controls. This can negatively impact usability if used wrong, as the user may try to click an area of the StyleBox that cannot actually receive clicks.

### expand_margin_right {#expand_margin_right}

```gdscript
expand_margin_right: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the stylebox outside of the control rect on the right edge. Useful in combination with border_width_right to draw a border outside the control rect.

Note: Unlike StyleBox.content_margin_right, expand_margin_right does not affect the size of the clickable area for Controls. This can negatively impact usability if used wrong, as the user may try to click an area of the StyleBox that cannot actually receive clicks.

### expand_margin_bottom {#expand_margin_bottom}

```gdscript
expand_margin_bottom: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the stylebox outside of the control rect on the bottom edge. Useful in combination with border_width_bottom to draw a border outside the control rect.

Note: Unlike StyleBox.content_margin_bottom, expand_margin_bottom does not affect the size of the clickable area for Controls. This can negatively impact usability if used wrong, as the user may try to click an area of the StyleBox that cannot actually receive clicks.

### shadow_color {#shadow_color}

```gdscript
shadow_color: Color
```

- void set_shadow_color(value: Color)
- Color get_shadow_color()

The color of the shadow. This has no effect if shadow_size is lower than 1.

### shadow_size {#shadow_size}

```gdscript
shadow_size: int
```

- void set_shadow_size(value: int)
- int get_shadow_size()

The shadow size in pixels.

### shadow_offset {#shadow_offset}

```gdscript
shadow_offset: Vector2
```

- void set_shadow_offset(value: Vector2)
- Vector2 get_shadow_offset()

The shadow offset in pixels. Adjusts the position of the shadow relatively to the stylebox.

### anti_aliasing {#anti_aliasing}

```gdscript
anti_aliasing: bool
```

- void set_anti_aliased(value: bool)
- bool is_anti_aliased()

Antialiasing draws a small ring around the edges, which fades to transparency. As a result, edges look much smoother. This is only noticeable when using rounded corners or skew.

Note: When using beveled corners with 45-degree angles (corner_detail = 1), it is recommended to set anti_aliasing to false to ensure crisp visuals and avoid possible visual glitches.

### anti_aliasing_size {#anti_aliasing_size}

```gdscript
anti_aliasing_size: float
```

- void set_aa_size(value: float)
- float get_aa_size()

This changes the size of the antialiasing effect. 1.0 is recommended for an optimal result at 100% scale, identical to how rounded rectangles are rendered in web browsers and most vector drawing software.

Note: Higher values may produce a blur effect but can also create undesired artifacts on small boxes with large-radius corners.
