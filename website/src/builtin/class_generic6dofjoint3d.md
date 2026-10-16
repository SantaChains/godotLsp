# Generic6DOFJoint3D

**继承**：[Joint3D](class_joint3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Generic6DOFJoint3D**

> The Generic6DOFJoint3D (6 Degrees Of Freedom) joint allows for implementing custom types of joints by locking the rotation and translation of certain axes.

<details><summary>官方英文描述（中文精译进行中）</summary>

The Generic6DOFJoint3D (6 Degrees Of Freedom) joint allows for implementing custom types of joints by locking the rotation and translation of certain axes.

The first 3 DOF represent the linear motion of the physics bodies and the last 3 DOF represent the angular motion of the physics bodies. Each axis can be either locked, or limited.

</details>

## 方法

### get_flag_x {#get_flag_x}

```gdscript
bool get_flag_x(flag: Generic6DOFJoint3D.Flag) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_flag_y {#get_flag_y}

```gdscript
bool get_flag_y(flag: Generic6DOFJoint3D.Flag) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_flag_z {#get_flag_z}

```gdscript
bool get_flag_z(flag: Generic6DOFJoint3D.Flag) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_param_x {#get_param_x}

```gdscript
float get_param_x(param: Generic6DOFJoint3D.Param) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_param_y {#get_param_y}

```gdscript
float get_param_y(param: Generic6DOFJoint3D.Param) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_param_z {#get_param_z}

```gdscript
float get_param_z(param: Generic6DOFJoint3D.Param) const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_flag_x {#set_flag_x}

```gdscript
void set_flag_x(flag: Generic6DOFJoint3D.Flag, value: bool)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_flag_y {#set_flag_y}

```gdscript
void set_flag_y(flag: Generic6DOFJoint3D.Flag, value: bool)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_flag_z {#set_flag_z}

```gdscript
void set_flag_z(flag: Generic6DOFJoint3D.Flag, value: bool)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_param_x {#set_param_x}

```gdscript
void set_param_x(param: Generic6DOFJoint3D.Param, value: float)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_param_y {#set_param_y}

```gdscript
void set_param_y(param: Generic6DOFJoint3D.Param, value: float)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_param_z {#set_param_z}

```gdscript
void set_param_z(param: Generic6DOFJoint3D.Param, value: float)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

## 枚举

### Param {#enum-Param}

```gdscript
enum Param { PARAM_LINEAR_LOWER_LIMIT = 0, PARAM_LINEAR_UPPER_LIMIT = 1, PARAM_LINEAR_LIMIT_SOFTNESS = 2, PARAM_LINEAR_RESTITUTION = 3, PARAM_LINEAR_DAMPING = 4, PARAM_LINEAR_MOTOR_TARGET_VELOCITY = 5, PARAM_LINEAR_MOTOR_FORCE_LIMIT = 6, PARAM_LINEAR_SPRING_STIFFNESS = 7, PARAM_LINEAR_SPRING_DAMPING = 8, PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT = 9, PARAM_ANGULAR_LOWER_LIMIT = 10, PARAM_ANGULAR_UPPER_LIMIT = 11, PARAM_ANGULAR_LIMIT_SOFTNESS = 12, PARAM_ANGULAR_DAMPING = 13, PARAM_ANGULAR_RESTITUTION = 14, PARAM_ANGULAR_FORCE_LIMIT = 15, PARAM_ANGULAR_ERP = 16, PARAM_ANGULAR_MOTOR_TARGET_VELOCITY = 17, PARAM_ANGULAR_MOTOR_FORCE_LIMIT = 18, PARAM_ANGULAR_SPRING_STIFFNESS = 19, PARAM_ANGULAR_SPRING_DAMPING = 20, PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT = 21, PARAM_MAX = 22 }
```

Param PARAM_LINEAR_LOWER_LIMIT = 0

The minimum difference between the pivot points' axes.

Param PARAM_LINEAR_UPPER_LIMIT = 1

The maximum difference between the pivot points' axes.

Param PARAM_LINEAR_LIMIT_SOFTNESS = 2

A factor applied to the movement across the axes. The lower, the slower the movement.

Param PARAM_LINEAR_RESTITUTION = 3

The amount of restitution on the axes' movement. The lower, the more momentum gets lost.

Param PARAM_LINEAR_DAMPING = 4

The amount of damping that happens at the linear motion across the axes.

Param PARAM_LINEAR_MOTOR_TARGET_VELOCITY = 5

The velocity the linear motor will try to reach.

Param PARAM_LINEAR_MOTOR_FORCE_LIMIT = 6

The maximum force the linear motor will apply while trying to reach the velocity target.

Param PARAM_LINEAR_SPRING_STIFFNESS = 7

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Param PARAM_LINEAR_SPRING_DAMPING = 8

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Param PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT = 9

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Param PARAM_ANGULAR_LOWER_LIMIT = 10

The minimum rotation in negative direction to break loose and rotate around the axes.

Param PARAM_ANGULAR_UPPER_LIMIT = 11

The minimum rotation in positive direction to break loose and rotate around the axes.

Param PARAM_ANGULAR_LIMIT_SOFTNESS = 12

The speed of all rotations across the axes.

Param PARAM_ANGULAR_DAMPING = 13

The amount of rotational damping across the axes. The lower, the more damping occurs.

Param PARAM_ANGULAR_RESTITUTION = 14

The amount of rotational restitution across the axes. The lower, the more restitution occurs.

Param PARAM_ANGULAR_FORCE_LIMIT = 15

The maximum amount of force that can occur, when rotating around the axes.

Param PARAM_ANGULAR_ERP = 16

When rotating across the axes, this error tolerance factor defines how much the correction gets slowed down. The lower, the slower.

Param PARAM_ANGULAR_MOTOR_TARGET_VELOCITY = 17

Target speed for the motor at the axes.

Param PARAM_ANGULAR_MOTOR_FORCE_LIMIT = 18

Maximum acceleration for the motor at the axes.

Param PARAM_ANGULAR_SPRING_STIFFNESS = 19

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Param PARAM_ANGULAR_SPRING_DAMPING = 20

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Param PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT = 21

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Param PARAM_MAX = 22

Represents the size of the Param enum.

#### PARAM_LINEAR_LOWER_LIMIT {#PARAM_LINEAR_LOWER_LIMIT}

```gdscript
PARAM_LINEAR_LOWER_LIMIT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_UPPER_LIMIT {#PARAM_LINEAR_UPPER_LIMIT}

```gdscript
PARAM_LINEAR_UPPER_LIMIT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_LIMIT_SOFTNESS {#PARAM_LINEAR_LIMIT_SOFTNESS}

```gdscript
PARAM_LINEAR_LIMIT_SOFTNESS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_RESTITUTION {#PARAM_LINEAR_RESTITUTION}

```gdscript
PARAM_LINEAR_RESTITUTION = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_DAMPING {#PARAM_LINEAR_DAMPING}

```gdscript
PARAM_LINEAR_DAMPING = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_MOTOR_TARGET_VELOCITY {#PARAM_LINEAR_MOTOR_TARGET_VELOCITY}

```gdscript
PARAM_LINEAR_MOTOR_TARGET_VELOCITY = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_MOTOR_FORCE_LIMIT {#PARAM_LINEAR_MOTOR_FORCE_LIMIT}

```gdscript
PARAM_LINEAR_MOTOR_FORCE_LIMIT = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_SPRING_STIFFNESS {#PARAM_LINEAR_SPRING_STIFFNESS}

```gdscript
PARAM_LINEAR_SPRING_STIFFNESS = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_SPRING_DAMPING {#PARAM_LINEAR_SPRING_DAMPING}

```gdscript
PARAM_LINEAR_SPRING_DAMPING = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT {#PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT}

```gdscript
PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_LOWER_LIMIT {#PARAM_ANGULAR_LOWER_LIMIT}

```gdscript
PARAM_ANGULAR_LOWER_LIMIT = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_UPPER_LIMIT {#PARAM_ANGULAR_UPPER_LIMIT}

```gdscript
PARAM_ANGULAR_UPPER_LIMIT = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_LIMIT_SOFTNESS {#PARAM_ANGULAR_LIMIT_SOFTNESS}

```gdscript
PARAM_ANGULAR_LIMIT_SOFTNESS = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_DAMPING {#PARAM_ANGULAR_DAMPING}

```gdscript
PARAM_ANGULAR_DAMPING = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_RESTITUTION {#PARAM_ANGULAR_RESTITUTION}

```gdscript
PARAM_ANGULAR_RESTITUTION = 14
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_FORCE_LIMIT {#PARAM_ANGULAR_FORCE_LIMIT}

```gdscript
PARAM_ANGULAR_FORCE_LIMIT = 15
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_ERP {#PARAM_ANGULAR_ERP}

```gdscript
PARAM_ANGULAR_ERP = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_MOTOR_TARGET_VELOCITY {#PARAM_ANGULAR_MOTOR_TARGET_VELOCITY}

```gdscript
PARAM_ANGULAR_MOTOR_TARGET_VELOCITY = 17
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_MOTOR_FORCE_LIMIT {#PARAM_ANGULAR_MOTOR_FORCE_LIMIT}

```gdscript
PARAM_ANGULAR_MOTOR_FORCE_LIMIT = 18
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_SPRING_STIFFNESS {#PARAM_ANGULAR_SPRING_STIFFNESS}

```gdscript
PARAM_ANGULAR_SPRING_STIFFNESS = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_SPRING_DAMPING {#PARAM_ANGULAR_SPRING_DAMPING}

```gdscript
PARAM_ANGULAR_SPRING_DAMPING = 20
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT {#PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT}

```gdscript
PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT = 21
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_MAX {#PARAM_MAX}

```gdscript
PARAM_MAX = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Flag {#enum-Flag}

```gdscript
enum Flag { FLAG_ENABLE_LINEAR_LIMIT = 0, FLAG_ENABLE_ANGULAR_LIMIT = 1, FLAG_ENABLE_LINEAR_SPRING = 3, FLAG_ENABLE_ANGULAR_SPRING = 2, FLAG_ENABLE_MOTOR = 4, FLAG_ENABLE_LINEAR_MOTOR = 5, FLAG_MAX = 6 }
```

Flag FLAG_ENABLE_LINEAR_LIMIT = 0

If enabled, linear motion is possible within the given limits.

Flag FLAG_ENABLE_ANGULAR_LIMIT = 1

If enabled, rotational motion is possible within the given limits.

Flag FLAG_ENABLE_LINEAR_SPRING = 3

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Flag FLAG_ENABLE_ANGULAR_SPRING = 2

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Flag FLAG_ENABLE_MOTOR = 4

If enabled, there is a rotational motor across these axes.

Flag FLAG_ENABLE_LINEAR_MOTOR = 5

If enabled, there is a linear motor across these axes.

Flag FLAG_MAX = 6

Represents the size of the Flag enum.

#### FLAG_ENABLE_LINEAR_LIMIT {#FLAG_ENABLE_LINEAR_LIMIT}

```gdscript
FLAG_ENABLE_LINEAR_LIMIT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_ENABLE_ANGULAR_LIMIT {#FLAG_ENABLE_ANGULAR_LIMIT}

```gdscript
FLAG_ENABLE_ANGULAR_LIMIT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_ENABLE_LINEAR_SPRING {#FLAG_ENABLE_LINEAR_SPRING}

```gdscript
FLAG_ENABLE_LINEAR_SPRING = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_ENABLE_ANGULAR_SPRING {#FLAG_ENABLE_ANGULAR_SPRING}

```gdscript
FLAG_ENABLE_ANGULAR_SPRING = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_ENABLE_MOTOR {#FLAG_ENABLE_MOTOR}

```gdscript
FLAG_ENABLE_MOTOR = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_ENABLE_LINEAR_MOTOR {#FLAG_ENABLE_LINEAR_MOTOR}

```gdscript
FLAG_ENABLE_LINEAR_MOTOR = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_MAX {#FLAG_MAX}

```gdscript
FLAG_MAX = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
