# CSGPrimitive3D

**继承**：[CSGShape3D](class_csgshape3d.md) → [GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **CSGPrimitive3D**

> Parent class for various CSG primitives. It contains code and functionality that is common between them. It cannot be used directly. Instead use one of the various classes that inherit from it.

<details><summary>官方英文描述</summary>

Parent class for various CSG primitives. It contains code and functionality that is common between them. It cannot be used directly. Instead use one of the various classes that inherit from it.

Note: CSG nodes are intended to be used for level prototyping. Creating CSG nodes has a significant CPU cost compared to creating a MeshInstance3D with a PrimitiveMesh. Moving a CSG node within another CSG node also has a significant CPU cost, so it should be avoided during gameplay.

</details>

## 方法

### get_flip_faces {#get_flip_faces}

```gdscript
bool get_flip_faces()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flip_faces {#set_flip_faces}

```gdscript
void set_flip_faces(flip_faces: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### flip_faces {#flip_faces}

```gdscript
flip_faces: bool
```

- void set_flip_faces(value: bool)
- bool get_flip_faces()

If set, the order of the vertices in each triangle are reversed resulting in the backside of the mesh being drawn.
