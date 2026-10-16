# CSGSphere3D

**继承**：[CSGPrimitive3D](class_csgprimitive3d.md) → [CSGShape3D](class_csgshape3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CSGSphere3D**

> This node allows you to create a sphere for use with the CSG system.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node allows you to create a sphere for use with the CSG system.

Note: CSG nodes are intended to be used for level prototyping. Creating CSG nodes has a significant CPU cost compared to creating a MeshInstance3D with a PrimitiveMesh. Moving a CSG node within another CSG node also has a significant CPU cost, so it should be avoided during gameplay.

</details>

## 方法

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radial_segments {#get_radial_segments}

```gdscript
int get_radial_segments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rings {#get_rings}

```gdscript
int get_rings() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_smooth_faces {#get_smooth_faces}

```gdscript
bool get_smooth_faces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radial_segments {#set_radial_segments}

```gdscript
void set_radial_segments(radial_segments: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rings {#set_rings}

```gdscript
void set_rings(rings: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_smooth_faces {#set_smooth_faces}

```gdscript
void set_smooth_faces(smooth_faces: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

Radius of the sphere.

### radial_segments {#radial_segments}

```gdscript
radial_segments: int
```

- void set_radial_segments(value: int)
- int get_radial_segments()

Number of vertical slices for the sphere.

### rings {#rings}

```gdscript
rings: int
```

- void set_rings(value: int)
- int get_rings()

Number of horizontal slices for the sphere.

### smooth_faces {#smooth_faces}

```gdscript
smooth_faces: bool
```

- void set_smooth_faces(value: bool)
- bool get_smooth_faces()

If true the normals of the sphere are set to give a smooth effect making the sphere seem rounded. If false the sphere will have a flat shaded look.

### material {#material}

```gdscript
material: BaseMaterial3D,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The material used to render the sphere.
