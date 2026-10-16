# LineEdit

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **LineEdit**

> LineEdit provides an input field for editing a single line of text.

<details><summary>官方英文描述（中文精译进行中）</summary>

LineEdit provides an input field for editing a single line of text.

- When the LineEdit control is focused using the keyboard arrow keys, it will only gain focus and not enter edit mode.
- To enter edit mode, click on the control with the mouse, see also keep_editing_on_text_submit.
- To exit edit mode, press ui_text_submit or ui_cancel (by default Escape) actions.
- Check edit(), unedit(), is_editing(), and editing_toggled for more information.

While entering text, it is possible to insert special characters using Unicode, OEM or Windows alt codes:

- To enter Unicode codepoints, hold Alt and type the codepoint on the numpad. For example, to enter the character á (U+00E1), hold Alt and type +E1 on the numpad (the leading zeroes can be omitted).
- To enter OEM codepoints, hold Alt and type the code on the numpad. For example, to enter the character á (OEM 160), hold Alt and type 160 on the numpad.
- To enter Windows codepoints, hold Alt and type the code on the numpad. For example, to enter the character á (Windows 0225), hold Alt and type 0, 2, 2, 5 on the numpad. The leading zero here must not be omitted, as this is how Windows codepoints are distinguished from OEM codepoints.

Important:

- Focusing the LineEdit with ui_focus_next (by default Tab) or ui_focus_prev (by default Shift + Tab) or Control.grab_focus() still enters edit mode (for compatibility).

LineEdit features many built-in shortcuts that are always available (Ctrl here maps to Cmd on macOS):

- Ctrl + C: Copy
- Ctrl + X: Cut
- Ctrl + V or Ctrl + Y: Paste/"yank"
- Ctrl + Z: Undo
- Ctrl + ~: Swap input direction.
- Ctrl + Shift + Z: Redo
- Ctrl + U: Delete text from the caret position to the beginning of the line
- Ctrl + K: Delete text from the caret position to the end of the line
- Ctrl + A: Select all text
- Up Arrow/Down Arrow: Move the caret to the beginning/end of the line

On macOS, some extra keyboard shortcuts are available:

- Cmd + F: Same as Right Arrow, move the caret one character right
- Cmd + B: Same as Left Arrow, move the caret one character left
- Cmd + P: Same as Up Arrow, move the caret to the previous line
- Cmd + N: Same as Down Arrow, move the caret to the next line
- Cmd + D: Same as Delete, delete the character on the right side of caret
- Cmd + H: Same as Backspace, delete the character on the left side of the caret
- Cmd + A: Same as Home, move the caret to the beginning of the line
- Cmd + E: Same as End, move the caret to the end of the line
- Cmd + Left Arrow: Same as Home, move the caret to the beginning of the line
- Cmd + Right Arrow: Same as End, move the caret to the end of the line

Note: Caret movement shortcuts listed above are not affected by shortcut_keys_enabled.

</details>

## 方法

### apply_ime {#apply_ime}

```gdscript
void apply_ime()
```

