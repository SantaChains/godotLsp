# GrooveJoint2D

**继承**：[Joint2D](class_joint2d.md) → [Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **GrooveJoint2D**

> A physics joint that restricts the movement of two 2D physics bodies to a fixed axis. For example, a StaticBody2D representing a piston base can be attached to a RigidBody2D representing the piston head, moving up and down.

<details><summary>官方英文描述（中文精译进行中）</summary>

A physics joint that restricts the movement of two 2D physics bodies to a fixed axis. For example, a StaticBody2D representing a piston base can be attached to a RigidBody2D representing the piston head, moving up and down.

</details>

## 方法

### get_initial_offset {#get_initial_offset}

```gdscript
float get_initial_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_length {#get_length}

```gdscript
float get_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_initial_offset {#set_initial_offset}

```gdscript
void set_initial_offset(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_length {#set_length}

```gdscript
void set_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### length {#length}

```gdscript
length: float
```

- void set_length(value: float)
- float get_length()

The groove's length. The groove is from the joint's origin towards length along the joint's local Y axis.

### initial_offset {#initial_offset}

```gdscript
initial_offset: float
```

- void set_initial_offset(value: float)
- float get_initial_offset()

The body B's initial anchor position defined by the joint's origin and a local offset initial_offset along the joint's Y axis (along the groove).
