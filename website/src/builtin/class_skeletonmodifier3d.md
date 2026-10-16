# SkeletonModifier3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SkeletonModifier3D**

> SkeletonModifier3D retrieves a target Skeleton3D by having a Skeleton3D parent.

<details><summary>官方英文描述（中文精译进行中）</summary>

SkeletonModifier3D retrieves a target Skeleton3D by having a Skeleton3D parent.

If there is an AnimationMixer, a modification always performs after playback process of the AnimationMixer.

This node should be used to implement custom IK solvers, constraints, or skeleton physics.

</details>

## 方法

### _process_modification {#_process_modification}

```gdscript
void _process_modification() virtual
```

Deprecated: Use _process_modification_with_delta() instead.

Override this virtual method to implement a custom skeleton modifier. You should do things like get the Skeleton3D's current pose and apply the pose here.

_process_modification() must not apply influence to bone poses because the Skeleton3D automatically applies influence to all bone poses set by the modifier.

### _process_modification_with_delta {#_process_modification_with_delta}

```gdscript
void _process_modification_with_delta(delta: float) virtual
```

Override this virtual method to implement a custom skeleton modifier. You should do things like get the Skeleton3D's current pose and apply the pose here.

_process_modification_with_delta() must not apply influence to bone poses because the Skeleton3D automatically applies influence to all bone poses set by the modifier.

delta is passed from parent Skeleton3D. See also Skeleton3D.advance().

Note: This method may be called outside Node._process() and Node._physics_process() with delta is 0.0, since the modification should be processed immediately after initialization of the Skeleton3D.

### _skeleton_changed {#_skeleton_changed}

```gdscript
void _skeleton_changed(old_skeleton: Skeleton3D, new_skeleton: Skeleton3D) virtual
```

Called when the skeleton is changed.

### _validate_bone_names {#_validate_bone_names}

```gdscript
void _validate_bone_names() virtual
```

Called when bone names and indices need to be validated, such as when entering the scene tree or changing skeleton.

### get_influence {#get_influence}

