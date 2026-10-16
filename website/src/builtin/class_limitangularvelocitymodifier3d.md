# LimitAngularVelocityModifier3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **LimitAngularVelocityModifier3D**

> This modifier limits bone rotation angular velocity by comparing poses between previous and current frame.

<details><summary>官方英文描述</summary>

This modifier limits bone rotation angular velocity by comparing poses between previous and current frame.

You can add bone chains by specifying their root and end bones, then add the bones between them to a list. Modifier processes either that list or the bones excluding those in the list depending on the option exclude.

Note: Most methods in this class take an index parameter. This parameter specifies which setting list entry to return if the IK has multiple entries (e.g. settings//root_bone_name).

</details>

## 方法

### clear_chains {#clear_chains}

```gdscript
void clear_chains()
```

Clear all chains.

### get_chain_count {#get_chain_count}

```gdscript
int get_chain_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_end_bone {#get_end_bone}

```gdscript
int get_end_bone(index: int) const
```

Returns the end bone index of the bone chain.

### get_end_bone_name {#get_end_bone_name}

```gdscript
String get_end_bone_name(index: int) const
```

Returns the end bone name of the bone chain.

### get_max_angular_velocity {#get_max_angular_velocity}

```gdscript
float get_max_angular_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_root_bone {#get_root_bone}

```gdscript
int get_root_bone(index: int) const
```

Returns the root bone index of the bone chain.

### get_root_bone_name {#get_root_bone_name}

```gdscript
String get_root_bone_name(index: int) const
```

Returns the root bone name of the bone chain.

### is_exclude {#is_exclude}

```gdscript
bool is_exclude() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### reset {#reset}

```gdscript
void reset()
```

Sets the reference pose for angle comparison to the current pose with the influence of constraints removed. This function is automatically triggered when joints change or upon activation.

### set_chain_count {#set_chain_count}

```gdscript
void set_chain_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_end_bone {#set_end_bone}

```gdscript
void set_end_bone(index: int, bone: int)
```

Sets the end bone index of the bone chain.

### set_end_bone_name {#set_end_bone_name}

```gdscript
void set_end_bone_name(index: int, bone_name: String)
```

Sets the end bone name of the bone chain.

Note: End bone must be the root bone or a child of the root bone.

### set_exclude {#set_exclude}

```gdscript
void set_exclude(exclude: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_angular_velocity {#set_max_angular_velocity}

```gdscript
void set_max_angular_velocity(angular_velocity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_root_bone {#set_root_bone}

```gdscript
void set_root_bone(index: int, bone: int)
```

Sets the root bone index of the bone chain.

### set_root_bone_name {#set_root_bone_name}

```gdscript
void set_root_bone_name(index: int, bone_name: String)
```

Sets the root bone name of the bone chain.

## 属性

### max_angular_velocity {#max_angular_velocity}

```gdscript
max_angular_velocity: float
```

- void set_max_angular_velocity(value: float)
- float get_max_angular_velocity()

The maximum angular velocity per second.

### exclude {#exclude}

```gdscript
exclude: bool
```

- void set_exclude(value: bool)
- bool is_exclude()

If true, the modifier processes bones not included in the bone list.

If false, the bones processed by the modifier are equal to the bone list.

### chain_count {#chain_count}

```gdscript
chain_count: int
```

- void set_chain_count(value: int)
- int get_chain_count()

The number of chains.

### joint_count {#joint_count}

```gdscript
joint_count: int
```

The number of joints in the list which created by chains dynamically.
