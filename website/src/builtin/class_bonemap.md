# BoneMap

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **BoneMap**

> This class contains a dictionary that uses a list of bone names in SkeletonProfile as key names.

<details><summary>官方英文描述</summary>

This class contains a dictionary that uses a list of bone names in SkeletonProfile as key names.

By assigning the actual Skeleton3D bone name as the key value, it maps the Skeleton3D to the SkeletonProfile.

</details>

## 方法

### find_profile_bone_name {#find_profile_bone_name}

```gdscript
StringName find_profile_bone_name(skeleton_bone_name: StringName) const
```

Returns a profile bone name having skeleton_bone_name. If not found, an empty StringName will be returned.

In the retargeting process, the returned bone name is the bone name of the target skeleton.

### get_profile {#get_profile}

```gdscript
SkeletonProfile get_profile() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton_bone_name {#get_skeleton_bone_name}

```gdscript
StringName get_skeleton_bone_name(profile_bone_name: StringName) const
```

Returns a skeleton bone name is mapped to profile_bone_name.

In the retargeting process, the returned bone name is the bone name of the source skeleton.

### set_profile {#set_profile}

```gdscript
void set_profile(profile: SkeletonProfile)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skeleton_bone_name {#set_skeleton_bone_name}

```gdscript
void set_skeleton_bone_name(profile_bone_name: StringName, skeleton_bone_name: StringName)
```

Maps a skeleton bone name to profile_bone_name.

In the retargeting process, the setting bone name is the bone name of the source skeleton.

## 属性

### profile {#profile}

```gdscript
profile: SkeletonProfile
```

- void set_profile(value: SkeletonProfile)
- SkeletonProfile get_profile()

A SkeletonProfile of the mapping target. Key names in the BoneMap are synchronized with it.

## 信号

### bone_map_updated {#bone_map_updated}

```gdscript
signal bone_map_updated()
```

This signal is emitted when change the key value in the BoneMap. This is used to validate mapping and to update BoneMap editor.

### profile_updated {#profile_updated}

```gdscript
signal profile_updated()
```

This signal is emitted when change the value in profile or change the reference of profile. This is used to update key names in the BoneMap and to redraw the BoneMap editor.
