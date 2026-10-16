# SpringBoneSimulator3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SpringBoneSimulator3D**

> This SkeletonModifier3D can be used to wiggle hair, cloth, and tails. This modifier behaves differently from PhysicalBoneSimulator3D as it attempts to return the original pose after modification.

<details><summary>官方英文描述（中文精译进行中）</summary>

This SkeletonModifier3D can be used to wiggle hair, cloth, and tails. This modifier behaves differently from PhysicalBoneSimulator3D as it attempts to return the original pose after modification.

If you setup set_root_bone() and set_end_bone(), it is treated as one bone chain. Note that it does not support a branched chain like Y-shaped chains.

When a bone chain is created, an array is generated from the bones that exist in between and listed in the joint list.

Several properties can be applied to each joint, such as set_joint_stiffness(), set_joint_drag(), and set_joint_gravity().

For simplicity, you can set values to all joints at the same time by using a Curve. If you want to specify detailed values individually, set set_individual_config() to true.

For physical simulation, SpringBoneSimulator3D can have children as self-standing collisions that are not related to PhysicsServer3D, see also SpringBoneCollision3D.

Warning: A scaled SpringBoneSimulator3D will likely not behave as expected. Make sure that the parent Skeleton3D and its bones are not scaled.

Note: Most methods in this class take an index parameter. This parameter specifies which setting list entry to return if the IK has multiple entries (e.g. settings//root_bone_name).

</details>

## 方法

### are_all_child_collisions_enabled {#are_all_child_collisions_enabled}

```gdscript
bool are_all_child_collisions_enabled(index: int) const
```

Returns true if all child SpringBoneCollision3Ds are contained in the collision list at index in the settings.

### are_bone_axes_mutable {#are_bone_axes_mutable}

```gdscript
bool are_bone_axes_mutable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### clear_collisions {#clear_collisions}

```gdscript
void clear_collisions(index: int)
```

Clears all collisions from the collision list at index in the settings when are_all_child_collisions_enabled() is false.

### clear_exclude_collisions {#clear_exclude_collisions}

```gdscript
void clear_exclude_collisions(index: int)
```

Clears all exclude collisions from the collision list at index in the settings when are_all_child_collisions_enabled() is true.

### clear_settings {#clear_settings}

```gdscript
void clear_settings()
```

Clears all settings.

### get_center_bone {#get_center_bone}

```gdscript
int get_center_bone(index: int) const
```

Returns the center bone index of the bone chain.

### get_center_bone_name {#get_center_bone_name}

```gdscript
String get_center_bone_name(index: int) const
```

Returns the center bone name of the bone chain.

### get_center_from {#get_center_from}

```gdscript
SpringBoneSimulator3D.CenterFrom get_center_from(index: int) const
```

Returns what the center originates from in the bone chain.

### get_center_node {#get_center_node}

```gdscript
NodePath get_center_node(index: int) const
```

Returns the center node path of the bone chain.

### get_collision_count {#get_collision_count}

```gdscript
int get_collision_count(index: int) const
```

Returns the collision count of the bone chain's collision list when are_all_child_collisions_enabled() is false.

### get_collision_path {#get_collision_path}

```gdscript
NodePath get_collision_path(index: int, collision: int) const
```

Returns the node path of the SpringBoneCollision3D at collision in the bone chain's collision list when are_all_child_collisions_enabled() is false.

### get_drag {#get_drag}

```gdscript
float get_drag(index: int) const
```

Returns the drag force damping curve of the bone chain.

### get_drag_damping_curve {#get_drag_damping_curve}

```gdscript
Curve get_drag_damping_curve(index: int) const
```

Returns the drag force damping curve of the bone chain.

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

### get_end_bone_length {#get_end_bone_length}

```gdscript
float get_end_bone_length(index: int) const
```

Returns the end bone tail length of the bone chain when is_end_bone_extended() is true.

### get_end_bone_name {#get_end_bone_name}

```gdscript
String get_end_bone_name(index: int) const
```

Returns the end bone name of the bone chain.

### get_exclude_collision_count {#get_exclude_collision_count}

```gdscript
int get_exclude_collision_count(index: int) const
```

Returns the exclude collision count of the bone chain's exclude collision list when are_all_child_collisions_enabled() is true.

### get_exclude_collision_path {#get_exclude_collision_path}

```gdscript
NodePath get_exclude_collision_path(index: int, collision: int) const
```

Returns the node path of the SpringBoneCollision3D at collision in the bone chain's exclude collision list when are_all_child_collisions_enabled() is true.

### get_external_force {#get_external_force}

```gdscript
Vector3 get_external_force() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gravity {#get_gravity}

```gdscript
float get_gravity(index: int) const
```

Returns the gravity amount of the bone chain.

### get_gravity_damping_curve {#get_gravity_damping_curve}

```gdscript
Curve get_gravity_damping_curve(index: int) const
```

Returns the gravity amount damping curve of the bone chain.

### get_gravity_direction {#get_gravity_direction}

```gdscript
Vector3 get_gravity_direction(index: int) const
```

Returns the gravity direction of the bone chain.

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

### get_joint_drag {#get_joint_drag}

```gdscript
float get_joint_drag(index: int, joint: int) const
```

Returns the drag force at joint in the bone chain's joint list.

### get_joint_gravity {#get_joint_gravity}

```gdscript
float get_joint_gravity(index: int, joint: int) const
```

Returns the gravity amount at joint in the bone chain's joint list.

### get_joint_gravity_direction {#get_joint_gravity_direction}

```gdscript
Vector3 get_joint_gravity_direction(index: int, joint: int) const
```

Returns the gravity direction at joint in the bone chain's joint list.

### get_joint_radius {#get_joint_radius}

```gdscript
float get_joint_radius(index: int, joint: int) const
```

Returns the radius at joint in the bone chain's joint list.

### get_joint_rotation_axis {#get_joint_rotation_axis}

```gdscript
SkeletonModifier3D.RotationAxis get_joint_rotation_axis(index: int, joint: int) const
```

Returns the rotation axis at joint in the bone chain's joint list.

### get_joint_rotation_axis_vector {#get_joint_rotation_axis_vector}

```gdscript
Vector3 get_joint_rotation_axis_vector(index: int, joint: int) const
```

Returns the rotation axis vector for the specified joint in the bone chain. This vector represents the axis around which the joint can rotate. It is determined based on the rotation axis set for the joint.

If get_joint_rotation_axis() is SkeletonModifier3D.ROTATION_AXIS_ALL, this method returns Vector3(0, 0, 0).

### get_joint_stiffness {#get_joint_stiffness}

```gdscript
float get_joint_stiffness(index: int, joint: int) const
```

Returns the stiffness force at joint in the bone chain's joint list.

### get_radius {#get_radius}

```gdscript
float get_radius(index: int) const
```

Returns the joint radius of the bone chain.

### get_radius_damping_curve {#get_radius_damping_curve}

```gdscript
Curve get_radius_damping_curve(index: int) const
```

Returns the joint radius damping curve of the bone chain.

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

### get_rotation_axis {#get_rotation_axis}

```gdscript
SkeletonModifier3D.RotationAxis get_rotation_axis(index: int) const
```

Returns the rotation axis of the bone chain.

### get_rotation_axis_vector {#get_rotation_axis_vector}

```gdscript
Vector3 get_rotation_axis_vector(index: int) const
```

Returns the rotation axis vector of the bone chain. This vector represents the axis around which the bone chain can rotate. It is determined based on the rotation axis set for the bone chain.

If get_rotation_axis() is SkeletonModifier3D.ROTATION_AXIS_ALL, this method returns Vector3(0, 0, 0).

### get_setting_count {#get_setting_count}

```gdscript
int get_setting_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stiffness {#get_stiffness}

```gdscript
float get_stiffness(index: int) const
```

Returns the stiffness force of the bone chain.

### get_stiffness_damping_curve {#get_stiffness_damping_curve}

```gdscript
Curve get_stiffness_damping_curve(index: int) const
```

Returns the stiffness force damping curve of the bone chain.

### is_config_individual {#is_config_individual}

```gdscript
bool is_config_individual(index: int) const
```

Returns true if the config can be edited individually for each joint.

### is_end_bone_extended {#is_end_bone_extended}

```gdscript
bool is_end_bone_extended(index: int) const
```

Returns true if the end bone is extended to have a tail.

### reset {#reset}

```gdscript
void reset()
```

Resets a simulating state with respect to the current bone pose.

It is useful to prevent the simulation result getting violent. For example, calling this immediately after a call to AnimationPlayer.play() without a fading, or within the previous SkeletonModifier3D.modification_processed signal if it's condition changes significantly.

### set_center_bone {#set_center_bone}

```gdscript
void set_center_bone(index: int, bone: int)
```

Sets the center bone index of the bone chain.

### set_center_bone_name {#set_center_bone_name}

```gdscript
void set_center_bone_name(index: int, bone_name: String)
```

Sets the center bone name of the bone chain.

### set_center_from {#set_center_from}

```gdscript
void set_center_from(index: int, center_from: SpringBoneSimulator3D.CenterFrom)
```

Sets what the center originates from in the bone chain.

Bone movement is calculated based on the difference in relative distance between center and bone in the previous and next frames.

For example, if the parent Skeleton3D is used as the center, the bones are considered to have not moved if the Skeleton3D moves in the world.

In this case, only a change in the bone pose is considered to be a bone movement.

### set_center_node {#set_center_node}

```gdscript
void set_center_node(index: int, node_path: NodePath)
```

Sets the center node path of the bone chain.

### set_collision_count {#set_collision_count}

```gdscript
void set_collision_count(index: int, count: int)
```

Sets the number of collisions in the collision list at index in the settings when are_all_child_collisions_enabled() is false.

### set_collision_path {#set_collision_path}

```gdscript
void set_collision_path(index: int, collision: int, node_path: NodePath)
```

Sets the node path of the SpringBoneCollision3D at collision in the bone chain's collision list when are_all_child_collisions_enabled() is false.

### set_drag {#set_drag}

```gdscript
void set_drag(index: int, drag: float)
```

Sets the drag force of the bone chain. The greater the value, the more suppressed the wiggling.

The value is scaled by set_drag_damping_curve() and cached in each joint setting in the joint list.

### set_drag_damping_curve {#set_drag_damping_curve}

```gdscript
void set_drag_damping_curve(index: int, curve: Curve)
```

Sets the drag force damping curve of the bone chain.

### set_enable_all_child_collisions {#set_enable_all_child_collisions}

```gdscript
void set_enable_all_child_collisions(index: int, enabled: bool)
```

If enabled is true, all child SpringBoneCollision3Ds are colliding and set_exclude_collision_path() is enabled as an exclusion list at index in the settings.

If enabled is false, you need to manually register all valid collisions with set_collision_path().

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

### set_end_bone_length {#set_end_bone_length}

```gdscript
void set_end_bone_length(index: int, length: float)
```

Sets the end bone tail length of the bone chain when is_end_bone_extended() is true.

### set_end_bone_name {#set_end_bone_name}

```gdscript
void set_end_bone_name(index: int, bone_name: String)
```

Sets the end bone name of the bone chain.

Note: End bone must be the root bone or a child of the root bone. If they are the same, the tail must be extended by set_extend_end_bone() to jiggle the bone.

### set_exclude_collision_count {#set_exclude_collision_count}

```gdscript
void set_exclude_collision_count(index: int, count: int)
```

Sets the number of exclude collisions in the exclude collision list at index in the settings when are_all_child_collisions_enabled() is true.

### set_exclude_collision_path {#set_exclude_collision_path}

```gdscript
void set_exclude_collision_path(index: int, collision: int, node_path: NodePath)
```

Sets the node path of the SpringBoneCollision3D at collision in the bone chain's exclude collision list when are_all_child_collisions_enabled() is true.

### set_extend_end_bone {#set_extend_end_bone}

```gdscript
void set_extend_end_bone(index: int, enabled: bool)
```

If enabled is true, the end bone is extended to have a tail.

The extended tail config is allocated to the last element in the joint list. In other words, if you set enabled to false, the config of the last element in the joint list has no effect in the simulated result.

### set_external_force {#set_external_force}

```gdscript
void set_external_force(force: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gravity {#set_gravity}

```gdscript
void set_gravity(index: int, gravity: float)
```

Sets the gravity amount of the bone chain. This value is not an acceleration, but a constant velocity of movement in set_gravity_direction().

If gravity is not 0, the modified pose will not return to the original pose since it is always affected by gravity.

The value is scaled by set_gravity_damping_curve() and cached in each joint setting in the joint list.

### set_gravity_damping_curve {#set_gravity_damping_curve}

```gdscript
void set_gravity_damping_curve(index: int, curve: Curve)
```

Sets the gravity amount damping curve of the bone chain.

### set_gravity_direction {#set_gravity_direction}

```gdscript
void set_gravity_direction(index: int, gravity_direction: Vector3)
```

Sets the gravity direction of the bone chain. This value is internally normalized and then multiplied by set_gravity().

The value is cached in each joint setting in the joint list.

### set_individual_config {#set_individual_config}

```gdscript
void set_individual_config(index: int, enabled: bool)
```

If enabled is true, the config can be edited individually for each joint.

### set_joint_drag {#set_joint_drag}

```gdscript
void set_joint_drag(index: int, joint: int, drag: float)
```

Sets the drag force at joint in the bone chain's joint list when is_config_individual() is true.

### set_joint_gravity {#set_joint_gravity}

```gdscript
void set_joint_gravity(index: int, joint: int, gravity: float)
```

Sets the gravity amount at joint in the bone chain's joint list when is_config_individual() is true.

### set_joint_gravity_direction {#set_joint_gravity_direction}

```gdscript
void set_joint_gravity_direction(index: int, joint: int, gravity_direction: Vector3)
```

Sets the gravity direction at joint in the bone chain's joint list when is_config_individual() is true.

### set_joint_radius {#set_joint_radius}

```gdscript
void set_joint_radius(index: int, joint: int, radius: float)
```

Sets the joint radius at joint in the bone chain's joint list when is_config_individual() is true.

### set_joint_rotation_axis {#set_joint_rotation_axis}

```gdscript
void set_joint_rotation_axis(index: int, joint: int, axis: SkeletonModifier3D.RotationAxis)
```

Sets the rotation axis at joint in the bone chain's joint list when is_config_individual() is true.

The axes are based on the reference pose's space, if axis is SkeletonModifier3D.ROTATION_AXIS_CUSTOM, you can specify any axis.

In here, the reference pose is the bone pose immediately before the simulation.

Note: The rotation axis and the forward vector shouldn't be colinear to avoid unintended rotation since SpringBoneSimulator3D does not factor in twisting forces.

### set_joint_rotation_axis_vector {#set_joint_rotation_axis_vector}

```gdscript
void set_joint_rotation_axis_vector(index: int, joint: int, vector: Vector3)
```

Sets the rotation axis vector for the specified joint in the bone chain.

This vector is normalized by an internal process and represents the axis around which the bone chain can rotate.

If the vector length is 0, it is considered synonymous with SkeletonModifier3D.ROTATION_AXIS_ALL.

### set_joint_stiffness {#set_joint_stiffness}

```gdscript
void set_joint_stiffness(index: int, joint: int, stiffness: float)
```

Sets the stiffness force at joint in the bone chain's joint list when is_config_individual() is true.

### set_mutable_bone_axes {#set_mutable_bone_axes}

```gdscript
void set_mutable_bone_axes(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(index: int, radius: float)
```

Sets the joint radius of the bone chain. It is used to move and slide with the SpringBoneCollision3D in the collision list.

The value is scaled by set_radius_damping_curve() and cached in each joint setting in the joint list.

### set_radius_damping_curve {#set_radius_damping_curve}

```gdscript
void set_radius_damping_curve(index: int, curve: Curve)
```

Sets the joint radius damping curve of the bone chain.

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

### set_rotation_axis {#set_rotation_axis}

```gdscript
void set_rotation_axis(index: int, axis: SkeletonModifier3D.RotationAxis)
```

Sets the rotation axis of the bone chain. If set to a specific axis, it acts like a hinge joint. The value is cached in each joint setting in the joint list.

The axes are based on the reference pose's space, if axis is SkeletonModifier3D.ROTATION_AXIS_CUSTOM, you can specify any axis.

In here, the reference pose is the bone pose immediately before the simulation.

Note: The rotation axis vector and the forward vector shouldn't be colinear to avoid unintended rotation since SpringBoneSimulator3D does not factor in twisting forces.

### set_rotation_axis_vector {#set_rotation_axis_vector}

```gdscript
void set_rotation_axis_vector(index: int, vector: Vector3)
```

Sets the rotation axis vector of the bone chain. The value is cached in each joint setting in the joint list.

This vector is normalized by an internal process and represents the axis around which the bone chain can rotate.

If the vector length is 0, it is considered synonymous with SkeletonModifier3D.ROTATION_AXIS_ALL.

### set_setting_count {#set_setting_count}

```gdscript
void set_setting_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stiffness {#set_stiffness}

```gdscript
void set_stiffness(index: int, stiffness: float)
```

Sets the stiffness force of the bone chain. The greater the value, the faster it recovers to its initial pose.

If stiffness is 0, the modified pose will not return to the original pose.

The value is scaled by set_stiffness_damping_curve() and cached in each joint setting in the joint list.

### set_stiffness_damping_curve {#set_stiffness_damping_curve}

```gdscript
void set_stiffness_damping_curve(index: int, curve: Curve)
```

Sets the stiffness force damping curve of the bone chain.

## 属性

### external_force {#external_force}

```gdscript
external_force: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### mutable_bone_axes {#mutable_bone_axes}

```gdscript
mutable_bone_axes: bool
```

- void set_mutable_bone_axes(value: bool)
- bool are_bone_axes_mutable()

If true, the solver retrieves the bone axis from the bone pose every frame.

If false, the solver retrieves the bone axis from the bone rest and caches it, which increases performance slightly, but position changes in the bone pose made before processing this SpringBoneSimulator3D are ignored.

### setting_count {#setting_count}

```gdscript
setting_count: int
```

- void set_setting_count(value: int)
- int get_setting_count()

The number of settings.

## 枚举

### CenterFrom {#enum-CenterFrom}

```gdscript
enum CenterFrom { CENTER_FROM_WORLD_ORIGIN = 0, CENTER_FROM_NODE = 1, CENTER_FROM_BONE = 2 }
```

CenterFrom CENTER_FROM_WORLD_ORIGIN = 0

The world origin is defined as center.

CenterFrom CENTER_FROM_NODE = 1

The Node3D specified by set_center_node() is defined as center.

If Node3D is not found, the parent Skeleton3D is treated as center.

CenterFrom CENTER_FROM_BONE = 2

The bone pose origin of the parent Skeleton3D specified by set_center_bone() is defined as center.

If Node3D is not found, the parent Skeleton3D is treated as center.

#### CENTER_FROM_WORLD_ORIGIN {#CENTER_FROM_WORLD_ORIGIN}

```gdscript
CENTER_FROM_WORLD_ORIGIN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CENTER_FROM_NODE {#CENTER_FROM_NODE}

```gdscript
CENTER_FROM_NODE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CENTER_FROM_BONE {#CENTER_FROM_BONE}

```gdscript
CENTER_FROM_BONE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
