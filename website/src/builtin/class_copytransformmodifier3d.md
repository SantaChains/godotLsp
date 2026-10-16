# CopyTransformModifier3D

**继承**：[BoneConstraint3D](class_boneconstraint3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CopyTransformModifier3D**

> Apply the copied transform of the bone set by BoneConstraint3D.set_reference_bone() to the bone set by BoneConstraint3D.set_apply_bone() with processing it with some masks and options.

<details><summary>官方英文描述</summary>

Apply the copied transform of the bone set by BoneConstraint3D.set_reference_bone() to the bone set by BoneConstraint3D.set_apply_bone() with processing it with some masks and options.

There are 4 ways to apply the transform, depending on the combination of set_relative() and set_additive().

Relative + Additive:

- Extract reference pose relative to the rest and add it to the apply bone's pose.

Relative + Not Additive:

- Extract reference pose relative to the rest and add it to the apply bone's rest.

Not Relative + Additive:

- Extract reference pose absolutely and add it to the apply bone's pose.

Not Relative + Not Additive:

- Extract reference pose absolutely and the apply bone's pose is replaced with it.

Note: Relative option is available only in the case BoneConstraint3D.get_reference_type() is BoneConstraint3D.REFERENCE_TYPE_BONE. See also ReferenceType.

</details>

## 方法

### get_axis_flags {#get_axis_flags}

```gdscript
bitfield::CopyTransformModifier3D.AxisFlag get_axis_flags(index: int) const
```

Returns the axis flags of the setting at index.

### get_copy_flags {#get_copy_flags}

```gdscript
bitfield::CopyTransformModifier3D.TransformFlag get_copy_flags(index: int) const
```

Returns the copy flags of the setting at index.

### get_invert_flags {#get_invert_flags}

```gdscript
bitfield::CopyTransformModifier3D.AxisFlag get_invert_flags(index: int) const
```

Returns the invert flags of the setting at index.

### is_additive {#is_additive}

```gdscript
bool is_additive(index: int) const
```

Returns true if the additive option is enabled in the setting at index.

### is_axis_x_enabled {#is_axis_x_enabled}

```gdscript
bool is_axis_x_enabled(index: int) const
```

Returns true if the enable flags has the flag for the X-axis in the setting at index. See also set_axis_flags().

### is_axis_x_inverted {#is_axis_x_inverted}

```gdscript
bool is_axis_x_inverted(index: int) const
```

Returns true if the invert flags has the flag for the X-axis in the setting at index. See also set_invert_flags().

### is_axis_y_enabled {#is_axis_y_enabled}

```gdscript
bool is_axis_y_enabled(index: int) const
```

Returns true if the enable flags has the flag for the Y-axis in the setting at index. See also set_axis_flags().

### is_axis_y_inverted {#is_axis_y_inverted}

```gdscript
bool is_axis_y_inverted(index: int) const
```

Returns true if the invert flags has the flag for the Y-axis in the setting at index. See also set_invert_flags().

### is_axis_z_enabled {#is_axis_z_enabled}

```gdscript
bool is_axis_z_enabled(index: int) const
```

Returns true if the enable flags has the flag for the Z-axis in the setting at index. See also set_axis_flags().

### is_axis_z_inverted {#is_axis_z_inverted}

```gdscript
bool is_axis_z_inverted(index: int) const
```

Returns true if the invert flags has the flag for the Z-axis in the setting at index. See also set_invert_flags().

### is_position_copying {#is_position_copying}

```gdscript
bool is_position_copying(index: int) const
```

Returns true if the copy flags has the flag for the position in the setting at index. See also set_copy_flags().

### is_relative {#is_relative}

```gdscript
bool is_relative(index: int) const
```

Returns true if the relative option is enabled in the setting at index.

### is_rotation_copying {#is_rotation_copying}

```gdscript
bool is_rotation_copying(index: int) const
```

Returns true if the copy flags has the flag for the rotation in the setting at index. See also set_copy_flags().

### is_scale_copying {#is_scale_copying}

```gdscript
bool is_scale_copying(index: int) const
```

Returns true if the copy flags has the flag for the scale in the setting at index. See also set_copy_flags().

### set_additive {#set_additive}

```gdscript
void set_additive(index: int, enabled: bool)
```

Sets additive option in the setting at index to enabled. This mainly affects the process of applying transform to the BoneConstraint3D.set_apply_bone().

If sets enabled to true, the processed transform is added to the pose of the current apply bone.

If sets enabled to false, the pose of the current apply bone is replaced with the processed transform. However, if set set_relative() to true, the transform is relative to rest.

### set_axis_flags {#set_axis_flags}

```gdscript
void set_axis_flags(index: int, axis_flags: bitfield::CopyTransformModifier3D.AxisFlag)
```

Sets the flags to copy axes. If the flag is valid, the axis is copied.

### set_axis_x_enabled {#set_axis_x_enabled}

```gdscript
void set_axis_x_enabled(index: int, enabled: bool)
```

If sets enabled to true, the X-axis will be copied.

### set_axis_x_inverted {#set_axis_x_inverted}

```gdscript
void set_axis_x_inverted(index: int, enabled: bool)
```

If sets enabled to true, the X-axis will be inverted.

### set_axis_y_enabled {#set_axis_y_enabled}

```gdscript
void set_axis_y_enabled(index: int, enabled: bool)
```

If sets enabled to true, the Y-axis will be copied.

### set_axis_y_inverted {#set_axis_y_inverted}

```gdscript
void set_axis_y_inverted(index: int, enabled: bool)
```

If sets enabled to true, the Y-axis will be inverted.

### set_axis_z_enabled {#set_axis_z_enabled}

```gdscript
void set_axis_z_enabled(index: int, enabled: bool)
```

If sets enabled to true, the Z-axis will be copied.

### set_axis_z_inverted {#set_axis_z_inverted}

```gdscript
void set_axis_z_inverted(index: int, enabled: bool)
```

If sets enabled to true, the Z-axis will be inverted.

### set_copy_flags {#set_copy_flags}

```gdscript
void set_copy_flags(index: int, copy_flags: bitfield::CopyTransformModifier3D.TransformFlag)
```

Sets the flags to process the transform operations. If the flag is valid, the transform operation is processed.

Note: If the rotation is valid for only one axis, it respects the roll of the valid axis. If the rotation is valid for two axes, it discards the roll of the invalid axis.

### set_copy_position {#set_copy_position}

```gdscript
void set_copy_position(index: int, enabled: bool)
```

If sets enabled to true, the position will be copied.

### set_copy_rotation {#set_copy_rotation}

```gdscript
void set_copy_rotation(index: int, enabled: bool)
```

If sets enabled to true, the rotation will be copied.

### set_copy_scale {#set_copy_scale}

```gdscript
void set_copy_scale(index: int, enabled: bool)
```

If sets enabled to true, the scale will be copied.

### set_invert_flags {#set_invert_flags}

```gdscript
void set_invert_flags(index: int, axis_flags: bitfield::CopyTransformModifier3D.AxisFlag)
```

Sets the flags to inverte axes. If the flag is valid, the axis is copied.

Note: An inverted scale means an inverse number, not a negative scale. For example, inverting 2.0 means 0.5.

Note: An inverted rotation flips the elements of the quaternion. For example, a two-axis inversion will flip the roll of each axis, and a three-axis inversion will flip the final orientation. However, be aware that flipping only one axis may cause unintended rotation by the unflipped axes, due to the characteristics of the quaternion.

### set_relative {#set_relative}

```gdscript
void set_relative(index: int, enabled: bool)
```

Sets relative option in the setting at index to enabled.

If sets enabled to true, the extracted and applying transform is relative to the rest.

If sets enabled to false, the extracted transform is absolute.

## 属性

### setting_count {#setting_count}

```gdscript
setting_count: int
```

- void set_setting_count(value: int)
- int get_setting_count()

The number of settings in the modifier.

## 枚举

### TransformFlag {#enum-TransformFlag}

```gdscript
enum TransformFlag（位标志） { TRANSFORM_FLAG_POSITION = 1, TRANSFORM_FLAG_ROTATION = 2, TRANSFORM_FLAG_SCALE = 4, TRANSFORM_FLAG_ALL = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSFORM_FLAG_POSITION {#TRANSFORM_FLAG_POSITION}

```gdscript
TRANSFORM_FLAG_POSITION = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSFORM_FLAG_ROTATION {#TRANSFORM_FLAG_ROTATION}

```gdscript
TRANSFORM_FLAG_ROTATION = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSFORM_FLAG_SCALE {#TRANSFORM_FLAG_SCALE}

```gdscript
TRANSFORM_FLAG_SCALE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSFORM_FLAG_ALL {#TRANSFORM_FLAG_ALL}

```gdscript
TRANSFORM_FLAG_ALL = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### AxisFlag {#enum-AxisFlag}

```gdscript
enum AxisFlag（位标志） { AXIS_FLAG_X = 1, AXIS_FLAG_Y = 2, AXIS_FLAG_Z = 4, AXIS_FLAG_ALL = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_FLAG_X {#AXIS_FLAG_X}

```gdscript
AXIS_FLAG_X = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_FLAG_Y {#AXIS_FLAG_Y}

```gdscript
AXIS_FLAG_Y = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_FLAG_Z {#AXIS_FLAG_Z}

```gdscript
AXIS_FLAG_Z = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_FLAG_ALL {#AXIS_FLAG_ALL}

```gdscript
AXIS_FLAG_ALL = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
