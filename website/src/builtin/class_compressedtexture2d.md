# CompressedTexture2D

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CompressedTexture2D**

> A texture that is loaded from a .ctex file. This file format is internal to Godot; it is created by importing other image formats with the import system. CompressedTexture2D can use one of 4 compression methods (including a lack of any compression):

<details><summary>官方英文描述</summary>

A texture that is loaded from a .ctex file. This file format is internal to Godot; it is created by importing other image formats with the import system. CompressedTexture2D can use one of 4 compression methods (including a lack of any compression):

- Lossless (WebP or PNG, uncompressed on the GPU)
- Lossy (WebP, uncompressed on the GPU)
- VRAM Compressed (compressed on the GPU)
- VRAM Uncompressed (uncompressed on the GPU)
- Basis Universal (compressed on the GPU. Lower file sizes than VRAM Compressed, but slower to compress and lower quality than VRAM Compressed)

Only VRAM Compressed actually reduces the memory usage on the GPU. The Lossless and Lossy compression methods will reduce the required storage on disk, but they will not reduce memory usage on the GPU as the texture is sent to the GPU uncompressed.

Using VRAM Compressed also improves loading times, as VRAM-compressed textures are faster to load compared to textures using lossless or lossy compression. VRAM compression can exhibit noticeable artifacts and is intended to be used for 3D rendering, not 2D.

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

The CompressedTexture2D's file path to a .ctex file.
