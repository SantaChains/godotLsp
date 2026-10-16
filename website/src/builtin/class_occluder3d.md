# Occluder3D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Occluder3D**

> Occluder3D stores an occluder shape that can be used by the engine's occlusion culling system.

<details><summary>官方英文描述（中文精译进行中）</summary>

Occluder3D stores an occluder shape that can be used by the engine's occlusion culling system.

See OccluderInstance3D's documentation for instructions on setting up occlusion culling.

</details>

## 方法

### get_indices {#get_indices}

```gdscript
PackedInt32Array get_indices() const
```

Returns the occluder shape's vertex indices.

### get_vertices {#get_vertices}

```gdscript
PackedVector3Array get_vertices() const
```

Returns the occluder shape's vertex positions.
