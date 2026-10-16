# SkeletonModification2DFABRIK

**继承**：[SkeletonModification2D](class_skeletonmodification2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2DFABRIK**

> This SkeletonModification2D uses an algorithm called Forward And Backward Reaching Inverse Kinematics, or FABRIK, to rotate a bone chain so that it reaches a target.

<details><summary>官方英文描述</summary>

This SkeletonModification2D uses an algorithm called Forward And Backward Reaching Inverse Kinematics, or FABRIK, to rotate a bone chain so that it reaches a target.

FABRIK works by knowing the positions and lengths of a series of bones, typically called a "bone chain". It first starts by running a forward pass, which places the final bone at the target's position. Then all other bones are moved towards the tip bone, so they stay at the defined bone length away. Then a backwards pass is performed, where the root/first bone in the FABRIK chain is placed back at the origin. Then all other bones are moved so they stay at the defined bone length away. This positions the bone chain so that it reaches the target when possible, but all of the bones stay the correct length away from each other.

Because of how FABRIK works, it often gives more natural results than those seen in SkeletonModification2DCCDIK.

Note: The FABRIK modifier has fabrik_joints, which are the data objects that hold the data for each joint in the FABRIK chain. This is different from Bone2D nodes! FABRIK joints hold the data needed for each Bone2D in the bone chain used by FABRIK.

To help control how the FABRIK joints move, a magnet vector can be passed, which can nudge the bones in a certain direction prior to solving, giving a level of control over the final result.

</details>

## 方法

### get_fabrik_data_chain_length {#get_fabrik_data_chain_length}

```gdscript
int get_fabrik_data_chain_length()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fabrik_joint_bone2d_node {#get_fabrik_joint_bone2d_node}

```gdscript
NodePath get_fabrik_joint_bone2d_node(joint_idx: int) const
```

Returns the Bone2D node assigned to the FABRIK joint at joint_idx.

### get_fabrik_joint_bone_index {#get_fabrik_joint_bone_index}

```gdscript
int get_fabrik_joint_bone_index(joint_idx: int) const
```

Returns the index of the Bone2D node assigned to the FABRIK joint at joint_idx.

### get_fabrik_joint_magnet_position {#get_fabrik_joint_magnet_position}

```gdscript
Vector2 get_fabrik_joint_magnet_position(joint_idx: int) const
```

Returns the magnet position vector for the joint at joint_idx.

### get_fabrik_joint_use_target_rotation {#get_fabrik_joint_use_target_rotation}

```gdscript
bool get_fabrik_joint_use_target_rotation(joint_idx: int) const
```

Returns whether the joint is using the target's rotation rather than allowing FABRIK to rotate the joint. This option only applies to the tip/final joint in the chain.

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fabrik_data_chain_length {#set_fabrik_data_chain_length}

```gdscript
void set_fabrik_data_chain_length(length: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fabrik_joint_bone2d_node {#set_fabrik_joint_bone2d_node}

```gdscript
void set_fabrik_joint_bone2d_node(joint_idx: int, bone2d_nodepath: NodePath)
```

Sets the Bone2D node assigned to the FABRIK joint at joint_idx.

### set_fabrik_joint_bone_index {#set_fabrik_joint_bone_index}

```gdscript
void set_fabrik_joint_bone_index(joint_idx: int, bone_idx: int)
```

Sets the bone index, bone_idx, of the FABRIK joint at joint_idx. When possible, this will also update the bone2d_node of the FABRIK joint based on data provided by the linked skeleton.

### set_fabrik_joint_magnet_position {#set_fabrik_joint_magnet_position}

```gdscript
void set_fabrik_joint_magnet_position(joint_idx: int, magnet_position: Vector2)
```

Sets the magnet position vector for the joint at joint_idx.

### set_fabrik_joint_use_target_rotation {#set_fabrik_joint_use_target_rotation}

```gdscript
void set_fabrik_joint_use_target_rotation(joint_idx: int, use_target_rotation: bool)
```

Sets whether the joint at joint_idx will use the target node's rotation rather than letting FABRIK rotate the node.

Note: This option only works for the tip/final joint in the chain. For all other nodes, this option will be ignored.

### set_target_node {#set_target_node}

```gdscript
void set_target_node(target_nodepath: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### target_nodepath {#target_nodepath}

```gdscript
target_nodepath: NodePath
```

- void set_target_node(value: NodePath)
- NodePath get_target_node()

The NodePath to the node that is the target for the FABRIK modification. This node is what the FABRIK chain will attempt to rotate the bone chain to.

### fabrik_data_chain_length {#fabrik_data_chain_length}

```gdscript
fabrik_data_chain_length: int
```

- void set_fabrik_data_chain_length(value: int)
- int get_fabrik_data_chain_length()

The number of FABRIK joints in the FABRIK modification.
