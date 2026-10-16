# ConvertTransformModifier3D

**继承**：[BoneConstraint3D](class_boneconstraint3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **ConvertTransformModifier3D**

> Apply the copied transform of the bone set by BoneConstraint3D.set_reference_bone() to the bone set by BoneConstraint3D.set_apply_bone() about the specific axis with remapping it with some options.

<details><summary>官方英文描述（中文精译进行中）</summary>

Apply the copied transform of the bone set by BoneConstraint3D.set_reference_bone() to the bone set by BoneConstraint3D.set_apply_bone() about the specific axis with remapping it with some options.

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

Note: If there is a rotation greater than 180 degrees with constrained axes, flipping may occur.

</details>

## 方法

### get_apply_axis {#get_apply_axis}

```gdscript
Vector3.Axis get_apply_axis(index: int) const
```

Returns the axis of the remapping destination transform.

### get_apply_range_max {#get_apply_range_max}

```gdscript
float get_apply_range_max(index: int) const
```

Returns the maximum value of the remapping destination range.

### get_apply_range_min {#get_apply_range_min}

```gdscript
float get_apply_range_min(index: int) const
```

Returns the minimum value of the remapping destination range.

### get_apply_transform_mode {#get_apply_transform_mode}

```gdscript
ConvertTransformModifier3D.TransformMode get_apply_transform_mode(index: int) const
```

Returns the operation of the remapping destination transform.

### get_reference_axis {#get_reference_axis}

```gdscript
Vector3.Axis get_reference_axis(index: int) const
```

Returns the axis of the remapping source transform.

### get_reference_range_max {#get_reference_range_max}

```gdscript
float get_reference_range_max(index: int) const
```

Returns the maximum value of the remapping source range.

### get_reference_range_min {#get_reference_range_min}

```gdscript
float get_reference_range_min(index: int) const
```

Returns the minimum value of the remapping source range.

### get_reference_transform_mode {#get_reference_transform_mode}

```gdscript
ConvertTransformModifier3D.TransformMode get_reference_transform_mode(index: int) const
```

Returns the operation of the remapping source transform.

### is_additive {#is_additive}

```gdscript
bool is_additive(index: int) const
```

Returns true if the additive option is enabled in the setting at index.

### is_relative {#is_relative}

```gdscript
bool is_relative(index: int) const
```

Returns true if the relative option is enabled in the setting at index.

### set_additive {#set_additive}

```gdscript
void set_additive(index: int, enabled: bool)
```

Sets additive option in the setting at index to enabled. This mainly affects the process of applying transform to the BoneConstraint3D.set_apply_bone().

If sets enabled to true, the processed transform is added to the pose of the current apply bone.

If sets enabled to false, the pose of the current apply bone is replaced with the processed transform. However, if set set_relative() to true, the transform is relative to rest.

### set_apply_axis {#set_apply_axis}

```gdscript
void set_apply_axis(index: int, axis: Vector3.Axis)
```

Sets the axis of the remapping destination transform.

### set_apply_range_max {#set_apply_range_max}

```gdscript
void set_apply_range_max(index: int, range_max: float)
```

Sets the maximum value of the remapping destination range.

### set_apply_range_min {#set_apply_range_min}

```gdscript
void set_apply_range_min(index: int, range_min: float)
```

Sets the minimum value of the remapping destination range.

### set_apply_transform_mode {#set_apply_transform_mode}

```gdscript
void set_apply_transform_mode(index: int, transform_mode: ConvertTransformModifier3D.TransformMode)
```

Sets the operation of the remapping destination transform.

### set_reference_axis {#set_reference_axis}

```gdscript
void set_reference_axis(index: int, axis: Vector3.Axis)
```

Sets the axis of the remapping source transform.

### set_reference_range_max {#set_reference_range_max}

```gdscript
void set_reference_range_max(index: int, range_max: float)
```

Sets the maximum value of the remapping source range.

### set_reference_range_min {#set_reference_range_min}

```gdscript
void set_reference_range_min(index: int, range_min: float)
```

Sets the minimum value of the remapping source range.

### set_reference_transform_mode {#set_reference_transform_mode}

```gdscript
void set_reference_transform_mode(index: int, transform_mode: ConvertTransformModifier3D.TransformMode)
```

Sets the operation of the remapping source transform.

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

### TransformMode {#enum-TransformMode}

```gdscript
enum TransformMode { TRANSFORM_MODE_POSITION = 0, TRANSFORM_MODE_ROTATION = 1, TRANSFORM_MODE_SCALE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSFORM_MODE_POSITION {#TRANSFORM_MODE_POSITION}

```gdscript
TRANSFORM_MODE_POSITION = 0
```

Convert with position. Transfer the difference.

#### TRANSFORM_MODE_ROTATION {#TRANSFORM_MODE_ROTATION}

```gdscript
TRANSFORM_MODE_ROTATION = 1
```

Convert with rotation. The angle is the roll for the specified axis.

#### TRANSFORM_MODE_SCALE {#TRANSFORM_MODE_SCALE}

```gdscript
TRANSFORM_MODE_SCALE = 2
```

Convert with scale. Transfers the ratio, not the difference.

---
