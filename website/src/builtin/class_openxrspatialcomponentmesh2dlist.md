# OpenXRSpatialComponentMesh2DList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentMesh2DList**

> Object for storing the queries 2D mesh result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述</summary>

Object for storing the queries 2D mesh result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_indices {#get_indices}

```gdscript
PackedInt32Array get_indices(snapshot: RID, index: int) const
```

Returns the mesh indices for the entity at this index.

### get_transform {#get_transform}

```gdscript
Transform3D get_transform(index: int) const
```

Returns the transform for positioning our mesh for the entity at this index.

### get_vertices {#get_vertices}

```gdscript
PackedVector2Array get_vertices(snapshot: RID, index: int) const
```

Returns the mesh vertices for the entity at this index.
