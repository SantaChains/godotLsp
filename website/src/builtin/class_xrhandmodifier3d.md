# XRHandModifier3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **XRHandModifier3D**

> This node uses hand tracking data from an XRHandTracker to pose the skeleton of a hand mesh.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node uses hand tracking data from an XRHandTracker to pose the skeleton of a hand mesh.

Positioning of hands is performed by creating an XRNode3D ancestor of the hand mesh driven by the same XRHandTracker.

The hand tracking position-data is scaled by Skeleton3D.motion_scale when applied to the skeleton, which can be used to adjust the tracked hand to match the scale of the hand model.

</details>

## 方法

### get_bone_update {#get_bone_update}

```gdscript
XRHandModifier3D.BoneUpdate get_bone_update() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hand_tracker {#get_hand_tracker}

```gdscript
StringName get_hand_tracker() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_update {#set_bone_update}

```gdscript
void set_bone_update(bone_update: XRHandModifier3D.BoneUpdate)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hand_tracker {#set_hand_tracker}

```gdscript
void set_hand_tracker(tracker_name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### hand_tracker {#hand_tracker}

```gdscript
hand_tracker: String
```

- void set_hand_tracker(value: StringName)
- StringName get_hand_tracker()

The name of the XRHandTracker registered with XRServer to obtain the hand tracking data from.

### bone_update {#bone_update}

```gdscript
bone_update: int
```

- void set_bone_update(value: BoneUpdate)
- BoneUpdate get_bone_update()

Specifies the type of updates to perform on the bones.

## 枚举

### BoneUpdate {#enum-BoneUpdate}

```gdscript
enum BoneUpdate { BONE_UPDATE_FULL = 0, BONE_UPDATE_ROTATION_ONLY = 1, BONE_UPDATE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_UPDATE_FULL {#BONE_UPDATE_FULL}

```gdscript
BONE_UPDATE_FULL = 0
```

The skeleton's bones are fully updated (both position and rotation) to match the tracked bones.

#### BONE_UPDATE_ROTATION_ONLY {#BONE_UPDATE_ROTATION_ONLY}

```gdscript
BONE_UPDATE_ROTATION_ONLY = 1
```

The skeleton's bones are only rotated to align with the tracked bones, preserving bone length.

#### BONE_UPDATE_MAX {#BONE_UPDATE_MAX}

```gdscript
BONE_UPDATE_MAX = 2
```

Represents the size of the BoneUpdate enum.

---
