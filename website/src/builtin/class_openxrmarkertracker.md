# OpenXRMarkerTracker

**继承**：[OpenXRSpatialEntityTracker](class_openxrspatialentitytracker.md) → [XRPositionalTracker](class_xrpositionaltracker.md) → [XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRMarkerTracker**

> Spatial entity tracker for our OpenXR spatial entity marker tracking extension. These trackers identify entities in our real space detected by a visual marker such as a QRCode or Aruco code, and map their location to our virtual space.

<details><summary>官方英文描述</summary>

Spatial entity tracker for our OpenXR spatial entity marker tracking extension. These trackers identify entities in our real space detected by a visual marker such as a QRCode or Aruco code, and map their location to our virtual space.

</details>

## 方法

### get_bounds_size {#get_bounds_size}

```gdscript
Vector2 get_bounds_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_marker_data {#get_marker_data}

```gdscript
Variant get_marker_data() const
```

Returns the marker data for this marker. This can return a String or PackedByteArray. Only applicable to QR Code based markers.

### get_marker_id {#get_marker_id}

```gdscript
int get_marker_id() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_marker_type {#get_marker_type}

```gdscript
OpenXRSpatialComponentMarkerList.MarkerType get_marker_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bounds_size {#set_bounds_size}

```gdscript
void set_bounds_size(bounds_size: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_marker_data {#set_marker_data}

```gdscript
void set_marker_data(marker_data: Variant)
```

Sets the marker data for this marker.

Note: This should only be set by marker discovery logic.

### set_marker_id {#set_marker_id}

```gdscript
void set_marker_id(marker_id: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_marker_type {#set_marker_type}

```gdscript
void set_marker_type(marker_type: OpenXRSpatialComponentMarkerList.MarkerType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### bounds_size {#bounds_size}

```gdscript
bounds_size: int
```

- void set_bounds_size(value: Vector2)
- Vector2 get_bounds_size()

The bounds size for this marker.

### marker_type {#marker_type}

```gdscript
marker_type: int
```

- void set_marker_type(value: MarkerType)
- MarkerType get_marker_type()

The type of marker.

### marker_id {#marker_id}

```gdscript
marker_id: int
```

- void set_marker_id(value: int)
- int get_marker_id()

The marker ID for this marker, this is only returned for Aruco and April Tag markers. Call get_marker_data() for QRCode markers.
