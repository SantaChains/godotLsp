# XRPositionalTracker

**继承**：[XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **XRPositionalTracker**

> An instance of this object represents a device that is tracked, such as a controller or anchor point. HMDs aren't represented here as they are handled internally.

<details><summary>官方英文描述（中文精译进行中）</summary>

An instance of this object represents a device that is tracked, such as a controller or anchor point. HMDs aren't represented here as they are handled internally.

As controllers are turned on and the XRInterface detects them, instances of this object are automatically added to this list of active tracking objects accessible through the XRServer.

The XRNode3D and XRAnchor3D both consume objects of this type and should be used in your project. The positional trackers are just under-the-hood objects that make this all work. These are mostly exposed so that GDExtension-based interfaces can interact with them.

</details>

## 方法

### get_input {#get_input}

```gdscript
Variant get_input(name: StringName) const
```

Deprecated: Use through XRControllerTracker.

Returns an input for this tracker. It can return a boolean, float or Vector2 value depending on whether the input is a button, trigger or thumbstick/thumbpad.

### get_pose {#get_pose}

```gdscript
XRPose get_pose(name: StringName) const
```

Returns the current XRPose state object for the bound name pose.

### get_tracker_hand {#get_tracker_hand}

```gdscript
XRPositionalTracker.TrackerHand get_tracker_hand() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tracker_profile {#get_tracker_profile}

```gdscript
String get_tracker_profile() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_pose {#has_pose}

```gdscript
bool has_pose(name: StringName) const
```

Returns true if the tracker is available and is currently tracking the bound name pose.

### invalidate_pose {#invalidate_pose}

```gdscript
void invalidate_pose(name: StringName)
```

Marks this pose as invalid, we don't clear the last reported state but it allows users to decide if trackers need to be hidden if we lose tracking or just remain at their last known position.

### set_input {#set_input}

```gdscript
void set_input(name: StringName, value: Variant)
```

Deprecated: Use through XRControllerTracker.

Changes the value for the given input. This method is called by an XRInterface implementation and should not be used directly.

### set_pose {#set_pose}

```gdscript
void set_pose(name: StringName, transform: Transform3D, linear_velocity: Vector3, angular_velocity: Vector3, tracking_confidence: XRPose.TrackingConfidence)
```

Sets the transform, linear velocity, angular velocity and tracking confidence for the given pose. This method is called by an XRInterface implementation and should not be used directly.

### set_tracker_hand {#set_tracker_hand}

```gdscript
void set_tracker_hand(hand: XRPositionalTracker.TrackerHand)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tracker_profile {#set_tracker_profile}

```gdscript
void set_tracker_profile(profile: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### profile {#profile}

```gdscript
profile: String
```

- void set_tracker_profile(value: String)
- String get_tracker_profile()

The profile associated with this tracker, interface dependent but will indicate the type of controller being tracked.

### hand {#hand}

```gdscript
hand: int
```

- void set_tracker_hand(value: TrackerHand)
- TrackerHand get_tracker_hand()

Defines which hand this tracker relates to.

## 信号

### pose_changed {#pose_changed}

```gdscript
signal pose_changed(pose: XRPose)
```

Emitted when the state of a pose tracked by this tracker changes.

### pose_lost_tracking {#pose_lost_tracking}

```gdscript
signal pose_lost_tracking(pose: XRPose)
```

Emitted when a pose tracked by this tracker stops getting updated tracking data.

### button_pressed {#button_pressed}

```gdscript
signal button_pressed(action_name: String)
```

Emitted when a button on this tracker is pressed. Note that many XR runtimes allow other inputs to be mapped to buttons.

### button_released {#button_released}

```gdscript
signal button_released(action_name: String)
```

Emitted when a button on this tracker is released.

### input_float_changed {#input_float_changed}

```gdscript
signal input_float_changed(action_name: String, value: float)
```

Emitted when a trigger or similar input on this tracker changes value.

### input_vector2_changed {#input_vector2_changed}

```gdscript
signal input_vector2_changed(action_name: String, vector: Vector2)
```

Emitted when a thumbstick or thumbpad on this tracker moves.

### profile_changed {#profile_changed}

```gdscript
signal profile_changed(role: String)
```

Emitted when the profile of our tracker changes.

## 枚举

### TrackerHand {#enum-TrackerHand}

```gdscript
enum TrackerHand { TRACKER_HAND_UNKNOWN = 0, TRACKER_HAND_LEFT = 1, TRACKER_HAND_RIGHT = 2, TRACKER_HAND_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRACKER_HAND_UNKNOWN {#TRACKER_HAND_UNKNOWN}

```gdscript
TRACKER_HAND_UNKNOWN = 0
```

The hand this tracker is held in is unknown or not applicable.

#### TRACKER_HAND_LEFT {#TRACKER_HAND_LEFT}

```gdscript
TRACKER_HAND_LEFT = 1
```

This tracker is the left hand controller.

#### TRACKER_HAND_RIGHT {#TRACKER_HAND_RIGHT}

```gdscript
TRACKER_HAND_RIGHT = 2
```

This tracker is the right hand controller.

#### TRACKER_HAND_MAX {#TRACKER_HAND_MAX}

```gdscript
TRACKER_HAND_MAX = 3
```

Represents the size of the TrackerHand enum.

---
