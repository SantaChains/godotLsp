# MeshInstance3D

**继承**：[GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **MeshInstance3D**

> MeshInstance3D is a node that takes a Mesh resource and adds it to the current scenario by creating an instance of it. This is the class most often used to render 3D geometry and can be used to instance a single Mesh in many places. This allows reusing geometry, which can save on resources. When a Mesh has to be instantiated more than thousands of times at close proximity, consider using a MultiMesh in a MultiMeshInstance3D instead.

<details><summary>官方英文描述（中文精译进行中）</summary>

MeshInstance3D is a node that takes a Mesh resource and adds it to the current scenario by creating an instance of it. This is the class most often used to render 3D geometry and can be used to instance a single Mesh in many places. This allows reusing geometry, which can save on resources. When a Mesh has to be instantiated more than thousands of times at close proximity, consider using a MultiMesh in a MultiMeshInstance3D instead.

</details>

## 方法

### bake_mesh_from_current_blend_shape_mix {#bake_mesh_from_current_blend_shape_mix}

```gdscript
ArrayMesh bake_mesh_from_current_blend_shape_mix(existing: ArrayMesh = null)
```

Takes a snapshot from the current ArrayMesh with all blend shapes applied according to their current weights and bakes it to the provided existing mesh. If no existing mesh is provided a new ArrayMesh is created, baked and returned. Mesh surface materials are not copied.

Performance: Mesh data needs to be received from the GPU, stalling the RenderingServer in the process.

### bake_mesh_from_current_skeleton_pose {#bake_mesh_from_current_skeleton_pose}

```gdscript
ArrayMesh bake_mesh_from_current_skeleton_pose(existing: ArrayMesh = null)
```

Takes a snapshot of the current animated skeleton pose of the skinned mesh and bakes it to the provided existing mesh. If no existing mesh is provided a new ArrayMesh is created, baked, and returned. Requires a skeleton with a registered skin to work. Blendshapes are ignored. Mesh surface materials are not copied.

Performance: Mesh data needs to be retrieved from the GPU, stalling the RenderingServer in the process.

### create_convex_collision {#create_convex_collision}

```gdscript
void create_convex_collision(clean: bool = true, simplify: bool = false)
```

This helper creates a StaticBody3D child node with a ConvexPolygonShape3D collision shape calculated from the mesh geometry. It's mainly used for testing.

If clean is true (default), duplicate and interior vertices are removed automatically. You can set it to false to make the process faster if not needed.

If simplify is true, the geometry can be further simplified to reduce the number of vertices. Disabled by default.

### create_debug_tangents {#create_debug_tangents}

```gdscript
void create_debug_tangents()
```

This helper creates a MeshInstance3D child node with gizmos at every vertex calculated from the mesh geometry. It's mainly used for testing.

### create_multiple_convex_collisions {#create_multiple_convex_collisions}

```gdscript
void create_multiple_convex_collisions(settings: MeshConvexDecompositionSettings = null)
```

This helper creates a StaticBody3D child node with multiple ConvexPolygonShape3D collision shapes calculated from the mesh geometry via convex decomposition. The convex decomposition operation can be controlled with parameters from the optional settings.

### create_trimesh_collision {#create_trimesh_collision}

```gdscript
void create_trimesh_collision()
```

This helper creates a StaticBody3D child node with a ConcavePolygonShape3D collision shape calculated from the mesh geometry. It's mainly used for testing.

### find_blend_shape_by_name {#find_blend_shape_by_name}

```gdscript
int find_blend_shape_by_name(name: StringName)
```

Returns the index of the blend shape with the given name. Returns -1 if no blend shape with this name exists, including when mesh is null.

### get_active_material {#get_active_material}

```gdscript
Material get_active_material(surface: int) const
```

Returns the Material that will be used by the Mesh when drawing. This can return the GeometryInstance3D.material_override, the surface override Material defined in this MeshInstance3D, or the surface Material defined in the mesh. For example, if GeometryInstance3D.material_override is used, all surfaces will return the override material.

Returns null if no material is active, including when mesh is null.

### get_blend_shape_count {#get_blend_shape_count}

```gdscript
int get_blend_shape_count() const
```

Returns the number of blend shapes available. Produces an error if mesh is null.

### get_blend_shape_value {#get_blend_shape_value}

```gdscript
float get_blend_shape_value(blend_shape_idx: int) const
```

Returns the value of the blend shape at the given blend_shape_idx. Returns 0.0 and produces an error if mesh is null or doesn't have a blend shape at that index.

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skeleton_path {#get_skeleton_path}

```gdscript
NodePath get_skeleton_path()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skin {#get_skin}

```gdscript
Skin get_skin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_skin_reference {#get_skin_reference}

```gdscript
SkinReference get_skin_reference() const
```

Returns the internal SkinReference containing the skeleton's RID attached to this RID. See also Resource.get_rid(), SkinReference.get_skeleton(), and RenderingServer.instance_attach_skeleton().

### get_surface_override_material {#get_surface_override_material}

```gdscript
Material get_surface_override_material(surface: int) const
```

Returns the override Material for the specified surface of the Mesh resource. See also get_surface_override_material_count().

Note: This returns the Material associated to the MeshInstance3D's Surface Material Override properties, not the material within the Mesh resource. To get the material within the Mesh resource, use Mesh.surface_get_material() instead.

### get_surface_override_material_count {#get_surface_override_material_count}

```gdscript
int get_surface_override_material_count() const
```

Returns the number of surface override materials. This is equivalent to Mesh.get_surface_count(). See also get_surface_override_material().

### set_blend_shape_value {#set_blend_shape_value}

```gdscript
void set_blend_shape_value(blend_shape_idx: int, value: float)
```

Sets the value of the blend shape at blend_shape_idx to value. Produces an error if mesh is null or doesn't have a blend shape at that index.

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: Mesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skeleton_path {#set_skeleton_path}

```gdscript
void set_skeleton_path(skeleton_path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_skin {#set_skin}

```gdscript
void set_skin(skin: Skin)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_surface_override_material {#set_surface_override_material}

```gdscript
void set_surface_override_material(surface: int, material: Material)
```

Sets the override material for the specified surface of the Mesh resource. This material is associated with this MeshInstance3D rather than with mesh.

Note: This assigns the Material associated to the MeshInstance3D's Surface Material Override properties, not the material within the Mesh resource. To set the material within the Mesh resource, use Mesh.surface_set_material() instead.

## 属性

### mesh {#mesh}

```gdscript
mesh: Mesh
```

- void set_mesh(value: Mesh)
- Mesh get_mesh()

The Mesh resource for the instance.

### skin {#skin}

```gdscript
skin: Skin
```

- void set_skin(value: Skin)
- Skin get_skin()

The Skin to be used by this instance.

### skeleton {#skeleton}

```gdscript
skeleton: NodePath
```

- void set_skeleton_path(value: NodePath)
- NodePath get_skeleton_path()

NodePath to the Skeleton3D associated with the instance.

Note: The default value of this property has changed in Godot 4.6. Enable ProjectSettings.animation/compatibility/default_parent_skeleton_in_mesh_instance_3d if the old behavior is needed for compatibility.
