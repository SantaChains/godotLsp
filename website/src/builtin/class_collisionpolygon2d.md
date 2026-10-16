# CollisionPolygon2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **CollisionPolygon2D**

> A node that provides a polygon shape to a CollisionObject2D parent and allows it to be edited. The polygon can be concave or convex. This can give a detection shape to an Area2D, turn a PhysicsBody2D into a solid object, or give a hollow shape to a StaticBody2D.

<details><summary>官方英文描述</summary>

A node that provides a polygon shape to a CollisionObject2D parent and allows it to be edited. The polygon can be concave or convex. This can give a detection shape to an Area2D, turn a PhysicsBody2D into a solid object, or give a hollow shape to a StaticBody2D.

Warning: A non-uniformly scaled CollisionPolygon2D will likely not behave as expected. Make sure to keep its scale the same on all axes and adjust its polygon instead.

</details>

## 方法

### get_build_mode {#get_build_mode}

```gdscript
CollisionPolygon2D.BuildMode get_build_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_one_way_collision_direction {#get_one_way_collision_direction}

```gdscript
Vector2 get_one_way_collision_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_one_way_collision_margin {#get_one_way_collision_margin}

```gdscript
float get_one_way_collision_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_polygon {#get_polygon}

```gdscript
PackedVector2Array get_polygon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_disabled {#is_disabled}

```gdscript
bool is_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_one_way_collision_enabled {#is_one_way_collision_enabled}

```gdscript
bool is_one_way_collision_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_build_mode {#set_build_mode}

```gdscript
void set_build_mode(build_mode: CollisionPolygon2D.BuildMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disabled {#set_disabled}

```gdscript
void set_disabled(disabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_one_way_collision {#set_one_way_collision}

```gdscript
void set_one_way_collision(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_one_way_collision_direction {#set_one_way_collision_direction}

```gdscript
void set_one_way_collision_direction(direction: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_one_way_collision_margin {#set_one_way_collision_margin}

```gdscript
void set_one_way_collision_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_polygon {#set_polygon}

```gdscript
void set_polygon(polygon: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### build_mode {#build_mode}

```gdscript
build_mode: int
```

- void set_build_mode(value: BuildMode)
- BuildMode get_build_mode()

Collision build mode.

### polygon {#polygon}

```gdscript
polygon: PackedVector2Array
```

- void set_polygon(value: PackedVector2Array)
- PackedVector2Array get_polygon()

The polygon's list of vertices. Each point will be connected to the next, and the final point will be connected to the first.

Note: The returned vertices are in the local coordinate space of the given CollisionPolygon2D.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector2Array for more details.

### disabled {#disabled}

```gdscript
disabled: bool
```

- void set_disabled(value: bool)
- bool is_disabled()

If true, no collisions will be detected. This property should be changed with Object.set_deferred().

### one_way_collision {#one_way_collision}

```gdscript
one_way_collision: bool
```

- void set_one_way_collision(value: bool)
- bool is_one_way_collision_enabled()

If true, only edges that face up, relative to CollisionPolygon2D's rotation, will collide with other objects.

Note: This property has no effect if this CollisionPolygon2D is a child of an Area2D node.

Note: The one way collision direction can be configured by setting one_way_collision_direction.

### one_way_collision_margin {#one_way_collision_margin}

```gdscript
one_way_collision_margin: float
```

- void set_one_way_collision_margin(value: float)
- float get_one_way_collision_margin()

The margin used for one-way collision (in pixels). Higher values will make the shape thicker, and work better for colliders that enter the polygon at a high velocity.

### one_way_collision_direction {#one_way_collision_direction}

```gdscript
one_way_collision_direction: Vector2
```

- void set_one_way_collision_direction(value: Vector2)
- Vector2 get_one_way_collision_direction()

The direction used for one-way collision.

## 枚举

### BuildMode {#enum-BuildMode}

```gdscript
enum BuildMode { BUILD_SOLIDS = 0, BUILD_SEGMENTS = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BUILD_SOLIDS {#BUILD_SOLIDS}

```gdscript
BUILD_SOLIDS = 0
```

Collisions will include the polygon and its contained area. In this mode the node has the same effect as several ConvexPolygonShape2D nodes, one for each convex shape in the convex decomposition of the polygon (but without the overhead of multiple nodes).

#### BUILD_SEGMENTS {#BUILD_SEGMENTS}

```gdscript
BUILD_SEGMENTS = 1
```

Collisions will only include the polygon edges. In this mode the node has the same effect as a single ConcavePolygonShape2D made of segments, with the restriction that each segment (after the first one) starts where the previous one ends, and the last one ends where the first one starts (forming a closed but hollow polygon).

---
