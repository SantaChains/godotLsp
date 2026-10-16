# Line2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Line2D**

> This node draws a 2D polyline, i.e. a shape consisting of several points connected by segments. Line2D is not a mathematical polyline, i.e. the segments are not infinitely thin. It is intended for rendering and it can be colored and optionally textured.

<details><summary>官方英文描述</summary>

This node draws a 2D polyline, i.e. a shape consisting of several points connected by segments. Line2D is not a mathematical polyline, i.e. the segments are not infinitely thin. It is intended for rendering and it can be colored and optionally textured.

Warning: Certain configurations may be impossible to draw nicely, such as very sharp angles. In these situations, the node uses fallback drawing logic to look decent.

Note: Line2D is drawn using a 2D mesh.

</details>

## 方法

### add_point {#add_point}

```gdscript
void add_point(position: Vector2, index: int = -1)
```

Adds a point with the specified position relative to the polyline's own position. If no index is provided, the new point will be added to the end of the points array.

If index is given, the new point is inserted before the existing point identified by index index. The indices of the points after the new point get increased by 1. The provided index must not exceed the number of existing points in the polyline. See get_point_count().

### clear_points {#clear_points}

```gdscript
void clear_points()
```

Removes all points from the polyline, making it empty.

### get_antialiased {#get_antialiased}

