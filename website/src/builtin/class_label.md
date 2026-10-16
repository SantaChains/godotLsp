# Label

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Label**

> A control for displaying plain text. It gives you control over the horizontal and vertical alignment and can wrap the text inside the node's bounding rectangle. It doesn't support bold, italics, or other rich text formatting. For that, use RichTextLabel instead.

<details><summary>官方英文描述（中文精译进行中）</summary>

A control for displaying plain text. It gives you control over the horizontal and vertical alignment and can wrap the text inside the node's bounding rectangle. It doesn't support bold, italics, or other rich text formatting. For that, use RichTextLabel instead.

Note: A single Label node is not designed to display huge amounts of text. To display large amounts of text in a single node, consider using RichTextLabel instead as it supports features like an integrated scroll bar and threading. RichTextLabel generally performs better when displaying large amounts of text (several pages or more).

</details>

## 方法

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

### get_character_bounds {#get_character_bounds}

```gdscript
Rect2 get_character_bounds(pos: int) const
```

Returns the bounding rectangle of the character at position pos in the label's local coordinate system. If the character is a non-visual character or pos is outside the valid range, an empty Rect2 is returned. If the character is a part of a composite grapheme, the bounding rectangle of the whole grapheme is returned.

### get_ellipsis_char {#get_ellipsis_char}

