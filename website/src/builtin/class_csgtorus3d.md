# CSGTorus3D

**继承**：[CSGPrimitive3D](class_csgprimitive3d.md) → [CSGShape3D](class_csgshape3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CSGTorus3D**

> This node allows you to create a torus for use with the CSG system.

<details><summary>官方英文描述</summary>

This node allows you to create a torus for use with the CSG system.

Note: CSG nodes are intended to be used for level prototyping. Creating CSG nodes has a significant CPU cost compared to creating a MeshInstance3D with a PrimitiveMesh. Moving a CSG node within another CSG node also has a significant CPU cost, so it should be avoided during gameplay.

</details>

## 方法

### get_inner_radius {#get_inner_radius}

```gdscript
float get_inner_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outer_radius {#get_outer_radius}

```gdscript
float get_outer_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ring_sides {#get_ring_sides}

```gdscript
int get_ring_sides() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sides {#get_sides}

```gdscript
int get_sides() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_smooth_faces {#get_smooth_faces}

```gdscript
bool get_smooth_faces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inner_radius {#set_inner_radius}

```gdscript
void set_inner_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outer_radius {#set_outer_radius}

```gdscript
void set_outer_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ring_sides {#set_ring_sides}

```gdscript
void set_ring_sides(sides: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sides {#set_sides}

```gdscript
void set_sides(sides: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_smooth_faces {#set_smooth_faces}

```gdscript
void set_smooth_faces(smooth_faces: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### inner_radius {#inner_radius}

```gdscript
inner_radius: float
```

- void set_inner_radius(value: float)
- float get_inner_radius()

The inner radius of the torus.

### outer_radius {#outer_radius}

```gdscript
outer_radius: float
```

- void set_outer_radius(value: float)
- float get_outer_radius()

The outer radius of the torus.

### sides {#sides}

```gdscript
sides: int
```

- void set_sides(value: int)
- int get_sides()

The number of slices the torus is constructed of.

### ring_sides {#ring_sides}

```gdscript
ring_sides: int
```

- void set_ring_sides(value: int)
- int get_ring_sides()

The number of edges each ring of the torus is constructed of.

### smooth_faces {#smooth_faces}

```gdscript
smooth_faces: bool
```

- void set_smooth_faces(value: bool)
- bool get_smooth_faces()

If true the normals of the torus are set to give a smooth effect making the torus seem rounded. If false the torus will have a flat shaded look.

### material {#material}

```gdscript
material: BaseMaterial3D,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The material used to render the torus.
