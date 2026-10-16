# CollisionShape3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CollisionShape3D**

> A node that provides a Shape3D to a CollisionObject3D parent and allows it to be edited. This can give a detection shape to an Area3D or turn a PhysicsBody3D into a solid object.

<details><summary>官方英文描述（中文精译进行中）</summary>

A node that provides a Shape3D to a CollisionObject3D parent and allows it to be edited. This can give a detection shape to an Area3D or turn a PhysicsBody3D into a solid object.

Warning: A non-uniformly scaled CollisionShape3D will likely not behave as expected. Make sure to keep its scale the same on all axes and adjust its shape resource instead.

</details>

## 方法

### get_debug_color {#get_debug_color}

```gdscript
Color get_debug_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_debug_fill {#get_enable_debug_fill}

```gdscript
bool get_enable_debug_fill() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
Shape3D get_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_disabled {#is_disabled}

```gdscript
bool is_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### make_convex_from_siblings {#make_convex_from_siblings}

```gdscript
void make_convex_from_siblings()
```

Sets the collision shape's shape to the addition of all its convexed MeshInstance3D siblings geometry.

### resource_changed {#resource_changed}

```gdscript
void resource_changed(resource: Resource)
```

Deprecated: Use Resource.changed instead.

This method does nothing.

### set_debug_color {#set_debug_color}

```gdscript
void set_debug_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disabled {#set_disabled}

```gdscript
void set_disabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_debug_fill {#set_enable_debug_fill}

```gdscript
void set_enable_debug_fill(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape {#set_shape}

```gdscript
void set_shape(shape: Shape3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### shape {#shape}

```gdscript
shape: Shape3D
```

- void set_shape(value: Shape3D)
- Shape3D get_shape()

The actual shape owned by this collision shape.

### disabled {#disabled}

```gdscript
disabled: bool
```

- void set_disabled(value: bool)
- bool is_disabled()

A disabled collision shape has no effect in the world. This property should be changed with Object.set_deferred().

### debug_color {#debug_color}

```gdscript
debug_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### debug_fill {#debug_fill}

```gdscript
debug_fill: bool
```

- void set_enable_debug_fill(value: bool)
- bool get_enable_debug_fill()

If true, when the shape is displayed, it will show a solid fill color in addition to its wireframe.
