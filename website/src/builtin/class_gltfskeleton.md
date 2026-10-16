# GLTFSkeleton

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFSkeleton**

## 方法

### get_bone_attachment {#get_bone_attachment}

```gdscript
BoneAttachment3D get_bone_attachment(idx: int)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_bone_attachment_count {#get_bone_attachment_count}

```gdscript
int get_bone_attachment_count()
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_godot_bone_node {#get_godot_bone_node}

```gdscript
Dictionary get_godot_bone_node()
```

Returns a Dictionary that maps skeleton bone indices to the indices of glTF nodes. This property is unused during import, and only set during export. In a glTF file, a bone is a node, so Godot converts skeleton bones to glTF nodes.

### get_godot_skeleton {#get_godot_skeleton}

```gdscript
Skeleton3D get_godot_skeleton()
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_joints {#get_joints}

```gdscript
PackedInt32Array get_joints()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_roots {#get_roots}

```gdscript
PackedInt32Array get_roots()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_unique_names {#get_unique_names}

```gdscript
typedarray::String get_unique_names()
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_godot_bone_node {#set_godot_bone_node}

```gdscript
void set_godot_bone_node(godot_bone_node: Dictionary)
```

Sets a Dictionary that maps skeleton bone indices to the indices of glTF nodes. This property is unused during import, and only set during export. In a glTF file, a bone is a node, so Godot converts skeleton bones to glTF nodes.

### set_joints {#set_joints}

```gdscript
void set_joints(joints: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_roots {#set_roots}

```gdscript
void set_roots(roots: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_unique_names {#set_unique_names}

```gdscript
void set_unique_names(unique_names: typedarray::String)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

## 属性

### joints {#joints}

```gdscript
joints: PackedInt32Array
```

- void set_joints(value: PackedInt32Array)
- PackedInt32Array get_joints()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### roots {#roots}

```gdscript
roots: PackedInt32Array
```

- void set_roots(value: PackedInt32Array)
- PackedInt32Array get_roots()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### unique_names {#unique_names}

```gdscript
unique_names: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### godot_bone_node {#godot_bone_node}

```gdscript
godot_bone_node: Dictionary
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