```gdscript
String get_ellipsis_char() const
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

### get_label_settings {#get_label_settings}

```gdscript
LabelSettings get_label_settings() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_count {#get_line_count}

```gdscript
int get_line_count() const
```

Returns the number of lines of text the Label has.

### get_line_height {#get_line_height}

```gdscript
int get_line_height(line: int = -1) const
```

Returns the height of the line line.

If line is set to -1, returns the biggest line height.

If there are no lines, returns font size in pixels.

### get_lines_skipped {#get_lines_skipped}

```gdscript
int get_lines_skipped() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_lines_visible {#get_max_lines_visible}

```gdscript
int get_max_lines_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_paragraph_separator {#get_paragraph_separator}

```gdscript
String get_paragraph_separator() const
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

### get_tab_stops {#get_tab_stops}

```gdscript
PackedFloat32Array get_tab_stops() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text {#get_text}

```gdscript
String get_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_direction {#get_text_direction}

```gdscript
Control.TextDirection get_text_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_overrun_behavior {#get_text_overrun_behavior}

```gdscript
TextServer.OverrunBehavior get_text_overrun_behavior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_total_character_count {#get_total_character_count}

```gdscript
int get_total_character_count() const
```

Returns the total number of printable characters in the text (excluding spaces and newlines).

### get_vertical_alignment {#get_vertical_alignment}

```gdscript
VerticalAlignment get_vertical_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visible_characters {#get_visible_characters}

```gdscript
int get_visible_characters() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visible_characters_behavior {#get_visible_characters_behavior}

```gdscript
TextServer.VisibleCharactersBehavior get_visible_characters_behavior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visible_line_count {#get_visible_line_count}

```gdscript
int get_visible_line_count() const
```

Returns the number of lines shown. Useful if the Label's height cannot currently display all lines.

### get_visible_ratio {#get_visible_ratio}

```gdscript
float get_visible_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_clipping_text {#is_clipping_text}

```gdscript
bool is_clipping_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_uppercase {#is_uppercase}

```gdscript
bool is_uppercase() const
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

### set_clip_text {#set_clip_text}

```gdscript
void set_clip_text(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ellipsis_char {#set_ellipsis_char}

```gdscript
void set_ellipsis_char(char: String)
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

### set_label_settings {#set_label_settings}

```gdscript
void set_label_settings(settings: LabelSettings)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_lines_skipped {#set_lines_skipped}

```gdscript
void set_lines_skipped(lines_skipped: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_lines_visible {#set_max_lines_visible}

```gdscript
void set_max_lines_visible(lines_visible: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_paragraph_separator {#set_paragraph_separator}

```gdscript
void set_paragraph_separator(paragraph_separator: String)
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

### set_tab_stops {#set_tab_stops}

```gdscript
void set_tab_stops(tab_stops: PackedFloat32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text {#set_text}

```gdscript
void set_text(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_direction {#set_text_direction}

```gdscript
void set_text_direction(direction: Control.TextDirection)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_overrun_behavior {#set_text_overrun_behavior}

```gdscript
void set_text_overrun_behavior(overrun_behavior: TextServer.OverrunBehavior)
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

### set_visible_characters {#set_visible_characters}

```gdscript
void set_visible_characters(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visible_characters_behavior {#set_visible_characters_behavior}

```gdscript
void set_visible_characters_behavior(behavior: TextServer.VisibleCharactersBehavior)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visible_ratio {#set_visible_ratio}

```gdscript
void set_visible_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### text {#text}

```gdscript
text: String
```

- void set_text(value: String)
- String get_text()

The text to display on screen.

### label_settings {#label_settings}

```gdscript
label_settings: LabelSettings
```

- void set_label_settings(value: LabelSettings)
- LabelSettings get_label_settings()

A LabelSettings resource that can be shared between multiple Label nodes. Takes priority over theme properties.

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

Controls the text's vertical alignment. Supports top, center, bottom, and fill.

### autowrap_mode {#autowrap_mode}

```gdscript
autowrap_mode: int
```

- void set_autowrap_mode(value: AutowrapMode)
- AutowrapMode get_autowrap_mode()

If set to something other than TextServer.AUTOWRAP_OFF, the text gets wrapped inside the node's bounding rectangle. If you resize the node, it will change its height automatically to show all the text.

Note: Labels with autowrapping enabled must have a custom maximum width configured to work correctly, either through the Label's own Control.custom_maximum_size or as a result of a propagated maximum size from a parent Control with Control.propagate_maximum_size enabled.

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

### paragraph_separator {#paragraph_separator}

```gdscript
paragraph_separator: String
```

- void set_paragraph_separator(value: String)
- String get_paragraph_separator()

String used as a paragraph separator. Each paragraph is processed independently, in its own BiDi context.

### clip_text {#clip_text}

```gdscript
clip_text: bool
```

- void set_clip_text(value: bool)
- bool is_clipping_text()

If true, the Label only shows the text that fits inside its bounding rectangle and will clip text horizontally.

### text_overrun_behavior {#text_overrun_behavior}

```gdscript
text_overrun_behavior: int
```

- void set_text_overrun_behavior(value: OverrunBehavior)
- OverrunBehavior get_text_overrun_behavior()

The clipping behavior when the text exceeds the node's bounding rectangle.

### ellipsis_char {#ellipsis_char}

```gdscript
ellipsis_char: String
```

- void set_ellipsis_char(value: String)
- String get_ellipsis_char()

Ellipsis character used for text clipping.

### uppercase {#uppercase}

```gdscript
uppercase: bool
```

- void set_uppercase(value: bool)
- bool is_uppercase()

If true, all the text displays as UPPERCASE.

### tab_stops {#tab_stops}

```gdscript
tab_stops: PackedFloat32Array
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### lines_skipped {#lines_skipped}

```gdscript
lines_skipped: int
```

- void set_lines_skipped(value: int)
- int get_lines_skipped()

The number of the lines ignored and not displayed from the start of the text value.

### max_lines_visible {#max_lines_visible}

```gdscript
max_lines_visible: int
```

- void set_max_lines_visible(value: int)
- int get_max_lines_visible()

Limits the lines of text the node shows on screen.

### visible_characters {#visible_characters}

```gdscript
visible_characters: int
```

- void set_visible_characters(value: int)
- int get_visible_characters()

The number of characters to display. If set to -1, all characters are displayed. This can be useful when animating the text appearing in a dialog box.

Note: Setting this property updates visible_ratio accordingly.

Note: Characters are counted as Unicode codepoints. A single visible grapheme may contain multiple codepoints (e.g. certain emoji use three codepoints). A single codepoint may contain two UTF-16 characters, which are used in C# strings.

### visible_characters_behavior {#visible_characters_behavior}

```gdscript
visible_characters_behavior: int
```

- void set_visible_characters_behavior(value: VisibleCharactersBehavior)
- VisibleCharactersBehavior get_visible_characters_behavior()

The clipping behavior when visible_characters or visible_ratio is set.

### visible_ratio {#visible_ratio}

```gdscript
visible_ratio: float
```

- void set_visible_ratio(value: float)
- float get_visible_ratio()

The fraction of characters to display, relative to the total number of characters (see get_total_character_count()). If set to 1.0, all characters are displayed. If set to 0.5, only half of the characters will be displayed. This can be useful when animating the text appearing in a dialog box.

Note: Setting this property updates visible_characters accordingly.

### text_direction {#text_direction}

```gdscript
text_direction: int
```

- void set_text_direction(value: TextDirection)
- TextDirection get_text_direction()

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
