# BoneTwistDisperser3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **BoneTwistDisperser3D**

> This BoneTwistDisperser3D allows for smooth twist interpolation between multiple bones by dispersing the end bone's twist to the parents. This only changes the twist without changing the global position of each joint.

<details><summary>官方英文描述（中文精译进行中）</summary>

This BoneTwistDisperser3D allows for smooth twist interpolation between multiple bones by dispersing the end bone's twist to the parents. This only changes the twist without changing the global position of each joint.

This is useful for smoothly twisting bones in combination with CopyTransformModifier3D and IK.

Note: If an extracted twist is greater than 180 degrees, flipping occurs. This is similar to ConvertTransformModifier3D.

Note: Most methods in this class take an index parameter. This parameter specifies which setting list entry to return if the IK has multiple entries (e.g. settings//root_bone_name).

</details>

## 方法

### are_bone_axes_mutable {#are_bone_axes_mutable}

```gdscript
bool are_bone_axes_mutable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### clear_settings {#clear_settings}

```gdscript
void clear_settings()
```

Clears all settings.

### get_damping_curve {#get_damping_curve}

```gdscript
Curve get_damping_curve(index: int) const
```

Returns the damping curve when get_disperse_mode() is DISPERSE_MODE_CUSTOM.

### get_disperse_mode {#get_disperse_mode}

```gdscript
BoneTwistDisperser3D.DisperseMode get_disperse_mode(index: int) const
```

Returns whether to use automatic amount assignment or to allow manual assignment.

### get_end_bone {#get_end_bone}

```gdscript
int get_end_bone(index: int) const
```

Returns the end bone index of the bone chain.

### get_end_bone_direction {#get_end_bone_direction}

```gdscript
SkeletonModifier3D.BoneDirection get_end_bone_direction(index: int) const
```

Returns the tail direction of the end bone of the bone chain when is_end_bone_extended() is true.

### get_end_bone_name {#get_end_bone_name}

```gdscript
String get_end_bone_name(index: int) const
```

Returns the end bone name of the bone chain.

### get_joint_bone {#get_joint_bone}

```gdscript
int get_joint_bone(index: int, joint: int) const
```

Returns the bone index at joint in the bone chain's joint list.

### get_joint_bone_name {#get_joint_bone_name}

```gdscript
String get_joint_bone_name(index: int, joint: int) const
```

Returns the bone name at joint in the bone chain's joint list.

### get_joint_count {#get_joint_count}

```gdscript
int get_joint_count(index: int) const
```

Returns the joint count of the bone chain's joint list.

### get_joint_twist_amount {#get_joint_twist_amount}

```gdscript
float get_joint_twist_amount(index: int, joint: int) const
```

Returns the twist amount at joint in the bone chain's joint list when get_disperse_mode() is DISPERSE_MODE_CUSTOM.

### get_reference_bone {#get_reference_bone}

```gdscript
int get_reference_bone(index: int) const
```

Returns the reference bone to extract twist of the setting at index.

This bone is either the end of the chain or its parent, depending on is_end_bone_extended().

### get_reference_bone_name {#get_reference_bone_name}

```gdscript
String get_reference_bone_name(index: int) const
```

Returns the reference bone name to extract twist of the setting at index.

This bone is either the end of the chain or its parent, depending on is_end_bone_extended().

### get_root_bone {#get_root_bone}

```gdscript
int get_root_bone(index: int) const
```

Returns the root bone index of the bone chain.

### get_root_bone_name {#get_root_bone_name}

```gdscript
String get_root_bone_name(index: int) const
```

Returns the root bone name of the bone chain.

### get_setting_count {#get_setting_count}

```gdscript
int get_setting_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_twist_from {#get_twist_from}

```gdscript
Quaternion get_twist_from(index: int) const
```

Returns the rotation to an arbitrary state before twisting for the current bone pose to extract the twist when is_twist_from_rest() is false.

### get_weight_position {#get_weight_position}

```gdscript
float get_weight_position(index: int) const
```

Returns the position at which to divide the segment between joints for weight assignment when get_disperse_mode() is DISPERSE_MODE_WEIGHTED.

### is_end_bone_extended {#is_end_bone_extended}

```gdscript
bool is_end_bone_extended(index: int) const
```

Returns true if the end bone is extended to have a tail.

### is_twist_from_rest {#is_twist_from_rest}

```gdscript
bool is_twist_from_rest(index: int) const
```

Returns true if extracting the twist amount from the difference between the bone rest and the current bone pose.

### set_damping_curve {#set_damping_curve}

```gdscript
void set_damping_curve(index: int, curve: Curve)
```

Sets the damping curve when get_disperse_mode() is DISPERSE_MODE_CUSTOM.

### set_disperse_mode {#set_disperse_mode}

```gdscript
void set_disperse_mode(index: int, disperse_mode: BoneTwistDisperser3D.DisperseMode)
```

Sets whether to use automatic amount assignment or to allow manual assignment.

### set_end_bone {#set_end_bone}

```gdscript
void set_end_bone(index: int, bone: int)
```

Sets the end bone index of the bone chain.

### set_end_bone_direction {#set_end_bone_direction}

```gdscript
void set_end_bone_direction(index: int, bone_direction: SkeletonModifier3D.BoneDirection)
```

Sets the end bone tail direction of the bone chain when is_end_bone_extended() is true.

### set_end_bone_name {#set_end_bone_name}

```gdscript
void set_end_bone_name(index: int, bone_name: String)
```

Sets the end bone name of the bone chain.

Note: The end bone must be a child of the root bone.

### set_extend_end_bone {#set_extend_end_bone}

```gdscript
void set_extend_end_bone(index: int, enabled: bool)
```

If enabled is true, the end bone is extended to have a tail.

If enabled is false, get_reference_bone() becomes a parent of the end bone and it uses the vector to the end bone as a twist axis.

### set_joint_twist_amount {#set_joint_twist_amount}

```gdscript
void set_joint_twist_amount(index: int, joint: int, twist_amount: float)
```

Sets the twist amount at joint in the bone chain's joint list when get_disperse_mode() is DISPERSE_MODE_CUSTOM.

### set_mutable_bone_axes {#set_mutable_bone_axes}

```gdscript
void set_mutable_bone_axes(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_root_bone {#set_root_bone}

```gdscript
void set_root_bone(index: int, bone: int)
```

Sets the root bone index of the bone chain.

### set_root_bone_name {#set_root_bone_name}

```gdscript
void set_root_bone_name(index: int, bone_name: String)
```

Sets the root bone name of the bone chain.

### set_setting_count {#set_setting_count}

```gdscript
void set_setting_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_twist_from {#set_twist_from}

```gdscript
void set_twist_from(index: int, from: Quaternion)
```

Sets the rotation to an arbitrary state before twisting for the current bone pose to extract the twist when is_twist_from_rest() is false.

In other words, by calling set_twist_from() by SkeletonModifier3D.modification_processed of a specific SkeletonModifier3D, you can extract only the twists generated by modifiers processed after that but before this BoneTwistDisperser3D.

### set_twist_from_rest {#set_twist_from_rest}

```gdscript
void set_twist_from_rest(index: int, enabled: bool)
```

If enabled is true, it extracts the twist amount from the difference between the bone rest and the current bone pose.

If enabled is false, it extracts the twist amount from the difference between get_twist_from() and the current bone pose. See also set_twist_from().

### set_weight_position {#set_weight_position}

```gdscript
void set_weight_position(index: int, weight_position: float)
```

Sets the position at which to divide the segment between joints for weight assignment when get_disperse_mode() is DISPERSE_MODE_WEIGHTED.

For example, when weight_position is 0.5, if two bone segments with a length of 1.0 exist between three joints, weights are assigned to each joint from root to end at ratios of 0.5, 1.0, and 0.5. Then amounts become 0.25, 0.75, and 1.0 respectively.

## 属性

### mutable_bone_axes {#mutable_bone_axes}

```gdscript
mutable_bone_axes: bool
```

- void set_mutable_bone_axes(value: bool)
- bool are_bone_axes_mutable()

If true, the solver retrieves the bone axis from the bone pose every frame.

If false, the solver retrieves the bone axis from the bone rest and caches it.

### setting_count {#setting_count}

```gdscript
setting_count: int
```

- void set_setting_count(value: int)
- int get_setting_count()

The number of settings.

## 枚举

### DisperseMode {#enum-DisperseMode}

```gdscript
enum DisperseMode { DISPERSE_MODE_EVEN = 0, DISPERSE_MODE_WEIGHTED = 1, DISPERSE_MODE_CUSTOM = 2 }
```

DisperseMode DISPERSE_MODE_EVEN = 0

Assign amounts so that they monotonically increase from 0.0 to 1.0, ensuring all weights are equal. For example, with five joints, the amounts would be 0.2, 0.4, 0.6, 0.8, and 1.0 starting from the root bone.

DisperseMode DISPERSE_MODE_WEIGHTED = 1

Assign amounts so that they monotonically increase from 0.0 to 1.0, based on the length of the bones between joint segments. See also set_weight_position().

DisperseMode DISPERSE_MODE_CUSTOM = 2

You can assign arbitrary amounts to the joint list. See also set_joint_twist_amount().

When is_end_bone_extended() is false, a child of the reference bone exists solely to determine the twist axis, so its custom amount has absolutely no effect at all.

#### DISPERSE_MODE_EVEN {#DISPERSE_MODE_EVEN}

```gdscript
DISPERSE_MODE_EVEN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DISPERSE_MODE_WEIGHTED {#DISPERSE_MODE_WEIGHTED}

```gdscript
DISPERSE_MODE_WEIGHTED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DISPERSE_MODE_CUSTOM {#DISPERSE_MODE_CUSTOM}

```gdscript
DISPERSE_MODE_CUSTOM = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
