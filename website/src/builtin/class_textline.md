# TextLine

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **TextLine**

> Abstraction over TextServer for handling a single line of text.

<details><summary>官方英文描述（中文精译进行中）</summary>

Abstraction over TextServer for handling a single line of text.

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

Clears text line (removes text and inline objects).

### draw {#draw}

```gdscript
void draw(canvas: RID, pos: Vector2, color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw text into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### draw_outline {#draw_outline}

```gdscript
void draw_outline(canvas: RID, pos: Vector2, outline_size: int = 1, color: Color = Color(1, 1, 1, 1), oversampling: float = 0.0) const
```

Draw text into a canvas item at a given position, with color. pos specifies the top left corner of the bounding box. If oversampling is greater than zero, it is used as font oversampling factor, otherwise viewport oversampling settings are used.

### duplicate {#duplicate}

```gdscript
TextLine duplicate() const
```

Duplicates this TextLine.

### get_direction {#get_direction}

```gdscript
TextServer.Direction get_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ellipsis_char {#get_ellipsis_char}

```gdscript
String get_ellipsis_char() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_flags {#get_flags}

```gdscript
bitfield::TextServer.JustificationFlag get_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_horizontal_alignment {#get_horizontal_alignment}

```gdscript
HorizontalAlignment get_horizontal_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inferred_direction {#get_inferred_direction}

```gdscript
TextServer.Direction get_inferred_direction() const
```

Returns the text writing direction inferred by the BiDi algorithm.

### get_line_ascent {#get_line_ascent}

```gdscript
float get_line_ascent() const
```

Returns the text ascent (number of pixels above the baseline for horizontal layout or to the left of baseline for vertical).

### get_line_descent {#get_line_descent}

```gdscript
float get_line_descent() const
```

Returns the text descent (number of pixels below the baseline for horizontal layout or to the right of baseline for vertical).

### get_line_underline_position {#get_line_underline_position}

```gdscript
float get_line_underline_position() const
```

Returns pixel offset of the underline below the baseline.

### get_line_underline_thickness {#get_line_underline_thickness}

```gdscript
float get_line_underline_thickness() const
```

Returns thickness of the underline.

### get_line_width {#get_line_width}

```gdscript
float get_line_width() const
```

Returns width (for horizontal layout) or height (for vertical) of the text.

### get_object_rect {#get_object_rect}

```gdscript
Rect2 get_object_rect(key: Variant) const
```

Returns bounding rectangle of the inline object.

### get_objects {#get_objects}

```gdscript
Array get_objects() const
```

Returns array of inline objects.

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

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns TextServer buffer RID.

### get_size {#get_size}

```gdscript
Vector2 get_size() const
```

Returns size of the bounding box of the text.

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

Returns true if an object with key is embedded in this line.

### hit_test {#hit_test}

```gdscript
int hit_test(coords: float) const
```

Returns caret character offset at the specified pixel offset at the baseline. This function always returns a valid position.

### resize_object {#resize_object}

```gdscript
bool resize_object(key: Variant, size: Vector2, inline_align: InlineAlignment = 5, baseline: float = 0.0)
```

Sets new size and alignment of embedded object.

### set_bidi_override {#set_bidi_override}

```gdscript
void set_bidi_override(override: Array)
```

Overrides BiDi for the structured text.

Override ranges should cover full source text without overlaps. BiDi algorithm will be used on each range separately.

### set_direction {#set_direction}

```gdscript
void set_direction(direction: TextServer.Direction)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ellipsis_char {#set_ellipsis_char}

```gdscript
void set_ellipsis_char(char: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flags {#set_flags}

```gdscript
void set_flags(flags: bitfield::TextServer.JustificationFlag)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_horizontal_alignment {#set_horizontal_alignment}

```gdscript
void set_horizontal_alignment(alignment: HorizontalAlignment)
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

Aligns text to the given tab-stops.

## 属性

### direction {#direction}

```gdscript
direction: int
```

- void set_direction(value: Direction)
- Direction get_direction()

Text writing direction.

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

### width {#width}

```gdscript
width: float
```

- void set_width(value: float)
- float get_width()

Text line width.

### alignment {#alignment}

```gdscript
alignment: int
```

- void set_horizontal_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_horizontal_alignment()

Sets text alignment within the line as if the line was horizontal.

### flags {#flags}

```gdscript
flags: int
```

- void set_flags(value: BitField[JustificationFlag])
- BitField[JustificationFlag] get_flags()

Line alignment rules. For more info see TextServer.

### text_overrun_behavior {#text_overrun_behavior}

```gdscript
text_overrun_behavior: int
```

- void set_text_overrun_behavior(value: OverrunBehavior)
- OverrunBehavior get_text_overrun_behavior()

The clipping behavior when the text exceeds the text line's set width.

### ellipsis_char {#ellipsis_char}

```gdscript
ellipsis_char: String
```

- void set_ellipsis_char(value: String)
- String get_ellipsis_char()

Ellipsis character used for text clipping.
