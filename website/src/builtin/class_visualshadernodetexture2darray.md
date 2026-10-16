# VisualShaderNodeTexture2DArray

**继承**：[VisualShaderNodeSample3D](class_visualshadernodesample3d.md) → [VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeTexture2DArray**

> Translated to uniform sampler2DArray in the shader language.

<details><summary>官方英文描述</summary>

Translated to uniform sampler2DArray in the shader language.

</details>

## 方法

### get_texture_array {#get_texture_array}

```gdscript
TextureLayered get_texture_array() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_array {#set_texture_array}

```gdscript
void set_texture_array(value: TextureLayered)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### texture_array {#texture_array}

```gdscript
texture_array: Texture2DArray,CompressedTexture2DArray,PlaceholderTexture2DArray,Texture2DArrayRD
```

- void set_texture_array(value: TextureLayered)
- TextureLayered get_texture_array()

A source texture array. Used if VisualShaderNodeSample3D.source is set to VisualShaderNodeSample3D.SOURCE_TEXTURE.
