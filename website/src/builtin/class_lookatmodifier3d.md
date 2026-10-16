# LookAtModifier3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **LookAtModifier3D**

> This SkeletonModifier3D rotates a bone to look at a target. This is helpful for moving a character's head to look at the player, rotating a turret to look at a target, or any other case where you want to make a bone rotate towards something quickly and easily.

<details><summary>官方英文描述（中文精译进行中）</summary>

This SkeletonModifier3D rotates a bone to look at a target. This is helpful for moving a character's head to look at the player, rotating a turret to look at a target, or any other case where you want to make a bone rotate towards something quickly and easily.

When applying multiple LookAtModifier3Ds, the LookAtModifier3D assigned to the parent bone must be put above the LookAtModifier3D assigned to the child bone in the list in order for the child bone results to be correct.

</details>

## 方法

### get_bone {#get_bone}

```gdscript
int get_bone() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_name {#get_bone_name}

```gdscript
String get_bone_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_duration {#get_duration}

```gdscript
float get_duration() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ease_type {#get_ease_type}

```gdscript
Tween.EaseType get_ease_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_forward_axis {#get_forward_axis}

```gdscript
SkeletonModifier3D.BoneAxis get_forward_axis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_interpolation_remaining {#get_interpolation_remaining}

```gdscript
float get_interpolation_remaining() const
```

Returns the remaining seconds of the time-based interpolation.

### get_origin_bone {#get_origin_bone}

```gdscript
int get_origin_bone() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_origin_bone_name {#get_origin_bone_name}

```gdscript
String get_origin_bone_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_origin_external_node {#get_origin_external_node}

```gdscript
NodePath get_origin_external_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_origin_from {#get_origin_from}

```gdscript
LookAtModifier3D.OriginFrom get_origin_from() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_origin_offset {#get_origin_offset}

```gdscript
Vector3 get_origin_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_origin_safe_margin {#get_origin_safe_margin}

```gdscript
float get_origin_safe_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_primary_damp_threshold {#get_primary_damp_threshold}

```gdscript
float get_primary_damp_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_primary_limit_angle {#get_primary_limit_angle}

```gdscript
float get_primary_limit_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_primary_negative_damp_threshold {#get_primary_negative_damp_threshold}

```gdscript
float get_primary_negative_damp_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_primary_negative_limit_angle {#get_primary_negative_limit_angle}

```gdscript
float get_primary_negative_limit_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_primary_positive_damp_threshold {#get_primary_positive_damp_threshold}

```gdscript
float get_primary_positive_damp_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_primary_positive_limit_angle {#get_primary_positive_limit_angle}

```gdscript
float get_primary_positive_limit_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_primary_rotation_axis {#get_primary_rotation_axis}

```gdscript
Vector3.Axis get_primary_rotation_axis() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_secondary_damp_threshold {#get_secondary_damp_threshold}

```gdscript
float get_secondary_damp_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_secondary_limit_angle {#get_secondary_limit_angle}

```gdscript
float get_secondary_limit_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_secondary_negative_damp_threshold {#get_secondary_negative_damp_threshold}

```gdscript
float get_secondary_negative_damp_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_secondary_negative_limit_angle {#get_secondary_negative_limit_angle}

```gdscript
float get_secondary_negative_limit_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_secondary_positive_damp_threshold {#get_secondary_positive_damp_threshold}

```gdscript
float get_secondary_positive_damp_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_secondary_positive_limit_angle {#get_secondary_positive_limit_angle}

```gdscript
float get_secondary_positive_limit_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transition_type {#get_transition_type}

```gdscript
Tween.TransitionType get_transition_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_interpolating {#is_interpolating}

```gdscript
bool is_interpolating() const
```

Returns true if time-based interpolation is running. If true, it is equivalent to get_interpolation_remaining() returning 0.0.

This is useful to determine whether a LookAtModifier3D can be removed safely.

### is_limitation_symmetry {#is_limitation_symmetry}

```gdscript
bool is_limitation_symmetry() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_relative {#is_relative}

```gdscript
bool is_relative() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_target_within_limitation {#is_target_within_limitation}

```gdscript
bool is_target_within_limitation() const
```

Returns whether the target is within the angle limitations. It is useful for unsetting the target_node when the target is outside of the angle limitations.

Note: The value is updated after SkeletonModifier3D._process_modification(). To retrieve this value correctly, we recommend using the signal SkeletonModifier3D.modification_processed.

### is_using_angle_limitation {#is_using_angle_limitation}

```gdscript
bool is_using_angle_limitation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_secondary_rotation {#is_using_secondary_rotation}

```gdscript
bool is_using_secondary_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone {#set_bone}

```gdscript
void set_bone(bone: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_name {#set_bone_name}

```gdscript
void set_bone_name(bone_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_duration {#set_duration}

```gdscript
void set_duration(duration: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ease_type {#set_ease_type}

```gdscript
void set_ease_type(ease_type: Tween.EaseType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_forward_axis {#set_forward_axis}

```gdscript
void set_forward_axis(forward_axis: SkeletonModifier3D.BoneAxis)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_origin_bone {#set_origin_bone}

```gdscript
void set_origin_bone(bone: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_origin_bone_name {#set_origin_bone_name}

```gdscript
void set_origin_bone_name(bone_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_origin_external_node {#set_origin_external_node}

```gdscript
void set_origin_external_node(external_node: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_origin_from {#set_origin_from}

```gdscript
void set_origin_from(origin_from: LookAtModifier3D.OriginFrom)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_origin_offset {#set_origin_offset}

```gdscript
void set_origin_offset(offset: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_origin_safe_margin {#set_origin_safe_margin}

```gdscript
void set_origin_safe_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_damp_threshold {#set_primary_damp_threshold}

```gdscript
void set_primary_damp_threshold(power: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_limit_angle {#set_primary_limit_angle}

```gdscript
void set_primary_limit_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_negative_damp_threshold {#set_primary_negative_damp_threshold}

```gdscript
void set_primary_negative_damp_threshold(power: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_negative_limit_angle {#set_primary_negative_limit_angle}

```gdscript
void set_primary_negative_limit_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_positive_damp_threshold {#set_primary_positive_damp_threshold}

```gdscript
void set_primary_positive_damp_threshold(power: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_positive_limit_angle {#set_primary_positive_limit_angle}

```gdscript
void set_primary_positive_limit_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_rotation_axis {#set_primary_rotation_axis}

```gdscript
void set_primary_rotation_axis(axis: Vector3.Axis)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_relative {#set_relative}

```gdscript
void set_relative(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secondary_damp_threshold {#set_secondary_damp_threshold}

```gdscript
void set_secondary_damp_threshold(power: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secondary_limit_angle {#set_secondary_limit_angle}

```gdscript
void set_secondary_limit_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secondary_negative_damp_threshold {#set_secondary_negative_damp_threshold}

```gdscript
void set_secondary_negative_damp_threshold(power: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secondary_negative_limit_angle {#set_secondary_negative_limit_angle}

```gdscript
void set_secondary_negative_limit_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secondary_positive_damp_threshold {#set_secondary_positive_damp_threshold}

```gdscript
void set_secondary_positive_damp_threshold(power: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secondary_positive_limit_angle {#set_secondary_positive_limit_angle}

```gdscript
void set_secondary_positive_limit_angle(angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_symmetry_limitation {#set_symmetry_limitation}

```gdscript
void set_symmetry_limitation(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_node {#set_target_node}

```gdscript
void set_target_node(target_node: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transition_type {#set_transition_type}

```gdscript
void set_transition_type(transition_type: Tween.TransitionType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_angle_limitation {#set_use_angle_limitation}

```gdscript
void set_use_angle_limitation(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_secondary_rotation {#set_use_secondary_rotation}

```gdscript
void set_use_secondary_rotation(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### target_node {#target_node}

```gdscript
target_node: NodePath
```

- void set_target_node(value: NodePath)
- NodePath get_target_node()

The NodePath to the node that is the target for the look at modification. This node is what the modification will rotate the bone to.

### bone_name {#bone_name}

```gdscript
bone_name: String
```

- void set_bone_name(value: String)
- String get_bone_name()

The bone name of the Skeleton3D that the modification will operate on.

### bone {#bone}

```gdscript
bone: int
```

- void set_bone(value: int)
- int get_bone()

Index of the bone_name in the parent Skeleton3D.

### forward_axis {#forward_axis}

```gdscript
forward_axis: int
```

- void set_forward_axis(value: BoneAxis)
- BoneAxis get_forward_axis()

The forward axis of the bone. This SkeletonModifier3D modifies the bone so that this axis points toward the target_node.

### primary_rotation_axis {#primary_rotation_axis}

```gdscript
primary_rotation_axis: int
```

- void set_primary_rotation_axis(value: Axis)
- Axis get_primary_rotation_axis()

The axis of the first rotation. This SkeletonModifier3D works by compositing the rotation by Euler angles to prevent to rotate the forward_axis.

### use_secondary_rotation {#use_secondary_rotation}

```gdscript
use_secondary_rotation: bool
```

- void set_use_secondary_rotation(value: bool)
- bool is_using_secondary_rotation()

If true, provides rotation by two axes.

### relative {#relative}

```gdscript
relative: bool
```

- void set_relative(value: bool)
- bool is_relative()

The relative option. If true, the rotation is applied relative to the pose. If false, the rotation is applied relative to the rest. It means to replace the current pose with the LookAtModifier3D's result.

Note: This option affects the base angle for use_angle_limitation. Since the LookAtModifier3D relies strongly on Euler rotation, the axis that determines the limitation and the actual rotation are strongly tied together.

### origin_from {#origin_from}

```gdscript
origin_from: int
```

- void set_origin_from(value: OriginFrom)
- OriginFrom get_origin_from()

This value determines from what origin is retrieved for use in the calculation of the forward vector.

### origin_bone_name {#origin_bone_name}

```gdscript
origin_bone_name: String
```

- void set_origin_bone_name(value: String)
- String get_origin_bone_name()

If origin_from is ORIGIN_FROM_SPECIFIC_BONE, the bone global pose position specified for this is used as origin.

### origin_bone {#origin_bone}

```gdscript
origin_bone: int
```

- void set_origin_bone(value: int)
- int get_origin_bone()

Index of the origin_bone_name in the parent Skeleton3D.

### origin_external_node {#origin_external_node}

```gdscript
origin_external_node: NodePath
```

- void set_origin_external_node(value: NodePath)
- NodePath get_origin_external_node()

If origin_from is ORIGIN_FROM_EXTERNAL_NODE, the global position of the Node3D specified for this is used as origin.

### origin_offset {#origin_offset}

```gdscript
origin_offset: Vector3
```

- void set_origin_offset(value: Vector3)
- Vector3 get_origin_offset()

The offset of the bone pose origin. Matching the origins by offset is useful for cases where multiple bones must always face the same direction, such as the eyes.

Note: This value indicates the local position of the object set in origin_from.

### origin_safe_margin {#origin_safe_margin}

```gdscript
origin_safe_margin: float
```

- void set_origin_safe_margin(value: float)
- float get_origin_safe_margin()

If the target passes through too close to the origin than this value, time-based interpolation is used even if the target is within the angular limitations, to prevent the angular velocity from becoming too high.

### duration {#duration}

```gdscript
duration: float
```

- void set_duration(value: float)
- float get_duration()

The duration of the time-based interpolation. Interpolation is triggered at the following cases:

- When the target node is changed
- When an axis is flipped due to angle limitation

Note: The flipping occurs when the target is outside the angle limitation and the internally computed secondary rotation axis of the forward vector is flipped. Visually, it occurs when the target is outside the angle limitation and crosses the plane of the forward_axis and primary_rotation_axis.

### transition_type {#transition_type}

```gdscript
transition_type: int
```

- void set_transition_type(value: TransitionType)
- TransitionType get_transition_type()

The transition type of the time-based interpolation. See also TransitionType.

### ease_type {#ease_type}

```gdscript
ease_type: int
```

- void set_ease_type(value: EaseType)
- EaseType get_ease_type()

The ease type of the time-based interpolation. See also EaseType.

### use_angle_limitation {#use_angle_limitation}

```gdscript
use_angle_limitation: bool
```

- void set_use_angle_limitation(value: bool)
- bool is_using_angle_limitation()

If true, limits the amount of rotation. For example, this helps to prevent a character's neck from rotating 360 degrees.

Note: As with AnimationTree blending, interpolation is provided that favors Skeleton3D.get_bone_rest() or Skeleton3D.get_bone_pose() depends on the relative option. This means that interpolation does not select the shortest path in some cases.

Note: Some values for transition_type (such as Tween.TRANS_BACK, Tween.TRANS_ELASTIC, and Tween.TRANS_SPRING) may exceed the limitations. If interpolation occurs while overshooting the limitations, the result might not respect the bone rest.

### symmetry_limitation {#symmetry_limitation}

```gdscript
symmetry_limitation: bool
```

- void set_symmetry_limitation(value: bool)
- bool is_limitation_symmetry()

If true, the limitations are spread from the bone symmetrically.

If false, the limitation can be specified separately for each side of the bone rest.

### primary_limit_angle {#primary_limit_angle}

```gdscript
primary_limit_angle: float
```

- void set_primary_limit_angle(value: float)
- float get_primary_limit_angle()

The limit angle of the primary rotation when symmetry_limitation is true, in radians.

### primary_damp_threshold {#primary_damp_threshold}

```gdscript
primary_damp_threshold: float
```

- void set_primary_damp_threshold(value: float)
- float get_primary_damp_threshold()

The threshold to start damping for primary_limit_angle. It provides non-linear (b-spline) interpolation, let it feel more resistance the more it rotate to the edge limit. This is useful for simulating the limits of human motion.

If 1.0, no damping is performed. If 0.0, damping is always performed.

### primary_positive_limit_angle {#primary_positive_limit_angle}

```gdscript
primary_positive_limit_angle: float
```

- void set_primary_positive_limit_angle(value: float)
- float get_primary_positive_limit_angle()

The limit angle of positive side of the primary rotation when symmetry_limitation is false, in radians.

### primary_positive_damp_threshold {#primary_positive_damp_threshold}

```gdscript
primary_positive_damp_threshold: float
```

- void set_primary_positive_damp_threshold(value: float)
- float get_primary_positive_damp_threshold()

The threshold to start damping for primary_positive_limit_angle.

### primary_negative_limit_angle {#primary_negative_limit_angle}

```gdscript
primary_negative_limit_angle: float
```

- void set_primary_negative_limit_angle(value: float)
- float get_primary_negative_limit_angle()

The limit angle of negative side of the primary rotation when symmetry_limitation is false, in radians.

### primary_negative_damp_threshold {#primary_negative_damp_threshold}

```gdscript
primary_negative_damp_threshold: float
```

- void set_primary_negative_damp_threshold(value: float)
- float get_primary_negative_damp_threshold()

The threshold to start damping for primary_negative_limit_angle.

### secondary_limit_angle {#secondary_limit_angle}

```gdscript
secondary_limit_angle: float
```

- void set_secondary_limit_angle(value: float)
- float get_secondary_limit_angle()

The limit angle of the secondary rotation when symmetry_limitation is true, in radians.

### secondary_damp_threshold {#secondary_damp_threshold}

```gdscript
secondary_damp_threshold: float
```

- void set_secondary_damp_threshold(value: float)
- float get_secondary_damp_threshold()

The threshold to start damping for secondary_limit_angle.

### secondary_positive_limit_angle {#secondary_positive_limit_angle}

```gdscript
secondary_positive_limit_angle: float
```

- void set_secondary_positive_limit_angle(value: float)
- float get_secondary_positive_limit_angle()

The limit angle of positive side of the secondary rotation when symmetry_limitation is false, in radians.

### secondary_positive_damp_threshold {#secondary_positive_damp_threshold}

```gdscript
secondary_positive_damp_threshold: float
```

- void set_secondary_positive_damp_threshold(value: float)
- float get_secondary_positive_damp_threshold()

The threshold to start damping for secondary_positive_limit_angle.

### secondary_negative_limit_angle {#secondary_negative_limit_angle}

```gdscript
secondary_negative_limit_angle: float
```

- void set_secondary_negative_limit_angle(value: float)
- float get_secondary_negative_limit_angle()

The limit angle of negative side of the secondary rotation when symmetry_limitation is false, in radians.

### secondary_negative_damp_threshold {#secondary_negative_damp_threshold}

```gdscript
secondary_negative_damp_threshold: float
```

- void set_secondary_negative_damp_threshold(value: float)
- float get_secondary_negative_damp_threshold()

The threshold to start damping for secondary_negative_limit_angle.

## 枚举

### OriginFrom {#enum-OriginFrom}

```gdscript
enum OriginFrom { ORIGIN_FROM_SELF = 0, ORIGIN_FROM_SPECIFIC_BONE = 1, ORIGIN_FROM_EXTERNAL_NODE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ORIGIN_FROM_SELF {#ORIGIN_FROM_SELF}

```gdscript
ORIGIN_FROM_SELF = 0
```

The bone rest position of the bone specified in bone is used as origin.

#### ORIGIN_FROM_SPECIFIC_BONE {#ORIGIN_FROM_SPECIFIC_BONE}

```gdscript
ORIGIN_FROM_SPECIFIC_BONE = 1
```

The bone global pose position of the bone specified in origin_bone is used as origin.

Note: It is recommended that you select only the parent bone unless you are familiar with the bone processing process. The specified bone pose at the time the LookAtModifier3D is processed is used as a reference. In other words, if you specify a child bone and the LookAtModifier3D causes the child bone to move, the rendered result and direction will not match.

#### ORIGIN_FROM_EXTERNAL_NODE {#ORIGIN_FROM_EXTERNAL_NODE}

```gdscript
ORIGIN_FROM_EXTERNAL_NODE = 2
```

The global position of the Node3D specified in origin_external_node is used as origin.

Note: Same as ORIGIN_FROM_SPECIFIC_BONE, when specifying a BoneAttachment3D with a child bone assigned, the rendered result and direction will not match.

---
