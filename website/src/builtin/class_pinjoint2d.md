# PinJoint2D

**继承**：[Joint2D](class_joint2d.md) → [Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **PinJoint2D**

> A physics joint that attaches two 2D physics bodies at a single point, allowing them to freely rotate. For example, a RigidBody2D can be attached to a StaticBody2D to create a pendulum or a seesaw.

<details><summary>官方英文描述（中文精译进行中）</summary>

A physics joint that attaches two 2D physics bodies at a single point, allowing them to freely rotate. For example, a RigidBody2D can be attached to a StaticBody2D to create a pendulum or a seesaw.

</details>

## 方法

### get_angular_limit_lower {#get_angular_limit_lower}

```gdscript
float get_angular_limit_lower() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_angular_limit_upper {#get_angular_limit_upper}

```gdscript
float get_angular_limit_upper() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_motor_target_velocity {#get_motor_target_velocity}

```gdscript
float get_motor_target_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_softness {#get_softness}

```gdscript
float get_softness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_angular_limit_enabled {#is_angular_limit_enabled}

```gdscript
bool is_angular_limit_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_motor_enabled {#is_motor_enabled}

```gdscript
bool is_motor_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_limit_enabled {#set_angular_limit_enabled}

```gdscript
void set_angular_limit_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_limit_lower {#set_angular_limit_lower}

```gdscript
void set_angular_limit_lower(angular_limit_lower: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_limit_upper {#set_angular_limit_upper}

```gdscript
void set_angular_limit_upper(angular_limit_upper: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_motor_enabled {#set_motor_enabled}

```gdscript
void set_motor_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_motor_target_velocity {#set_motor_target_velocity}

```gdscript
void set_motor_target_velocity(motor_target_velocity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_softness {#set_softness}

```gdscript
void set_softness(softness: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### softness {#softness}

```gdscript
softness: float
```

- void set_softness(value: float)
- float get_softness()

The higher this value, the more the bond to the pinned partner can flex.

### angular_limit_enabled {#angular_limit_enabled}

```gdscript
angular_limit_enabled: bool
```

- void set_angular_limit_enabled(value: bool)
- bool is_angular_limit_enabled()

If true, the pin maximum and minimum rotation, defined by angular_limit_lower and angular_limit_upper are applied.

### angular_limit_lower {#angular_limit_lower}

```gdscript
angular_limit_lower: float
```

- void set_angular_limit_lower(value: float)
- float get_angular_limit_lower()

The minimum rotation. Only active if angular_limit_enabled is true.

### angular_limit_upper {#angular_limit_upper}

```gdscript
angular_limit_upper: float
```

- void set_angular_limit_upper(value: float)
- float get_angular_limit_upper()

The maximum rotation. Only active if angular_limit_enabled is true.

### motor_enabled {#motor_enabled}

```gdscript
motor_enabled: bool
```

- void set_motor_enabled(value: bool)
- bool is_motor_enabled()

When activated, a motor turns the pin.

### motor_target_velocity {#motor_target_velocity}

```gdscript
motor_target_velocity: float
```

- void set_motor_target_velocity(value: float)
- float get_motor_target_velocity()

Target speed for the motor. In radians per second.
