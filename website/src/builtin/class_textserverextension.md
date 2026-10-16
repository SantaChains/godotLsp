# TextServerExtension

**继承**：[TextServer](class_textserver.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TextServerExtension**

> External TextServer implementations should inherit from this class.

<details><summary>官方英文描述</summary>

External TextServer implementations should inherit from this class.

</details>

## 方法

### _cleanup {#_cleanup}

```gdscript
void _cleanup() virtual
```

This method is called before text server is unregistered.

### _create_font {#_create_font}

```gdscript
RID _create_font() virtual
```

Creates a new, empty font cache entry resource.

### _create_font_linked_variation {#_create_font_linked_variation}

```gdscript
RID _create_font_linked_variation(font_rid: RID) virtual
```

Optional, implement if font supports extra spacing or baseline offset.

Creates a new variation existing font which is reusing the same glyph cache and font data.

### _create_shaped_text {#_create_shaped_text}

```gdscript
RID _create_shaped_text(direction: TextServer.Direction, orientation: TextServer.Orientation) virtual
```

Creates a new buffer for complex text layout, with the given direction and orientation.

### _draw_hex_code_box {#_draw_hex_code_box}

```gdscript
void _draw_hex_code_box(canvas: RID, size: int, pos: Vector2, index: int, color: Color) virtual const
```

Draws box displaying character hexadecimal code.

### _font_clear_glyphs {#_font_clear_glyphs}

```gdscript
void _font_clear_glyphs(font_rid: RID, size: Vector2i) virtual
```

Removes all rendered glyph information from the cache entry.

### _font_clear_kerning_map {#_font_clear_kerning_map}

```gdscript
void _font_clear_kerning_map(font_rid: RID, size: int) virtual
```

Removes all kerning overrides.

### _font_clear_size_cache {#_font_clear_size_cache}

```gdscript
void _font_clear_size_cache(font_rid: RID) virtual
```

Removes all font sizes from the cache entry.

### _font_clear_system_fallback_cache {#_font_clear_system_fallback_cache}

```gdscript
void _font_clear_system_fallback_cache() virtual
```

Frees all automatically loaded system fonts.

### _font_clear_textures {#_font_clear_textures}

```gdscript
void _font_clear_textures(font_rid: RID, size: Vector2i) virtual
```

Removes all textures from font cache entry.

### _font_draw_glyph {#_font_draw_glyph}

```gdscript
void _font_draw_glyph(font_rid: RID, canvas: RID, size: int, pos: Vector2, index: int, color: Color, oversampling: float) virtual const
```

Draws single glyph into a canvas item at the position, using font_rid at the size size. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### _font_draw_glyph_outline {#_font_draw_glyph_outline}

```gdscript
void _font_draw_glyph_outline(font_rid: RID, canvas: RID, size: int, outline_size: int, pos: Vector2, index: int, color: Color, oversampling: float) virtual const
```

Draws single glyph outline of size outline_size into a canvas item at the position, using font_rid at the size size. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### _font_get_antialiasing {#_font_get_antialiasing}

```gdscript
TextServer.FontAntialiasing _font_get_antialiasing(font_rid: RID) virtual const
```

Returns font anti-aliasing mode.

### _font_get_ascent {#_font_get_ascent}

```gdscript
float _font_get_ascent(font_rid: RID, size: int) virtual const
```

Returns the font ascent (number of pixels above the baseline).

### _font_get_baseline_offset {#_font_get_baseline_offset}

```gdscript
float _font_get_baseline_offset(font_rid: RID) virtual const
```

Returns extra baseline offset (as a fraction of font height).

### _font_get_char_from_glyph_index {#_font_get_char_from_glyph_index}

```gdscript
int _font_get_char_from_glyph_index(font_rid: RID, size: int, glyph_index: int) virtual const
```

Returns character code associated with glyph_index, or 0 if glyph_index is invalid.

### _font_get_descent {#_font_get_descent}

```gdscript
float _font_get_descent(font_rid: RID, size: int) virtual const
```

Returns the font descent (number of pixels below the baseline).

### _font_get_disable_embedded_bitmaps {#_font_get_disable_embedded_bitmaps}

```gdscript
bool _font_get_disable_embedded_bitmaps(font_rid: RID) virtual const
```

Returns whether the font's embedded bitmap loading is disabled.

### _font_get_embolden {#_font_get_embolden}

```gdscript
float _font_get_embolden(font_rid: RID) virtual const
```

Returns font embolden strength.

### _font_get_face_count {#_font_get_face_count}

```gdscript
int _font_get_face_count(font_rid: RID) virtual const
```

Returns number of faces in the TrueType / OpenType collection.

### _font_get_face_index {#_font_get_face_index}

```gdscript
int _font_get_face_index(font_rid: RID) virtual const
```

Returns an active face index in the TrueType / OpenType collection.

### _font_get_fixed_size {#_font_get_fixed_size}

```gdscript
int _font_get_fixed_size(font_rid: RID) virtual const
```

Returns bitmap font fixed size.

### _font_get_fixed_size_scale_mode {#_font_get_fixed_size_scale_mode}

```gdscript
TextServer.FixedSizeScaleMode _font_get_fixed_size_scale_mode(font_rid: RID) virtual const
```

Returns bitmap font scaling mode.

### _font_get_generate_mipmaps {#_font_get_generate_mipmaps}

```gdscript
bool _font_get_generate_mipmaps(font_rid: RID) virtual const
```

Returns true if font texture mipmap generation is enabled.

### _font_get_global_oversampling {#_font_get_global_oversampling}

```gdscript
float _font_get_global_oversampling() virtual const
```

Returns the font oversampling factor, shared by all fonts in the TextServer.

### _font_get_glyph_advance {#_font_get_glyph_advance}

```gdscript
Vector2 _font_get_glyph_advance(font_rid: RID, size: int, glyph: int) virtual const
```

Returns glyph advance (offset of the next glyph).

### _font_get_glyph_contours {#_font_get_glyph_contours}

```gdscript
Dictionary _font_get_glyph_contours(font_rid: RID, size: int, index: int) virtual const
```

Returns outline contours of the glyph.

### _font_get_glyph_index {#_font_get_glyph_index}

```gdscript
int _font_get_glyph_index(font_rid: RID, size: int, char: int, variation_selector: int) virtual const
```

Returns the glyph index of a char, optionally modified by the variation_selector.

### _font_get_glyph_list {#_font_get_glyph_list}

```gdscript
PackedInt32Array _font_get_glyph_list(font_rid: RID, size: Vector2i) virtual const
```

Returns list of rendered glyphs in the cache entry.

### _font_get_glyph_offset {#_font_get_glyph_offset}

```gdscript
Vector2 _font_get_glyph_offset(font_rid: RID, size: Vector2i, glyph: int) virtual const
```

Returns glyph offset from the baseline.

### _font_get_glyph_size {#_font_get_glyph_size}

```gdscript
Vector2 _font_get_glyph_size(font_rid: RID, size: Vector2i, glyph: int) virtual const
```

Returns size of the glyph.

### _font_get_glyph_texture_idx {#_font_get_glyph_texture_idx}

```gdscript
int _font_get_glyph_texture_idx(font_rid: RID, size: Vector2i, glyph: int) virtual const
```

Returns index of the cache texture containing the glyph.

### _font_get_glyph_texture_rid {#_font_get_glyph_texture_rid}

```gdscript
RID _font_get_glyph_texture_rid(font_rid: RID, size: Vector2i, glyph: int) virtual const
```

Returns resource ID of the cache texture containing the glyph.

### _font_get_glyph_texture_size {#_font_get_glyph_texture_size}

```gdscript
Vector2 _font_get_glyph_texture_size(font_rid: RID, size: Vector2i, glyph: int) virtual const
```

Returns size of the cache texture containing the glyph.

### _font_get_glyph_uv_rect {#_font_get_glyph_uv_rect}

```gdscript
Rect2 _font_get_glyph_uv_rect(font_rid: RID, size: Vector2i, glyph: int) virtual const
```

Returns rectangle in the cache texture containing the glyph.

### _font_get_hinting {#_font_get_hinting}

```gdscript
TextServer.Hinting _font_get_hinting(font_rid: RID) virtual const
```

Returns the font hinting mode. Used by dynamic fonts only.

### _font_get_keep_rounding_remainders {#_font_get_keep_rounding_remainders}

```gdscript
bool _font_get_keep_rounding_remainders(font_rid: RID) virtual const
```

Returns glyph position rounding behavior. If set to true, when aligning glyphs to the pixel boundaries rounding remainders are accumulated to ensure more uniform glyph distribution. This setting has no effect if subpixel positioning is enabled.

### _font_get_kerning {#_font_get_kerning}

```gdscript
Vector2 _font_get_kerning(font_rid: RID, size: int, glyph_pair: Vector2i) virtual const
```

Returns kerning for the pair of glyphs.

### _font_get_kerning_list {#_font_get_kerning_list}

```gdscript
typedarray::Vector2i _font_get_kerning_list(font_rid: RID, size: int) virtual const
```

Returns list of the kerning overrides.

### _font_get_language_support_override {#_font_get_language_support_override}

```gdscript
bool _font_get_language_support_override(font_rid: RID, language: String) virtual
```

Returns true if support override is enabled for the language.

### _font_get_language_support_overrides {#_font_get_language_support_overrides}

```gdscript
PackedStringArray _font_get_language_support_overrides(font_rid: RID) virtual
```

Returns list of language support overrides.

### _font_get_msdf_pixel_range {#_font_get_msdf_pixel_range}

```gdscript
int _font_get_msdf_pixel_range(font_rid: RID) virtual const
```

Returns the width of the range around the shape between the minimum and maximum representable signed distance.

### _font_get_msdf_size {#_font_get_msdf_size}

```gdscript
int _font_get_msdf_size(font_rid: RID) virtual const
```

Returns source font size used to generate MSDF textures.

### _font_get_name {#_font_get_name}

```gdscript
String _font_get_name(font_rid: RID) virtual const
```

Returns font family name.

### _font_get_opentype_feature_overrides {#_font_get_opentype_feature_overrides}

```gdscript
Dictionary _font_get_opentype_feature_overrides(font_rid: RID) virtual const
```

Returns font OpenType feature set override.

### _font_get_ot_name_strings {#_font_get_ot_name_strings}

```gdscript
Dictionary _font_get_ot_name_strings(font_rid: RID) virtual const
```

Returns Dictionary with OpenType font name strings (localized font names, version, description, license information, sample text, etc.).

### _font_get_oversampling {#_font_get_oversampling}

```gdscript
float _font_get_oversampling(font_rid: RID) virtual const
```

Returns oversampling factor override. If set to a positive value, overrides the oversampling factor of the viewport this font is used in. See Viewport.oversampling. This value doesn't override the oversampling parameter of draw_* methods. Used by dynamic fonts only.

### _font_get_palette_colors {#_font_get_palette_colors}

```gdscript
PackedColorArray _font_get_palette_colors(font_rid: RID, index: int) virtual const
```

Returns the array in the predefined color palette at index. Palette contains all colors used to render font glyphs. Each palette has the same number of colors. Colors can be overridden using _font_set_palette_custom_colors().

### _font_get_palette_count {#_font_get_palette_count}

```gdscript
int _font_get_palette_count(font_rid: RID) virtual const
```

Returns the number of predefined color palettes. Palette contains all colors used to render font glyphs. Each palette has the same number of colors.

### _font_get_palette_custom_colors {#_font_get_palette_custom_colors}

```gdscript
PackedColorArray _font_get_palette_custom_colors(font_rid: RID) virtual const
```

Returns array of custom colors to override predefined palette.

### _font_get_palette_name {#_font_get_palette_name}

```gdscript
String _font_get_palette_name(font_rid: RID, index: int) virtual const
```

Returns the name of the predefined color palette at index. Palette contains all colors used to render font glyphs. Each palette has the same number of colors.

### _font_get_scale {#_font_get_scale}

```gdscript
float _font_get_scale(font_rid: RID, size: int) virtual const
```

Returns scaling factor of the color bitmap font.

### _font_get_script_support_override {#_font_get_script_support_override}

```gdscript
bool _font_get_script_support_override(font_rid: RID, script: String) virtual
```

Returns true if support override is enabled for the script.

### _font_get_script_support_overrides {#_font_get_script_support_overrides}

```gdscript
PackedStringArray _font_get_script_support_overrides(font_rid: RID) virtual
```

Returns list of script support overrides.

### _font_get_size_cache_info {#_font_get_size_cache_info}

```gdscript
typedarray::Dictionary _font_get_size_cache_info(font_rid: RID) virtual const
```

Returns font cache information, each entry contains the following fields: Vector2i size_px - font size in pixels, float viewport_oversampling - viewport oversampling factor, int glyphs - number of rendered glyphs, int textures - number of used textures, int textures_size - size of texture data in bytes.

### _font_get_size_cache_list {#_font_get_size_cache_list}

```gdscript
typedarray::Vector2i _font_get_size_cache_list(font_rid: RID) virtual const
```

Returns list of the font sizes in the cache. Each size is Vector2i with font size and outline size.

### _font_get_spacing {#_font_get_spacing}

```gdscript
int _font_get_spacing(font_rid: RID, spacing: TextServer.SpacingType) virtual const
```

Returns the spacing for spacing in pixels (not relative to the font size).

### _font_get_stretch {#_font_get_stretch}

```gdscript
int _font_get_stretch(font_rid: RID) virtual const
```

Returns font stretch amount, compared to a normal width. A percentage value between 50% and 200%.

### _font_get_style {#_font_get_style}

```gdscript
bitfield::TextServer.FontStyle _font_get_style(font_rid: RID) virtual const
```

Returns font style flags.

### _font_get_style_name {#_font_get_style_name}

```gdscript
String _font_get_style_name(font_rid: RID) virtual const
```

Returns font style name.

### _font_get_subpixel_positioning {#_font_get_subpixel_positioning}

```gdscript
TextServer.SubpixelPositioning _font_get_subpixel_positioning(font_rid: RID) virtual const
```

Returns font subpixel glyph positioning mode.

### _font_get_supported_chars {#_font_get_supported_chars}

```gdscript
String _font_get_supported_chars(font_rid: RID) virtual const
```

Returns a string containing all the characters available in the font.

### _font_get_supported_glyphs {#_font_get_supported_glyphs}

```gdscript
PackedInt32Array _font_get_supported_glyphs(font_rid: RID) virtual const
```

Returns an array containing all glyph indices in the font.

### _font_get_texture_count {#_font_get_texture_count}

```gdscript
int _font_get_texture_count(font_rid: RID, size: Vector2i) virtual const
```

Returns number of textures used by font cache entry.

### _font_get_texture_image {#_font_get_texture_image}

```gdscript
Image _font_get_texture_image(font_rid: RID, size: Vector2i, texture_index: int) virtual const
```

Returns font cache texture image data.

### _font_get_texture_offsets {#_font_get_texture_offsets}

```gdscript
PackedInt32Array _font_get_texture_offsets(font_rid: RID, size: Vector2i, texture_index: int) virtual const
```

Returns array containing glyph packing data.

### _font_get_transform {#_font_get_transform}

```gdscript
Transform2D _font_get_transform(font_rid: RID) virtual const
```

Returns 2D transform applied to the font outlines.

### _font_get_underline_position {#_font_get_underline_position}

```gdscript
float _font_get_underline_position(font_rid: RID, size: int) virtual const
```

Returns pixel offset of the underline below the baseline.

### _font_get_underline_thickness {#_font_get_underline_thickness}

```gdscript
float _font_get_underline_thickness(font_rid: RID, size: int) virtual const
```

Returns thickness of the underline in pixels.

### _font_get_used_palette {#_font_get_used_palette}

```gdscript
int _font_get_used_palette(font_rid: RID) virtual const
```

Returns used palette index.

### _font_get_variation_coordinates {#_font_get_variation_coordinates}

```gdscript
Dictionary _font_get_variation_coordinates(font_rid: RID) virtual const
```

Returns variation coordinates for the specified font cache entry.

### _font_get_weight {#_font_get_weight}

```gdscript
int _font_get_weight(font_rid: RID) virtual const
```

Returns weight (boldness) of the font. A value in the 100...999 range, normal font weight is 400, bold font weight is 700.

### _font_has_char {#_font_has_char}

```gdscript
bool _font_has_char(font_rid: RID, char: int) virtual const
```

Returns true if a Unicode char is available in the font.

### _font_is_allow_system_fallback {#_font_is_allow_system_fallback}

```gdscript
bool _font_is_allow_system_fallback(font_rid: RID) virtual const
```

Returns true if system fonts can be automatically used as fallbacks.

### _font_is_force_autohinter {#_font_is_force_autohinter}

```gdscript
bool _font_is_force_autohinter(font_rid: RID) virtual const
```

Returns true if auto-hinting is supported and preferred over font built-in hinting.

### _font_is_language_supported {#_font_is_language_supported}

```gdscript
bool _font_is_language_supported(font_rid: RID, language: String) virtual const
```

Returns true if the font supports the given language (as a ISO 639 [https://en.wikipedia.org/wiki/ISO_639-1] code).

### _font_is_modulate_color_glyphs {#_font_is_modulate_color_glyphs}

```gdscript
bool _font_is_modulate_color_glyphs(font_rid: RID) virtual const
```

Returns true if color modulation is applied when drawing the font's colored glyphs.

### _font_is_multichannel_signed_distance_field {#_font_is_multichannel_signed_distance_field}

```gdscript
bool _font_is_multichannel_signed_distance_field(font_rid: RID) virtual const
```

Returns true if glyphs of all sizes are rendered using single multichannel signed distance field generated from the dynamic font vector data.

### _font_is_script_supported {#_font_is_script_supported}

```gdscript
bool _font_is_script_supported(font_rid: RID, script: String) virtual const
```

Returns true if the font supports the given script (as a ISO 15924 [https://en.wikipedia.org/wiki/ISO_15924] code).

### _font_remove_glyph {#_font_remove_glyph}

```gdscript
void _font_remove_glyph(font_rid: RID, size: Vector2i, glyph: int) virtual
```

Removes specified rendered glyph information from the cache entry.

### _font_remove_kerning {#_font_remove_kerning}

```gdscript
void _font_remove_kerning(font_rid: RID, size: int, glyph_pair: Vector2i) virtual
```

Removes kerning override for the pair of glyphs.

### _font_remove_language_support_override {#_font_remove_language_support_override}

```gdscript
void _font_remove_language_support_override(font_rid: RID, language: String) virtual
```

Remove language support override.

### _font_remove_script_support_override {#_font_remove_script_support_override}

```gdscript
void _font_remove_script_support_override(font_rid: RID, script: String) virtual
```

Removes script support override.

### _font_remove_size_cache {#_font_remove_size_cache}

```gdscript
void _font_remove_size_cache(font_rid: RID, size: Vector2i) virtual
```

Removes specified font size from the cache entry.

### _font_remove_texture {#_font_remove_texture}

```gdscript
void _font_remove_texture(font_rid: RID, size: Vector2i, texture_index: int) virtual
```

Removes specified texture from the cache entry.

### _font_render_glyph {#_font_render_glyph}

```gdscript
void _font_render_glyph(font_rid: RID, size: Vector2i, index: int) virtual
```

Renders specified glyph to the font cache texture.

### _font_render_range {#_font_render_range}

```gdscript
void _font_render_range(font_rid: RID, size: Vector2i, start: int, end: int) virtual
```

Renders the range of characters to the font cache texture.

### _font_set_allow_system_fallback {#_font_set_allow_system_fallback}

```gdscript
void _font_set_allow_system_fallback(font_rid: RID, allow_system_fallback: bool) virtual
```

If set to true, system fonts can be automatically used as fallbacks.

### _font_set_antialiasing {#_font_set_antialiasing}

```gdscript
void _font_set_antialiasing(font_rid: RID, antialiasing: TextServer.FontAntialiasing) virtual
```

Sets font anti-aliasing mode.

### _font_set_ascent {#_font_set_ascent}

```gdscript
void _font_set_ascent(font_rid: RID, size: int, ascent: float) virtual
```

Sets the font ascent (number of pixels above the baseline).

### _font_set_baseline_offset {#_font_set_baseline_offset}

```gdscript
void _font_set_baseline_offset(font_rid: RID, baseline_offset: float) virtual
```

Sets extra baseline offset (as a fraction of font height).

### _font_set_data {#_font_set_data}

```gdscript
void _font_set_data(font_rid: RID, data: PackedByteArray) virtual
```

Sets font source data, e.g contents of the dynamic font source file.

### _font_set_data_ptr {#_font_set_data_ptr}

```gdscript
void _font_set_data_ptr(font_rid: RID, data_ptr: const uint8_t*, data_size: int) virtual
```

Sets pointer to the font source data, e.g contents of the dynamic font source file.

### _font_set_descent {#_font_set_descent}

```gdscript
void _font_set_descent(font_rid: RID, size: int, descent: float) virtual
```

Sets the font descent (number of pixels below the baseline).

### _font_set_disable_embedded_bitmaps {#_font_set_disable_embedded_bitmaps}

```gdscript
void _font_set_disable_embedded_bitmaps(font_rid: RID, disable_embedded_bitmaps: bool) virtual
```

If set to true, embedded font bitmap loading is disabled.

### _font_set_embolden {#_font_set_embolden}

```gdscript
void _font_set_embolden(font_rid: RID, strength: float) virtual
```

Sets font embolden strength. If strength is not equal to zero, emboldens the font outlines. Negative values reduce the outline thickness.

### _font_set_face_index {#_font_set_face_index}

```gdscript
void _font_set_face_index(font_rid: RID, face_index: int) virtual
```

Sets an active face index in the TrueType / OpenType collection.

### _font_set_fixed_size {#_font_set_fixed_size}

```gdscript
void _font_set_fixed_size(font_rid: RID, fixed_size: int) virtual
```

Sets bitmap font fixed size. If set to value greater than zero, same cache entry will be used for all font sizes.

### _font_set_fixed_size_scale_mode {#_font_set_fixed_size_scale_mode}

```gdscript
void _font_set_fixed_size_scale_mode(font_rid: RID, fixed_size_scale_mode: TextServer.FixedSizeScaleMode) virtual
```

Sets bitmap font scaling mode. This property is used only if fixed_size is greater than zero.

### _font_set_force_autohinter {#_font_set_force_autohinter}

```gdscript
void _font_set_force_autohinter(font_rid: RID, force_autohinter: bool) virtual
```

If set to true auto-hinting is preferred over font built-in hinting.

### _font_set_generate_mipmaps {#_font_set_generate_mipmaps}

```gdscript
void _font_set_generate_mipmaps(font_rid: RID, generate_mipmaps: bool) virtual
```

If set to true font texture mipmap generation is enabled.

### _font_set_global_oversampling {#_font_set_global_oversampling}

```gdscript
void _font_set_global_oversampling(oversampling: float) virtual
```

Sets oversampling factor, shared by all font in the TextServer.

### _font_set_glyph_advance {#_font_set_glyph_advance}

```gdscript
void _font_set_glyph_advance(font_rid: RID, size: int, glyph: int, advance: Vector2) virtual
```

Sets glyph advance (offset of the next glyph).

### _font_set_glyph_offset {#_font_set_glyph_offset}

```gdscript
void _font_set_glyph_offset(font_rid: RID, size: Vector2i, glyph: int, offset: Vector2) virtual
```

Sets glyph offset from the baseline.

### _font_set_glyph_size {#_font_set_glyph_size}

```gdscript
void _font_set_glyph_size(font_rid: RID, size: Vector2i, glyph: int, gl_size: Vector2) virtual
```

Sets size of the glyph.

### _font_set_glyph_texture_idx {#_font_set_glyph_texture_idx}

```gdscript
void _font_set_glyph_texture_idx(font_rid: RID, size: Vector2i, glyph: int, texture_idx: int) virtual
```

Sets index of the cache texture containing the glyph.

### _font_set_glyph_uv_rect {#_font_set_glyph_uv_rect}

```gdscript
void _font_set_glyph_uv_rect(font_rid: RID, size: Vector2i, glyph: int, uv_rect: Rect2) virtual
```

Sets rectangle in the cache texture containing the glyph.

### _font_set_hinting {#_font_set_hinting}

```gdscript
void _font_set_hinting(font_rid: RID, hinting: TextServer.Hinting) virtual
```

Sets font hinting mode. Used by dynamic fonts only.

### _font_set_keep_rounding_remainders {#_font_set_keep_rounding_remainders}

```gdscript
void _font_set_keep_rounding_remainders(font_rid: RID, keep_rounding_remainders: bool) virtual
```

Sets glyph position rounding behavior. If set to true, when aligning glyphs to the pixel boundaries rounding remainders are accumulated to ensure more uniform glyph distribution. This setting has no effect if subpixel positioning is enabled.

### _font_set_kerning {#_font_set_kerning}

```gdscript
void _font_set_kerning(font_rid: RID, size: int, glyph_pair: Vector2i, kerning: Vector2) virtual
```

Sets kerning for the pair of glyphs.

### _font_set_language_support_override {#_font_set_language_support_override}

```gdscript
void _font_set_language_support_override(font_rid: RID, language: String, supported: bool) virtual
```

Adds override for _font_is_language_supported().

### _font_set_modulate_color_glyphs {#_font_set_modulate_color_glyphs}

```gdscript
void _font_set_modulate_color_glyphs(font_rid: RID, modulate: bool) virtual
```

If set to true, color modulation is applied when drawing colored glyphs, otherwise it's applied to the monochrome glyphs only.

### _font_set_msdf_pixel_range {#_font_set_msdf_pixel_range}

```gdscript
void _font_set_msdf_pixel_range(font_rid: RID, msdf_pixel_range: int) virtual
```

Sets the width of the range around the shape between the minimum and maximum representable signed distance.

### _font_set_msdf_size {#_font_set_msdf_size}

```gdscript
void _font_set_msdf_size(font_rid: RID, msdf_size: int) virtual
```

Sets source font size used to generate MSDF textures.

### _font_set_multichannel_signed_distance_field {#_font_set_multichannel_signed_distance_field}

```gdscript
void _font_set_multichannel_signed_distance_field(font_rid: RID, msdf: bool) virtual
```

If set to true, glyphs of all sizes are rendered using single multichannel signed distance field generated from the dynamic font vector data. MSDF rendering allows displaying the font at any scaling factor without blurriness, and without incurring a CPU cost when the font size changes (since the font no longer needs to be rasterized on the CPU). As a downside, font hinting is not available with MSDF. The lack of font hinting may result in less crisp and less readable fonts at small sizes.

### _font_set_name {#_font_set_name}

```gdscript
void _font_set_name(font_rid: RID, name: String) virtual
```

Sets the font family name.

### _font_set_opentype_feature_overrides {#_font_set_opentype_feature_overrides}

```gdscript
void _font_set_opentype_feature_overrides(font_rid: RID, overrides: Dictionary) virtual
```

Sets font OpenType feature set override.

### _font_set_oversampling {#_font_set_oversampling}

```gdscript
void _font_set_oversampling(font_rid: RID, oversampling: float) virtual
```

If set to a positive value, overrides the oversampling factor of the viewport this font is used in. See Viewport.oversampling. This value doesn't override the oversampling parameter of draw_* methods. Used by dynamic fonts only.

### _font_set_palette_custom_colors {#_font_set_palette_custom_colors}

```gdscript
void _font_set_palette_custom_colors(font_rid: RID, colors: PackedColorArray) virtual
```

Sets array of custom colors to override predefined palette. Set to empty array to reset overrides. Use Color(0, 0, 0, 0), to keep predefined palette color at specific position.

### _font_set_scale {#_font_set_scale}

```gdscript
void _font_set_scale(font_rid: RID, size: int, scale: float) virtual
```

Sets scaling factor of the color bitmap font.

### _font_set_script_support_override {#_font_set_script_support_override}

```gdscript
void _font_set_script_support_override(font_rid: RID, script: String, supported: bool) virtual
```

Adds override for _font_is_script_supported().

### _font_set_spacing {#_font_set_spacing}

```gdscript
void _font_set_spacing(font_rid: RID, spacing: TextServer.SpacingType, value: int) virtual
```

Sets the spacing for spacing to value in pixels (not relative to the font size).

### _font_set_stretch {#_font_set_stretch}

```gdscript
void _font_set_stretch(font_rid: RID, stretch: int) virtual
```

Sets font stretch amount, compared to a normal width. A percentage value between 50% and 200%.

### _font_set_style {#_font_set_style}

```gdscript
void _font_set_style(font_rid: RID, style: bitfield::TextServer.FontStyle) virtual
```

Sets the font style flags.

### _font_set_style_name {#_font_set_style_name}

```gdscript
void _font_set_style_name(font_rid: RID, name_style: String) virtual
```

Sets the font style name.

### _font_set_subpixel_positioning {#_font_set_subpixel_positioning}

```gdscript
void _font_set_subpixel_positioning(font_rid: RID, subpixel_positioning: TextServer.SubpixelPositioning) virtual
```

Sets font subpixel glyph positioning mode.

### _font_set_texture_image {#_font_set_texture_image}

```gdscript
void _font_set_texture_image(font_rid: RID, size: Vector2i, texture_index: int, image: Image) virtual
```

Sets font cache texture image data.

### _font_set_texture_offsets {#_font_set_texture_offsets}

```gdscript
void _font_set_texture_offsets(font_rid: RID, size: Vector2i, texture_index: int, offset: PackedInt32Array) virtual
```

Sets array containing glyph packing data.

### _font_set_transform {#_font_set_transform}

```gdscript
void _font_set_transform(font_rid: RID, transform: Transform2D) virtual
```

Sets 2D transform, applied to the font outlines, can be used for slanting, flipping, and rotating glyphs.

### _font_set_underline_position {#_font_set_underline_position}

```gdscript
void _font_set_underline_position(font_rid: RID, size: int, underline_position: float) virtual
```

Sets pixel offset of the underline below the baseline.

### _font_set_underline_thickness {#_font_set_underline_thickness}

```gdscript
void _font_set_underline_thickness(font_rid: RID, size: int, underline_thickness: float) virtual
```

Sets thickness of the underline in pixels.

### _font_set_used_palette {#_font_set_used_palette}

```gdscript
void _font_set_used_palette(font_rid: RID, index: int) virtual
```

Sets used palette index.

### _font_set_variation_coordinates {#_font_set_variation_coordinates}

```gdscript
void _font_set_variation_coordinates(font_rid: RID, variation_coordinates: Dictionary) virtual
```

Sets variation coordinates for the specified font cache entry.

### _font_set_weight {#_font_set_weight}

```gdscript
void _font_set_weight(font_rid: RID, weight: int) virtual
```

Sets weight (boldness) of the font. A value in the 100...999 range, normal font weight is 400, bold font weight is 700.

### _font_supported_feature_list {#_font_supported_feature_list}

```gdscript
Dictionary _font_supported_feature_list(font_rid: RID) virtual const
```

Returns the dictionary of the supported OpenType features.

### _font_supported_variation_list {#_font_supported_variation_list}

```gdscript
Dictionary _font_supported_variation_list(font_rid: RID) virtual const
```

Returns the dictionary of the supported OpenType variation coordinates.

### _format_number {#_format_number}

```gdscript
String _format_number(number: String, language: String) virtual const
```

Deprecated: Use TranslationServer.format_number() instead.

Converts a number from Western Arabic (0..9) to the numeral system used in the given language.

If language is an empty string, the active locale will be used.

### _free_rid {#_free_rid}

```gdscript
void _free_rid(rid: RID) virtual
```

Frees an object created by this TextServer.

### _get_features {#_get_features}

```gdscript
int _get_features() virtual const
```

Returns text server features, see Feature.

### _get_hex_code_box_size {#_get_hex_code_box_size}

```gdscript
Vector2 _get_hex_code_box_size(size: int, index: int) virtual const
```

Returns size of the replacement character (box with character hexadecimal code that is drawn in place of invalid characters).

### _get_name {#_get_name}

```gdscript
String _get_name() virtual const
```

Returns the name of the server interface.

### _get_support_data {#_get_support_data}

```gdscript
PackedByteArray _get_support_data() virtual const
```

Returns default TextServer database (e.g. ICU break iterators and dictionaries).

### _get_support_data_filename {#_get_support_data_filename}

```gdscript
String _get_support_data_filename() virtual const
```

Returns default TextServer database (e.g. ICU break iterators and dictionaries) filename.

### _get_support_data_info {#_get_support_data_info}

```gdscript
String _get_support_data_info() virtual const
```

Returns TextServer database (e.g. ICU break iterators and dictionaries) description.

### _has {#_has}

```gdscript
bool _has(rid: RID) virtual
```

Returns true if rid is valid resource owned by this text server.

### _has_feature {#_has_feature}

```gdscript
bool _has_feature(feature: TextServer.Feature) virtual const
```

Returns true if the server supports a feature.

### _is_confusable {#_is_confusable}

```gdscript
int _is_confusable(string: String, dict: PackedStringArray) virtual const
```

Returns index of the first string in dict which is visually confusable with the string, or -1 if none is found.

### _is_locale_right_to_left {#_is_locale_right_to_left}

```gdscript
bool _is_locale_right_to_left(locale: String) virtual const
```

Returns true if locale is right-to-left.

### _is_locale_using_support_data {#_is_locale_using_support_data}

```gdscript
bool _is_locale_using_support_data(locale: String) virtual const
```

Returns true if the locale requires text server support data for line/word breaking.

### _is_valid_identifier {#_is_valid_identifier}

```gdscript
bool _is_valid_identifier(string: String) virtual const
```

Returns true if string is a valid identifier.

### _is_valid_letter {#_is_valid_letter}

```gdscript
bool _is_valid_letter(unicode: int) virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### _load_support_data {#_load_support_data}

```gdscript
bool _load_support_data(filename: String) virtual
```

Loads optional TextServer database (e.g. ICU break iterators and dictionaries).

### _name_to_tag {#_name_to_tag}

```gdscript
int _name_to_tag(name: String) virtual const
```

Converts the given readable name of a feature, variation, script, or language to an OpenType tag.

### _parse_number {#_parse_number}

```gdscript
String _parse_number(number: String, language: String) virtual const
```

Deprecated: Use TranslationServer.parse_number() instead.

Converts number from the numeral system used in the given language to Western Arabic (0..9).

If language is an empty string, the active locale will be used.

### _parse_structured_text {#_parse_structured_text}

```gdscript
typedarray::Vector3i _parse_structured_text(parser_type: TextServer.StructuredTextParser, args: Array, text: String) virtual const
```

Default implementation of the BiDi algorithm override function.

### _percent_sign {#_percent_sign}

```gdscript
String _percent_sign(language: String) virtual const
```

Deprecated: Use TranslationServer.get_percent_sign() instead.

Returns percent sign used in the given language.

### _reference_oversampling_level {#_reference_oversampling_level}

```gdscript
void _reference_oversampling_level(oversampling: float) virtual
```

Increases the reference count of the specified oversampling level. This method is called by Viewport, and should not be used directly.

### _save_support_data {#_save_support_data}

```gdscript
bool _save_support_data(filename: String) virtual const
```

Saves optional TextServer database (e.g. ICU break iterators and dictionaries) to the file.

### _shaped_get_run_count {#_shaped_get_run_count}

```gdscript
int _shaped_get_run_count(shaped: RID) virtual const
```

Returns the number of uniform text runs in the buffer.

### _shaped_get_run_direction {#_shaped_get_run_direction}

```gdscript
TextServer.Direction _shaped_get_run_direction(shaped: RID, index: int) virtual const
```

Returns the direction of the index text run (in visual order).

### _shaped_get_run_font_rid {#_shaped_get_run_font_rid}

```gdscript
RID _shaped_get_run_font_rid(shaped: RID, index: int) virtual const
```

Returns the font RID of the index text run (in visual order).

### _shaped_get_run_font_size {#_shaped_get_run_font_size}

```gdscript
int _shaped_get_run_font_size(shaped: RID, index: int) virtual const
```

Returns the font size of the index text run (in visual order).

### _shaped_get_run_glyph_range {#_shaped_get_run_glyph_range}

```gdscript
Vector2i _shaped_get_run_glyph_range(shaped: RID, index: int) virtual const
```

Returns the glyph range of the index text run (in visual order).

### _shaped_get_run_language {#_shaped_get_run_language}

```gdscript
String _shaped_get_run_language(shaped: RID, index: int) virtual const
```

Returns the language of the index text run (in visual order).

### _shaped_get_run_object {#_shaped_get_run_object}

```gdscript
Variant _shaped_get_run_object(shaped: RID, index: int) virtual const
```

Returns the embedded object of the index text run (in visual order).

### _shaped_get_run_range {#_shaped_get_run_range}

```gdscript
Vector2i _shaped_get_run_range(shaped: RID, index: int) virtual const
```

Returns the source text range of the index text run (in visual order).

### _shaped_get_run_text {#_shaped_get_run_text}

```gdscript
String _shaped_get_run_text(shaped: RID, index: int) virtual const
```

Returns the source text of the index text run (in visual order).

### _shaped_get_span_count {#_shaped_get_span_count}

```gdscript
int _shaped_get_span_count(shaped: RID) virtual const
```

Returns number of text spans added using _shaped_text_add_string() or _shaped_text_add_object().

### _shaped_get_span_embedded_object {#_shaped_get_span_embedded_object}

```gdscript
Variant _shaped_get_span_embedded_object(shaped: RID, index: int) virtual const
```

Returns text embedded object key.

### _shaped_get_span_meta {#_shaped_get_span_meta}

```gdscript
Variant _shaped_get_span_meta(shaped: RID, index: int) virtual const
```

Returns text span metadata.

### _shaped_get_span_object {#_shaped_get_span_object}

```gdscript
Variant _shaped_get_span_object(shaped: RID, index: int) virtual const
```

Returns the text span embedded object key.

### _shaped_get_span_text {#_shaped_get_span_text}

```gdscript
String _shaped_get_span_text(shaped: RID, index: int) virtual const
```

Returns the text span source text.

### _shaped_get_text {#_shaped_get_text}

```gdscript
String _shaped_get_text(shaped: RID) virtual const
```

Returns the text buffer source text, including object replacement characters.

### _shaped_set_span_update_font {#_shaped_set_span_update_font}

```gdscript
void _shaped_set_span_update_font(shaped: RID, index: int, fonts: typedarray::RID, size: int, opentype_features: Dictionary) virtual
```

Changes text span font, font size, and OpenType features, without changing the text.

### _shaped_text_add_object {#_shaped_text_add_object}

```gdscript
bool _shaped_text_add_object(shaped: RID, key: Variant, size: Vector2, inline_align: InlineAlignment, length: int, baseline: float) virtual
```

Adds inline object to the text buffer, key must be unique. In the text, object is represented as length object replacement characters.

### _shaped_text_add_string {#_shaped_text_add_string}

```gdscript
bool _shaped_text_add_string(shaped: RID, text: String, fonts: typedarray::RID, size: int, opentype_features: Dictionary, language: String, meta: Variant) virtual
```

Adds text span and font to draw it to the text buffer.

### _shaped_text_clear {#_shaped_text_clear}

```gdscript
void _shaped_text_clear(shaped: RID) virtual
```

Clears text buffer (removes text and inline objects).

### _shaped_text_closest_character_pos {#_shaped_text_closest_character_pos}

```gdscript
int _shaped_text_closest_character_pos(shaped: RID, pos: int) virtual const
```

Returns composite character position closest to the pos.

### _shaped_text_draw {#_shaped_text_draw}

```gdscript
void _shaped_text_draw(shaped: RID, canvas: RID, pos: Vector2, clip_l: float, clip_r: float, color: Color, oversampling: float) virtual const
```

Draw shaped text into a canvas item at a given position, with color. pos specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### _shaped_text_draw_outline {#_shaped_text_draw_outline}

```gdscript
void _shaped_text_draw_outline(shaped: RID, canvas: RID, pos: Vector2, clip_l: float, clip_r: float, outline_size: int, color: Color, oversampling: float) virtual const
```

Draw the outline of the shaped text into a canvas item at a given position, with color. pos specifies the leftmost point of the baseline (for horizontal layout) or topmost point of the baseline (for vertical layout). If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### _shaped_text_duplicate {#_shaped_text_duplicate}

```gdscript
RID _shaped_text_duplicate(shaped: RID) virtual
```

Duplicates shaped text buffer.

### _shaped_text_fit_to_width {#_shaped_text_fit_to_width}

```gdscript
float _shaped_text_fit_to_width(shaped: RID, width: float, justification_flags: bitfield::TextServer.JustificationFlag) virtual
```

Adjusts text width to fit to specified width, returns new text width.

### _shaped_text_get_ascent {#_shaped_text_get_ascent}

```gdscript
float _shaped_text_get_ascent(shaped: RID) virtual const
```

Returns the text ascent (number of pixels above the baseline for horizontal layout or to the left of baseline for vertical).

### _shaped_text_get_carets {#_shaped_text_get_carets}

```gdscript
void _shaped_text_get_carets(shaped: RID, position: int, r_caret: CaretInfo*) virtual const
```

Returns shapes of the carets corresponding to the character offset position in the text. Returned caret shape is 1 pixel wide rectangle.

### _shaped_text_get_character_breaks {#_shaped_text_get_character_breaks}

```gdscript
PackedInt32Array _shaped_text_get_character_breaks(shaped: RID) virtual const
```

Returns array of the composite character boundaries.

### _shaped_text_get_custom_ellipsis {#_shaped_text_get_custom_ellipsis}

```gdscript
int _shaped_text_get_custom_ellipsis(shaped: RID) virtual const
```

Returns ellipsis character used for text clipping.

### _shaped_text_get_custom_punctuation {#_shaped_text_get_custom_punctuation}

```gdscript
String _shaped_text_get_custom_punctuation(shaped: RID) virtual const
```

Returns custom punctuation character list, used for word breaking. If set to empty string, server defaults are used.

### _shaped_text_get_descent {#_shaped_text_get_descent}

```gdscript
float _shaped_text_get_descent(shaped: RID) virtual const
```

Returns the text descent (number of pixels below the baseline for horizontal layout or to the right of baseline for vertical).

### _shaped_text_get_direction {#_shaped_text_get_direction}

```gdscript
TextServer.Direction _shaped_text_get_direction(shaped: RID) virtual const
```

Returns direction of the text.

### _shaped_text_get_dominant_direction_in_range {#_shaped_text_get_dominant_direction_in_range}

```gdscript
int _shaped_text_get_dominant_direction_in_range(shaped: RID, start: int, end: int) virtual const
```

Returns dominant direction of in the range of text.

### _shaped_text_get_ellipsis_glyph_count {#_shaped_text_get_ellipsis_glyph_count}

```gdscript
int _shaped_text_get_ellipsis_glyph_count(shaped: RID) virtual const
```

Returns number of glyphs in the ellipsis.

### _shaped_text_get_ellipsis_glyphs {#_shaped_text_get_ellipsis_glyphs}

```gdscript
const Glyph* _shaped_text_get_ellipsis_glyphs(shaped: RID) virtual const
```

Returns array of the glyphs in the ellipsis.

### _shaped_text_get_ellipsis_pos {#_shaped_text_get_ellipsis_pos}

```gdscript
int _shaped_text_get_ellipsis_pos(shaped: RID) virtual const
```

Returns position of the ellipsis.

### _shaped_text_get_glyph_count {#_shaped_text_get_glyph_count}

```gdscript
int _shaped_text_get_glyph_count(shaped: RID) virtual const
```

Returns number of glyphs in the buffer.

### _shaped_text_get_glyphs {#_shaped_text_get_glyphs}

```gdscript
const Glyph* _shaped_text_get_glyphs(shaped: RID) virtual const
```

Returns an array of glyphs in the visual order.

### _shaped_text_get_grapheme_bounds {#_shaped_text_get_grapheme_bounds}

```gdscript
Vector2 _shaped_text_get_grapheme_bounds(shaped: RID, pos: int) virtual const
```

Returns composite character's bounds as offsets from the start of the line.

### _shaped_text_get_inferred_direction {#_shaped_text_get_inferred_direction}

```gdscript
TextServer.Direction _shaped_text_get_inferred_direction(shaped: RID) virtual const
```

Returns direction of the text, inferred by the BiDi algorithm.

### _shaped_text_get_line_breaks {#_shaped_text_get_line_breaks}

```gdscript
PackedInt32Array _shaped_text_get_line_breaks(shaped: RID, width: float, start: int, break_flags: bitfield::TextServer.LineBreakFlag) virtual const
```

Breaks text to the lines and returns character ranges for each line.

### _shaped_text_get_line_breaks_adv {#_shaped_text_get_line_breaks_adv}

```gdscript
PackedInt32Array _shaped_text_get_line_breaks_adv(shaped: RID, width: PackedFloat32Array, start: int, once: bool, break_flags: bitfield::TextServer.LineBreakFlag) virtual const
```

Breaks text to the lines and columns. Returns character ranges for each segment.

### _shaped_text_get_object_glyph {#_shaped_text_get_object_glyph}

```gdscript
int _shaped_text_get_object_glyph(shaped: RID, key: Variant) virtual const
```

Returns the glyph index of the inline object.

### _shaped_text_get_object_range {#_shaped_text_get_object_range}

```gdscript
Vector2i _shaped_text_get_object_range(shaped: RID, key: Variant) virtual const
```

Returns the character range of the inline object.

### _shaped_text_get_object_rect {#_shaped_text_get_object_rect}

```gdscript
Rect2 _shaped_text_get_object_rect(shaped: RID, key: Variant) virtual const
```

Returns bounding rectangle of the inline object.

### _shaped_text_get_objects {#_shaped_text_get_objects}

```gdscript
Array _shaped_text_get_objects(shaped: RID) virtual const
```

Returns array of inline objects.

### _shaped_text_get_orientation {#_shaped_text_get_orientation}

```gdscript
TextServer.Orientation _shaped_text_get_orientation(shaped: RID) virtual const
```

Returns text orientation.

### _shaped_text_get_parent {#_shaped_text_get_parent}

```gdscript
RID _shaped_text_get_parent(shaped: RID) virtual const
```

Returns the parent buffer from which the substring originates.

### _shaped_text_get_preserve_control {#_shaped_text_get_preserve_control}

```gdscript
bool _shaped_text_get_preserve_control(shaped: RID) virtual const
```

Returns true if text buffer is configured to display control characters.

### _shaped_text_get_preserve_invalid {#_shaped_text_get_preserve_invalid}

```gdscript
bool _shaped_text_get_preserve_invalid(shaped: RID) virtual const
```

Returns true if text buffer is configured to display hexadecimal codes in place of invalid characters.

### _shaped_text_get_range {#_shaped_text_get_range}

```gdscript
Vector2i _shaped_text_get_range(shaped: RID) virtual const
```

Returns substring buffer character range in the parent buffer.

### _shaped_text_get_selection {#_shaped_text_get_selection}

```gdscript
PackedVector2Array _shaped_text_get_selection(shaped: RID, start: int, end: int) virtual const
```

Returns selection rectangles for the specified character range.

### _shaped_text_get_size {#_shaped_text_get_size}

```gdscript
Vector2 _shaped_text_get_size(shaped: RID) virtual const
```

Returns size of the text.

### _shaped_text_get_spacing {#_shaped_text_get_spacing}

```gdscript
int _shaped_text_get_spacing(shaped: RID, spacing: TextServer.SpacingType) virtual const
```

Returns extra spacing added between glyphs or lines in pixels.

### _shaped_text_get_trim_pos {#_shaped_text_get_trim_pos}

```gdscript
int _shaped_text_get_trim_pos(shaped: RID) virtual const
```

Returns the position of the overrun trim.

### _shaped_text_get_underline_position {#_shaped_text_get_underline_position}

```gdscript
float _shaped_text_get_underline_position(shaped: RID) virtual const
```

Returns pixel offset of the underline below the baseline.

### _shaped_text_get_underline_thickness {#_shaped_text_get_underline_thickness}

```gdscript
float _shaped_text_get_underline_thickness(shaped: RID) virtual const
```

Returns thickness of the underline.

### _shaped_text_get_width {#_shaped_text_get_width}

```gdscript
float _shaped_text_get_width(shaped: RID) virtual const
```

Returns width (for horizontal layout) or height (for vertical) of the text.

### _shaped_text_get_word_breaks {#_shaped_text_get_word_breaks}

```gdscript
PackedInt32Array _shaped_text_get_word_breaks(shaped: RID, grapheme_flags: bitfield::TextServer.GraphemeFlag, skip_grapheme_flags: bitfield::TextServer.GraphemeFlag) virtual const
```

Breaks text into words and returns array of character ranges. Use grapheme_flags to set what characters are used for breaking.

### _shaped_text_has_object {#_shaped_text_has_object}

```gdscript
bool _shaped_text_has_object(shaped: RID, key: Variant) virtual const
```

Returns true if an object with key is embedded in this shaped text buffer.

### _shaped_text_hit_test_grapheme {#_shaped_text_hit_test_grapheme}

```gdscript
int _shaped_text_hit_test_grapheme(shaped: RID, coord: float) virtual const
```

Returns grapheme index at the specified pixel offset at the baseline, or -1 if none is found.

### _shaped_text_hit_test_position {#_shaped_text_hit_test_position}

```gdscript
int _shaped_text_hit_test_position(shaped: RID, coord: float) virtual const
```

Returns caret character offset at the specified pixel offset at the baseline. This function always returns a valid position.

### _shaped_text_is_ready {#_shaped_text_is_ready}

```gdscript
bool _shaped_text_is_ready(shaped: RID) virtual const
```

Returns true if buffer is successfully shaped.

### _shaped_text_next_character_pos {#_shaped_text_next_character_pos}

```gdscript
int _shaped_text_next_character_pos(shaped: RID, pos: int) virtual const
```

Returns composite character end position closest to the pos.

### _shaped_text_next_grapheme_pos {#_shaped_text_next_grapheme_pos}

```gdscript
int _shaped_text_next_grapheme_pos(shaped: RID, pos: int) virtual const
```

Returns grapheme end position closest to the pos.

### _shaped_text_overrun_trim_to_width {#_shaped_text_overrun_trim_to_width}

```gdscript
void _shaped_text_overrun_trim_to_width(shaped: RID, width: float, trim_flags: bitfield::TextServer.TextOverrunFlag) virtual
```

Trims text if it exceeds the given width.

### _shaped_text_prev_character_pos {#_shaped_text_prev_character_pos}

```gdscript
int _shaped_text_prev_character_pos(shaped: RID, pos: int) virtual const
```

Returns composite character start position closest to the pos.

### _shaped_text_prev_grapheme_pos {#_shaped_text_prev_grapheme_pos}

```gdscript
int _shaped_text_prev_grapheme_pos(shaped: RID, pos: int) virtual const
```

Returns grapheme start position closest to the pos.

### _shaped_text_resize_object {#_shaped_text_resize_object}

```gdscript
bool _shaped_text_resize_object(shaped: RID, key: Variant, size: Vector2, inline_align: InlineAlignment, baseline: float) virtual
```

Sets new size and alignment of embedded object.

### _shaped_text_set_bidi_override {#_shaped_text_set_bidi_override}

```gdscript
void _shaped_text_set_bidi_override(shaped: RID, override: Array) virtual
```

Overrides BiDi for the structured text.

### _shaped_text_set_custom_ellipsis {#_shaped_text_set_custom_ellipsis}

```gdscript
void _shaped_text_set_custom_ellipsis(shaped: RID, char: int) virtual
```

Sets ellipsis character used for text clipping.

### _shaped_text_set_custom_punctuation {#_shaped_text_set_custom_punctuation}

```gdscript
void _shaped_text_set_custom_punctuation(shaped: RID, punct: String) virtual
```

Sets custom punctuation character list, used for word breaking. If set to empty string, server defaults are used.

### _shaped_text_set_direction {#_shaped_text_set_direction}

```gdscript
void _shaped_text_set_direction(shaped: RID, direction: TextServer.Direction) virtual
```

Sets desired text direction. If set to TextServer.DIRECTION_AUTO, direction will be detected based on the buffer contents and current locale.

### _shaped_text_set_orientation {#_shaped_text_set_orientation}

```gdscript
void _shaped_text_set_orientation(shaped: RID, orientation: TextServer.Orientation) virtual
```

Sets desired text orientation.

### _shaped_text_set_preserve_control {#_shaped_text_set_preserve_control}

```gdscript
void _shaped_text_set_preserve_control(shaped: RID, enabled: bool) virtual
```

If set to true text buffer will display control characters.

### _shaped_text_set_preserve_invalid {#_shaped_text_set_preserve_invalid}

```gdscript
void _shaped_text_set_preserve_invalid(shaped: RID, enabled: bool) virtual
```

If set to true text buffer will display invalid characters as hexadecimal codes, otherwise nothing is displayed.

### _shaped_text_set_spacing {#_shaped_text_set_spacing}

```gdscript
void _shaped_text_set_spacing(shaped: RID, spacing: TextServer.SpacingType, value: int) virtual
```

Sets extra spacing added between glyphs or lines in pixels.

### _shaped_text_shape {#_shaped_text_shape}

```gdscript
bool _shaped_text_shape(shaped: RID) virtual
```

Shapes buffer if it's not shaped. Returns true if the string is shaped successfully.

### _shaped_text_sort_logical {#_shaped_text_sort_logical}

```gdscript
const Glyph* _shaped_text_sort_logical(shaped: RID) virtual
```

Returns text glyphs in the logical order.

### _shaped_text_substr {#_shaped_text_substr}

```gdscript
RID _shaped_text_substr(shaped: RID, start: int, length: int) virtual const
```

Returns text buffer for the substring of the text in the shaped text buffer (including inline objects).

### _shaped_text_tab_align {#_shaped_text_tab_align}

```gdscript
float _shaped_text_tab_align(shaped: RID, tab_stops: PackedFloat32Array) virtual
```

Aligns shaped text to the given tab-stops.

### _shaped_text_update_breaks {#_shaped_text_update_breaks}

```gdscript
bool _shaped_text_update_breaks(shaped: RID) virtual
```

Updates break points in the shaped text. This method is called by default implementation of text breaking functions.

### _shaped_text_update_justification_ops {#_shaped_text_update_justification_ops}

```gdscript
bool _shaped_text_update_justification_ops(shaped: RID) virtual
```

Updates justification points in the shaped text. This method is called by default implementation of text justification functions.

### _spoof_check {#_spoof_check}

```gdscript
bool _spoof_check(string: String) virtual const
```

Returns true if string is likely to be an attempt at confusing the reader.

### _string_get_character_breaks {#_string_get_character_breaks}

```gdscript
PackedInt32Array _string_get_character_breaks(string: String, language: String) virtual const
```

Returns array of the composite character boundaries.

### _string_get_word_breaks {#_string_get_word_breaks}

```gdscript
PackedInt32Array _string_get_word_breaks(string: String, language: String, chars_per_line: int) virtual const
```

Returns an array of the word break boundaries. Elements in the returned array are the offsets of the start and end of words. Therefore the length of the array is always even.

### _string_to_lower {#_string_to_lower}

```gdscript
String _string_to_lower(string: String, language: String) virtual const
```

Returns the string converted to lowercase.

### _string_to_title {#_string_to_title}

```gdscript
String _string_to_title(string: String, language: String) virtual const
```

Returns the string converted to Title Case.

### _string_to_upper {#_string_to_upper}

```gdscript
String _string_to_upper(string: String, language: String) virtual const
```

Returns the string converted to UPPERCASE.

### _strip_diacritics {#_strip_diacritics}

```gdscript
String _strip_diacritics(string: String) virtual const
```

Strips diacritics from the string.

### _tag_to_name {#_tag_to_name}

```gdscript
String _tag_to_name(tag: int) virtual const
```

Converts the given OpenType tag to the readable name of a feature, variation, script, or language.

### _unreference_oversampling_level {#_unreference_oversampling_level}

```gdscript
void _unreference_oversampling_level(oversampling: float) virtual
```

Decreases the reference count of the specified oversampling level, and frees the font cache for oversampling level when the reference count reaches zero. This method is called by Viewport, and should not be used directly.
