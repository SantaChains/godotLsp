# SystemFont

**继承**：[Font](class_font.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SystemFont**

> SystemFont loads a font from a system font with the first matching name from font_names.

<details><summary>官方英文描述（中文精译进行中）</summary>

SystemFont loads a font from a system font with the first matching name from font_names.

It will attempt to match font style, but it's not guaranteed.

The returned font might be part of a font collection or be a variable font with OpenType "weight", "width" and/or "italic" features set.

You can create FontVariation of the system font for precise control over its features.

Note: This class is implemented on iOS, Linux, macOS and Windows, on other platforms it will fallback to default theme font.

</details>

## 方法

### get_antialiasing {#get_antialiasing}

```gdscript
TextServer.FontAntialiasing get_antialiasing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_disable_embedded_bitmaps {#get_disable_embedded_bitmaps}

```gdscript
bool get_disable_embedded_bitmaps() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_font_italic {#get_font_italic}

```gdscript
bool get_font_italic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_font_names {#get_font_names}

```gdscript
PackedStringArray get_font_names() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_generate_mipmaps {#get_generate_mipmaps}

```gdscript
bool get_generate_mipmaps() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hinting {#get_hinting}

```gdscript
TextServer.Hinting get_hinting() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_keep_rounding_remainders {#get_keep_rounding_remainders}

```gdscript
bool get_keep_rounding_remainders() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_msdf_pixel_range {#get_msdf_pixel_range}

```gdscript
int get_msdf_pixel_range() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_msdf_size {#get_msdf_size}

```gdscript
int get_msdf_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_oversampling {#get_oversampling}

```gdscript
float get_oversampling() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subpixel_positioning {#get_subpixel_positioning}

```gdscript
TextServer.SubpixelPositioning get_subpixel_positioning() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_allow_system_fallback {#is_allow_system_fallback}

```gdscript
bool is_allow_system_fallback() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_force_autohinter {#is_force_autohinter}

```gdscript
bool is_force_autohinter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_modulate_color_glyphs {#is_modulate_color_glyphs}

```gdscript
bool is_modulate_color_glyphs() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_multichannel_signed_distance_field {#is_multichannel_signed_distance_field}

```gdscript
bool is_multichannel_signed_distance_field() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_system_fallback {#set_allow_system_fallback}

```gdscript
void set_allow_system_fallback(allow_system_fallback: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_antialiasing {#set_antialiasing}

```gdscript
void set_antialiasing(antialiasing: TextServer.FontAntialiasing)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_embedded_bitmaps {#set_disable_embedded_bitmaps}

```gdscript
void set_disable_embedded_bitmaps(disable_embedded_bitmaps: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font_italic {#set_font_italic}

```gdscript
void set_font_italic(italic: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font_names {#set_font_names}

```gdscript
void set_font_names(names: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font_stretch {#set_font_stretch}

```gdscript
void set_font_stretch(stretch: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font_weight {#set_font_weight}

```gdscript
void set_font_weight(weight: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_force_autohinter {#set_force_autohinter}

```gdscript
void set_force_autohinter(force_autohinter: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_generate_mipmaps {#set_generate_mipmaps}

```gdscript
void set_generate_mipmaps(generate_mipmaps: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hinting {#set_hinting}

```gdscript
void set_hinting(hinting: TextServer.Hinting)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keep_rounding_remainders {#set_keep_rounding_remainders}

```gdscript
void set_keep_rounding_remainders(keep_rounding_remainders: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_modulate_color_glyphs {#set_modulate_color_glyphs}

```gdscript
void set_modulate_color_glyphs(modulate: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_msdf_pixel_range {#set_msdf_pixel_range}

```gdscript
void set_msdf_pixel_range(msdf_pixel_range: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_msdf_size {#set_msdf_size}

```gdscript
void set_msdf_size(msdf_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_multichannel_signed_distance_field {#set_multichannel_signed_distance_field}

```gdscript
void set_multichannel_signed_distance_field(msdf: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_oversampling {#set_oversampling}

```gdscript
void set_oversampling(oversampling: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subpixel_positioning {#set_subpixel_positioning}

```gdscript
void set_subpixel_positioning(subpixel_positioning: TextServer.SubpixelPositioning)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### font_names {#font_names}

```gdscript
font_names: PackedStringArray
```

- void set_font_names(value: PackedStringArray)
- PackedStringArray get_font_names()

Array of font family names to search, first matching font found is used.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedStringArray for more details.

### font_italic {#font_italic}

```gdscript
font_italic: bool
```

- void set_font_italic(value: bool)
- bool get_font_italic()

If set to true, italic or oblique font is preferred.

### font_weight {#font_weight}

```gdscript
font_weight: int
```

- void set_font_weight(value: int)
- int get_font_weight()

Preferred weight (boldness) of the font. A value in the 100...999 range, normal font weight is 400, bold font weight is 700.

### font_stretch {#font_stretch}

```gdscript
font_stretch: int
```

- void set_font_stretch(value: int)
- int get_font_stretch()

Preferred font stretch amount, compared to a normal width. A percentage value between 50% and 200%.

### antialiasing {#antialiasing}

```gdscript
antialiasing: int
```

- void set_antialiasing(value: FontAntialiasing)
- FontAntialiasing get_antialiasing()

Font anti-aliasing mode.

### generate_mipmaps {#generate_mipmaps}

```gdscript
generate_mipmaps: bool
```

- void set_generate_mipmaps(value: bool)
- bool get_generate_mipmaps()

If set to true, generate mipmaps for the font textures.

### disable_embedded_bitmaps {#disable_embedded_bitmaps}

```gdscript
disable_embedded_bitmaps: bool
```

- void set_disable_embedded_bitmaps(value: bool)
- bool get_disable_embedded_bitmaps()

If set to true, embedded font bitmap loading is disabled (bitmap-only and color fonts ignore this property).

### allow_system_fallback {#allow_system_fallback}

```gdscript
allow_system_fallback: bool
```

- void set_allow_system_fallback(value: bool)
- bool is_allow_system_fallback()

If set to true, system fonts can be automatically used as fallbacks.

### force_autohinter {#force_autohinter}

```gdscript
force_autohinter: bool
```

- void set_force_autohinter(value: bool)
- bool is_force_autohinter()

If set to true, auto-hinting is supported and preferred over font built-in hinting.

### modulate_color_glyphs {#modulate_color_glyphs}

```gdscript
modulate_color_glyphs: bool
```

- void set_modulate_color_glyphs(value: bool)
- bool is_modulate_color_glyphs()

If set to true, color modulation is applied when drawing colored glyphs, otherwise it's applied to the monochrome glyphs only.

### hinting {#hinting}

```gdscript
hinting: int
```

- void set_hinting(value: Hinting)
- Hinting get_hinting()

Font hinting mode.

### subpixel_positioning {#subpixel_positioning}

```gdscript
subpixel_positioning: int
```

- void set_subpixel_positioning(value: SubpixelPositioning)
- SubpixelPositioning get_subpixel_positioning()

Font glyph subpixel positioning mode. Subpixel positioning provides shaper text and better kerning for smaller font sizes, at the cost of memory usage and font rasterization speed. Use TextServer.SUBPIXEL_POSITIONING_AUTO to automatically enable it based on the font size.

### keep_rounding_remainders {#keep_rounding_remainders}

```gdscript
keep_rounding_remainders: bool
```

- void set_keep_rounding_remainders(value: bool)
- bool get_keep_rounding_remainders()

If set to true, when aligning glyphs to the pixel boundaries rounding remainders are accumulated to ensure more uniform glyph distribution. This setting has no effect if subpixel positioning is enabled.

### multichannel_signed_distance_field {#multichannel_signed_distance_field}

```gdscript
multichannel_signed_distance_field: bool
```

- void set_multichannel_signed_distance_field(value: bool)
- bool is_multichannel_signed_distance_field()

If set to true, glyphs of all sizes are rendered using single multichannel signed distance field generated from the dynamic font vector data.

### msdf_pixel_range {#msdf_pixel_range}

```gdscript
msdf_pixel_range: int
```

- void set_msdf_pixel_range(value: int)
- int get_msdf_pixel_range()

The width of the range around the shape between the minimum and maximum representable signed distance. If using font outlines, msdf_pixel_range must be set to at least twice the size of the largest font outline. The default msdf_pixel_range value of 16 allows outline sizes up to 8 to look correct.

### msdf_size {#msdf_size}

```gdscript
msdf_size: int
```

- void set_msdf_size(value: int)
- int get_msdf_size()

Source font size used to generate MSDF textures. Higher values allow for more precision, but are slower to render and require more memory. Only increase this value if you notice a visible lack of precision in glyph rendering.

### oversampling {#oversampling}

```gdscript
oversampling: float
```

- void set_oversampling(value: float)
- float get_oversampling()

If set to a positive value, overrides the oversampling factor of the viewport this font is used in. See Viewport.oversampling. This value doesn't override the oversampling parameter of draw_* methods.
