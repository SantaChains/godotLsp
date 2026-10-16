# XRPose

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **XRPose**

> XR runtimes often identify multiple locations on devices such as controllers that are spatially tracked.

<details><summary>官方英文描述</summary>

XR runtimes often identify multiple locations on devices such as controllers that are spatially tracked.

Orientation, location, linear velocity and angular velocity are all provided for each pose by the XR runtime. This object contains this state of a pose.

</details>

## 方法

### get_adjusted_transform {#get_adjusted_transform}

```gdscript
Transform3D get_adjusted_transform() const
```

Returns the transform with world scale and our reference frame applied. This is the transform used to position XRNode3D objects.

### get_angular_velocity {#get_angular_velocity}

```gdscript
Vector3 get_angular_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_has_tracking_data {#get_has_tracking_data}

```gdscript
bool get_has_tracking_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_linear_velocity {#get_linear_velocity}

```gdscript
Vector3 get_linear_velocity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_name {#get_name}

```gdscript
StringName get_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tracking_confidence {#get_tracking_confidence}

```gdscript
XRPose.TrackingConfidence get_tracking_confidence() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transform {#get_transform}

```gdscript
Transform3D get_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_angular_velocity {#set_angular_velocity}

```gdscript
void set_angular_velocity(velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_has_tracking_data {#set_has_tracking_data}

```gdscript
void set_has_tracking_data(has_tracking_data: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_linear_velocity {#set_linear_velocity}

```gdscript
void set_linear_velocity(velocity: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_name {#set_name}

```gdscript
void set_name(name: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tracking_confidence {#set_tracking_confidence}

```gdscript
void set_tracking_confidence(tracking_confidence: XRPose.TrackingConfidence)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transform {#set_transform}

```gdscript
void set_transform(transform: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### has_tracking_data {#has_tracking_data}

```gdscript
has_tracking_data: bool
```

- void set_has_tracking_data(value: bool)
- bool get_has_tracking_data()

If true our tracking data is up to date. If false we're no longer receiving new tracking data and our state is whatever that last valid state was.

### name {#name}

```gdscript
name: String
```

- void set_name(value: StringName)
- StringName get_name()

The name of this pose. Usually, this name is derived from an action map set up by the user. Godot also suggests some pose names that XRInterface objects are expected to implement:

- root is the root location, often used for tracked objects that do not have further nodes.
- aim is the tip of a controller with its orientation pointing outwards, often used for raycasts.
- grip is the location where the user grips the controller.
- skeleton is the root location for a hand mesh, when using hand tracking and an animated skeleton is supplied by the XR runtime.

### transform {#transform}

```gdscript
transform: String
```

- void set_transform(value: Transform3D)
- Transform3D get_transform()

The transform containing the original and transform as reported by the XR runtime.

### linear_velocity {#linear_velocity}

```gdscript
linear_velocity: String
```

- void set_linear_velocity(value: Vector3)
- Vector3 get_linear_velocity()

The linear velocity of this pose.

### angular_velocity {#angular_velocity}

```gdscript
angular_velocity: String
```

- void set_angular_velocity(value: Vector3)
- Vector3 get_angular_velocity()

The angular velocity for this pose.

### tracking_confidence {#tracking_confidence}

```gdscript
tracking_confidence: int
```

- void set_tracking_confidence(value: TrackingConfidence)
- TrackingConfidence get_tracking_confidence()

The tracking confidence for this pose, provides insight on how accurate the spatial positioning of this record is.

## 枚举

### TrackingConfidence {#enum-TrackingConfidence}

```gdscript
enum TrackingConfidence { XR_TRACKING_CONFIDENCE_NONE = 0, XR_TRACKING_CONFIDENCE_LOW = 1, XR_TRACKING_CONFIDENCE_HIGH = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_TRACKING_CONFIDENCE_NONE {#XR_TRACKING_CONFIDENCE_NONE}

```gdscript
XR_TRACKING_CONFIDENCE_NONE = 0
```

No tracking information is available for this pose.

#### XR_TRACKING_CONFIDENCE_LOW {#XR_TRACKING_CONFIDENCE_LOW}

```gdscript
XR_TRACKING_CONFIDENCE_LOW = 1
```

Tracking information may be inaccurate or estimated. For example, with inside out tracking this would indicate a controller may be (partially) obscured.

#### XR_TRACKING_CONFIDENCE_HIGH {#XR_TRACKING_CONFIDENCE_HIGH}

```gdscript
XR_TRACKING_CONFIDENCE_HIGH = 2
```

Tracking information is considered accurate and up to date.

---
