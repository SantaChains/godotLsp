# XRBodyTracker

**继承**：[XRPositionalTracker](class_xrpositionaltracker.md) → [XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **XRBodyTracker**

> A body tracking system will create an instance of this object and add it to the XRServer. This tracking system will then obtain skeleton data, convert it to the Godot Humanoid skeleton and store this data on the XRBodyTracker object.

<details><summary>官方英文描述</summary>

A body tracking system will create an instance of this object and add it to the XRServer. This tracking system will then obtain skeleton data, convert it to the Godot Humanoid skeleton and store this data on the XRBodyTracker object.

Use XRBodyModifier3D to animate a body mesh using body tracking data.

</details>

## 方法

### get_body_flags {#get_body_flags}

```gdscript
bitfield::XRBodyTracker.BodyFlags get_body_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_has_tracking_data {#get_has_tracking_data}

```gdscript
bool get_has_tracking_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_joint_flags {#get_joint_flags}

```gdscript
bitfield::XRBodyTracker.JointFlags get_joint_flags(joint: XRBodyTracker.Joint) const
```

Returns flags about the validity of the tracking data for the given body joint.

### get_joint_transform {#get_joint_transform}

```gdscript
Transform3D get_joint_transform(joint: XRBodyTracker.Joint) const
```

Returns the transform for the given body joint.

### set_body_flags {#set_body_flags}

```gdscript
void set_body_flags(flags: bitfield::XRBodyTracker.BodyFlags)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_has_tracking_data {#set_has_tracking_data}

```gdscript
void set_has_tracking_data(has_data: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_joint_flags {#set_joint_flags}

```gdscript
void set_joint_flags(joint: XRBodyTracker.Joint, flags: bitfield::XRBodyTracker.JointFlags)
```

Sets flags about the validity of the tracking data for the given body joint.

### set_joint_transform {#set_joint_transform}

```gdscript
void set_joint_transform(joint: XRBodyTracker.Joint, transform: Transform3D)
```

Sets the transform for the given body joint.

## 属性

### has_tracking_data {#has_tracking_data}

```gdscript
has_tracking_data: bool
```

- void set_has_tracking_data(value: bool)
- bool get_has_tracking_data()

If true, the body tracking data is valid.

### body_flags {#body_flags}

```gdscript
body_flags: int
```

- void set_body_flags(value: BitField[BodyFlags])
- BitField[BodyFlags] get_body_flags()

The type of body tracking data captured.

## 枚举

### BodyFlags {#enum-BodyFlags}

```gdscript
enum BodyFlags（位标志） { BODY_FLAG_UPPER_BODY_SUPPORTED = 1, BODY_FLAG_LOWER_BODY_SUPPORTED = 2, BODY_FLAG_HANDS_SUPPORTED = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BODY_FLAG_UPPER_BODY_SUPPORTED {#BODY_FLAG_UPPER_BODY_SUPPORTED}

```gdscript
BODY_FLAG_UPPER_BODY_SUPPORTED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BODY_FLAG_LOWER_BODY_SUPPORTED {#BODY_FLAG_LOWER_BODY_SUPPORTED}

```gdscript
BODY_FLAG_LOWER_BODY_SUPPORTED = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BODY_FLAG_HANDS_SUPPORTED {#BODY_FLAG_HANDS_SUPPORTED}

```gdscript
BODY_FLAG_HANDS_SUPPORTED = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Joint {#enum-Joint}

```gdscript
enum Joint { JOINT_ROOT = 0, JOINT_HIPS = 1, JOINT_SPINE = 2, JOINT_CHEST = 3, JOINT_UPPER_CHEST = 4, JOINT_NECK = 5, JOINT_HEAD = 6, JOINT_HEAD_TIP = 7, JOINT_LEFT_SHOULDER = 8, JOINT_LEFT_UPPER_ARM = 9, JOINT_LEFT_LOWER_ARM = 10, JOINT_RIGHT_SHOULDER = 11, JOINT_RIGHT_UPPER_ARM = 12, JOINT_RIGHT_LOWER_ARM = 13, JOINT_LEFT_UPPER_LEG = 14, JOINT_LEFT_LOWER_LEG = 15, JOINT_LEFT_FOOT = 16, JOINT_LEFT_TOES = 17, JOINT_RIGHT_UPPER_LEG = 18, JOINT_RIGHT_LOWER_LEG = 19, JOINT_RIGHT_FOOT = 20, JOINT_RIGHT_TOES = 21, JOINT_LEFT_HAND = 22, JOINT_LEFT_PALM = 23, JOINT_LEFT_WRIST = 24, JOINT_LEFT_THUMB_METACARPAL = 25, JOINT_LEFT_THUMB_PHALANX_PROXIMAL = 26, JOINT_LEFT_THUMB_PHALANX_DISTAL = 27, JOINT_LEFT_THUMB_TIP = 28, JOINT_LEFT_INDEX_FINGER_METACARPAL = 29, JOINT_LEFT_INDEX_FINGER_PHALANX_PROXIMAL = 30, JOINT_LEFT_INDEX_FINGER_PHALANX_INTERMEDIATE = 31, JOINT_LEFT_INDEX_FINGER_PHALANX_DISTAL = 32, JOINT_LEFT_INDEX_FINGER_TIP = 33, JOINT_LEFT_MIDDLE_FINGER_METACARPAL = 34, JOINT_LEFT_MIDDLE_FINGER_PHALANX_PROXIMAL = 35, JOINT_LEFT_MIDDLE_FINGER_PHALANX_INTERMEDIATE = 36, JOINT_LEFT_MIDDLE_FINGER_PHALANX_DISTAL = 37, JOINT_LEFT_MIDDLE_FINGER_TIP = 38, JOINT_LEFT_RING_FINGER_METACARPAL = 39, JOINT_LEFT_RING_FINGER_PHALANX_PROXIMAL = 40, JOINT_LEFT_RING_FINGER_PHALANX_INTERMEDIATE = 41, JOINT_LEFT_RING_FINGER_PHALANX_DISTAL = 42, JOINT_LEFT_RING_FINGER_TIP = 43, JOINT_LEFT_PINKY_FINGER_METACARPAL = 44, JOINT_LEFT_PINKY_FINGER_PHALANX_PROXIMAL = 45, JOINT_LEFT_PINKY_FINGER_PHALANX_INTERMEDIATE = 46, JOINT_LEFT_PINKY_FINGER_PHALANX_DISTAL = 47, JOINT_LEFT_PINKY_FINGER_TIP = 48, JOINT_RIGHT_HAND = 49, JOINT_RIGHT_PALM = 50, JOINT_RIGHT_WRIST = 51, JOINT_RIGHT_THUMB_METACARPAL = 52, JOINT_RIGHT_THUMB_PHALANX_PROXIMAL = 53, JOINT_RIGHT_THUMB_PHALANX_DISTAL = 54, JOINT_RIGHT_THUMB_TIP = 55, JOINT_RIGHT_INDEX_FINGER_METACARPAL = 56, JOINT_RIGHT_INDEX_FINGER_PHALANX_PROXIMAL = 57, JOINT_RIGHT_INDEX_FINGER_PHALANX_INTERMEDIATE = 58, JOINT_RIGHT_INDEX_FINGER_PHALANX_DISTAL = 59, JOINT_RIGHT_INDEX_FINGER_TIP = 60, JOINT_RIGHT_MIDDLE_FINGER_METACARPAL = 61, JOINT_RIGHT_MIDDLE_FINGER_PHALANX_PROXIMAL = 62, JOINT_RIGHT_MIDDLE_FINGER_PHALANX_INTERMEDIATE = 63, JOINT_RIGHT_MIDDLE_FINGER_PHALANX_DISTAL = 64, JOINT_RIGHT_MIDDLE_FINGER_TIP = 65, JOINT_RIGHT_RING_FINGER_METACARPAL = 66, JOINT_RIGHT_RING_FINGER_PHALANX_PROXIMAL = 67, JOINT_RIGHT_RING_FINGER_PHALANX_INTERMEDIATE = 68, JOINT_RIGHT_RING_FINGER_PHALANX_DISTAL = 69, JOINT_RIGHT_RING_FINGER_TIP = 70, JOINT_RIGHT_PINKY_FINGER_METACARPAL = 71, JOINT_RIGHT_PINKY_FINGER_PHALANX_PROXIMAL = 72, JOINT_RIGHT_PINKY_FINGER_PHALANX_INTERMEDIATE = 73, JOINT_RIGHT_PINKY_FINGER_PHALANX_DISTAL = 74, JOINT_RIGHT_PINKY_FINGER_TIP = 75, JOINT_LOWER_CHEST = 76, JOINT_LEFT_SCAPULA = 77, JOINT_LEFT_WRIST_TWIST = 78, JOINT_RIGHT_SCAPULA = 79, JOINT_RIGHT_WRIST_TWIST = 80, JOINT_LEFT_FOOT_TWIST = 81, JOINT_LEFT_HEEL = 82, JOINT_LEFT_MIDDLE_FOOT = 83, JOINT_RIGHT_FOOT_TWIST = 84, JOINT_RIGHT_HEEL = 85, JOINT_RIGHT_MIDDLE_FOOT = 86, JOINT_MAX = 87 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOINT_ROOT {#JOINT_ROOT}

```gdscript
JOINT_ROOT = 0
```

Root joint.

#### JOINT_HIPS {#JOINT_HIPS}

```gdscript
JOINT_HIPS = 1
```

Hips joint.

#### JOINT_SPINE {#JOINT_SPINE}

```gdscript
JOINT_SPINE = 2
```

Spine joint.

#### JOINT_CHEST {#JOINT_CHEST}

```gdscript
JOINT_CHEST = 3
```

Chest joint.

#### JOINT_UPPER_CHEST {#JOINT_UPPER_CHEST}

```gdscript
JOINT_UPPER_CHEST = 4
```

Upper chest joint.

#### JOINT_NECK {#JOINT_NECK}

```gdscript
JOINT_NECK = 5
```

Neck joint.

#### JOINT_HEAD {#JOINT_HEAD}

```gdscript
JOINT_HEAD = 6
```

Head joint.

#### JOINT_HEAD_TIP {#JOINT_HEAD_TIP}

```gdscript
JOINT_HEAD_TIP = 7
```

Head tip joint.

#### JOINT_LEFT_SHOULDER {#JOINT_LEFT_SHOULDER}

```gdscript
JOINT_LEFT_SHOULDER = 8
```

Left shoulder joint.

#### JOINT_LEFT_UPPER_ARM {#JOINT_LEFT_UPPER_ARM}

```gdscript
JOINT_LEFT_UPPER_ARM = 9
```

Left upper arm joint.

#### JOINT_LEFT_LOWER_ARM {#JOINT_LEFT_LOWER_ARM}

```gdscript
JOINT_LEFT_LOWER_ARM = 10
```

Left lower arm joint.

#### JOINT_RIGHT_SHOULDER {#JOINT_RIGHT_SHOULDER}

```gdscript
JOINT_RIGHT_SHOULDER = 11
```

Right shoulder joint.

#### JOINT_RIGHT_UPPER_ARM {#JOINT_RIGHT_UPPER_ARM}

```gdscript
JOINT_RIGHT_UPPER_ARM = 12
```

Right upper arm joint.

#### JOINT_RIGHT_LOWER_ARM {#JOINT_RIGHT_LOWER_ARM}

```gdscript
JOINT_RIGHT_LOWER_ARM = 13
```

Right lower arm joint.

#### JOINT_LEFT_UPPER_LEG {#JOINT_LEFT_UPPER_LEG}

```gdscript
JOINT_LEFT_UPPER_LEG = 14
```

Left upper leg joint.

#### JOINT_LEFT_LOWER_LEG {#JOINT_LEFT_LOWER_LEG}

```gdscript
JOINT_LEFT_LOWER_LEG = 15
```

Left lower leg joint.

#### JOINT_LEFT_FOOT {#JOINT_LEFT_FOOT}

```gdscript
JOINT_LEFT_FOOT = 16
```

Left foot joint.

#### JOINT_LEFT_TOES {#JOINT_LEFT_TOES}

```gdscript
JOINT_LEFT_TOES = 17
```

Left toes joint.

#### JOINT_RIGHT_UPPER_LEG {#JOINT_RIGHT_UPPER_LEG}

```gdscript
JOINT_RIGHT_UPPER_LEG = 18
```

Right upper leg joint.

#### JOINT_RIGHT_LOWER_LEG {#JOINT_RIGHT_LOWER_LEG}

```gdscript
JOINT_RIGHT_LOWER_LEG = 19
```

Right lower leg joint.

#### JOINT_RIGHT_FOOT {#JOINT_RIGHT_FOOT}

```gdscript
JOINT_RIGHT_FOOT = 20
```

Right foot joint.

#### JOINT_RIGHT_TOES {#JOINT_RIGHT_TOES}

```gdscript
JOINT_RIGHT_TOES = 21
```

Right toes joint.

#### JOINT_LEFT_HAND {#JOINT_LEFT_HAND}

```gdscript
JOINT_LEFT_HAND = 22
```

Left hand joint.

#### JOINT_LEFT_PALM {#JOINT_LEFT_PALM}

```gdscript
JOINT_LEFT_PALM = 23
```

Left palm joint.

#### JOINT_LEFT_WRIST {#JOINT_LEFT_WRIST}

```gdscript
JOINT_LEFT_WRIST = 24
```

Left wrist joint.

#### JOINT_LEFT_THUMB_METACARPAL {#JOINT_LEFT_THUMB_METACARPAL}

```gdscript
JOINT_LEFT_THUMB_METACARPAL = 25
```

Left thumb metacarpal joint.

#### JOINT_LEFT_THUMB_PHALANX_PROXIMAL {#JOINT_LEFT_THUMB_PHALANX_PROXIMAL}

```gdscript
JOINT_LEFT_THUMB_PHALANX_PROXIMAL = 26
```

Left thumb phalanx proximal joint.

#### JOINT_LEFT_THUMB_PHALANX_DISTAL {#JOINT_LEFT_THUMB_PHALANX_DISTAL}

```gdscript
JOINT_LEFT_THUMB_PHALANX_DISTAL = 27
```

Left thumb phalanx distal joint.

#### JOINT_LEFT_THUMB_TIP {#JOINT_LEFT_THUMB_TIP}

```gdscript
JOINT_LEFT_THUMB_TIP = 28
```

Left thumb tip joint.

#### JOINT_LEFT_INDEX_FINGER_METACARPAL {#JOINT_LEFT_INDEX_FINGER_METACARPAL}

```gdscript
JOINT_LEFT_INDEX_FINGER_METACARPAL = 29
```

Left index finger metacarpal joint.

#### JOINT_LEFT_INDEX_FINGER_PHALANX_PROXIMAL {#JOINT_LEFT_INDEX_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_LEFT_INDEX_FINGER_PHALANX_PROXIMAL = 30
```

Left index finger phalanx proximal joint.

#### JOINT_LEFT_INDEX_FINGER_PHALANX_INTERMEDIATE {#JOINT_LEFT_INDEX_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_LEFT_INDEX_FINGER_PHALANX_INTERMEDIATE = 31
```

Left index finger phalanx intermediate joint.

#### JOINT_LEFT_INDEX_FINGER_PHALANX_DISTAL {#JOINT_LEFT_INDEX_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_LEFT_INDEX_FINGER_PHALANX_DISTAL = 32
```

Left index finger phalanx distal joint.

#### JOINT_LEFT_INDEX_FINGER_TIP {#JOINT_LEFT_INDEX_FINGER_TIP}

```gdscript
JOINT_LEFT_INDEX_FINGER_TIP = 33
```

Left index finger tip joint.

#### JOINT_LEFT_MIDDLE_FINGER_METACARPAL {#JOINT_LEFT_MIDDLE_FINGER_METACARPAL}

```gdscript
JOINT_LEFT_MIDDLE_FINGER_METACARPAL = 34
```

Left middle finger metacarpal joint.

#### JOINT_LEFT_MIDDLE_FINGER_PHALANX_PROXIMAL {#JOINT_LEFT_MIDDLE_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_LEFT_MIDDLE_FINGER_PHALANX_PROXIMAL = 35
```

Left middle finger phalanx proximal joint.

#### JOINT_LEFT_MIDDLE_FINGER_PHALANX_INTERMEDIATE {#JOINT_LEFT_MIDDLE_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_LEFT_MIDDLE_FINGER_PHALANX_INTERMEDIATE = 36
```

Left middle finger phalanx intermediate joint.

#### JOINT_LEFT_MIDDLE_FINGER_PHALANX_DISTAL {#JOINT_LEFT_MIDDLE_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_LEFT_MIDDLE_FINGER_PHALANX_DISTAL = 37
```

Left middle finger phalanx distal joint.

#### JOINT_LEFT_MIDDLE_FINGER_TIP {#JOINT_LEFT_MIDDLE_FINGER_TIP}

```gdscript
JOINT_LEFT_MIDDLE_FINGER_TIP = 38
```

Left middle finger tip joint.

#### JOINT_LEFT_RING_FINGER_METACARPAL {#JOINT_LEFT_RING_FINGER_METACARPAL}

```gdscript
JOINT_LEFT_RING_FINGER_METACARPAL = 39
```

Left ring finger metacarpal joint.

#### JOINT_LEFT_RING_FINGER_PHALANX_PROXIMAL {#JOINT_LEFT_RING_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_LEFT_RING_FINGER_PHALANX_PROXIMAL = 40
```

Left ring finger phalanx proximal joint.

#### JOINT_LEFT_RING_FINGER_PHALANX_INTERMEDIATE {#JOINT_LEFT_RING_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_LEFT_RING_FINGER_PHALANX_INTERMEDIATE = 41
```

Left ring finger phalanx intermediate joint.

#### JOINT_LEFT_RING_FINGER_PHALANX_DISTAL {#JOINT_LEFT_RING_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_LEFT_RING_FINGER_PHALANX_DISTAL = 42
```

Left ring finger phalanx distal joint.

#### JOINT_LEFT_RING_FINGER_TIP {#JOINT_LEFT_RING_FINGER_TIP}

```gdscript
JOINT_LEFT_RING_FINGER_TIP = 43
```

Left ring finger tip joint.

#### JOINT_LEFT_PINKY_FINGER_METACARPAL {#JOINT_LEFT_PINKY_FINGER_METACARPAL}

```gdscript
JOINT_LEFT_PINKY_FINGER_METACARPAL = 44
```

Left pinky finger metacarpal joint.

#### JOINT_LEFT_PINKY_FINGER_PHALANX_PROXIMAL {#JOINT_LEFT_PINKY_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_LEFT_PINKY_FINGER_PHALANX_PROXIMAL = 45
```

Left pinky finger phalanx proximal joint.

#### JOINT_LEFT_PINKY_FINGER_PHALANX_INTERMEDIATE {#JOINT_LEFT_PINKY_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_LEFT_PINKY_FINGER_PHALANX_INTERMEDIATE = 46
```

Left pinky finger phalanx intermediate joint.

#### JOINT_LEFT_PINKY_FINGER_PHALANX_DISTAL {#JOINT_LEFT_PINKY_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_LEFT_PINKY_FINGER_PHALANX_DISTAL = 47
```

Left pinky finger phalanx distal joint.

#### JOINT_LEFT_PINKY_FINGER_TIP {#JOINT_LEFT_PINKY_FINGER_TIP}

```gdscript
JOINT_LEFT_PINKY_FINGER_TIP = 48
```

Left pinky finger tip joint.

#### JOINT_RIGHT_HAND {#JOINT_RIGHT_HAND}

```gdscript
JOINT_RIGHT_HAND = 49
```

Right hand joint.

#### JOINT_RIGHT_PALM {#JOINT_RIGHT_PALM}

```gdscript
JOINT_RIGHT_PALM = 50
```

Right palm joint.

#### JOINT_RIGHT_WRIST {#JOINT_RIGHT_WRIST}

```gdscript
JOINT_RIGHT_WRIST = 51
```

Right wrist joint.

#### JOINT_RIGHT_THUMB_METACARPAL {#JOINT_RIGHT_THUMB_METACARPAL}

```gdscript
JOINT_RIGHT_THUMB_METACARPAL = 52
```

Right thumb metacarpal joint.

#### JOINT_RIGHT_THUMB_PHALANX_PROXIMAL {#JOINT_RIGHT_THUMB_PHALANX_PROXIMAL}

```gdscript
JOINT_RIGHT_THUMB_PHALANX_PROXIMAL = 53
```

Right thumb phalanx proximal joint.

#### JOINT_RIGHT_THUMB_PHALANX_DISTAL {#JOINT_RIGHT_THUMB_PHALANX_DISTAL}

```gdscript
JOINT_RIGHT_THUMB_PHALANX_DISTAL = 54
```

Right thumb phalanx distal joint.

#### JOINT_RIGHT_THUMB_TIP {#JOINT_RIGHT_THUMB_TIP}

```gdscript
JOINT_RIGHT_THUMB_TIP = 55
```

Right thumb tip joint.

#### JOINT_RIGHT_INDEX_FINGER_METACARPAL {#JOINT_RIGHT_INDEX_FINGER_METACARPAL}

```gdscript
JOINT_RIGHT_INDEX_FINGER_METACARPAL = 56
```

Right index finger metacarpal joint.

#### JOINT_RIGHT_INDEX_FINGER_PHALANX_PROXIMAL {#JOINT_RIGHT_INDEX_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_RIGHT_INDEX_FINGER_PHALANX_PROXIMAL = 57
```

Right index finger phalanx proximal joint.

#### JOINT_RIGHT_INDEX_FINGER_PHALANX_INTERMEDIATE {#JOINT_RIGHT_INDEX_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_RIGHT_INDEX_FINGER_PHALANX_INTERMEDIATE = 58
```

Right index finger phalanx intermediate joint.

#### JOINT_RIGHT_INDEX_FINGER_PHALANX_DISTAL {#JOINT_RIGHT_INDEX_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_RIGHT_INDEX_FINGER_PHALANX_DISTAL = 59
```

Right index finger phalanx distal joint.

#### JOINT_RIGHT_INDEX_FINGER_TIP {#JOINT_RIGHT_INDEX_FINGER_TIP}

```gdscript
JOINT_RIGHT_INDEX_FINGER_TIP = 60
```

Right index finger tip joint.

#### JOINT_RIGHT_MIDDLE_FINGER_METACARPAL {#JOINT_RIGHT_MIDDLE_FINGER_METACARPAL}

```gdscript
JOINT_RIGHT_MIDDLE_FINGER_METACARPAL = 61
```

Right middle finger metacarpal joint.

#### JOINT_RIGHT_MIDDLE_FINGER_PHALANX_PROXIMAL {#JOINT_RIGHT_MIDDLE_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_RIGHT_MIDDLE_FINGER_PHALANX_PROXIMAL = 62
```

Right middle finger phalanx proximal joint.

#### JOINT_RIGHT_MIDDLE_FINGER_PHALANX_INTERMEDIATE {#JOINT_RIGHT_MIDDLE_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_RIGHT_MIDDLE_FINGER_PHALANX_INTERMEDIATE = 63
```

Right middle finger phalanx intermediate joint.

#### JOINT_RIGHT_MIDDLE_FINGER_PHALANX_DISTAL {#JOINT_RIGHT_MIDDLE_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_RIGHT_MIDDLE_FINGER_PHALANX_DISTAL = 64
```

Right middle finger phalanx distal joint.

#### JOINT_RIGHT_MIDDLE_FINGER_TIP {#JOINT_RIGHT_MIDDLE_FINGER_TIP}

```gdscript
JOINT_RIGHT_MIDDLE_FINGER_TIP = 65
```

Right middle finger tip joint.

#### JOINT_RIGHT_RING_FINGER_METACARPAL {#JOINT_RIGHT_RING_FINGER_METACARPAL}

```gdscript
JOINT_RIGHT_RING_FINGER_METACARPAL = 66
```

Right ring finger metacarpal joint.

#### JOINT_RIGHT_RING_FINGER_PHALANX_PROXIMAL {#JOINT_RIGHT_RING_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_RIGHT_RING_FINGER_PHALANX_PROXIMAL = 67
```

Right ring finger phalanx proximal joint.

#### JOINT_RIGHT_RING_FINGER_PHALANX_INTERMEDIATE {#JOINT_RIGHT_RING_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_RIGHT_RING_FINGER_PHALANX_INTERMEDIATE = 68
```

Right ring finger phalanx intermediate joint.

#### JOINT_RIGHT_RING_FINGER_PHALANX_DISTAL {#JOINT_RIGHT_RING_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_RIGHT_RING_FINGER_PHALANX_DISTAL = 69
```

Right ring finger phalanx distal joint.

#### JOINT_RIGHT_RING_FINGER_TIP {#JOINT_RIGHT_RING_FINGER_TIP}

```gdscript
JOINT_RIGHT_RING_FINGER_TIP = 70
```

Right ring finger tip joint.

#### JOINT_RIGHT_PINKY_FINGER_METACARPAL {#JOINT_RIGHT_PINKY_FINGER_METACARPAL}

```gdscript
JOINT_RIGHT_PINKY_FINGER_METACARPAL = 71
```

Right pinky finger metacarpal joint.

#### JOINT_RIGHT_PINKY_FINGER_PHALANX_PROXIMAL {#JOINT_RIGHT_PINKY_FINGER_PHALANX_PROXIMAL}

```gdscript
JOINT_RIGHT_PINKY_FINGER_PHALANX_PROXIMAL = 72
```

Right pinky finger phalanx proximal joint.

#### JOINT_RIGHT_PINKY_FINGER_PHALANX_INTERMEDIATE {#JOINT_RIGHT_PINKY_FINGER_PHALANX_INTERMEDIATE}

```gdscript
JOINT_RIGHT_PINKY_FINGER_PHALANX_INTERMEDIATE = 73
```

Right pinky finger phalanx intermediate joint.

#### JOINT_RIGHT_PINKY_FINGER_PHALANX_DISTAL {#JOINT_RIGHT_PINKY_FINGER_PHALANX_DISTAL}

```gdscript
JOINT_RIGHT_PINKY_FINGER_PHALANX_DISTAL = 74
```

Right pinky finger phalanx distal joint.

#### JOINT_RIGHT_PINKY_FINGER_TIP {#JOINT_RIGHT_PINKY_FINGER_TIP}

```gdscript
JOINT_RIGHT_PINKY_FINGER_TIP = 75
```

Right pinky finger tip joint.

#### JOINT_LOWER_CHEST {#JOINT_LOWER_CHEST}

```gdscript
JOINT_LOWER_CHEST = 76
```

Lower chest joint.

#### JOINT_LEFT_SCAPULA {#JOINT_LEFT_SCAPULA}

```gdscript
JOINT_LEFT_SCAPULA = 77
```

Left scapula joint.

#### JOINT_LEFT_WRIST_TWIST {#JOINT_LEFT_WRIST_TWIST}

```gdscript
JOINT_LEFT_WRIST_TWIST = 78
```

Left wrist twist joint.

#### JOINT_RIGHT_SCAPULA {#JOINT_RIGHT_SCAPULA}

```gdscript
JOINT_RIGHT_SCAPULA = 79
```

Right scapula joint.

#### JOINT_RIGHT_WRIST_TWIST {#JOINT_RIGHT_WRIST_TWIST}

```gdscript
JOINT_RIGHT_WRIST_TWIST = 80
```

Right wrist twist joint.

#### JOINT_LEFT_FOOT_TWIST {#JOINT_LEFT_FOOT_TWIST}

```gdscript
JOINT_LEFT_FOOT_TWIST = 81
```

Left foot twist joint.

#### JOINT_LEFT_HEEL {#JOINT_LEFT_HEEL}

```gdscript
JOINT_LEFT_HEEL = 82
```

Left heel joint.

#### JOINT_LEFT_MIDDLE_FOOT {#JOINT_LEFT_MIDDLE_FOOT}

```gdscript
JOINT_LEFT_MIDDLE_FOOT = 83
```

Left middle foot joint.

#### JOINT_RIGHT_FOOT_TWIST {#JOINT_RIGHT_FOOT_TWIST}

```gdscript
JOINT_RIGHT_FOOT_TWIST = 84
```

Right foot twist joint.

#### JOINT_RIGHT_HEEL {#JOINT_RIGHT_HEEL}

```gdscript
JOINT_RIGHT_HEEL = 85
```

Right heel joint.

#### JOINT_RIGHT_MIDDLE_FOOT {#JOINT_RIGHT_MIDDLE_FOOT}

```gdscript
JOINT_RIGHT_MIDDLE_FOOT = 86
```

Right middle foot joint.

#### JOINT_MAX {#JOINT_MAX}

```gdscript
JOINT_MAX = 87
```

Represents the size of the Joint enum.

---

flags JointFlags:

### JointFlags {#enum-JointFlags}

```gdscript
enum JointFlags（位标志） { JOINT_FLAG_ORIENTATION_VALID = 1, JOINT_FLAG_ORIENTATION_TRACKED = 2, JOINT_FLAG_POSITION_VALID = 4, JOINT_FLAG_POSITION_TRACKED = 8 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOINT_FLAG_ORIENTATION_VALID {#JOINT_FLAG_ORIENTATION_VALID}

```gdscript
JOINT_FLAG_ORIENTATION_VALID = 1
```

The joint's orientation data is valid.

#### JOINT_FLAG_ORIENTATION_TRACKED {#JOINT_FLAG_ORIENTATION_TRACKED}

```gdscript
JOINT_FLAG_ORIENTATION_TRACKED = 2
```

The joint's orientation is actively tracked. May not be set if tracking has been temporarily lost.

#### JOINT_FLAG_POSITION_VALID {#JOINT_FLAG_POSITION_VALID}

```gdscript
JOINT_FLAG_POSITION_VALID = 4
```

The joint's position data is valid.

#### JOINT_FLAG_POSITION_TRACKED {#JOINT_FLAG_POSITION_TRACKED}

```gdscript
JOINT_FLAG_POSITION_TRACKED = 8
```

The joint's position is actively tracked. May not be set if tracking has been temporarily lost.

---
