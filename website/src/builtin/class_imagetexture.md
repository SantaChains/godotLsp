# ImageTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ImageTexture**

> A Texture2D based on an Image. For an image to be displayed, an ImageTexture has to be created from it using the create_from_image() method:

<details><summary>官方英文描述</summary>

A Texture2D based on an Image. For an image to be displayed, an ImageTexture has to be created from it using the create_from_image() method:

```gdscript
var image = Image.load_from_file("res://icon.svg")
var texture = ImageTexture.create_from_image(image)
$Sprite2D.texture = texture
```

This way, textures can be created at run-time by loading images both from within the editor and externally.

Warning: Prefer to load imported textures with @GDScript.load() over loading them from within the filesystem dynamically with Image.load(), as it may not work in exported projects:

```gdscript
var texture = load("res://icon.svg")
$Sprite2D.texture = texture
```

This is because images have to be imported as a CompressedTexture2D first to be loaded with @GDScript.load(). If you'd still like to load an image file just like any other Resource, import it as an Image resource instead, and then load it normally using the @GDScript.load() method.

Note: The image can be retrieved from an imported texture using the Texture2D.get_image() method, which returns a copy of the image:

```gdscript
var texture = load("res://icon.svg")
var image = texture.get_image()
```

An ImageTexture is not meant to be operated from within the editor interface directly, and is mostly useful for rendering images on screen dynamically via code. If you need to generate images procedurally from within the editor, consider saving and importing images as custom texture resources implementing a new EditorImportPlugin.

Note: The maximum texture size is 16384×16384 pixels due to graphics hardware limitations.

</details>

## 方法

### create_from_image {#create_from_image}

```gdscript
static ImageTexture create_from_image(image: Image)
```

Creates a new ImageTexture and initializes it by allocating and setting the data from an Image.

### set_image {#set_image}

```gdscript
void set_image(image: Image)
```

Replaces the texture's data with a new Image. This will re-allocate new memory for the texture.

If you want to update the image, but don't need to change its parameters (format, size), use update() instead for better performance.

### set_size_override {#set_size_override}

```gdscript
void set_size_override(size: Vector2i)
```

Resizes the texture to the specified dimensions.

### update {#update}

```gdscript
void update(image: Image)
```

Replaces the texture's data with a new Image.

Note: The texture has to be created using create_from_image() or initialized first with the set_image() method before it can be updated. The new image dimensions, format, and mipmaps configuration should match the existing texture's image configuration.

Use this method over set_image() if you need to update the texture frequently, which is faster than allocating additional memory for a new texture each time.

## 属性

### image {#image}

```gdscript
image: Image
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
