# BoneConstraint3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **BoneConstraint3D**

> Base class of SkeletonModifier3D that modifies the bone set in set_apply_bone() based on the transform of the bone retrieved by get_reference_bone().

<details><summary>官方英文描述（中文精译进行中）</summary>

Base class of SkeletonModifier3D that modifies the bone set in set_apply_bone() based on the transform of the bone retrieved by get_reference_bone().

Note: Most methods in this class take an index parameter. This parameter specifies which setting list entry to return if the IK has multiple entries (e.g. settings//amount).

</details>

## 方法

### clear_setting {#clear_setting}

```gdscript
void clear_setting()
```

Clear all settings.

### get_amount {#get_amount}

```gdscript
float get_amount(index: int) const
```

Returns the apply amount of the setting at index.

### get_apply_bone {#get_apply_bone}

```gdscript
int get_apply_bone(index: int) const
```

Returns the apply bone of the setting at index. This bone will be modified.

### get_apply_bone_name {#get_apply_bone_name}

```gdscript
String get_apply_bone_name(index: int) const
```

Returns the apply bone name of the setting at index. This bone will be modified.

### get_reference_bone {#get_reference_bone}

```gdscript
int get_reference_bone(index: int) const
```

Returns the reference bone of the setting at index.

This bone will be only referenced and not modified by this modifier.

### get_reference_bone_name {#get_reference_bone_name}

```gdscript
String get_reference_bone_name(index: int) const
```

Returns the reference bone name of the setting at index.

This bone will be only referenced and not modified by this modifier.

### get_reference_node {#get_reference_node}

```gdscript
NodePath get_reference_node(index: int) const
```

Returns the reference node path of the setting at index.

This node will be only referenced and not modified by this modifier.

### get_reference_type {#get_reference_type}

```gdscript
BoneConstraint3D.ReferenceType get_reference_type(index: int) const
```

Returns the reference target type of the setting at index. See also ReferenceType.

### get_setting_count {#get_setting_count}

```gdscript
int get_setting_count() const
```

Returns the number of settings in the modifier.

### set_amount {#set_amount}

```gdscript
void set_amount(index: int, amount: float)
```

Sets the apply amount of the setting at index to amount.

### set_apply_bone {#set_apply_bone}

```gdscript
void set_apply_bone(index: int, bone: int)
```

Sets the apply bone of the setting at index to bone. This bone will be modified.

### set_apply_bone_name {#set_apply_bone_name}

```gdscript
void set_apply_bone_name(index: int, bone_name: String)
```

Sets the apply bone of the setting at index to bone_name. This bone will be modified.

### set_reference_bone {#set_reference_bone}

```gdscript
void set_reference_bone(index: int, bone: int)
```

Sets the reference bone of the setting at index to bone.

This bone will be only referenced and not modified by this modifier.

### set_reference_bone_name {#set_reference_bone_name}

```gdscript
void set_reference_bone_name(index: int, bone_name: String)
```

Sets the reference bone of the setting at index to bone_name.

This bone will be only referenced and not modified by this modifier.

### set_reference_node {#set_reference_node}

```gdscript
void set_reference_node(index: int, node: NodePath)
```

Sets the reference node path of the setting at index to node.

This node will be only referenced and not modified by this modifier.

### set_reference_type {#set_reference_type}

```gdscript
void set_reference_type(index: int, type: BoneConstraint3D.ReferenceType)
```

Sets the reference target type of the setting at index to type. See also ReferenceType.

### set_setting_count {#set_setting_count}

```gdscript
void set_setting_count(count: int)
```

Sets the number of settings in the modifier.

## 枚举

### ReferenceType {#enum-ReferenceType}

```gdscript
enum ReferenceType { REFERENCE_TYPE_BONE = 0, REFERENCE_TYPE_NODE = 1 }
```

ReferenceType REFERENCE_TYPE_BONE = 0

The reference target is a bone. In this case, the reference target spaces is local space.

ReferenceType REFERENCE_TYPE_NODE = 1

The reference target is a Node3D. In this case, the reference target spaces is model space.

In other words, the reference target's coordinates are treated as if it were placed directly under Skeleton3D which parent of the BoneConstraint3D.

#### REFERENCE_TYPE_BONE {#REFERENCE_TYPE_BONE}

```gdscript
REFERENCE_TYPE_BONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### REFERENCE_TYPE_NODE {#REFERENCE_TYPE_NODE}

```gdscript
REFERENCE_TYPE_NODE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
