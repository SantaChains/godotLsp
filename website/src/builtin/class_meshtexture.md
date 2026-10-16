# MeshTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MeshTexture**

> Simple texture that uses a mesh to draw itself. It's limited because flags can't be changed and region drawing is not supported.

<details><summary>官方英文描述（中文精译进行中）</summary>

Simple texture that uses a mesh to draw itself. It's limited because flags can't be changed and region drawing is not supported.

</details>

## 方法

### get_base_texture {#get_base_texture}

```gdscript
Texture2D get_base_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_image_size {#get_image_size}

```gdscript
Vector2 get_image_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_base_texture {#set_base_texture}

```gdscript
void set_base_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_image_size {#set_image_size}

```gdscript
void set_image_size(size: Vector2)
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
mesh: Mesh
```

- void set_mesh(value: Mesh)
- Mesh get_mesh()

Sets the mesh used to draw. It must be a mesh using 2D vertices.

### base_texture {#base_texture}

```gdscript
base_texture: Texture2D
```

- void set_base_texture(value: Texture2D)
- Texture2D get_base_texture()

Sets the base texture that the Mesh will use to draw.

### image_size {#image_size}

```gdscript
image_size: Vector2
```

- void set_image_size(value: Vector2)
- Vector2 get_image_size()

Sets the size of the image, needed for reference.
