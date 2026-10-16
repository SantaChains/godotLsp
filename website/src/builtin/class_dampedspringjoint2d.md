# DampedSpringJoint2D

**继承**：[Joint2D](class_joint2d.md) → [Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **DampedSpringJoint2D**

> A physics joint that connects two 2D physics bodies with a spring-like force. This behaves like a spring that always wants to stretch to a given length.

<details><summary>官方英文描述</summary>

A physics joint that connects two 2D physics bodies with a spring-like force. This behaves like a spring that always wants to stretch to a given length.

</details>

## 方法

### get_damping {#get_damping}

```gdscript
float get_damping() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_length {#get_length}

```gdscript
float get_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rest_length {#get_rest_length}

```gdscript
float get_rest_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stiffness {#get_stiffness}

```gdscript
float get_stiffness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_damping {#set_damping}

```gdscript
void set_damping(damping: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_length {#set_length}

```gdscript
void set_length(length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rest_length {#set_rest_length}

```gdscript
void set_rest_length(rest_length: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stiffness {#set_stiffness}

```gdscript
void set_stiffness(stiffness: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### length {#length}

```gdscript
length: float
```

- void set_length(value: float)
- float get_length()

The spring joint's maximum length. The two attached bodies cannot stretch it past this value.

### rest_length {#rest_length}

```gdscript
rest_length: float
```

- void set_rest_length(value: float)
- float get_rest_length()

When the bodies attached to the spring joint move they stretch or squash it. The joint always tries to resize towards this length.

### stiffness {#stiffness}

```gdscript
stiffness: float
```

- void set_stiffness(value: float)
- float get_stiffness()

The higher the value, the less the bodies attached to the joint will deform it. The joint applies an opposing force to the bodies, the product of the stiffness multiplied by the size difference from its resting length.

### damping {#damping}

```gdscript
damping: float
```

- void set_damping(value: float)
- float get_damping()

The spring joint's damping ratio. A value between 0 and 1. When the two bodies move into different directions the system tries to align them to the spring axis again. A high damping value forces the attached bodies to align faster.
