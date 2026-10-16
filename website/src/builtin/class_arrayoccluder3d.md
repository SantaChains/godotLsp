# ArrayOccluder3D

**继承**：[Occluder3D](class_occluder3d.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ArrayOccluder3D**

> ArrayOccluder3D stores an arbitrary 3D polygon shape that can be used by the engine's occlusion culling system. This is analogous to ArrayMesh, but for occluders.

<details><summary>官方英文描述（中文精译进行中）</summary>

ArrayOccluder3D stores an arbitrary 3D polygon shape that can be used by the engine's occlusion culling system. This is analogous to ArrayMesh, but for occluders.

See OccluderInstance3D's documentation for instructions on setting up occlusion culling.

</details>

## 方法

### set_arrays {#set_arrays}

```gdscript
void set_arrays(vertices: PackedVector3Array, indices: PackedInt32Array)
```

Sets indices and vertices, while updating the final occluder only once after both values are set.

### set_indices {#set_indices}

```gdscript
void set_indices(indices: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertices {#set_vertices}

```gdscript
void set_vertices(vertices: PackedVector3Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### vertices {#vertices}

```gdscript
vertices: PackedVector3Array
```

- void set_vertices(value: PackedVector3Array)
- PackedVector3Array get_vertices()

The occluder's vertex positions in local 3D coordinates.

Note: The occluder is always updated after setting this value. If creating occluders procedurally, consider using set_arrays() instead to avoid updating the occluder twice when it's created.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector3Array for more details.

### indices {#indices}

```gdscript
indices: PackedInt32Array
```

- void set_indices(value: PackedInt32Array)
- PackedInt32Array get_indices()

The occluder's index position. Indices determine which points from the vertices array should be drawn, and in which order.

Note: The occluder is always updated after setting this value. If creating occluders procedurally, consider using set_arrays() instead to avoid updating the occluder twice when it's created.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.
