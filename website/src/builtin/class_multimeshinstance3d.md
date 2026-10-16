# MultiMeshInstance3D

**继承**：[GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **MultiMeshInstance3D**

> MultiMeshInstance3D is a specialized node to instance GeometryInstance3Ds based on a MultiMesh resource.

<details><summary>官方英文描述</summary>

MultiMeshInstance3D is a specialized node to instance GeometryInstance3Ds based on a MultiMesh resource.

This is useful to optimize the rendering of a high number of instances of a given mesh (for example trees in a forest or grass strands).

</details>

## 方法

### get_multimesh {#get_multimesh}

```gdscript
MultiMesh get_multimesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_multimesh {#set_multimesh}

```gdscript
void set_multimesh(multimesh: MultiMesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### multimesh {#multimesh}

```gdscript
multimesh: MultiMesh
```

- void set_multimesh(value: MultiMesh)
- MultiMesh get_multimesh()

The MultiMesh resource that will be used and shared among all instances of the MultiMeshInstance3D.
