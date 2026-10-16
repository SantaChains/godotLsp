# SkeletonModification2DPhysicalBones

**继承**：[SkeletonModification2D](class_skeletonmodification2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2DPhysicalBones**

> This modification takes the transforms of PhysicalBone2D nodes and applies them to Bone2D nodes. This allows the Bone2D nodes to react to physics thanks to the linked PhysicalBone2D nodes.

<details><summary>官方英文描述</summary>

This modification takes the transforms of PhysicalBone2D nodes and applies them to Bone2D nodes. This allows the Bone2D nodes to react to physics thanks to the linked PhysicalBone2D nodes.

</details>

## 方法

### fetch_physical_bones {#fetch_physical_bones}

```gdscript
void fetch_physical_bones()
```

Empties the list of PhysicalBone2D nodes and populates it with all PhysicalBone2D nodes that are children of the Skeleton2D.

### get_physical_bone_chain_length {#get_physical_bone_chain_length}

```gdscript
int get_physical_bone_chain_length()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physical_bone_node {#get_physical_bone_node}

```gdscript
NodePath get_physical_bone_node(joint_idx: int) const
```

Returns the PhysicalBone2D node at joint_idx.

### set_physical_bone_chain_length {#set_physical_bone_chain_length}

```gdscript
void set_physical_bone_chain_length(length: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physical_bone_node {#set_physical_bone_node}

```gdscript
void set_physical_bone_node(joint_idx: int, physicalbone2d_node: NodePath)
```

Sets the PhysicalBone2D node at joint_idx.

Note: This is just the index used for this modification, not the bone index used in the Skeleton2D.

### start_simulation {#start_simulation}

```gdscript
void start_simulation(bones: typedarray::StringName = [])
```

Tell the PhysicalBone2D nodes to start simulating and interacting with the physics world.

Optionally, an array of bone names can be passed to this function, and that will cause only PhysicalBone2D nodes with those names to start simulating.

### stop_simulation {#stop_simulation}

```gdscript
void stop_simulation(bones: typedarray::StringName = [])
```

Tell the PhysicalBone2D nodes to stop simulating and interacting with the physics world.

Optionally, an array of bone names can be passed to this function, and that will cause only PhysicalBone2D nodes with those names to stop simulating.

## 属性

### physical_bone_chain_length {#physical_bone_chain_length}

```gdscript
physical_bone_chain_length: int
```

- void set_physical_bone_chain_length(value: int)
- int get_physical_bone_chain_length()

The number of PhysicalBone2D nodes linked in this modification.
