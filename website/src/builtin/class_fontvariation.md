# FontVariation

**继承**：[Font](class_font.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **FontVariation**

> Provides OpenType variations, simulated bold / slant, and additional font settings like OpenType features and extra spacing.

<details><summary>官方英文描述（中文精译进行中）</summary>

Provides OpenType variations, simulated bold / slant, and additional font settings like OpenType features and extra spacing.

To use simulated bold font variant:

```
var fv = FontVariation.new()
fv.base_font = load("res://BarlowCondensed-Regular.ttf")
fv.variation_embolden = 1.2
$Label.add_theme_font_override("font", fv)
$Label.add_theme_font_size_override("font_size", 64)
```

```
var fv = new FontVariation();
fv.SetBaseFont(ResourceLoader.Load("res://BarlowCondensed-Regular.ttf"));
fv.SetVariationEmbolden(1.2);
GetNode("Label").AddThemeFontOverride("font", fv);
GetNode("Label").AddThemeFontSizeOverride("font_size", 64);
```

To set the coordinate of multiple variation axes:

```gdscript
var fv = FontVariation.new();
var ts = TextServerManager.get_primary_interface()
fv.base_font = load("res://BarlowCondensed-Regular.ttf")
fv.variation_opentype = { ts.name_to_tag("wght"): 900, ts.name_to_tag("custom_hght"): 900 }
```

</details>

## 方法

### get_base_font {#get_base_font}

```gdscript
Font get_base_font() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_baseline_offset {#get_baseline_offset}

```gdscript
float get_baseline_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_palette_custom_colors {#get_palette_custom_colors}

```gdscript
PackedColorArray get_palette_custom_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_palette_index {#get_palette_index}

```gdscript
int get_palette_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_variation_embolden {#get_variation_embolden}

```gdscript
float get_variation_embolden() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_variation_face_index {#get_variation_face_index}

```gdscript
int get_variation_face_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_variation_opentype {#get_variation_opentype}

```gdscript
Dictionary get_variation_opentype() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_variation_transform {#get_variation_transform}

```gdscript
Transform2D get_variation_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_base_font {#set_base_font}

```gdscript
void set_base_font(font: Font)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_baseline_offset {#set_baseline_offset}

```gdscript
void set_baseline_offset(baseline_offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_opentype_features {#set_opentype_features}

```gdscript
void set_opentype_features(features: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_palette_custom_colors {#set_palette_custom_colors}

```gdscript
void set_palette_custom_colors(colors: PackedColorArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_palette_index {#set_palette_index}

```gdscript
void set_palette_index(palette_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_spacing {#set_spacing}

```gdscript
void set_spacing(spacing: TextServer.SpacingType, value: int)
```

Sets the spacing for spacing to value in pixels (not relative to the font size).

### set_variation_embolden {#set_variation_embolden}

```gdscript
void set_variation_embolden(strength: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_variation_face_index {#set_variation_face_index}

```gdscript
void set_variation_face_index(face_index: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_variation_opentype {#set_variation_opentype}

```gdscript
void set_variation_opentype(coords: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_variation_transform {#set_variation_transform}

```gdscript
void set_variation_transform(transform: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### base_font {#base_font}

```gdscript
base_font: Font
```

- void set_base_font(value: Font)
- Font get_base_font()

Base font used to create a variation. If not set, default Theme font is used.

### variation_opentype {#variation_opentype}

```gdscript
variation_opentype: Dictionary
```

- void set_variation_opentype(value: Dictionary)
- Dictionary get_variation_opentype()

Font OpenType variation coordinates. More info: OpenType variation tags [https://docs.microsoft.com/en-us/typography/opentype/spec/dvaraxisreg].

Note: This Dictionary uses OpenType tags as keys. Variation axes can be identified both by tags (int, e.g. 0x77678674) and names (String, e.g. wght). Some axes might be accessible by multiple names. For example, wght refers to the same axis as weight. Tags on the other hand are unique. To convert between names and tags, use TextServer.name_to_tag() and TextServer.tag_to_name().

Note: To get available variation axes of a font, use Font.get_supported_variation_list().

### variation_face_index {#variation_face_index}

```gdscript
variation_face_index: int
```

- void set_variation_face_index(value: int)
- int get_variation_face_index()

Active face index in the TrueType / OpenType collection file.

### variation_embolden {#variation_embolden}

```gdscript
variation_embolden: float
```

- void set_variation_embolden(value: float)
- float get_variation_embolden()

If is not equal to zero, emboldens the font outlines. Negative values reduce the outline thickness.

Note: Emboldened fonts might have self-intersecting outlines, which will prevent MSDF fonts and TextMesh from working correctly.

### variation_transform {#variation_transform}

```gdscript
variation_transform: Transform2D
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### opentype_features {#opentype_features}

```gdscript
opentype_features: Dictionary
```

- void set_opentype_features(value: Dictionary)
- Dictionary get_opentype_features()

A set of OpenType feature tags. More info: OpenType feature tags [https://docs.microsoft.com/en-us/typography/opentype/spec/featuretags].

### spacing_glyph {#spacing_glyph}

```gdscript
spacing_glyph: int
```

- void set_spacing(spacing: SpacingType, value: int)
- int get_spacing()

Extra spacing between graphical glyphs.

### spacing_space {#spacing_space}

```gdscript
spacing_space: int
```

- void set_spacing(spacing: SpacingType, value: int)
- int get_spacing()

Extra width of the space glyphs.

### spacing_top {#spacing_top}

```gdscript
spacing_top: int
```

- void set_spacing(spacing: SpacingType, value: int)
- int get_spacing()

Extra spacing at the top of the line in pixels.

### spacing_bottom {#spacing_bottom}

```gdscript
spacing_bottom: int
```

- void set_spacing(spacing: SpacingType, value: int)
- int get_spacing()

Extra spacing at the bottom of the line in pixels.

### baseline_offset {#baseline_offset}

```gdscript
baseline_offset: float
```

- void set_baseline_offset(value: float)
- float get_baseline_offset()

Extra baseline offset (as a fraction of font height).

### palette_index {#palette_index}

```gdscript
palette_index: int
```

- void set_palette_index(value: int)
- int get_palette_index()

A palette index.

### palette_custom_colors {#palette_custom_colors}

```gdscript
palette_custom_colors: PackedColorArray
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
