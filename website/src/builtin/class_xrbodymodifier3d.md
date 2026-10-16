# XRBodyModifier3D

**继承**：[SkeletonModifier3D](class_skeletonmodifier3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **XRBodyModifier3D**

> This node uses body tracking data from an XRBodyTracker to pose the skeleton of a body mesh.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node uses body tracking data from an XRBodyTracker to pose the skeleton of a body mesh.

Positioning of the body is performed by creating an XRNode3D ancestor of the body mesh driven by the same XRBodyTracker.

The body tracking position-data is scaled by Skeleton3D.motion_scale when applied to the skeleton, which can be used to adjust the tracked body to match the scale of the body model.

</details>

## 方法

### get_body_tracker {#get_body_tracker}

```gdscript
StringName get_body_tracker() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_body_update {#get_body_update}

```gdscript
bitfield::XRBodyModifier3D.BodyUpdate get_body_update() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bone_update {#get_bone_update}

```gdscript
XRBodyModifier3D.BoneUpdate get_bone_update() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_body_tracker {#set_body_tracker}

```gdscript
void set_body_tracker(tracker_name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_body_update {#set_body_update}

```gdscript
void set_body_update(body_update: bitfield::XRBodyModifier3D.BodyUpdate)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_update {#set_bone_update}

```gdscript
void set_bone_update(bone_update: XRBodyModifier3D.BoneUpdate)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### body_tracker {#body_tracker}

```gdscript
body_tracker: String
```

- void set_body_tracker(value: StringName)
- StringName get_body_tracker()

The name of the XRBodyTracker registered with XRServer to obtain the body tracking data from.

### body_update {#body_update}

```gdscript
body_update: int
```

- void set_body_update(value: BitField[BodyUpdate])
- BitField[BodyUpdate] get_body_update()

Specifies the body parts to update.

### bone_update {#bone_update}

```gdscript
bone_update: int
```

- void set_bone_update(value: BoneUpdate)
- BoneUpdate get_bone_update()

Specifies the type of updates to perform on the bones.

## 枚举

### BodyUpdate {#enum-BodyUpdate}

```gdscript
enum BodyUpdate（位标志） { BODY_UPDATE_UPPER_BODY = 1, BODY_UPDATE_LOWER_BODY = 2, BODY_UPDATE_HANDS = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BODY_UPDATE_UPPER_BODY {#BODY_UPDATE_UPPER_BODY}

```gdscript
BODY_UPDATE_UPPER_BODY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BODY_UPDATE_LOWER_BODY {#BODY_UPDATE_LOWER_BODY}

```gdscript
BODY_UPDATE_LOWER_BODY = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BODY_UPDATE_HANDS {#BODY_UPDATE_HANDS}

```gdscript
BODY_UPDATE_HANDS = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### BoneUpdate {#enum-BoneUpdate}

```gdscript
enum BoneUpdate { BONE_UPDATE_FULL = 0, BONE_UPDATE_ROTATION_ONLY = 1, BONE_UPDATE_MAX = 2 }
```

BoneUpdate BONE_UPDATE_FULL = 0

The skeleton's bones are fully updated (both position and rotation) to match the tracked bones.

BoneUpdate BONE_UPDATE_ROTATION_ONLY = 1

The skeleton's bones are only rotated to align with the tracked bones, preserving bone length.

BoneUpdate BONE_UPDATE_MAX = 2

Represents the size of the BoneUpdate enum.

#### BONE_UPDATE_FULL {#BONE_UPDATE_FULL}

```gdscript
BONE_UPDATE_FULL = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_UPDATE_ROTATION_ONLY {#BONE_UPDATE_ROTATION_ONLY}

```gdscript
BONE_UPDATE_ROTATION_ONLY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_UPDATE_MAX {#BONE_UPDATE_MAX}

```gdscript
BONE_UPDATE_MAX = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
