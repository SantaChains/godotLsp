# DPITexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **DPITexture**

> An automatically scalable Texture2D based on an SVG image. DPITextures are used to automatically re-rasterize icons and other texture based UI theme elements to match viewport scale and font oversampling. See also ProjectSettings.display/window/stretch/mode ("canvas_items" mode) and Viewport.oversampling_override.

<details><summary>官方英文描述（中文精译进行中）</summary>

An automatically scalable Texture2D based on an SVG image. DPITextures are used to automatically re-rasterize icons and other texture based UI theme elements to match viewport scale and font oversampling. See also ProjectSettings.display/window/stretch/mode ("canvas_items" mode) and Viewport.oversampling_override.

</details>

## 方法

### create_from_string {#create_from_string}

```gdscript
static DPITexture create_from_string(source: String, scale: float = 1.0, saturation: float = 1.0, color_map: Dictionary = {})
```

Creates a new DPITexture and initializes it by allocating and setting the SVG data to source.

### get_base_scale {#get_base_scale}

```gdscript
float get_base_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_color_map {#get_color_map}

```gdscript
Dictionary get_color_map() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fix_alpha_border {#get_fix_alpha_border}

```gdscript
bool get_fix_alpha_border() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_premult_alpha {#get_premult_alpha}

```gdscript
bool get_premult_alpha() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_saturation {#get_saturation}

```gdscript
float get_saturation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scaled_rid {#get_scaled_rid}

```gdscript
RID get_scaled_rid() const
```

Returns the RID of the texture rasterized to match the oversampling of the currently drawn canvas item.

### get_source {#get_source}

```gdscript
String get_source() const
```

Returns this SVG texture's source code.

### set_base_scale {#set_base_scale}

```gdscript
void set_base_scale(base_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color_map {#set_color_map}

```gdscript
void set_color_map(color_map: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fix_alpha_border {#set_fix_alpha_border}

```gdscript
void set_fix_alpha_border(fix_alpha_border: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_premult_alpha {#set_premult_alpha}

```gdscript
void set_premult_alpha(premult_alpha: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_saturation {#set_saturation}

```gdscript
void set_saturation(saturation: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size_override {#set_size_override}

```gdscript
void set_size_override(size: Vector2i)
```

Resizes the texture to the specified dimensions.

### set_source {#set_source}

```gdscript
void set_source(source: String)
```

Sets this SVG texture's source code.

## 属性

### fix_alpha_border {#fix_alpha_border}

```gdscript
fix_alpha_border: bool
```

- void set_fix_alpha_border(value: bool)
- bool get_fix_alpha_border()

If true, puts pixels of the same surrounding color in transition from transparent to opaque areas. For textures displayed with bilinear filtering, this helps to reduce the outline effect when exporting images from an image editor.

### premult_alpha {#premult_alpha}

```gdscript
premult_alpha: bool
```

- void set_premult_alpha(value: bool)
- bool get_premult_alpha()

An alternative to fixing darkened borders with fix_alpha_border is to use premultiplied alpha. By enabling this option, the texture will be converted to this format. A premultiplied alpha texture requires specific materials to be displayed correctly:

- In 2D, a CanvasItemMaterial will need to be created and configured to use the CanvasItemMaterial.BLEND_MODE_PREMULT_ALPHA blend mode on CanvasItems that use this texture. In custom canvas_item shaders, render_mode blend_premul_alpha; should be used.
- In 3D, a BaseMaterial3D will need to be created and configured to use the BaseMaterial3D.BLEND_MODE_PREMULT_ALPHA blend mode on materials that use this texture. In custom spatial shaders, render_mode blend_premul_alpha; should be used.

### base_scale {#base_scale}

```gdscript
base_scale: float
```

- void set_base_scale(value: float)
- float get_base_scale()

Texture scale. 1.0 is the original SVG size. Higher values result in a larger image.

### saturation {#saturation}

```gdscript
saturation: float
```

- void set_saturation(value: float)
- float get_saturation()

Overrides texture saturation.

### color_map {#color_map}

```gdscript
color_map: typeddictionary::Color;Color
```

- void set_color_map(value: Dictionary)
- Dictionary get_color_map()

If set, remaps texture colors according to Color-Color map.
