# SkeletonModification2DCCDIK

**继承**：[SkeletonModification2D](class_skeletonmodification2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2DCCDIK**

> This SkeletonModification2D uses an algorithm called Cyclic Coordinate Descent Inverse Kinematics, or CCDIK, to manipulate a chain of bones in a Skeleton2D so it reaches a defined target.

<details><summary>官方英文描述（中文精译进行中）</summary>

This SkeletonModification2D uses an algorithm called Cyclic Coordinate Descent Inverse Kinematics, or CCDIK, to manipulate a chain of bones in a Skeleton2D so it reaches a defined target.

CCDIK works by rotating a set of bones, typically called a "bone chain", on a single axis. Each bone is rotated to face the target from the tip (by default), which over a chain of bones allow it to rotate properly to reach the target. Because the bones only rotate on a single axis, CCDIK can look more robotic than other IK solvers.

Note: The CCDIK modifier has ccdik_joints, which are the data objects that hold the data for each joint in the CCDIK chain. This is different from a bone! CCDIK joints hold the data needed for each bone in the bone chain used by CCDIK.

CCDIK also fully supports angle constraints, allowing for more control over how a solution is met.

</details>

## 方法

### get_ccdik_data_chain_length {#get_ccdik_data_chain_length}

```gdscript
int get_ccdik_data_chain_length()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ccdik_joint_bone2d_node {#get_ccdik_joint_bone2d_node}

```gdscript
NodePath get_ccdik_joint_bone2d_node(joint_idx: int) const
```

Returns the Bone2D node assigned to the CCDIK joint at joint_idx.

### get_ccdik_joint_bone_index {#get_ccdik_joint_bone_index}

```gdscript
int get_ccdik_joint_bone_index(joint_idx: int) const
```

Returns the index of the Bone2D node assigned to the CCDIK joint at joint_idx.

### get_ccdik_joint_constraint_angle_invert {#get_ccdik_joint_constraint_angle_invert}

```gdscript
bool get_ccdik_joint_constraint_angle_invert(joint_idx: int) const
```

Returns whether the CCDIK joint at joint_idx uses an inverted joint constraint. See set_ccdik_joint_constraint_angle_invert() for details.

### get_ccdik_joint_constraint_angle_max {#get_ccdik_joint_constraint_angle_max}

```gdscript
float get_ccdik_joint_constraint_angle_max(joint_idx: int) const
```

Returns the maximum angle constraint for the joint at joint_idx.

### get_ccdik_joint_constraint_angle_min {#get_ccdik_joint_constraint_angle_min}

```gdscript
float get_ccdik_joint_constraint_angle_min(joint_idx: int) const
```

Returns the minimum angle constraint for the joint at joint_idx.

### get_ccdik_joint_enable_constraint {#get_ccdik_joint_enable_constraint}

```gdscript
bool get_ccdik_joint_enable_constraint(joint_idx: int) const
```

Returns whether angle constraints on the CCDIK joint at joint_idx are enabled.

### get_ccdik_joint_rotate_from_joint {#get_ccdik_joint_rotate_from_joint}

```gdscript
bool get_ccdik_joint_rotate_from_joint(joint_idx: int) const
```

Returns whether the joint at joint_idx is set to rotate from the joint, true, or to rotate from the tip, false. The default is to rotate from the tip.

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tip_node {#get_tip_node}

```gdscript
NodePath get_tip_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ccdik_data_chain_length {#set_ccdik_data_chain_length}

```gdscript
void set_ccdik_data_chain_length(length: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ccdik_joint_bone2d_node {#set_ccdik_joint_bone2d_node}

```gdscript
void set_ccdik_joint_bone2d_node(joint_idx: int, bone2d_nodepath: NodePath)
```

Sets the Bone2D node assigned to the CCDIK joint at joint_idx.

### set_ccdik_joint_bone_index {#set_ccdik_joint_bone_index}

```gdscript
void set_ccdik_joint_bone_index(joint_idx: int, bone_idx: int)
```

Sets the bone index, bone_idx, of the CCDIK joint at joint_idx. When possible, this will also update the bone2d_node of the CCDIK joint based on data provided by the linked skeleton.

### set_ccdik_joint_constraint_angle_invert {#set_ccdik_joint_constraint_angle_invert}

```gdscript
void set_ccdik_joint_constraint_angle_invert(joint_idx: int, invert: bool)
```

Sets whether the CCDIK joint at joint_idx uses an inverted joint constraint.

An inverted joint constraint only constraints the CCDIK joint to the angles outside of the inputted minimum and maximum angles. For this reason, it is referred to as an inverted joint constraint, as it constraints the joint to the outside of the inputted values.

### set_ccdik_joint_constraint_angle_max {#set_ccdik_joint_constraint_angle_max}

```gdscript
void set_ccdik_joint_constraint_angle_max(joint_idx: int, angle_max: float)
```

Sets the maximum angle constraint for the joint at joint_idx.

### set_ccdik_joint_constraint_angle_min {#set_ccdik_joint_constraint_angle_min}

```gdscript
void set_ccdik_joint_constraint_angle_min(joint_idx: int, angle_min: float)
```

Sets the minimum angle constraint for the joint at joint_idx.

### set_ccdik_joint_enable_constraint {#set_ccdik_joint_enable_constraint}

```gdscript
void set_ccdik_joint_enable_constraint(joint_idx: int, enable_constraint: bool)
```

Determines whether angle constraints on the CCDIK joint at joint_idx are enabled. When true, constraints will be enabled and taken into account when solving.

### set_ccdik_joint_rotate_from_joint {#set_ccdik_joint_rotate_from_joint}

```gdscript
void set_ccdik_joint_rotate_from_joint(joint_idx: int, rotate_from_joint: bool)
```

Sets whether the joint at joint_idx is set to rotate from the joint, true, or to rotate from the tip, false.

### set_target_node {#set_target_node}

```gdscript
void set_target_node(target_nodepath: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tip_node {#set_tip_node}

```gdscript
void set_tip_node(tip_nodepath: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### target_nodepath {#target_nodepath}

```gdscript
target_nodepath: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### tip_nodepath {#tip_nodepath}

```gdscript
tip_nodepath: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ccdik_data_chain_length {#ccdik_data_chain_length}

```gdscript
ccdik_data_chain_length: int
```

- void set_ccdik_data_chain_length(value: int)
- int get_ccdik_data_chain_length()

The number of CCDIK joints in the CCDIK modification.
