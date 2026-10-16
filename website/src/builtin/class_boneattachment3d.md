# BoneAttachment3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **BoneAttachment3D**

> This node selects a bone in a Skeleton3D and attaches to it. This means that the BoneAttachment3D node will either dynamically copy or override the 3D transform of the selected bone.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node selects a bone in a Skeleton3D and attaches to it. This means that the BoneAttachment3D node will either dynamically copy or override the 3D transform of the selected bone.

</details>

## 方法

### get_bone_idx {#get_bone_idx}

```gdscript
int get_bone_idx() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_name {#get_bone_name}

```gdscript
String get_bone_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_external_skeleton {#get_external_skeleton}

```gdscript
NodePath get_external_skeleton() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_override_pose {#get_override_pose}

```gdscript
bool get_override_pose() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton {#get_skeleton}

```gdscript
Skeleton3D get_skeleton()
```

Returns the parent or external Skeleton3D node if it exists, otherwise returns null.

### get_use_external_skeleton {#get_use_external_skeleton}

```gdscript
bool get_use_external_skeleton() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### on_skeleton_update {#on_skeleton_update}

```gdscript
void on_skeleton_update()
```

A function that is called automatically when the Skeleton3D is updated. This function is where the BoneAttachment3D node updates its position so it is correctly bound when it is not set to override the bone pose.

### set_bone_idx {#set_bone_idx}

```gdscript
void set_bone_idx(bone_idx: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_name {#set_bone_name}

```gdscript
void set_bone_name(bone_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_external_skeleton {#set_external_skeleton}

```gdscript
void set_external_skeleton(external_skeleton: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_override_pose {#set_override_pose}

```gdscript
void set_override_pose(override_pose: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_external_skeleton {#set_use_external_skeleton}

```gdscript
void set_use_external_skeleton(use_external_skeleton: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### bone_name {#bone_name}

```gdscript
bone_name: StringName
```

- void set_bone_name(value: String)
- String get_bone_name()

The name of the attached bone.

### bone_idx {#bone_idx}

```gdscript
bone_idx: int
```

- void set_bone_idx(value: int)
- int get_bone_idx()

The index of the attached bone.

### override_pose {#override_pose}

```gdscript
override_pose: bool
```

- void set_override_pose(value: bool)
- bool get_override_pose()

Whether the BoneAttachment3D node will override the bone pose of the bone it is attached to. When set to true, the BoneAttachment3D node can change the pose of the bone. When set to false, the BoneAttachment3D will always be set to the bone's transform.

Note: This override performs interruptively in the skeleton update process using signals due to the old design. It may cause unintended behavior when used at the same time with SkeletonModifier3D.

### use_external_skeleton {#use_external_skeleton}

```gdscript
use_external_skeleton: bool
```

- void set_use_external_skeleton(value: bool)
- bool get_use_external_skeleton()

Whether the BoneAttachment3D node will use an external Skeleton3D node rather than attempting to use its parent node as the Skeleton3D. When set to true, the BoneAttachment3D node will use the external Skeleton3D node set in external_skeleton.

### external_skeleton {#external_skeleton}

```gdscript
external_skeleton: NodePath
```

- void set_external_skeleton(value: NodePath)
- NodePath get_external_skeleton()

The NodePath to the external Skeleton3D node.
