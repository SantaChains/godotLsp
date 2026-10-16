# HingeJoint3D

**继承**：[Joint3D](class_joint3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **HingeJoint3D**

> A physics joint that restricts the rotation of a 3D physics body around an axis relative to another physics body. For example, Body A can be a StaticBody3D representing a door hinge that a RigidBody3D rotates around.

<details><summary>官方英文描述（中文精译进行中）</summary>

A physics joint that restricts the rotation of a 3D physics body around an axis relative to another physics body. For example, Body A can be a StaticBody3D representing a door hinge that a RigidBody3D rotates around.

</details>

## 方法

### get_flag {#get_flag}

```gdscript
bool get_flag(flag: HingeJoint3D.Flag) const
```

Returns the value of the specified flag.

### get_param {#get_param}

```gdscript
float get_param(param: HingeJoint3D.Param) const
```

Returns the value of the specified parameter.

### set_flag {#set_flag}

```gdscript
void set_flag(flag: HingeJoint3D.Flag, enabled: bool)
```

If true, enables the specified flag.

### set_param {#set_param}

```gdscript
void set_param(param: HingeJoint3D.Param, value: float)
```

Sets the value of the specified parameter.

## 枚举

### Param {#enum-Param}

```gdscript
enum Param { PARAM_BIAS = 0, PARAM_LIMIT_UPPER = 1, PARAM_LIMIT_LOWER = 2, PARAM_LIMIT_BIAS = 3, PARAM_LIMIT_SOFTNESS = 4, PARAM_LIMIT_RELAXATION = 5, PARAM_MOTOR_TARGET_VELOCITY = 6, PARAM_MOTOR_MAX_IMPULSE = 7, PARAM_MAX = 8 }
```

Param PARAM_BIAS = 0

The speed with which the two bodies get pulled together when they move in different directions.

Param PARAM_LIMIT_UPPER = 1

The maximum rotation. Only active if angular_limit/enable is true.

Param PARAM_LIMIT_LOWER = 2

The minimum rotation. Only active if angular_limit/enable is true.

Param PARAM_LIMIT_BIAS = 3

The speed with which the rotation across the axis perpendicular to the hinge gets corrected.

Param PARAM_LIMIT_SOFTNESS = 4

Deprecated: This property is never used by the engine and is kept for compatibility purpose.

Param PARAM_LIMIT_RELAXATION = 5

The lower this value, the more the rotation gets slowed down.

Param PARAM_MOTOR_TARGET_VELOCITY = 6

Target speed for the motor.

Param PARAM_MOTOR_MAX_IMPULSE = 7

Maximum acceleration for the motor.

Param PARAM_MAX = 8

Represents the size of the Param enum.

#### PARAM_BIAS {#PARAM_BIAS}

```gdscript
PARAM_BIAS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LIMIT_UPPER {#PARAM_LIMIT_UPPER}

```gdscript
PARAM_LIMIT_UPPER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LIMIT_LOWER {#PARAM_LIMIT_LOWER}

```gdscript
PARAM_LIMIT_LOWER = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LIMIT_BIAS {#PARAM_LIMIT_BIAS}

```gdscript
PARAM_LIMIT_BIAS = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LIMIT_SOFTNESS {#PARAM_LIMIT_SOFTNESS}

```gdscript
PARAM_LIMIT_SOFTNESS = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LIMIT_RELAXATION {#PARAM_LIMIT_RELAXATION}

```gdscript
PARAM_LIMIT_RELAXATION = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_MOTOR_TARGET_VELOCITY {#PARAM_MOTOR_TARGET_VELOCITY}

```gdscript
PARAM_MOTOR_TARGET_VELOCITY = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_MOTOR_MAX_IMPULSE {#PARAM_MOTOR_MAX_IMPULSE}

```gdscript
PARAM_MOTOR_MAX_IMPULSE = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_MAX {#PARAM_MAX}

```gdscript
PARAM_MAX = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Flag {#enum-Flag}

```gdscript
enum Flag { FLAG_USE_LIMIT = 0, FLAG_ENABLE_MOTOR = 1, FLAG_MAX = 2 }
```

Flag FLAG_USE_LIMIT = 0

If true, the hinges maximum and minimum rotation, defined by angular_limit/lower and angular_limit/upper has effects.

Flag FLAG_ENABLE_MOTOR = 1

When activated, a motor turns the hinge.

Flag FLAG_MAX = 2

Represents the size of the Flag enum.

#### FLAG_USE_LIMIT {#FLAG_USE_LIMIT}

```gdscript
FLAG_USE_LIMIT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_ENABLE_MOTOR {#FLAG_ENABLE_MOTOR}

```gdscript
FLAG_ENABLE_MOTOR = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_MAX {#FLAG_MAX}

```gdscript
FLAG_MAX = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
