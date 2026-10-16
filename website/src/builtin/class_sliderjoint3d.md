# SliderJoint3D

**继承**：[Joint3D](class_joint3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SliderJoint3D**

> A physics joint that restricts the movement of a 3D physics body along an axis relative to another physics body. For example, Body A could be a StaticBody3D representing a piston base, while Body B could be a RigidBody3D representing the piston head, moving up and down.

<details><summary>官方英文描述</summary>

A physics joint that restricts the movement of a 3D physics body along an axis relative to another physics body. For example, Body A could be a StaticBody3D representing a piston base, while Body B could be a RigidBody3D representing the piston head, moving up and down.

</details>

## 方法

### get_param {#get_param}

```gdscript
float get_param(param: SliderJoint3D.Param) const
```

Returns the value of the given parameter.

### set_param {#set_param}

```gdscript
void set_param(param: SliderJoint3D.Param, value: float)
```

Assigns value to the given parameter.

## 枚举

### Param {#enum-Param}

```gdscript
enum Param { PARAM_LINEAR_LIMIT_UPPER = 0, PARAM_LINEAR_LIMIT_LOWER = 1, PARAM_LINEAR_LIMIT_SOFTNESS = 2, PARAM_LINEAR_LIMIT_RESTITUTION = 3, PARAM_LINEAR_LIMIT_DAMPING = 4, PARAM_LINEAR_MOTION_SOFTNESS = 5, PARAM_LINEAR_MOTION_RESTITUTION = 6, PARAM_LINEAR_MOTION_DAMPING = 7, PARAM_LINEAR_ORTHOGONAL_SOFTNESS = 8, PARAM_LINEAR_ORTHOGONAL_RESTITUTION = 9, PARAM_LINEAR_ORTHOGONAL_DAMPING = 10, PARAM_ANGULAR_LIMIT_UPPER = 11, PARAM_ANGULAR_LIMIT_LOWER = 12, PARAM_ANGULAR_LIMIT_SOFTNESS = 13, PARAM_ANGULAR_LIMIT_RESTITUTION = 14, PARAM_ANGULAR_LIMIT_DAMPING = 15, PARAM_ANGULAR_MOTION_SOFTNESS = 16, PARAM_ANGULAR_MOTION_RESTITUTION = 17, PARAM_ANGULAR_MOTION_DAMPING = 18, PARAM_ANGULAR_ORTHOGONAL_SOFTNESS = 19, PARAM_ANGULAR_ORTHOGONAL_RESTITUTION = 20, PARAM_ANGULAR_ORTHOGONAL_DAMPING = 21, PARAM_MAX = 22 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_LINEAR_LIMIT_UPPER {#PARAM_LINEAR_LIMIT_UPPER}

```gdscript
PARAM_LINEAR_LIMIT_UPPER = 0
```

Constant for accessing linear_limit/upper_distance. The maximum difference between the pivot points on their X axis before damping happens.

#### PARAM_LINEAR_LIMIT_LOWER {#PARAM_LINEAR_LIMIT_LOWER}

```gdscript
PARAM_LINEAR_LIMIT_LOWER = 1
```

Constant for accessing linear_limit/lower_distance. The minimum difference between the pivot points on their X axis before damping happens.

#### PARAM_LINEAR_LIMIT_SOFTNESS {#PARAM_LINEAR_LIMIT_SOFTNESS}

```gdscript
PARAM_LINEAR_LIMIT_SOFTNESS = 2
```

Constant for accessing linear_limit/softness. A factor applied to the movement across the slider axis once the limits get surpassed. The lower, the slower the movement.

#### PARAM_LINEAR_LIMIT_RESTITUTION {#PARAM_LINEAR_LIMIT_RESTITUTION}

```gdscript
PARAM_LINEAR_LIMIT_RESTITUTION = 3
```

Constant for accessing linear_limit/restitution. The amount of restitution once the limits are surpassed. The lower, the more velocity-energy gets lost.

#### PARAM_LINEAR_LIMIT_DAMPING {#PARAM_LINEAR_LIMIT_DAMPING}

```gdscript
PARAM_LINEAR_LIMIT_DAMPING = 4
```

Constant for accessing linear_limit/damping. The amount of damping once the slider limits are surpassed.

#### PARAM_LINEAR_MOTION_SOFTNESS {#PARAM_LINEAR_MOTION_SOFTNESS}

```gdscript
PARAM_LINEAR_MOTION_SOFTNESS = 5
```

Constant for accessing linear_motion/softness. A factor applied to the movement across the slider axis as long as the slider is in the limits. The lower, the slower the movement.

#### PARAM_LINEAR_MOTION_RESTITUTION {#PARAM_LINEAR_MOTION_RESTITUTION}

```gdscript
PARAM_LINEAR_MOTION_RESTITUTION = 6
```

Constant for accessing linear_motion/restitution. The amount of restitution inside the slider limits.

#### PARAM_LINEAR_MOTION_DAMPING {#PARAM_LINEAR_MOTION_DAMPING}

```gdscript
PARAM_LINEAR_MOTION_DAMPING = 7
```

Constant for accessing linear_motion/damping. The amount of damping inside the slider limits.

#### PARAM_LINEAR_ORTHOGONAL_SOFTNESS {#PARAM_LINEAR_ORTHOGONAL_SOFTNESS}

```gdscript
PARAM_LINEAR_ORTHOGONAL_SOFTNESS = 8
```

Constant for accessing linear_ortho/softness. A factor applied to the movement across axes orthogonal to the slider.

#### PARAM_LINEAR_ORTHOGONAL_RESTITUTION {#PARAM_LINEAR_ORTHOGONAL_RESTITUTION}

```gdscript
PARAM_LINEAR_ORTHOGONAL_RESTITUTION = 9
```

Constant for accessing linear_motion/restitution. The amount of restitution when movement is across axes orthogonal to the slider.

#### PARAM_LINEAR_ORTHOGONAL_DAMPING {#PARAM_LINEAR_ORTHOGONAL_DAMPING}

```gdscript
PARAM_LINEAR_ORTHOGONAL_DAMPING = 10
```

Constant for accessing linear_motion/damping. The amount of damping when movement is across axes orthogonal to the slider.

#### PARAM_ANGULAR_LIMIT_UPPER {#PARAM_ANGULAR_LIMIT_UPPER}

```gdscript
PARAM_ANGULAR_LIMIT_UPPER = 11
```

Constant for accessing angular_limit/upper_angle. The upper limit of rotation in the slider.

#### PARAM_ANGULAR_LIMIT_LOWER {#PARAM_ANGULAR_LIMIT_LOWER}

```gdscript
PARAM_ANGULAR_LIMIT_LOWER = 12
```

Constant for accessing angular_limit/lower_angle. The lower limit of rotation in the slider.

#### PARAM_ANGULAR_LIMIT_SOFTNESS {#PARAM_ANGULAR_LIMIT_SOFTNESS}

```gdscript
PARAM_ANGULAR_LIMIT_SOFTNESS = 13
```

Constant for accessing angular_limit/softness. A factor applied to the all rotation once the limit is surpassed.

#### PARAM_ANGULAR_LIMIT_RESTITUTION {#PARAM_ANGULAR_LIMIT_RESTITUTION}

```gdscript
PARAM_ANGULAR_LIMIT_RESTITUTION = 14
```

Constant for accessing angular_limit/restitution. The amount of restitution of the rotation when the limit is surpassed.

#### PARAM_ANGULAR_LIMIT_DAMPING {#PARAM_ANGULAR_LIMIT_DAMPING}

```gdscript
PARAM_ANGULAR_LIMIT_DAMPING = 15
```

Constant for accessing angular_limit/damping. The amount of damping of the rotation when the limit is surpassed.

#### PARAM_ANGULAR_MOTION_SOFTNESS {#PARAM_ANGULAR_MOTION_SOFTNESS}

```gdscript
PARAM_ANGULAR_MOTION_SOFTNESS = 16
```

Constant for accessing angular_motion/softness. A factor applied to the all rotation in the limits.

#### PARAM_ANGULAR_MOTION_RESTITUTION {#PARAM_ANGULAR_MOTION_RESTITUTION}

```gdscript
PARAM_ANGULAR_MOTION_RESTITUTION = 17
```

Constant for accessing angular_motion/restitution. The amount of restitution of the rotation in the limits.

#### PARAM_ANGULAR_MOTION_DAMPING {#PARAM_ANGULAR_MOTION_DAMPING}

```gdscript
PARAM_ANGULAR_MOTION_DAMPING = 18
```

Constant for accessing angular_motion/damping. The amount of damping of the rotation in the limits.

#### PARAM_ANGULAR_ORTHOGONAL_SOFTNESS {#PARAM_ANGULAR_ORTHOGONAL_SOFTNESS}

```gdscript
PARAM_ANGULAR_ORTHOGONAL_SOFTNESS = 19
```

Constant for accessing angular_ortho/softness. A factor applied to the all rotation across axes orthogonal to the slider.

#### PARAM_ANGULAR_ORTHOGONAL_RESTITUTION {#PARAM_ANGULAR_ORTHOGONAL_RESTITUTION}

```gdscript
PARAM_ANGULAR_ORTHOGONAL_RESTITUTION = 20
```

Constant for accessing angular_ortho/restitution. The amount of restitution of the rotation across axes orthogonal to the slider.

#### PARAM_ANGULAR_ORTHOGONAL_DAMPING {#PARAM_ANGULAR_ORTHOGONAL_DAMPING}

```gdscript
PARAM_ANGULAR_ORTHOGONAL_DAMPING = 21
```

Constant for accessing angular_ortho/damping. The amount of damping of the rotation across axes orthogonal to the slider.

#### PARAM_MAX {#PARAM_MAX}

```gdscript
PARAM_MAX = 22
```

Represents the size of the Param enum.

---
