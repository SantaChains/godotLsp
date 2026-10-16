# ModifierBoneTarget3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **ModifierBoneTarget3D**

> This node selects a bone in a Skeleton3D and attaches to it. This means that the ModifierBoneTarget3D node will dynamically copy the 3D transform of the selected bone.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node selects a bone in a Skeleton3D and attaches to it. This means that the ModifierBoneTarget3D node will dynamically copy the 3D transform of the selected bone.

The functionality is similar to BoneAttachment3D, but this node adopts the SkeletonModifier3D cycle and is intended to be used as another SkeletonModifier3D's target.

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

## 属性

### bone_name {#bone_name}

```gdscript
bone_name: String
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
