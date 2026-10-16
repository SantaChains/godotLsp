# Skeleton2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Skeleton2D**

> Skeleton2D parents a hierarchy of Bone2D nodes. It holds a reference to each Bone2D's rest pose and acts as a single point of access to its bones.

<details><summary>官方英文描述</summary>

Skeleton2D parents a hierarchy of Bone2D nodes. It holds a reference to each Bone2D's rest pose and acts as a single point of access to its bones.

To set up different types of inverse kinematics for the given Skeleton2D, a SkeletonModificationStack2D should be created. The inverse kinematics be applied by increasing SkeletonModificationStack2D.modification_count and creating the desired number of modifications.

</details>

## 方法

### execute_modifications {#execute_modifications}

```gdscript
void execute_modifications(delta: float, execution_mode: int)
```

Executes all the modifications on the SkeletonModificationStack2D, if the Skeleton2D has one assigned.

### get_bone {#get_bone}

```gdscript
Bone2D get_bone(idx: int)
```

Returns a Bone2D from the node hierarchy parented by Skeleton2D. The object to return is identified by the parameter idx. Bones are indexed by descending the node hierarchy from top to bottom, adding the children of each branch before moving to the next sibling.

### get_bone_count {#get_bone_count}

```gdscript
int get_bone_count() const
```

Returns the number of Bone2D nodes in the node hierarchy parented by Skeleton2D.

### get_bone_local_pose_override {#get_bone_local_pose_override}

```gdscript
Transform2D get_bone_local_pose_override(bone_idx: int)
```

Returns the local pose override transform for bone_idx.

### get_modification_stack {#get_modification_stack}

```gdscript
SkeletonModificationStack2D get_modification_stack() const
```

Returns the SkeletonModificationStack2D attached to this skeleton, if one exists.

### get_skeleton {#get_skeleton}

```gdscript
RID get_skeleton() const
```

Returns the RID of a Skeleton2D instance.

### set_bone_local_pose_override {#set_bone_local_pose_override}

```gdscript
void set_bone_local_pose_override(bone_idx: int, override_pose: Transform2D, strength: float, persistent: bool)
```

Sets the local pose transform, override_pose, for the bone at bone_idx.

strength is the interpolation strength that will be used when applying the pose, and persistent determines if the applied pose will remain.

Note: The pose transform needs to be a local transform relative to the Bone2D node at bone_idx!

### set_modification_stack {#set_modification_stack}

```gdscript
void set_modification_stack(modification_stack: SkeletonModificationStack2D)
```

Sets the SkeletonModificationStack2D attached to this skeleton.

## 信号

### bone_setup_changed {#bone_setup_changed}

```gdscript
signal bone_setup_changed()
```

Emitted when the Bone2D setup attached to this skeletons changes. This is primarily used internally within the skeleton.
