# TextParagraph

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **TextParagraph**

> Abstraction over TextServer for handling a single paragraph of text.

<details><summary>官方英文描述</summary>

Abstraction over TextServer for handling a single paragraph of text.

</details>

## 方法

### add_object {#add_object}

```gdscript
bool add_object(key: Variant, size: Vector2, inline_align: InlineAlignment = 5, length: int = 1, baseline: float = 0.0)
```

Adds inline object to the text buffer, key must be unique. In the text, object is represented as length object replacement characters.

### add_string {#add_string}

```gdscript
bool add_string(text: String, font: Font, font_size: int, language: String = "", meta: Variant = null)
```

Adds text span and font to draw it.

### clear {#clear}

```gdscript
void clear()
```

Clears text paragraph (removes text and inline objects).

### clear_dropcap {#clear_dropcap}

```gdscript
void clear_dropcap()
```

Removes dropcap.

### draw {#draw}

```gdscript
void draw(canvas: RID, pos: Vector2, color: Color = Color(1, 1, 1, 1), dc_color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw all lines of the text and drop cap into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### draw_dropcap {#draw_dropcap}

```gdscript
void draw_dropcap(canvas: RID, pos: Vector2, color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw drop cap into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### draw_dropcap_outline {#draw_dropcap_outline}

```gdscript
void draw_dropcap_outline(canvas: RID, pos: Vector2, outline_size: int = 1, color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw drop cap outline into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### draw_line {#draw_line}

```gdscript
void draw_line(canvas: RID, pos: Vector2, line: int, color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw single line of text into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### draw_line_outline {#draw_line_outline}

```gdscript
void draw_line_outline(canvas: RID, pos: Vector2, line: int, outline_size: int = 1, color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw outline of the single line of text into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### draw_outline {#draw_outline}

```gdscript
void draw_outline(canvas: RID, pos: Vector2, outline_size: int = 1, color: Color = Color(1, 1, 1, 1), dc_color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw outlines of all lines of the text and drop cap into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### duplicate {#duplicate}

```gdscript
TextParagraph duplicate() const
```

Duplicates this TextParagraph.

### get_alignment {#get_alignment}

```gdscript
HorizontalAlignment get_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_break_flags {#get_break_flags}

```gdscript
bitfield::TextServer.LineBreakFlag get_break_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_punctuation {#get_custom_punctuation}

```gdscript
String get_custom_punctuation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_direction {#get_direction}

```gdscript
TextServer.Direction get_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dropcap_lines {#get_dropcap_lines}

```gdscript
int get_dropcap_lines() const
```

Returns number of lines used by dropcap.

### get_dropcap_rid {#get_dropcap_rid}

```gdscript
RID get_dropcap_rid() const
```

Returns drop cap text buffer RID.

### get_dropcap_size {#get_dropcap_size}

```gdscript
Vector2 get_dropcap_size() const
```

Returns drop cap bounding box size.

### get_ellipsis_char {#get_ellipsis_char}

```gdscript
String get_ellipsis_char() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inferred_direction {#get_inferred_direction}

```gdscript
TextServer.Direction get_inferred_direction() const
```

Returns the text writing direction inferred by the BiDi algorithm.

### get_justification_flags {#get_justification_flags}

```gdscript
bitfield::TextServer.JustificationFlag get_justification_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_ascent {#get_line_ascent}

```gdscript
float get_line_ascent(line: int) const
```

Returns the text line ascent (number of pixels above the baseline for horizontal layout or to the left of baseline for vertical).

### get_line_count {#get_line_count}

```gdscript
int get_line_count() const
```

Returns number of lines in the paragraph.

### get_line_descent {#get_line_descent}

```gdscript
float get_line_descent(line: int) const
```

Returns the text line descent (number of pixels below the baseline for horizontal layout or to the right of baseline for vertical).

### get_line_object_rect {#get_line_object_rect}

```gdscript
Rect2 get_line_object_rect(line: int, key: Variant) const
```

Returns bounding rectangle of the inline object.

### get_line_objects {#get_line_objects}

```gdscript
Array get_line_objects(line: int) const
```

Returns array of inline objects in the line.

### get_line_range {#get_line_range}

```gdscript
Vector2i get_line_range(line: int) const
```

Returns character range of the line.

### get_line_rid {#get_line_rid}

```gdscript
RID get_line_rid(line: int) const
```

Returns TextServer line buffer RID.

### get_line_size {#get_line_size}

```gdscript
Vector2 get_line_size(line: int) const
```

Returns size of the bounding box of the line of text. Returned size is rounded up.

### get_line_spacing {#get_line_spacing}

```gdscript
float get_line_spacing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_underline_position {#get_line_underline_position}

```gdscript
float get_line_underline_position(line: int) const
```

Returns pixel offset of the underline below the baseline.

### get_line_underline_thickness {#get_line_underline_thickness}

```gdscript
float get_line_underline_thickness(line: int) const
```

Returns thickness of the underline.

### get_line_width {#get_line_width}

```gdscript
float get_line_width(line: int) const
```

Returns width (for horizontal layout) or height (for vertical) of the line of text.

### get_max_lines_visible {#get_max_lines_visible}

```gdscript
int get_max_lines_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_non_wrapped_size {#get_non_wrapped_size}

```gdscript
Vector2 get_non_wrapped_size() const
```

Returns the size of the bounding box of the paragraph, without line breaks.

### get_orientation {#get_orientation}

```gdscript
TextServer.Orientation get_orientation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_preserve_control {#get_preserve_control}

```gdscript
bool get_preserve_control() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_preserve_invalid {#get_preserve_invalid}

```gdscript
bool get_preserve_invalid() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_range {#get_range}

```gdscript
Vector2i get_range() const
```

Returns the character range of the paragraph.

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns TextServer full string buffer RID.

### get_size {#get_size}

```gdscript
Vector2 get_size() const
```

Returns the size of the bounding box of the paragraph.

### get_text_overrun_behavior {#get_text_overrun_behavior}

```gdscript
TextServer.OverrunBehavior get_text_overrun_behavior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_width {#get_width}

```gdscript
float get_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_object {#has_object}

```gdscript
bool has_object(key: Variant) const
```

Returns true if an object with key is embedded in this shaped text buffer.

### hit_test {#hit_test}

```gdscript
int hit_test(coords: Vector2) const
```

Returns caret character offset at the specified coordinates. This function always returns a valid position.

### resize_object {#resize_object}

```gdscript
bool resize_object(key: Variant, size: Vector2, inline_align: InlineAlignment = 5, baseline: float = 0.0)
```

Sets new size and alignment of embedded object.

### set_alignment {#set_alignment}

```gdscript
void set_alignment(alignment: HorizontalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bidi_override {#set_bidi_override}

```gdscript
void set_bidi_override(override: Array)
```

Overrides BiDi for the structured text.

Override ranges should cover full source text without overlaps. BiDi algorithm will be used on each range separately.

### set_break_flags {#set_break_flags}

```gdscript
void set_break_flags(flags: bitfield::TextServer.LineBreakFlag)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_punctuation {#set_custom_punctuation}

```gdscript
void set_custom_punctuation(custom_punctuation: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_direction {#set_direction}

```gdscript
void set_direction(direction: TextServer.Direction)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dropcap {#set_dropcap}

```gdscript
bool set_dropcap(text: String, font: Font, font_size: int, dropcap_margins: Rect2 = Rect2(0, 0, 0, 0), language: String = "")
```

Sets drop cap, overrides previously set drop cap. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text.

### set_ellipsis_char {#set_ellipsis_char}

```gdscript
void set_ellipsis_char(char: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_justification_flags {#set_justification_flags}

```gdscript
void set_justification_flags(flags: bitfield::TextServer.JustificationFlag)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line_spacing {#set_line_spacing}

```gdscript
void set_line_spacing(line_spacing: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_lines_visible {#set_max_lines_visible}

```gdscript
void set_max_lines_visible(max_lines_visible: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_orientation {#set_orientation}

```gdscript
void set_orientation(orientation: TextServer.Orientation)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_preserve_control {#set_preserve_control}

```gdscript
void set_preserve_control(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_preserve_invalid {#set_preserve_invalid}

```gdscript
void set_preserve_invalid(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_overrun_behavior {#set_text_overrun_behavior}

```gdscript
void set_text_overrun_behavior(overrun_behavior: TextServer.OverrunBehavior)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_width {#set_width}

```gdscript
void set_width(width: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### tab_align {#tab_align}

```gdscript
void tab_align(tab_stops: PackedFloat32Array)
```

Aligns paragraph to the given tab-stops.

## 属性

### direction {#direction}

```gdscript
direction: int
```

- void set_direction(value: Direction)
- Direction get_direction()

Text writing direction.

### custom_punctuation {#custom_punctuation}

```gdscript
custom_punctuation: String
```

- void set_custom_punctuation(value: String)
- String get_custom_punctuation()

Custom punctuation character list, used for word breaking. If set to empty string, server defaults are used.

### orientation {#orientation}

```gdscript
orientation: int
```

- void set_orientation(value: Orientation)
- Orientation get_orientation()

Text orientation.

### preserve_invalid {#preserve_invalid}

```gdscript
preserve_invalid: bool
```

- void set_preserve_invalid(value: bool)
- bool get_preserve_invalid()

If set to true text will display invalid characters.

### preserve_control {#preserve_control}

```gdscript
preserve_control: bool
```

- void set_preserve_control(value: bool)
- bool get_preserve_control()

If set to true text will display control characters.

### alignment {#alignment}

```gdscript
alignment: int
```

- void set_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_alignment()

Paragraph horizontal alignment.

### break_flags {#break_flags}

```gdscript
break_flags: int
```

- void set_break_flags(value: BitField[LineBreakFlag])
- BitField[LineBreakFlag] get_break_flags()

Line breaking rules. For more info see TextServer.

### justification_flags {#justification_flags}

```gdscript
justification_flags: int
```

- void set_justification_flags(value: BitField[JustificationFlag])
- BitField[JustificationFlag] get_justification_flags()

Line fill alignment rules.

### text_overrun_behavior {#text_overrun_behavior}

```gdscript
text_overrun_behavior: int
```

- void set_text_overrun_behavior(value: OverrunBehavior)
- OverrunBehavior get_text_overrun_behavior()

The clipping behavior when the text exceeds the paragraph's set width.

### ellipsis_char {#ellipsis_char}

```gdscript
ellipsis_char: String
```

- void set_ellipsis_char(value: String)
- String get_ellipsis_char()

Ellipsis character used for text clipping.

### width {#width}

```gdscript
width: float
```

- void set_width(value: float)
- float get_width()

Paragraph width.

### max_lines_visible {#max_lines_visible}

```gdscript
max_lines_visible: int
```

- void set_max_lines_visible(value: int)
- int get_max_lines_visible()

Limits the lines of text shown.

### line_spacing {#line_spacing}

```gdscript
line_spacing: float
```

- void set_line_spacing(value: float)
- float get_line_spacing()

Additional vertical spacing between lines (in pixels), spacing is added to line descent. This value can be negative.
