# OpenXRSpatialComponentBounded2DList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentBounded2DList**

> Object for storing the queries 2D bounding rectangle result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing the queries 2D bounding rectangle result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_center_pose {#get_center_pose}

```gdscript
Transform3D get_center_pose(index: int) const
```

Returns the center of our bounding rectangle for the entity at this index.

### get_size {#get_size}

```gdscript
Vector2 get_size(index: int) const
```

Returns the size of our bounding rectangle for the entity at this index.
