# Texture3DRD

**继承**：[Texture3D](class_texture3d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Texture3DRD**

> This texture class allows you to use a 3D texture created directly on the RenderingDevice as a texture for materials, meshes, etc.

<details><summary>官方英文描述（中文精译进行中）</summary>

This texture class allows you to use a 3D texture created directly on the RenderingDevice as a texture for materials, meshes, etc.

Note: Texture3DRD is intended for low-level usage with RenderingDevice. For most use cases, use Texture3D instead.

</details>

## 方法

### get_texture_rd_rid {#get_texture_rd_rid}

```gdscript
RID get_texture_rd_rid() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_rd_rid {#set_texture_rd_rid}

```gdscript
void set_texture_rd_rid(texture_rd_rid: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture_rd_rid {#texture_rd_rid}

```gdscript
texture_rd_rid: RID
```

- void set_texture_rd_rid(value: RID)
- RID get_texture_rd_rid()

The RID of the texture object created on the RenderingDevice.
