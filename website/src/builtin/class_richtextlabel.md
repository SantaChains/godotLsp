# RichTextLabel

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **RichTextLabel**

> A control for displaying text that can contain custom fonts, images, and basic formatting. RichTextLabel manages these as an internal tag stack. It also adapts itself to given width/heights.

<details><summary>官方英文描述</summary>

A control for displaying text that can contain custom fonts, images, and basic formatting. RichTextLabel manages these as an internal tag stack. It also adapts itself to given width/heights.

Note: newline(), push_paragraph(), "\n", "\r\n", p tag, and alignment tags start a new paragraph. Each paragraph is processed independently, in its own BiDi context. If you want to force line wrapping within paragraph, any other line breaking character can be used, for example, Form Feed (U+000C), Next Line (U+0085), Line Separator (U+2028).

Note: Assignments to text clear the tag stack and reconstruct it from the property's contents. Any edits made to text will erase previous edits made from other manual sources such as append_text() and the push_* / pop() methods.

Note: RichTextLabel doesn't support entangled BBCode tags. For example, instead of using [b]bold[i]bold italic[/b]italic[/i], use [b]bold[i]bold italic[/i][/b][i]italic[/i].

Note: push_*/pop_* functions won't affect BBCode.

Note: While bbcode_enabled is enabled, alignment tags such as [center] will take priority over the horizontal_alignment setting which determines the default text alignment.

</details>

## 方法

### add_hr {#add_hr}

```gdscript
void add_hr(width: int = 90, height: int = 2, color: Color = Color(1, 1, 1, 1), alignment: HorizontalAlignment = 1, width_in_percent: bool = true, height_in_percent: bool = false)
```

Adds a horizontal rule that can be used to separate content.

If width_in_percent is set, width values are percentages of the control width instead of pixels.

If height_in_percent is set, height values are percentages of the control width instead of pixels.

### add_image {#add_image}

```gdscript
void add_image(image: Texture2D, width: float = 0, height: float = 0, color: Color = Color(1, 1, 1, 1), inline_align: InlineAlignment = 5, region: Rect2 = Rect2(0, 0, 0, 0), key: Variant = null, pad: bool = false, tooltip: String = "", width_unit: RichTextLabel.ImageUnit = 0, height_unit: RichTextLabel.ImageUnit = 0, alt_text: String = "")
```

Adds an image's opening and closing tags to the tag stack, optionally providing a width and height to resize the image, a color to tint the image and a region to only use parts of the image.

If width or height is set to 0, the image size will be adjusted in order to keep the original aspect ratio.

If width and height are not set, but region is, the region's rect will be used.

key is an optional identifier, that can be used to modify the image via update_image().

If pad is set, and the image is smaller than the size specified by width and height, the image padding is added to match the size instead of upscaling.

Parameters width_unit and height_unit determine the units used to calculate the image width and height, respectively.

alt_text is used as the image description for assistive apps.

### add_text {#add_text}

```gdscript
void add_text(text: String)
```

Adds raw non-BBCode-parsed text to the tag stack.

### append_text {#append_text}

```gdscript
void append_text(bbcode: String)
```

Parses bbcode and adds tags to the tag stack as needed.

Note: Using this method, you can't close a tag that was opened in a previous append_text() call. This is done to improve performance, especially when updating large RichTextLabels since rebuilding the whole BBCode every time would be slower. If you absolutely need to close a tag in a future method call, append the text instead of using append_text().

### clear {#clear}

```gdscript
void clear()
```

Clears the tag stack, causing the label to display nothing.

Note: This method does not affect text, and its contents will show again if the label is redrawn. However, setting text to an empty String also clears the stack.

### deselect {#deselect}

```gdscript
void deselect()
```

Clears the current selection.

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

### get_character_line {#get_character_line}

```gdscript
int get_character_line(character: int)
```

