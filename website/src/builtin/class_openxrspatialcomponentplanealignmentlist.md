# OpenXRSpatialComponentPlaneAlignmentList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentPlaneAlignmentList**

> Object for storing the queries plane alignment result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述</summary>

Object for storing the queries plane alignment result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_plane_alignment {#get_plane_alignment}

```gdscript
OpenXRSpatialComponentPlaneAlignmentList.PlaneAlignment get_plane_alignment(index: int) const
```

Returns the plane alignment for the parent entity at this index.

## 枚举

### PlaneAlignment {#enum-PlaneAlignment}

```gdscript
enum PlaneAlignment { PLANE_ALIGNMENT_HORIZONTAL_UPWARD = 0, PLANE_ALIGNMENT_HORIZONTAL_DOWNWARD = 1, PLANE_ALIGNMENT_VERTICAL = 2, PLANE_ALIGNMENT_ARBITRARY = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PLANE_ALIGNMENT_HORIZONTAL_UPWARD {#PLANE_ALIGNMENT_HORIZONTAL_UPWARD}

```gdscript
PLANE_ALIGNMENT_HORIZONTAL_UPWARD = 0
```

Plane is facing upward.

#### PLANE_ALIGNMENT_HORIZONTAL_DOWNWARD {#PLANE_ALIGNMENT_HORIZONTAL_DOWNWARD}

```gdscript
PLANE_ALIGNMENT_HORIZONTAL_DOWNWARD = 1
```

Plane is facing downwards.

#### PLANE_ALIGNMENT_VERTICAL {#PLANE_ALIGNMENT_VERTICAL}

```gdscript
PLANE_ALIGNMENT_VERTICAL = 2
```

Plane is vertically aligned.

#### PLANE_ALIGNMENT_ARBITRARY {#PLANE_ALIGNMENT_ARBITRARY}

```gdscript
PLANE_ALIGNMENT_ARBITRARY = 3
```

Plane has an arbitrary alignment.

---
