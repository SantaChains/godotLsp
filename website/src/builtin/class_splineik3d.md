# SplineIK3D

**继承**：[ChainIK3D](class_chainik3d.md) → [IKModifier3D](class_ikmodifier3d.md) → [SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SplineIK3D**

> A SkeletonModifier3D for aligning bones along a Path3D. The smoothness of the fitting depends on the Curve3D.bake_interval.

<details><summary>官方英文描述</summary>

A SkeletonModifier3D for aligning bones along a Path3D. The smoothness of the fitting depends on the Curve3D.bake_interval.

If you want the Path3D to attach to a specific bone, it is recommended to place a ModifierBoneTarget3D before the SplineIK3D in the SkeletonModifier3D list (children of the Skeleton3D), and then place a Path3D as the ModifierBoneTarget3D's child.

Bone twist is determined based on the Curve3D.get_point_tilt().

If the root bone joint and the start point of the Curve3D are separated, it assumes that there is a linear line segment between them. This means that the vector pointing toward the start point of the Curve3D takes precedence over the shortest intersection point along the Curve3D.

If the end bone joint exceeds the path length, it is bent as close as possible to the end point of the Curve3D.

Note: All the methods in this class take an index parameter. This parameter specifies which setting list entry to return if the IK has multiple entries (e.g. settings//root_bone_name).

</details>

## 方法

### get_path_3d {#get_path_3d}

```gdscript
NodePath get_path_3d(index: int) const
```

Returns the node path of the Path3D which is describing the path.

### get_tilt_fade_in {#get_tilt_fade_in}

```gdscript
int get_tilt_fade_in(index: int) const
```

Returns the tilt interpolation method used between the root bone and the start point of the Curve3D when they are apart. See also set_tilt_fade_in().

### get_tilt_fade_out {#get_tilt_fade_out}

```gdscript
int get_tilt_fade_out(index: int) const
```

Returns the tilt interpolation method used between the end bone and the end point of the Curve3D when they are apart. See also set_tilt_fade_out().

### is_tilt_enabled {#is_tilt_enabled}

```gdscript
bool is_tilt_enabled(index: int) const
```

Returns if the tilt property of the Curve3D affects the bone twist.

### set_path_3d {#set_path_3d}

```gdscript
void set_path_3d(index: int, path_3d: NodePath)
```

Sets the node path of the Path3D which is describing the path.

### set_tilt_enabled {#set_tilt_enabled}

```gdscript
void set_tilt_enabled(index: int, enabled: bool)
```

Sets if the tilt property of the Curve3D should affect the bone twist.

### set_tilt_fade_in {#set_tilt_fade_in}

```gdscript
void set_tilt_fade_in(index: int, size: int)
```

If size is greater than 0, the tilt is interpolated between size start bones from the start point of the Curve3D when they are apart.

If size is equal 0, the tilts between the root bone head and the start point of the Curve3D are unified with a tilt of the start point of the Curve3D.

If size is less than 0, the tilts between the root bone and the start point of the Curve3D are 0.0.

### set_tilt_fade_out {#set_tilt_fade_out}

```gdscript
void set_tilt_fade_out(index: int, size: int)
```

If size is greater than 0, the tilt is interpolated between size end bones from the end point of the Curve3D when they are apart.

If size is equal 0, the tilts between the end bone tail and the end point of the Curve3D are unified with a tilt of the end point of the Curve3D.

If size is less than 0, the tilts between the end bone and the end point of the Curve3D are 0.0.

## 属性

### setting_count {#setting_count}

```gdscript
setting_count: int
```

- void set_setting_count(value: int)
- int get_setting_count()

The number of settings.
