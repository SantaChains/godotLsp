# OpenXRSpatialComponentAnchorList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentAnchorList**

> Object for storing the queries anchor result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing the queries anchor result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_entity_pose {#get_entity_pose}

```gdscript
Transform3D get_entity_pose(index: int) const
```

Returns the transform for the entity at this index.
