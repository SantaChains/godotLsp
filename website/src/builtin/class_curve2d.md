# Curve2D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Curve2D**

> This class describes a Bézier curve in 2D space. It is mainly used to give a shape to a Path2D, but can be manually sampled for other purposes.

<details><summary>官方英文描述</summary>

This class describes a Bézier curve in 2D space. It is mainly used to give a shape to a Path2D, but can be manually sampled for other purposes.

It keeps a cache of precalculated points along the curve, to speed up further calculations.

</details>

## 方法

### add_point {#add_point}

```gdscript
void add_point(position: Vector2, in: Vector2 = Vector2(0, 0), out: Vector2 = Vector2(0, 0), index: int = -1)
```

Adds a point with the specified position relative to the curve's own position, with control points in and out. Appends the new point at the end of the point list.

If index is given, the new point is inserted before the existing point identified by index index. Every existing point starting from index is shifted further down the list of points. The index must be greater than or equal to 0 and must not exceed the number of existing points in the line. See point_count.

### clear_points {#clear_points}

```gdscript
void clear_points()
```

Removes all points from the curve.

### get_bake_interval {#get_bake_interval}

```gdscript
float get_bake_interval() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_baked_length {#get_baked_length}

```gdscript
float get_baked_length() const
```

Returns the total length of the curve, based on the cached points. Given enough density (see bake_interval), it should be approximate enough.

### get_baked_points {#get_baked_points}

```gdscript
PackedVector2Array get_baked_points() const
```

Returns the cache of points as a PackedVector2Array.

### get_closest_offset {#get_closest_offset}

```gdscript
float get_closest_offset(to_point: Vector2) const
```

Returns the closest offset to to_point. This offset is meant to be used in sample_baked().

to_point must be in this curve's local space.

### get_closest_point {#get_closest_point}

```gdscript
Vector2 get_closest_point(to_point: Vector2) const
```

Returns the closest point on baked segments (in curve's local space) to to_point.

to_point must be in this curve's local space.

### get_point_count {#get_point_count}

```gdscript
int get_point_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_point_in {#get_point_in}

```gdscript
Vector2 get_point_in(idx: int) const
```

Returns the position of the control point leading to the vertex idx. The returned position is relative to the vertex idx. If the index is out of bounds, the function sends an error to the console, and returns (0, 0).

### get_point_out {#get_point_out}

```gdscript
Vector2 get_point_out(idx: int) const
```

Returns the position of the control point leading out of the vertex idx. The returned position is relative to the vertex idx. If the index is out of bounds, the function sends an error to the console, and returns (0, 0).

### get_point_position {#get_point_position}

```gdscript
Vector2 get_point_position(idx: int) const
```

Returns the position of the vertex idx. If the index is out of bounds, the function sends an error to the console, and returns (0, 0).

### remove_point {#remove_point}

```gdscript
void remove_point(idx: int)
```

Deletes the point idx from the curve. Sends an error to the console if idx is out of bounds.

### sample {#sample}

```gdscript
Vector2 sample(idx: int, t: float) const
```

Returns the position between the vertex idx and the vertex idx + 1, where t controls if the point is the first vertex (t = 0.0), the last vertex (t = 1.0), or in between. Values of t outside the range (0.0 <= t <= 1.0) give strange, but predictable results.

If idx is out of bounds it is truncated to the first or last vertex, and t is ignored. If the curve has no points, the function sends an error to the console, and returns (0, 0).

### sample_baked {#sample_baked}

```gdscript
Vector2 sample_baked(offset: float = 0.0, cubic: bool = false) const
```

Returns a point within the curve at position offset, where offset is measured as a pixel distance along the curve.

To do that, it finds the two cached points where the offset lies between, then interpolates the values. This interpolation is cubic if cubic is set to true, or linear if set to false.

Cubic interpolation tends to follow the curves better, but linear is faster (and often, precise enough).

### sample_baked_with_rotation {#sample_baked_with_rotation}

```gdscript
Transform2D sample_baked_with_rotation(offset: float = 0.0, cubic: bool = false) const
```

Similar to sample_baked(), but returns Transform2D that includes a rotation along the curve, with Transform2D.origin as the point position and the Transform2D.x vector pointing in the direction of the path at that point. Returns an empty transform if the length of the curve is 0.

```gdscript
var baked = curve.sample_baked_with_rotation(offset)
# The returned Transform2D can be set directly.
transform = baked
# You can also read the origin and rotation separately from the returned Transform2D.
position = baked.get_origin()
rotation = baked.get_rotation()
```

### samplef {#samplef}

```gdscript
Vector2 samplef(fofs: float) const
```

Returns the position at the vertex fofs. It calls sample() using the integer part of fofs as idx, and its fractional part as t.

### set_bake_interval {#set_bake_interval}

```gdscript
void set_bake_interval(distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_point_count {#set_point_count}

```gdscript
void set_point_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_point_in {#set_point_in}

```gdscript
void set_point_in(idx: int, position: Vector2)
```

Sets the position of the control point leading to the vertex idx. If the index is out of bounds, the function sends an error to the console. The position is relative to the vertex.

### set_point_out {#set_point_out}

```gdscript
void set_point_out(idx: int, position: Vector2)
```

Sets the position of the control point leading out of the vertex idx. If the index is out of bounds, the function sends an error to the console. The position is relative to the vertex.

### set_point_position {#set_point_position}

```gdscript
void set_point_position(idx: int, position: Vector2)
```

Sets the position for the vertex idx. If the index is out of bounds, the function sends an error to the console.

### tessellate {#tessellate}

```gdscript
PackedVector2Array tessellate(max_stages: int = 5, tolerance_degrees: float = 4) const
```

Returns a list of points along the curve, with a curvature controlled point density. That is, the curvier parts will have more points than the straighter parts.

This approximation makes straight segments between each point, then subdivides those segments until the resulting shape is similar enough.

max_stages controls how many subdivisions a curve segment may face before it is considered approximate enough. Each subdivision splits the segment in half, so the default 5 stages may mean up to 32 subdivisions per curve segment. Increase with care!

tolerance_degrees controls how many degrees the midpoint of a segment may deviate from the real curve, before the segment has to be subdivided.

### tessellate_even_length {#tessellate_even_length}

```gdscript
PackedVector2Array tessellate_even_length(max_stages: int = 5, tolerance_length: float = 20.0) const
```

Returns a list of points along the curve, with almost uniform density. max_stages controls how many subdivisions a curve segment may face before it is considered approximate enough. Each subdivision splits the segment in half, so the default 5 stages may mean up to 32 subdivisions per curve segment. Increase with care!

tolerance_length controls the maximal distance between two neighboring points, before the segment has to be subdivided.

## 属性

### bake_interval {#bake_interval}

```gdscript
bake_interval: float
```

- void set_bake_interval(value: float)
- float get_bake_interval()

The distance in pixels between two adjacent cached points. Changing it forces the cache to be recomputed the next time the get_baked_points() or get_baked_length() function is called. The smaller the distance, the more points in the cache and the more memory it will consume, so use with care.

### point_count {#point_count}

```gdscript
point_count: int
```

- void set_point_count(value: int)
- int get_point_count()

The number of points describing the curve.
