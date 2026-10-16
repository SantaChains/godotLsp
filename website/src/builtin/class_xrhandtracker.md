# XRHandTracker

**继承**：[XRPositionalTracker](class_xrpositionaltracker.md) → [XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **XRHandTracker**

> A hand tracking system will create an instance of this object and add it to the XRServer. This tracking system will then obtain skeleton data, convert it to the Godot Humanoid hand skeleton and store this data on the XRHandTracker object.

<details><summary>官方英文描述</summary>

A hand tracking system will create an instance of this object and add it to the XRServer. This tracking system will then obtain skeleton data, convert it to the Godot Humanoid hand skeleton and store this data on the XRHandTracker object.

Use XRHandModifier3D to animate a hand mesh using hand tracking data.

</details>

## 方法

### get_hand_joint_angular_velocity {#get_hand_joint_angular_velocity}

```gdscript
Vector3 get_hand_joint_angular_velocity(joint: XRHandTracker.HandJoint) const
```

Returns the angular velocity for the given hand joint.

### get_hand_joint_flags {#get_hand_joint_flags}

```gdscript
bitfield::XRHandTracker.HandJointFlags get_hand_joint_flags(joint: XRHandTracker.HandJoint) const
```

Returns flags about the validity of the tracking data for the given hand joint.

### get_hand_joint_linear_velocity {#get_hand_joint_linear_velocity}

```gdscript
Vector3 get_hand_joint_linear_velocity(joint: XRHandTracker.HandJoint) const
```

Returns the linear velocity for the given hand joint.

### get_hand_joint_radius {#get_hand_joint_radius}

```gdscript
float get_hand_joint_radius(joint: XRHandTracker.HandJoint) const
```

Returns the radius of the given hand joint.

### get_hand_joint_transform {#get_hand_joint_transform}

```gdscript
Transform3D get_hand_joint_transform(joint: XRHandTracker.HandJoint) const
```

Returns the transform for the given hand joint.

### get_hand_tracking_source {#get_hand_tracking_source}

```gdscript
XRHandTracker.HandTrackingSource get_hand_tracking_source() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_has_tracking_data {#get_has_tracking_data}

```gdscript
bool get_has_tracking_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hand_joint_angular_velocity {#set_hand_joint_angular_velocity}

```gdscript
void set_hand_joint_angular_velocity(joint: XRHandTracker.HandJoint, angular_velocity: Vector3)
```

Sets the angular velocity for the given hand joint.

### set_hand_joint_flags {#set_hand_joint_flags}

```gdscript
void set_hand_joint_flags(joint: XRHandTracker.HandJoint, flags: bitfield::XRHandTracker.HandJointFlags)
```

Sets flags about the validity of the tracking data for the given hand joint.

### set_hand_joint_linear_velocity {#set_hand_joint_linear_velocity}

```gdscript
void set_hand_joint_linear_velocity(joint: XRHandTracker.HandJoint, linear_velocity: Vector3)
```

Sets the linear velocity for the given hand joint.

### set_hand_joint_radius {#set_hand_joint_radius}

```gdscript
void set_hand_joint_radius(joint: XRHandTracker.HandJoint, radius: float)
```

Sets the radius of the given hand joint.

### set_hand_joint_transform {#set_hand_joint_transform}

```gdscript
void set_hand_joint_transform(joint: XRHandTracker.HandJoint, transform: Transform3D)
```

Sets the transform for the given hand joint.

### set_hand_tracking_source {#set_hand_tracking_source}

```gdscript
void set_hand_tracking_source(source: XRHandTracker.HandTrackingSource)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_has_tracking_data {#set_has_tracking_data}

```gdscript
void set_has_tracking_data(has_data: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### has_tracking_data {#has_tracking_data}

```gdscript
has_tracking_data: bool
```

- void set_has_tracking_data(value: bool)
- bool get_has_tracking_data()

If true, the hand tracking data is valid.

### hand_tracking_source {#hand_tracking_source}

```gdscript
hand_tracking_source: int
```

- void set_hand_tracking_source(value: HandTrackingSource)
- HandTrackingSource get_hand_tracking_source()

The source of the hand tracking data.

## 枚举

### HandTrackingSource {#enum-HandTrackingSource}

```gdscript
enum HandTrackingSource { HAND_TRACKING_SOURCE_UNKNOWN = 0, HAND_TRACKING_SOURCE_UNOBSTRUCTED = 1, HAND_TRACKING_SOURCE_CONTROLLER = 2, HAND_TRACKING_SOURCE_NOT_TRACKED = 3, HAND_TRACKING_SOURCE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_TRACKING_SOURCE_UNKNOWN {#HAND_TRACKING_SOURCE_UNKNOWN}

```gdscript
HAND_TRACKING_SOURCE_UNKNOWN = 0
```

The source of hand tracking data is unknown.

#### HAND_TRACKING_SOURCE_UNOBSTRUCTED {#HAND_TRACKING_SOURCE_UNOBSTRUCTED}

```gdscript
HAND_TRACKING_SOURCE_UNOBSTRUCTED = 1
```

The source of hand tracking data is unobstructed, meaning that an accurate method of hand tracking is used. These include optical hand tracking, data gloves, etc.

#### HAND_TRACKING_SOURCE_CONTROLLER {#HAND_TRACKING_SOURCE_CONTROLLER}

```gdscript
HAND_TRACKING_SOURCE_CONTROLLER = 2
```

The source of hand tracking data is a controller, meaning that joint positions are inferred from controller inputs.

#### HAND_TRACKING_SOURCE_NOT_TRACKED {#HAND_TRACKING_SOURCE_NOT_TRACKED}

```gdscript
HAND_TRACKING_SOURCE_NOT_TRACKED = 3
```

No hand tracking data is tracked, this either means the hand is obscured, the controller is turned off, or tracking is not supported for the current input type.

#### HAND_TRACKING_SOURCE_MAX {#HAND_TRACKING_SOURCE_MAX}

```gdscript
HAND_TRACKING_SOURCE_MAX = 4
```

Represents the size of the HandTrackingSource enum.

---

### HandJoint {#enum-HandJoint}

```gdscript
enum HandJoint { HAND_JOINT_PALM = 0, HAND_JOINT_WRIST = 1, HAND_JOINT_THUMB_METACARPAL = 2, HAND_JOINT_THUMB_PHALANX_PROXIMAL = 3, HAND_JOINT_THUMB_PHALANX_DISTAL = 4, HAND_JOINT_THUMB_TIP = 5, HAND_JOINT_INDEX_FINGER_METACARPAL = 6, HAND_JOINT_INDEX_FINGER_PHALANX_PROXIMAL = 7, HAND_JOINT_INDEX_FINGER_PHALANX_INTERMEDIATE = 8, HAND_JOINT_INDEX_FINGER_PHALANX_DISTAL = 9, HAND_JOINT_INDEX_FINGER_TIP = 10, HAND_JOINT_MIDDLE_FINGER_METACARPAL = 11, HAND_JOINT_MIDDLE_FINGER_PHALANX_PROXIMAL = 12, HAND_JOINT_MIDDLE_FINGER_PHALANX_INTERMEDIATE = 13, HAND_JOINT_MIDDLE_FINGER_PHALANX_DISTAL = 14, HAND_JOINT_MIDDLE_FINGER_TIP = 15, HAND_JOINT_RING_FINGER_METACARPAL = 16, HAND_JOINT_RING_FINGER_PHALANX_PROXIMAL = 17, HAND_JOINT_RING_FINGER_PHALANX_INTERMEDIATE = 18, HAND_JOINT_RING_FINGER_PHALANX_DISTAL = 19, HAND_JOINT_RING_FINGER_TIP = 20, HAND_JOINT_PINKY_FINGER_METACARPAL = 21, HAND_JOINT_PINKY_FINGER_PHALANX_PROXIMAL = 22, HAND_JOINT_PINKY_FINGER_PHALANX_INTERMEDIATE = 23, HAND_JOINT_PINKY_FINGER_PHALANX_DISTAL = 24, HAND_JOINT_PINKY_FINGER_TIP = 25, HAND_JOINT_MAX = 26 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_JOINT_PALM {#HAND_JOINT_PALM}

```gdscript
HAND_JOINT_PALM = 0
```

Palm joint.

#### HAND_JOINT_WRIST {#HAND_JOINT_WRIST}

```gdscript
HAND_JOINT_WRIST = 1
```

Wrist joint.

#### HAND_JOINT_THUMB_METACARPAL {#HAND_JOINT_THUMB_METACARPAL}

```gdscript
HAND_JOINT_THUMB_METACARPAL = 2
```

Thumb metacarpal joint.

#### HAND_JOINT_THUMB_PHALANX_PROXIMAL {#HAND_JOINT_THUMB_PHALANX_PROXIMAL}

```gdscript
HAND_JOINT_THUMB_PHALANX_PROXIMAL = 3
```

Thumb phalanx proximal joint.

#### HAND_JOINT_THUMB_PHALANX_DISTAL {#HAND_JOINT_THUMB_PHALANX_DISTAL}

```gdscript
HAND_JOINT_THUMB_PHALANX_DISTAL = 4
```

Thumb phalanx distal joint.

#### HAND_JOINT_THUMB_TIP {#HAND_JOINT_THUMB_TIP}

```gdscript
HAND_JOINT_THUMB_TIP = 5
```

Thumb tip joint.

#### HAND_JOINT_INDEX_FINGER_METACARPAL {#HAND_JOINT_INDEX_FINGER_METACARPAL}

```gdscript
HAND_JOINT_INDEX_FINGER_METACARPAL = 6
```

Index finger metacarpal joint.

#### HAND_JOINT_INDEX_FINGER_PHALANX_PROXIMAL {#HAND_JOINT_INDEX_FINGER_PHALANX_PROXIMAL}

```gdscript
HAND_JOINT_INDEX_FINGER_PHALANX_PROXIMAL = 7
```

Index finger phalanx proximal joint.

#### HAND_JOINT_INDEX_FINGER_PHALANX_INTERMEDIATE {#HAND_JOINT_INDEX_FINGER_PHALANX_INTERMEDIATE}

```gdscript
HAND_JOINT_INDEX_FINGER_PHALANX_INTERMEDIATE = 8
```

Index finger phalanx intermediate joint.

#### HAND_JOINT_INDEX_FINGER_PHALANX_DISTAL {#HAND_JOINT_INDEX_FINGER_PHALANX_DISTAL}

```gdscript
HAND_JOINT_INDEX_FINGER_PHALANX_DISTAL = 9
```

Index finger phalanx distal joint.

#### HAND_JOINT_INDEX_FINGER_TIP {#HAND_JOINT_INDEX_FINGER_TIP}

```gdscript
HAND_JOINT_INDEX_FINGER_TIP = 10
```

Index finger tip joint.

#### HAND_JOINT_MIDDLE_FINGER_METACARPAL {#HAND_JOINT_MIDDLE_FINGER_METACARPAL}

```gdscript
HAND_JOINT_MIDDLE_FINGER_METACARPAL = 11
```

Middle finger metacarpal joint.

#### HAND_JOINT_MIDDLE_FINGER_PHALANX_PROXIMAL {#HAND_JOINT_MIDDLE_FINGER_PHALANX_PROXIMAL}

```gdscript
HAND_JOINT_MIDDLE_FINGER_PHALANX_PROXIMAL = 12
```

Middle finger phalanx proximal joint.

#### HAND_JOINT_MIDDLE_FINGER_PHALANX_INTERMEDIATE {#HAND_JOINT_MIDDLE_FINGER_PHALANX_INTERMEDIATE}

```gdscript
HAND_JOINT_MIDDLE_FINGER_PHALANX_INTERMEDIATE = 13
```

Middle finger phalanx intermediate joint.

#### HAND_JOINT_MIDDLE_FINGER_PHALANX_DISTAL {#HAND_JOINT_MIDDLE_FINGER_PHALANX_DISTAL}

```gdscript
HAND_JOINT_MIDDLE_FINGER_PHALANX_DISTAL = 14
```

Middle finger phalanx distal joint.

#### HAND_JOINT_MIDDLE_FINGER_TIP {#HAND_JOINT_MIDDLE_FINGER_TIP}

```gdscript
HAND_JOINT_MIDDLE_FINGER_TIP = 15
```

Middle finger tip joint.

#### HAND_JOINT_RING_FINGER_METACARPAL {#HAND_JOINT_RING_FINGER_METACARPAL}

```gdscript
HAND_JOINT_RING_FINGER_METACARPAL = 16
```

Ring finger metacarpal joint.

#### HAND_JOINT_RING_FINGER_PHALANX_PROXIMAL {#HAND_JOINT_RING_FINGER_PHALANX_PROXIMAL}

```gdscript
HAND_JOINT_RING_FINGER_PHALANX_PROXIMAL = 17
```

Ring finger phalanx proximal joint.

#### HAND_JOINT_RING_FINGER_PHALANX_INTERMEDIATE {#HAND_JOINT_RING_FINGER_PHALANX_INTERMEDIATE}

```gdscript
HAND_JOINT_RING_FINGER_PHALANX_INTERMEDIATE = 18
```

Ring finger phalanx intermediate joint.

#### HAND_JOINT_RING_FINGER_PHALANX_DISTAL {#HAND_JOINT_RING_FINGER_PHALANX_DISTAL}

```gdscript
HAND_JOINT_RING_FINGER_PHALANX_DISTAL = 19
```

Ring finger phalanx distal joint.

#### HAND_JOINT_RING_FINGER_TIP {#HAND_JOINT_RING_FINGER_TIP}

```gdscript
HAND_JOINT_RING_FINGER_TIP = 20
```

Ring finger tip joint.

#### HAND_JOINT_PINKY_FINGER_METACARPAL {#HAND_JOINT_PINKY_FINGER_METACARPAL}

```gdscript
HAND_JOINT_PINKY_FINGER_METACARPAL = 21
```

Pinky finger metacarpal joint.

#### HAND_JOINT_PINKY_FINGER_PHALANX_PROXIMAL {#HAND_JOINT_PINKY_FINGER_PHALANX_PROXIMAL}

```gdscript
HAND_JOINT_PINKY_FINGER_PHALANX_PROXIMAL = 22
```

Pinky finger phalanx proximal joint.

#### HAND_JOINT_PINKY_FINGER_PHALANX_INTERMEDIATE {#HAND_JOINT_PINKY_FINGER_PHALANX_INTERMEDIATE}

```gdscript
HAND_JOINT_PINKY_FINGER_PHALANX_INTERMEDIATE = 23
```

Pinky finger phalanx intermediate joint.

#### HAND_JOINT_PINKY_FINGER_PHALANX_DISTAL {#HAND_JOINT_PINKY_FINGER_PHALANX_DISTAL}

```gdscript
HAND_JOINT_PINKY_FINGER_PHALANX_DISTAL = 24
```

Pinky finger phalanx distal joint.

#### HAND_JOINT_PINKY_FINGER_TIP {#HAND_JOINT_PINKY_FINGER_TIP}

```gdscript
HAND_JOINT_PINKY_FINGER_TIP = 25
```

Pinky finger tip joint.

#### HAND_JOINT_MAX {#HAND_JOINT_MAX}

```gdscript
HAND_JOINT_MAX = 26
```

Represents the size of the HandJoint enum.

---

flags HandJointFlags:

### HandJointFlags {#enum-HandJointFlags}

```gdscript
enum HandJointFlags（位标志） { HAND_JOINT_FLAG_ORIENTATION_VALID = 1, HAND_JOINT_FLAG_ORIENTATION_TRACKED = 2, HAND_JOINT_FLAG_POSITION_VALID = 4, HAND_JOINT_FLAG_POSITION_TRACKED = 8, HAND_JOINT_FLAG_LINEAR_VELOCITY_VALID = 16, HAND_JOINT_FLAG_ANGULAR_VELOCITY_VALID = 32 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_JOINT_FLAG_ORIENTATION_VALID {#HAND_JOINT_FLAG_ORIENTATION_VALID}

```gdscript
HAND_JOINT_FLAG_ORIENTATION_VALID = 1
```

The hand joint's orientation data is valid.

#### HAND_JOINT_FLAG_ORIENTATION_TRACKED {#HAND_JOINT_FLAG_ORIENTATION_TRACKED}

```gdscript
HAND_JOINT_FLAG_ORIENTATION_TRACKED = 2
```

The hand joint's orientation is actively tracked. May not be set if tracking has been temporarily lost.

#### HAND_JOINT_FLAG_POSITION_VALID {#HAND_JOINT_FLAG_POSITION_VALID}

```gdscript
HAND_JOINT_FLAG_POSITION_VALID = 4
```

The hand joint's position data is valid.

#### HAND_JOINT_FLAG_POSITION_TRACKED {#HAND_JOINT_FLAG_POSITION_TRACKED}

```gdscript
HAND_JOINT_FLAG_POSITION_TRACKED = 8
```

The hand joint's position is actively tracked. May not be set if tracking has been temporarily lost.

#### HAND_JOINT_FLAG_LINEAR_VELOCITY_VALID {#HAND_JOINT_FLAG_LINEAR_VELOCITY_VALID}

```gdscript
HAND_JOINT_FLAG_LINEAR_VELOCITY_VALID = 16
```

The hand joint's linear velocity data is valid.

#### HAND_JOINT_FLAG_ANGULAR_VELOCITY_VALID {#HAND_JOINT_FLAG_ANGULAR_VELOCITY_VALID}

```gdscript
HAND_JOINT_FLAG_ANGULAR_VELOCITY_VALID = 32
```

The hand joint's angular velocity data is valid.

---
