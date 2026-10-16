# PortableCompressedTexture2D

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PortableCompressedTexture2D**

> This class allows storing compressed textures as self contained (not imported) resources.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class allows storing compressed textures as self contained (not imported) resources.

For 2D usage (compressed on disk, uncompressed on VRAM), the lossy and lossless modes are recommended. For 3D usage (compressed on VRAM) it depends on the target platform.

If you intend to only use desktop, S3TC or BPTC are recommended. For only mobile, ETC2 is recommended.

For portable, self contained 3D textures that work on both desktop and mobile, Basis Universal is recommended (although it has a small quality cost and longer compression time as a tradeoff).

This resource is intended to be created from code.

</details>

## 方法

### create_from_image {#create_from_image}

```gdscript
void create_from_image(image: Image, compression_mode: PortableCompressedTexture2D.CompressionMode, normal_map: bool = false, lossy_quality: float = 0.8)
```

Initializes the compressed texture from a base image. The compression mode must be provided.

normal_map is recommended to ensure optimum quality if this image will be used as a normal map.

If lossy compression is requested, the quality setting can optionally be provided. This maps to Lossy WebP compression quality.

### get_compression_mode {#get_compression_mode}

```gdscript
PortableCompressedTexture2D.CompressionMode get_compression_mode() const
```

Return the compression mode used (valid after initialized).

### get_size_override {#get_size_override}

```gdscript
Vector2 get_size_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_keeping_all_compressed_buffers {#is_keeping_all_compressed_buffers}

```gdscript
static bool is_keeping_all_compressed_buffers()
```

Returns true if the flag is overridden for all textures of this type.

### is_keeping_compressed_buffer {#is_keeping_compressed_buffer}

```gdscript
bool is_keeping_compressed_buffer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_basisu_compressor_params {#set_basisu_compressor_params}

```gdscript
void set_basisu_compressor_params(uastc_level: int, rdo_quality_loss: float)
```

Sets the compressor parameters for Basis Universal compression. See also the settings in ResourceImporterTexture.

Note: This method must be called before create_from_image() for this to work.

### set_keep_all_compressed_buffers {#set_keep_all_compressed_buffers}

```gdscript
static void set_keep_all_compressed_buffers(keep: bool)
```

If keep is true, overrides the flag globally for all textures of this type. This is used primarily by the editor.

### set_keep_compressed_buffer {#set_keep_compressed_buffer}

```gdscript
void set_keep_compressed_buffer(keep: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size_override {#set_size_override}

```gdscript
void set_size_override(size: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### size_override {#size_override}

```gdscript
size_override: Vector2
```

- void set_size_override(value: Vector2)
- Vector2 get_size_override()

Allows overriding the texture's size (for 2D only).

### keep_compressed_buffer {#keep_compressed_buffer}

```gdscript
keep_compressed_buffer: bool
```

- void set_keep_compressed_buffer(value: bool)
- bool is_keeping_compressed_buffer()

If true, when running in the editor, this texture will keep the source-compressed data in memory, allowing the data to persist after loading. Otherwise, the source-compressed data is lost after loading and the texture can't be re-saved.

Note: This property must be set before create_from_image() for this to work.

## 枚举

### CompressionMode {#enum-CompressionMode}

```gdscript
enum CompressionMode { COMPRESSION_MODE_LOSSLESS = 0, COMPRESSION_MODE_LOSSY = 1, COMPRESSION_MODE_BASIS_UNIVERSAL = 2, COMPRESSION_MODE_S3TC = 3, COMPRESSION_MODE_ETC2 = 4, COMPRESSION_MODE_BPTC = 5, COMPRESSION_MODE_ASTC = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COMPRESSION_MODE_LOSSLESS {#COMPRESSION_MODE_LOSSLESS}

```gdscript
COMPRESSION_MODE_LOSSLESS = 0
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### COMPRESSION_MODE_LOSSY {#COMPRESSION_MODE_LOSSY}

```gdscript
COMPRESSION_MODE_LOSSY = 1
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### COMPRESSION_MODE_BASIS_UNIVERSAL {#COMPRESSION_MODE_BASIS_UNIVERSAL}

```gdscript
COMPRESSION_MODE_BASIS_UNIVERSAL = 2
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### COMPRESSION_MODE_S3TC {#COMPRESSION_MODE_S3TC}

```gdscript
COMPRESSION_MODE_S3TC = 3
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### COMPRESSION_MODE_ETC2 {#COMPRESSION_MODE_ETC2}

```gdscript
COMPRESSION_MODE_ETC2 = 4
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### COMPRESSION_MODE_BPTC {#COMPRESSION_MODE_BPTC}

```gdscript
COMPRESSION_MODE_BPTC = 5
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### COMPRESSION_MODE_ASTC {#COMPRESSION_MODE_ASTC}

```gdscript
COMPRESSION_MODE_ASTC = 6
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

---
