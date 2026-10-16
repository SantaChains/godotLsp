# AnimationNodeBlendSpace1D

**继承**：[AnimationRootNode](class_animationrootnode.md) → [AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeBlendSpace1D**

> A resource used by AnimationNodeBlendTree.

<details><summary>官方英文描述</summary>

A resource used by AnimationNodeBlendTree.

AnimationNodeBlendSpace1D represents a virtual axis on which any type of AnimationRootNodes can be added using add_blend_point(). Outputs the linear blend of the two AnimationRootNodes adjacent to the current value.

You can set the extents of the axis with min_space and max_space.

</details>

## 方法

### add_blend_point {#add_blend_point}

```gdscript
void add_blend_point(node: AnimationRootNode, pos: float, at_index: int = -1, name: StringName = &"")
```

Adds a new point with name that represents a node on the virtual axis at a given position set by pos. You can insert it at a specific index using the at_index argument. If you use the default value for at_index, the point is inserted at the end of the blend points array.

Note: If no name is provided, safe index is used as reference. In the future, empty names will be deprecated, so explicitly passing a name is recommended.

### find_blend_point_by_name {#find_blend_point_by_name}

```gdscript
int find_blend_point_by_name(name: StringName) const
```

Returns the index of the blend point with the given name. Returns -1 if no blend point with that name is found.

### get_blend_mode {#get_blend_mode}

```gdscript
AnimationNodeBlendSpace1D.BlendMode get_blend_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_blend_point_count {#get_blend_point_count}

```gdscript
int get_blend_point_count() const
```

Returns the number of points on the blend axis.

### get_blend_point_name {#get_blend_point_name}

```gdscript
StringName get_blend_point_name(point: int) const
```

Returns the name of the blend point at index point.

### get_blend_point_node {#get_blend_point_node}

```gdscript
AnimationRootNode get_blend_point_node(point: int) const
```

Returns the AnimationNode referenced by the point at index point.

### get_blend_point_position {#get_blend_point_position}

```gdscript
float get_blend_point_position(point: int) const
```

Returns the position of the point at index point.

### get_cyclic_length {#get_cyclic_length}

```gdscript
float get_cyclic_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_space {#get_max_space}

```gdscript
float get_max_space() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_space {#get_min_space}

```gdscript
float get_min_space() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_snap {#get_snap}

```gdscript
float get_snap() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sync_mode {#get_sync_mode}

```gdscript
AnimationNodeBlendSpace1D.SyncMode get_sync_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_value_label {#get_value_label}

```gdscript
String get_value_label() const
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

Removes the point at index point from the blend axis.

### reorder_blend_point {#reorder_blend_point}

```gdscript
void reorder_blend_point(from_index: int, to_index: int)
```

Swaps the blend points at indices from_index and to_index, exchanging their positions and properties.

### set_blend_mode {#set_blend_mode}

```gdscript
void set_blend_mode(mode: AnimationNodeBlendSpace1D.BlendMode)
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
void set_blend_point_position(point: int, pos: float)
```

Updates the position of the point at index point on the blend axis.

### set_cyclic_length {#set_cyclic_length}

```gdscript
void set_cyclic_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_space {#set_max_space}

```gdscript
void set_max_space(max_space: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_space {#set_min_space}

```gdscript
void set_min_space(min_space: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_snap {#set_snap}

```gdscript
void set_snap(snap: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sync_mode {#set_sync_mode}

```gdscript
void set_sync_mode(sync_mode: AnimationNodeBlendSpace1D.SyncMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_sync {#set_use_sync}

```gdscript
void set_use_sync(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_value_label {#set_value_label}

```gdscript
void set_value_label(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### min_space {#min_space}

```gdscript
min_space: float
```

- void set_min_space(value: float)
- float get_min_space()

The blend space's axis's lower limit for the points' position. See add_blend_point().

### max_space {#max_space}

```gdscript
max_space: float
```

- void set_max_space(value: float)
- float get_max_space()

The blend space's axis's upper limit for the points' position. See add_blend_point().

### snap {#snap}

```gdscript
snap: float
```

- void set_snap(value: float)
- float get_snap()

Position increment to snap to when moving a point on the axis.

### value_label {#value_label}

```gdscript
value_label: String
```

- void set_value_label(value: String)
- String get_value_label()

Label of the virtual axis of the blend space.

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
