# TextEdit

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **TextEdit**

> A multiline text editor. It also has limited facilities for editing code, such as syntax highlighting support. For more advanced facilities for editing code, see CodeEdit.

<details><summary>官方英文描述（中文精译进行中）</summary>

A multiline text editor. It also has limited facilities for editing code, such as syntax highlighting support. For more advanced facilities for editing code, see CodeEdit.

While entering text, it is possible to insert special characters using Unicode, OEM or Windows alt codes:

- To enter Unicode codepoints, hold Alt and type the codepoint on the numpad. For example, to enter the character á (U+00E1), hold Alt and type +E1 on the numpad (the leading zeroes can be omitted).
- To enter OEM codepoints, hold Alt and type the code on the numpad. For example, to enter the character á (OEM 160), hold Alt and type 160 on the numpad.
- To enter Windows codepoints, hold Alt and type the code on the numpad. For example, to enter the character á (Windows 0225), hold Alt and type 0, 2, 2, 5 on the numpad. The leading zero here must not be omitted, as this is how Windows codepoints are distinguished from OEM codepoints.

Note: Most viewport, caret, and edit methods contain a caret_index argument for caret_multiple support. The argument should be one of the following: -1 for all carets, 0 for the main caret, or greater than 0 for secondary carets in the order they were created.

Note: When holding down Alt, the vertical scroll wheel will scroll 5 times as fast as it would normally do. This also works in the Godot script editor.

</details>

## 方法

### _backspace {#_backspace}

```gdscript
void _backspace(caret_index: int) virtual
```

Override this method to define what happens when the user presses the backspace key.

### _copy {#_copy}

```gdscript
void _copy(caret_index: int) virtual
```

Override this method to define what happens when the user performs a copy operation.

### _cut {#_cut}

```gdscript
void _cut(caret_index: int) virtual
```

Override this method to define what happens when the user performs a cut operation.

### _handle_unicode_input {#_handle_unicode_input}

```gdscript
void _handle_unicode_input(unicode_char: int, caret_index: int) virtual
```

Override this method to define what happens when the user types in the provided key unicode_char.

### _paste {#_paste}

```gdscript
void _paste(caret_index: int) virtual
```

Override this method to define what happens when the user performs a paste operation.

### _paste_primary_clipboard {#_paste_primary_clipboard}

```gdscript
void _paste_primary_clipboard(caret_index: int) virtual
```

Override this method to define what happens when the user performs a paste operation with middle mouse button.

Note: This method is only implemented on Linux.

### add_caret {#add_caret}

```gdscript
int add_caret(line: int, column: int)
```

Adds a new caret at the given location. Returns the index of the new caret, or -1 if the location is invalid.

### add_caret_at_carets {#add_caret_at_carets}

```gdscript
void add_caret_at_carets(below: bool)
```

Adds an additional caret above or below every caret. If below is true the new caret will be added below and above otherwise.

### add_gutter {#add_gutter}

```gdscript
void add_gutter(at: int = -1)
```

Register a new gutter to this TextEdit. Use at to have a specific gutter order. A value of -1 appends the gutter to the right.

### add_selection_for_next_occurrence {#add_selection_for_next_occurrence}

```gdscript
void add_selection_for_next_occurrence()
```

Adds a selection and a caret for the next occurrence of the current selection. If there is no active selection, selects word under caret.

### adjust_carets_after_edit {#adjust_carets_after_edit}

```gdscript
void adjust_carets_after_edit(caret: int, from_line: int, from_col: int, to_line: int, to_col: int)
```

Deprecated: No longer necessary since methods now adjust carets themselves.

This method does nothing.

### adjust_viewport_to_caret {#adjust_viewport_to_caret}

```gdscript
void adjust_viewport_to_caret(caret_index: int = 0)
```

Adjust the viewport so the caret is visible.

### apply_ime {#apply_ime}

```gdscript
void apply_ime()
```

