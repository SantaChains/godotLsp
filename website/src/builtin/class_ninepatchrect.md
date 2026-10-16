# NinePatchRect

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **NinePatchRect**

> Also known as 9-slice panels, NinePatchRect produces clean panels of any size based on a small texture. To do so, it splits the texture in a 3×3 grid. When you scale the node, it tiles the texture's edges horizontally or vertically, tiles the center on both axes, and leaves the corners unchanged.

<details><summary>官方英文描述（中文精译进行中）</summary>

Also known as 9-slice panels, NinePatchRect produces clean panels of any size based on a small texture. To do so, it splits the texture in a 3×3 grid. When you scale the node, it tiles the texture's edges horizontally or vertically, tiles the center on both axes, and leaves the corners unchanged.

</details>

## 方法

### get_h_axis_stretch_mode {#get_h_axis_stretch_mode}

```gdscript
NinePatchRect.AxisStretchMode get_h_axis_stretch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_patch_margin {#get_patch_margin}

```gdscript
int get_patch_margin(margin: Side) const
```

Returns the size of the margin on the specified Side.

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

### get_v_axis_stretch_mode {#get_v_axis_stretch_mode}

```gdscript
NinePatchRect.AxisStretchMode get_v_axis_stretch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draw_center_enabled {#is_draw_center_enabled}

```gdscript
bool is_draw_center_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_center {#set_draw_center}

```gdscript
void set_draw_center(draw_center: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_h_axis_stretch_mode {#set_h_axis_stretch_mode}

```gdscript
void set_h_axis_stretch_mode(mode: NinePatchRect.AxisStretchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_patch_margin {#set_patch_margin}

```gdscript
void set_patch_margin(margin: Side, value: int)
```

Sets the size of the margin on the specified Side to value pixels.

### set_region_rect {#set_region_rect}

```gdscript
void set_region_rect(rect: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_axis_stretch_mode {#set_v_axis_stretch_mode}

```gdscript
void set_v_axis_stretch_mode(mode: NinePatchRect.AxisStretchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The node's texture resource.

### draw_center {#draw_center}

```gdscript
draw_center: bool
```

- void set_draw_center(value: bool)
- bool is_draw_center_enabled()

If true, draw the panel's center. Else, only draw the 9-slice's borders.

### region_rect {#region_rect}

```gdscript
region_rect: Rect2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### patch_margin_left {#patch_margin_left}

```gdscript
patch_margin_left: int
```

- void set_patch_margin(margin: Side, value: int)
- int get_patch_margin(margin: Side) const

The width of the 9-slice's left column. A margin of 16 means the 9-slice's left corners and side will have a width of 16 pixels. You can set all 4 margin values individually to create panels with non-uniform borders.

### patch_margin_top {#patch_margin_top}

```gdscript
patch_margin_top: int
```

- void set_patch_margin(margin: Side, value: int)
- int get_patch_margin(margin: Side) const

The height of the 9-slice's top row. A margin of 16 means the 9-slice's top corners and side will have a height of 16 pixels. You can set all 4 margin values individually to create panels with non-uniform borders.

### patch_margin_right {#patch_margin_right}

```gdscript
patch_margin_right: int
```

- void set_patch_margin(margin: Side, value: int)
- int get_patch_margin(margin: Side) const

The width of the 9-slice's right column. A margin of 16 means the 9-slice's right corners and side will have a width of 16 pixels. You can set all 4 margin values individually to create panels with non-uniform borders.

### patch_margin_bottom {#patch_margin_bottom}

```gdscript
patch_margin_bottom: int
```

- void set_patch_margin(margin: Side, value: int)
- int get_patch_margin(margin: Side) const

The height of the 9-slice's bottom row. A margin of 16 means the 9-slice's bottom corners and side will have a height of 16 pixels. You can set all 4 margin values individually to create panels with non-uniform borders.

### axis_stretch_horizontal {#axis_stretch_horizontal}

```gdscript
axis_stretch_horizontal: int
```

- void set_h_axis_stretch_mode(value: AxisStretchMode)
- AxisStretchMode get_h_axis_stretch_mode()

The stretch mode to use for horizontal stretching/tiling.

### axis_stretch_vertical {#axis_stretch_vertical}

```gdscript
axis_stretch_vertical: int
```

- void set_v_axis_stretch_mode(value: AxisStretchMode)
- AxisStretchMode get_v_axis_stretch_mode()

The stretch mode to use for vertical stretching/tiling.

## 信号

### texture_changed {#texture_changed}

```gdscript
signal texture_changed()
```

Emitted when the node's texture changes.

## 枚举

### AxisStretchMode {#enum-AxisStretchMode}

```gdscript
enum AxisStretchMode { AXIS_STRETCH_MODE_STRETCH = 0, AXIS_STRETCH_MODE_TILE = 1, AXIS_STRETCH_MODE_TILE_FIT = 2 }
```

AxisStretchMode AXIS_STRETCH_MODE_STRETCH = 0

Stretches the center texture across the NinePatchRect. This may cause the texture to be distorted.

AxisStretchMode AXIS_STRETCH_MODE_TILE = 1

Repeats the center texture across the NinePatchRect. This won't cause any visible distortion. The texture must be seamless for this to work without displaying artifacts between edges.

AxisStretchMode AXIS_STRETCH_MODE_TILE_FIT = 2

Repeats the center texture across the NinePatchRect, but will also stretch the texture to make sure each tile is visible in full. This may cause the texture to be distorted, but less than AXIS_STRETCH_MODE_STRETCH. The texture must be seamless for this to work without displaying artifacts between edges.

#### AXIS_STRETCH_MODE_STRETCH {#AXIS_STRETCH_MODE_STRETCH}

```gdscript
AXIS_STRETCH_MODE_STRETCH = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_STRETCH_MODE_TILE {#AXIS_STRETCH_MODE_TILE}

```gdscript
AXIS_STRETCH_MODE_TILE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_STRETCH_MODE_TILE_FIT {#AXIS_STRETCH_MODE_TILE_FIT}

```gdscript
AXIS_STRETCH_MODE_TILE_FIT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
