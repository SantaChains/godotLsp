# CompressedTextureLayered

**继承**：[TextureLayered](class_texturelayered.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CompressedTextureLayered**

> Base class for CompressedTexture2DArray and CompressedTexture3D. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also TextureLayered.

<details><summary>官方英文描述</summary>

Base class for CompressedTexture2DArray and CompressedTexture3D. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also TextureLayered.

</details>

## 方法

### get_load_path {#get_load_path}

```gdscript
String get_load_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### load {#load}

```gdscript
Error load(path: String)
```

Loads the texture at path.

## 属性

### load_path {#load_path}

```gdscript
load_path: String
```

- Error load(path: String)
- String get_load_path()

The path the texture should be loaded from.
