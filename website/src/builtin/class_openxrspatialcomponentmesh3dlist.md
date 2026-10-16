# OpenXRSpatialComponentMesh3DList

**继承**：[OpenXRSpatialComponentData](class_openxrspatialcomponentdata.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRSpatialComponentMesh3DList**

> Object for storing the queries 3d mesh result data when calling OpenXRSpatialEntityExtension.query_snapshot().

<details><summary>官方英文描述</summary>

Object for storing the queries 3d mesh result data when calling OpenXRSpatialEntityExtension.query_snapshot().

</details>

## 方法

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh(index: int) const
```

Returns the mesh for the entity at this index.

### get_transform {#get_transform}

```gdscript
Transform3D get_transform(index: int) const
```

Returns the transform for positioning our mesh for the entity at this index.
