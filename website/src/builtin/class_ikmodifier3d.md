# IKModifier3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **IKModifier3D**

> Base class of SkeletonModifier3Ds that has some joint lists and applies inverse kinematics. This class has some structs, enums, and helper methods which are useful to solve inverse kinematics.

<details><summary>官方英文描述</summary>

Base class of SkeletonModifier3Ds that has some joint lists and applies inverse kinematics. This class has some structs, enums, and helper methods which are useful to solve inverse kinematics.

</details>

## 方法

### are_bone_axes_mutable {#are_bone_axes_mutable}

```gdscript
bool are_bone_axes_mutable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### clear_settings {#clear_settings}

```gdscript
void clear_settings()
```

Clears all settings.

### get_setting_count {#get_setting_count}

```gdscript
int get_setting_count() const
```

Returns the number of settings.

### reset {#reset}

```gdscript
void reset()
```

Resets a state with respect to the current bone pose.

### set_mutable_bone_axes {#set_mutable_bone_axes}

```gdscript
void set_mutable_bone_axes(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_setting_count {#set_setting_count}

```gdscript
void set_setting_count(count: int)
```

Sets the number of settings.

## 属性

### mutable_bone_axes {#mutable_bone_axes}

```gdscript
mutable_bone_axes: bool
```

- void set_mutable_bone_axes(value: bool)
- bool are_bone_axes_mutable()

If true, the solver retrieves the bone axis from the bone pose every frame.

If false, the solver retrieves the bone axis from the bone rest and caches it, which increases performance slightly, but position changes in the bone pose made before processing this IKModifier3D are ignored.
