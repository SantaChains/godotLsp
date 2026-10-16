# Label3D

**继承**：[GeometryInstance3D](class_geometryinstance3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Label3D**

> A node for displaying plain text in 3D space. By adjusting various properties of this node, you can configure things such as the text's appearance and whether it always faces the camera.

<details><summary>官方英文描述（中文精译进行中）</summary>

A node for displaying plain text in 3D space. By adjusting various properties of this node, you can configure things such as the text's appearance and whether it always faces the camera.

</details>

## 方法

### generate_triangle_mesh {#generate_triangle_mesh}

```gdscript
TriangleMesh generate_triangle_mesh() const
```

Returns a TriangleMesh with the label's vertices following its current configuration (such as its pixel_size).

### get_alpha_antialiasing {#get_alpha_antialiasing}

```gdscript
BaseMaterial3D.AlphaAntiAliasing get_alpha_antialiasing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_alpha_antialiasing_edge {#get_alpha_antialiasing_edge}

```gdscript
float get_alpha_antialiasing_edge() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_alpha_cut_mode {#get_alpha_cut_mode}

```gdscript
Label3D.AlphaCutMode get_alpha_cut_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_alpha_hash_scale {#get_alpha_hash_scale}

```gdscript
float get_alpha_hash_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_alpha_scissor_threshold {#get_alpha_scissor_threshold}

```gdscript
float get_alpha_scissor_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_autowrap_mode {#get_autowrap_mode}

```gdscript
TextServer.AutowrapMode get_autowrap_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_autowrap_trim_flags {#get_autowrap_trim_flags}

```gdscript
bitfield::TextServer.LineBreakFlag get_autowrap_trim_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_billboard_mode {#get_billboard_mode}

```gdscript
BaseMaterial3D.BillboardMode get_billboard_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_draw_flag {#get_draw_flag}

```gdscript
bool get_draw_flag(flag: Label3D.DrawFlags) const
```

Returns the value of the specified flag.

### get_font {#get_font}

```gdscript
Font get_font() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_font_size {#get_font_size}

```gdscript
int get_font_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_horizontal_alignment {#get_horizontal_alignment}

```gdscript
HorizontalAlignment get_horizontal_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_justification_flags {#get_justification_flags}

```gdscript
bitfield::TextServer.JustificationFlag get_justification_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_spacing {#get_line_spacing}

```gdscript
float get_line_spacing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_modulate {#get_modulate}

```gdscript
Color get_modulate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset {#get_offset}

```gdscript
Vector2 get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outline_modulate {#get_outline_modulate}

```gdscript
Color get_outline_modulate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outline_render_priority {#get_outline_render_priority}

```gdscript
int get_outline_render_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outline_size {#get_outline_size}

```gdscript
int get_outline_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pixel_size {#get_pixel_size}

```gdscript
float get_pixel_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_render_priority {#get_render_priority}

```gdscript
int get_render_priority() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_structured_text_bidi_override {#get_structured_text_bidi_override}

```gdscript
TextServer.StructuredTextParser get_structured_text_bidi_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_structured_text_bidi_override_options {#get_structured_text_bidi_override_options}

```gdscript
Array get_structured_text_bidi_override_options() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text {#get_text}

```gdscript
String get_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_direction {#get_text_direction}

```gdscript
TextServer.Direction get_text_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_texture_filter {#get_texture_filter}

```gdscript
BaseMaterial3D.TextureFilter get_texture_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertical_alignment {#get_vertical_alignment}

```gdscript
VerticalAlignment get_vertical_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_width {#get_width}

```gdscript
float get_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_uppercase {#is_uppercase}

```gdscript
bool is_uppercase() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_antialiasing {#set_alpha_antialiasing}

```gdscript
void set_alpha_antialiasing(alpha_aa: BaseMaterial3D.AlphaAntiAliasing)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_antialiasing_edge {#set_alpha_antialiasing_edge}

```gdscript
void set_alpha_antialiasing_edge(edge: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_cut_mode {#set_alpha_cut_mode}

```gdscript
void set_alpha_cut_mode(mode: Label3D.AlphaCutMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_hash_scale {#set_alpha_hash_scale}

```gdscript
void set_alpha_hash_scale(threshold: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_scissor_threshold {#set_alpha_scissor_threshold}

```gdscript
void set_alpha_scissor_threshold(threshold: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autowrap_mode {#set_autowrap_mode}

```gdscript
void set_autowrap_mode(autowrap_mode: TextServer.AutowrapMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autowrap_trim_flags {#set_autowrap_trim_flags}

```gdscript
void set_autowrap_trim_flags(autowrap_trim_flags: bitfield::TextServer.LineBreakFlag)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_billboard_mode {#set_billboard_mode}

```gdscript
void set_billboard_mode(mode: BaseMaterial3D.BillboardMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_flag {#set_draw_flag}

```gdscript
void set_draw_flag(flag: Label3D.DrawFlags, enabled: bool)
```

If true, the specified flag will be enabled.

### set_font {#set_font}

```gdscript
void set_font(font: Font)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font_size {#set_font_size}

```gdscript
void set_font_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_horizontal_alignment {#set_horizontal_alignment}

```gdscript
void set_horizontal_alignment(alignment: HorizontalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_justification_flags {#set_justification_flags}

```gdscript
void set_justification_flags(justification_flags: bitfield::TextServer.JustificationFlag)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line_spacing {#set_line_spacing}

```gdscript
void set_line_spacing(line_spacing: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_modulate {#set_modulate}

```gdscript
void set_modulate(modulate: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outline_modulate {#set_outline_modulate}

```gdscript
void set_outline_modulate(modulate: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outline_render_priority {#set_outline_render_priority}

```gdscript
void set_outline_render_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outline_size {#set_outline_size}

```gdscript
void set_outline_size(outline_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pixel_size {#set_pixel_size}

```gdscript
void set_pixel_size(pixel_size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_render_priority {#set_render_priority}

```gdscript
void set_render_priority(priority: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_structured_text_bidi_override {#set_structured_text_bidi_override}

```gdscript
void set_structured_text_bidi_override(parser: TextServer.StructuredTextParser)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_structured_text_bidi_override_options {#set_structured_text_bidi_override_options}

```gdscript
void set_structured_text_bidi_override_options(args: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text {#set_text}

```gdscript
void set_text(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_direction {#set_text_direction}

```gdscript
void set_text_direction(direction: TextServer.Direction)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_filter {#set_texture_filter}

```gdscript
void set_texture_filter(mode: BaseMaterial3D.TextureFilter)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_uppercase {#set_uppercase}

```gdscript
void set_uppercase(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical_alignment {#set_vertical_alignment}

```gdscript
void set_vertical_alignment(alignment: VerticalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_width {#set_width}

```gdscript
void set_width(width: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### pixel_size {#pixel_size}

```gdscript
pixel_size: float
```

- void set_pixel_size(value: float)
- float get_pixel_size()

The size of one pixel's width on the label to scale it in 3D. To make the font look more detailed when up close, increase font_size while decreasing pixel_size at the same time.

### offset {#offset}

```gdscript
offset: Vector2
```

- void set_offset(value: Vector2)
- Vector2 get_offset()

The text drawing offset (in pixels).

### billboard {#billboard}

```gdscript
billboard: int
```

- void set_billboard_mode(value: BillboardMode)
- BillboardMode get_billboard_mode()

The billboard mode to use for the label.

### shaded {#shaded}

```gdscript
shaded: bool
```

- void set_draw_flag(flag: DrawFlags, enabled: bool)
- bool get_draw_flag(flag: DrawFlags) const

If true, the Light3D in the Environment has effects on the label.

### double_sided {#double_sided}

```gdscript
double_sided: bool
```

- void set_draw_flag(flag: DrawFlags, enabled: bool)
- bool get_draw_flag(flag: DrawFlags) const

If true, text can be seen from the back as well, if false, it is invisible when looking at it from behind.

### no_depth_test {#no_depth_test}

```gdscript
no_depth_test: bool
```

- void set_draw_flag(flag: DrawFlags, enabled: bool)
- bool get_draw_flag(flag: DrawFlags) const

If true, depth testing is disabled and the object will be drawn in render order.

### fixed_size {#fixed_size}

```gdscript
fixed_size: bool
```

- void set_draw_flag(flag: DrawFlags, enabled: bool)
- bool get_draw_flag(flag: DrawFlags) const

If true, the label is rendered at the same size regardless of distance. The label's size on screen is the same as if the camera was 1.0 units away from the label's origin, regardless of the actual distance from the camera. The Camera3D's field of view (or Camera3D.size when in orthogonal/frustum mode) still affects the size the label is drawn at.

### alpha_cut {#alpha_cut}

```gdscript
alpha_cut: int
```

- void set_alpha_cut_mode(value: AlphaCutMode)
- AlphaCutMode get_alpha_cut_mode()

The alpha cutting mode to use for the sprite.

### alpha_scissor_threshold {#alpha_scissor_threshold}

```gdscript
alpha_scissor_threshold: float
```

- void set_alpha_scissor_threshold(value: float)
- float get_alpha_scissor_threshold()

Threshold at which the alpha scissor will discard values.

### alpha_hash_scale {#alpha_hash_scale}

```gdscript
alpha_hash_scale: float
```

- void set_alpha_hash_scale(value: float)
- float get_alpha_hash_scale()

The hashing scale for Alpha Hash. Recommended values between 0 and 2.

### alpha_antialiasing_mode {#alpha_antialiasing_mode}

```gdscript
alpha_antialiasing_mode: int
```

- void set_alpha_antialiasing(value: AlphaAntiAliasing)
- AlphaAntiAliasing get_alpha_antialiasing()

The type of alpha antialiasing to apply.

### alpha_antialiasing_edge {#alpha_antialiasing_edge}

```gdscript
alpha_antialiasing_edge: float
```

- void set_alpha_antialiasing_edge(value: float)
- float get_alpha_antialiasing_edge()

Threshold at which antialiasing will be applied on the alpha channel.

### texture_filter {#texture_filter}

```gdscript
texture_filter: int
```

- void set_texture_filter(value: TextureFilter)
- TextureFilter get_texture_filter()

Filter flags for the texture.

### render_priority {#render_priority}

```gdscript
render_priority: int
```

- void set_render_priority(value: int)
- int get_render_priority()

Sets the render priority for the text. Higher priority objects will be sorted in front of lower priority objects.

Note: This only applies if alpha_cut is set to ALPHA_CUT_DISABLED (default value).

Note: This only applies to sorting of transparent objects. This will not impact how transparent objects are sorted relative to opaque objects. This is because opaque objects are not sorted, while transparent objects are sorted from back to front (subject to priority).

### outline_render_priority {#outline_render_priority}

```gdscript
outline_render_priority: int
```

- void set_outline_render_priority(value: int)
- int get_outline_render_priority()

Sets the render priority for the text outline. Higher priority objects will be sorted in front of lower priority objects.

Note: This only applies if alpha_cut is set to ALPHA_CUT_DISABLED (default value).

Note: This only applies to sorting of transparent objects. This will not impact how transparent objects are sorted relative to opaque objects. This is because opaque objects are not sorted, while transparent objects are sorted from back to front (subject to priority).

### modulate {#modulate}

```gdscript
modulate: Color
```

- void set_modulate(value: Color)
- Color get_modulate()

Text Color of the Label3D.

### outline_modulate {#outline_modulate}

```gdscript
outline_modulate: Color
```

- void set_outline_modulate(value: Color)
- Color get_outline_modulate()

The tint of text outline.

### text {#text}

```gdscript
text: String
```

- void set_text(value: String)
- String get_text()

The text to display on screen.

### font {#font}

```gdscript
font: Font
```

- void set_font(value: Font)
- Font get_font()

Font configuration used to display text.

### font_size {#font_size}

```gdscript
font_size: int
```

- void set_font_size(value: int)
- int get_font_size()

Font size of the Label3D's text. To make the font look more detailed when up close, increase font_size while decreasing pixel_size at the same time.

Higher font sizes require more time to render new characters, which can cause stuttering during gameplay.

### outline_size {#outline_size}

```gdscript
outline_size: int
```

- void set_outline_size(value: int)
- int get_outline_size()

Text outline size.

### horizontal_alignment {#horizontal_alignment}

```gdscript
horizontal_alignment: int
```

- void set_horizontal_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_horizontal_alignment()

Controls the text's horizontal alignment. Supports left, center, right, and fill (also known as justify).

### vertical_alignment {#vertical_alignment}

```gdscript
vertical_alignment: int
```

- void set_vertical_alignment(value: VerticalAlignment)
- VerticalAlignment get_vertical_alignment()

Controls the text's vertical alignment. Supports top, center, and bottom.

### uppercase {#uppercase}

```gdscript
uppercase: bool
```

- void set_uppercase(value: bool)
- bool is_uppercase()

If true, all the text displays as UPPERCASE.

### line_spacing {#line_spacing}

```gdscript
line_spacing: float
```

- void set_line_spacing(value: float)
- float get_line_spacing()

Additional vertical spacing between lines (in pixels), spacing is added to line descent. This value can be negative.

### autowrap_mode {#autowrap_mode}

```gdscript
autowrap_mode: int
```

- void set_autowrap_mode(value: AutowrapMode)
- AutowrapMode get_autowrap_mode()

If set to something other than TextServer.AUTOWRAP_OFF, the text gets wrapped inside the node's bounding rectangle. If you resize the node, it will change its height automatically to show all the text.

### autowrap_trim_flags {#autowrap_trim_flags}

```gdscript
autowrap_trim_flags: int
```

- void set_autowrap_trim_flags(value: BitField[LineBreakFlag])
- BitField[LineBreakFlag] get_autowrap_trim_flags()

Autowrap space trimming flags. See TextServer.BREAK_TRIM_START_EDGE_SPACES and TextServer.BREAK_TRIM_END_EDGE_SPACES for more info.

### justification_flags {#justification_flags}

```gdscript
justification_flags: int
```

- void set_justification_flags(value: BitField[JustificationFlag])
- BitField[JustificationFlag] get_justification_flags()

Line fill alignment rules.

### width {#width}

```gdscript
width: float
```

- void set_width(value: float)
- float get_width()

Text width (in pixels), used for autowrap and fill alignment.

### text_direction {#text_direction}

```gdscript
text_direction: int
```

- void set_text_direction(value: Direction)
- Direction get_text_direction()

Base text writing direction.

### language {#language}

```gdscript
language: String
```

- void set_language(value: String)
- String get_language()

Language code used for line-breaking and text shaping algorithms. If left empty, the current locale is used instead.

### structured_text_bidi_override {#structured_text_bidi_override}

```gdscript
structured_text_bidi_override: int
```

- void set_structured_text_bidi_override(value: StructuredTextParser)
- StructuredTextParser get_structured_text_bidi_override()

Set BiDi algorithm override for the structured text.

### structured_text_bidi_override_options {#structured_text_bidi_override_options}

```gdscript
structured_text_bidi_override_options: Array
```

- void set_structured_text_bidi_override_options(value: Array)
- Array get_structured_text_bidi_override_options()

Set additional options for BiDi override.

## 枚举

### DrawFlags {#enum-DrawFlags}

```gdscript
enum DrawFlags { FLAG_SHADED = 0, FLAG_DOUBLE_SIDED = 1, FLAG_DISABLE_DEPTH_TEST = 2, FLAG_FIXED_SIZE = 3, FLAG_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_SHADED {#FLAG_SHADED}

```gdscript
FLAG_SHADED = 0
```

If set, lights in the environment affect the label.

#### FLAG_DOUBLE_SIDED {#FLAG_DOUBLE_SIDED}

```gdscript
FLAG_DOUBLE_SIDED = 1
```

If set, text can be seen from the back as well. If not, the text is invisible when looking at it from behind.

#### FLAG_DISABLE_DEPTH_TEST {#FLAG_DISABLE_DEPTH_TEST}

```gdscript
FLAG_DISABLE_DEPTH_TEST = 2
```

Disables the depth test, so this object is drawn on top of all others. However, objects drawn after it in the draw order may cover it.

#### FLAG_FIXED_SIZE {#FLAG_FIXED_SIZE}

```gdscript
FLAG_FIXED_SIZE = 3
```

Label is scaled by depth so that it always appears the same size on screen.

#### FLAG_MAX {#FLAG_MAX}

```gdscript
FLAG_MAX = 4
```

Represents the size of the DrawFlags enum.

---

### AlphaCutMode {#enum-AlphaCutMode}

```gdscript
enum AlphaCutMode { ALPHA_CUT_DISABLED = 0, ALPHA_CUT_DISCARD = 1, ALPHA_CUT_OPAQUE_PREPASS = 2, ALPHA_CUT_HASH = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ALPHA_CUT_DISABLED {#ALPHA_CUT_DISABLED}

```gdscript
ALPHA_CUT_DISABLED = 0
```

This mode performs standard alpha blending. It can display translucent areas, but transparency sorting issues may be visible when multiple transparent materials are overlapping. GeometryInstance3D.cast_shadow has no effect when this transparency mode is used; the Label3D will never cast shadows.

#### ALPHA_CUT_DISCARD {#ALPHA_CUT_DISCARD}

```gdscript
ALPHA_CUT_DISCARD = 1
```

This mode only allows fully transparent or fully opaque pixels. Harsh edges will be visible unless some form of screen-space antialiasing is enabled (see ProjectSettings.rendering/anti_aliasing/quality/screen_space_aa). This mode is also known as alpha testing or 1-bit transparency.

Note: This mode might have issues with anti-aliased fonts and outlines, try adjusting alpha_scissor_threshold or using MSDF font.

Note: When using text with overlapping glyphs (e.g., cursive scripts), this mode might have transparency sorting issues between the main text and the outline.

#### ALPHA_CUT_OPAQUE_PREPASS {#ALPHA_CUT_OPAQUE_PREPASS}

```gdscript
ALPHA_CUT_OPAQUE_PREPASS = 2
```

This mode draws fully opaque pixels in the depth prepass. This is slower than ALPHA_CUT_DISABLED or ALPHA_CUT_DISCARD, but it allows displaying translucent areas and smooth edges while using proper sorting.

Note: When using text with overlapping glyphs (e.g., cursive scripts), this mode might have transparency sorting issues between the main text and the outline.

#### ALPHA_CUT_HASH {#ALPHA_CUT_HASH}

```gdscript
ALPHA_CUT_HASH = 3
```

This mode draws cuts off all values below a spatially-deterministic threshold, the rest will remain opaque.

---
