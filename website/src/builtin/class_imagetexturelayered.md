# ImageTextureLayered

**继承**：[TextureLayered](class_texturelayered.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ImageTextureLayered**

> Base class for Texture2DArray, Cubemap and CubemapArray. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also Texture3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base class for Texture2DArray, Cubemap and CubemapArray. Cannot be used directly, but contains all the functions necessary for accessing the derived resource types. See also Texture3D.

</details>

## 方法

### create_from_images {#create_from_images}

```gdscript
Error create_from_images(images: typedarray::Image)
```

Creates an ImageTextureLayered from an array of Images. See Image.create() for the expected data format. The first image decides the width, height, image format and mipmapping setting. The other images must have the same width, height, image format and mipmapping setting.

Each Image represents one layer.

```gdscript
# Fill in an array of Images with different colors.
var images = []
const LAYERS = 6
for i in LAYERS:
    var image = Image.create_empty(128, 128, false, Image.FORMAT_RGB8)
    if i % 3 == 0:
        image.fill(Color.RED)
    elif i % 3 == 1:
        image.fill(Color.GREEN)
    else:
        image.fill(Color.BLUE)
    images.push_back(image)
# Create and save a 2D texture array. The array of images must have at least 1 Image.
var texture_2d_array = Texture2DArray.new()
texture_2d_array.create_from_images(images)
ResourceSaver.save(texture_2d_array, "res://texture_2d_array.res", ResourceSaver.FLAG_COMPRESS)
# Create and save a cubemap. The array of images must have exactly 6 Images.
# The cubemap's images are specified in this order: X+, X-, Y+, Y-, Z+, Z-
# (in Godot's coordinate system, so Y+ is "up" and Z- is "forward").
var cubemap = Cubemap.new()
cubemap.create_from_images(images)
ResourceSaver.save(cubemap, "res://cubemap.res", ResourceSaver.FLAG_COMPRESS)
# Create and save a cubemap array. The array of images must have a multiple of 6 Images.
# Each cubemap's images are specified in this order: X+, X-, Y+, Y-, Z+, Z-
# (in Godot's coordinate system, so Y+ is "up" and Z- is "forward").
var cubemap_array = CubemapArray.new()
cubemap_array.create_from_images(images)
ResourceSaver.save(cubemap_array, "res://cubemap_array.res", ResourceSaver.FLAG_COMPRESS)
```

### update_layer {#update_layer}

```gdscript
void update_layer(image: Image, layer: int)
```

Replaces the existing Image data at the given layer with this new image.

The given Image must have the same width, height, image format, and mipmapping flag as the rest of the referenced images.

If the image format is unsupported, it will be decompressed and converted to a similar and supported Format.

The update is immediate: it's synchronized with drawing.
