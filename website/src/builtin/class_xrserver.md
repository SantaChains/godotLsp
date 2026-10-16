# XRServer

**继承**：[Object](class_object.md) → **XRServer**

> The AR/VR server is the heart of our Advanced and Virtual Reality solution and handles all the processing.

<details><summary>官方英文描述（中文精译进行中）</summary>

The AR/VR server is the heart of our Advanced and Virtual Reality solution and handles all the processing.

</details>

## 方法

### add_interface {#add_interface}

```gdscript
void add_interface(interface: XRInterface)
```

Registers an XRInterface object.

### add_tracker {#add_tracker}

```gdscript
void add_tracker(tracker: XRTracker)
```

Registers a new XRTracker that tracks a physical object.

### center_on_hmd {#center_on_hmd}

```gdscript
void center_on_hmd(rotation_mode: XRServer.RotationMode, keep_height: bool)
```

This is an important function to understand correctly. AR and VR platforms all handle positioning slightly differently.

For platforms that do not offer spatial tracking, our origin point (0, 0, 0) is the location of our HMD, but you have little control over the direction the player is facing in the real world.

For platforms that do offer spatial tracking, our origin point depends very much on the system. For OpenVR, our origin point is usually the center of the tracking space, on the ground. For other platforms, it's often the location of the tracking camera.

This method allows you to center your tracker on the location of the HMD. It will take the current location of the HMD and use that to adjust all your tracking data; in essence, realigning the real world to your player's current position in the game world.

For this method to produce usable results, tracking information must be available. This often takes a few frames after starting your game.

You should call this method after a few seconds have passed. For example, when the user requests a realignment of the display holding a designated button on a controller for a short period of time, or when implementing a teleport mechanism.

### clear_reference_frame {#clear_reference_frame}

```gdscript
void clear_reference_frame()
```

Clears the reference frame that was set by previous calls to center_on_hmd().

### find_interface {#find_interface}

```gdscript
XRInterface find_interface(name: String) const
```

Finds an interface by its name. For example, if your project uses capabilities of an AR/VR platform, you can find the interface for that platform by name and initialize it.

### get_hmd_transform {#get_hmd_transform}

```gdscript
Transform3D get_hmd_transform()
```

Returns the primary interface's transformation.

### get_interface {#get_interface}

```gdscript
XRInterface get_interface(idx: int) const
```

Returns the interface registered at the given idx index in the list of interfaces.

### get_interface_count {#get_interface_count}

```gdscript
int get_interface_count() const
```

Returns the number of interfaces currently registered with the AR/VR server. If your project supports multiple AR/VR platforms, you can look through the available interface, and either present the user with a selection or simply try to initialize each interface and use the first one that returns true.

### get_interfaces {#get_interfaces}

```gdscript
typedarray::Dictionary get_interfaces() const
```

Returns a list of available interfaces the ID and name of each interface.

### get_primary_interface {#get_primary_interface}

