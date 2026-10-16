# TextureLayeredRD

**继承**：[TextureLayered](class_texturelayered.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TextureLayeredRD**

> Base class for Texture2DArrayRD, TextureCubemapRD and TextureCubemapArrayRD. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base class for Texture2DArrayRD, TextureCubemapRD and TextureCubemapArrayRD. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types.

Note: TextureLayeredRD is intended for low-level usage with RenderingDevice. For most use cases, use TextureLayered instead.

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
