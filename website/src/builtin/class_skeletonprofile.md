# SkeletonProfile

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SkeletonProfile**

> This resource is used in EditorScenePostImport. Some parameters are referring to bones in Skeleton3D, Skin, Animation, and some other nodes are rewritten based on the parameters of SkeletonProfile.

<details><summary>官方英文描述（中文精译进行中）</summary>

This resource is used in EditorScenePostImport. Some parameters are referring to bones in Skeleton3D, Skin, Animation, and some other nodes are rewritten based on the parameters of SkeletonProfile.

Note: These parameters need to be set only when creating a custom profile. In SkeletonProfileHumanoid, they are defined internally as read-only values.

</details>

## 方法

### find_bone {#find_bone}

```gdscript
int find_bone(bone_name: StringName) const
```

Returns the bone index that matches bone_name as its name.

### get_bone_name {#get_bone_name}

```gdscript
StringName get_bone_name(bone_idx: int) const
```

Returns the name of the bone at bone_idx that will be the key name in the BoneMap.

In the retargeting process, the returned bone name is the bone name of the target skeleton.

### get_bone_parent {#get_bone_parent}

```gdscript
StringName get_bone_parent(bone_idx: int) const
```

Returns the name of the bone which is the parent to the bone at bone_idx. The result is empty if the bone has no parent.

### get_bone_size {#get_bone_size}