```gdscript
XRInterface get_primary_interface() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_reference_frame {#get_reference_frame}

```gdscript
Transform3D get_reference_frame() const
```

Returns the reference frame transform. Mostly used internally and exposed for GDExtension build interfaces.

### get_tracker {#get_tracker}

```gdscript
XRTracker get_tracker(tracker_name: StringName) const
```

Returns the positional tracker with the given tracker_name.

### get_trackers {#get_trackers}

```gdscript
Dictionary get_trackers(tracker_types: int)
```

Returns a dictionary of trackers for tracker_types.

### get_world_origin {#get_world_origin}

```gdscript
Transform3D get_world_origin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_world_scale {#get_world_scale}

```gdscript
float get_world_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_camera_locked_to_origin {#is_camera_locked_to_origin}

```gdscript
bool is_camera_locked_to_origin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_interface {#remove_interface}

```gdscript
void remove_interface(interface: XRInterface)
```

Removes this interface.

### remove_tracker {#remove_tracker}

```gdscript
void remove_tracker(tracker: XRTracker)
```

Removes this tracker.

### set_camera_locked_to_origin {#set_camera_locked_to_origin}

```gdscript
void set_camera_locked_to_origin(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_primary_interface {#set_primary_interface}

```gdscript
void set_primary_interface(interface: XRInterface)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_world_origin {#set_world_origin}

```gdscript
void set_world_origin(world_origin: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_world_scale {#set_world_scale}

```gdscript
void set_world_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### world_scale {#world_scale}

```gdscript
world_scale: float
```

- void set_world_scale(value: float)
- float get_world_scale()

The scale of the game world compared to the real world. By default, most AR/VR platforms assume that 1 game unit corresponds to 1 real world meter.

### world_origin {#world_origin}

```gdscript
world_origin: Vector3
```

- void set_world_origin(value: Transform3D)
- Transform3D get_world_origin()

The current origin of our tracking space in the virtual world. This is used by the renderer to properly position the camera with new tracking data.

Note: This property is managed by the current XROrigin3D node. It is exposed for access from GDExtensions.

### camera_locked_to_origin {#camera_locked_to_origin}

```gdscript
camera_locked_to_origin: bool
```

- void set_camera_locked_to_origin(value: bool)
- bool is_camera_locked_to_origin()

If set to true, the scene will be rendered as if the camera is locked to the XROrigin3D.

Note: This doesn't provide a very comfortable experience for users. This setting exists for doing benchmarking or automated testing, where you want to control what is rendered via code.

### primary_interface {#primary_interface}

```gdscript
primary_interface: Object
```

- void set_primary_interface(value: XRInterface)
- XRInterface get_primary_interface()

The primary XRInterface currently bound to the XRServer.

## 信号

### reference_frame_changed {#reference_frame_changed}

```gdscript
signal reference_frame_changed()
```

Emitted when the reference frame transform changes.

### interface_added {#interface_added}

```gdscript
signal interface_added(interface_name: StringName)
```

Emitted when a new interface has been added.

### interface_removed {#interface_removed}

```gdscript
signal interface_removed(interface_name: StringName)
```

Emitted when an interface is removed.

### tracker_added {#tracker_added}

```gdscript
signal tracker_added(tracker_name: StringName, type: int)
```

Emitted when a new tracker has been added. If you don't use a fixed number of controllers or if you're using XRAnchor3Ds for an AR solution, it is important to react to this signal to add the appropriate XRController3D or XRAnchor3D nodes related to this new tracker.

### tracker_updated {#tracker_updated}

```gdscript
signal tracker_updated(tracker_name: StringName, type: int)
```

Emitted when an existing tracker has been updated. This can happen if the user switches controllers.

### tracker_removed {#tracker_removed}

```gdscript
signal tracker_removed(tracker_name: StringName, type: int)
```

Emitted when a tracker is removed. You should remove any XRController3D or XRAnchor3D points if applicable. This is not mandatory, the nodes simply become inactive and will be made active again when a new tracker becomes available (i.e. a new controller is switched on that takes the place of the previous one).

### world_origin_changed {#world_origin_changed}

```gdscript
signal world_origin_changed()
```

Emitted when the world origin transform changes.

## 枚举

### TrackerType {#enum-TrackerType}

```gdscript
enum TrackerType { TRACKER_HEAD = 1, TRACKER_CONTROLLER = 2, TRACKER_BASESTATION = 4, TRACKER_ANCHOR = 8, TRACKER_HAND = 16, TRACKER_BODY = 32, TRACKER_FACE = 64, TRACKER_ANY_KNOWN = 127, TRACKER_UNKNOWN = 128, TRACKER_ANY = 255 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRACKER_HEAD {#TRACKER_HEAD}

```gdscript
TRACKER_HEAD = 1
```

The tracker tracks the location of the player's head. This is usually a location centered between the player's eyes. Note that for handheld AR devices this can be the current location of the device.

#### TRACKER_CONTROLLER {#TRACKER_CONTROLLER}

```gdscript
TRACKER_CONTROLLER = 2
```

The tracker tracks the location of a controller.

#### TRACKER_BASESTATION {#TRACKER_BASESTATION}

```gdscript
TRACKER_BASESTATION = 4
```

The tracker tracks the location of a base station.

#### TRACKER_ANCHOR {#TRACKER_ANCHOR}

```gdscript
TRACKER_ANCHOR = 8
```

The tracker tracks the location and size of an AR anchor.

#### TRACKER_HAND {#TRACKER_HAND}

```gdscript
TRACKER_HAND = 16
```

The tracker tracks the location and joints of a hand.

#### TRACKER_BODY {#TRACKER_BODY}

```gdscript
TRACKER_BODY = 32
```

The tracker tracks the location and joints of a body.

#### TRACKER_FACE {#TRACKER_FACE}

```gdscript
TRACKER_FACE = 64
```

The tracker tracks the expressions of a face.

#### TRACKER_ANY_KNOWN {#TRACKER_ANY_KNOWN}

```gdscript
TRACKER_ANY_KNOWN = 127
```

Used internally to filter trackers of any known type.

#### TRACKER_UNKNOWN {#TRACKER_UNKNOWN}

```gdscript
TRACKER_UNKNOWN = 128
```

Used internally if we haven't set the tracker type yet.

#### TRACKER_ANY {#TRACKER_ANY}

```gdscript
TRACKER_ANY = 255
```

Used internally to select all trackers.

---

### RotationMode {#enum-RotationMode}

```gdscript
enum RotationMode { RESET_FULL_ROTATION = 0, RESET_BUT_KEEP_TILT = 1, DONT_RESET_ROTATION = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESET_FULL_ROTATION {#RESET_FULL_ROTATION}

```gdscript
RESET_FULL_ROTATION = 0
```

Fully reset the orientation of the HMD. Regardless of what direction the user is looking to in the real world. The user will look dead ahead in the virtual world.

#### RESET_BUT_KEEP_TILT {#RESET_BUT_KEEP_TILT}

```gdscript
RESET_BUT_KEEP_TILT = 1
```

Resets the orientation but keeps the tilt of the device. So if we're looking down, we keep looking down but heading will be reset.

#### DONT_RESET_ROTATION {#DONT_RESET_ROTATION}

```gdscript
DONT_RESET_ROTATION = 2
```

Does not reset the orientation of the HMD, only the position of the player gets centered.

---
