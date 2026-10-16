# CompressedTexture3D

**继承**：[Texture3D](class_texture3d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CompressedTexture3D**

> CompressedTexture3D is the VRAM-compressed counterpart of ImageTexture3D. The file extension for CompressedTexture3D files is .ctex3d. This file format is internal to Godot; it is created by importing other image formats with the import system.

<details><summary>官方英文描述（中文精译进行中）</summary>

CompressedTexture3D is the VRAM-compressed counterpart of ImageTexture3D. The file extension for CompressedTexture3D files is .ctex3d. This file format is internal to Godot; it is created by importing other image formats with the import system.

CompressedTexture3D uses VRAM compression, which allows to reduce memory usage on the GPU when rendering the texture. This also improves loading times, as VRAM-compressed textures are faster to load compared to textures using lossless compression. VRAM compression can exhibit noticeable artifacts and is intended to be used for 3D rendering, not 2D.

See Texture3D for a general description of 3D textures.

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

Loads the texture from the specified path.

## 属性

### load_path {#load_path}

```gdscript
load_path: String
```

- Error load(path: String)
- String get_load_path()

The CompressedTexture3D's file path to a .ctex3d file.
