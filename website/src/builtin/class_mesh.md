# Mesh

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Mesh**

> Mesh is a type of Resource that contains vertex array-based geometry, divided in surfaces. Each surface contains a completely separate array and a material used to draw it. Design wise, a mesh with multiple surfaces is preferred to a single surface, because objects created in 3D editing software commonly contain multiple materials. The maximum number of surfaces per mesh is RenderingServer.MAX_MESH_SURFACES.

<details><summary>官方英文描述（中文精译进行中）</summary>

Mesh is a type of Resource that contains vertex array-based geometry, divided in surfaces. Each surface contains a completely separate array and a material used to draw it. Design wise, a mesh with multiple surfaces is preferred to a single surface, because objects created in 3D editing software commonly contain multiple materials. The maximum number of surfaces per mesh is RenderingServer.MAX_MESH_SURFACES.

</details>

## 方法

### _get_aabb {#_get_aabb}

```gdscript
AABB _get_aabb() virtual const
```

Virtual method to override the AABB for a custom class extending Mesh.

### _get_blend_shape_count {#_get_blend_shape_count}

```gdscript
int _get_blend_shape_count() virtual const
```

Virtual method to override the number of blend shapes for a custom class extending Mesh.

### _get_blend_shape_name {#_get_blend_shape_name}

```gdscript
StringName _get_blend_shape_name(index: int) virtual const
```

Virtual method to override the retrieval of blend shape names for a custom class extending Mesh.

### _get_surface_count {#_get_surface_count}

```gdscript
int _get_surface_count() virtual const
```

Virtual method to override the surface count for a custom class extending Mesh.

### _set_blend_shape_name {#_set_blend_shape_name}

```gdscript
void _set_blend_shape_name(index: int, name: StringName) virtual
```

Virtual method to override the names of blend shapes for a custom class extending Mesh.

### _surface_get_array_index_len {#_surface_get_array_index_len}

```gdscript
int _surface_get_array_index_len(index: int) virtual const
```

Virtual method to override the surface array index length for a custom class extending Mesh.

### _surface_get_array_len {#_surface_get_array_len}

```gdscript
int _surface_get_array_len(index: int) virtual const
```

Virtual method to override the surface array length for a custom class extending Mesh.

### _surface_get_arrays {#_surface_get_arrays}

```gdscript
Array _surface_get_arrays(index: int) virtual const
```

Virtual method to override the surface arrays for a custom class extending Mesh.

### _surface_get_blend_shape_arrays {#_surface_get_blend_shape_arrays}

```gdscript
typedarray::Array _surface_get_blend_shape_arrays(index: int) virtual const
```

Virtual method to override the blend shape arrays for a custom class extending Mesh.

### _surface_get_format {#_surface_get_format}

```gdscript
int _surface_get_format(index: int) virtual const
```

Virtual method to override the surface format for a custom class extending Mesh.

### _surface_get_lods {#_surface_get_lods}

```gdscript
Dictionary _surface_get_lods(index: int) virtual const
```

Virtual method to override the surface LODs for a custom class extending Mesh.

### _surface_get_material {#_surface_get_material}

```gdscript
Material _surface_get_material(index: int) virtual const
```

Virtual method to override the surface material for a custom class extending Mesh.

### _surface_get_primitive_type {#_surface_get_primitive_type}

```gdscript
int _surface_get_primitive_type(index: int) virtual const
```

Virtual method to override the surface primitive type for a custom class extending Mesh.

### _surface_set_material {#_surface_set_material}

```gdscript
void _surface_set_material(index: int, material: Material) virtual
```

Virtual method to override the setting of a material at the given index for a custom class extending Mesh.

### create_convex_shape {#create_convex_shape}

```gdscript
ConvexPolygonShape3D create_convex_shape(clean: bool = true, simplify: bool = false) const
```

Calculate a ConvexPolygonShape3D from the mesh.

If clean is true (default), duplicate and interior vertices are removed automatically. You can set it to false to make the process faster if not needed.

If simplify is true, the geometry can be further simplified to reduce the number of vertices. Disabled by default.

### create_outline {#create_outline}

```gdscript
Mesh create_outline(margin: float) const
```

Calculate an outline mesh at a defined offset (margin) from the original mesh.

