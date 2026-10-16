# CSGBox3D

**继承**：[CSGPrimitive3D](class_csgprimitive3d.md) → [CSGShape3D](class_csgshape3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CSGBox3D**

> This node allows you to create a box for use with the CSG system.

<details><summary>官方英文描述（中文精译进行中）</summary>

This node allows you to create a box for use with the CSG system.

Note: CSG nodes are intended to be used for level prototyping. Creating CSG nodes has a significant CPU cost compared to creating a MeshInstance3D with a PrimitiveMesh. Moving a CSG node within another CSG node also has a significant CPU cost, so it should be avoided during gameplay.

</details>

## 方法

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size {#size}

```gdscript
size: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### material {#material}

```gdscript
material: BaseMaterial3D,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

The material used to render the box.