Returns the line number of the character position provided. Line and character numbers are both zero-indexed.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_character_paragraph {#get_character_paragraph}

```gdscript
int get_character_paragraph(character: int)
```

Returns the paragraph number of the character position provided. Paragraph and character numbers are both zero-indexed.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_content_height {#get_content_height}

```gdscript
int get_content_height() const
```

Returns the height of the content.

Note: This method always returns the full content size, and is not affected by visible_ratio and visible_characters. To get the visible content size, use get_visible_content_rect().

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_content_width {#get_content_width}

```gdscript
int get_content_width() const
```

Returns the width of the content.

Note: This method always returns the full content size, and is not affected by visible_ratio and visible_characters. To get the visible content size, use get_visible_content_rect().

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_effects {#get_effects}

```gdscript
Array get_effects()
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

### get_line_count {#get_line_count}

```gdscript
int get_line_count() const
```

Returns the total number of lines in the text. Wrapped text is counted as multiple lines.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_line_height {#get_line_height}

```gdscript
int get_line_height(line: int) const
```

Returns the height of the line found at the provided index.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether the document is fully loaded.

### get_line_offset {#get_line_offset}

```gdscript
float get_line_offset(line: int)
```

Returns the vertical offset of the line found at the provided index.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_line_range {#get_line_range}

```gdscript
Vector2i get_line_range(line: int)
```

Returns the indexes of the first and last visible characters for the given line, as a Vector2i.

Note: If visible_characters_behavior is set to TextServer.VC_CHARS_BEFORE_SHAPING only visible wrapped lines are counted.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_line_width {#get_line_width}

```gdscript
int get_line_width(line: int) const
```

Returns the width of the line found at the provided index.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether the document is fully loaded.

### get_menu {#get_menu}

```gdscript
PopupMenu get_menu() const
```

Returns the PopupMenu of this RichTextLabel. By default, this menu is displayed when right-clicking on the RichTextLabel.

You can add custom menu items or remove standard ones. Make sure your IDs don't conflict with the standard ones (see MenuItems). For example:

```
func _ready():
    var menu = get_menu()
    # Remove "Select All" item.
    menu.remove_item(MENU_SELECT_ALL)
    # Add custom items.
    menu.add_separator()
    menu.add_item("Duplicate Text", MENU_MAX + 1)
    # Connect callback.
    menu.id_pressed.connect(_on_item_pressed)
func _on_item_pressed(id):
    if id == MENU_MAX + 1:
        add_text("\n" + get_parsed_text())
```

```
public override void _Ready()
{
    var menu = GetMenu();
    // Remove "Select All" item.
    menu.RemoveItem(RichTextLabel.MenuItems.SelectAll);
    // Add custom items.
    menu.AddSeparator();
    menu.AddItem("Duplicate Text", RichTextLabel.MenuItems.Max + 1);
    // Add event handler.
    menu.IdPressed += OnItemPressed;
}
public void OnItemPressed(int id)
{
    if (id == TextEdit.MenuItems.Max + 1)
    {
        AddText("\n" + GetParsedText());
    }
}
```

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their Window.visible property.

### get_paragraph_count {#get_paragraph_count}

```gdscript
int get_paragraph_count() const
```

Returns the total number of paragraphs (newlines or p tags in the tag stack's text tags). Considers wrapped text as one paragraph.

### get_paragraph_offset {#get_paragraph_offset}

```gdscript
float get_paragraph_offset(paragraph: int)
```

Returns the vertical offset of the paragraph found at the provided index.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_parsed_text {#get_parsed_text}

```gdscript
String get_parsed_text() const
```

Returns the text without BBCode mark-up.

### get_progress_bar_delay {#get_progress_bar_delay}

```gdscript
int get_progress_bar_delay() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_selected_text {#get_selected_text}

```gdscript
String get_selected_text() const
```

Returns the current selection text. Does not include BBCodes.

### get_selection_from {#get_selection_from}

```gdscript
int get_selection_from() const
```

Returns the current selection first character index if a selection is active, -1 otherwise. Does not include BBCodes.

### get_selection_line_offset {#get_selection_line_offset}

```gdscript
float get_selection_line_offset() const
```

Returns the current selection vertical line offset if a selection is active, -1.0 otherwise.

### get_selection_to {#get_selection_to}

```gdscript
int get_selection_to() const
```

Returns the current selection last character index if a selection is active, -1 otherwise. Does not include BBCodes.

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

### get_tab_size {#get_tab_size}

```gdscript
int get_tab_size() const
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

### get_total_character_count {#get_total_character_count}

```gdscript
int get_total_character_count() const
```

Returns the total number of characters from text tags. Does not include BBCodes.

### get_v_scroll_bar {#get_v_scroll_bar}

```gdscript
VScrollBar get_v_scroll_bar()
```

Returns the vertical scrollbar.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

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

### get_visible_content_rect {#get_visible_content_rect}

```gdscript
Rect2i get_visible_content_rect() const
```

Returns the bounding rectangle of the visible content.

Note: This method returns a correct value only after the label has been drawn.

```
extends RichTextLabel
@export var background_panel: Panel
func _ready():
    await draw
    background_panel.position = get_visible_content_rect().position
    background_panel.size = get_visible_content_rect().size
```

```
public partial class TestLabel : RichTextLabel
{
    [Export]
    public Panel BackgroundPanel { get; set; }
    public override async void _Ready()
    {
        await ToSignal(this, Control.SignalName.Draw);
        BackgroundGPanel.Position = GetVisibleContentRect().Position;
        BackgroundPanel.Size = GetVisibleContentRect().Size;
    }
}
```

### get_visible_line_count {#get_visible_line_count}

```gdscript
int get_visible_line_count() const
```

Returns the number of visible lines.

Note: This method returns a correct value only after the label has been drawn.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_visible_paragraph_count {#get_visible_paragraph_count}

```gdscript
int get_visible_paragraph_count() const
```

Returns the number of visible paragraphs. A paragraph is considered visible if at least one of its lines is visible.

Note: This method returns a correct value only after the label has been drawn.

Note: If threaded is enabled, this method returns a value for the loaded part of the document. Use is_finished() or finished to determine whether document is fully loaded.

### get_visible_ratio {#get_visible_ratio}

```gdscript
float get_visible_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### install_effect {#install_effect}

```gdscript
void install_effect(effect: Variant)
```

Installs a custom effect. This can also be done in the Inspector through the custom_effects property. effect should be a valid RichTextEffect.

Example: With the following script extending from RichTextEffect:

```gdscript
# effect.gd
class_name MyCustomEffect
extends RichTextEffect
var bbcode = "my_custom_effect"
# ...
```

The above effect can be installed in RichTextLabel from a script:

```gdscript
# rich_text_label.gd
extends RichTextLabel
func _ready():
    install_effect(MyCustomEffect.new())
    # Alternatively, if not using `class_name` in the script that extends RichTextEffect:
    install_effect(preload("res://effect.gd").new())
```

### invalidate_paragraph {#invalidate_paragraph}

```gdscript
bool invalidate_paragraph(paragraph: int)
```

Invalidates paragraph and all subsequent paragraphs cache.

### is_context_menu_enabled {#is_context_menu_enabled}

```gdscript
bool is_context_menu_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_deselect_on_focus_loss_enabled {#is_deselect_on_focus_loss_enabled}

```gdscript
bool is_deselect_on_focus_loss_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drag_and_drop_selection_enabled {#is_drag_and_drop_selection_enabled}

```gdscript
bool is_drag_and_drop_selection_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_finished {#is_finished}

```gdscript
bool is_finished() const
```

If threaded is enabled, returns true if the background thread has finished text processing, otherwise always return true.

### is_fit_content_enabled {#is_fit_content_enabled}

```gdscript
bool is_fit_content_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hint_underlined {#is_hint_underlined}

```gdscript
bool is_hint_underlined() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_menu_visible {#is_menu_visible}

```gdscript
bool is_menu_visible() const
```

Returns whether the menu is visible. Use this instead of get_menu().visible to improve performance (so the creation of the menu is avoided).

### is_meta_underlined {#is_meta_underlined}

```gdscript
bool is_meta_underlined() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_ready {#is_ready}

```gdscript
bool is_ready() const
```

Deprecated: Use is_finished() instead.

If threaded is enabled, returns true if the background thread has finished text processing, otherwise always return true.

### is_scroll_active {#is_scroll_active}

```gdscript
bool is_scroll_active() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scroll_following {#is_scroll_following}

```gdscript
bool is_scroll_following() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scroll_following_visible_characters {#is_scroll_following_visible_characters}

```gdscript
bool is_scroll_following_visible_characters() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_selection_enabled {#is_selection_enabled}

```gdscript
bool is_selection_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_shortcut_keys_enabled {#is_shortcut_keys_enabled}

```gdscript
bool is_shortcut_keys_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_threaded {#is_threaded}

```gdscript
bool is_threaded() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_bbcode {#is_using_bbcode}

```gdscript
bool is_using_bbcode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### menu_option {#menu_option}

```gdscript
void menu_option(option: int)
```

Executes a given action as defined in the MenuItems enum.

### newline {#newline}

```gdscript
void newline()
```

Adds a newline tag to the tag stack.

### parse_bbcode {#parse_bbcode}

```gdscript
void parse_bbcode(bbcode: String)
```

The assignment version of append_text(). Clears the tag stack and inserts the new content.

### parse_expressions_for_values {#parse_expressions_for_values}

```gdscript
Dictionary parse_expressions_for_values(expressions: PackedStringArray)
```

Parses BBCode parameter expressions into a dictionary.

### pop {#pop}

```gdscript
void pop()
```

Terminates the current tag. Use after push_* methods to close BBCodes manually. Does not need to follow add_* methods.

### pop_all {#pop_all}

```gdscript
void pop_all()
```

Terminates all tags opened by push_* methods.

### pop_context {#pop_context}

```gdscript
void pop_context()
```

Terminates tags opened after the last push_context() call (including context marker), or all tags if there's no context marker on the stack.

### push_bgcolor {#push_bgcolor}

```gdscript
void push_bgcolor(bgcolor: Color)
```

Adds a [bgcolor] tag to the tag stack.

Note: The background color has padding applied by default, which is controlled using text_highlight_h_padding and text_highlight_v_padding. This can lead to overlapping highlights if background colors are placed on neighboring lines/columns, so consider setting those theme items to 0 if you want to avoid this.

### push_bold {#push_bold}

```gdscript
void push_bold()
```

Adds a [font] tag with a bold font to the tag stack. This is the same as adding a [b] tag if not currently in a [i] tag.

### push_bold_italics {#push_bold_italics}

```gdscript
void push_bold_italics()
```

Adds a [font] tag with a bold italics font to the tag stack.

### push_cell {#push_cell}

```gdscript
void push_cell()
```

Adds a [cell] tag to the tag stack. Must be inside a [table] tag. See push_table() for details. Use set_table_column_expand() to set column expansion ratio, set_cell_border_color() to set cell border, set_cell_row_background_color() to set cell background, set_cell_size_override() to override cell size, and set_cell_padding() to set padding.

### push_color {#push_color}

```gdscript
void push_color(color: Color)
```

Adds a [color] tag to the tag stack.

### push_context {#push_context}

```gdscript
void push_context()
```

Adds a context marker to the tag stack. See pop_context().

### push_customfx {#push_customfx}

```gdscript
void push_customfx(effect: RichTextEffect, env: Dictionary)
```

Adds a custom effect tag to the tag stack. The effect does not need to be in custom_effects. The environment is directly passed to the effect.

### push_dropcap {#push_dropcap}

```gdscript
void push_dropcap(string: String, font: Font, size: int, dropcap_margins: Rect2 = Rect2(0, 0, 0, 0), color: Color = Color(1, 1, 1, 1), outline_size: int = 0, outline_color: Color = Color(0, 0, 0, 0))
```

Adds a [dropcap] tag to the tag stack. Drop cap (dropped capital) is a decorative element at the beginning of a paragraph that is larger than the rest of the text.

### push_fgcolor {#push_fgcolor}

```gdscript
void push_fgcolor(fgcolor: Color)
```

Adds a [fgcolor] tag to the tag stack.

Note: The foreground color has padding applied by default, which is controlled using text_highlight_h_padding and text_highlight_v_padding. This can lead to overlapping highlights if foreground colors are placed on neighboring lines/columns, so consider setting those theme items to 0 if you want to avoid this.

### push_font {#push_font}

```gdscript
void push_font(font: Font, font_size: int = 0)
```

Adds a [font] tag to the tag stack. Overrides default fonts for its duration.

Passing 0 to font_size will use the existing default font size.

### push_font_size {#push_font_size}

```gdscript
void push_font_size(font_size: int)
```

Adds a [font_size] tag to the tag stack. Overrides default font size for its duration.

### push_hint {#push_hint}

```gdscript
void push_hint(description: String)
```

Adds a [hint] tag to the tag stack. Same as BBCode [hint=something]{text}[/hint].

### push_indent {#push_indent}

```gdscript
void push_indent(level: int)
```

Adds an [indent] tag to the tag stack. Multiplies level by current tab_size to determine new margin length.

### push_italics {#push_italics}

```gdscript
void push_italics()
```

Adds a [font] tag with an italics font to the tag stack. This is the same as adding an [i] tag if not currently in a [b] tag.

### push_language {#push_language}

```gdscript
void push_language(language: String)
```

Adds language code used for text shaping algorithm and Open-Type font features.

### push_list {#push_list}

```gdscript
void push_list(level: int, type: RichTextLabel.ListType, capitalize: bool, bullet: String = "•")
```

Adds [ol] or [ul] tag to the tag stack. Multiplies level by current tab_size to determine new margin length.

### push_meta {#push_meta}

```gdscript
void push_meta(data: Variant, underline_mode: RichTextLabel.MetaUnderline = 1, tooltip: String = "")
```

Adds a meta tag to the tag stack. Similar to the BBCode {text}, but supports non-String metadata types.

If meta_underlined is true, meta tags display an underline. This behavior can be customized with underline_mode.

Note: Meta tags do nothing by default when clicked. To assign behavior when clicked, connect meta_clicked to a function that is called when the meta tag is clicked.

### push_mono {#push_mono}

```gdscript
void push_mono()
```

Adds a [font] tag with a monospace font to the tag stack.

### push_normal {#push_normal}

```gdscript
void push_normal()
```

Adds a [font] tag with a normal font to the tag stack.

### push_outline_color {#push_outline_color}

```gdscript
void push_outline_color(color: Color)
```

Adds a [outline_color] tag to the tag stack. Adds text outline for its duration.

### push_outline_size {#push_outline_size}

```gdscript
void push_outline_size(outline_size: int)
```

Adds a [outline_size] tag to the tag stack. Overrides default text outline size for its duration.

### push_paragraph {#push_paragraph}

```gdscript
void push_paragraph(alignment: HorizontalAlignment, base_direction: Control.TextDirection = 0, language: String = "", st_parser: TextServer.StructuredTextParser = 0, justification_flags: bitfield::TextServer.JustificationFlag = 163, tab_stops: PackedFloat32Array = PackedFloat32Array())
```

Adds a [p] tag to the tag stack.

### push_strikethrough {#push_strikethrough}

```gdscript
void push_strikethrough(color: Color = Color(0, 0, 0, 0))
```

Adds a [s] tag to the tag stack. If color's alpha value is 0.0, the current font's color with its alpha multiplied by strikethrough_alpha is used.

### push_table {#push_table}

```gdscript
void push_table(columns: int, inline_align: InlineAlignment = 0, align_to_row: int = -1, name: String = "")
```

Adds a [table=columns,inline_align] tag to the tag stack. Use set_table_column_expand() to set column expansion ratio. Use push_cell() to add cells. name is used as the table name for assistive apps.

### push_underline {#push_underline}

```gdscript
void push_underline(color: Color = Color(0, 0, 0, 0))
```

Adds a [u] tag to the tag stack. If color's alpha value is 0.0, the current font's color with its alpha multiplied by underline_alpha is used.

### reload_effects {#reload_effects}

```gdscript
void reload_effects()
```

Reloads custom effects. Useful when custom_effects is modified manually.

### remove_paragraph {#remove_paragraph}

```gdscript
bool remove_paragraph(paragraph: int, no_invalidate: bool = false)
```

Removes a paragraph of content from the label. Returns true if the paragraph exists.

The paragraph argument is the index of the paragraph to remove, it can take values in the interval [0, get_paragraph_count() - 1].

If no_invalidate is set to true, cache for the subsequent paragraphs is not invalidated. Use it for faster updates if deleted paragraph is fully self-contained (have no unclosed tags), or this call is part of the complex edit operation and invalidate_paragraph() will be called at the end of operation.

### scroll_to_line {#scroll_to_line}

```gdscript
void scroll_to_line(line: int)
```

Scrolls the window's top line to match line.

### scroll_to_paragraph {#scroll_to_paragraph}

```gdscript
void scroll_to_paragraph(paragraph: int)
```

Scrolls the window's top line to match first line of the paragraph.

### scroll_to_selection {#scroll_to_selection}

```gdscript
void scroll_to_selection()
```

Scrolls to the beginning of the current selection.

### select_all {#select_all}

```gdscript
void select_all()
```

Select all the text.

If selection_enabled is false, no selection will occur.

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

### set_cell_border_color {#set_cell_border_color}

```gdscript
void set_cell_border_color(color: Color)
```

Sets color of a table cell border.

### set_cell_padding {#set_cell_padding}

```gdscript
void set_cell_padding(padding: Rect2)
```

Sets inner padding of a table cell.

### set_cell_row_background_color {#set_cell_row_background_color}

```gdscript
void set_cell_row_background_color(odd_row_bg: Color, even_row_bg: Color)
```

Sets color of a table cell. Separate colors for alternating rows can be specified.

### set_cell_size_override {#set_cell_size_override}

```gdscript
void set_cell_size_override(min_size: Vector2, max_size: Vector2)
```

Sets minimum and maximum size overrides for a table cell.

### set_context_menu_enabled {#set_context_menu_enabled}

```gdscript
void set_context_menu_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deselect_on_focus_loss_enabled {#set_deselect_on_focus_loss_enabled}

```gdscript
void set_deselect_on_focus_loss_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_and_drop_selection_enabled {#set_drag_and_drop_selection_enabled}

```gdscript
void set_drag_and_drop_selection_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_effects {#set_effects}

```gdscript
void set_effects(effects: Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fit_content {#set_fit_content}

```gdscript
void set_fit_content(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hint_underline {#set_hint_underline}

```gdscript
void set_hint_underline(enable: bool)
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

### set_meta_underline {#set_meta_underline}

```gdscript
void set_meta_underline(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_progress_bar_delay {#set_progress_bar_delay}

```gdscript
void set_progress_bar_delay(delay_ms: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_active {#set_scroll_active}

```gdscript
void set_scroll_active(active: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_follow {#set_scroll_follow}

```gdscript
void set_scroll_follow(follow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_follow_visible_characters {#set_scroll_follow_visible_characters}

```gdscript
void set_scroll_follow_visible_characters(follow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selection_enabled {#set_selection_enabled}

```gdscript
void set_selection_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shortcut_keys_enabled {#set_shortcut_keys_enabled}

```gdscript
void set_shortcut_keys_enabled(enabled: bool)
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

### set_tab_size {#set_tab_size}

```gdscript
void set_tab_size(spaces: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_stops {#set_tab_stops}

```gdscript
void set_tab_stops(tab_stops: PackedFloat32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_table_column_expand {#set_table_column_expand}

```gdscript
void set_table_column_expand(column: int, expand: bool, ratio: int = 1, shrink: bool = true)
```

Edits the selected column's expansion options. If expand is true, the column expands in proportion to its expansion ratio versus the other columns' ratios.

For example, 2 columns with ratios 3 and 4 plus 70 pixels in available width would expand 30 and 40 pixels, respectively.

If expand is false, the column will not contribute to the total ratio.

### set_table_column_name {#set_table_column_name}

```gdscript
void set_table_column_name(column: int, name: String)
```

Sets table column name for assistive apps.

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

### set_threaded {#set_threaded}

```gdscript
void set_threaded(threaded: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_bbcode {#set_use_bbcode}

```gdscript
void set_use_bbcode(enable: bool)
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

### update_image {#update_image}

```gdscript
void update_image(key: Variant, mask: bitfield::RichTextLabel.ImageUpdateMask, image: Texture2D, width: float = 0, height: float = 0, color: Color = Color(1, 1, 1, 1), inline_align: InlineAlignment = 5, region: Rect2 = Rect2(0, 0, 0, 0), pad: bool = false, tooltip: String = "", width_unit: RichTextLabel.ImageUnit = 0, height_unit: RichTextLabel.ImageUnit = 0)
```

Updates the existing images with the key key. Only properties specified by mask bits are updated. See add_image().

## 属性

### bbcode_enabled {#bbcode_enabled}

```gdscript
bbcode_enabled: bool
```

- void set_use_bbcode(value: bool)
- bool is_using_bbcode()

If true, the label uses BBCode formatting.

Note: This only affects the contents of text, not the tag stack.

### text {#text}

```gdscript
text: String
```

- void set_text(value: String)
- String get_text()

The label's text in BBCode format. Is not representative of manual modifications to the internal tag stack. Erases changes made by other methods when edited.

Note: If bbcode_enabled is true, it is unadvised to use the += operator with text (e.g. text += "some string") as it replaces the whole text and can cause slowdowns. It will also erase all BBCode that was added to stack using push_* methods. Use append_text() for adding text instead, unless you absolutely need to close a tag that was opened in an earlier method call.

### fit_content {#fit_content}

```gdscript
fit_content: bool
```

- void set_fit_content(value: bool)
- bool is_fit_content_enabled()

If true, the label's minimum size will be automatically updated to fit its content, matching the behavior of Label.

Note: RichTextLabels with autowrapping and fit_content enabled must have a custom maximum width configured to work correctly, either through the RichTextLabel's own Control.custom_maximum_size or as a result of a propagated maximum size from a parent Control with Control.propagate_maximum_size enabled.

### scroll_active {#scroll_active}

```gdscript
scroll_active: bool
```

- void set_scroll_active(value: bool)
- bool is_scroll_active()

If true, the scrollbar is visible. Setting this to false does not block scrolling completely. See scroll_to_line().

### scroll_following {#scroll_following}

```gdscript
scroll_following: bool
```

- void set_scroll_follow(value: bool)
- bool is_scroll_following()

If true, the window scrolls down to display new content automatically.

### scroll_following_visible_characters {#scroll_following_visible_characters}

```gdscript
scroll_following_visible_characters: bool
```

- void set_scroll_follow_visible_characters(value: bool)
- bool is_scroll_following_visible_characters()

If true, the window scrolls to display the last visible line when visible_characters or visible_ratio is changed.

### autowrap_mode {#autowrap_mode}

```gdscript
autowrap_mode: int
```

- void set_autowrap_mode(value: AutowrapMode)
- AutowrapMode get_autowrap_mode()

If set to something other than TextServer.AUTOWRAP_OFF, the text gets wrapped inside the node's bounding rectangle.

Note: RichTextLabels with autowrapping and fit_content enabled must have a custom maximum width configured to work correctly, either through the RichTextLabel's own Control.custom_maximum_size or as a result of a propagated maximum size from a parent Control with Control.propagate_maximum_size enabled.

### autowrap_trim_flags {#autowrap_trim_flags}

```gdscript
autowrap_trim_flags: int
```

- void set_autowrap_trim_flags(value: BitField[LineBreakFlag])
- BitField[LineBreakFlag] get_autowrap_trim_flags()

Autowrap space trimming flags. See TextServer.BREAK_TRIM_START_EDGE_SPACES and TextServer.BREAK_TRIM_END_EDGE_SPACES for more info.

### tab_size {#tab_size}

```gdscript
tab_size: int
```

- void set_tab_size(value: int)
- int get_tab_size()

The number of spaces associated with a single tab length. Does not affect \t in text tags, only indent tags.

### context_menu_enabled {#context_menu_enabled}

```gdscript
context_menu_enabled: bool
```

- void set_context_menu_enabled(value: bool)
- bool is_context_menu_enabled()

If true, a right-click displays the context menu.

### shortcut_keys_enabled {#shortcut_keys_enabled}

```gdscript
shortcut_keys_enabled: bool
```

- void set_shortcut_keys_enabled(value: bool)
- bool is_shortcut_keys_enabled()

If true, shortcut keys for context menu items are enabled, even if the context menu is disabled.

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

### justification_flags {#justification_flags}

```gdscript
justification_flags: int
```

- void set_justification_flags(value: BitField[JustificationFlag])
- BitField[JustificationFlag] get_justification_flags()

Line fill alignment rules.

### tab_stops {#tab_stops}

```gdscript
tab_stops: PackedFloat32Array
```

- void set_tab_stops(value: PackedFloat32Array)
- PackedFloat32Array get_tab_stops()

Aligns text to the given tab-stops.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedFloat32Array for more details.

### custom_effects {#custom_effects}

```gdscript
custom_effects: typedarray::24/17:RichTextEffect
```

- void set_effects(value: Array)
- Array get_effects()

The currently installed custom effects. This is an array of RichTextEffects.

To add a custom effect, it's more convenient to use install_effect().

### meta_underlined {#meta_underlined}

```gdscript
meta_underlined: bool
```

- void set_meta_underline(value: bool)
- bool is_meta_underlined()

If true, the label underlines meta tags such as {text}. These tags can call a function when clicked if meta_clicked is connected to a function.

### hint_underlined {#hint_underlined}

```gdscript
hint_underlined: bool
```

- void set_hint_underline(value: bool)
- bool is_hint_underlined()

If true, the label underlines hint tags such as [hint=description]{text}[/hint].

### threaded {#threaded}

```gdscript
threaded: bool
```

- void set_threaded(value: bool)
- bool is_threaded()

If true, text processing is done in a background thread.

### progress_bar_delay {#progress_bar_delay}

```gdscript
progress_bar_delay: int
```

- void set_progress_bar_delay(value: int)
- int get_progress_bar_delay()

The delay after which the loading progress bar is displayed, in milliseconds. Set to -1 to disable progress bar entirely.

Note: Progress bar is displayed only if threaded is enabled.

### selection_enabled {#selection_enabled}

```gdscript
selection_enabled: bool
```

- void set_selection_enabled(value: bool)
- bool is_selection_enabled()

If true, the label allows text selection.

### deselect_on_focus_loss_enabled {#deselect_on_focus_loss_enabled}

```gdscript
deselect_on_focus_loss_enabled: bool
```

- void set_deselect_on_focus_loss_enabled(value: bool)
- bool is_deselect_on_focus_loss_enabled()

If true, the selected text will be deselected when focus is lost.

### drag_and_drop_selection_enabled {#drag_and_drop_selection_enabled}

```gdscript
drag_and_drop_selection_enabled: bool
```

- void set_drag_and_drop_selection_enabled(value: bool)
- bool is_drag_and_drop_selection_enabled()

If true, allow drag and drop of selected text.

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

## 信号

### meta_clicked {#meta_clicked}

```gdscript
signal meta_clicked(meta: Variant)
```

Triggered when the user clicks on content between meta (URL) tags. If the meta is defined in BBCode, e.g. Text, then the parameter for this signal will always be a String type. If a particular type or an object is desired, the push_meta() method must be used to manually insert the data into the tag stack. Alternatively, you can convert the String input to the desired type based on its contents (such as calling JSON.parse() on it).

For example, the following method can be connected to meta_clicked to open clicked URLs using the user's default web browser:

```
# This assumes RichTextLabel's `meta_clicked` signal was connected to
# the function below using the signal connection dialog.
func _richtextlabel_on_meta_clicked(meta):
    # `meta` is of Variant type, so convert it to a String to avoid script errors at run-time.
    OS.shell_open(str(meta))
```

### meta_hover_started {#meta_hover_started}

```gdscript
signal meta_hover_started(meta: Variant)
```

Triggers when the mouse enters a meta tag.

### meta_hover_ended {#meta_hover_ended}

```gdscript
signal meta_hover_ended(meta: Variant)
```

Triggers when the mouse exits a meta tag.

### finished {#finished}

```gdscript
signal finished()
```

Triggered when the document is fully loaded.

Note: This can happen before the text is processed for drawing. Scrolling values may not be valid until the document is drawn for the first time after this signal.

## 枚举

### ListType {#enum-ListType}

```gdscript
enum ListType { LIST_NUMBERS = 0, LIST_LETTERS = 1, LIST_ROMAN = 2, LIST_DOTS = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIST_NUMBERS {#LIST_NUMBERS}

```gdscript
LIST_NUMBERS = 0
```

Each list item has a number marker.

#### LIST_LETTERS {#LIST_LETTERS}

```gdscript
LIST_LETTERS = 1
```

Each list item has a letter marker.

#### LIST_ROMAN {#LIST_ROMAN}

```gdscript
LIST_ROMAN = 2
```

Each list item has a roman number marker.

#### LIST_DOTS {#LIST_DOTS}

```gdscript
LIST_DOTS = 3
```

Each list item has a filled circle marker.

---

### MenuItems {#enum-MenuItems}

```gdscript
enum MenuItems { MENU_COPY = 0, MENU_SELECT_ALL = 1, MENU_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_COPY {#MENU_COPY}

```gdscript
MENU_COPY = 0
```

Copies the selected text.

#### MENU_SELECT_ALL {#MENU_SELECT_ALL}

```gdscript
MENU_SELECT_ALL = 1
```

Selects the whole RichTextLabel text.

#### MENU_MAX {#MENU_MAX}

```gdscript
MENU_MAX = 2
```

Represents the size of the MenuItems enum.

---

### MetaUnderline {#enum-MetaUnderline}

```gdscript
enum MetaUnderline { META_UNDERLINE_NEVER = 0, META_UNDERLINE_ALWAYS = 1, META_UNDERLINE_ON_HOVER = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### META_UNDERLINE_NEVER {#META_UNDERLINE_NEVER}

```gdscript
META_UNDERLINE_NEVER = 0
```

Meta tag does not display an underline, even if meta_underlined is true.

#### META_UNDERLINE_ALWAYS {#META_UNDERLINE_ALWAYS}

```gdscript
META_UNDERLINE_ALWAYS = 1
```

If meta_underlined is true, meta tag always display an underline.

#### META_UNDERLINE_ON_HOVER {#META_UNDERLINE_ON_HOVER}

```gdscript
META_UNDERLINE_ON_HOVER = 2
```

If meta_underlined is true, meta tag display an underline when the mouse cursor is over it.

---

flags ImageUpdateMask:

### ImageUpdateMask {#enum-ImageUpdateMask}

```gdscript
enum ImageUpdateMask（位标志） { UPDATE_TEXTURE = 1, UPDATE_SIZE = 2, UPDATE_COLOR = 4, UPDATE_ALIGNMENT = 8, UPDATE_REGION = 16, UPDATE_PAD = 32, UPDATE_TOOLTIP = 64, UPDATE_WIDTH_UNIT = 128 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPDATE_TEXTURE {#UPDATE_TEXTURE}

```gdscript
UPDATE_TEXTURE = 1
```

If this bit is set, update_image() changes image texture.

#### UPDATE_SIZE {#UPDATE_SIZE}

```gdscript
UPDATE_SIZE = 2
```

If this bit is set, update_image() changes image size.

#### UPDATE_COLOR {#UPDATE_COLOR}

```gdscript
UPDATE_COLOR = 4
```

If this bit is set, update_image() changes image color.

#### UPDATE_ALIGNMENT {#UPDATE_ALIGNMENT}

```gdscript
UPDATE_ALIGNMENT = 8
```

If this bit is set, update_image() changes image inline alignment.

#### UPDATE_REGION {#UPDATE_REGION}

```gdscript
UPDATE_REGION = 16
```

If this bit is set, update_image() changes image texture region.

#### UPDATE_PAD {#UPDATE_PAD}

```gdscript
UPDATE_PAD = 32
```

If this bit is set, update_image() changes image padding.

#### UPDATE_TOOLTIP {#UPDATE_TOOLTIP}

```gdscript
UPDATE_TOOLTIP = 64
```

If this bit is set, update_image() changes image tooltip.

#### UPDATE_WIDTH_UNIT {#UPDATE_WIDTH_UNIT}

```gdscript
UPDATE_WIDTH_UNIT = 128
```

If this bit is set, update_image() changes the units used to calculate image size.

---

### ImageUnit {#enum-ImageUnit}

```gdscript
enum ImageUnit { IMAGE_UNIT_PIXEL = 0, IMAGE_UNIT_PERCENT = 1, IMAGE_UNIT_EM = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMAGE_UNIT_PIXEL {#IMAGE_UNIT_PIXEL}

```gdscript
IMAGE_UNIT_PIXEL = 0
```

Images drawn with this unit will be in pixels.

#### IMAGE_UNIT_PERCENT {#IMAGE_UNIT_PERCENT}

```gdscript
IMAGE_UNIT_PERCENT = 1
```

Images drawn with this unit will be in percentages of the control width.

#### IMAGE_UNIT_EM {#IMAGE_UNIT_EM}

```gdscript
IMAGE_UNIT_EM = 2
```

Images drawn with this unit will be in percentages of the surrounding font size.

---
