# OpenXRPlaneTracker

**继承**：[OpenXRSpatialEntityTracker](class_openxrspatialentitytracker.md) → [XRPositionalTracker](class_xrpositionaltracker.md) → [XRTracker](class_xrtracker.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRPlaneTracker**

> Spatial entity tracker for our OpenXR spatial entity plane tracking extension. These trackers identify entities in our real space such as walls, floors, tables, etc. and map their location to our virtual space.

<details><summary>官方英文描述</summary>

Spatial entity tracker for our OpenXR spatial entity plane tracking extension. These trackers identify entities in our real space such as walls, floors, tables, etc. and map their location to our virtual space.

</details>

## 方法

### clear_mesh_data {#clear_mesh_data}

```gdscript
void clear_mesh_data()
```

Clears the mesh data for this tracker. You should only call this if you are handling your own discovery logic.

### get_bounds_size {#get_bounds_size}

```gdscript
Vector2 get_bounds_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh()
```

Gets a mesh created from either the mesh data or from our bounding size for this plane.

### get_mesh_offset {#get_mesh_offset}

```gdscript
Transform3D get_mesh_offset() const
```

Gets the transform by which to offset the mesh and collision shape from our pose to display these correctly.

### get_plane_alignment {#get_plane_alignment}

```gdscript
OpenXRSpatialComponentPlaneAlignmentList.PlaneAlignment get_plane_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_plane_label {#get_plane_label}

```gdscript
String get_plane_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
Shape3D get_shape(thickness: float = 0.01)
```

Gets a collision shape built either from the mesh data or from our bounding size for this plane.

### set_bounds_size {#set_bounds_size}

```gdscript
void set_bounds_size(bounds_size: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh_data {#set_mesh_data}

```gdscript
void set_mesh_data(origin: Transform3D, vertices: PackedVector2Array, indices: PackedInt32Array = PackedInt32Array())
```

Sets the mesh data for this plane. You should only call this if you are handling your own discovery logic.

### set_plane_alignment {#set_plane_alignment}

```gdscript
void set_plane_alignment(plane_alignment: OpenXRSpatialComponentPlaneAlignmentList.PlaneAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_plane_label {#set_plane_label}

```gdscript
void set_plane_label(plane_label: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### bounds_size {#bounds_size}

```gdscript
bounds_size: int
```

- void set_bounds_size(value: Vector2)
- Vector2 get_bounds_size()

The bounding size of the plane. This is a 2D size.

### plane_alignment {#plane_alignment}

```gdscript
plane_alignment: int
```

- void set_plane_alignment(value: PlaneAlignment)
- PlaneAlignment get_plane_alignment()

The main alignment in space of this plane.

### plane_label {#plane_label}

```gdscript
plane_label: String
```

- void set_plane_label(value: String)
- String get_plane_label()

The semantic label for this plane.

## 信号

### mesh_changed {#mesh_changed}

```gdscript
signal mesh_changed()
```

Emitted when our mesh data has changed the mesh instance and collision needs to be updated.
