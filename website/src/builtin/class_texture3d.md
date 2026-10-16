# Texture3D

**继承**：[Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Texture3D**

> Base class for ImageTexture3D and CompressedTexture3D. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. Texture3D is the base class for all 3-dimensional texture types. See also TextureLayered.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base class for ImageTexture3D and CompressedTexture3D. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. Texture3D is the base class for all 3-dimensional texture types. See also TextureLayered.

All images need to have the same width, height and number of mipmap levels.

To create such a texture file yourself, reimport your image files using the Godot Editor import presets.

</details>

## 方法

### _get_data {#_get_data}

```gdscript
typedarray::Image _get_data() virtual const
```

Called when the Texture3D's data is queried.

### _get_depth {#_get_depth}

```gdscript
int _get_depth() virtual const
```

Called when the Texture3D's depth is queried.

### _get_format {#_get_format}

```gdscript
Image.Format _get_format() virtual const
```

Called when the Texture3D's format is queried.

### _get_height {#_get_height}

```gdscript
int _get_height() virtual const
```

Called when the Texture3D's height is queried.

### _get_width {#_get_width}

```gdscript
int _get_width() virtual const
```

Called when the Texture3D's width is queried.

### _has_mipmaps {#_has_mipmaps}

```gdscript
bool _has_mipmaps() virtual const
```

Called when the presence of mipmaps in the Texture3D is queried.

### create_placeholder {#create_placeholder}

```gdscript
Resource create_placeholder() const
```

Creates a placeholder version of this resource (PlaceholderTexture3D).

### get_data {#get_data}

```gdscript
typedarray::Image get_data() const
```

Returns the Texture3D's data as an array of Images. Each Image represents a slice of the Texture3D, with different slices mapping to different depth (Z axis) levels.

### get_depth {#get_depth}

```gdscript
int get_depth() const
```

Returns the Texture3D's depth in pixels. Depth is typically represented by the Z axis (a dimension not present in Texture2D).

### get_format {#get_format}

```gdscript
Image.Format get_format() const
```

Returns the current format being used by this texture.

### get_height {#get_height}

```gdscript
int get_height() const
```

Returns the Texture3D's height in pixels. Width is typically represented by the Y axis.

### get_width {#get_width}

```gdscript
int get_width() const
```

Returns the Texture3D's width in pixels. Width is typically represented by the X axis.

### has_mipmaps {#has_mipmaps}

```gdscript
bool has_mipmaps() const
```

Returns true if the Texture3D has generated mipmaps.
