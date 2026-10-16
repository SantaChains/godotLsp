# ImageFormatLoaderExtension

**继承**：[ImageFormatLoader](class_imageformatloader.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ImageFormatLoaderExtension**

> The engine supports multiple image formats out of the box (PNG, SVG, JPEG, WebP to name a few), but you can choose to implement support for additional image formats by extending this class.

<details><summary>官方英文描述（中文精译进行中）</summary>

The engine supports multiple image formats out of the box (PNG, SVG, JPEG, WebP to name a few), but you can choose to implement support for additional image formats by extending this class.

Be sure to respect the documented return types and values. You should create an instance of it, and call add_format_loader() to register that loader during the initialization phase.

</details>

## 方法

### _get_recognized_extensions {#_get_recognized_extensions}

```gdscript
PackedStringArray _get_recognized_extensions() virtual const
```

Returns the list of file extensions for this image format. Files with the given extensions will be treated as image file and loaded using this class.

### _load_image {#_load_image}

```gdscript
Error _load_image(image: Image, fileaccess: FileAccess, flags: bitfield::ImageFormatLoader.LoaderFlags, scale: float) virtual
```

Loads the content of fileaccess into the provided image.

### add_format_loader {#add_format_loader}

```gdscript
void add_format_loader()
```

Add this format loader to the engine, allowing it to recognize the file extensions returned by _get_recognized_extensions().

### remove_format_loader {#remove_format_loader}

```gdscript
void remove_format_loader()
```

Remove this format loader from the engine.
