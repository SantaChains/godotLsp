# TextureLayered

**继承**：[Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TextureLayered**

> Base class for ImageTextureLayered and CompressedTextureLayered. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also Texture3D.

<details><summary>官方英文描述</summary>

Base class for ImageTextureLayered and CompressedTextureLayered. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also Texture3D.

Data is set on a per-layer basis. For Texture2DArrays, the layer specifies the array layer.

All images need to have the same width, height and number of mipmap levels.

A TextureLayered can be loaded with ResourceLoader.load().

Internally, Godot maps these files to their respective counterparts in the target rendering driver (Vulkan, OpenGL3).

</details>

## 方法

### _get_format {#_get_format}

```gdscript
Image.Format _get_format() virtual const
```

Called when the TextureLayered's format is queried.

### _get_height {#_get_height}

```gdscript
int _get_height() virtual const
```

Called when the TextureLayered's height is queried.

### _get_layer_data {#_get_layer_data}

```gdscript
Image _get_layer_data(layer_index: int) virtual const
```

Called when the data for a layer in the TextureLayered is queried.

### _get_layered_type {#_get_layered_type}

```gdscript
int _get_layered_type() virtual const
```

Called when the layers' type in the TextureLayered is queried.

### _get_layers {#_get_layers}

```gdscript
int _get_layers() virtual const
```

Called when the number of layers in the TextureLayered is queried.

### _get_width {#_get_width}

```gdscript
int _get_width() virtual const
```

Called when the TextureLayered's width queried.

### _has_mipmaps {#_has_mipmaps}

```gdscript
bool _has_mipmaps() virtual const
```

Called when the presence of mipmaps in the TextureLayered is queried.

### get_format {#get_format}

```gdscript
Image.Format get_format() const
```

Returns the current format being used by this texture.

### get_height {#get_height}

```gdscript
int get_height() const
```

Returns the height of the texture in pixels. Height is typically represented by the Y axis.

### get_layer_data {#get_layer_data}

```gdscript
Image get_layer_data(layer: int) const
```

Returns an Image resource with the data from specified layer.

### get_layered_type {#get_layered_type}

```gdscript
TextureLayered.LayeredType get_layered_type() const
```

Returns the TextureLayered's type. The type determines how the data is accessed, with cubemaps having special types.

### get_layers {#get_layers}

```gdscript
int get_layers() const
```

Returns the number of referenced Images.

### get_width {#get_width}

```gdscript
int get_width() const
```

Returns the width of the texture in pixels. Width is typically represented by the X axis.

### has_mipmaps {#has_mipmaps}

```gdscript
bool has_mipmaps() const
```

Returns true if the layers have generated mipmaps.

## 枚举

### LayeredType {#enum-LayeredType}

```gdscript
enum LayeredType { LAYERED_TYPE_2D_ARRAY = 0, LAYERED_TYPE_CUBEMAP = 1, LAYERED_TYPE_CUBEMAP_ARRAY = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LAYERED_TYPE_2D_ARRAY {#LAYERED_TYPE_2D_ARRAY}

```gdscript
LAYERED_TYPE_2D_ARRAY = 0
```

Texture is a generic Texture2DArray.

#### LAYERED_TYPE_CUBEMAP {#LAYERED_TYPE_CUBEMAP}

```gdscript
LAYERED_TYPE_CUBEMAP = 1
```

Texture is a Cubemap, with each side in its own layer (6 in total).

#### LAYERED_TYPE_CUBEMAP_ARRAY {#LAYERED_TYPE_CUBEMAP_ARRAY}

```gdscript
LAYERED_TYPE_CUBEMAP_ARRAY = 2
```

Texture is a CubemapArray, with each cubemap being made of 6 layers.

---
