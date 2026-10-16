# SpringBoneCollision3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **SpringBoneCollision3D**

> A collision can be a child of SpringBoneSimulator3D. If it is not a child of SpringBoneSimulator3D, it has no effect.

<details><summary>官方英文描述（中文精译进行中）</summary>

A collision can be a child of SpringBoneSimulator3D. If it is not a child of SpringBoneSimulator3D, it has no effect.

The colliding and sliding are done in the SpringBoneSimulator3D's modification process in order of its collision list which is set by SpringBoneSimulator3D.set_collision_path(). If SpringBoneSimulator3D.are_all_child_collisions_enabled() is true, the order matches SceneTree.

If bone is set, it synchronizes with the bone pose of the ancestor Skeleton3D, which is done in before the SpringBoneSimulator3D's modification process as the pre-process.

Warning: A scaled SpringBoneCollision3D will likely not behave as expected. Make sure that the parent Skeleton3D and its bones are not scaled.

</details>

## 方法

### get_bone {#get_bone}

```gdscript
int get_bone() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_name {#get_bone_name}

```gdscript
String get_bone_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position_offset {#get_position_offset}

```gdscript
Vector3 get_position_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation_offset {#get_rotation_offset}

```gdscript
Quaternion get_rotation_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton {#get_skeleton}

```gdscript
Skeleton3D get_skeleton() const
```

Get parent Skeleton3D node of the parent SpringBoneSimulator3D if found.

### set_bone {#set_bone}

```gdscript
void set_bone(bone: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_name {#set_bone_name}

```gdscript
void set_bone_name(bone_name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position_offset {#set_position_offset}

```gdscript
void set_position_offset(offset: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_offset {#set_rotation_offset}

```gdscript
void set_rotation_offset(offset: Quaternion)
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

### bone {#bone}

```gdscript
bone: int
```

- void set_bone(value: int)
- int get_bone()

The index of the attached bone.

### position_offset {#position_offset}

```gdscript
position_offset: Vector3
```

- void set_position_offset(value: Vector3)
- Vector3 get_position_offset()

The offset of the position from Skeleton3D's bone pose position.

### rotation_offset {#rotation_offset}

```gdscript
rotation_offset: Quaternion
```

- void set_rotation_offset(value: Quaternion)
- Quaternion get_rotation_offset()

The offset of the rotation from Skeleton3D's bone pose rotation.
