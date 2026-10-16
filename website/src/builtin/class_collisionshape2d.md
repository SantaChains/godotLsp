# CollisionShape2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **CollisionShape2D**

> A node that provides a Shape2D to a CollisionObject2D parent and allows it to be edited. This can give a detection shape to an Area2D or turn a PhysicsBody2D into a solid object.

<details><summary>官方英文描述（中文精译进行中）</summary>

A node that provides a Shape2D to a CollisionObject2D parent and allows it to be edited. This can give a detection shape to an Area2D or turn a PhysicsBody2D into a solid object.

</details>

## 方法

### get_debug_color {#get_debug_color}

```gdscript
Color get_debug_color() const
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

### get_shape {#get_shape}

```gdscript
Shape2D get_shape() const
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

### set_debug_color {#set_debug_color}

```gdscript
void set_debug_color(color: Color)
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

### set_shape {#set_shape}

```gdscript
void set_shape(shape: Shape2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### shape {#shape}

```gdscript
shape: Shape2D
```

- void set_shape(value: Shape2D)
- Shape2D get_shape()

The actual shape owned by this collision shape.

### disabled {#disabled}

```gdscript
disabled: bool
```

- void set_disabled(value: bool)
- bool is_disabled()

A disabled collision shape has no effect in the world. This property should be changed with Object.set_deferred().

### one_way_collision {#one_way_collision}

```gdscript
one_way_collision: bool
```

- void set_one_way_collision(value: bool)
- bool is_one_way_collision_enabled()

Sets whether this collision shape should only detect collision on one side (top or bottom).

Note: This property has no effect if this CollisionShape2D is a child of an Area2D node.

Note: The one way collision direction can be configured by setting one_way_collision_direction.

### one_way_collision_margin {#one_way_collision_margin}

```gdscript
one_way_collision_margin: float
```

- void set_one_way_collision_margin(value: float)
- float get_one_way_collision_margin()

The margin used for one-way collision (in pixels). Higher values will make the shape thicker, and work better for colliders that enter the shape at a high velocity.

### one_way_collision_direction {#one_way_collision_direction}

```gdscript
one_way_collision_direction: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### debug_color {#debug_color}

```gdscript
debug_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
