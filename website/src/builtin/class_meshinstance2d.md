# MeshInstance2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **MeshInstance2D**

> Node used for displaying a Mesh in 2D. This can be faster to render compared to displaying a Sprite2D node with large transparent areas, especially if the node takes up a lot of space on screen at high viewport resolutions. This is because using a mesh designed to fit the sprite's opaque areas will reduce GPU fill rate utilization (at the cost of increased vertex processing utilization).

<details><summary>官方英文描述（中文精译进行中）</summary>

Node used for displaying a Mesh in 2D. This can be faster to render compared to displaying a Sprite2D node with large transparent areas, especially if the node takes up a lot of space on screen at high viewport resolutions. This is because using a mesh designed to fit the sprite's opaque areas will reduce GPU fill rate utilization (at the cost of increased vertex processing utilization).

When a Mesh has to be instantiated more than thousands of times close to each other, consider using a MultiMesh in a MultiMeshInstance2D instead.

A MeshInstance2D can be created from an existing Sprite2D via a tool in the editor toolbar. Select the Sprite2D node, then choose Sprite2D > Convert to MeshInstance2D at the top of the 2D editor viewport.

</details>

## 方法

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture {#get_texture}

```gdscript
Texture2D get_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: Mesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture {#set_texture}

```gdscript
void set_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mesh {#mesh}

```gdscript
mesh: Mesh
```

- void set_mesh(value: Mesh)
- Mesh get_mesh()

The Mesh that will be drawn by the MeshInstance2D.

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