Note: This method typically returns the vertices in reverse order (e.g. clockwise to counterclockwise).

### create_placeholder {#create_placeholder}

```gdscript
Resource create_placeholder() const
```

Creates a placeholder version of this resource (PlaceholderMesh).

### create_trimesh_shape {#create_trimesh_shape}

```gdscript
ConcavePolygonShape3D create_trimesh_shape() const
```

Calculate a ConcavePolygonShape3D from the mesh.

### generate_triangle_mesh {#generate_triangle_mesh}

```gdscript
TriangleMesh generate_triangle_mesh() const
```

Generate a TriangleMesh from the mesh. Considers only surfaces using one of these primitive types: PRIMITIVE_TRIANGLES, PRIMITIVE_TRIANGLE_STRIP.

### get_aabb {#get_aabb}

```gdscript
AABB get_aabb() const
```

Returns the smallest AABB enclosing this mesh in local space. Not affected by custom_aabb.

Note: This is only implemented for ArrayMesh and PrimitiveMesh.

### get_faces {#get_faces}

```gdscript
PackedVector3Array get_faces() const
```

Returns all the vertices that make up the faces of the mesh. Each three vertices represent one triangle.

### get_lightmap_size_hint {#get_lightmap_size_hint}

```gdscript
Vector2i get_lightmap_size_hint() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_surface_count {#get_surface_count}

```gdscript
int get_surface_count() const
```

Returns the number of surfaces that the Mesh holds. This is equivalent to MeshInstance3D.get_surface_override_material_count().

### set_lightmap_size_hint {#set_lightmap_size_hint}

```gdscript
void set_lightmap_size_hint(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### surface_get_arrays {#surface_get_arrays}

```gdscript
Array surface_get_arrays(surf_idx: int) const
```

Returns the arrays for the vertices, normals, UVs, etc. that make up the requested surface (see ArrayMesh.add_surface_from_arrays()).

### surface_get_blend_shape_arrays {#surface_get_blend_shape_arrays}

```gdscript
typedarray::Array surface_get_blend_shape_arrays(surf_idx: int) const
```

Returns the blend shape arrays for the requested surface.

### surface_get_material {#surface_get_material}

```gdscript
Material surface_get_material(surf_idx: int) const
```

Returns a Material in a given surface. Surface is rendered using this material.

Note: This returns the material within the Mesh resource, not the Material associated to the MeshInstance3D's Surface Material Override properties. To get the Material associated to the MeshInstance3D's Surface Material Override properties, use MeshInstance3D.get_surface_override_material() instead.

### surface_set_material {#surface_set_material}

```gdscript
void surface_set_material(surf_idx: int, material: Material)
```

Sets a Material for a given surface. Surface will be rendered using this material.

Note: This assigns the material within the Mesh resource, not the Material associated to the MeshInstance3D's Surface Material Override properties. To set the Material associated to the MeshInstance3D's Surface Material Override properties, use MeshInstance3D.set_surface_override_material() instead.

## 属性

### lightmap_size_hint {#lightmap_size_hint}

```gdscript
lightmap_size_hint: Vector2i
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### PrimitiveType {#enum-PrimitiveType}

```gdscript
enum PrimitiveType { PRIMITIVE_POINTS = 0, PRIMITIVE_LINES = 1, PRIMITIVE_LINE_STRIP = 2, PRIMITIVE_TRIANGLES = 3, PRIMITIVE_TRIANGLE_STRIP = 4 }
```

PrimitiveType PRIMITIVE_POINTS = 0

Render array as points (one vertex equals one point).

PrimitiveType PRIMITIVE_LINES = 1

Render array as lines (every two vertices a line is created).

PrimitiveType PRIMITIVE_LINE_STRIP = 2

Render array as line strip.

PrimitiveType PRIMITIVE_TRIANGLES = 3

Render array as triangles (every three vertices a triangle is created).

PrimitiveType PRIMITIVE_TRIANGLE_STRIP = 4

Render array as triangle strips.

#### PRIMITIVE_POINTS {#PRIMITIVE_POINTS}

```gdscript
PRIMITIVE_POINTS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRIMITIVE_LINES {#PRIMITIVE_LINES}

```gdscript
PRIMITIVE_LINES = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRIMITIVE_LINE_STRIP {#PRIMITIVE_LINE_STRIP}

```gdscript
PRIMITIVE_LINE_STRIP = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRIMITIVE_TRIANGLES {#PRIMITIVE_TRIANGLES}

```gdscript
PRIMITIVE_TRIANGLES = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRIMITIVE_TRIANGLE_STRIP {#PRIMITIVE_TRIANGLE_STRIP}

```gdscript
PRIMITIVE_TRIANGLE_STRIP = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ArrayType {#enum-ArrayType}

```gdscript
enum ArrayType { ARRAY_VERTEX = 0, ARRAY_NORMAL = 1, ARRAY_TANGENT = 2, ARRAY_COLOR = 3, ARRAY_TEX_UV = 4, ARRAY_TEX_UV2 = 5, ARRAY_CUSTOM0 = 6, ARRAY_CUSTOM1 = 7, ARRAY_CUSTOM2 = 8, ARRAY_CUSTOM3 = 9, ARRAY_BONES = 10, ARRAY_WEIGHTS = 11, ARRAY_INDEX = 12, ARRAY_MAX = 13 }
```

ArrayType ARRAY_VERTEX = 0

PackedVector3Array, PackedVector2Array, or Array of vertex positions.

ArrayType ARRAY_NORMAL = 1

PackedVector3Array of vertex normals.

Note: The array has to consist of normal vectors, otherwise they will be normalized by the engine, potentially causing visual discrepancies.

ArrayType ARRAY_TANGENT = 2

PackedFloat32Array of vertex tangents. Each element in groups of 4 floats, first 3 floats determine the tangent, and the last the binormal direction as -1 or 1.

ArrayType ARRAY_COLOR = 3

PackedColorArray of vertex colors.

ArrayType ARRAY_TEX_UV = 4

PackedVector2Array for UV coordinates.

ArrayType ARRAY_TEX_UV2 = 5

PackedVector2Array for second UV coordinates.

ArrayType ARRAY_CUSTOM0 = 6

Contains custom color channel 0. PackedByteArray if (format >> Mesh.ARRAY_FORMAT_CUSTOM0_SHIFT) & Mesh.ARRAY_FORMAT_CUSTOM_MASK is ARRAY_CUSTOM_RGBA8_UNORM, ARRAY_CUSTOM_RGBA8_SNORM, ARRAY_CUSTOM_RG_HALF, or ARRAY_CUSTOM_RGBA_HALF. PackedFloat32Array otherwise.

ArrayType ARRAY_CUSTOM1 = 7

Contains custom color channel 1. PackedByteArray if (format >> Mesh.ARRAY_FORMAT_CUSTOM1_SHIFT) & Mesh.ARRAY_FORMAT_CUSTOM_MASK is ARRAY_CUSTOM_RGBA8_UNORM, ARRAY_CUSTOM_RGBA8_SNORM, ARRAY_CUSTOM_RG_HALF, or ARRAY_CUSTOM_RGBA_HALF. PackedFloat32Array otherwise.

ArrayType ARRAY_CUSTOM2 = 8

Contains custom color channel 2. PackedByteArray if (format >> Mesh.ARRAY_FORMAT_CUSTOM2_SHIFT) & Mesh.ARRAY_FORMAT_CUSTOM_MASK is ARRAY_CUSTOM_RGBA8_UNORM, ARRAY_CUSTOM_RGBA8_SNORM, ARRAY_CUSTOM_RG_HALF, or ARRAY_CUSTOM_RGBA_HALF. PackedFloat32Array otherwise.

ArrayType ARRAY_CUSTOM3 = 9

Contains custom color channel 3. PackedByteArray if (format >> Mesh.ARRAY_FORMAT_CUSTOM3_SHIFT) & Mesh.ARRAY_FORMAT_CUSTOM_MASK is ARRAY_CUSTOM_RGBA8_UNORM, ARRAY_CUSTOM_RGBA8_SNORM, ARRAY_CUSTOM_RG_HALF, or ARRAY_CUSTOM_RGBA_HALF. PackedFloat32Array otherwise.

ArrayType ARRAY_BONES = 10

PackedFloat32Array or PackedInt32Array of bone indices. Contains either 4 or 8 numbers per vertex depending on the presence of the ARRAY_FLAG_USE_8_BONE_WEIGHTS flag.

ArrayType ARRAY_WEIGHTS = 11

PackedFloat32Array or PackedFloat64Array of bone weights in the range 0.0 to 1.0 (inclusive). Contains either 4 or 8 numbers per vertex depending on the presence of the ARRAY_FLAG_USE_8_BONE_WEIGHTS flag.

ArrayType ARRAY_INDEX = 12

PackedInt32Array of integers used as indices referencing vertices, colors, normals, tangents, and textures. All of those arrays must have the same number of elements as the vertex array. No index can be beyond the vertex array size. When this index array is present, it puts the function into "index mode," where the index selects the i'th vertex, normal, tangent, color, UV, etc. This means if you want to have different normals or colors along an edge, you have to duplicate the vertices.

For triangles, the index array is interpreted as triples, referring to the vertices of each triangle. For lines, the index array is in pairs indicating the start and end of each line.

ArrayType ARRAY_MAX = 13

Represents the size of the ArrayType enum.

#### ARRAY_VERTEX {#ARRAY_VERTEX}

```gdscript
ARRAY_VERTEX = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_NORMAL {#ARRAY_NORMAL}

```gdscript
ARRAY_NORMAL = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_TANGENT {#ARRAY_TANGENT}

```gdscript
ARRAY_TANGENT = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_COLOR {#ARRAY_COLOR}

```gdscript
ARRAY_COLOR = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_TEX_UV {#ARRAY_TEX_UV}

```gdscript
ARRAY_TEX_UV = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_TEX_UV2 {#ARRAY_TEX_UV2}

```gdscript
ARRAY_TEX_UV2 = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM0 {#ARRAY_CUSTOM0}

```gdscript
ARRAY_CUSTOM0 = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM1 {#ARRAY_CUSTOM1}

```gdscript
ARRAY_CUSTOM1 = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM2 {#ARRAY_CUSTOM2}

```gdscript
ARRAY_CUSTOM2 = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM3 {#ARRAY_CUSTOM3}

```gdscript
ARRAY_CUSTOM3 = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_BONES {#ARRAY_BONES}

```gdscript
ARRAY_BONES = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_WEIGHTS {#ARRAY_WEIGHTS}

```gdscript
ARRAY_WEIGHTS = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_INDEX {#ARRAY_INDEX}

```gdscript
ARRAY_INDEX = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_MAX {#ARRAY_MAX}

```gdscript
ARRAY_MAX = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ArrayCustomFormat {#enum-ArrayCustomFormat}

```gdscript
enum ArrayCustomFormat { ARRAY_CUSTOM_RGBA8_UNORM = 0, ARRAY_CUSTOM_RGBA8_SNORM = 1, ARRAY_CUSTOM_RG_HALF = 2, ARRAY_CUSTOM_RGBA_HALF = 3, ARRAY_CUSTOM_R_FLOAT = 4, ARRAY_CUSTOM_RG_FLOAT = 5, ARRAY_CUSTOM_RGB_FLOAT = 6, ARRAY_CUSTOM_RGBA_FLOAT = 7, ARRAY_CUSTOM_MAX = 8 }
```

ArrayCustomFormat ARRAY_CUSTOM_RGBA8_UNORM = 0

Indicates this custom channel contains unsigned normalized byte colors from 0 to 1, encoded as PackedByteArray.

ArrayCustomFormat ARRAY_CUSTOM_RGBA8_SNORM = 1

Indicates this custom channel contains signed normalized byte colors from -1 to 1, encoded as PackedByteArray.

ArrayCustomFormat ARRAY_CUSTOM_RG_HALF = 2

Indicates this custom channel contains half precision float colors, encoded as PackedByteArray. Only red and green channels are used.

ArrayCustomFormat ARRAY_CUSTOM_RGBA_HALF = 3

Indicates this custom channel contains half precision float colors, encoded as PackedByteArray.

ArrayCustomFormat ARRAY_CUSTOM_R_FLOAT = 4

Indicates this custom channel contains full float colors, in a PackedFloat32Array. Only the red channel is used.

ArrayCustomFormat ARRAY_CUSTOM_RG_FLOAT = 5

Indicates this custom channel contains full float colors, in a PackedFloat32Array. Only red and green channels are used.

ArrayCustomFormat ARRAY_CUSTOM_RGB_FLOAT = 6

Indicates this custom channel contains full float colors, in a PackedFloat32Array. Only red, green and blue channels are used.

ArrayCustomFormat ARRAY_CUSTOM_RGBA_FLOAT = 7

Indicates this custom channel contains full float colors, in a PackedFloat32Array.

ArrayCustomFormat ARRAY_CUSTOM_MAX = 8

Represents the size of the ArrayCustomFormat enum.

#### ARRAY_CUSTOM_RGBA8_UNORM {#ARRAY_CUSTOM_RGBA8_UNORM}

```gdscript
ARRAY_CUSTOM_RGBA8_UNORM = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_RGBA8_SNORM {#ARRAY_CUSTOM_RGBA8_SNORM}

```gdscript
ARRAY_CUSTOM_RGBA8_SNORM = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_RG_HALF {#ARRAY_CUSTOM_RG_HALF}

```gdscript
ARRAY_CUSTOM_RG_HALF = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_RGBA_HALF {#ARRAY_CUSTOM_RGBA_HALF}

```gdscript
ARRAY_CUSTOM_RGBA_HALF = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_R_FLOAT {#ARRAY_CUSTOM_R_FLOAT}

```gdscript
ARRAY_CUSTOM_R_FLOAT = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_RG_FLOAT {#ARRAY_CUSTOM_RG_FLOAT}

```gdscript
ARRAY_CUSTOM_RG_FLOAT = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_RGB_FLOAT {#ARRAY_CUSTOM_RGB_FLOAT}

```gdscript
ARRAY_CUSTOM_RGB_FLOAT = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_RGBA_FLOAT {#ARRAY_CUSTOM_RGBA_FLOAT}

```gdscript
ARRAY_CUSTOM_RGBA_FLOAT = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_MAX {#ARRAY_CUSTOM_MAX}

```gdscript
ARRAY_CUSTOM_MAX = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ArrayFormat {#enum-ArrayFormat}

```gdscript
enum ArrayFormat（位标志） { ARRAY_FORMAT_VERTEX = 1, ARRAY_FORMAT_NORMAL = 2, ARRAY_FORMAT_TANGENT = 4, ARRAY_FORMAT_COLOR = 8, ARRAY_FORMAT_TEX_UV = 16, ARRAY_FORMAT_TEX_UV2 = 32, ARRAY_FORMAT_CUSTOM0 = 64, ARRAY_FORMAT_CUSTOM1 = 128, ARRAY_FORMAT_CUSTOM2 = 256, ARRAY_FORMAT_CUSTOM3 = 512, ARRAY_FORMAT_BONES = 1024, ARRAY_FORMAT_WEIGHTS = 2048, ARRAY_FORMAT_INDEX = 4096, ARRAY_FORMAT_BLEND_SHAPE_MASK = 7, ARRAY_FORMAT_CUSTOM_BASE = 13, ARRAY_FORMAT_CUSTOM_BITS = 3, ARRAY_FORMAT_CUSTOM0_SHIFT = 13, ARRAY_FORMAT_CUSTOM1_SHIFT = 16, ARRAY_FORMAT_CUSTOM2_SHIFT = 19, ARRAY_FORMAT_CUSTOM3_SHIFT = 22, ARRAY_FORMAT_CUSTOM_MASK = 7, ARRAY_COMPRESS_FLAGS_BASE = 25, ARRAY_FLAG_USE_2D_VERTICES = 33554432, ARRAY_FLAG_USE_DYNAMIC_UPDATE = 67108864, ARRAY_FLAG_USE_8_BONE_WEIGHTS = 134217728, ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY = 268435456, ARRAY_FLAG_COMPRESS_ATTRIBUTES = 536870912 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_VERTEX {#ARRAY_FORMAT_VERTEX}

```gdscript
ARRAY_FORMAT_VERTEX = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_NORMAL {#ARRAY_FORMAT_NORMAL}

```gdscript
ARRAY_FORMAT_NORMAL = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_TANGENT {#ARRAY_FORMAT_TANGENT}

```gdscript
ARRAY_FORMAT_TANGENT = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_COLOR {#ARRAY_FORMAT_COLOR}

```gdscript
ARRAY_FORMAT_COLOR = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_TEX_UV {#ARRAY_FORMAT_TEX_UV}

```gdscript
ARRAY_FORMAT_TEX_UV = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_TEX_UV2 {#ARRAY_FORMAT_TEX_UV2}

```gdscript
ARRAY_FORMAT_TEX_UV2 = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM0 {#ARRAY_FORMAT_CUSTOM0}

```gdscript
ARRAY_FORMAT_CUSTOM0 = 64
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM1 {#ARRAY_FORMAT_CUSTOM1}

```gdscript
ARRAY_FORMAT_CUSTOM1 = 128
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM2 {#ARRAY_FORMAT_CUSTOM2}

```gdscript
ARRAY_FORMAT_CUSTOM2 = 256
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM3 {#ARRAY_FORMAT_CUSTOM3}

```gdscript
ARRAY_FORMAT_CUSTOM3 = 512
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_BONES {#ARRAY_FORMAT_BONES}

```gdscript
ARRAY_FORMAT_BONES = 1024
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_WEIGHTS {#ARRAY_FORMAT_WEIGHTS}

```gdscript
ARRAY_FORMAT_WEIGHTS = 2048
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_INDEX {#ARRAY_FORMAT_INDEX}

```gdscript
ARRAY_FORMAT_INDEX = 4096
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_BLEND_SHAPE_MASK {#ARRAY_FORMAT_BLEND_SHAPE_MASK}

```gdscript
ARRAY_FORMAT_BLEND_SHAPE_MASK = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM_BASE {#ARRAY_FORMAT_CUSTOM_BASE}

```gdscript
ARRAY_FORMAT_CUSTOM_BASE = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM_BITS {#ARRAY_FORMAT_CUSTOM_BITS}

```gdscript
ARRAY_FORMAT_CUSTOM_BITS = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM0_SHIFT {#ARRAY_FORMAT_CUSTOM0_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM0_SHIFT = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM1_SHIFT {#ARRAY_FORMAT_CUSTOM1_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM1_SHIFT = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM2_SHIFT {#ARRAY_FORMAT_CUSTOM2_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM2_SHIFT = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM3_SHIFT {#ARRAY_FORMAT_CUSTOM3_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM3_SHIFT = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_CUSTOM_MASK {#ARRAY_FORMAT_CUSTOM_MASK}

```gdscript
ARRAY_FORMAT_CUSTOM_MASK = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_COMPRESS_FLAGS_BASE {#ARRAY_COMPRESS_FLAGS_BASE}

```gdscript
ARRAY_COMPRESS_FLAGS_BASE = 25
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FLAG_USE_2D_VERTICES {#ARRAY_FLAG_USE_2D_VERTICES}

```gdscript
ARRAY_FLAG_USE_2D_VERTICES = 33554432
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FLAG_USE_DYNAMIC_UPDATE {#ARRAY_FLAG_USE_DYNAMIC_UPDATE}

```gdscript
ARRAY_FLAG_USE_DYNAMIC_UPDATE = 67108864
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FLAG_USE_8_BONE_WEIGHTS {#ARRAY_FLAG_USE_8_BONE_WEIGHTS}

```gdscript
ARRAY_FLAG_USE_8_BONE_WEIGHTS = 134217728
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY {#ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY}

```gdscript
ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY = 268435456
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FLAG_COMPRESS_ATTRIBUTES {#ARRAY_FLAG_COMPRESS_ATTRIBUTES}

```gdscript
ARRAY_FLAG_COMPRESS_ATTRIBUTES = 536870912
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### BlendShapeMode {#enum-BlendShapeMode}

```gdscript
enum BlendShapeMode { BLEND_SHAPE_MODE_NORMALIZED = 0, BLEND_SHAPE_MODE_RELATIVE = 1 }
```

BlendShapeMode BLEND_SHAPE_MODE_NORMALIZED = 0

Blend shapes are normalized.

BlendShapeMode BLEND_SHAPE_MODE_RELATIVE = 1

Blend shapes are relative to base weight.

#### BLEND_SHAPE_MODE_NORMALIZED {#BLEND_SHAPE_MODE_NORMALIZED}

```gdscript
BLEND_SHAPE_MODE_NORMALIZED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BLEND_SHAPE_MODE_RELATIVE {#BLEND_SHAPE_MODE_RELATIVE}

```gdscript
BLEND_SHAPE_MODE_RELATIVE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
