# ShapeCast2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ShapeCast2D**

> Shape casting allows to detect collision objects by sweeping its shape along the cast direction determined by target_position. This is similar to RayCast2D, but it allows for sweeping a region of space, rather than just a straight line. ShapeCast2D can detect multiple collision objects. It is useful for things like wide laser beams or snapping a simple shape to a floor.

<details><summary>官方英文描述（中文精译进行中）</summary>

Shape casting allows to detect collision objects by sweeping its shape along the cast direction determined by target_position. This is similar to RayCast2D, but it allows for sweeping a region of space, rather than just a straight line. ShapeCast2D can detect multiple collision objects. It is useful for things like wide laser beams or snapping a simple shape to a floor.

Immediate collision overlaps can be done with the target_position set to Vector2(0, 0) and by calling force_shapecast_update() within the same physics frame. This helps to overcome some limitations of Area2D when used as an instantaneous detection area, as collision information isn't immediately available to it.

Note: Shape casting is more computationally expensive than ray casting.

</details>

## 方法

### add_exception {#add_exception}

```gdscript
void add_exception(node: CollisionObject2D)
```

Adds a collision exception so the shape does not report collisions with the specified node.

### add_exception_rid {#add_exception_rid}

```gdscript
void add_exception_rid(rid: RID)
```

Adds a collision exception so the shape does not report collisions with the specified RID.

### clear_exceptions {#clear_exceptions}

```gdscript
void clear_exceptions()
```

Removes all collision exceptions for this shape.

### force_shapecast_update {#force_shapecast_update}

```gdscript
void force_shapecast_update()
```

Updates the collision information for the shape immediately, without waiting for the next _physics_process call. Use this method, for example, when the shape or its parent has changed state.

Note: Setting enabled to true is not required for this to work.

### get_closest_collision_safe_fraction {#get_closest_collision_safe_fraction}

```gdscript
float get_closest_collision_safe_fraction() const
```

Returns the fraction from this cast's origin to its target_position of how far the shape can move without triggering a collision, as a value between 0.0 and 1.0.

### get_closest_collision_unsafe_fraction {#get_closest_collision_unsafe_fraction}

```gdscript
float get_closest_collision_unsafe_fraction() const
```

Returns the fraction from this cast's origin to its target_position of how far the shape must move to trigger a collision, as a value between 0.0 and 1.0.

In ideal conditions this would be the same as get_closest_collision_safe_fraction(), however shape casting is calculated in discrete steps, so the precise point of collision can occur between two calculated positions.

### get_collider {#get_collider}

```gdscript
Object get_collider(index: int) const
```

Returns the collided Object of one of the multiple collisions at index, or null if no object is intersecting the shape (i.e. is_colliding() returns false).

### get_collider_rid {#get_collider_rid}

```gdscript
RID get_collider_rid(index: int) const
```

Returns the RID of the collided object of one of the multiple collisions at index.

### get_collider_shape {#get_collider_shape}

```gdscript
int get_collider_shape(index: int) const
```

Returns the shape ID of the colliding shape of one of the multiple collisions at index, or 0 if no object is intersecting the shape (i.e. is_colliding() returns false).

### get_collision_count {#get_collision_count}

```gdscript
int get_collision_count() const
```

The number of collisions detected at the point of impact. Use this to iterate over multiple collisions as provided by get_collider(), get_collider_shape(), get_collision_point(), and get_collision_normal() methods.

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_collision_mask_value {#get_collision_mask_value}

```gdscript
bool get_collision_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the collision_mask is enabled, given a layer_number between 1 and 32.

### get_collision_normal {#get_collision_normal}

```gdscript
Vector2 get_collision_normal(index: int) const
```

Returns the normal of one of the multiple collisions at index of the intersecting object.

### get_collision_point {#get_collision_point}

```gdscript
Vector2 get_collision_point(index: int) const
```

Returns the collision point of one of the multiple collisions at index where the shape intersects the colliding object.

Note: This point is in the global coordinate system.

### get_collision_result {#get_collision_result}

```gdscript
Array get_collision_result() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_exclude_parent_body {#get_exclude_parent_body}

```gdscript
bool get_exclude_parent_body() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_margin {#get_margin}

```gdscript
float get_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_results {#get_max_results}

```gdscript
int get_max_results() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
Shape2D get_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_position {#get_target_position}

```gdscript
Vector2 get_target_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collide_with_areas_enabled {#is_collide_with_areas_enabled}

```gdscript
bool is_collide_with_areas_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collide_with_bodies_enabled {#is_collide_with_bodies_enabled}

```gdscript
bool is_collide_with_bodies_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_colliding {#is_colliding}

```gdscript
bool is_colliding() const
```

Returns whether any object is intersecting with the shape's vector (considering the vector length).

### is_enabled {#is_enabled}

```gdscript
bool is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_exception {#remove_exception}

```gdscript
void remove_exception(node: CollisionObject2D)
```

Removes a collision exception so the shape does report collisions with the specified node.

### remove_exception_rid {#remove_exception_rid}

```gdscript
void remove_exception_rid(rid: RID)
```

Removes a collision exception so the shape does report collisions with the specified RID.

### set_collide_with_areas {#set_collide_with_areas}

```gdscript
void set_collide_with_areas(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collide_with_bodies {#set_collide_with_bodies}

```gdscript
void set_collide_with_bodies(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask_value {#set_collision_mask_value}

```gdscript
void set_collision_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the collision_mask, given a layer_number between 1 and 32.

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exclude_parent_body {#set_exclude_parent_body}

```gdscript
void set_exclude_parent_body(mask: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin {#set_margin}

```gdscript
void set_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_results {#set_max_results}

```gdscript
void set_max_results(max_results: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape {#set_shape}

```gdscript
void set_shape(shape: Shape2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_position {#set_target_position}

```gdscript
void set_target_position(local_point: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

If true, collisions will be reported.

### shape {#shape}

```gdscript
shape: Shape2D
```

- void set_shape(value: Shape2D)
- Shape2D get_shape()

The shape to be used for collision queries.

### exclude_parent {#exclude_parent}

```gdscript
exclude_parent: bool
```

- void set_exclude_parent_body(value: bool)
- bool get_exclude_parent_body()

If true, the parent node will be excluded from collision detection.

### target_position {#target_position}

```gdscript
target_position: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### margin {#margin}

```gdscript
margin: float
```

- void set_margin(value: float)
- float get_margin()

The collision margin for the shape. A larger margin helps detecting collisions more consistently, at the cost of precision.

### max_results {#max_results}

```gdscript
max_results: int
```

- void set_max_results(value: int)
- int get_max_results()

The number of intersections can be limited with this parameter, to reduce the processing time.

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The shape's collision mask. Only objects in at least one collision layer enabled in the mask will be detected. See Collision layers and masks in the documentation for more information.

### collision_result {#collision_result}

```gdscript
collision_result: Array
```

- Array get_collision_result()

Returns the complete collision information from the collision sweep. The data returned is the same as in the PhysicsDirectSpaceState2D.get_rest_info() method.

### collide_with_areas {#collide_with_areas}

```gdscript
collide_with_areas: bool
```

- void set_collide_with_areas(value: bool)
- bool is_collide_with_areas_enabled()

If true, collisions with Area2Ds will be reported.

### collide_with_bodies {#collide_with_bodies}

```gdscript
collide_with_bodies: bool
```

- void set_collide_with_bodies(value: bool)
- bool is_collide_with_bodies_enabled()

If true, collisions with PhysicsBody2Ds will be reported.
