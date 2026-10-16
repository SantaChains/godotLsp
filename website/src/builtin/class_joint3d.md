# Joint3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Joint3D**

> Abstract base class for all joints in 3D physics. 3D joints bind together two physics bodies (node_a and node_b) and apply a constraint. If only one body is defined, it is attached to a fixed StaticBody3D without collision shapes.

<details><summary>官方英文描述</summary>

Abstract base class for all joints in 3D physics. 3D joints bind together two physics bodies (node_a and node_b) and apply a constraint. If only one body is defined, it is attached to a fixed StaticBody3D without collision shapes.

</details>

## 方法

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

Returns the joint's internal RID from the PhysicsServer3D.

### get_solver_priority {#get_solver_priority}

```gdscript
int get_solver_priority() const
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

### set_solver_priority {#set_solver_priority}

```gdscript
void set_solver_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### node_a {#node_a}

```gdscript
node_a: NodePath
```

- void set_node_a(value: NodePath)
- NodePath get_node_a()

Path to the first node (A) attached to the joint. The node must inherit PhysicsBody3D.

If left empty and node_b is set, the body is attached to a fixed StaticBody3D without collision shapes.

### node_b {#node_b}

```gdscript
node_b: NodePath
```

- void set_node_b(value: NodePath)
- NodePath get_node_b()

Path to the second node (B) attached to the joint. The node must inherit PhysicsBody3D.

If left empty and node_a is set, the body is attached to a fixed StaticBody3D without collision shapes.

### solver_priority {#solver_priority}

```gdscript
solver_priority: int
```

- void set_solver_priority(value: int)
- int get_solver_priority()

The priority used to define which solver is executed first for multiple joints. The lower the value, the higher the priority.

Note: Only supported when using GodotPhysics3D. This property is ignored when using Jolt Physics.

### exclude_nodes_from_collision {#exclude_nodes_from_collision}

```gdscript
exclude_nodes_from_collision: bool
```

- void set_exclude_nodes_from_collision(value: bool)
- bool get_exclude_nodes_from_collision()

If true, the two bodies bound together do not collide with each other.
