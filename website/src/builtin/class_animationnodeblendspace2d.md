# AnimationNodeBlendSpace2D

**继承**：[AnimationRootNode](class_animationrootnode.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeBlendSpace2D**

> A resource used by AnimationNodeBlendTree.

<details><summary>官方英文描述</summary>

A resource used by AnimationNodeBlendTree.

AnimationNodeBlendSpace2D represents a virtual 2D space on which AnimationRootNodes are placed. Outputs the linear blend of the three adjacent animations using a Vector2 weight. Adjacent in this context means the three AnimationRootNodes making up the triangle that contains the current value.

You can add vertices to the blend space with add_blend_point() and automatically triangulate it by setting auto_triangles to true. Otherwise, use add_triangle() and remove_triangle() to triangulate the blend space by hand.

</details>

## 方法

### add_blend_point {#add_blend_point}

```gdscript
void add_blend_point(node: AnimationRootNode, pos: Vector2, at_index: int = -1, name: StringName = &"")
```

Adds a new point with name that represents a node at the position set by pos. You can insert it at a specific index using the at_index argument. If you use the default value for at_index, the point is inserted at the end of the blend points array.

Note: If no name is provided, safe index is used as reference. In the future, empty names will be deprecated, so explicitly passing a name is recommended.

### add_triangle {#add_triangle}

```gdscript
void add_triangle(x: int, y: int, z: int, at_index: int = -1)
```

Creates a new triangle using three points x, y, and z. Triangles can overlap. You can insert the triangle at a specific index using the at_index argument. If you use the default value for at_index, the point is inserted at the end of the blend points array.

### find_blend_point_by_name {#find_blend_point_by_name}

```gdscript
int find_blend_point_by_name(name: StringName) const
```

Returns the index of the blend point with the given name. Returns -1 if no blend point with that name is found.

### get_auto_triangles {#get_auto_triangles}

```gdscript
bool get_auto_triangles() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_blend_mode {#get_blend_mode}

```gdscript
AnimationNodeBlendSpace2D.BlendMode get_blend_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_blend_point_count {#get_blend_point_count}

```gdscript
int get_blend_point_count() const
```

Returns the number of points in the blend space.

### get_blend_point_name {#get_blend_point_name}

```gdscript
StringName get_blend_point_name(point: int) const
```

Returns the name of the blend point at index point.

### get_blend_point_node {#get_blend_point_node}

```gdscript
AnimationRootNode get_blend_point_node(point: int) const
```

Returns the AnimationRootNode referenced by the point at index point.

### get_blend_point_position {#get_blend_point_position}

```gdscript
Vector2 get_blend_point_position(point: int) const
```

Returns the position of the point at index point.

### get_cyclic_length {#get_cyclic_length}

```gdscript
float get_cyclic_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_space {#get_max_space}

```gdscript
Vector2 get_max_space() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_space {#get_min_space}

```gdscript
Vector2 get_min_space() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_snap {#get_snap}

```gdscript
Vector2 get_snap() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sync_mode {#get_sync_mode}

```gdscript
AnimationNodeBlendSpace2D.SyncMode get_sync_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_triangle_count {#get_triangle_count}

```gdscript
int get_triangle_count() const
```

Returns the number of triangles in the blend space.

### get_triangle_point {#get_triangle_point}

```gdscript
int get_triangle_point(triangle: int, point: int)
```

Returns the position of the point at index point in the triangle of index triangle.

### get_x_label {#get_x_label}

```gdscript
String get_x_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_y_label {#get_y_label}

```gdscript
String get_y_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_sync {#is_using_sync}

```gdscript
bool is_using_sync() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_blend_point {#remove_blend_point}

```gdscript
void remove_blend_point(point: int)
```

Removes the point at index point from the blend space.

### remove_triangle {#remove_triangle}

```gdscript
void remove_triangle(triangle: int)
```

Removes the triangle at index triangle from the blend space.

### reorder_blend_point {#reorder_blend_point}

```gdscript
void reorder_blend_point(from_index: int, to_index: int)
```

Swaps the blend points at indices from_index and to_index, exchanging their positions and properties.

### set_auto_triangles {#set_auto_triangles}

```gdscript
void set_auto_triangles(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_mode {#set_blend_mode}

```gdscript
void set_blend_mode(mode: AnimationNodeBlendSpace2D.BlendMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_point_name {#set_blend_point_name}

```gdscript
void set_blend_point_name(point: int, name: StringName)
```

Sets the name of the blend point at index point. If the name conflicts with an existing point, a unique name will be generated automatically.

### set_blend_point_node {#set_blend_point_node}

```gdscript
void set_blend_point_node(point: int, node: AnimationRootNode)
```

Changes the AnimationNode referenced by the point at index point.

### set_blend_point_position {#set_blend_point_position}

```gdscript
void set_blend_point_position(point: int, pos: Vector2)
```

Updates the position of the point at index point in the blend space.

### set_cyclic_length {#set_cyclic_length}

```gdscript
void set_cyclic_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_space {#set_max_space}

```gdscript
void set_max_space(max_space: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_space {#set_min_space}

```gdscript
void set_min_space(min_space: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_snap {#set_snap}

```gdscript
void set_snap(snap: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sync_mode {#set_sync_mode}

```gdscript
void set_sync_mode(sync_mode: AnimationNodeBlendSpace2D.SyncMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_sync {#set_use_sync}

```gdscript
void set_use_sync(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_x_label {#set_x_label}

```gdscript
void set_x_label(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_y_label {#set_y_label}

```gdscript
void set_y_label(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### auto_triangles {#auto_triangles}

```gdscript
auto_triangles: bool
```

- void set_auto_triangles(value: bool)
- bool get_auto_triangles()

If true, the blend space is triangulated automatically. The mesh updates every time you add or remove points with add_blend_point() and remove_blend_point().

### triangles {#triangles}

```gdscript
triangles: PackedInt32Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### min_space {#min_space}

```gdscript
min_space: Vector2
```

- void set_min_space(value: Vector2)
- Vector2 get_min_space()

The blend space's X and Y axes' lower limit for the points' position. See add_blend_point().

### max_space {#max_space}

```gdscript
max_space: Vector2
```

- void set_max_space(value: Vector2)
- Vector2 get_max_space()

The blend space's X and Y axes' upper limit for the points' position. See add_blend_point().

### snap {#snap}

```gdscript
snap: Vector2
```

- void set_snap(value: Vector2)
- Vector2 get_snap()

Position increment to snap to when moving a point.

### x_label {#x_label}

```gdscript
x_label: String
```

- void set_x_label(value: String)
- String get_x_label()

Name of the blend space's X axis.

### y_label {#y_label}

```gdscript
y_label: String
```

- void set_y_label(value: String)
- String get_y_label()

Name of the blend space's Y axis.

### blend_mode {#blend_mode}

```gdscript
blend_mode: int
```

- void set_blend_mode(value: BlendMode)
- BlendMode get_blend_mode()

Controls the interpolation between animations.

### sync {#sync}

```gdscript
sync: bool
```

- void set_use_sync(value: bool)
- bool is_using_sync()

Deprecated: Use sync_mode instead.

If true, sync mode is enabled (equivalent to SYNC_MODE_INDEPENDENT). This property is kept for backward compatibility.

### sync_mode {#sync_mode}

```gdscript
sync_mode: int
```

- void set_sync_mode(value: SyncMode)
- SyncMode get_sync_mode()

Controls how animations are synced when blended. See SyncMode for available options.

### cyclic_length {#cyclic_length}

```gdscript
cyclic_length: float
```

- void set_cyclic_length(value: float)
- float get_cyclic_length()

The cycle length in seconds used by SYNC_MODE_CYCLIC_CONSTANT. All animations are time-scaled so they complete one full cycle in this duration. Must be greater than 0 for cyclic sync to take effect.

## 信号

### triangles_updated {#triangles_updated}

```gdscript
signal triangles_updated()
```

Emitted every time the blend space's triangles are created, removed, or when one of their vertices changes position.

## 枚举

### BlendMode {#enum-BlendMode}

```gdscript
enum BlendMode { BLEND_MODE_INTERPOLATED = 0, BLEND_MODE_DISCRETE = 1, BLEND_MODE_DISCRETE_CARRY = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BLEND_MODE_INTERPOLATED {#BLEND_MODE_INTERPOLATED}

```gdscript
BLEND_MODE_INTERPOLATED = 0
```

The interpolation between animations is linear.

#### BLEND_MODE_DISCRETE {#BLEND_MODE_DISCRETE}

```gdscript
BLEND_MODE_DISCRETE = 1
```

The blend space plays the animation of the animation node which blending position is closest to. Useful for frame-by-frame 2D animations.

#### BLEND_MODE_DISCRETE_CARRY {#BLEND_MODE_DISCRETE_CARRY}

```gdscript
BLEND_MODE_DISCRETE_CARRY = 2
```

Similar to BLEND_MODE_DISCRETE, but starts the new animation at the last animation's playback position.

---

### SyncMode {#enum-SyncMode}

```gdscript
enum SyncMode { SYNC_MODE_NONE = 0, SYNC_MODE_INDEPENDENT = 1, SYNC_MODE_CYCLIC_MUTABLE = 2, SYNC_MODE_CYCLIC_CONSTANT = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SYNC_MODE_NONE {#SYNC_MODE_NONE}

```gdscript
SYNC_MODE_NONE = 0
```

Inactive animations are frozen and do not advance.

#### SYNC_MODE_INDEPENDENT {#SYNC_MODE_INDEPENDENT}

```gdscript
SYNC_MODE_INDEPENDENT = 1
```

Inactive animations advance with a weight of 0. This is equivalent to the previous sync = true behavior.

#### SYNC_MODE_CYCLIC_MUTABLE {#SYNC_MODE_CYCLIC_MUTABLE}

```gdscript
SYNC_MODE_CYCLIC_MUTABLE = 2
```

All animations are time-scaled so they stay in sync, with the cycle length dynamically computed from active blend weights. This is self-normalizing: a solo animation plays at normal speed.

Note: If you apply AnimationNodeTimeSeek to the result when handling animations of different lengths, synchronization will be broken. In such cases, it is recommended to use AnimationNodeAnimation.use_custom_timeline to align the animation lengths.

#### SYNC_MODE_CYCLIC_CONSTANT {#SYNC_MODE_CYCLIC_CONSTANT}

```gdscript
SYNC_MODE_CYCLIC_CONSTANT = 3
```

All animations are time-scaled so they complete one cycle in cyclic_length seconds, keeping them in sync regardless of their individual lengths.

Note: If you apply AnimationNodeTimeSeek to the result when handling animations of different lengths, synchronization will be broken. In such cases, it is recommended to use AnimationNodeAnimation.use_custom_timeline to align the animation lengths.

---
