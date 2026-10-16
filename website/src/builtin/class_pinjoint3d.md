# PinJoint3D

**继承**：[Joint3D](class_joint3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **PinJoint3D**

> A physics joint that attaches two 3D physics bodies at a single point, allowing them to freely rotate. For example, a RigidBody3D can be attached to a StaticBody3D to create a pendulum or a seesaw.

<details><summary>官方英文描述</summary>

A physics joint that attaches two 3D physics bodies at a single point, allowing them to freely rotate. For example, a RigidBody3D can be attached to a StaticBody3D to create a pendulum or a seesaw.

</details>

## 方法

### get_param {#get_param}

```gdscript
float get_param(param: PinJoint3D.Param) const
```

Returns the value of the specified parameter.

### set_param {#set_param}

```gdscript
void set_param(param: PinJoint3D.Param, value: float)
```

Sets the value of the specified parameter.

## 枚举

### Param {#enum-Param}

```gdscript
enum Param { PARAM_BIAS = 0, PARAM_DAMPING = 1, PARAM_IMPULSE_CLAMP = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARAM_BIAS {#PARAM_BIAS}

```gdscript
PARAM_BIAS = 0
```

The force with which the pinned objects stay in positional relation to each other. The higher, the stronger.

#### PARAM_DAMPING {#PARAM_DAMPING}

```gdscript
PARAM_DAMPING = 1
```

The force with which the pinned objects stay in velocity relation to each other. The higher, the stronger.

#### PARAM_IMPULSE_CLAMP {#PARAM_IMPULSE_CLAMP}

```gdscript
PARAM_IMPULSE_CLAMP = 2
```

If above 0, this value is the maximum value for an impulse that this Joint3D produces.

---
