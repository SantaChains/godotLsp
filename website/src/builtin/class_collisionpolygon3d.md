# CollisionPolygon3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CollisionPolygon3D**

> A node that provides a thickened polygon shape (a prism) to a CollisionObject3D parent and allows it to be edited. The polygon can be concave or convex. This can give a detection shape to an Area3D or turn a PhysicsBody3D into a solid object.

<details><summary>官方英文描述（中文精译进行中）</summary>

A node that provides a thickened polygon shape (a prism) to a CollisionObject3D parent and allows it to be edited. The polygon can be concave or convex. This can give a detection shape to an Area3D or turn a PhysicsBody3D into a solid object.

Warning: A non-uniformly scaled CollisionShape3D will likely not behave as expected. Make sure to keep its scale the same on all axes and adjust its shape resource instead.

</details>

## 方法

### get_debug_color {#get_debug_color}

```gdscript
Color get_debug_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_depth {#get_depth}

```gdscript
float get_depth() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_debug_fill {#get_enable_debug_fill}

```gdscript
bool get_enable_debug_fill() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_margin {#get_margin}

```gdscript
float get_margin() const
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

### set_debug_color {#set_debug_color}

```gdscript
void set_debug_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_depth {#set_depth}

```gdscript
void set_depth(depth: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disabled {#set_disabled}

```gdscript
void set_disabled(disabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_debug_fill {#set_enable_debug_fill}

```gdscript
void set_enable_debug_fill(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin {#set_margin}

```gdscript
void set_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_polygon {#set_polygon}

```gdscript
void set_polygon(polygon: PackedVector2Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### depth {#depth}

```gdscript
depth: float
```

- void set_depth(value: float)
- float get_depth()

Length that the resulting collision extends in either direction perpendicular to its 2D polygon.

### disabled {#disabled}

```gdscript
disabled: bool
```

- void set_disabled(value: bool)
- bool is_disabled()

If true, no collision will be produced. This property should be changed with Object.set_deferred().

### polygon {#polygon}

```gdscript
polygon: PackedVector2Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### margin {#margin}

```gdscript
margin: float
```

- void set_margin(value: float)
- float get_margin()

The collision margin for the generated Shape3D. See Shape3D.margin for more details.

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
