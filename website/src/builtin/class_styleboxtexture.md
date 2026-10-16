# StyleBoxTexture

**继承**：[StyleBox](class_stylebox.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StyleBoxTexture**

> A texture-based nine-patch StyleBox, in a way similar to NinePatchRect. This stylebox performs a 3×3 scaling of a texture, where only the center cell is fully stretched. This makes it possible to design bordered styles regardless of the stylebox's size.

<details><summary>官方英文描述</summary>

A texture-based nine-patch StyleBox, in a way similar to NinePatchRect. This stylebox performs a 3×3 scaling of a texture, where only the center cell is fully stretched. This makes it possible to design bordered styles regardless of the stylebox's size.

</details>

## 方法

### get_expand_margin {#get_expand_margin}

```gdscript
float get_expand_margin(margin: Side) const
```

Returns the expand margin size of the specified Side.

### get_h_axis_stretch_mode {#get_h_axis_stretch_mode}

```gdscript
StyleBoxTexture.AxisStretchMode get_h_axis_stretch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_modulate {#get_modulate}

```gdscript
Color get_modulate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_region_rect {#get_region_rect}

```gdscript
Rect2 get_region_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_margin {#get_texture_margin}

```gdscript
float get_texture_margin(margin: Side) const
```

Returns the margin size of the specified Side.

### get_v_axis_stretch_mode {#get_v_axis_stretch_mode}

```gdscript
StyleBoxTexture.AxisStretchMode get_v_axis_stretch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draw_center_enabled {#is_draw_center_enabled}

```gdscript
bool is_draw_center_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_center {#set_draw_center}

```gdscript
void set_draw_center(enable: bool)
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

### set_h_axis_stretch_mode {#set_h_axis_stretch_mode}

```gdscript
void set_h_axis_stretch_mode(mode: StyleBoxTexture.AxisStretchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_modulate {#set_modulate}

```gdscript
void set_modulate(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_region_rect {#set_region_rect}

```gdscript
void set_region_rect(region: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_margin {#set_texture_margin}

```gdscript
void set_texture_margin(margin: Side, size: float)
```

Sets the margin to size pixels for the specified Side.

### set_texture_margin_all {#set_texture_margin_all}

```gdscript
void set_texture_margin_all(size: float)
```

Sets the margin to size pixels for all sides.

### set_v_axis_stretch_mode {#set_v_axis_stretch_mode}

```gdscript
void set_v_axis_stretch_mode(mode: StyleBoxTexture.AxisStretchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The texture to use when drawing this style box.

### texture_margin_left {#texture_margin_left}

```gdscript
texture_margin_left: float
```

- void set_texture_margin(margin: Side, size: float)
- float get_texture_margin(margin: Side) const

Increases the left margin of the 3×3 texture box.

A higher value means more of the source texture is considered to be part of the left border of the 3×3 box.

This is also the value used as fallback for StyleBox.content_margin_left if it is negative.

### texture_margin_top {#texture_margin_top}

```gdscript
texture_margin_top: float
```

- void set_texture_margin(margin: Side, size: float)
- float get_texture_margin(margin: Side) const

Increases the top margin of the 3×3 texture box.

A higher value means more of the source texture is considered to be part of the top border of the 3×3 box.

This is also the value used as fallback for StyleBox.content_margin_top if it is negative.

### texture_margin_right {#texture_margin_right}

```gdscript
texture_margin_right: float
```

- void set_texture_margin(margin: Side, size: float)
- float get_texture_margin(margin: Side) const

Increases the right margin of the 3×3 texture box.

A higher value means more of the source texture is considered to be part of the right border of the 3×3 box.

This is also the value used as fallback for StyleBox.content_margin_right if it is negative.

### texture_margin_bottom {#texture_margin_bottom}

```gdscript
texture_margin_bottom: float
```

- void set_texture_margin(margin: Side, size: float)
- float get_texture_margin(margin: Side) const

Increases the bottom margin of the 3×3 texture box.

A higher value means more of the source texture is considered to be part of the bottom border of the 3×3 box.

This is also the value used as fallback for StyleBox.content_margin_bottom if it is negative.

### expand_margin_left {#expand_margin_left}

```gdscript
expand_margin_left: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the left margin of this style box when drawing, causing it to be drawn larger than requested.

### expand_margin_top {#expand_margin_top}

```gdscript
expand_margin_top: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the top margin of this style box when drawing, causing it to be drawn larger than requested.

### expand_margin_right {#expand_margin_right}

```gdscript
expand_margin_right: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the right margin of this style box when drawing, causing it to be drawn larger than requested.

### expand_margin_bottom {#expand_margin_bottom}

```gdscript
expand_margin_bottom: float
```

- void set_expand_margin(margin: Side, size: float)
- float get_expand_margin(margin: Side) const

Expands the bottom margin of this style box when drawing, causing it to be drawn larger than requested.

### axis_stretch_horizontal {#axis_stretch_horizontal}

```gdscript
axis_stretch_horizontal: int
```

- void set_h_axis_stretch_mode(value: AxisStretchMode)
- AxisStretchMode get_h_axis_stretch_mode()

Controls how the stylebox's texture will be stretched or tiled horizontally.

### axis_stretch_vertical {#axis_stretch_vertical}

```gdscript
axis_stretch_vertical: int
```

- void set_v_axis_stretch_mode(value: AxisStretchMode)
- AxisStretchMode get_v_axis_stretch_mode()

Controls how the stylebox's texture will be stretched or tiled vertically.

### region_rect {#region_rect}

```gdscript
region_rect: Rect2
```

- void set_region_rect(value: Rect2)
- Rect2 get_region_rect()

The region to use from the texture.

This is equivalent to first wrapping the texture in an AtlasTexture with the same region.

If empty (Rect2(0, 0, 0, 0)), the whole texture is used.

### modulate_color {#modulate_color}

```gdscript
modulate_color: Color
```

- void set_modulate(value: Color)
- Color get_modulate()

Modulates the color of the texture when this style box is drawn.

### draw_center {#draw_center}

```gdscript
draw_center: bool
```

- void set_draw_center(value: bool)
- bool is_draw_center_enabled()

If true, the nine-patch texture's center tile will be drawn.

## 枚举

### AxisStretchMode {#enum-AxisStretchMode}

```gdscript
enum AxisStretchMode { AXIS_STRETCH_MODE_STRETCH = 0, AXIS_STRETCH_MODE_TILE = 1, AXIS_STRETCH_MODE_TILE_FIT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_STRETCH_MODE_STRETCH {#AXIS_STRETCH_MODE_STRETCH}

```gdscript
AXIS_STRETCH_MODE_STRETCH = 0
```

Stretch the stylebox's texture. This results in visible distortion unless the texture size matches the stylebox's size perfectly.

#### AXIS_STRETCH_MODE_TILE {#AXIS_STRETCH_MODE_TILE}

```gdscript
AXIS_STRETCH_MODE_TILE = 1
```

Repeats the stylebox's texture to match the stylebox's size according to the nine-patch system.

#### AXIS_STRETCH_MODE_TILE_FIT {#AXIS_STRETCH_MODE_TILE_FIT}

```gdscript
AXIS_STRETCH_MODE_TILE_FIT = 2
```

Repeats the stylebox's texture to match the stylebox's size according to the nine-patch system. Unlike AXIS_STRETCH_MODE_TILE, the texture may be slightly stretched to make the nine-patch texture tile seamlessly.

---
