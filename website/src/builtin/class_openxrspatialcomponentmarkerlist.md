# OpenXRSpatialComponentMarkerList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentMarkerList**

> Object for storing the queries marker result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing the queries marker result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_marker_data {#get_marker_data}

```gdscript
Variant get_marker_data(snapshot: RID, index: int) const
```

Returns either a String or a PackedByteArray buffer with data for the marker at this index. Only applicable for QR code markers.

### get_marker_id {#get_marker_id}

```gdscript
int get_marker_id(index: int) const
```

Returns the marker ID for the marker at this index. Only applicable for Aruco or April Tag markers.

### get_marker_type {#get_marker_type}

```gdscript
OpenXRSpatialComponentMarkerList.MarkerType get_marker_type(index: int) const
```

Returns the marker type for the marker at this index.

## 枚举

### MarkerType {#enum-MarkerType}

```gdscript
enum MarkerType { MARKER_TYPE_UNKNOWN = 0, MARKER_TYPE_QRCODE = 1, MARKER_TYPE_MICRO_QRCODE = 2, MARKER_TYPE_ARUCO = 3, MARKER_TYPE_APRIL_TAG = 4, MARKER_TYPE_MAX = 5 }
```

MarkerType MARKER_TYPE_UNKNOWN = 0

Unknown or unset marker type.

MarkerType MARKER_TYPE_QRCODE = 1

Marker based on a QR code.

MarkerType MARKER_TYPE_MICRO_QRCODE = 2

Marker based on a micro QR code.

MarkerType MARKER_TYPE_ARUCO = 3

Marker based on an Aruco code.

MarkerType MARKER_TYPE_APRIL_TAG = 4

Marker based on an April Tag.

MarkerType MARKER_TYPE_MAX = 5

Maximum value for this enum.

#### MARKER_TYPE_UNKNOWN {#MARKER_TYPE_UNKNOWN}

```gdscript
MARKER_TYPE_UNKNOWN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MARKER_TYPE_QRCODE {#MARKER_TYPE_QRCODE}

```gdscript
MARKER_TYPE_QRCODE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MARKER_TYPE_MICRO_QRCODE {#MARKER_TYPE_MICRO_QRCODE}

```gdscript
MARKER_TYPE_MICRO_QRCODE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MARKER_TYPE_ARUCO {#MARKER_TYPE_ARUCO}

```gdscript
MARKER_TYPE_ARUCO = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MARKER_TYPE_APRIL_TAG {#MARKER_TYPE_APRIL_TAG}

```gdscript
MARKER_TYPE_APRIL_TAG = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MARKER_TYPE_MAX {#MARKER_TYPE_MAX}

```gdscript
MARKER_TYPE_MAX = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
