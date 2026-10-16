# XRTracker

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **XRTracker**

> This object is the base of all XR trackers.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object is the base of all XR trackers.

</details>

## 方法

### get_tracker_desc {#get_tracker_desc}

```gdscript
String get_tracker_desc() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tracker_name {#get_tracker_name}

```gdscript
StringName get_tracker_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tracker_type {#get_tracker_type}

```gdscript
XRServer.TrackerType get_tracker_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tracker_desc {#set_tracker_desc}

```gdscript
void set_tracker_desc(description: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tracker_name {#set_tracker_name}

```gdscript
void set_tracker_name(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tracker_type {#set_tracker_type}

```gdscript
void set_tracker_type(type: XRServer.TrackerType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### type {#type}

```gdscript
type: int
```

- void set_tracker_type(value: TrackerType)
- TrackerType get_tracker_type()

The type of tracker.

### name {#name}

```gdscript
name: String
```

- void set_tracker_name(value: StringName)
- StringName get_tracker_name()

The unique name of this tracker. The trackers that are available differ between various XR runtimes and can often be configured by the user. Godot maintains a number of reserved names that it expects the XRInterface to implement if applicable:

- "head" identifies the XRPositionalTracker of the player's head
- "left_hand" identifies the XRControllerTracker in the player's left hand
- "right_hand" identifies the XRControllerTracker in the player's right hand
- "/user/hand_tracker/left" identifies the XRHandTracker for the player's left hand
- "/user/hand_tracker/right" identifies the XRHandTracker for the player's right hand
- "/user/body_tracker" identifies the XRBodyTracker for the player's body
- "/user/face_tracker" identifies the XRFaceTracker for the player's face

### description {#description}

```gdscript
description: String
```

- void set_tracker_desc(value: String)
- String get_tracker_desc()

The description of this tracker.
