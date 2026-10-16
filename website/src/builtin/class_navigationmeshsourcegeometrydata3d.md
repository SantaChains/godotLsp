# NavigationMeshSourceGeometryData3D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **NavigationMeshSourceGeometryData3D**

> Container for parsed source geometry data used in navigation mesh baking.

<details><summary>官方英文描述</summary>

Container for parsed source geometry data used in navigation mesh baking.

</details>

## 方法

### add_faces {#add_faces}

```gdscript
void add_faces(faces: PackedVector3Array, xform: Transform3D)
```

Adds an array of vertex positions to the geometry data for navigation mesh baking to form triangulated faces. For each face the array must have three vertex positions in clockwise winding order. Since NavigationMesh resources have no transform, all vertex positions need to be offset by the node's transform using xform.

### add_mesh {#add_mesh}

```gdscript
void add_mesh(mesh: Mesh, xform: Transform3D)
```

Adds the geometry data of a Mesh resource to the navigation mesh baking data. The mesh must have valid triangulated mesh data to be considered. Since NavigationMesh resources have no transform, all vertex positions need to be offset by the node's transform using xform.

### add_mesh_array {#add_mesh_array}

```gdscript
void add_mesh_array(mesh_array: Array, xform: Transform3D)
```

Adds an Array the size of Mesh.ARRAY_MAX and with vertices at index Mesh.ARRAY_VERTEX and indices at index Mesh.ARRAY_INDEX to the navigation mesh baking data. The array must have valid triangulated mesh data to be considered. Since NavigationMesh resources have no transform, all vertex positions need to be offset by the node's transform using xform.

### add_projected_obstruction {#add_projected_obstruction}

```gdscript
void add_projected_obstruction(vertices: PackedVector3Array, elevation: float, height: float, carve: bool)
```

Adds a projected obstruction shape to the source geometry. The vertices are considered projected on an xz-axes plane, placed at the global y-axis elevation and extruded by height. If carve is true the carved shape will not be affected by additional offsets (e.g. agent radius) of the navigation mesh baking process.

### append_arrays {#append_arrays}

```gdscript
void append_arrays(vertices: PackedFloat32Array, indices: PackedInt32Array)
```

Appends arrays of vertices and indices at the end of the existing arrays. Adds the existing index as an offset to the appended indices.

### clear {#clear}

```gdscript
void clear()
```

Clears the internal data.

### clear_projected_obstructions {#clear_projected_obstructions}

```gdscript
void clear_projected_obstructions()
```

Clears all projected obstructions.

### get_bounds {#get_bounds}

```gdscript
AABB get_bounds()
```

Returns an axis-aligned bounding box that covers all the stored geometry data. The bounds are calculated when calling this function with the result cached until further geometry changes are made.

### get_indices {#get_indices}

```gdscript
PackedInt32Array get_indices() const
```

Returns the parsed source geometry data indices array.

### get_projected_obstructions {#get_projected_obstructions}

```gdscript
Array get_projected_obstructions() const
```

Returns the projected obstructions as an Array of dictionaries. Each Dictionary contains the following entries:

- vertices - A PackedFloat32Array that defines the outline points of the projected shape.
- elevation - A float that defines the projected shape placement on the y-axis.
- height - A float that defines how much the projected shape is extruded along the y-axis.
- carve - A bool that defines how the obstacle affects the navigation mesh baking. If true the projected shape will not be affected by addition offsets, e.g. agent radius.

### get_vertices {#get_vertices}

```gdscript
PackedFloat32Array get_vertices() const
```

Returns the parsed source geometry data vertices array.

### has_data {#has_data}

```gdscript
bool has_data()
```

Returns true when parsed source geometry data exists.

### merge {#merge}

```gdscript
void merge(other_geometry: NavigationMeshSourceGeometryData3D)
```

Adds the geometry data of another NavigationMeshSourceGeometryData3D to the navigation mesh baking data.

### set_indices {#set_indices}

```gdscript
void set_indices(indices: PackedInt32Array)
```

Sets the parsed source geometry data indices. The indices need to be matched with appropriated vertices.

Warning: Inappropriate data can crash the baking process of the involved third-party libraries.

### set_projected_obstructions {#set_projected_obstructions}

```gdscript
void set_projected_obstructions(projected_obstructions: Array)
```

Sets the projected obstructions with an Array of Dictionaries with the following key value pairs:

```
"vertices" : PackedFloat32Array
"elevation" : float
"height" : float
"carve" : bool
```

### set_vertices {#set_vertices}

```gdscript
void set_vertices(vertices: PackedFloat32Array)
```

Sets the parsed source geometry data vertices. The vertices need to be matched with appropriated indices.

Warning: Inappropriate data can crash the baking process of the involved third-party libraries.

## 属性

### vertices {#vertices}

```gdscript
vertices: PackedVector3Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### indices {#indices}

```gdscript
indices: PackedInt32Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### projected_obstructions {#projected_obstructions}

```gdscript
projected_obstructions: Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
