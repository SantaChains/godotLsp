# CSGCylinder3D

**继承**：[CSGPrimitive3D](class_csgprimitive3d.md) → [CSGShape3D](class_csgshape3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CSGCylinder3D**

> This node allows you to create a cylinder (or cone) for use with the CSG system.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node allows you to create a cylinder (or cone) for use with the CSG system.

Note: CSG nodes are intended to be used for level prototyping. Creating CSG nodes has a significant CPU cost compared to creating a MeshInstance3D with a PrimitiveMesh. Moving a CSG node within another CSG node also has a significant CPU cost, so it should be avoided during gameplay.

</details>

## 方法

### get_height {#get_height}

```gdscript
float get_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radius {#get_radius}

```gdscript
float get_radius() const
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

### is_cone {#is_cone}

```gdscript
bool is_cone() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cone {#set_cone}

```gdscript
void set_cone(cone: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_height {#set_height}

```gdscript
void set_height(height: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radius {#set_radius}

```gdscript
void set_radius(radius: float)
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

### radius {#radius}

```gdscript
radius: float
```

- void set_radius(value: float)
- float get_radius()

The radius of the cylinder.

### height {#height}

```gdscript
height: float
```

- void set_height(value: float)
- float get_height()

The height of the cylinder.

### sides {#sides}

```gdscript
sides: int
```

- void set_sides(value: int)
- int get_sides()

The number of sides of the cylinder, the higher this number the more detail there will be in the cylinder.

### cone {#cone}

```gdscript
cone: bool
```

- void set_cone(value: bool)
- bool is_cone()

If true a cone is created, the radius will only apply to one side.

### smooth_faces {#smooth_faces}

```gdscript
smooth_faces: bool
```

- void set_smooth_faces(value: bool)
- bool get_smooth_faces()

If true the normals of the cylinder are set to give a smooth effect making the cylinder seem rounded. If false the cylinder will have a flat shaded look.

### material {#material}

```gdscript
material: BaseMaterial3D,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The material used to render the cylinder.
