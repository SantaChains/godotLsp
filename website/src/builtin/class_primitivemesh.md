# PrimitiveMesh

**继承**：[Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PrimitiveMesh**

> Base class for all primitive meshes. Handles applying a Material to a primitive mesh. Examples include BoxMesh, CapsuleMesh, CylinderMesh, PlaneMesh, PrismMesh, and SphereMesh.

<details><summary>官方英文描述</summary>

Base class for all primitive meshes. Handles applying a Material to a primitive mesh. Examples include BoxMesh, CapsuleMesh, CylinderMesh, PlaneMesh, PrismMesh, and SphereMesh.

</details>

## 方法

### _create_mesh_array {#_create_mesh_array}

```gdscript
Array _create_mesh_array() virtual const
```

Override this method to customize how this primitive mesh should be generated. Should return an Array where each element is another Array of values required for the mesh (see the ArrayType constants).

### get_add_uv2 {#get_add_uv2}

```gdscript
bool get_add_uv2() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_aabb {#get_custom_aabb}

```gdscript
AABB get_custom_aabb() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_flip_faces {#get_flip_faces}

```gdscript
bool get_flip_faces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh_arrays {#get_mesh_arrays}

```gdscript
Array get_mesh_arrays() const
```

Returns the mesh arrays used to make up the surface of this primitive mesh.

Example: Pass the result to ArrayMesh.add_surface_from_arrays() to create a new surface:

```
var c = CylinderMesh.new()
var arr_mesh = ArrayMesh.new()
arr_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, c.get_mesh_arrays())
```

```
var c = new CylinderMesh();
var arrMesh = new ArrayMesh();
arrMesh.AddSurfaceFromArrays(Mesh.PrimitiveType.Triangles, c.GetMeshArrays());
```

### get_uv2_padding {#get_uv2_padding}

```gdscript
float get_uv2_padding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### request_update {#request_update}

```gdscript
void request_update()
```

Request an update of this primitive mesh based on its properties.

### set_add_uv2 {#set_add_uv2}

```gdscript
void set_add_uv2(add_uv2: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_aabb {#set_custom_aabb}

```gdscript
void set_custom_aabb(aabb: AABB)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_faces {#set_flip_faces}

```gdscript
void set_flip_faces(flip_faces: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_uv2_padding {#set_uv2_padding}

```gdscript
void set_uv2_padding(uv2_padding: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### material {#material}

```gdscript
material: BaseMaterial3D,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The current Material of the primitive mesh.

### custom_aabb {#custom_aabb}

```gdscript
custom_aabb: AABB
```

- void set_custom_aabb(value: AABB)
- AABB get_custom_aabb()

Overrides the AABB with one defined by user for use with frustum culling. Especially useful to avoid unexpected culling when using a shader to offset vertices.

### flip_faces {#flip_faces}

```gdscript
flip_faces: bool
```

- void set_flip_faces(value: bool)
- bool get_flip_faces()

If true, the order of the vertices in each triangle is reversed, resulting in the backside of the mesh being drawn.

This gives the same result as using BaseMaterial3D.CULL_FRONT in BaseMaterial3D.cull_mode.

### add_uv2 {#add_uv2}

```gdscript
add_uv2: bool
```

- void set_add_uv2(value: bool)
- bool get_add_uv2()

If set, generates UV2 UV coordinates applying a padding using the uv2_padding setting. UV2 is needed for lightmapping.

### uv2_padding {#uv2_padding}

```gdscript
uv2_padding: float
```

- void set_uv2_padding(value: float)
- float get_uv2_padding()

If add_uv2 is set, specifies the padding in pixels applied along seams of the mesh. Lower padding values allow making better use of the lightmap texture (resulting in higher texel density), but may introduce visible lightmap bleeding along edges.

If the size of the lightmap texture can't be determined when generating the mesh, UV2 is calculated assuming a texture size of 1024x1024.