```gdscript
bool get_antialiased() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_begin_cap_mode {#get_begin_cap_mode}

```gdscript
Line2D.LineCapMode get_begin_cap_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_curve {#get_curve}

```gdscript
Curve get_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_color {#get_default_color}

```gdscript
Color get_default_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_end_cap_mode {#get_end_cap_mode}

```gdscript
Line2D.LineCapMode get_end_cap_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gradient {#get_gradient}

```gdscript
Gradient get_gradient() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint_mode {#get_joint_mode}

```gdscript
Line2D.LineJointMode get_joint_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_point_count {#get_point_count}

```gdscript
int get_point_count() const
```

Returns the number of points in the polyline.

### get_point_position {#get_point_position}

```gdscript
Vector2 get_point_position(index: int) const
```

Returns the position of the point at index index.

### get_points {#get_points}

```gdscript
PackedVector2Array get_points() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_round_precision {#get_round_precision}

```gdscript
int get_round_precision() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sharp_limit {#get_sharp_limit}

```gdscript
float get_sharp_limit() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_mode {#get_texture_mode}

```gdscript
Line2D.LineTextureMode get_texture_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_width {#get_width}

```gdscript
float get_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_closed {#is_closed}

```gdscript
bool is_closed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_point {#remove_point}

```gdscript
void remove_point(index: int)
```

Removes the point at index index from the polyline.

### set_antialiased {#set_antialiased}

```gdscript
void set_antialiased(antialiased: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_begin_cap_mode {#set_begin_cap_mode}

```gdscript
void set_begin_cap_mode(mode: Line2D.LineCapMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_closed {#set_closed}

```gdscript
void set_closed(closed: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_curve {#set_curve}

```gdscript
void set_curve(curve: Curve)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_color {#set_default_color}

```gdscript
void set_default_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_end_cap_mode {#set_end_cap_mode}

```gdscript
void set_end_cap_mode(mode: Line2D.LineCapMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gradient {#set_gradient}

```gdscript
void set_gradient(color: Gradient)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_joint_mode {#set_joint_mode}

```gdscript
void set_joint_mode(mode: Line2D.LineJointMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_point_position {#set_point_position}

```gdscript
void set_point_position(index: int, position: Vector2)
```

Overwrites the position of the point at the given index with the supplied position.

### set_points {#set_points}

```gdscript
void set_points(points: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_round_precision {#set_round_precision}

```gdscript
void set_round_precision(precision: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sharp_limit {#set_sharp_limit}

```gdscript
void set_sharp_limit(limit: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_mode {#set_texture_mode}

```gdscript
void set_texture_mode(mode: Line2D.LineTextureMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_width {#set_width}

```gdscript
void set_width(width: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### points {#points}

```gdscript
points: PackedVector2Array
```

- void set_points(value: PackedVector2Array)
- PackedVector2Array get_points()

The points of the polyline, interpreted in local 2D coordinates. Segments are drawn between the adjacent points in this array.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector2Array for more details.

### closed {#closed}

```gdscript
closed: bool
```

- void set_closed(value: bool)
- bool is_closed()

If true and the polyline has more than 2 points, the last point and the first one will be connected by a segment.

Note: The shape of the closing segment is not guaranteed to be seamless if a width_curve is provided.

Note: The joint between the closing segment and the first segment is drawn first and it samples the gradient and the width_curve at the beginning. This is an implementation detail that might change in a future version.

### width {#width}

```gdscript
width: float
```

- void set_width(value: float)
- float get_width()

The polyline's width.

### width_curve {#width_curve}

```gdscript
width_curve: Curve
```

- void set_curve(value: Curve)
- Curve get_curve()

The polyline's width curve. The width of the polyline over its length will be equivalent to the value of the width curve over its domain. The width curve should be a unit Curve.

### default_color {#default_color}

```gdscript
default_color: Color
```

- void set_default_color(value: Color)
- Color get_default_color()

The color of the polyline. Will not be used if a gradient is set.

### gradient {#gradient}

```gdscript
gradient: Gradient
```

- void set_gradient(value: Gradient)
- Gradient get_gradient()

The gradient is drawn through the whole line from start to finish. The default_color will not be used if this property is set.

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The texture used for the polyline. Uses texture_mode for drawing style.

### texture_mode {#texture_mode}

```gdscript
texture_mode: int
```

- void set_texture_mode(value: LineTextureMode)
- LineTextureMode get_texture_mode()

The style to render the texture of the polyline.

### joint_mode {#joint_mode}

```gdscript
joint_mode: int
```

- void set_joint_mode(value: LineJointMode)
- LineJointMode get_joint_mode()

The style of the connections between segments of the polyline.

### begin_cap_mode {#begin_cap_mode}

```gdscript
begin_cap_mode: int
```

- void set_begin_cap_mode(value: LineCapMode)
- LineCapMode get_begin_cap_mode()

The style of the beginning of the polyline, if closed is false.

### end_cap_mode {#end_cap_mode}

```gdscript
end_cap_mode: int
```

- void set_end_cap_mode(value: LineCapMode)
- LineCapMode get_end_cap_mode()

The style of the end of the polyline, if closed is false.

### sharp_limit {#sharp_limit}

```gdscript
sharp_limit: float
```

- void set_sharp_limit(value: float)
- float get_sharp_limit()

Determines the miter limit of the polyline. Normally, when joint_mode is set to LINE_JOINT_SHARP, sharp angles fall back to using the logic of LINE_JOINT_BEVEL joints to prevent very long miters. Higher values of this property mean that the fallback to a bevel joint will happen at sharper angles.

### round_precision {#round_precision}

```gdscript
round_precision: int
```

- void set_round_precision(value: int)
- int get_round_precision()

The smoothness used for rounded joints and caps. Higher values result in smoother corners, but are more demanding to render and update.

### antialiased {#antialiased}

```gdscript
antialiased: bool
```

- void set_antialiased(value: bool)
- bool get_antialiased()

If true, the polyline's border will be anti-aliased.

Note: Line2D is not accelerated by batching when being anti-aliased.

## 枚举

### LineJointMode {#enum-LineJointMode}

```gdscript
enum LineJointMode { LINE_JOINT_SHARP = 0, LINE_JOINT_BEVEL = 1, LINE_JOINT_ROUND = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LINE_JOINT_SHARP {#LINE_JOINT_SHARP}

```gdscript
LINE_JOINT_SHARP = 0
```

Makes the polyline's joints pointy, connecting the sides of the two segments by extending them until they intersect. If the rotation of a joint is too big (based on sharp_limit), the joint falls back to LINE_JOINT_BEVEL to prevent very long miters.

#### LINE_JOINT_BEVEL {#LINE_JOINT_BEVEL}

```gdscript
LINE_JOINT_BEVEL = 1
```

Makes the polyline's joints bevelled/chamfered, connecting the sides of the two segments with a simple line.

#### LINE_JOINT_ROUND {#LINE_JOINT_ROUND}

```gdscript
LINE_JOINT_ROUND = 2
```

Makes the polyline's joints rounded, connecting the sides of the two segments with an arc. The detail of this arc depends on round_precision.

---

### LineCapMode {#enum-LineCapMode}

```gdscript
enum LineCapMode { LINE_CAP_NONE = 0, LINE_CAP_BOX = 1, LINE_CAP_ROUND = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LINE_CAP_NONE {#LINE_CAP_NONE}

```gdscript
LINE_CAP_NONE = 0
```

Draws no line cap.

#### LINE_CAP_BOX {#LINE_CAP_BOX}

```gdscript
LINE_CAP_BOX = 1
```

Draws the line cap as a box, slightly extending the first/last segment.

#### LINE_CAP_ROUND {#LINE_CAP_ROUND}

```gdscript
LINE_CAP_ROUND = 2
```

Draws the line cap as a semicircle attached to the first/last segment.

---

### LineTextureMode {#enum-LineTextureMode}

```gdscript
enum LineTextureMode { LINE_TEXTURE_NONE = 0, LINE_TEXTURE_TILE = 1, LINE_TEXTURE_STRETCH = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LINE_TEXTURE_NONE {#LINE_TEXTURE_NONE}

```gdscript
LINE_TEXTURE_NONE = 0
```

Takes the left pixels of the texture and renders them over the whole polyline.

#### LINE_TEXTURE_TILE {#LINE_TEXTURE_TILE}

```gdscript
LINE_TEXTURE_TILE = 1
```

Tiles the texture over the polyline. CanvasItem.texture_repeat of the Line2D node must be CanvasItem.TEXTURE_REPEAT_ENABLED or CanvasItem.TEXTURE_REPEAT_MIRROR for it to work properly.

#### LINE_TEXTURE_STRETCH {#LINE_TEXTURE_STRETCH}

```gdscript
LINE_TEXTURE_STRETCH = 2
```

Stretches the texture across the polyline. CanvasItem.texture_repeat of the Line2D node must be CanvasItem.TEXTURE_REPEAT_DISABLED for best results.

---
