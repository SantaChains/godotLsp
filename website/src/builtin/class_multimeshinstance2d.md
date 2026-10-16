# MultiMeshInstance2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **MultiMeshInstance2D**

> MultiMeshInstance2D is a specialized node to instance a MultiMesh resource in 2D. This can be faster to render compared to displaying many Sprite2D nodes with large transparent areas, especially if the nodes take up a lot of space on screen at high viewport resolutions. This is because using a mesh designed to fit the sprites' opaque areas will reduce GPU fill rate utilization (at the cost of increased vertex processing utilization).

<details><summary>官方英文描述（中文精译进行中）</summary>

MultiMeshInstance2D is a specialized node to instance a MultiMesh resource in 2D. This can be faster to render compared to displaying many Sprite2D nodes with large transparent areas, especially if the nodes take up a lot of space on screen at high viewport resolutions. This is because using a mesh designed to fit the sprites' opaque areas will reduce GPU fill rate utilization (at the cost of increased vertex processing utilization).

Usage is the same as MultiMeshInstance3D.

</details>

## 方法

### get_multimesh {#get_multimesh}

```gdscript
MultiMesh get_multimesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_multimesh {#set_multimesh}

```gdscript
void set_multimesh(multimesh: MultiMesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### multimesh {#multimesh}

```gdscript
multimesh: MultiMesh
```

- void set_multimesh(value: MultiMesh)
- MultiMesh get_multimesh()

The MultiMesh that will be drawn by the MultiMeshInstance2D.

### texture {#texture}

```gdscript
texture: Texture2D
```

- void set_texture(value: Texture2D)
- Texture2D get_texture()

The Texture2D that will be used if using the default CanvasItemMaterial. Can be accessed as TEXTURE in CanvasItem shader.

## 信号

### texture_changed {#texture_changed}

```gdscript
signal texture_changed()
```

Emitted when the texture is changed.
