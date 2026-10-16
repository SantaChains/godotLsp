# AimModifier3D

**继承**：[BoneConstraint3D](class_boneconstraint3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **AimModifier3D**

> This is a simple version of LookAtModifier3D that only allows bone to the reference without advanced options such as angle limitation or time-based interpolation.

<details><summary>官方英文描述</summary>

This is a simple version of LookAtModifier3D that only allows bone to the reference without advanced options such as angle limitation or time-based interpolation.

The feature is simplified, but instead it is implemented with smooth tracking without euler, see set_use_euler().

</details>

## 方法

### get_forward_axis {#get_forward_axis}

```gdscript
SkeletonModifier3D.BoneAxis get_forward_axis(index: int) const
```

Returns the forward axis of the bone.

### get_primary_rotation_axis {#get_primary_rotation_axis}

```gdscript
Vector3.Axis get_primary_rotation_axis(index: int) const
```

Returns the axis of the first rotation. It is enabled only if is_using_euler() is true.

### is_relative {#is_relative}

```gdscript
bool is_relative(index: int) const
```

Returns true if the relative option is enabled in the setting at index.

### is_using_euler {#is_using_euler}

```gdscript
bool is_using_euler(index: int) const
```

Returns true if it provides rotation with using euler.

### is_using_secondary_rotation {#is_using_secondary_rotation}

```gdscript
bool is_using_secondary_rotation(index: int) const
```

Returns true if it provides rotation by two axes. It is enabled only if is_using_euler() is true.

### set_forward_axis {#set_forward_axis}

```gdscript
void set_forward_axis(index: int, axis: SkeletonModifier3D.BoneAxis)
```

Sets the forward axis of the bone.

### set_primary_rotation_axis {#set_primary_rotation_axis}

```gdscript
void set_primary_rotation_axis(index: int, axis: Vector3.Axis)
```

Sets the axis of the first rotation. It is enabled only if is_using_euler() is true.

### set_relative {#set_relative}

```gdscript
void set_relative(index: int, enabled: bool)
```

Sets relative option in the setting at index to enabled.

If sets enabled to true, the rotation is applied relative to the pose.

If sets enabled to false, the rotation is applied relative to the rest. It means to replace the current pose with the AimModifier3D's result.

### set_use_euler {#set_use_euler}

```gdscript
void set_use_euler(index: int, enabled: bool)
```

If sets enabled to true, it provides rotation with using euler.

If sets enabled to false, it provides rotation with using rotation by arc generated from the forward axis vector and the vector toward the reference.

### set_use_secondary_rotation {#set_use_secondary_rotation}

```gdscript
void set_use_secondary_rotation(index: int, enabled: bool)
```

If sets enabled to true, it provides rotation by two axes. It is enabled only if is_using_euler() is true.

## 属性

### setting_count {#setting_count}

```gdscript
setting_count: int
```

- void set_setting_count(value: int)
- int get_setting_count()

The number of settings in the modifier.
