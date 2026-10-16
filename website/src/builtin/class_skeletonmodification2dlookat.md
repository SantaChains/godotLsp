# SkeletonModification2DLookAt

**继承**：[SkeletonModification2D](class_skeletonmodification2d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonModification2DLookAt**

> This SkeletonModification2D rotates a bone to look a target. This is extremely helpful for moving character's head to look at the player, rotating a turret to look at a target, or any other case where you want to make a bone rotate towards something quickly and easily.

<details><summary>官方英文描述</summary>

This SkeletonModification2D rotates a bone to look a target. This is extremely helpful for moving character's head to look at the player, rotating a turret to look at a target, or any other case where you want to make a bone rotate towards something quickly and easily.

</details>

## 方法

### get_additional_rotation {#get_additional_rotation}

```gdscript
float get_additional_rotation() const
```

Returns the amount of additional rotation that is applied after the LookAt modification executes.

### get_bone2d_node {#get_bone2d_node}

```gdscript
NodePath get_bone2d_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_index {#get_bone_index}

```gdscript
int get_bone_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_constraint_angle_invert {#get_constraint_angle_invert}

```gdscript
bool get_constraint_angle_invert() const
```

Returns whether the constraints to this modification are inverted or not.

### get_constraint_angle_max {#get_constraint_angle_max}

```gdscript
float get_constraint_angle_max() const
```

Returns the constraint's maximum allowed angle.

### get_constraint_angle_min {#get_constraint_angle_min}

```gdscript
float get_constraint_angle_min() const
```

Returns the constraint's minimum allowed angle.

### get_enable_constraint {#get_enable_constraint}

```gdscript
bool get_enable_constraint() const
```

Returns true if the LookAt modification is using constraints.

### get_target_node {#get_target_node}

```gdscript
NodePath get_target_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_additional_rotation {#set_additional_rotation}

```gdscript
void set_additional_rotation(rotation: float)
```

Sets the amount of additional rotation that is to be applied after executing the modification. This allows for offsetting the results by the inputted rotation amount.

### set_bone2d_node {#set_bone2d_node}

```gdscript
void set_bone2d_node(bone2d_nodepath: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_index {#set_bone_index}

```gdscript
void set_bone_index(bone_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_constraint_angle_invert {#set_constraint_angle_invert}

```gdscript
void set_constraint_angle_invert(invert: bool)
```

When true, the modification will use an inverted joint constraint.

An inverted joint constraint only constraints the Bone2D to the angles outside of the inputted minimum and maximum angles. For this reason, it is referred to as an inverted joint constraint, as it constraints the joint to the outside of the inputted values.

### set_constraint_angle_max {#set_constraint_angle_max}

```gdscript
void set_constraint_angle_max(angle_max: float)
```

Sets the constraint's maximum allowed angle.

### set_constraint_angle_min {#set_constraint_angle_min}

```gdscript
void set_constraint_angle_min(angle_min: float)
```

Sets the constraint's minimum allowed angle.

### set_enable_constraint {#set_enable_constraint}

```gdscript
void set_enable_constraint(enable_constraint: bool)
```

Sets whether this modification will use constraints or not. When true, constraints will be applied when solving the LookAt modification.

### set_target_node {#set_target_node}

```gdscript
void set_target_node(target_nodepath: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### bone_index {#bone_index}

```gdscript
bone_index: int
```

- void set_bone_index(value: int)
- int get_bone_index()

The index of the Bone2D node that the modification will operate on.

### bone2d_node {#bone2d_node}

```gdscript
bone2d_node: NodePath
```

- void set_bone2d_node(value: NodePath)
- NodePath get_bone2d_node()

The Bone2D node that the modification will operate on.

### target_nodepath {#target_nodepath}

```gdscript
target_nodepath: NodePath
```

- void set_target_node(value: NodePath)
- NodePath get_target_node()

The NodePath to the node that is the target for the LookAt modification. This node is what the modification will rotate the Bone2D to.