Applies text from the Input Method Editor [https://en.wikipedia.org/wiki/Input_method] (IME) to each caret and closes the IME if it is open.

### backspace {#backspace}

```gdscript
void backspace(caret_index: int = -1)
```

Called when the user presses the backspace key. Can be overridden with _backspace().

### begin_complex_operation {#begin_complex_operation}

```gdscript
void begin_complex_operation()
```

Starts a multipart edit. All edits will be treated as one action until end_complex_operation() is called.

### begin_multicaret_edit {#begin_multicaret_edit}

```gdscript
void begin_multicaret_edit()
```

Starts an edit for multiple carets. The edit must be ended with end_multicaret_edit(). Multicaret edits can be used to edit text at multiple carets and delay merging the carets until the end, so the caret indexes aren't affected immediately. begin_multicaret_edit() and end_multicaret_edit() can be nested, and the merge will happen at the last end_multicaret_edit().

```gdscript
begin_complex_operation()
begin_multicaret_edit()
for i in range(get_caret_count()):
    if multicaret_edit_ignore_caret(i):
        continue
    # Logic here.
end_multicaret_edit()
end_complex_operation()
```

### cancel_ime {#cancel_ime}

```gdscript
void cancel_ime()
```

Closes the Input Method Editor [https://en.wikipedia.org/wiki/Input_method] (IME) if it is open. Any text in the IME will be lost.

### center_viewport_to_caret {#center_viewport_to_caret}

```gdscript
void center_viewport_to_caret(caret_index: int = 0)
```

Centers the viewport on the line the editing caret is at. This also resets the scroll_horizontal value to 0.

### clear {#clear}

```gdscript
void clear()
```

Performs a full reset of TextEdit, including undo history.

### clear_undo_history {#clear_undo_history}

```gdscript
void clear_undo_history()
```

Clears the undo history.

### collapse_carets {#collapse_carets}

```gdscript
void collapse_carets(from_line: int, from_column: int, to_line: int, to_column: int, inclusive: bool = false)
```

Collapse all carets in the given range to the from_line and from_column position.

inclusive applies to both ends.

If is_in_mulitcaret_edit() is true, carets that are collapsed will be true for multicaret_edit_ignore_caret().

merge_overlapping_carets() will be called if any carets were collapsed.

### copy {#copy}

```gdscript
void copy(caret_index: int = -1)
```

Copies the current text selection. Can be overridden with _copy().

### cut {#cut}

```gdscript
void cut(caret_index: int = -1)
```

Cut's the current selection. Can be overridden with _cut().

### delete_selection {#delete_selection}

```gdscript
void delete_selection(caret_index: int = -1)
```

Deletes the selected text.

### deselect {#deselect}

```gdscript
void deselect(caret_index: int = -1)
```

Deselects the current selection.

### end_action {#end_action}

```gdscript
void end_action()
```

Marks the end of steps in the current action started with start_action().

### end_complex_operation {#end_complex_operation}

```gdscript
void end_complex_operation()
```

Ends a multipart edit, started with begin_complex_operation(). If called outside a complex operation, the current operation is pushed onto the undo/redo stack.

### end_multicaret_edit {#end_multicaret_edit}

```gdscript
void end_multicaret_edit()
```

Ends an edit for multiple carets, that was started with begin_multicaret_edit(). If this was the last end_multicaret_edit() and merge_overlapping_carets() was called, carets will be merged.

### get_autowrap_mode {#get_autowrap_mode}

```gdscript
TextServer.AutowrapMode get_autowrap_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_caret_blink_interval {#get_caret_blink_interval}

```gdscript
float get_caret_blink_interval() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_caret_column {#get_caret_column}

```gdscript
int get_caret_column(caret_index: int = 0) const
```

Returns the column the editing caret is at.

### get_caret_count {#get_caret_count}

```gdscript
int get_caret_count() const
```

Returns the number of carets in this TextEdit.

### get_caret_draw_pos {#get_caret_draw_pos}

```gdscript
Vector2 get_caret_draw_pos(caret_index: int = 0) const
```

Returns the caret pixel draw position.

### get_caret_index_edit_order {#get_caret_index_edit_order}

```gdscript
PackedInt32Array get_caret_index_edit_order()
```

Deprecated: Carets no longer need to be edited in any specific order. If the carets need to be sorted, use get_sorted_carets() instead.

Returns a list of caret indexes in their edit order, this done from bottom to top. Edit order refers to the way actions such as insert_text_at_caret() are applied.

### get_caret_line {#get_caret_line}

```gdscript
int get_caret_line(caret_index: int = 0) const
```

Returns the line the editing caret is on.

### get_caret_type {#get_caret_type}

```gdscript
TextEdit.CaretType get_caret_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_caret_wrap_index {#get_caret_wrap_index}

```gdscript
int get_caret_wrap_index(caret_index: int = 0) const
```

Returns the wrap index the editing caret is on.

### get_custom_word_separators {#get_custom_word_separators}

```gdscript
String get_custom_word_separators() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_draw_control_chars {#get_draw_control_chars}

```gdscript
bool get_draw_control_chars() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_first_non_whitespace_column {#get_first_non_whitespace_column}

```gdscript
int get_first_non_whitespace_column(line: int) const
```

Returns the first column containing a non-whitespace character on the given line. If there is only whitespace, returns the number of characters.

### get_first_visible_line {#get_first_visible_line}

```gdscript
int get_first_visible_line() const
```

Returns the first visible line.

### get_gutter_count {#get_gutter_count}

```gdscript
int get_gutter_count() const
```

Returns the number of gutters registered.

### get_gutter_name {#get_gutter_name}

```gdscript
String get_gutter_name(gutter: int) const
```

Returns the name of the gutter at the given index.

### get_gutter_type {#get_gutter_type}

```gdscript
TextEdit.GutterType get_gutter_type(gutter: int) const
```

Returns the type of the gutter at the given index. Gutters can contain icons, text, or custom visuals.

### get_gutter_width {#get_gutter_width}

```gdscript
int get_gutter_width(gutter: int) const
```

Returns the width of the gutter at the given index.

### get_h_scroll {#get_h_scroll}

```gdscript
int get_h_scroll() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_h_scroll_bar {#get_h_scroll_bar}

```gdscript
HScrollBar get_h_scroll_bar() const
```

Returns the HScrollBar used by TextEdit.

### get_indent_level {#get_indent_level}

```gdscript
int get_indent_level(line: int) const
```

Returns the indent level of the given line. This is the number of spaces and tabs at the beginning of the line, with the tabs taking the tab size into account (see get_tab_size()).

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_last_full_visible_line {#get_last_full_visible_line}

```gdscript
int get_last_full_visible_line() const
```

Returns the last visible line. Use get_last_full_visible_line_wrap_index() for the wrap index.

### get_last_full_visible_line_wrap_index {#get_last_full_visible_line_wrap_index}

```gdscript
int get_last_full_visible_line_wrap_index() const
```

Returns the last visible wrap index of the last visible line.

### get_last_unhidden_line {#get_last_unhidden_line}

```gdscript
int get_last_unhidden_line() const
```

Returns the last unhidden line in the entire TextEdit.

### get_line {#get_line}

```gdscript
String get_line(line: int) const
```

Returns the text of a specific line.

### get_line_background_color {#get_line_background_color}

```gdscript
Color get_line_background_color(line: int) const
```

Returns the custom background color of the given line. If no color is set, returns Color(0, 0, 0, 0).

### get_line_column_at_pos {#get_line_column_at_pos}

```gdscript
Vector2i get_line_column_at_pos(position: Vector2i, clamp_line: bool = true, clamp_column: bool = true) const
```

Returns the line and column at the given position. In the returned vector, x is the column and y is the line.

If clamp_line is false and position is below the last line, Vector2i(-1, -1) is returned.

If clamp_column is false and position is outside the column range of the line, Vector2i(-1, -1) is returned.

### get_line_count {#get_line_count}

```gdscript
int get_line_count() const
```

Returns the number of lines in the text.

### get_line_gutter_icon {#get_line_gutter_icon}

```gdscript
Texture2D get_line_gutter_icon(line: int, gutter: int) const
```

Returns the icon currently in gutter at line. This only works when the gutter type is GUTTER_TYPE_ICON (see set_gutter_type()).

### get_line_gutter_item_color {#get_line_gutter_item_color}

```gdscript
Color get_line_gutter_item_color(line: int, gutter: int) const
```

Returns the color currently in gutter at line.

### get_line_gutter_metadata {#get_line_gutter_metadata}

```gdscript
Variant get_line_gutter_metadata(line: int, gutter: int) const
```

Returns the metadata currently in gutter at line.

### get_line_gutter_text {#get_line_gutter_text}

```gdscript
String get_line_gutter_text(line: int, gutter: int) const
```

Returns the text currently in gutter at line. This only works when the gutter type is GUTTER_TYPE_STRING (see set_gutter_type()).

### get_line_height {#get_line_height}

```gdscript
int get_line_height() const
```

Returns the maximum value of the line height among all lines.

Note: The return value is influenced by line_spacing and font_size. And it will not be less than 1.

### get_line_ranges_from_carets {#get_line_ranges_from_carets}

```gdscript
typedarray::Vector2i get_line_ranges_from_carets(only_selections: bool = false, merge_adjacent: bool = true) const
```

Returns an Array of line ranges where x is the first line and y is the last line. All lines within these ranges will have a caret on them or be part of a selection. Each line will only be part of one line range, even if it has multiple carets on it.

If a selection's end column (get_selection_to_column()) is at column 0, that line will not be included. If a selection begins on the line after another selection ends and merge_adjacent is true, or they begin and end on the same line, one line range will include both selections.

### get_line_width {#get_line_width}

```gdscript
int get_line_width(line: int, wrap_index: int = -1) const
```

Returns the width in pixels of the wrap_index on line.

### get_line_with_ime {#get_line_with_ime}

```gdscript
String get_line_with_ime(line: int) const
```

Returns line text as it is currently displayed, including IME composition string.

### get_line_wrap_count {#get_line_wrap_count}

```gdscript
int get_line_wrap_count(line: int) const
```

Returns the number of times the given line is wrapped.

### get_line_wrap_index_at_column {#get_line_wrap_index_at_column}

```gdscript
int get_line_wrap_index_at_column(line: int, column: int) const
```

Returns the wrap index of the given column on the given line. This ranges from 0 to get_line_wrap_count().

### get_line_wrapped_text {#get_line_wrapped_text}

```gdscript
PackedStringArray get_line_wrapped_text(line: int) const
```

Returns an array of Strings representing each wrapped index.

### get_line_wrapping_mode {#get_line_wrapping_mode}

```gdscript
TextEdit.LineWrappingMode get_line_wrapping_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_local_mouse_pos {#get_local_mouse_pos}

```gdscript
Vector2 get_local_mouse_pos() const
```

Returns the local mouse position adjusted for the text direction.

### get_menu {#get_menu}

```gdscript
PopupMenu get_menu() const
```

Returns the PopupMenu of this TextEdit. By default, this menu is displayed when right-clicking on the TextEdit.

You can add custom menu items or remove standard ones. Make sure your IDs don't conflict with the standard ones (see MenuItems). For example:

```
func _ready():
    var menu = get_menu()
    # Remove all items after "Redo".
    menu.item_count = menu.get_item_index(MENU_REDO) + 1
    # Add custom items.
    menu.add_separator()
    menu.add_item("Insert Date", MENU_MAX + 1)
    # Connect callback.
    menu.id_pressed.connect(_on_item_pressed)
func _on_item_pressed(id):
    if id == MENU_MAX + 1:
        insert_text_at_caret(Time.get_date_string_from_system())
```

```
public override void _Ready()
{
    var menu = GetMenu();
    // Remove all items after "Redo".
    menu.ItemCount = menu.GetItemIndex(TextEdit.MenuItems.Redo) + 1;
    // Add custom items.
    menu.AddSeparator();
    menu.AddItem("Insert Date", TextEdit.MenuItems.Max + 1);
    // Add event handler.
    menu.IdPressed += OnItemPressed;
}
public void OnItemPressed(int id)
{
    if (id == TextEdit.MenuItems.Max + 1)
    {
        InsertTextAtCaret(Time.GetDateStringFromSystem());
    }
}
```

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their Window.visible property.

### get_minimap_line_at_pos {#get_minimap_line_at_pos}

```gdscript
int get_minimap_line_at_pos(position: Vector2i) const
```

Returns the equivalent minimap line at position.

### get_minimap_visible_lines {#get_minimap_visible_lines}

```gdscript
int get_minimap_visible_lines() const
```

Returns the number of lines that may be drawn on the minimap.

### get_minimap_width {#get_minimap_width}

```gdscript
int get_minimap_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_next_composite_character_column {#get_next_composite_character_column}

```gdscript
int get_next_composite_character_column(line: int, column: int) const
```

Returns the correct column at the end of a composite character like ❤️‍🩹 (mending heart; Unicode: U+2764 U+FE0F U+200D U+1FA79) which is comprised of more than one Unicode code point, if the caret is at the start of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.

Note: To check at caret location use get_next_composite_character_column(get_caret_line(), get_caret_column())

### get_next_visible_line_index_offset_from {#get_next_visible_line_index_offset_from}

```gdscript
Vector2i get_next_visible_line_index_offset_from(line: int, wrap_index: int, visible_amount: int) const
```

Similar to get_next_visible_line_offset_from(), but takes into account the line wrap indexes. In the returned vector, x is the line, y is the wrap index.

### get_next_visible_line_offset_from {#get_next_visible_line_offset_from}

```gdscript
int get_next_visible_line_offset_from(line: int, visible_amount: int) const
```

Returns the count to the next visible line from line to line + visible_amount. Can also count backwards. For example if a TextEdit has 5 lines with lines 2 and 3 hidden, calling this with line = 1, visible_amount = 1 would return 3.

### get_placeholder {#get_placeholder}

```gdscript
String get_placeholder() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pos_at_line_column {#get_pos_at_line_column}

```gdscript
Vector2i get_pos_at_line_column(line: int, column: int) const
```

Returns the local position for the given line and column. If x or y of the returned vector equal -1, the position is outside of the viewable area of the control.

Note: The Y position corresponds to the bottom side of the line. Use get_rect_at_line_column() to get the top side position.

### get_previous_composite_character_column {#get_previous_composite_character_column}

```gdscript
int get_previous_composite_character_column(line: int, column: int) const
```

Returns the correct column at the start of a composite character like ❤️‍🩹 (mending heart; Unicode: U+2764 U+FE0F U+200D U+1FA79) which is comprised of more than one Unicode code point, if the caret is at the end of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.

Note: To check at caret location use get_previous_composite_character_column(get_caret_line(), get_caret_column())

### get_rect_at_line_column {#get_rect_at_line_column}

```gdscript
Rect2i get_rect_at_line_column(line: int, column: int) const
```

Returns the local position and size for the grapheme at the given line and column. If x or y position of the returned rect equal -1, the position is outside of the viewable area of the control.

Note: The Y position of the returned rect corresponds to the top side of the line, unlike get_pos_at_line_column() which returns the bottom side.

### get_saved_version {#get_saved_version}

```gdscript
int get_saved_version() const
```

Returns the last tagged saved version from tag_saved_version().

### get_scroll_pos_for_line {#get_scroll_pos_for_line}

```gdscript
float get_scroll_pos_for_line(line: int, wrap_index: int = 0) const
```

Returns the scroll position for wrap_index of line.

### get_selected_text {#get_selected_text}

```gdscript
String get_selected_text(caret_index: int = -1)
```

Returns the text inside the selection of a caret, or all the carets if caret_index is its default value -1.

### get_selection_at_line_column {#get_selection_at_line_column}

```gdscript
int get_selection_at_line_column(line: int, column: int, include_edges: bool = true, only_selections: bool = true) const
```

Returns the caret index of the selection at the given line and column, or -1 if there is none.

If include_edges is false, the position must be inside the selection and not at either end. If only_selections is false, carets without a selection will also be considered.

### get_selection_column {#get_selection_column}

```gdscript
int get_selection_column(caret_index: int = 0) const
```

Deprecated: Use get_selection_origin_column() instead.

Returns the original start column of the selection.

### get_selection_from_column {#get_selection_from_column}

```gdscript
int get_selection_from_column(caret_index: int = 0) const
```

Returns the selection begin column. Returns the caret column if there is no selection.

### get_selection_from_line {#get_selection_from_line}

```gdscript
int get_selection_from_line(caret_index: int = 0) const
```

Returns the selection begin line. Returns the caret line if there is no selection.

### get_selection_line {#get_selection_line}

```gdscript
int get_selection_line(caret_index: int = 0) const
```

Deprecated: Use get_selection_origin_line() instead.

Returns the original start line of the selection.

### get_selection_mode {#get_selection_mode}

```gdscript
TextEdit.SelectionMode get_selection_mode() const
```

Returns the current selection mode.

### get_selection_origin_column {#get_selection_origin_column}

```gdscript
int get_selection_origin_column(caret_index: int = 0) const
```

Returns the origin column of the selection. This is the opposite end from the caret.

### get_selection_origin_line {#get_selection_origin_line}

```gdscript
int get_selection_origin_line(caret_index: int = 0) const
```

Returns the origin line of the selection. This is the opposite end from the caret.

### get_selection_to_column {#get_selection_to_column}

```gdscript
int get_selection_to_column(caret_index: int = 0) const
```

Returns the selection end column. Returns the caret column if there is no selection.

### get_selection_to_line {#get_selection_to_line}

```gdscript
int get_selection_to_line(caret_index: int = 0) const
```

Returns the selection end line. Returns the caret line if there is no selection.

### get_sorted_carets {#get_sorted_carets}

```gdscript
PackedInt32Array get_sorted_carets(include_ignored_carets: bool = false) const
```

Returns the carets sorted by selection beginning from lowest line and column to highest (from top to bottom of text).

If include_ignored_carets is false, carets from multicaret_edit_ignore_caret() will be ignored.

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

### get_syntax_highlighter {#get_syntax_highlighter}

```gdscript
SyntaxHighlighter get_syntax_highlighter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tab_input_mode {#get_tab_input_mode}

```gdscript
bool get_tab_input_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tab_size {#get_tab_size}

```gdscript
int get_tab_size() const
```

Returns the TextEdit's' tab size.

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

### get_total_gutter_width {#get_total_gutter_width}

```gdscript
int get_total_gutter_width() const
```

Returns the total width of all gutters and internal padding.

### get_total_visible_line_count {#get_total_visible_line_count}

```gdscript
int get_total_visible_line_count() const
```

Returns the total number of lines in the text. This includes wrapped lines and excludes folded lines. If wrap_mode is set to LINE_WRAPPING_NONE and no lines are folded (see CodeEdit.is_line_folded()) then this is equivalent to get_line_count(). See get_visible_line_count_in_range() for a limited range of lines.

### get_v_scroll {#get_v_scroll}

```gdscript
float get_v_scroll() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_v_scroll_bar {#get_v_scroll_bar}

```gdscript
VScrollBar get_v_scroll_bar() const
```

Returns the VScrollBar of the TextEdit.

### get_v_scroll_speed {#get_v_scroll_speed}

```gdscript
float get_v_scroll_speed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_version {#get_version}

```gdscript
int get_version() const
```

Returns the current version of the TextEdit. The version is a count of recorded operations by the undo/redo history.

### get_virtual_keyboard_show_on_focus {#get_virtual_keyboard_show_on_focus}

```gdscript
bool get_virtual_keyboard_show_on_focus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visible_line_count {#get_visible_line_count}

```gdscript
int get_visible_line_count() const
```

Returns the number of lines that can visually fit, rounded down, based on this control's height.

### get_visible_line_count_in_range {#get_visible_line_count_in_range}

```gdscript
int get_visible_line_count_in_range(from_line: int, to_line: int) const
```

Returns the total number of lines between from_line and to_line (inclusive) in the text. This includes wrapped lines and excludes folded lines. If the range covers all lines it is equivalent to get_total_visible_line_count().

### get_word_at_pos {#get_word_at_pos}

```gdscript
String get_word_at_pos(position: Vector2) const
```

Returns the word at position.

### get_word_under_caret {#get_word_under_caret}

```gdscript
String get_word_under_caret(caret_index: int = -1) const
```

Returns a String text with the word under the caret's location.

### has_ime_text {#has_ime_text}

```gdscript
bool has_ime_text() const
```

Returns true if the user has text in the Input Method Editor [https://en.wikipedia.org/wiki/Input_method] (IME).

### has_redo {#has_redo}

```gdscript
bool has_redo() const
```

Returns true if a "redo" action is available.

### has_selection {#has_selection}

```gdscript
bool has_selection(caret_index: int = -1) const
```

Returns true if the user has selected text.

### has_undo {#has_undo}

```gdscript
bool has_undo() const
```

Returns true if an "undo" action is available.

### insert_line_at {#insert_line_at}

```gdscript
void insert_line_at(line: int, text: String)
```

Inserts a new line with text at line.

### insert_text {#insert_text}

```gdscript
void insert_text(text: String, line: int, column: int, before_selection_begin: bool = true, before_selection_end: bool = false)
```

Inserts the text at line and column.

If before_selection_begin is true, carets and selections that begin at line and column will moved to the end of the inserted text, along with all carets after it.

If before_selection_end is true, selections that end at line and column will be extended to the end of the inserted text. These parameters can be used to insert text inside of or outside of selections.

### insert_text_at_caret {#insert_text_at_caret}

```gdscript
void insert_text_at_caret(text: String, caret_index: int = -1)
```

Insert the specified text at the caret position.

### is_backspace_deletes_composite_character_enabled {#is_backspace_deletes_composite_character_enabled}

```gdscript
bool is_backspace_deletes_composite_character_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_caret_after_selection_origin {#is_caret_after_selection_origin}

```gdscript
bool is_caret_after_selection_origin(caret_index: int = 0) const
```

Returns true if the caret of the selection is after the selection origin. This can be used to determine the direction of the selection.

### is_caret_blink_enabled {#is_caret_blink_enabled}

```gdscript
bool is_caret_blink_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_caret_mid_grapheme_enabled {#is_caret_mid_grapheme_enabled}

```gdscript
bool is_caret_mid_grapheme_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_caret_visible {#is_caret_visible}

```gdscript
bool is_caret_visible(caret_index: int = 0) const
```

Returns true if the caret is visible, false otherwise. A caret will be considered hidden if it is outside the scrollable area when scrolling is enabled.

Note: is_caret_visible() does not account for a caret being off-screen if it is still within the scrollable area. It will return true even if the caret is off-screen as long as it meets TextEdit's own conditions for being visible. This includes uses of scroll_fit_content_width and scroll_fit_content_height that cause the TextEdit to expand beyond the viewport's bounds.

Note: This method does not guarantee an accurate visibility check immediately after setting the caret position. The correct value may only be available in the next frame after the TextEdit has finished drawing. This also applies to any operation that causes the TextEdit to change in size.

### is_context_menu_enabled {#is_context_menu_enabled}

```gdscript
bool is_context_menu_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_custom_word_separators_enabled {#is_custom_word_separators_enabled}

```gdscript
bool is_custom_word_separators_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_default_word_separators_enabled {#is_default_word_separators_enabled}

```gdscript
bool is_default_word_separators_enabled() const
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

### is_dragging_cursor {#is_dragging_cursor}

```gdscript
bool is_dragging_cursor() const
```

Returns true if the user is dragging their mouse for scrolling, selecting, or text dragging.

### is_drawing_caret_when_editable_disabled {#is_drawing_caret_when_editable_disabled}

```gdscript
bool is_drawing_caret_when_editable_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drawing_minimap {#is_drawing_minimap}

```gdscript
bool is_drawing_minimap() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drawing_spaces {#is_drawing_spaces}

```gdscript
bool is_drawing_spaces() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drawing_tabs {#is_drawing_tabs}

```gdscript
bool is_drawing_tabs() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editable {#is_editable}

```gdscript
bool is_editable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_emoji_menu_enabled {#is_emoji_menu_enabled}

```gdscript
bool is_emoji_menu_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_empty_selection_clipboard_enabled {#is_empty_selection_clipboard_enabled}

```gdscript
bool is_empty_selection_clipboard_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_fit_content_height_enabled {#is_fit_content_height_enabled}

```gdscript
bool is_fit_content_height_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_fit_content_width_enabled {#is_fit_content_width_enabled}

```gdscript
bool is_fit_content_width_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_gutter_clickable {#is_gutter_clickable}

```gdscript
bool is_gutter_clickable(gutter: int) const
```

Returns true if the gutter at the given index is clickable. See set_gutter_clickable().

### is_gutter_drawn {#is_gutter_drawn}

```gdscript
bool is_gutter_drawn(gutter: int) const
```

Returns true if the gutter at the given index is currently drawn. See set_gutter_draw().

### is_gutter_overwritable {#is_gutter_overwritable}

```gdscript
bool is_gutter_overwritable(gutter: int) const
```

Returns true if the gutter at the given index is overwritable. See set_gutter_overwritable().

### is_highlight_all_occurrences_enabled {#is_highlight_all_occurrences_enabled}

```gdscript
bool is_highlight_all_occurrences_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_highlight_current_line_enabled {#is_highlight_current_line_enabled}

```gdscript
bool is_highlight_current_line_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_in_mulitcaret_edit {#is_in_mulitcaret_edit}

```gdscript
bool is_in_mulitcaret_edit() const
```

Returns true if a begin_multicaret_edit() has been called and end_multicaret_edit() has not yet been called.

### is_indent_wrapped_lines {#is_indent_wrapped_lines}

```gdscript
bool is_indent_wrapped_lines() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_line_gutter_clickable {#is_line_gutter_clickable}

```gdscript
bool is_line_gutter_clickable(line: int, gutter: int) const
```

Returns true if the gutter at the given index on the given line is clickable. See set_line_gutter_clickable().

### is_line_in_viewport {#is_line_in_viewport}

```gdscript
bool is_line_in_viewport(line: int) const
```

Returns true if the given line is within the scope of the scrollable area of the viewport.

### is_line_wrapped {#is_line_wrapped}

```gdscript
bool is_line_wrapped(line: int) const
```

Returns if the given line is wrapped.

### is_menu_visible {#is_menu_visible}

```gdscript
bool is_menu_visible() const
```

Returns true if the menu is visible. Use this instead of get_menu().visible to improve performance (so the creation of the menu is avoided). See get_menu().

### is_middle_mouse_paste_enabled {#is_middle_mouse_paste_enabled}

```gdscript
bool is_middle_mouse_paste_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_mouse_over_selection {#is_mouse_over_selection}

```gdscript
bool is_mouse_over_selection(edges: bool, caret_index: int = -1) const
```

Returns true if the mouse is over a selection. If edges is true, the edges are considered part of the selection.

### is_move_caret_on_right_click_enabled {#is_move_caret_on_right_click_enabled}

```gdscript
bool is_move_caret_on_right_click_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_multiple_carets_enabled {#is_multiple_carets_enabled}

```gdscript
bool is_multiple_carets_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_overtype_mode_enabled {#is_overtype_mode_enabled}

```gdscript
bool is_overtype_mode_enabled() const
```

Returns true if overtype mode is enabled. See set_overtype_mode_enabled().

### is_scroll_past_end_of_file_enabled {#is_scroll_past_end_of_file_enabled}

```gdscript
bool is_scroll_past_end_of_file_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_selecting_enabled {#is_selecting_enabled}

```gdscript
bool is_selecting_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_shortcut_keys_enabled {#is_shortcut_keys_enabled}

```gdscript
bool is_shortcut_keys_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_smooth_scroll_enabled {#is_smooth_scroll_enabled}

```gdscript
bool is_smooth_scroll_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_virtual_keyboard_enabled {#is_virtual_keyboard_enabled}

```gdscript
bool is_virtual_keyboard_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### menu_option {#menu_option}

```gdscript
void menu_option(option: int)
```

Executes a given action as defined in the MenuItems enum.

### merge_gutters {#merge_gutters}

```gdscript
void merge_gutters(from_line: int, to_line: int)
```

Merge the gutters from from_line into to_line. Only overwritable gutters will be copied. See set_gutter_overwritable().

### merge_overlapping_carets {#merge_overlapping_carets}

```gdscript
void merge_overlapping_carets()
```

Merges any overlapping carets. Will favor the newest caret, or the caret with a selection.

If is_in_mulitcaret_edit() is true, the merge will be queued to happen at the end of the multicaret edit. See begin_multicaret_edit() and end_multicaret_edit().

Note: This is not called when a caret changes position but after certain actions, so it is possible to get into a state where carets overlap.

### multicaret_edit_ignore_caret {#multicaret_edit_ignore_caret}

```gdscript
bool multicaret_edit_ignore_caret(caret_index: int) const
```

Returns true if the given caret_index should be ignored as part of a multicaret edit. See begin_multicaret_edit() and end_multicaret_edit(). Carets that should be ignored are ones that were part of removed text and will likely be merged at the end of the edit, or carets that were added during the edit.

It is recommended to continue within a loop iterating on multiple carets if a caret should be ignored.

### paste {#paste}

```gdscript
void paste(caret_index: int = -1)
```

Paste at the current location. Can be overridden with _paste().

### paste_primary_clipboard {#paste_primary_clipboard}

```gdscript
void paste_primary_clipboard(caret_index: int = -1)
```

Pastes the primary clipboard.

### redo {#redo}

```gdscript
void redo()
```

Perform redo operation.

### remove_caret {#remove_caret}

```gdscript
void remove_caret(caret: int)
```

Removes the given caret index.

Note: This can result in adjustment of all other caret indices.

### remove_gutter {#remove_gutter}

```gdscript
void remove_gutter(gutter: int)
```

Removes the gutter at the given index.

### remove_line_at {#remove_line_at}

```gdscript
void remove_line_at(line: int, move_carets_down: bool = true)
```

Removes the line of text at line. Carets on this line will attempt to match their previous visual x position.

If move_carets_down is true carets will move to the next line down, otherwise carets will move up.

### remove_secondary_carets {#remove_secondary_carets}

```gdscript
void remove_secondary_carets()
```

Removes all additional carets.

### remove_text {#remove_text}

```gdscript
void remove_text(from_line: int, from_column: int, to_line: int, to_column: int)
```

Removes text between the given positions.

### search {#search}

```gdscript
Vector2i search(text: String, flags: int, from_line: int, from_column: int) const
```

Perform a search inside the text. Search flags can be specified in the SearchFlags enum.

In the returned vector, x is the column, y is the line. If no results are found, both are equal to -1.

```
var result = search("print", SEARCH_WHOLE_WORDS, 0, 0)
if result.x != -1:
    # Result found.
    var line_number = result.y
    var column_number = result.x
```

```
Vector2I result = Search("print", (uint)TextEdit.SearchFlags.WholeWords, 0, 0);
if (result.X != -1)
{
    // Result found.
    int lineNumber = result.Y;
    int columnNumber = result.X;
}
```

### select {#select}

```gdscript
void select(origin_line: int, origin_column: int, caret_line: int, caret_column: int, caret_index: int = 0)
```

Selects text from origin_line and origin_column to caret_line and caret_column for the given caret_index. This moves the selection origin and the caret. If the positions are the same, the selection will be deselected.

If selecting_enabled is false, no selection will occur.

Note: If supporting multiple carets this will not check for any overlap. See merge_overlapping_carets().

### select_all {#select_all}

```gdscript
void select_all()
```

Select all the text.

If selecting_enabled is false, no selection will occur.

### select_word_under_caret {#select_word_under_caret}

```gdscript
void select_word_under_caret(caret_index: int = -1)
```

Selects the word under the caret.

### set_autowrap_mode {#set_autowrap_mode}

```gdscript
void set_autowrap_mode(autowrap_mode: TextServer.AutowrapMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_backspace_deletes_composite_character_enabled {#set_backspace_deletes_composite_character_enabled}

```gdscript
void set_backspace_deletes_composite_character_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_blink_enabled {#set_caret_blink_enabled}

```gdscript
void set_caret_blink_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_blink_interval {#set_caret_blink_interval}

```gdscript
void set_caret_blink_interval(interval: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_column {#set_caret_column}

```gdscript
void set_caret_column(column: int, adjust_viewport: bool = true, caret_index: int = 0)
```

Moves the caret to the specified column index.

If adjust_viewport is true, the viewport will center at the caret position after the move occurs.

Note: If supporting multiple carets this will not check for any overlap. See merge_overlapping_carets().

### set_caret_line {#set_caret_line}

```gdscript
void set_caret_line(line: int, adjust_viewport: bool = true, can_be_hidden: bool = true, wrap_index: int = 0, caret_index: int = 0)
```

Moves the caret to the specified line index. The caret column will be moved to the same visual position it was at the last time set_caret_column() was called, or clamped to the end of the line.

If adjust_viewport is true, the viewport will center at the caret position after the move occurs.

If can_be_hidden is true, the specified line can be hidden.

If wrap_index is -1, the caret column will be clamped to the line's length. If wrap_index is greater than -1, the column will be moved to attempt to match the visual x position on the line's wrap_index to the position from the last time set_caret_column() was called.

Note: If supporting multiple carets this will not check for any overlap. See merge_overlapping_carets().

### set_caret_mid_grapheme_enabled {#set_caret_mid_grapheme_enabled}

```gdscript
void set_caret_mid_grapheme_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_type {#set_caret_type}

```gdscript
void set_caret_type(type: TextEdit.CaretType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_context_menu_enabled {#set_context_menu_enabled}

```gdscript
void set_context_menu_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_word_separators {#set_custom_word_separators}

```gdscript
void set_custom_word_separators(custom_word_separators: String)
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

### set_draw_caret_when_editable_disabled {#set_draw_caret_when_editable_disabled}

```gdscript
void set_draw_caret_when_editable_disabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_control_chars {#set_draw_control_chars}

```gdscript
void set_draw_control_chars(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_minimap {#set_draw_minimap}

```gdscript
void set_draw_minimap(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_spaces {#set_draw_spaces}

```gdscript
void set_draw_spaces(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_tabs {#set_draw_tabs}

```gdscript
void set_draw_tabs(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_editable {#set_editable}

```gdscript
void set_editable(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emoji_menu_enabled {#set_emoji_menu_enabled}

```gdscript
void set_emoji_menu_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_empty_selection_clipboard_enabled {#set_empty_selection_clipboard_enabled}

```gdscript
void set_empty_selection_clipboard_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fit_content_height_enabled {#set_fit_content_height_enabled}

```gdscript
void set_fit_content_height_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fit_content_width_enabled {#set_fit_content_width_enabled}

```gdscript
void set_fit_content_width_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gutter_clickable {#set_gutter_clickable}

```gdscript
void set_gutter_clickable(gutter: int, clickable: bool)
```

If true, the mouse cursor will change to a pointing hand (Control.CURSOR_POINTING_HAND) when hovering over the gutter at the given index. See is_gutter_clickable() and set_line_gutter_clickable().

### set_gutter_custom_draw {#set_gutter_custom_draw}

```gdscript
void set_gutter_custom_draw(column: int, draw_callback: Callable)
```

Set a custom draw callback for the gutter at the given index. draw_callback must take the following arguments: A line index int, a gutter index int, and an area Rect2. This callback only works when the gutter type is GUTTER_TYPE_CUSTOM (see set_gutter_type()).

### set_gutter_draw {#set_gutter_draw}

```gdscript
void set_gutter_draw(gutter: int, draw: bool)
```

If true, the gutter at the given index is drawn. The gutter type (set_gutter_type()) determines how it is drawn. See is_gutter_drawn().

### set_gutter_name {#set_gutter_name}

```gdscript
void set_gutter_name(gutter: int, name: String)
```

Sets the name of the gutter at the given index.

### set_gutter_overwritable {#set_gutter_overwritable}

```gdscript
void set_gutter_overwritable(gutter: int, overwritable: bool)
```

If true, the line data of the gutter at the given index can be overridden when using merge_gutters(). See is_gutter_overwritable().

### set_gutter_type {#set_gutter_type}

```gdscript
void set_gutter_type(gutter: int, type: TextEdit.GutterType)
```

Sets the type of gutter at the given index. Gutters can contain icons, text, or custom visuals.

### set_gutter_width {#set_gutter_width}

```gdscript
void set_gutter_width(gutter: int, width: int)
```

Set the width of the gutter at the given index.

### set_h_scroll {#set_h_scroll}

```gdscript
void set_h_scroll(value: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_highlight_all_occurrences {#set_highlight_all_occurrences}

```gdscript
void set_highlight_all_occurrences(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_highlight_current_line {#set_highlight_current_line}

```gdscript
void set_highlight_current_line(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_indent_wrapped_lines {#set_indent_wrapped_lines}

```gdscript
void set_indent_wrapped_lines(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line {#set_line}

```gdscript
void set_line(line: int, new_text: String)
```

Sets the text for a specific line.

Carets on the line will attempt to keep their visual x position.

### set_line_as_center_visible {#set_line_as_center_visible}

```gdscript
void set_line_as_center_visible(line: int, wrap_index: int = 0)
```

Positions the wrap_index of line at the center of the viewport.

### set_line_as_first_visible {#set_line_as_first_visible}

```gdscript
void set_line_as_first_visible(line: int, wrap_index: int = 0)
```

Positions the wrap_index of line at the top of the viewport.

### set_line_as_last_visible {#set_line_as_last_visible}

```gdscript
void set_line_as_last_visible(line: int, wrap_index: int = 0)
```

Positions the wrap_index of line at the bottom of the viewport.

### set_line_background_color {#set_line_background_color}

```gdscript
void set_line_background_color(line: int, color: Color)
```

Sets the custom background color of the given line. If transparent, this color is applied on top of the default background color (See background_color). If set to Color(0, 0, 0, 0), no additional color is applied.

### set_line_gutter_clickable {#set_line_gutter_clickable}

```gdscript
void set_line_gutter_clickable(line: int, gutter: int, clickable: bool)
```

If clickable is true, makes the gutter on the given line clickable. This is like set_gutter_clickable(), but for a single line. If is_gutter_clickable() is true, this will not have any effect. See is_line_gutter_clickable() and gutter_clicked.

### set_line_gutter_icon {#set_line_gutter_icon}

```gdscript
void set_line_gutter_icon(line: int, gutter: int, icon: Texture2D)
```

Sets the icon for gutter on line to icon. This only works when the gutter type is GUTTER_TYPE_ICON (see set_gutter_type()).

### set_line_gutter_item_color {#set_line_gutter_item_color}

```gdscript
void set_line_gutter_item_color(line: int, gutter: int, color: Color)
```

Sets the color for gutter on line to color.

### set_line_gutter_metadata {#set_line_gutter_metadata}

```gdscript
void set_line_gutter_metadata(line: int, gutter: int, metadata: Variant)
```

Sets the metadata for gutter on line to metadata.

### set_line_gutter_text {#set_line_gutter_text}

```gdscript
void set_line_gutter_text(line: int, gutter: int, text: String)
```

Sets the text for gutter on line to text. This only works when the gutter type is GUTTER_TYPE_STRING (see set_gutter_type()).

### set_line_wrapping_mode {#set_line_wrapping_mode}

```gdscript
void set_line_wrapping_mode(mode: TextEdit.LineWrappingMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_middle_mouse_paste_enabled {#set_middle_mouse_paste_enabled}

```gdscript
void set_middle_mouse_paste_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_minimap_width {#set_minimap_width}

```gdscript
void set_minimap_width(width: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_move_caret_on_right_click_enabled {#set_move_caret_on_right_click_enabled}

```gdscript
void set_move_caret_on_right_click_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_multiple_carets_enabled {#set_multiple_carets_enabled}

```gdscript
void set_multiple_carets_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_overtype_mode_enabled {#set_overtype_mode_enabled}

```gdscript
void set_overtype_mode_enabled(enabled: bool)
```

If true, enables overtype mode. In this mode, typing overrides existing text instead of inserting text. The ProjectSettings.input/ui_text_toggle_insert_mode action toggles overtype mode. See is_overtype_mode_enabled().

### set_placeholder {#set_placeholder}

```gdscript
void set_placeholder(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_past_end_of_file_enabled {#set_scroll_past_end_of_file_enabled}

```gdscript
void set_scroll_past_end_of_file_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_search_flags {#set_search_flags}

```gdscript
void set_search_flags(flags: int)
```

Sets the search flags. This is used with set_search_text() to highlight occurrences of the searched text. Search flags can be specified from the SearchFlags enum.

### set_search_text {#set_search_text}

```gdscript
void set_search_text(search_text: String)
```

Sets the search text. See set_search_flags().

### set_selecting_enabled {#set_selecting_enabled}

```gdscript
void set_selecting_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selection_mode {#set_selection_mode}

```gdscript
void set_selection_mode(mode: TextEdit.SelectionMode)
```

Sets the current selection mode.

### set_selection_origin_column {#set_selection_origin_column}

```gdscript
void set_selection_origin_column(column: int, caret_index: int = 0)
```

Sets the selection origin column to the column for the given caret_index. If the selection origin is moved to the caret position, the selection will deselect.

### set_selection_origin_line {#set_selection_origin_line}

```gdscript
void set_selection_origin_line(line: int, can_be_hidden: bool = true, wrap_index: int = -1, caret_index: int = 0)
```

Sets the selection origin line to the line for the given caret_index. If the selection origin is moved to the caret position, the selection will deselect.

If can_be_hidden is false, The line will be set to the nearest unhidden line below or above.

If wrap_index is -1, the selection origin column will be clamped to the line's length. If wrap_index is greater than -1, the column will be moved to attempt to match the visual x position on the line's wrap_index to the position from the last time set_selection_origin_column() or select() was called.

### set_shortcut_keys_enabled {#set_shortcut_keys_enabled}

```gdscript
void set_shortcut_keys_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_smooth_scroll_enabled {#set_smooth_scroll_enabled}

```gdscript
void set_smooth_scroll_enabled(enable: bool)
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

### set_syntax_highlighter {#set_syntax_highlighter}

```gdscript
void set_syntax_highlighter(syntax_highlighter: SyntaxHighlighter)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_input_mode {#set_tab_input_mode}

```gdscript
void set_tab_input_mode(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tab_size {#set_tab_size}

```gdscript
void set_tab_size(size: int)
```

Sets the tab size for the TextEdit to use.

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

### set_tooltip_request_func {#set_tooltip_request_func}

```gdscript
void set_tooltip_request_func(callback: Callable)
```

Provide custom tooltip text. The callback method must take the following args: hovered_word: String.

### set_use_custom_word_separators {#set_use_custom_word_separators}

```gdscript
void set_use_custom_word_separators(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_default_word_separators {#set_use_default_word_separators}

```gdscript
void set_use_default_word_separators(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_scroll {#set_v_scroll}

```gdscript
void set_v_scroll(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_scroll_speed {#set_v_scroll_speed}

```gdscript
void set_v_scroll_speed(speed: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_virtual_keyboard_enabled {#set_virtual_keyboard_enabled}

```gdscript
void set_virtual_keyboard_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_virtual_keyboard_show_on_focus {#set_virtual_keyboard_show_on_focus}

```gdscript
void set_virtual_keyboard_show_on_focus(show_on_focus: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### skip_selection_for_next_occurrence {#skip_selection_for_next_occurrence}

```gdscript
void skip_selection_for_next_occurrence()
```

Moves a selection and a caret for the next occurrence of the current selection. If there is no active selection, moves to the next occurrence of the word under caret.

### start_action {#start_action}

```gdscript
void start_action(action: TextEdit.EditAction)
```

Starts an action, will end the current action if action is different.

An action will also end after a call to end_action(), after ProjectSettings.gui/timers/text_edit_idle_detect_sec is triggered or a new undoable step outside the start_action() and end_action() calls.

### swap_lines {#swap_lines}

```gdscript
void swap_lines(from_line: int, to_line: int)
```

Swaps the two lines. Carets will be swapped with the lines.

### tag_saved_version {#tag_saved_version}

```gdscript
void tag_saved_version()
```

Tag the current version as saved.

### undo {#undo}

```gdscript
void undo()
```

Perform undo operation.

## 属性

### text {#text}

```gdscript
text: String
```

- void set_text(value: String)
- String get_text()

String value of the TextEdit.

### placeholder_text {#placeholder_text}

```gdscript
placeholder_text: String
```

- void set_placeholder(value: String)
- String get_placeholder()

Text shown when the TextEdit is empty. It is not the TextEdit's default value (see text).

### editable {#editable}

```gdscript
editable: bool
```

- void set_editable(value: bool)
- bool is_editable()

If false, existing text cannot be modified and new text cannot be added.

### context_menu_enabled {#context_menu_enabled}

```gdscript
context_menu_enabled: bool
```

- void set_context_menu_enabled(value: bool)
- bool is_context_menu_enabled()

If true, a right-click displays the context menu.

### emoji_menu_enabled {#emoji_menu_enabled}

```gdscript
emoji_menu_enabled: bool
```

- void set_emoji_menu_enabled(value: bool)
- bool is_emoji_menu_enabled()

If true, "Emoji and Symbols" menu is enabled.

### backspace_deletes_composite_character_enabled {#backspace_deletes_composite_character_enabled}

```gdscript
backspace_deletes_composite_character_enabled: bool
```

- void set_backspace_deletes_composite_character_enabled(value: bool)
- bool is_backspace_deletes_composite_character_enabled()

If true and caret_mid_grapheme is false, backspace deletes an entire composite character such as ❤️‍🩹, instead of deleting part of the composite character.

### shortcut_keys_enabled {#shortcut_keys_enabled}

```gdscript
shortcut_keys_enabled: bool
```

- void set_shortcut_keys_enabled(value: bool)
- bool is_shortcut_keys_enabled()

If true, shortcut keys for context menu items are enabled, even if the context menu is disabled.

### selecting_enabled {#selecting_enabled}

```gdscript
selecting_enabled: bool
```

- void set_selecting_enabled(value: bool)
- bool is_selecting_enabled()

If true, text can be selected.

If false, text can not be selected by the user or by the select() or select_all() methods.

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

If true, allow drag and drop of selected text. Text can still be dropped from other sources.

### middle_mouse_paste_enabled {#middle_mouse_paste_enabled}

```gdscript
middle_mouse_paste_enabled: bool
```

- void set_middle_mouse_paste_enabled(value: bool)
- bool is_middle_mouse_paste_enabled()

If false, using middle mouse button to paste clipboard will be disabled.

Note: This method is only implemented on Linux.

### empty_selection_clipboard_enabled {#empty_selection_clipboard_enabled}

```gdscript
empty_selection_clipboard_enabled: bool
```

- void set_empty_selection_clipboard_enabled(value: bool)
- bool is_empty_selection_clipboard_enabled()

If true, copying or cutting without a selection is performed on all lines with a caret. Otherwise, copy and cut require a selection.

### wrap_mode {#wrap_mode}

```gdscript
wrap_mode: int
```

- void set_line_wrapping_mode(value: LineWrappingMode)
- LineWrappingMode get_line_wrapping_mode()

Sets the line wrapping mode to use.

### autowrap_mode {#autowrap_mode}

```gdscript
autowrap_mode: int
```

- void set_autowrap_mode(value: AutowrapMode)
- AutowrapMode get_autowrap_mode()

If wrap_mode is set to LINE_WRAPPING_BOUNDARY, sets text wrapping mode.

### indent_wrapped_lines {#indent_wrapped_lines}

```gdscript
indent_wrapped_lines: bool
```

- void set_indent_wrapped_lines(value: bool)
- bool is_indent_wrapped_lines()

If true, all wrapped lines are indented to the same amount as the unwrapped line.

### tab_input_mode {#tab_input_mode}

```gdscript
tab_input_mode: bool
```

- void set_tab_input_mode(value: bool)
- bool get_tab_input_mode()

If true, ProjectSettings.input/ui_text_indent input Tab character, otherwise it moves keyboard focus to the next Control in the scene.

### virtual_keyboard_enabled {#virtual_keyboard_enabled}

```gdscript
virtual_keyboard_enabled: bool
```

- void set_virtual_keyboard_enabled(value: bool)
- bool is_virtual_keyboard_enabled()

If true, the native virtual keyboard is enabled on platforms that support it.

### virtual_keyboard_show_on_focus {#virtual_keyboard_show_on_focus}

```gdscript
virtual_keyboard_show_on_focus: bool
```

- void set_virtual_keyboard_show_on_focus(value: bool)
- bool get_virtual_keyboard_show_on_focus()

If true, the native virtual keyboard is shown on focus events on platforms that support it.

### scroll_smooth {#scroll_smooth}

```gdscript
scroll_smooth: bool
```

- void set_smooth_scroll_enabled(value: bool)
- bool is_smooth_scroll_enabled()

Scroll smoothly over the text rather than jumping to the next location.

### scroll_v_scroll_speed {#scroll_v_scroll_speed}

```gdscript
scroll_v_scroll_speed: float
```

- void set_v_scroll_speed(value: float)
- float get_v_scroll_speed()

Sets the scroll speed with the minimap or when scroll_smooth is enabled.

### scroll_past_end_of_file {#scroll_past_end_of_file}

```gdscript
scroll_past_end_of_file: bool
```

- void set_scroll_past_end_of_file_enabled(value: bool)
- bool is_scroll_past_end_of_file_enabled()

Allow scrolling past the last line into "virtual" space.

### scroll_vertical {#scroll_vertical}

```gdscript
scroll_vertical: float
```

- void set_v_scroll(value: float)
- float get_v_scroll()

If there is a vertical scrollbar, this determines the current vertical scroll value in line numbers, starting at 0 for the top line.

### scroll_horizontal {#scroll_horizontal}

```gdscript
scroll_horizontal: int
```

- void set_h_scroll(value: int)
- int get_h_scroll()

If there is a horizontal scrollbar, this determines the current horizontal scroll value in pixels.

### scroll_fit_content_height {#scroll_fit_content_height}

```gdscript
scroll_fit_content_height: bool
```

- void set_fit_content_height_enabled(value: bool)
- bool is_fit_content_height_enabled()

If true, TextEdit fits its minimum height to the number of visible lines instead of scrolling vertically. If a maximum height is set (for example via Control.custom_maximum_size) and content exceeds it, a vertical scrollbar is shown.

### scroll_fit_content_width {#scroll_fit_content_width}

```gdscript
scroll_fit_content_width: bool
```

- void set_fit_content_width_enabled(value: bool)
- bool is_fit_content_width_enabled()

If true, TextEdit fits its minimum width to the widest line instead of scrolling horizontally. If a maximum width is set (for example via Control.custom_maximum_size) and content exceeds it, a horizontal scrollbar is shown.

### minimap_draw {#minimap_draw}

```gdscript
minimap_draw: bool
```

- void set_draw_minimap(value: bool)
- bool is_drawing_minimap()

If true, a minimap is shown, providing an outline of your source code. The minimap uses a fixed-width text size.

### minimap_width {#minimap_width}

```gdscript
minimap_width: int
```

- void set_minimap_width(value: int)
- int get_minimap_width()

The width, in pixels, of the minimap.

### caret_type {#caret_type}

```gdscript
caret_type: int
```

- void set_caret_type(value: CaretType)
- CaretType get_caret_type()

Set the type of caret to draw.

### caret_blink {#caret_blink}

```gdscript
caret_blink: bool
```

- void set_caret_blink_enabled(value: bool)
- bool is_caret_blink_enabled()

If true, makes the caret blink.

### caret_blink_interval {#caret_blink_interval}

```gdscript
caret_blink_interval: float
```

- void set_caret_blink_interval(value: float)
- float get_caret_blink_interval()

The interval at which the caret blinks (in seconds).

### caret_draw_when_editable_disabled {#caret_draw_when_editable_disabled}

```gdscript
caret_draw_when_editable_disabled: bool
```

- void set_draw_caret_when_editable_disabled(value: bool)
- bool is_drawing_caret_when_editable_disabled()

If true, caret will be visible when editable is disabled.

### caret_move_on_right_click {#caret_move_on_right_click}

```gdscript
caret_move_on_right_click: bool
```

- void set_move_caret_on_right_click_enabled(value: bool)
- bool is_move_caret_on_right_click_enabled()

If true, a right-click moves the caret at the mouse position before displaying the context menu.

If false, the context menu ignores mouse location.

### caret_mid_grapheme {#caret_mid_grapheme}

```gdscript
caret_mid_grapheme: bool
```

- void set_caret_mid_grapheme_enabled(value: bool)
- bool is_caret_mid_grapheme_enabled()

Allow moving caret, selecting and removing the individual composite character components.

Note: Backspace is always removing individual composite character components.

### caret_multiple {#caret_multiple}

```gdscript
caret_multiple: bool
```

- void set_multiple_carets_enabled(value: bool)
- bool is_multiple_carets_enabled()

If true, multiple carets are allowed. Left-clicking with Alt adds a new caret. See add_caret() and get_caret_count().

### use_default_word_separators {#use_default_word_separators}

```gdscript
use_default_word_separators: bool
```

- void set_use_default_word_separators(value: bool)
- bool is_default_word_separators_enabled()

If false, using Ctrl + Left or Ctrl + Right (Cmd + Left or Cmd + Right on macOS) bindings will stop moving caret only if a space or punctuation is detected. If true, it will also stop the caret if a character is part of !"#$%&'()*+,-./:;<=>?@[\]^`{|}~, the Unicode General Punctuation table, or the Unicode CJK Punctuation table. Useful for subword moving. This behavior also will be applied to the behavior of text selection.

### use_custom_word_separators {#use_custom_word_separators}

```gdscript
use_custom_word_separators: bool
```

- void set_use_custom_word_separators(value: bool)
- bool is_custom_word_separators_enabled()

If false, using Ctrl + Left or Ctrl + Right (Cmd + Left or Cmd + Right on macOS) bindings will use the behavior of use_default_word_separators. If true, it will also stop the caret if a character within custom_word_separators is detected. Useful for subword moving. This behavior also will be applied to the behavior of text selection.

### custom_word_separators {#custom_word_separators}

```gdscript
custom_word_separators: String
```

- void set_custom_word_separators(value: String)
- String get_custom_word_separators()

The characters to consider as word delimiters if use_custom_word_separators is true. The characters should be defined without separation, for example #_!.

### syntax_highlighter {#syntax_highlighter}

```gdscript
syntax_highlighter: SyntaxHighlighter
```

- void set_syntax_highlighter(value: SyntaxHighlighter)
- SyntaxHighlighter get_syntax_highlighter()

The syntax highlighter to use.

Note: A SyntaxHighlighter instance should not be used across multiple TextEdit nodes.

### highlight_all_occurrences {#highlight_all_occurrences}

```gdscript
highlight_all_occurrences: bool
```

- void set_highlight_all_occurrences(value: bool)
- bool is_highlight_all_occurrences_enabled()

If true, all occurrences of the selected text will be highlighted.

### highlight_current_line {#highlight_current_line}

```gdscript
highlight_current_line: bool
```

- void set_highlight_current_line(value: bool)
- bool is_highlight_current_line_enabled()

If true, the line containing the cursor is highlighted.

### draw_control_chars {#draw_control_chars}

```gdscript
draw_control_chars: bool
```

- void set_draw_control_chars(value: bool)
- bool get_draw_control_chars()

If true, control characters are displayed.

### draw_tabs {#draw_tabs}

```gdscript
draw_tabs: bool
```

- void set_draw_tabs(value: bool)
- bool is_drawing_tabs()

If true, the "tab" character will have a visible representation.

### draw_spaces {#draw_spaces}

```gdscript
draw_spaces: bool
```

- void set_draw_spaces(value: bool)
- bool is_drawing_spaces()

If true, the "space" character will have a visible representation.

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

### text_set {#text_set}

```gdscript
signal text_set()
```

Emitted when clear() is called or text is set.

### text_changed {#text_changed}

```gdscript
signal text_changed()
```

Emitted when the text changes.

### lines_edited_from {#lines_edited_from}

```gdscript
signal lines_edited_from(from_line: int, to_line: int)
```

Emitted immediately when the text changes.

When text is added from_line will be less than to_line. On a remove to_line will be less than from_line.

### caret_changed {#caret_changed}

```gdscript
signal caret_changed()
```

Emitted when any caret changes position.

### gutter_clicked {#gutter_clicked}

```gdscript
signal gutter_clicked(line: int, gutter: int)
```

Emitted when a gutter is clicked.

### gutter_added {#gutter_added}

```gdscript
signal gutter_added()
```

Emitted when a gutter is added.

### gutter_removed {#gutter_removed}

```gdscript
signal gutter_removed()
```

Emitted when a gutter is removed.

## 枚举

### MenuItems {#enum-MenuItems}

```gdscript
enum MenuItems { MENU_CUT = 0, MENU_COPY = 1, MENU_PASTE = 2, MENU_CLEAR = 3, MENU_SELECT_ALL = 4, MENU_UNDO = 5, MENU_REDO = 6, MENU_SUBMENU_TEXT_DIR = 7, MENU_DIR_INHERITED = 8, MENU_DIR_AUTO = 9, MENU_DIR_LTR = 10, MENU_DIR_RTL = 11, MENU_DISPLAY_UCC = 12, MENU_SUBMENU_INSERT_UCC = 13, MENU_INSERT_LRM = 14, MENU_INSERT_RLM = 15, MENU_INSERT_LRE = 16, MENU_INSERT_RLE = 17, MENU_INSERT_LRO = 18, MENU_INSERT_RLO = 19, MENU_INSERT_PDF = 20, MENU_INSERT_ALM = 21, MENU_INSERT_LRI = 22, MENU_INSERT_RLI = 23, MENU_INSERT_FSI = 24, MENU_INSERT_PDI = 25, MENU_INSERT_ZWJ = 26, MENU_INSERT_ZWNJ = 27, MENU_INSERT_WJ = 28, MENU_INSERT_SHY = 29, MENU_EMOJI_AND_SYMBOL = 30, MENU_MAX = 31 }
```

MenuItems MENU_CUT = 0

Cuts (copies and clears) the selected text.

MenuItems MENU_COPY = 1

Copies the selected text.

MenuItems MENU_PASTE = 2

Pastes the clipboard text over the selected text (or at the cursor's position).

MenuItems MENU_CLEAR = 3

Erases the whole TextEdit text.

MenuItems MENU_SELECT_ALL = 4

Selects the whole TextEdit text.

MenuItems MENU_UNDO = 5

Undoes the previous action.

MenuItems MENU_REDO = 6

Redoes the previous action.

MenuItems MENU_SUBMENU_TEXT_DIR = 7

ID of "Text Writing Direction" submenu.

MenuItems MENU_DIR_INHERITED = 8

Sets text direction to inherited.

MenuItems MENU_DIR_AUTO = 9

Sets text direction to automatic.

MenuItems MENU_DIR_LTR = 10

Sets text direction to left-to-right.

MenuItems MENU_DIR_RTL = 11

Sets text direction to right-to-left.

MenuItems MENU_DISPLAY_UCC = 12

Toggles control character display.

MenuItems MENU_SUBMENU_INSERT_UCC = 13

ID of "Insert Control Character" submenu.

MenuItems MENU_INSERT_LRM = 14

Inserts left-to-right mark (LRM) character.

MenuItems MENU_INSERT_RLM = 15

Inserts right-to-left mark (RLM) character.

MenuItems MENU_INSERT_LRE = 16

Inserts start of left-to-right embedding (LRE) character.

MenuItems MENU_INSERT_RLE = 17

Inserts start of right-to-left embedding (RLE) character.

MenuItems MENU_INSERT_LRO = 18

Inserts start of left-to-right override (LRO) character.

MenuItems MENU_INSERT_RLO = 19

Inserts start of right-to-left override (RLO) character.

MenuItems MENU_INSERT_PDF = 20

Inserts pop direction formatting (PDF) character.

MenuItems MENU_INSERT_ALM = 21

Inserts Arabic letter mark (ALM) character.

MenuItems MENU_INSERT_LRI = 22

Inserts left-to-right isolate (LRI) character.

MenuItems MENU_INSERT_RLI = 23

Inserts right-to-left isolate (RLI) character.

MenuItems MENU_INSERT_FSI = 24

Inserts first strong isolate (FSI) character.

MenuItems MENU_INSERT_PDI = 25

Inserts pop direction isolate (PDI) character.

MenuItems MENU_INSERT_ZWJ = 26

Inserts zero width joiner (ZWJ) character.

MenuItems MENU_INSERT_ZWNJ = 27

Inserts zero width non-joiner (ZWNJ) character.

MenuItems MENU_INSERT_WJ = 28

Inserts word joiner (WJ) character.

MenuItems MENU_INSERT_SHY = 29

Inserts soft hyphen (SHY) character.

MenuItems MENU_EMOJI_AND_SYMBOL = 30

Opens system emoji and symbol picker.

MenuItems MENU_MAX = 31

Represents the size of the MenuItems enum.

#### MENU_CUT {#MENU_CUT}

```gdscript
MENU_CUT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_COPY {#MENU_COPY}

```gdscript
MENU_COPY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_PASTE {#MENU_PASTE}

```gdscript
MENU_PASTE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_CLEAR {#MENU_CLEAR}

```gdscript
MENU_CLEAR = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_SELECT_ALL {#MENU_SELECT_ALL}

```gdscript
MENU_SELECT_ALL = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_UNDO {#MENU_UNDO}

```gdscript
MENU_UNDO = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_REDO {#MENU_REDO}

```gdscript
MENU_REDO = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_SUBMENU_TEXT_DIR {#MENU_SUBMENU_TEXT_DIR}

```gdscript
MENU_SUBMENU_TEXT_DIR = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_DIR_INHERITED {#MENU_DIR_INHERITED}

```gdscript
MENU_DIR_INHERITED = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_DIR_AUTO {#MENU_DIR_AUTO}

```gdscript
MENU_DIR_AUTO = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_DIR_LTR {#MENU_DIR_LTR}

```gdscript
MENU_DIR_LTR = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_DIR_RTL {#MENU_DIR_RTL}

```gdscript
MENU_DIR_RTL = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_DISPLAY_UCC {#MENU_DISPLAY_UCC}

```gdscript
MENU_DISPLAY_UCC = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_SUBMENU_INSERT_UCC {#MENU_SUBMENU_INSERT_UCC}

```gdscript
MENU_SUBMENU_INSERT_UCC = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_LRM {#MENU_INSERT_LRM}

```gdscript
MENU_INSERT_LRM = 14
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_RLM {#MENU_INSERT_RLM}

```gdscript
MENU_INSERT_RLM = 15
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_LRE {#MENU_INSERT_LRE}

```gdscript
MENU_INSERT_LRE = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_RLE {#MENU_INSERT_RLE}

```gdscript
MENU_INSERT_RLE = 17
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_LRO {#MENU_INSERT_LRO}

```gdscript
MENU_INSERT_LRO = 18
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_RLO {#MENU_INSERT_RLO}

```gdscript
MENU_INSERT_RLO = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_PDF {#MENU_INSERT_PDF}

```gdscript
MENU_INSERT_PDF = 20
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_ALM {#MENU_INSERT_ALM}

```gdscript
MENU_INSERT_ALM = 21
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_LRI {#MENU_INSERT_LRI}

```gdscript
MENU_INSERT_LRI = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_RLI {#MENU_INSERT_RLI}

```gdscript
MENU_INSERT_RLI = 23
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_FSI {#MENU_INSERT_FSI}

```gdscript
MENU_INSERT_FSI = 24
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_PDI {#MENU_INSERT_PDI}

```gdscript
MENU_INSERT_PDI = 25
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_ZWJ {#MENU_INSERT_ZWJ}

```gdscript
MENU_INSERT_ZWJ = 26
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_ZWNJ {#MENU_INSERT_ZWNJ}

```gdscript
MENU_INSERT_ZWNJ = 27
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_WJ {#MENU_INSERT_WJ}

```gdscript
MENU_INSERT_WJ = 28
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_INSERT_SHY {#MENU_INSERT_SHY}

```gdscript
MENU_INSERT_SHY = 29
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_EMOJI_AND_SYMBOL {#MENU_EMOJI_AND_SYMBOL}

```gdscript
MENU_EMOJI_AND_SYMBOL = 30
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_MAX {#MENU_MAX}

```gdscript
MENU_MAX = 31
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### EditAction {#enum-EditAction}

```gdscript
enum EditAction { ACTION_NONE = 0, ACTION_TYPING = 1, ACTION_BACKSPACE = 2, ACTION_DELETE = 3 }
```

EditAction ACTION_NONE = 0

No current action.

EditAction ACTION_TYPING = 1

A typing action.

EditAction ACTION_BACKSPACE = 2

A backwards delete action.

EditAction ACTION_DELETE = 3

A forward delete action.

#### ACTION_NONE {#ACTION_NONE}

```gdscript
ACTION_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ACTION_TYPING {#ACTION_TYPING}

```gdscript
ACTION_TYPING = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ACTION_BACKSPACE {#ACTION_BACKSPACE}

```gdscript
ACTION_BACKSPACE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ACTION_DELETE {#ACTION_DELETE}

```gdscript
ACTION_DELETE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### SearchFlags {#enum-SearchFlags}

```gdscript
enum SearchFlags { SEARCH_MATCH_CASE = 1, SEARCH_WHOLE_WORDS = 2, SEARCH_BACKWARDS = 4 }
```

SearchFlags SEARCH_MATCH_CASE = 1

Match case when searching.

SearchFlags SEARCH_WHOLE_WORDS = 2

Match whole words when searching.

SearchFlags SEARCH_BACKWARDS = 4

Search from end to beginning.

#### SEARCH_MATCH_CASE {#SEARCH_MATCH_CASE}

```gdscript
SEARCH_MATCH_CASE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SEARCH_WHOLE_WORDS {#SEARCH_WHOLE_WORDS}

```gdscript
SEARCH_WHOLE_WORDS = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SEARCH_BACKWARDS {#SEARCH_BACKWARDS}

```gdscript
SEARCH_BACKWARDS = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CaretType {#enum-CaretType}

```gdscript
enum CaretType { CARET_TYPE_LINE = 0, CARET_TYPE_BLOCK = 1 }
```

CaretType CARET_TYPE_LINE = 0

Vertical line caret.

CaretType CARET_TYPE_BLOCK = 1

Block caret.

#### CARET_TYPE_LINE {#CARET_TYPE_LINE}

```gdscript
CARET_TYPE_LINE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CARET_TYPE_BLOCK {#CARET_TYPE_BLOCK}

```gdscript
CARET_TYPE_BLOCK = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### SelectionMode {#enum-SelectionMode}

```gdscript
enum SelectionMode { SELECTION_MODE_NONE = 0, SELECTION_MODE_SHIFT = 1, SELECTION_MODE_POINTER = 2, SELECTION_MODE_WORD = 3, SELECTION_MODE_LINE = 4 }
```

SelectionMode SELECTION_MODE_NONE = 0

Not selecting.

SelectionMode SELECTION_MODE_SHIFT = 1

Select as if shift is pressed.

SelectionMode SELECTION_MODE_POINTER = 2

Select single characters as if the user single clicked.

SelectionMode SELECTION_MODE_WORD = 3

Select whole words as if the user double clicked.

SelectionMode SELECTION_MODE_LINE = 4

Select whole lines as if the user triple clicked.

#### SELECTION_MODE_NONE {#SELECTION_MODE_NONE}

```gdscript
SELECTION_MODE_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SELECTION_MODE_SHIFT {#SELECTION_MODE_SHIFT}

```gdscript
SELECTION_MODE_SHIFT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SELECTION_MODE_POINTER {#SELECTION_MODE_POINTER}

```gdscript
SELECTION_MODE_POINTER = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SELECTION_MODE_WORD {#SELECTION_MODE_WORD}

```gdscript
SELECTION_MODE_WORD = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SELECTION_MODE_LINE {#SELECTION_MODE_LINE}

```gdscript
SELECTION_MODE_LINE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### LineWrappingMode {#enum-LineWrappingMode}

```gdscript
enum LineWrappingMode { LINE_WRAPPING_NONE = 0, LINE_WRAPPING_BOUNDARY = 1 }
```

LineWrappingMode LINE_WRAPPING_NONE = 0

Line wrapping is disabled.

LineWrappingMode LINE_WRAPPING_BOUNDARY = 1

Line wrapping occurs at the control boundary, beyond what would normally be visible.

#### LINE_WRAPPING_NONE {#LINE_WRAPPING_NONE}

```gdscript
LINE_WRAPPING_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LINE_WRAPPING_BOUNDARY {#LINE_WRAPPING_BOUNDARY}

```gdscript
LINE_WRAPPING_BOUNDARY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### GutterType {#enum-GutterType}

```gdscript
enum GutterType { GUTTER_TYPE_STRING = 0, GUTTER_TYPE_ICON = 1, GUTTER_TYPE_CUSTOM = 2 }
```

GutterType GUTTER_TYPE_STRING = 0

When a gutter is set to string using set_gutter_type(), it is used to contain text set via the set_line_gutter_text() method.

GutterType GUTTER_TYPE_ICON = 1

When a gutter is set to icon using set_gutter_type(), it is used to contain an icon set via the set_line_gutter_icon() method.

GutterType GUTTER_TYPE_CUSTOM = 2

When a gutter is set to custom using set_gutter_type(), it is used to contain custom visuals controlled by a callback method set via the set_gutter_custom_draw() method.

#### GUTTER_TYPE_STRING {#GUTTER_TYPE_STRING}

```gdscript
GUTTER_TYPE_STRING = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GUTTER_TYPE_ICON {#GUTTER_TYPE_ICON}

```gdscript
GUTTER_TYPE_ICON = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GUTTER_TYPE_CUSTOM {#GUTTER_TYPE_CUSTOM}

```gdscript
GUTTER_TYPE_CUSTOM = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
