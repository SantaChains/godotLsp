# TwoBoneIK3D

**继承**：[IKModifier3D](class_ikmodifier3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **TwoBoneIK3D**

> This IKModifier3D requires a pole target. It provides deterministic results by constructing a plane from each joint and pole target and finding the intersection of two circles (disks in 3D).

<details><summary>官方英文描述（中文精译进行中）</summary>

This IKModifier3D requires a pole target. It provides deterministic results by constructing a plane from each joint and pole target and finding the intersection of two circles (disks in 3D).

This IK can handle twist by setting the pole direction. If there are more than one bone between each set bone, their rotations are ignored, and the straight line connecting the root-middle and middle-end joints are treated as virtual bones.

Note: All the methods in this class take an index parameter. This parameter specifies which setting list entry to return if the IK has multiple entries (e.g. settings//root_bone_name).

</details>

## 方法

### get_end_bone {#get_end_bone}

```gdscript
int get_end_bone(index: int) const
```

Returns the end bone index.

### get_end_bone_direction {#get_end_bone_direction}

```gdscript
SkeletonModifier3D.BoneDirection get_end_bone_direction(index: int) const
```

Returns the end bone's tail direction when is_end_bone_extended() is true.

### get_end_bone_length {#get_end_bone_length}

```gdscript
float get_end_bone_length(index: int) const
```

Returns the end bone tail length of the bone chain when is_end_bone_extended() is true.

### get_end_bone_name {#get_end_bone_name}

```gdscript
String get_end_bone_name(index: int) const
```

Returns the end bone name.

### get_middle_bone {#get_middle_bone}

```gdscript
int get_middle_bone(index: int) const
```

Returns the middle bone index.

### get_middle_bone_name {#get_middle_bone_name}

```gdscript
String get_middle_bone_name(index: int) const
```

Returns the middle bone name.

### get_pole_direction {#get_pole_direction}

```gdscript
SkeletonModifier3D.SecondaryDirection get_pole_direction(index: int) const
```

Returns the pole direction.

### get_pole_direction_vector {#get_pole_direction_vector}

```gdscript
Vector3 get_pole_direction_vector(index: int) const
```

Returns the pole direction vector.

If get_pole_direction() is SkeletonModifier3D.SECONDARY_DIRECTION_NONE, this method returns Vector3(0, 0, 0).

### get_pole_node {#get_pole_node}

```gdscript
NodePath get_pole_node(index: int) const
```

Returns the pole target node that constructs a plane which the joints are all on and the pole is trying to direct.

### get_root_bone {#get_root_bone}

```gdscript
int get_root_bone(index: int) const
```

Returns the root bone index.

### get_root_bone_name {#get_root_bone_name}

```gdscript
String get_root_bone_name(index: int) const
```

Returns the root bone name.

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node(index: int) const
```

Returns the target node that the end bone is trying to reach.

### is_end_bone_extended {#is_end_bone_extended}

```gdscript
bool is_end_bone_extended(index: int) const
```

Returns true if the end bone is extended to have a tail.

### is_using_virtual_end {#is_using_virtual_end}

```gdscript
bool is_using_virtual_end(index: int) const
```

Returns true if the end bone is extended from the middle bone as a virtual bone.

### set_end_bone {#set_end_bone}

```gdscript
void set_end_bone(index: int, bone: int)
```

Sets the end bone index.

### set_end_bone_direction {#set_end_bone_direction}

```gdscript
void set_end_bone_direction(index: int, bone_direction: SkeletonModifier3D.BoneDirection)
```

Sets the end bone tail direction when is_end_bone_extended() is true.

### set_end_bone_length {#set_end_bone_length}

```gdscript
void set_end_bone_length(index: int, length: float)
```

Sets the end bone tail length when is_end_bone_extended() is true.

### set_end_bone_name {#set_end_bone_name}

```gdscript
void set_end_bone_name(index: int, bone_name: String)
```

Sets the end bone name.

Note: The end bone must be a child of the middle bone.

### set_extend_end_bone {#set_extend_end_bone}

```gdscript
void set_extend_end_bone(index: int, enabled: bool)
```

If enabled is true, the end bone is extended to have a tail.

### set_middle_bone {#set_middle_bone}

```gdscript
void set_middle_bone(index: int, bone: int)
```

Sets the middle bone index.

### set_middle_bone_name {#set_middle_bone_name}

```gdscript
void set_middle_bone_name(index: int, bone_name: String)
```

Sets the middle bone name.

Note: The middle bone must be a child of the root bone.

### set_pole_direction {#set_pole_direction}

```gdscript
void set_pole_direction(index: int, direction: SkeletonModifier3D.SecondaryDirection)
```

Sets the pole direction.

The pole is on the middle bone and will direct to the pole target.

The rotation axis is a vector that is orthogonal to this and the forward vector.

Note: The pole direction and the forward vector shouldn't be colinear to avoid unintended rotation.

### set_pole_direction_vector {#set_pole_direction_vector}

```gdscript
void set_pole_direction_vector(index: int, vector: Vector3)
```

Sets the pole direction vector.

This vector is normalized by an internal process.

If the vector length is 0, it is considered synonymous with SkeletonModifier3D.SECONDARY_DIRECTION_NONE.

### set_pole_node {#set_pole_node}

```gdscript
void set_pole_node(index: int, pole_node: NodePath)
```

Sets the pole target node that constructs a plane which the joints are all on and the pole is trying to direct.

### set_root_bone {#set_root_bone}

```gdscript
void set_root_bone(index: int, bone: int)
```

Sets the root bone index.

### set_root_bone_name {#set_root_bone_name}

```gdscript
void set_root_bone_name(index: int, bone_name: String)
```

Sets the root bone name.

### set_target_node {#set_target_node}

```gdscript
void set_target_node(index: int, target_node: NodePath)
```

Sets the target node that the end bone is trying to reach.

### set_use_virtual_end {#set_use_virtual_end}

```gdscript
void set_use_virtual_end(index: int, enabled: bool)
```

If enabled is true, the end bone is extended from the middle bone as a virtual bone.

## 属性

### setting_count {#setting_count}

```gdscript
setting_count: int
```

- void set_setting_count(value: int)
- int get_setting_count()

The number of settings.
