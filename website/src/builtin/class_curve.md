# Curve

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Curve**

> This resource describes a mathematical curve by defining a set of points and tangents at each point. By default, it ranges between 0 and 1 on the X and Y axes, but these ranges can be changed.

<details><summary>官方英文描述（中文精译进行中）</summary>

This resource describes a mathematical curve by defining a set of points and tangents at each point. By default, it ranges between 0 and 1 on the X and Y axes, but these ranges can be changed.

Please note that many resources and nodes assume they are given unit curves. A unit curve is a curve whose domain (the X axis) is between 0 and 1. Some examples of unit curve usage are CPUParticles2D.angle_curve and Line2D.width_curve.

</details>

## 方法

### add_point {#add_point}

```gdscript
int add_point(position: Vector2, left_tangent: float = 0, right_tangent: float = 0, left_mode: Curve.TangentMode = 0, right_mode: Curve.TangentMode = 0)
```

Adds a point to the curve. For each side, if the *_mode is TANGENT_LINEAR, the *_tangent angle (in degrees) uses the slope of the curve halfway to the adjacent point. Allows custom assignments to the *_tangent angle if *_mode is set to TANGENT_FREE.

### bake {#bake}

```gdscript
void bake()
```

Recomputes the baked cache of points for the curve.

### clean_dupes {#clean_dupes}

```gdscript
void clean_dupes()
```

Removes duplicate points, i.e. points that are less than 0.00001 units (engine epsilon value) away from their neighbor on the curve.

### clear_points {#clear_points}

```gdscript
void clear_points()
```

Removes all points from the curve.

### get_bake_resolution {#get_bake_resolution}

```gdscript
int get_bake_resolution() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_domain_range {#get_domain_range}

```gdscript
float get_domain_range() const
```

Returns the difference between min_domain and max_domain.

### get_max_domain {#get_max_domain}

```gdscript
float get_max_domain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_value {#get_max_value}

```gdscript
float get_max_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_domain {#get_min_domain}

```gdscript
float get_min_domain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_value {#get_min_value}

```gdscript
float get_min_value() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_point_count {#get_point_count}

```gdscript
int get_point_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_point_left_mode {#get_point_left_mode}

```gdscript
Curve.TangentMode get_point_left_mode(index: int) const
```

Returns the left TangentMode for the point at index.

### get_point_left_tangent {#get_point_left_tangent}

```gdscript
float get_point_left_tangent(index: int) const
```

Returns the left tangent angle (in degrees) for the point at index.

### get_point_position {#get_point_position}

```gdscript
Vector2 get_point_position(index: int) const
```

Returns the curve coordinates for the point at index.

### get_point_right_mode {#get_point_right_mode}

```gdscript
Curve.TangentMode get_point_right_mode(index: int) const
```

Returns the right TangentMode for the point at index.

### get_point_right_tangent {#get_point_right_tangent}

```gdscript
float get_point_right_tangent(index: int) const
```

Returns the right tangent angle (in degrees) for the point at index.

### get_value_range {#get_value_range}

```gdscript
float get_value_range() const
```

Returns the difference between min_value and max_value.

### remove_point {#remove_point}

```gdscript
void remove_point(index: int)
```

Removes the point at index from the curve.

### sample {#sample}

```gdscript
float sample(offset: float) const
```

Returns the Y value for the point that would exist at the X position offset along the curve.

### sample_baked {#sample_baked}

```gdscript
float sample_baked(offset: float) const
```

Returns the Y value for the point that would exist at the X position offset along the curve using the baked cache. Bakes the curve's points if not already baked.

### set_bake_resolution {#set_bake_resolution}

```gdscript
void set_bake_resolution(resolution: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_domain {#set_max_domain}

```gdscript
void set_max_domain(max: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_value {#set_max_value}

```gdscript
void set_max_value(max: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_domain {#set_min_domain}

```gdscript
void set_min_domain(min: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_value {#set_min_value}

```gdscript
void set_min_value(min: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_point_count {#set_point_count}

```gdscript
void set_point_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_point_left_mode {#set_point_left_mode}

```gdscript
void set_point_left_mode(index: int, mode: Curve.TangentMode)
```

Sets the left TangentMode for the point at index to mode.

### set_point_left_tangent {#set_point_left_tangent}

```gdscript
void set_point_left_tangent(index: int, tangent: float)
```

Sets the left tangent angle for the point at index to tangent.

### set_point_offset {#set_point_offset}

```gdscript
int set_point_offset(index: int, offset: float)
```

Assigns the horizontal position offset to the point at index.

### set_point_right_mode {#set_point_right_mode}

```gdscript
void set_point_right_mode(index: int, mode: Curve.TangentMode)
```

Sets the right TangentMode for the point at index to mode.

### set_point_right_tangent {#set_point_right_tangent}

```gdscript
void set_point_right_tangent(index: int, tangent: float)
```

Sets the right tangent angle for the point at index to tangent.

### set_point_value {#set_point_value}

```gdscript
void set_point_value(index: int, y: float)
```

Assigns the vertical position y to the point at index.

## 属性

### min_domain {#min_domain}

```gdscript
min_domain: float
```

- void set_min_domain(value: float)
- float get_min_domain()

The minimum domain (x-coordinate) that points can have.

### max_domain {#max_domain}

```gdscript
max_domain: float
```

- void set_max_domain(value: float)
- float get_max_domain()

The maximum domain (x-coordinate) that points can have.

### min_value {#min_value}

```gdscript
min_value: float
```

- void set_min_value(value: float)
- float get_min_value()

The minimum value (y-coordinate) that points can have. Tangents can cause lower values between points.

### max_value {#max_value}

```gdscript
max_value: float
```

- void set_max_value(value: float)
- float get_max_value()

The maximum value (y-coordinate) that points can have. Tangents can cause higher values between points.

### bake_resolution {#bake_resolution}

```gdscript
bake_resolution: int
```

- void set_bake_resolution(value: int)
- int get_bake_resolution()

The number of points to include in the baked (i.e. cached) curve data.

### point_count {#point_count}

```gdscript
point_count: int
```

- void set_point_count(value: int)
- int get_point_count()

The number of points describing the curve.

## 信号

### range_changed {#range_changed}

```gdscript
signal range_changed()
```

Emitted when max_value or min_value is changed.

### domain_changed {#domain_changed}

```gdscript
signal domain_changed()
```

Emitted when max_domain or min_domain is changed.

## 枚举

### TangentMode {#enum-TangentMode}

```gdscript
enum TangentMode { TANGENT_FREE = 0, TANGENT_LINEAR = 1, TANGENT_MODE_COUNT = 2 }
```

TangentMode TANGENT_FREE = 0

The tangent on this side of the point is user-defined.

TangentMode TANGENT_LINEAR = 1

The curve calculates the tangent on this side of the point as the slope halfway towards the adjacent point.

TangentMode TANGENT_MODE_COUNT = 2

The total number of available tangent modes.

#### TANGENT_FREE {#TANGENT_FREE}

```gdscript
TANGENT_FREE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TANGENT_LINEAR {#TANGENT_LINEAR}

```gdscript
TANGENT_LINEAR = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TANGENT_MODE_COUNT {#TANGENT_MODE_COUNT}

```gdscript
TANGENT_MODE_COUNT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
