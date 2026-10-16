# ConeTwistJoint3D

**继承**：[Joint3D](class_joint3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **ConeTwistJoint3D**

> A physics joint that connects two 3D physics bodies in a way that simulates a ball-and-socket joint. The twist axis is initiated as the X axis of the ConeTwistJoint3D. Once the physics bodies swing, the twist axis is calculated as the middle of the X axes of the joint in the local space of the two physics bodies. Useful for limbs like shoulders and hips, lamps hanging off a ceiling, etc.

<details><summary>官方英文描述（中文精译进行中）</summary>

A physics joint that connects two 3D physics bodies in a way that simulates a ball-and-socket joint. The twist axis is initiated as the X axis of the ConeTwistJoint3D. Once the physics bodies swing, the twist axis is calculated as the middle of the X axes of the joint in the local space of the two physics bodies. Useful for limbs like shoulders and hips, lamps hanging off a ceiling, etc.

</details>

## 方法

### get_param {#get_param}

```gdscript
float get_param(param: ConeTwistJoint3D.Param) const
```

Returns the value of the specified parameter.

### set_param {#set_param}

```gdscript
void set_param(param: ConeTwistJoint3D.Param, value: float)
```

Sets the value of the specified parameter.

## 属性

### swing_span {#swing_span}

```gdscript
swing_span: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Swing is rotation from side to side, around the axis perpendicular to the twist axis.

The swing span defines, how much rotation will not get corrected along the swing axis.

Could be defined as looseness in the ConeTwistJoint3D.

If below 0.05, this behavior is locked.

### twist_span {#twist_span}

```gdscript
twist_span: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Twist is the rotation around the twist axis, this value defined how far the joint can twist.

Twist is locked if below 0.05.

### bias {#bias}

```gdscript
bias: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

The speed with which the swing or twist will take place.

The higher, the faster.

### softness {#softness}

```gdscript
softness: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

The ease with which the joint starts to twist. If it's too low, it takes more force to start twisting the joint.

### relaxation {#relaxation}

```gdscript
relaxation: float
```

- void set_param(param: Param, value: float)
- float get_param(param: Param) const

Defines, how fast the swing- and twist-speed-difference on both sides gets synced.

## 枚举

### Param {#enum-Param}

```gdscript
enum Param { PARAM_SWING_SPAN = 0, PARAM_TWIST_SPAN = 1, PARAM_BIAS = 2, PARAM_SOFTNESS = 3, PARAM_RELAXATION = 4, PARAM_MAX = 5 }
```

Param PARAM_SWING_SPAN = 0

Swing is rotation from side to side, around the axis perpendicular to the twist axis.

The swing span defines, how much rotation will not get corrected along the swing axis.

Could be defined as looseness in the ConeTwistJoint3D.

If below 0.05, this behavior is locked.

Param PARAM_TWIST_SPAN = 1

Twist is the rotation around the twist axis, this value defined how far the joint can twist.

Twist is locked if below 0.05.

Param PARAM_BIAS = 2

The speed with which the swing or twist will take place.

The higher, the faster.

Param PARAM_SOFTNESS = 3

The ease with which the joint starts to twist. If it's too low, it takes more force to start twisting the joint.

Param PARAM_RELAXATION = 4

Defines, how fast the swing- and twist-speed-difference on both sides gets synced.

Param PARAM_MAX = 5

Represents the size of the Param enum.

#### PARAM_SWING_SPAN {#PARAM_SWING_SPAN}

```gdscript
PARAM_SWING_SPAN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_TWIST_SPAN {#PARAM_TWIST_SPAN}

```gdscript
PARAM_TWIST_SPAN = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_BIAS {#PARAM_BIAS}

```gdscript
PARAM_BIAS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_SOFTNESS {#PARAM_SOFTNESS}

```gdscript
PARAM_SOFTNESS = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_RELAXATION {#PARAM_RELAXATION}

```gdscript
PARAM_RELAXATION = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_MAX {#PARAM_MAX}

```gdscript
PARAM_MAX = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