Applies text from the Input Method Editor [https://en.wikipedia.org/wiki/Input_method] (IME) and closes the IME if it is open.

### cancel_ime {#cancel_ime}

```gdscript
void cancel_ime()
```

Closes the Input Method Editor [https://en.wikipedia.org/wiki/Input_method] (IME) if it is open. Any text in the IME will be lost.

### clear {#clear}

```gdscript
void clear()
```

Erases the LineEdit's text.

### delete_char_at_caret {#delete_char_at_caret}

```gdscript
void delete_char_at_caret()
```

Deletes one character at the caret's current position (equivalent to pressing Delete).

### delete_text {#delete_text}

```gdscript
void delete_text(from_column: int, to_column: int)
```

Deletes a section of the text going from position from_column to to_column. Both parameters should be within the text's length.

### deselect {#deselect}

```gdscript
void deselect()
```

Clears the current selection.

### edit {#edit}

```gdscript
void edit(hide_focus: bool = false)
```

Allows entering edit mode whether the LineEdit is focused or not. If hide_focus is true, the focused state will not be shown (see Control.grab_focus()).

See also keep_editing_on_text_submit.

### get_caret_blink_interval {#get_caret_blink_interval}

```gdscript
float get_caret_blink_interval() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_caret_column {#get_caret_column}

```gdscript
int get_caret_column() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_draw_control_chars {#get_draw_control_chars}

```gdscript
bool get_draw_control_chars() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_horizontal_alignment {#get_horizontal_alignment}

```gdscript
HorizontalAlignment get_horizontal_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_icon_expand_mode {#get_icon_expand_mode}

```gdscript
LineEdit.ExpandMode get_icon_expand_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_length {#get_max_length}

```gdscript
int get_max_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_menu {#get_menu}

```gdscript
PopupMenu get_menu() const
```

Returns the PopupMenu of this LineEdit. By default, this menu is displayed when right-clicking on the LineEdit.

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
    menu.ItemCount = menu.GetItemIndex(LineEdit.MenuItems.Redo) + 1;
    // Add custom items.
    menu.AddSeparator();
    menu.AddItem("Insert Date", LineEdit.MenuItems.Max + 1);
    // Add event handler.
    menu.IdPressed += OnItemPressed;
}
public void OnItemPressed(int id)
{
    if (id == LineEdit.MenuItems.Max + 1)
    {
        InsertTextAtCaret(Time.GetDateStringFromSystem());
    }
}
```

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their Window.visible property.

### get_next_composite_character_column {#get_next_composite_character_column}

```gdscript
int get_next_composite_character_column(column: int) const
```

Returns the correct column at the end of a composite character like ❤️‍🩹 (mending heart; Unicode: U+2764 U+FE0F U+200D U+1FA79) which is comprised of more than one Unicode code point, if the caret is at the start of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.

Note: To check at caret location use get_next_composite_character_column(get_caret_column())

### get_placeholder {#get_placeholder}

```gdscript
String get_placeholder() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_previous_composite_character_column {#get_previous_composite_character_column}

```gdscript
int get_previous_composite_character_column(column: int) const
```

Returns the correct column at the start of a composite character like ❤️‍🩹 (mending heart; Unicode: U+2764 U+FE0F U+200D U+1FA79) which is comprised of more than one Unicode code point, if the caret is at the end of the composite character. Also returns the correct column with the caret at mid grapheme and for non-composite characters.

Note: To check at caret location use get_previous_composite_character_column(get_caret_column())

### get_right_icon {#get_right_icon}

```gdscript
Texture2D get_right_icon()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_right_icon_scale {#get_right_icon_scale}

```gdscript
float get_right_icon_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scroll_offset {#get_scroll_offset}

```gdscript
float get_scroll_offset() const
```

Returns the scroll offset due to caret_column, as a number of characters.

### get_secret_character {#get_secret_character}

```gdscript
String get_secret_character() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_selected_text {#get_selected_text}

```gdscript
String get_selected_text()
```

Returns the text inside the selection.

### get_selection_from_column {#get_selection_from_column}

```gdscript
int get_selection_from_column() const
```

Returns the selection begin column.

### get_selection_to_column {#get_selection_to_column}

```gdscript
int get_selection_to_column() const
```

Returns the selection end column.

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
Control.TextDirection get_text_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_virtual_keyboard_show_on_focus {#get_virtual_keyboard_show_on_focus}

```gdscript
bool get_virtual_keyboard_show_on_focus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_virtual_keyboard_type {#get_virtual_keyboard_type}

```gdscript
LineEdit.VirtualKeyboardType get_virtual_keyboard_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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
bool has_selection() const
```

Returns true if the user has selected text.

### has_undo {#has_undo}

```gdscript
bool has_undo() const
```

Returns true if an "undo" action is available.

### insert_text_at_caret {#insert_text_at_caret}

```gdscript
void insert_text_at_caret(text: String)
```

Inserts text at the caret. If the resulting value is longer than max_length, nothing happens.

### is_backspace_deletes_composite_character_enabled {#is_backspace_deletes_composite_character_enabled}

```gdscript
bool is_backspace_deletes_composite_character_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_caret_blink_enabled {#is_caret_blink_enabled}

```gdscript
bool is_caret_blink_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_caret_force_displayed {#is_caret_force_displayed}

```gdscript
bool is_caret_force_displayed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_caret_mid_grapheme_enabled {#is_caret_mid_grapheme_enabled}

```gdscript
bool is_caret_mid_grapheme_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_clear_button_enabled {#is_clear_button_enabled}

```gdscript
bool is_clear_button_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_context_menu_enabled {#is_context_menu_enabled}

```gdscript
bool is_context_menu_enabled()
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

### is_editable {#is_editable}

```gdscript
bool is_editable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editing {#is_editing}

```gdscript
bool is_editing() const
```

Returns whether the LineEdit is being edited.

### is_editing_kept_on_text_submit {#is_editing_kept_on_text_submit}

```gdscript
bool is_editing_kept_on_text_submit() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_emoji_menu_enabled {#is_emoji_menu_enabled}

```gdscript
bool is_emoji_menu_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_expand_to_text_length_enabled {#is_expand_to_text_length_enabled}

```gdscript
bool is_expand_to_text_length_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_flat {#is_flat}

```gdscript
bool is_flat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_menu_visible {#is_menu_visible}

```gdscript
bool is_menu_visible() const
```

Returns whether the menu is visible. Use this instead of get_menu().visible to improve performance (so the creation of the menu is avoided).

### is_middle_mouse_paste_enabled {#is_middle_mouse_paste_enabled}

```gdscript
bool is_middle_mouse_paste_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_secret {#is_secret}

```gdscript
bool is_secret() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_select_all_on_focus {#is_select_all_on_focus}

```gdscript
bool is_select_all_on_focus() const
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

### select {#select}

```gdscript
void select(from: int = 0, to: int = -1)
```

Selects characters inside LineEdit between from and to. By default, from is at the beginning and to at the end.

```
text = "Welcome"
select() # Will select "Welcome".
select(4) # Will select "ome".
select(2, 5) # Will select "lco".
```

```
Text = "Welcome";
Select(); // Will select "Welcome".
Select(4); // Will select "ome".
Select(2, 5); // Will select "lco".
```

### select_all {#select_all}

```gdscript
void select_all()
```

Selects the whole String.

### set_backspace_deletes_composite_character_enabled {#set_backspace_deletes_composite_character_enabled}

```gdscript
void set_backspace_deletes_composite_character_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_blink_enabled {#set_caret_blink_enabled}

```gdscript
void set_caret_blink_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_blink_interval {#set_caret_blink_interval}

```gdscript
void set_caret_blink_interval(interval: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_column {#set_caret_column}

```gdscript
void set_caret_column(position: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_force_displayed {#set_caret_force_displayed}

```gdscript
void set_caret_force_displayed(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_caret_mid_grapheme_enabled {#set_caret_mid_grapheme_enabled}

```gdscript
void set_caret_mid_grapheme_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_clear_button_enabled {#set_clear_button_enabled}

```gdscript
void set_clear_button_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_context_menu_enabled {#set_context_menu_enabled}

```gdscript
void set_context_menu_enabled(enable: bool)
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

### set_draw_control_chars {#set_draw_control_chars}

```gdscript
void set_draw_control_chars(enable: bool)
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

### set_expand_to_text_length_enabled {#set_expand_to_text_length_enabled}

```gdscript
void set_expand_to_text_length_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flat {#set_flat}

```gdscript
void set_flat(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_horizontal_alignment {#set_horizontal_alignment}

```gdscript
void set_horizontal_alignment(alignment: HorizontalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_icon_expand_mode {#set_icon_expand_mode}

```gdscript
void set_icon_expand_mode(mode: LineEdit.ExpandMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keep_editing_on_text_submit {#set_keep_editing_on_text_submit}

```gdscript
void set_keep_editing_on_text_submit(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_length {#set_max_length}

```gdscript
void set_max_length(chars: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_middle_mouse_paste_enabled {#set_middle_mouse_paste_enabled}

```gdscript
void set_middle_mouse_paste_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_placeholder {#set_placeholder}

```gdscript
void set_placeholder(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_right_icon {#set_right_icon}

```gdscript
void set_right_icon(icon: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_right_icon_scale {#set_right_icon_scale}

```gdscript
void set_right_icon_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secret {#set_secret}

```gdscript
void set_secret(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_secret_character {#set_secret_character}

```gdscript
void set_secret_character(character: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_select_all_on_focus {#set_select_all_on_focus}

```gdscript
void set_select_all_on_focus(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selecting_enabled {#set_selecting_enabled}

```gdscript
void set_selecting_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shortcut_keys_enabled {#set_shortcut_keys_enabled}

```gdscript
void set_shortcut_keys_enabled(enable: bool)
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
void set_text_direction(direction: Control.TextDirection)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_virtual_keyboard_enabled {#set_virtual_keyboard_enabled}

```gdscript
void set_virtual_keyboard_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_virtual_keyboard_show_on_focus {#set_virtual_keyboard_show_on_focus}

```gdscript
void set_virtual_keyboard_show_on_focus(show_on_focus: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_virtual_keyboard_type {#set_virtual_keyboard_type}

```gdscript
void set_virtual_keyboard_type(type: LineEdit.VirtualKeyboardType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### unedit {#unedit}

```gdscript
void unedit()
```

Allows exiting edit mode while preserving focus.

## 属性

### text {#text}

```gdscript
text: String
```

- void set_text(value: String)
- String get_text()

String value of the LineEdit.

Note: Changing text using this property won't emit the text_changed signal.

### placeholder_text {#placeholder_text}

```gdscript
placeholder_text: String
```

- void set_placeholder(value: String)
- String get_placeholder()

Text shown when the LineEdit is empty. It is not the LineEdit's default value (see text).

### alignment {#alignment}

```gdscript
alignment: int
```

- void set_horizontal_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_horizontal_alignment()

The text's horizontal alignment.

### max_length {#max_length}

```gdscript
max_length: int
```

- void set_max_length(value: int)
- int get_max_length()

Maximum number of characters that can be entered inside the LineEdit. If 0, there is no limit.

When a limit is defined, characters that would exceed max_length are truncated. This happens both for existing text contents when setting the max length, or for new text inserted in the LineEdit, including pasting.

If any input text is truncated, the text_change_rejected signal is emitted with the truncated substring as a parameter:

```
text = "Hello world"
max_length = 5
# `text` becomes "Hello".
max_length = 10
text += " goodbye"
# `text` becomes "Hello good".
# `text_change_rejected` is emitted with "bye" as a parameter.
```

```
Text = "Hello world";
MaxLength = 5;
// `Text` becomes "Hello".
MaxLength = 10;
Text += " goodbye";
// `Text` becomes "Hello good".
// `text_change_rejected` is emitted with "bye" as a parameter.
```

### editable {#editable}

```gdscript
editable: bool
```

- void set_editable(value: bool)
- bool is_editable()

If false, existing text cannot be modified and new text cannot be added.

### keep_editing_on_text_submit {#keep_editing_on_text_submit}

```gdscript
keep_editing_on_text_submit: bool
```

- void set_keep_editing_on_text_submit(value: bool)
- bool is_editing_kept_on_text_submit()

If true, the LineEdit will not exit edit mode when text is submitted by pressing ui_text_submit action (by default: Enter or Kp Enter).

### expand_to_text_length {#expand_to_text_length}

```gdscript
expand_to_text_length: bool
```

- void set_expand_to_text_length_enabled(value: bool)
- bool is_expand_to_text_length_enabled()

If true, the LineEdit width will increase to stay longer than the text. It will not compress if the text is shortened.

### context_menu_enabled {#context_menu_enabled}

```gdscript
context_menu_enabled: bool
```

- void set_context_menu_enabled(value: bool)
- bool is_context_menu_enabled()

If true, the context menu will appear when right-clicked.

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

### clear_button_enabled {#clear_button_enabled}

```gdscript
clear_button_enabled: bool
```

- void set_clear_button_enabled(value: bool)
- bool is_clear_button_enabled()

If true, the LineEdit will show a clear button if text is not empty, which can be used to clear the text quickly.

### shortcut_keys_enabled {#shortcut_keys_enabled}

```gdscript
shortcut_keys_enabled: bool
```

- void set_shortcut_keys_enabled(value: bool)
- bool is_shortcut_keys_enabled()

If true, shortcut keys for context menu items are enabled, even if the context menu is disabled.

### middle_mouse_paste_enabled {#middle_mouse_paste_enabled}

```gdscript
middle_mouse_paste_enabled: bool
```

- void set_middle_mouse_paste_enabled(value: bool)
- bool is_middle_mouse_paste_enabled()

If false, using middle mouse button to paste clipboard will be disabled.

Note: This method is only implemented on Linux.

### selecting_enabled {#selecting_enabled}

```gdscript
selecting_enabled: bool
```

- void set_selecting_enabled(value: bool)
- bool is_selecting_enabled()

If false, it's impossible to select the text using mouse nor keyboard.

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

### flat {#flat}

```gdscript
flat: bool
```

- void set_flat(value: bool)
- bool is_flat()

If true, the LineEdit doesn't display decoration.

### draw_control_chars {#draw_control_chars}

```gdscript
draw_control_chars: bool
```

- void set_draw_control_chars(value: bool)
- bool get_draw_control_chars()

If true, control characters are displayed.

### select_all_on_focus {#select_all_on_focus}

```gdscript
select_all_on_focus: bool
```

- void set_select_all_on_focus(value: bool)
- bool is_select_all_on_focus()

If true, the LineEdit will select the whole text when it gains focus.

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

### virtual_keyboard_type {#virtual_keyboard_type}

```gdscript
virtual_keyboard_type: int
```

- void set_virtual_keyboard_type(value: VirtualKeyboardType)
- VirtualKeyboardType get_virtual_keyboard_type()

Specifies the type of virtual keyboard to show.

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

### caret_column {#caret_column}

```gdscript
caret_column: int
```

- void set_caret_column(value: int)
- int get_caret_column()

The caret's column position inside the LineEdit. When set, the text may scroll to accommodate it.

### caret_force_displayed {#caret_force_displayed}

```gdscript
caret_force_displayed: bool
```

- void set_caret_force_displayed(value: bool)
- bool is_caret_force_displayed()

If true, the LineEdit will always show the caret, even if not editing or focus is lost.

### caret_mid_grapheme {#caret_mid_grapheme}

```gdscript
caret_mid_grapheme: bool
```

- void set_caret_mid_grapheme_enabled(value: bool)
- bool is_caret_mid_grapheme_enabled()

Allow moving caret, selecting and removing the individual composite character components.

Note: Backspace is always removing individual composite character components.

### secret {#secret}

```gdscript
secret: bool
```

- void set_secret(value: bool)
- bool is_secret()

If true, every character is replaced with the secret character (see secret_character).

### secret_character {#secret_character}

```gdscript
secret_character: String
```

- void set_secret_character(value: String)
- String get_secret_character()

The character to use to mask secret input. Only a single character can be used as the secret character. If it is longer than one character, only the first one will be used. If it is empty, a space will be used instead.

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

### right_icon {#right_icon}

```gdscript
right_icon: Texture2D
```

- void set_right_icon(value: Texture2D)
- Texture2D get_right_icon()

Sets the icon that will appear in the right end of the LineEdit if there's no text, or always, if clear_button_enabled is set to false.

### icon_expand_mode {#icon_expand_mode}

```gdscript
icon_expand_mode: int
```

- void set_icon_expand_mode(value: ExpandMode)
- ExpandMode get_icon_expand_mode()

Define the scaling behavior of the right_icon.

### right_icon_scale {#right_icon_scale}

```gdscript
right_icon_scale: float
```

- void set_right_icon_scale(value: float)
- float get_right_icon_scale()

Scale ratio of the icon when icon_expand_mode is set to EXPAND_MODE_FIT_TO_LINE_EDIT.

## 信号

### text_changed {#text_changed}

```gdscript
signal text_changed(new_text: String)
```

Emitted when the text changes.

### text_change_rejected {#text_change_rejected}

```gdscript
signal text_change_rejected(rejected_substring: String)
```

Emitted when appending text that overflows the max_length. The appended text is truncated to fit max_length, and the part that couldn't fit is passed as the rejected_substring argument.

### text_submitted {#text_submitted}

```gdscript
signal text_submitted(new_text: String)
```

Emitted when the user presses the ui_text_submit action (by default: Enter or Kp Enter) while the LineEdit has focus.

### editing_toggled {#editing_toggled}

```gdscript
signal editing_toggled(toggled_on: bool)
```

Emitted when the LineEdit switches in or out of edit mode.

## 枚举

### MenuItems {#enum-MenuItems}

```gdscript
enum MenuItems { MENU_CUT = 0, MENU_COPY = 1, MENU_PASTE = 2, MENU_CLEAR = 3, MENU_SELECT_ALL = 4, MENU_UNDO = 5, MENU_REDO = 6, MENU_SUBMENU_TEXT_DIR = 7, MENU_DIR_INHERITED = 8, MENU_DIR_AUTO = 9, MENU_DIR_LTR = 10, MENU_DIR_RTL = 11, MENU_DISPLAY_UCC = 12, MENU_SUBMENU_INSERT_UCC = 13, MENU_INSERT_LRM = 14, MENU_INSERT_RLM = 15, MENU_INSERT_LRE = 16, MENU_INSERT_RLE = 17, MENU_INSERT_LRO = 18, MENU_INSERT_RLO = 19, MENU_INSERT_PDF = 20, MENU_INSERT_ALM = 21, MENU_INSERT_LRI = 22, MENU_INSERT_RLI = 23, MENU_INSERT_FSI = 24, MENU_INSERT_PDI = 25, MENU_INSERT_ZWJ = 26, MENU_INSERT_ZWNJ = 27, MENU_INSERT_WJ = 28, MENU_INSERT_SHY = 29, MENU_EMOJI_AND_SYMBOL = 30, MENU_MAX = 31 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MENU_CUT {#MENU_CUT}

```gdscript
MENU_CUT = 0
```

Cuts (copies and clears) the selected text.

#### MENU_COPY {#MENU_COPY}

```gdscript
MENU_COPY = 1
```

Copies the selected text.

#### MENU_PASTE {#MENU_PASTE}

```gdscript
MENU_PASTE = 2
```

Pastes the clipboard text over the selected text (or at the caret's position).

Non-printable escape characters are automatically stripped from the OS clipboard via String.strip_escapes().

#### MENU_CLEAR {#MENU_CLEAR}

```gdscript
MENU_CLEAR = 3
```

Erases the whole LineEdit text.

#### MENU_SELECT_ALL {#MENU_SELECT_ALL}

```gdscript
MENU_SELECT_ALL = 4
```

Selects the whole LineEdit text.

#### MENU_UNDO {#MENU_UNDO}

```gdscript
MENU_UNDO = 5
```

Undoes the previous action.

#### MENU_REDO {#MENU_REDO}

```gdscript
MENU_REDO = 6
```

Reverse the last undo action.

#### MENU_SUBMENU_TEXT_DIR {#MENU_SUBMENU_TEXT_DIR}

```gdscript
MENU_SUBMENU_TEXT_DIR = 7
```

ID of "Text Writing Direction" submenu.

#### MENU_DIR_INHERITED {#MENU_DIR_INHERITED}

```gdscript
MENU_DIR_INHERITED = 8
```

Sets text direction to inherited.

#### MENU_DIR_AUTO {#MENU_DIR_AUTO}

```gdscript
MENU_DIR_AUTO = 9
```

Sets text direction to automatic.

#### MENU_DIR_LTR {#MENU_DIR_LTR}

```gdscript
MENU_DIR_LTR = 10
```

Sets text direction to left-to-right.

#### MENU_DIR_RTL {#MENU_DIR_RTL}

```gdscript
MENU_DIR_RTL = 11
```

Sets text direction to right-to-left.

#### MENU_DISPLAY_UCC {#MENU_DISPLAY_UCC}

```gdscript
MENU_DISPLAY_UCC = 12
```

Toggles control character display.

#### MENU_SUBMENU_INSERT_UCC {#MENU_SUBMENU_INSERT_UCC}

```gdscript
MENU_SUBMENU_INSERT_UCC = 13
```

ID of "Insert Control Character" submenu.

#### MENU_INSERT_LRM {#MENU_INSERT_LRM}

```gdscript
MENU_INSERT_LRM = 14
```

Inserts left-to-right mark (LRM) character.

#### MENU_INSERT_RLM {#MENU_INSERT_RLM}

```gdscript
MENU_INSERT_RLM = 15
```

Inserts right-to-left mark (RLM) character.

#### MENU_INSERT_LRE {#MENU_INSERT_LRE}

```gdscript
MENU_INSERT_LRE = 16
```

Inserts start of left-to-right embedding (LRE) character.

#### MENU_INSERT_RLE {#MENU_INSERT_RLE}

```gdscript
MENU_INSERT_RLE = 17
```

Inserts start of right-to-left embedding (RLE) character.

#### MENU_INSERT_LRO {#MENU_INSERT_LRO}

```gdscript
MENU_INSERT_LRO = 18
```

Inserts start of left-to-right override (LRO) character.

#### MENU_INSERT_RLO {#MENU_INSERT_RLO}

```gdscript
MENU_INSERT_RLO = 19
```

Inserts start of right-to-left override (RLO) character.

#### MENU_INSERT_PDF {#MENU_INSERT_PDF}

```gdscript
MENU_INSERT_PDF = 20
```

Inserts pop direction formatting (PDF) character.

#### MENU_INSERT_ALM {#MENU_INSERT_ALM}

```gdscript
MENU_INSERT_ALM = 21
```

Inserts Arabic letter mark (ALM) character.

#### MENU_INSERT_LRI {#MENU_INSERT_LRI}

```gdscript
MENU_INSERT_LRI = 22
```

Inserts left-to-right isolate (LRI) character.

#### MENU_INSERT_RLI {#MENU_INSERT_RLI}

```gdscript
MENU_INSERT_RLI = 23
```

Inserts right-to-left isolate (RLI) character.

#### MENU_INSERT_FSI {#MENU_INSERT_FSI}

```gdscript
MENU_INSERT_FSI = 24
```

Inserts first strong isolate (FSI) character.

#### MENU_INSERT_PDI {#MENU_INSERT_PDI}

```gdscript
MENU_INSERT_PDI = 25
```

Inserts pop direction isolate (PDI) character.

#### MENU_INSERT_ZWJ {#MENU_INSERT_ZWJ}

```gdscript
MENU_INSERT_ZWJ = 26
```

Inserts zero width joiner (ZWJ) character.

#### MENU_INSERT_ZWNJ {#MENU_INSERT_ZWNJ}

```gdscript
MENU_INSERT_ZWNJ = 27
```

Inserts zero width non-joiner (ZWNJ) character.

#### MENU_INSERT_WJ {#MENU_INSERT_WJ}

```gdscript
MENU_INSERT_WJ = 28
```

Inserts word joiner (WJ) character.

#### MENU_INSERT_SHY {#MENU_INSERT_SHY}

```gdscript
MENU_INSERT_SHY = 29
```

Inserts soft hyphen (SHY) character.

#### MENU_EMOJI_AND_SYMBOL {#MENU_EMOJI_AND_SYMBOL}

```gdscript
MENU_EMOJI_AND_SYMBOL = 30
```

Opens system emoji and symbol picker.

#### MENU_MAX {#MENU_MAX}

```gdscript
MENU_MAX = 31
```

Represents the size of the MenuItems enum.

---

### VirtualKeyboardType {#enum-VirtualKeyboardType}

```gdscript
enum VirtualKeyboardType { KEYBOARD_TYPE_DEFAULT = 0, KEYBOARD_TYPE_MULTILINE = 1, KEYBOARD_TYPE_NUMBER = 2, KEYBOARD_TYPE_NUMBER_DECIMAL = 3, KEYBOARD_TYPE_PHONE = 4, KEYBOARD_TYPE_EMAIL_ADDRESS = 5, KEYBOARD_TYPE_PASSWORD = 6, KEYBOARD_TYPE_URL = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KEYBOARD_TYPE_DEFAULT {#KEYBOARD_TYPE_DEFAULT}

```gdscript
KEYBOARD_TYPE_DEFAULT = 0
```

Default text virtual keyboard.

#### KEYBOARD_TYPE_MULTILINE {#KEYBOARD_TYPE_MULTILINE}

```gdscript
KEYBOARD_TYPE_MULTILINE = 1
```

Multiline virtual keyboard.

#### KEYBOARD_TYPE_NUMBER {#KEYBOARD_TYPE_NUMBER}

```gdscript
KEYBOARD_TYPE_NUMBER = 2
```

Virtual number keypad, useful for PIN entry.

#### KEYBOARD_TYPE_NUMBER_DECIMAL {#KEYBOARD_TYPE_NUMBER_DECIMAL}

```gdscript
KEYBOARD_TYPE_NUMBER_DECIMAL = 3
```

Virtual number keypad, useful for entering fractional numbers.

#### KEYBOARD_TYPE_PHONE {#KEYBOARD_TYPE_PHONE}

```gdscript
KEYBOARD_TYPE_PHONE = 4
```

Virtual phone number keypad.

#### KEYBOARD_TYPE_EMAIL_ADDRESS {#KEYBOARD_TYPE_EMAIL_ADDRESS}

```gdscript
KEYBOARD_TYPE_EMAIL_ADDRESS = 5
```

Virtual keyboard with additional keys to assist with typing email addresses.

#### KEYBOARD_TYPE_PASSWORD {#KEYBOARD_TYPE_PASSWORD}

```gdscript
KEYBOARD_TYPE_PASSWORD = 6
```

Virtual keyboard for entering a password. On most platforms, this should disable autocomplete and autocapitalization.

Note: This is not supported on Web. Instead, this behaves identically to KEYBOARD_TYPE_DEFAULT.

#### KEYBOARD_TYPE_URL {#KEYBOARD_TYPE_URL}

```gdscript
KEYBOARD_TYPE_URL = 7
```

Virtual keyboard with additional keys to assist with typing URLs.

---

### ExpandMode {#enum-ExpandMode}

```gdscript
enum ExpandMode { EXPAND_MODE_ORIGINAL_SIZE = 0, EXPAND_MODE_FIT_TO_TEXT = 1, EXPAND_MODE_FIT_TO_LINE_EDIT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPAND_MODE_ORIGINAL_SIZE {#EXPAND_MODE_ORIGINAL_SIZE}

```gdscript
EXPAND_MODE_ORIGINAL_SIZE = 0
```

Use the original size for the right icon.

#### EXPAND_MODE_FIT_TO_TEXT {#EXPAND_MODE_FIT_TO_TEXT}

```gdscript
EXPAND_MODE_FIT_TO_TEXT = 1
```

Scale the right icon's size to match the size of the text.

#### EXPAND_MODE_FIT_TO_LINE_EDIT {#EXPAND_MODE_FIT_TO_LINE_EDIT}

```gdscript
EXPAND_MODE_FIT_TO_LINE_EDIT = 2
```

Scale the right icon to fit the LineEdit.

---
