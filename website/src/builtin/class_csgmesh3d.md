# CSGMesh3D

**继承**：[CSGPrimitive3D](class_csgprimitive3d.md) → [CSGShape3D](class_csgshape3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CSGMesh3D**

> This CSG node allows you to use any mesh resource as a CSG shape, provided it is manifold. A manifold shape is closed, does not self-intersect, does not contain internal faces and has no edges that connect to more than two faces. See also CSGPolygon3D for drawing 2D extruded polygons to be used as CSG nodes.

<details><summary>官方英文描述（中文精译进行中）</summary>

This CSG node allows you to use any mesh resource as a CSG shape, provided it is manifold. A manifold shape is closed, does not self-intersect, does not contain internal faces and has no edges that connect to more than two faces. See also CSGPolygon3D for drawing 2D extruded polygons to be used as CSG nodes.

Note: CSG nodes are intended to be used for level prototyping. Creating CSG nodes has a significant CPU cost compared to creating a MeshInstance3D with a PrimitiveMesh. Moving a CSG node within another CSG node also has a significant CPU cost, so it should be avoided during gameplay.

</details>

## 方法

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: Mesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mesh {#mesh}

```gdscript
mesh: Mesh,-PlaneMesh,-PointMesh,-QuadMesh,-RibbonTrailMesh
```

- void set_mesh(value: Mesh)
- Mesh get_mesh()

The Mesh resource to use as a CSG shape.

Note: Some Mesh types such as PlaneMesh, PointMesh, QuadMesh, and RibbonTrailMesh are excluded from the type hint for this property, as these primitives are non-manifold and thus not compatible with the CSG algorithm.

Note: When using an ArrayMesh, all vertex attributes except Mesh.ARRAY_VERTEX, Mesh.ARRAY_NORMAL and Mesh.ARRAY_TEX_UV are left unused. Only Mesh.ARRAY_VERTEX and Mesh.ARRAY_TEX_UV will be passed to the GPU.

Mesh.ARRAY_NORMAL is only used to determine which faces require the use of flat shading. By default, CSGMesh will ignore the mesh's vertex normals, recalculate them for each vertex and use a smooth shader. If a flat shader is required for a face, ensure that all vertex normals of the face are approximately equal.

### material {#material}

```gdscript
material: BaseMaterial3D,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The Material used in drawing the CSG shape.
