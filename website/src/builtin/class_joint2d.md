# Joint2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Joint2D**

> Abstract base class for all joints in 2D physics. 2D joints bind together two physics bodies (node_a and node_b) and apply a constraint.

<details><summary>官方英文描述（中文精译进行中）</summary>

Abstract base class for all joints in 2D physics. 2D joints bind together two physics bodies (node_a and node_b) and apply a constraint.

</details>

## 方法

### get_bias {#get_bias}

```gdscript
float get_bias() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_exclude_nodes_from_collision {#get_exclude_nodes_from_collision}

```gdscript
bool get_exclude_nodes_from_collision() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_node_a {#get_node_a}

```gdscript
NodePath get_node_a() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_node_b {#get_node_b}

```gdscript
NodePath get_node_b() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the joint's internal RID from the PhysicsServer2D.

### set_bias {#set_bias}

```gdscript
void set_bias(bias: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exclude_nodes_from_collision {#set_exclude_nodes_from_collision}

```gdscript
void set_exclude_nodes_from_collision(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_node_a {#set_node_a}

```gdscript
void set_node_a(node: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_node_b {#set_node_b}

```gdscript
void set_node_b(node: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### node_a {#node_a}

```gdscript
node_a: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### node_b {#node_b}

```gdscript
node_b: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### bias {#bias}

```gdscript
bias: float
```

- void set_bias(value: float)
- float get_bias()

When node_a and node_b move in different directions the bias controls how fast the joint pulls them back to their original position. The lower the bias the more the two bodies can pull on the joint.

When set to 0, the default value from ProjectSettings.physics/2d/solver/default_constraint_bias is used.

### disable_collision {#disable_collision}

```gdscript
disable_collision: bool
```

- void set_exclude_nodes_from_collision(value: bool)
- bool get_exclude_nodes_from_collision()

If true, the two bodies bound together do not collide with each other.