```gdscript
int get_bone_size()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_tail {#get_bone_tail}

```gdscript
StringName get_bone_tail(bone_idx: int) const
```

Returns the name of the bone which is the tail of the bone at bone_idx.

### get_group {#get_group}

```gdscript
StringName get_group(bone_idx: int) const
```

Returns the group of the bone at bone_idx.

### get_group_name {#get_group_name}

```gdscript
StringName get_group_name(group_idx: int) const
```

Returns the name of the group at group_idx that will be the drawing group in the BoneMap editor.

### get_group_size {#get_group_size}

```gdscript
int get_group_size()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_handle_offset {#get_handle_offset}

```gdscript
Vector2 get_handle_offset(bone_idx: int) const
```

Returns the offset of the bone at bone_idx that will be the button position in the BoneMap editor.

This is the offset with origin at the top left corner of the square.

### get_reference_pose {#get_reference_pose}

```gdscript
Transform3D get_reference_pose(bone_idx: int) const
```

Returns the reference pose transform for bone bone_idx.

### get_root_bone {#get_root_bone}

```gdscript
StringName get_root_bone()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale_base_bone {#get_scale_base_bone}

```gdscript
StringName get_scale_base_bone()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tail_direction {#get_tail_direction}

```gdscript
SkeletonProfile.TailDirection get_tail_direction(bone_idx: int) const
```

Returns the tail direction of the bone at bone_idx.

### get_texture {#get_texture}

```gdscript
Texture2D get_texture(group_idx: int) const
```

Returns the texture of the group at group_idx that will be the drawing group background image in the BoneMap editor.

### is_required {#is_required}

```gdscript
bool is_required(bone_idx: int) const
```

Returns whether the bone at bone_idx is required for retargeting.

This value is used by the bone map editor. If this method returns true, and no bone is assigned, the handle color will be red on the bone map editor.

### set_bone_name {#set_bone_name}

```gdscript
void set_bone_name(bone_idx: int, bone_name: StringName)
```

Sets the name of the bone at bone_idx that will be the key name in the BoneMap.

In the retargeting process, the setting bone name is the bone name of the target skeleton.

### set_bone_parent {#set_bone_parent}

```gdscript
void set_bone_parent(bone_idx: int, bone_parent: StringName)
```

Sets the bone with name bone_parent as the parent of the bone at bone_idx. If an empty string is passed, then the bone has no parent.

### set_bone_size {#set_bone_size}

```gdscript
void set_bone_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_tail {#set_bone_tail}

```gdscript
void set_bone_tail(bone_idx: int, bone_tail: StringName)
```

Sets the bone with name bone_tail as the tail of the bone at bone_idx.

### set_group {#set_group}

```gdscript
void set_group(bone_idx: int, group: StringName)
```

Sets the group of the bone at bone_idx.

### set_group_name {#set_group_name}

```gdscript
void set_group_name(group_idx: int, group_name: StringName)
```

Sets the name of the group at group_idx that will be the drawing group in the BoneMap editor.

### set_group_size {#set_group_size}

```gdscript
void set_group_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_handle_offset {#set_handle_offset}

```gdscript
void set_handle_offset(bone_idx: int, handle_offset: Vector2)
```

Sets the offset of the bone at bone_idx that will be the button position in the BoneMap editor.

This is the offset with origin at the top left corner of the square.

### set_reference_pose {#set_reference_pose}

```gdscript
void set_reference_pose(bone_idx: int, bone_name: Transform3D)
```

Sets the reference pose transform for bone bone_idx.

### set_required {#set_required}

```gdscript
void set_required(bone_idx: int, required: bool)
```

Sets the required status for bone bone_idx to required.

### set_root_bone {#set_root_bone}

```gdscript
void set_root_bone(bone_name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale_base_bone {#set_scale_base_bone}

```gdscript
void set_scale_base_bone(bone_name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tail_direction {#set_tail_direction}

```gdscript
void set_tail_direction(bone_idx: int, tail_direction: SkeletonProfile.TailDirection)
```

Sets the tail direction of the bone at bone_idx.

Note: This only specifies the method of calculation. The actual coordinates required should be stored in an external skeleton, so the calculation itself needs to be done externally.

### set_texture {#set_texture}

```gdscript
void set_texture(group_idx: int, texture: Texture2D)
```

Sets the texture of the group at group_idx that will be the drawing group background image in the BoneMap editor.

## 属性

### root_bone {#root_bone}

```gdscript
root_bone: StringName
```

- void set_root_bone(value: StringName)
- StringName get_root_bone()

A bone name that will be used as the root bone in AnimationTree. This should be the bone of the parent of hips that exists at the world origin.

### scale_base_bone {#scale_base_bone}

```gdscript
scale_base_bone: StringName
```

- void set_scale_base_bone(value: StringName)
- StringName get_scale_base_bone()

A bone name which will use model's height as the coefficient for normalization. For example, SkeletonProfileHumanoid defines it as Hips.

### group_size {#group_size}

```gdscript
group_size: int
```

- void set_group_size(value: int)
- int get_group_size()

The amount of groups of bones in retargeting section's BoneMap editor. For example, SkeletonProfileHumanoid has 4 groups.

This property exists to separate the bone list into several sections in the editor.

### bone_size {#bone_size}

```gdscript
bone_size: int
```

- void set_bone_size(value: int)
- int get_bone_size()

The amount of bones in retargeting section's BoneMap editor. For example, SkeletonProfileHumanoid has 56 bones.

The size of elements in BoneMap updates when changing this property in it's assigned SkeletonProfile.

## 信号

### profile_updated {#profile_updated}

```gdscript
signal profile_updated()
```

This signal is emitted when change the value in profile. This is used to update key name in the BoneMap and to redraw the BoneMap editor.

Note: This signal is not connected directly to editor to simplify the reference, instead it is passed on to editor through the BoneMap.

## 枚举

### TailDirection {#enum-TailDirection}

```gdscript
enum TailDirection { TAIL_DIRECTION_AVERAGE_CHILDREN = 0, TAIL_DIRECTION_SPECIFIC_CHILD = 1, TAIL_DIRECTION_END = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TAIL_DIRECTION_AVERAGE_CHILDREN {#TAIL_DIRECTION_AVERAGE_CHILDREN}

```gdscript
TAIL_DIRECTION_AVERAGE_CHILDREN = 0
```

Direction to the average coordinates of bone children.

#### TAIL_DIRECTION_SPECIFIC_CHILD {#TAIL_DIRECTION_SPECIFIC_CHILD}

```gdscript
TAIL_DIRECTION_SPECIFIC_CHILD = 1
```

Direction to the coordinates of specified bone child.

#### TAIL_DIRECTION_END {#TAIL_DIRECTION_END}

```gdscript
TAIL_DIRECTION_END = 2
```

Direction is not calculated.

---
