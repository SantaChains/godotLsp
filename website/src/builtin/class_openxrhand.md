# OpenXRHand

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **OpenXRHand**

> This node enables OpenXR's hand tracking functionality. The node should be a child node of an XROrigin3D node, tracking will update its position to the player's tracked hand Palm joint location (the center of the middle finger's metacarpal bone). This node also updates the skeleton of a properly skinned hand or avatar model.

<details><summary>官方英文描述</summary>

This node enables OpenXR's hand tracking functionality. The node should be a child node of an XROrigin3D node, tracking will update its position to the player's tracked hand Palm joint location (the center of the middle finger's metacarpal bone). This node also updates the skeleton of a properly skinned hand or avatar model.

If the skeleton is a hand (one of the hand bones is the root node of the skeleton), then the skeleton will be placed relative to the hand palm location and the hand mesh and skeleton should be children of the OpenXRHand node.

If the hand bones are part of a full skeleton, then the root of the hand will keep its location with the assumption that IK is used to position the hand and arm.

By default the skeleton hand bones are repositioned to match the size of the tracked hand. To preserve the modeled bone sizes change bone_update to apply rotation only.

</details>

## 方法

### get_bone_update {#get_bone_update}

```gdscript
OpenXRHand.BoneUpdate get_bone_update() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hand {#get_hand}

```gdscript
OpenXRHand.Hands get_hand() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hand_skeleton {#get_hand_skeleton}

```gdscript
NodePath get_hand_skeleton() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_motion_range {#get_motion_range}

```gdscript
OpenXRHand.MotionRange get_motion_range() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton_rig {#get_skeleton_rig}

```gdscript
OpenXRHand.SkeletonRig get_skeleton_rig() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bone_update {#set_bone_update}

```gdscript
void set_bone_update(bone_update: OpenXRHand.BoneUpdate)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hand {#set_hand}

```gdscript
void set_hand(hand: OpenXRHand.Hands)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hand_skeleton {#set_hand_skeleton}

```gdscript
void set_hand_skeleton(hand_skeleton: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_motion_range {#set_motion_range}

```gdscript
void set_motion_range(motion_range: OpenXRHand.MotionRange)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skeleton_rig {#set_skeleton_rig}

```gdscript
void set_skeleton_rig(skeleton_rig: OpenXRHand.SkeletonRig)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### hand {#hand}

```gdscript
hand: int
```

- void set_hand(value: Hands)
- Hands get_hand()

Specifies whether this node tracks the left or right hand of the player.

### motion_range {#motion_range}

```gdscript
motion_range: int
```

- void set_motion_range(value: MotionRange)
- MotionRange get_motion_range()

Set the motion range (if supported) limiting the hand motion.

### hand_skeleton {#hand_skeleton}

```gdscript
hand_skeleton: NodePath
```

- void set_hand_skeleton(value: NodePath)
- NodePath get_hand_skeleton()

Set a Skeleton3D node for which the pose positions will be updated.

### skeleton_rig {#skeleton_rig}

```gdscript
skeleton_rig: int
```

- void set_skeleton_rig(value: SkeletonRig)
- SkeletonRig get_skeleton_rig()

Set the type of skeleton rig the hand_skeleton is compliant with.

### bone_update {#bone_update}

```gdscript
bone_update: int
```

- void set_bone_update(value: BoneUpdate)
- BoneUpdate get_bone_update()

Specify the type of updates to perform on the bone.

## 枚举

### Hands {#enum-Hands}

```gdscript
enum Hands { HAND_LEFT = 0, HAND_RIGHT = 1, HAND_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_LEFT {#HAND_LEFT}

```gdscript
HAND_LEFT = 0
```

Tracking the player's left hand.

#### HAND_RIGHT {#HAND_RIGHT}

```gdscript
HAND_RIGHT = 1
```

Tracking the player's right hand.

#### HAND_MAX {#HAND_MAX}

```gdscript
HAND_MAX = 2
```

Maximum supported hands.

---

### MotionRange {#enum-MotionRange}

```gdscript
enum MotionRange { MOTION_RANGE_UNOBSTRUCTED = 0, MOTION_RANGE_CONFORM_TO_CONTROLLER = 1, MOTION_RANGE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MOTION_RANGE_UNOBSTRUCTED {#MOTION_RANGE_UNOBSTRUCTED}

```gdscript
MOTION_RANGE_UNOBSTRUCTED = 0
```

When player grips, hand skeleton will form a full fist.

#### MOTION_RANGE_CONFORM_TO_CONTROLLER {#MOTION_RANGE_CONFORM_TO_CONTROLLER}

```gdscript
MOTION_RANGE_CONFORM_TO_CONTROLLER = 1
```

When player grips, hand skeleton conforms to the controller the player is holding.

#### MOTION_RANGE_MAX {#MOTION_RANGE_MAX}

```gdscript
MOTION_RANGE_MAX = 2
```

Maximum supported motion ranges.

---

### SkeletonRig {#enum-SkeletonRig}

```gdscript
enum SkeletonRig { SKELETON_RIG_OPENXR = 0, SKELETON_RIG_HUMANOID = 1, SKELETON_RIG_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SKELETON_RIG_OPENXR {#SKELETON_RIG_OPENXR}

```gdscript
SKELETON_RIG_OPENXR = 0
```

An OpenXR compliant skeleton.

#### SKELETON_RIG_HUMANOID {#SKELETON_RIG_HUMANOID}

```gdscript
SKELETON_RIG_HUMANOID = 1
```

A SkeletonProfileHumanoid compliant skeleton.

#### SKELETON_RIG_MAX {#SKELETON_RIG_MAX}

```gdscript
SKELETON_RIG_MAX = 2
```

Maximum supported hands.

---

### BoneUpdate {#enum-BoneUpdate}

```gdscript
enum BoneUpdate { BONE_UPDATE_FULL = 0, BONE_UPDATE_ROTATION_ONLY = 1, BONE_UPDATE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BONE_UPDATE_FULL {#BONE_UPDATE_FULL}

```gdscript
BONE_UPDATE_FULL = 0
```

The skeletons bones are fully updated (both position and rotation) to match the tracked bones.

#### BONE_UPDATE_ROTATION_ONLY {#BONE_UPDATE_ROTATION_ONLY}

```gdscript
BONE_UPDATE_ROTATION_ONLY = 1
```

The skeletons bones are only rotated to align with the tracked bones, preserving bone length.

#### BONE_UPDATE_MAX {#BONE_UPDATE_MAX}

```gdscript
BONE_UPDATE_MAX = 2
```

Maximum supported bone update mode.

---
