# RetargetModifier3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **RetargetModifier3D**

> Retrieves the pose (or global pose) relative to the parent Skeleton's rest in model space and transfers it to the child Skeleton.

<details><summary>官方英文描述（中文精译进行中）</summary>

Retrieves the pose (or global pose) relative to the parent Skeleton's rest in model space and transfers it to the child Skeleton.

This modifier rewrites the pose of the child skeleton directly in the parent skeleton's update process. This means that it overwrites the mapped bone pose set in the normal process on the target skeleton. If you want to set the target skeleton bone pose after retargeting, you will need to add a SkeletonModifier3D child to the target skeleton and thereby modify the pose.

Note: When the use_global_pose is enabled, even if it is an unmapped bone, it can cause visual problems because the global pose is applied ignoring the parent bone's pose if it has mapped bone children. See also use_global_pose.

</details>

## 方法

### get_enable_flags {#get_enable_flags}

```gdscript
bitfield::RetargetModifier3D.TransformFlag get_enable_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_profile {#get_profile}

```gdscript
SkeletonProfile get_profile() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_position_enabled {#is_position_enabled}

```gdscript
bool is_position_enabled() const
```

Returns true if enable has TRANSFORM_FLAG_POSITION.

### is_rotation_enabled {#is_rotation_enabled}

```gdscript
bool is_rotation_enabled() const
```

Returns true if enable has TRANSFORM_FLAG_ROTATION.

### is_scale_enabled {#is_scale_enabled}

```gdscript
bool is_scale_enabled() const
```

Returns true if enable has TRANSFORM_FLAG_SCALE.

### is_using_global_pose {#is_using_global_pose}

```gdscript
bool is_using_global_pose() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_flags {#set_enable_flags}

```gdscript
void set_enable_flags(enable_flags: bitfield::RetargetModifier3D.TransformFlag)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position_enabled {#set_position_enabled}

```gdscript
void set_position_enabled(enabled: bool)
```

Sets TRANSFORM_FLAG_POSITION into enable.

### set_profile {#set_profile}

```gdscript
void set_profile(profile: SkeletonProfile)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_enabled {#set_rotation_enabled}

```gdscript
void set_rotation_enabled(enabled: bool)
```

Sets TRANSFORM_FLAG_ROTATION into enable.

### set_scale_enabled {#set_scale_enabled}

```gdscript
void set_scale_enabled(enabled: bool)
```

Sets TRANSFORM_FLAG_SCALE into enable.

### set_use_global_pose {#set_use_global_pose}

```gdscript
void set_use_global_pose(use_global_pose: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### profile {#profile}

```gdscript
profile: SkeletonProfile
```

- void set_profile(value: SkeletonProfile)
- SkeletonProfile get_profile()

SkeletonProfile for retargeting bones with names matching the bone list.

### use_global_pose {#use_global_pose}

```gdscript
use_global_pose: bool
```

- void set_use_global_pose(value: bool)
- bool is_using_global_pose()

If false, in case the target skeleton has fewer bones than the source skeleton, the source bone parent's transform will be ignored.

Instead, it is possible to retarget between models with different body shapes, and position, rotation, and scale can be retargeted separately.

If true, retargeting is performed taking into account global pose.

In case the target skeleton has fewer bones than the source skeleton, the source bone parent's transform is taken into account. However, bone length between skeletons must match exactly, if not, the bones will be forced to expand or shrink.

This is useful for using dummy bone with length 0 to match postures when retargeting between models with different number of bones.

### enable {#enable}

```gdscript
enable: int
```

- void set_enable_flags(value: BitField[TransformFlag])
- BitField[TransformFlag] get_enable_flags()

Flags to control the process of the transform elements individually when use_global_pose is disabled.

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