```gdscript
float get_influence() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton {#get_skeleton}

```gdscript
Skeleton3D get_skeleton() const
```

Returns the parent Skeleton3D node if it exists. Otherwise, returns null.

### is_active {#is_active}

```gdscript
bool is_active() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_active {#set_active}

```gdscript
void set_active(active: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_influence {#set_influence}

```gdscript
void set_influence(influence: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### active {#active}

```gdscript
active: bool
```

- void set_active(value: bool)
- bool is_active()

If true, the SkeletonModifier3D will be processing.

### influence {#influence}

```gdscript
influence: float
```

- void set_influence(value: float)
- float get_influence()

Sets the influence of the modification.

Note: This value is used by Skeleton3D to blend, so the SkeletonModifier3D should always apply only 100% of the result without interpolation.

## 信号

### modification_processed {#modification_processed}

```gdscript
signal modification_processed()
```

Notifies when the modification have been finished.

Note: If you want to get the modified bone pose by the modifier, you must use Skeleton3D.get_bone_pose() or Skeleton3D.get_bone_global_pose() at the moment this signal is fired.

## 枚举

### BoneAxis {#enum-BoneAxis}

```gdscript
enum BoneAxis { BONE_AXIS_PLUS_X = 0, BONE_AXIS_MINUS_X = 1, BONE_AXIS_PLUS_Y = 2, BONE_AXIS_MINUS_Y = 3, BONE_AXIS_PLUS_Z = 4, BONE_AXIS_MINUS_Z = 5 }
```

BoneAxis BONE_AXIS_PLUS_X = 0

Enumerated value for the +X axis.

BoneAxis BONE_AXIS_MINUS_X = 1

Enumerated value for the -X axis.

BoneAxis BONE_AXIS_PLUS_Y = 2

Enumerated value for the +Y axis.

BoneAxis BONE_AXIS_MINUS_Y = 3

Enumerated value for the -Y axis.

BoneAxis BONE_AXIS_PLUS_Z = 4

Enumerated value for the +Z axis.

BoneAxis BONE_AXIS_MINUS_Z = 5

Enumerated value for the -Z axis.

#### BONE_AXIS_PLUS_X {#BONE_AXIS_PLUS_X}

```gdscript
BONE_AXIS_PLUS_X = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_AXIS_MINUS_X {#BONE_AXIS_MINUS_X}

```gdscript
BONE_AXIS_MINUS_X = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_AXIS_PLUS_Y {#BONE_AXIS_PLUS_Y}

```gdscript
BONE_AXIS_PLUS_Y = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_AXIS_MINUS_Y {#BONE_AXIS_MINUS_Y}

```gdscript
BONE_AXIS_MINUS_Y = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_AXIS_PLUS_Z {#BONE_AXIS_PLUS_Z}

```gdscript
BONE_AXIS_PLUS_Z = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_AXIS_MINUS_Z {#BONE_AXIS_MINUS_Z}

```gdscript
BONE_AXIS_MINUS_Z = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### BoneDirection {#enum-BoneDirection}

```gdscript
enum BoneDirection { BONE_DIRECTION_PLUS_X = 0, BONE_DIRECTION_MINUS_X = 1, BONE_DIRECTION_PLUS_Y = 2, BONE_DIRECTION_MINUS_Y = 3, BONE_DIRECTION_PLUS_Z = 4, BONE_DIRECTION_MINUS_Z = 5, BONE_DIRECTION_FROM_PARENT = 6 }
```

BoneDirection BONE_DIRECTION_PLUS_X = 0

Enumerated value for the +X axis.

BoneDirection BONE_DIRECTION_MINUS_X = 1

Enumerated value for the -X axis.

BoneDirection BONE_DIRECTION_PLUS_Y = 2

Enumerated value for the +Y axis.

BoneDirection BONE_DIRECTION_MINUS_Y = 3

Enumerated value for the -Y axis.

BoneDirection BONE_DIRECTION_PLUS_Z = 4

Enumerated value for the +Z axis.

BoneDirection BONE_DIRECTION_MINUS_Z = 5

Enumerated value for the -Z axis.

BoneDirection BONE_DIRECTION_FROM_PARENT = 6

Enumerated value for the axis from a parent bone to the child bone.

#### BONE_DIRECTION_PLUS_X {#BONE_DIRECTION_PLUS_X}

```gdscript
BONE_DIRECTION_PLUS_X = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_DIRECTION_MINUS_X {#BONE_DIRECTION_MINUS_X}

```gdscript
BONE_DIRECTION_MINUS_X = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_DIRECTION_PLUS_Y {#BONE_DIRECTION_PLUS_Y}

```gdscript
BONE_DIRECTION_PLUS_Y = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_DIRECTION_MINUS_Y {#BONE_DIRECTION_MINUS_Y}

```gdscript
BONE_DIRECTION_MINUS_Y = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_DIRECTION_PLUS_Z {#BONE_DIRECTION_PLUS_Z}

```gdscript
BONE_DIRECTION_PLUS_Z = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_DIRECTION_MINUS_Z {#BONE_DIRECTION_MINUS_Z}

```gdscript
BONE_DIRECTION_MINUS_Z = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_DIRECTION_FROM_PARENT {#BONE_DIRECTION_FROM_PARENT}

```gdscript
BONE_DIRECTION_FROM_PARENT = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### SecondaryDirection {#enum-SecondaryDirection}

```gdscript
enum SecondaryDirection { SECONDARY_DIRECTION_NONE = 0, SECONDARY_DIRECTION_PLUS_X = 1, SECONDARY_DIRECTION_MINUS_X = 2, SECONDARY_DIRECTION_PLUS_Y = 3, SECONDARY_DIRECTION_MINUS_Y = 4, SECONDARY_DIRECTION_PLUS_Z = 5, SECONDARY_DIRECTION_MINUS_Z = 6, SECONDARY_DIRECTION_CUSTOM = 7 }
```

SecondaryDirection SECONDARY_DIRECTION_NONE = 0

Enumerated value for the case when the axis is undefined.

SecondaryDirection SECONDARY_DIRECTION_PLUS_X = 1

Enumerated value for the +X axis.

SecondaryDirection SECONDARY_DIRECTION_MINUS_X = 2

Enumerated value for the -X axis.

SecondaryDirection SECONDARY_DIRECTION_PLUS_Y = 3

Enumerated value for the +Y axis.

SecondaryDirection SECONDARY_DIRECTION_MINUS_Y = 4

Enumerated value for the -Y axis.

SecondaryDirection SECONDARY_DIRECTION_PLUS_Z = 5

Enumerated value for the +Z axis.

SecondaryDirection SECONDARY_DIRECTION_MINUS_Z = 6

Enumerated value for the -Z axis.

SecondaryDirection SECONDARY_DIRECTION_CUSTOM = 7

Enumerated value for an optional axis.

#### SECONDARY_DIRECTION_NONE {#SECONDARY_DIRECTION_NONE}

```gdscript
SECONDARY_DIRECTION_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SECONDARY_DIRECTION_PLUS_X {#SECONDARY_DIRECTION_PLUS_X}

```gdscript
SECONDARY_DIRECTION_PLUS_X = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SECONDARY_DIRECTION_MINUS_X {#SECONDARY_DIRECTION_MINUS_X}

```gdscript
SECONDARY_DIRECTION_MINUS_X = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SECONDARY_DIRECTION_PLUS_Y {#SECONDARY_DIRECTION_PLUS_Y}

```gdscript
SECONDARY_DIRECTION_PLUS_Y = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SECONDARY_DIRECTION_MINUS_Y {#SECONDARY_DIRECTION_MINUS_Y}

```gdscript
SECONDARY_DIRECTION_MINUS_Y = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SECONDARY_DIRECTION_PLUS_Z {#SECONDARY_DIRECTION_PLUS_Z}

```gdscript
SECONDARY_DIRECTION_PLUS_Z = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SECONDARY_DIRECTION_MINUS_Z {#SECONDARY_DIRECTION_MINUS_Z}

```gdscript
SECONDARY_DIRECTION_MINUS_Z = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SECONDARY_DIRECTION_CUSTOM {#SECONDARY_DIRECTION_CUSTOM}

```gdscript
SECONDARY_DIRECTION_CUSTOM = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### RotationAxis {#enum-RotationAxis}

```gdscript
enum RotationAxis { ROTATION_AXIS_X = 0, ROTATION_AXIS_Y = 1, ROTATION_AXIS_Z = 2, ROTATION_AXIS_ALL = 3, ROTATION_AXIS_CUSTOM = 4 }
```

RotationAxis ROTATION_AXIS_X = 0

Enumerated value for the rotation of the X axis.

RotationAxis ROTATION_AXIS_Y = 1

Enumerated value for the rotation of the Y axis.

RotationAxis ROTATION_AXIS_Z = 2

Enumerated value for the rotation of the Z axis.

RotationAxis ROTATION_AXIS_ALL = 3

Enumerated value for the unconstrained rotation.

RotationAxis ROTATION_AXIS_CUSTOM = 4

Enumerated value for an optional rotation axis.

#### ROTATION_AXIS_X {#ROTATION_AXIS_X}

```gdscript
ROTATION_AXIS_X = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROTATION_AXIS_Y {#ROTATION_AXIS_Y}

```gdscript
ROTATION_AXIS_Y = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROTATION_AXIS_Z {#ROTATION_AXIS_Z}

```gdscript
ROTATION_AXIS_Z = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROTATION_AXIS_ALL {#ROTATION_AXIS_ALL}

```gdscript
ROTATION_AXIS_ALL = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ROTATION_AXIS_CUSTOM {#ROTATION_AXIS_CUSTOM}

```gdscript
ROTATION_AXIS_CUSTOM = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
