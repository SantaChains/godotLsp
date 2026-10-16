# SkeletonModification2DTwoBoneIK

**继承**：[SkeletonModification2D](class_skeletonmodification2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2DTwoBoneIK**

> This SkeletonModification2D uses an algorithm typically called TwoBoneIK. This algorithm works by leveraging the law of cosines and the lengths of the bones to figure out what rotation the bones currently have, and what rotation they need to make a complete triangle, where the first bone, the second bone, and the target form the three vertices of the triangle. Because the algorithm works by making a triangle, it can only operate on two bones.

<details><summary>官方英文描述</summary>

This SkeletonModification2D uses an algorithm typically called TwoBoneIK. This algorithm works by leveraging the law of cosines and the lengths of the bones to figure out what rotation the bones currently have, and what rotation they need to make a complete triangle, where the first bone, the second bone, and the target form the three vertices of the triangle. Because the algorithm works by making a triangle, it can only operate on two bones.

TwoBoneIK is great for arms, legs, and really any joints that can be represented by just two bones that bend to reach a target. This solver is more lightweight than SkeletonModification2DFABRIK, but gives similar, natural looking results.

</details>

## 方法

### get_flip_bend_direction {#get_flip_bend_direction}

```gdscript
bool get_flip_bend_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint_one_bone2d_node {#get_joint_one_bone2d_node}

```gdscript
NodePath get_joint_one_bone2d_node() const
```

Returns the Bone2D node that is being used as the first bone in the TwoBoneIK modification.

### get_joint_one_bone_idx {#get_joint_one_bone_idx}

```gdscript
int get_joint_one_bone_idx() const
```

Returns the index of the Bone2D node that is being used as the first bone in the TwoBoneIK modification.

### get_joint_two_bone2d_node {#get_joint_two_bone2d_node}

```gdscript
NodePath get_joint_two_bone2d_node() const
```

Returns the Bone2D node that is being used as the second bone in the TwoBoneIK modification.

### get_joint_two_bone_idx {#get_joint_two_bone_idx}

```gdscript
int get_joint_two_bone_idx() const
```

Returns the index of the Bone2D node that is being used as the second bone in the TwoBoneIK modification.

### get_target_maximum_distance {#get_target_maximum_distance}

```gdscript
float get_target_maximum_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_minimum_distance {#get_target_minimum_distance}

```gdscript
float get_target_minimum_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_bend_direction {#set_flip_bend_direction}

```gdscript
void set_flip_bend_direction(flip_direction: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_joint_one_bone2d_node {#set_joint_one_bone2d_node}

```gdscript
void set_joint_one_bone2d_node(bone2d_node: NodePath)
```

Sets the Bone2D node that is being used as the first bone in the TwoBoneIK modification.

### set_joint_one_bone_idx {#set_joint_one_bone_idx}

```gdscript
void set_joint_one_bone_idx(bone_idx: int)
```

Sets the index of the Bone2D node that is being used as the first bone in the TwoBoneIK modification.

### set_joint_two_bone2d_node {#set_joint_two_bone2d_node}

```gdscript
void set_joint_two_bone2d_node(bone2d_node: NodePath)
```

Sets the Bone2D node that is being used as the second bone in the TwoBoneIK modification.

### set_joint_two_bone_idx {#set_joint_two_bone_idx}

```gdscript
void set_joint_two_bone_idx(bone_idx: int)
```

Sets the index of the Bone2D node that is being used as the second bone in the TwoBoneIK modification.

### set_target_maximum_distance {#set_target_maximum_distance}

```gdscript
void set_target_maximum_distance(maximum_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_minimum_distance {#set_target_minimum_distance}

```gdscript
void set_target_minimum_distance(minimum_distance: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

The NodePath to the node that is the target for the TwoBoneIK modification. This node is what the modification will use when bending the Bone2D nodes.

### target_minimum_distance {#target_minimum_distance}

```gdscript
target_minimum_distance: float
```

- void set_target_minimum_distance(value: float)
- float get_target_minimum_distance()

The minimum distance the target can be at. If the target is closer than this distance, the modification will solve as if it's at this minimum distance. When set to 0, the modification will solve without distance constraints.

### target_maximum_distance {#target_maximum_distance}

```gdscript
target_maximum_distance: float
```

- void set_target_maximum_distance(value: float)
- float get_target_maximum_distance()

The maximum distance the target can be at. If the target is farther than this distance, the modification will solve as if it's at this maximum distance. When set to 0, the modification will solve without distance constraints.

### flip_bend_direction {#flip_bend_direction}

```gdscript
flip_bend_direction: bool
```

- void set_flip_bend_direction(value: bool)
- bool get_flip_bend_direction()

If true, the bones in the modification will bend outward as opposed to inwards when contracting. If false, the bones will bend inwards when contracting.
