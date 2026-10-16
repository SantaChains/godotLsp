# IterateIK3D

**继承**：[ChainIK3D](class_chainik3d.md) → [IKModifier3D](class_ikmodifier3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **IterateIK3D**

> Base class of SkeletonModifier3D to approach the goal by repeating small rotations.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base class of SkeletonModifier3D to approach the goal by repeating small rotations.

Each bone chain (setting) has one effector, which is processed in order of the setting list. You can set some limitations for each joint.

Note: All the methods in this class take an index parameter. This parameter specifies which setting list entry to return if the IK has multiple entries (e.g. settings//target_node).

</details>

## 方法

### get_angular_delta_limit {#get_angular_delta_limit}

```gdscript
float get_angular_delta_limit() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint_limitation {#get_joint_limitation}

```gdscript
JointLimitation3D get_joint_limitation(index: int, joint: int) const
```

Returns the joint limitation at joint in the bone chain's joint list.

### get_joint_limitation_right_axis {#get_joint_limitation_right_axis}

```gdscript
SkeletonModifier3D.SecondaryDirection get_joint_limitation_right_axis(index: int, joint: int) const
```

Returns the joint limitation right axis at joint in the bone chain's joint list.

### get_joint_limitation_right_axis_vector {#get_joint_limitation_right_axis_vector}

```gdscript
Vector3 get_joint_limitation_right_axis_vector(index: int, joint: int) const
```

Returns the joint limitation right axis vector at joint in the bone chain's joint list.

If get_joint_limitation_right_axis() is SkeletonModifier3D.SECONDARY_DIRECTION_NONE, this method returns Vector3(0, 0, 0).

### get_joint_limitation_rotation_offset {#get_joint_limitation_rotation_offset}

```gdscript
Quaternion get_joint_limitation_rotation_offset(index: int, joint: int) const
```

Returns the joint limitation rotation offset at joint in the bone chain's joint list.

Rotation is done in the local space which is constructed by the bone direction (in general parent to child) as the +Y axis and get_joint_limitation_right_axis_vector() as the +X axis.

If the +X and +Y axes are not orthogonal, the +X axis is implicitly modified to make it orthogonal.

Also, if the length of get_joint_limitation_right_axis_vector() is zero, the space is created by rotating the reference pose using the shortest arc that rotates the +Y axis of the reference pose to match the bone direction.

In here, the reference pose is the bone pose immediately before processing IK.

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

### get_max_iterations {#get_max_iterations}

```gdscript
int get_max_iterations() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_distance {#get_min_distance}

```gdscript
float get_min_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node(index: int) const
```

Returns the target node that the end bone is trying to reach.

### is_deterministic {#is_deterministic}

```gdscript
bool is_deterministic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_delta_limit {#set_angular_delta_limit}

```gdscript
void set_angular_delta_limit(angular_delta_limit: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deterministic {#set_deterministic}

```gdscript
void set_deterministic(deterministic: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_joint_limitation {#set_joint_limitation}

```gdscript
void set_joint_limitation(index: int, joint: int, limitation: JointLimitation3D)
```

Sets the joint limitation at joint in the bone chain's joint list.

### set_joint_limitation_right_axis {#set_joint_limitation_right_axis}

```gdscript
void set_joint_limitation_right_axis(index: int, joint: int, direction: SkeletonModifier3D.SecondaryDirection)
```

Sets the joint limitation right axis at joint in the bone chain's joint list.

### set_joint_limitation_right_axis_vector {#set_joint_limitation_right_axis_vector}

```gdscript
void set_joint_limitation_right_axis_vector(index: int, joint: int, vector: Vector3)
```

Sets the optional joint limitation right axis vector at joint in the bone chain's joint list.

### set_joint_limitation_rotation_offset {#set_joint_limitation_rotation_offset}

```gdscript
void set_joint_limitation_rotation_offset(index: int, joint: int, offset: Quaternion)
```

Sets the joint limitation rotation offset at joint in the bone chain's joint list.

Rotation is done in the local space which is constructed by the bone direction (in general parent to child) as the +Y axis and get_joint_limitation_right_axis_vector() as the +X axis.

If the +X and +Y axes are not orthogonal, the +X axis is implicitly modified to make it orthogonal.

Also, if the length of get_joint_limitation_right_axis_vector() is zero, the space is created by rotating the reference pose using the shortest arc that rotates the +Y axis of the reference pose to match the bone direction.

In here, the reference pose is the bone pose immediately before processing IK.

### set_joint_rotation_axis {#set_joint_rotation_axis}

```gdscript
void set_joint_rotation_axis(index: int, joint: int, axis: SkeletonModifier3D.RotationAxis)
```

Sets the rotation axis at joint in the bone chain's joint list.

The axes are based on the reference pose's space, if axis is SkeletonModifier3D.ROTATION_AXIS_CUSTOM, you can specify any axis.

In here, the reference pose is the bone pose immediately before processing IK.

Note: The rotation axis and the forward vector shouldn't be colinear to avoid unintended rotation since ChainIK3D does not factor in twisting forces.

### set_joint_rotation_axis_vector {#set_joint_rotation_axis_vector}

```gdscript
void set_joint_rotation_axis_vector(index: int, joint: int, axis_vector: Vector3)
```

Sets the rotation axis vector for the specified joint in the bone chain.

This vector is normalized by an internal process and represents the axis around which the bone chain can rotate.

If the vector length is 0, it is considered synonymous with SkeletonModifier3D.ROTATION_AXIS_ALL.

### set_max_iterations {#set_max_iterations}

```gdscript
void set_max_iterations(max_iterations: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_distance {#set_min_distance}

```gdscript
void set_min_distance(min_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_node {#set_target_node}

```gdscript
void set_target_node(index: int, target_node: NodePath)
```

Sets the target node that the end bone is trying to reach.

## 属性

### max_iterations {#max_iterations}

```gdscript
max_iterations: int
```

- void set_max_iterations(value: int)
- int get_max_iterations()

The number of iteration loops used by the IK solver to produce more accurate results.

### min_distance {#min_distance}

```gdscript
min_distance: float
```

- void set_min_distance(value: float)
- float get_min_distance()

The minimum distance between the end bone and the target. If the distance is below this value, the IK solver stops any further iterations.

### angular_delta_limit {#angular_delta_limit}

```gdscript
angular_delta_limit: float
```

- void set_angular_delta_limit(value: float)
- float get_angular_delta_limit()

The maximum amount each bone can rotate in a single iteration.

Note: This limitation is applied during each iteration. For example, if max_iterations is 4 and angular_delta_limit is 5 degrees, the maximum rotation possible in a single frame is 20 degrees.

### deterministic {#deterministic}

```gdscript
deterministic: bool
```

- void set_deterministic(value: bool)
- bool is_deterministic()

If false, the result is calculated from the previous frame's IterateIK3D result as the initial state.

If true, the previous frame's IterateIK3D result is discarded. At this point, the new result is calculated from the bone pose excluding the IterateIK3D as the initial state. This means the result will be always equal as long as the target position and the previous bone pose are the same. However, if angular_delta_limit and max_iterations are set too small, the end bone of the chain will never reach the target.

### setting_count {#setting_count}

```gdscript
setting_count: int
```

- void set_setting_count(value: int)
- int get_setting_count()

The number of settings.
