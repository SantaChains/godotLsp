# OpenXRSpatialComponentPolygon2DList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentPolygon2DList**

> Object for storing the queries 2D polygon result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing the queries 2D polygon result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_transform {#get_transform}

```gdscript
Transform3D get_transform(index: int) const
```

Returns the transform for positioning our polygon for the entity at this index.

### get_vertices {#get_vertices}

```gdscript
PackedVector2Array get_vertices(snapshot: RID, index: int) const
```

Returns the polygon vertices for the entity at this index.
