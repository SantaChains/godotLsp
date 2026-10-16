# XRNode3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **XRNode3D**

> This node can be bound to a specific pose of an XRPositionalTracker and will automatically have its Node3D.transform updated by the XRServer. Nodes of this type must be added as children of the XROrigin3D node.

<details><summary>官方英文描述</summary>

This node can be bound to a specific pose of an XRPositionalTracker and will automatically have its Node3D.transform updated by the XRServer. Nodes of this type must be added as children of the XROrigin3D node.

</details>

## 方法

### get_has_tracking_data {#get_has_tracking_data}

```gdscript
bool get_has_tracking_data() const
```

Returns true if the tracker has current tracking data for the pose being tracked.

### get_is_active {#get_is_active}

```gdscript
bool get_is_active() const
```

Returns true if the tracker has been registered and the pose is being tracked.

### get_pose {#get_pose}

```gdscript
XRPose get_pose()
```

Returns the XRPose containing the current state of the pose being tracked. This gives access to additional properties of this pose.

### get_pose_name {#get_pose_name}

```gdscript
StringName get_pose_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_show_when_tracked {#get_show_when_tracked}

```gdscript
bool get_show_when_tracked() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tracker {#get_tracker}

```gdscript
StringName get_tracker() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pose_name {#set_pose_name}

```gdscript
void set_pose_name(pose: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_when_tracked {#set_show_when_tracked}

```gdscript
void set_show_when_tracked(show: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tracker {#set_tracker}

```gdscript
void set_tracker(tracker_name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### trigger_haptic_pulse {#trigger_haptic_pulse}

```gdscript
void trigger_haptic_pulse(action_name: String, frequency: float, amplitude: float, duration_sec: float, delay_sec: float)
```

Triggers a haptic pulse on a device associated with this interface.

action_name is the name of the action for this pulse.

frequency is the frequency of the pulse, set to 0.0 to have the system use a default frequency.

amplitude is the amplitude of the pulse between 0.0 and 1.0.

duration_sec is the duration of the pulse in seconds.

delay_sec is a delay in seconds before the pulse is given.

## 属性

### tracker {#tracker}

```gdscript
tracker: String
```

- void set_tracker(value: StringName)
- StringName get_tracker()

The name of the tracker we're bound to. Which trackers are available is not known during design time.

Godot defines a number of standard trackers such as left_hand and right_hand but others may be configured within a given XRInterface.

### pose {#pose}

```gdscript
pose: String
```

- void set_pose_name(value: StringName)
- StringName get_pose_name()

The name of the pose we're bound to. Which poses a tracker supports is not known during design time.

Godot defines number of standard pose names such as aim and grip but other may be configured within a given XRInterface.

### show_when_tracked {#show_when_tracked}

```gdscript
show_when_tracked: bool
```

- void set_show_when_tracked(value: bool)
- bool get_show_when_tracked()

Enables showing the node when tracking starts, and hiding the node when tracking is lost.

## 信号

### tracking_changed {#tracking_changed}

```gdscript
signal tracking_changed(tracking: bool)
```

Emitted when the tracker starts or stops receiving updated tracking data for the pose being tracked. The tracking argument indicates whether the tracker is getting updated tracking data.
