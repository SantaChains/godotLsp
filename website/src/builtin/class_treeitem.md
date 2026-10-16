# TreeItem

**继承**：[Object](class_object.md) → **TreeItem**

> A single item of a Tree control. It can contain other TreeItems as children, which allows it to create a hierarchy. It can also contain text and buttons. TreeItem is not a Node, it is internal to the Tree.

<details><summary>官方英文描述（中文精译进行中）</summary>

A single item of a Tree control. It can contain other TreeItems as children, which allows it to create a hierarchy. It can also contain text and buttons. TreeItem is not a Node, it is internal to the Tree.

To create a TreeItem, use Tree.create_item() or create_child(). To remove a TreeItem, use Object.free().

Note: The ID values used for buttons are 32-bit, unlike int which is always 64-bit. They go from -2147483648 to 2147483647.

</details>

## 方法

### add_button {#add_button}

```gdscript
void add_button(column: int, button: Texture2D, id: int = -1, disabled: bool = false, tooltip_text: String = "", description: String = "")
```

Adds a button with Texture2D button to the end of the cell at column column. The id is used to identify the button in the according Tree.button_clicked signal and can be different from the buttons index. If not specified, the next available index is used, which may be retrieved by calling get_button_count() immediately before this method. Optionally, the button can be disabled and have a tooltip_text. description is used as the button description for assistive apps.

### add_child {#add_child}

```gdscript
void add_child(child: TreeItem)
```

Adds a previously unparented TreeItem as a direct child of this one. The child item must not be a part of any Tree or parented to any TreeItem. See also remove_child().

### call_recursive {#call_recursive}

```gdscript
void call_recursive(method: StringName, ...)
```

Calls the method on the actual TreeItem and its children recursively. Pass parameters as a comma separated list.

### clear_buttons {#clear_buttons}

```gdscript
void clear_buttons()
```

Removes all buttons from all columns of this item.

### clear_custom_bg_color {#clear_custom_bg_color}

```gdscript
void clear_custom_bg_color(column: int)
```

Resets the background color for the given column to default.

### clear_custom_color {#clear_custom_color}

```gdscript
void clear_custom_color(column: int)
```

Resets the color for the given column to default.

### create_child {#create_child}

```gdscript
TreeItem create_child(index: int = -1)
```

Creates an item and adds it as a child.

The new item will be inserted as position index (the default value -1 means the last position), or it will be the last child if index is higher than the child count.

### deselect {#deselect}

```gdscript
void deselect(column: int)
```

Deselects the given column.

### erase_button {#erase_button}

```gdscript
void erase_button(column: int, button_index: int)
```

Removes the button at index button_index in column column.

### get_auto_translate_mode {#get_auto_translate_mode}

```gdscript
Node.AutoTranslateMode get_auto_translate_mode(column: int) const
```

Returns the column's auto translate mode.

### get_autowrap_mode {#get_autowrap_mode}

```gdscript
TextServer.AutowrapMode get_autowrap_mode(column: int) const
```

Returns the text autowrap mode in the given column. By default it is TextServer.AUTOWRAP_OFF.

### get_autowrap_trim_flags {#get_autowrap_trim_flags}

```gdscript
bitfield::TextServer.LineBreakFlag get_autowrap_trim_flags(column: int) const
```

Returns the autowrap trim flags for the given column. By default, both TextServer.BREAK_TRIM_START_EDGE_SPACES and TextServer.BREAK_TRIM_END_EDGE_SPACES are enabled.

### get_button {#get_button}

```gdscript
Texture2D get_button(column: int, button_index: int) const
```

Returns the Texture2D of the button at index button_index in column column.

### get_button_by_id {#get_button_by_id}

```gdscript
int get_button_by_id(column: int, id: int) const
```

Returns the button index if there is a button with ID id in column column, otherwise returns -1.

### get_button_color {#get_button_color}

```gdscript
Color get_button_color(column: int, id: int) const
```

Returns the color of the button with ID id in column column. If the specified button does not exist, returns Color.BLACK.

### get_button_count {#get_button_count}

```gdscript
int get_button_count(column: int) const
```

Returns the number of buttons in column column.

### get_button_id {#get_button_id}

```gdscript
int get_button_id(column: int, button_index: int) const
```

Returns the ID for the button at index button_index in column column.

### get_button_tooltip_text {#get_button_tooltip_text}

```gdscript
String get_button_tooltip_text(column: int, button_index: int) const
```

Returns the tooltip text for the button at index button_index in column column.

### get_cell_mode {#get_cell_mode}

```gdscript
TreeItem.TreeCellMode get_cell_mode(column: int) const
```

Returns the column's cell mode.

### get_child {#get_child}

```gdscript
TreeItem get_child(index: int)
```

Returns a child item by its index (see get_child_count()). This method is often used for iterating all children of an item.

Negative indices access the children from the last one.

### get_child_count {#get_child_count}

```gdscript
int get_child_count()
```

Returns the number of child items.

### get_children {#get_children}

```gdscript
typedarray::TreeItem get_children()
```

Returns an array of references to the item's children.

### get_custom_bg_color {#get_custom_bg_color}

```gdscript
Color get_custom_bg_color(column: int) const
```

Returns the custom background color of column column.

### get_custom_color {#get_custom_color}

```gdscript
Color get_custom_color(column: int) const
```

Returns the custom color of column column.

### get_custom_draw_callback {#get_custom_draw_callback}

```gdscript
Callable get_custom_draw_callback(column: int) const
```

Returns the custom callback of column column.

### get_custom_font {#get_custom_font}

```gdscript
Font get_custom_font(column: int) const
```

Returns custom font used to draw text in the column column.

### get_custom_font_size {#get_custom_font_size}

```gdscript
int get_custom_font_size(column: int) const
```

Returns custom font size used to draw text in the column column.

### get_custom_minimum_height {#get_custom_minimum_height}

```gdscript
int get_custom_minimum_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_stylebox {#get_custom_stylebox}

```gdscript
StyleBox get_custom_stylebox(column: int) const
```

Returns the given column's custom StyleBox used to draw the background.

### get_description {#get_description}

```gdscript
String get_description(column: int) const
```

Returns the given column's description for assistive apps.

### get_expand_right {#get_expand_right}

```gdscript
bool get_expand_right(column: int) const
```

Returns true if expand_right is set.

### get_first_child {#get_first_child}

```gdscript
TreeItem get_first_child() const
```

Returns the TreeItem's first child.

### get_icon {#get_icon}

```gdscript
Texture2D get_icon(column: int) const
```

Returns the given column's icon Texture2D. Error if no icon is set.

### get_icon_max_width {#get_icon_max_width}

```gdscript
int get_icon_max_width(column: int) const
```

Returns the maximum allowed width of the icon in the given column.

### get_icon_modulate {#get_icon_modulate}

```gdscript
Color get_icon_modulate(column: int) const
```

Returns the Color modulating the column's icon.

### get_icon_overlay {#get_icon_overlay}

```gdscript
Texture2D get_icon_overlay(column: int) const
```

Returns the given column's icon overlay Texture2D.

### get_icon_region {#get_icon_region}

```gdscript
Rect2 get_icon_region(column: int) const
```

Returns the icon Texture2D region as Rect2.

### get_index {#get_index}

```gdscript
int get_index()
```

Returns the node's order in the tree. For example, if called on the first child item the position is 0.

### get_language {#get_language}

```gdscript
String get_language(column: int) const
```

Returns item's text language code.

### get_metadata {#get_metadata}

```gdscript
Variant get_metadata(column: int) const
```

Returns the metadata value that was set for the given column using set_metadata().

### get_next {#get_next}

```gdscript
TreeItem get_next() const
```

Returns the next sibling TreeItem in the tree or a null object if there is none.

### get_next_in_tree {#get_next_in_tree}

```gdscript
TreeItem get_next_in_tree(wrap: bool = false)
```

Returns the next TreeItem in the tree (in the context of a depth-first search) or a null object if there is none.

If wrap is enabled, the method will wrap around to the first element in the tree when called on the last element, otherwise it returns null.

### get_next_visible {#get_next_visible}

```gdscript
TreeItem get_next_visible(wrap: bool = false)
```

Returns the next visible TreeItem in the tree (in the context of a depth-first search) or a null object if there is none.

If wrap is enabled, the method will wrap around to the first visible element in the tree when called on the last visible element, otherwise it returns null.

### get_parent {#get_parent}

```gdscript
TreeItem get_parent() const
```

Returns the parent TreeItem or a null object if there is none.

### get_prev {#get_prev}

```gdscript
TreeItem get_prev()
```

Returns the previous sibling TreeItem in the tree or a null object if there is none.

### get_prev_in_tree {#get_prev_in_tree}

```gdscript
TreeItem get_prev_in_tree(wrap: bool = false)
```

Returns the previous TreeItem in the tree (in the context of a depth-first search) or a null object if there is none.

If wrap is enabled, the method will wrap around to the last element in the tree when called on the first visible element, otherwise it returns null.

### get_prev_visible {#get_prev_visible}

```gdscript
TreeItem get_prev_visible(wrap: bool = false)
```

Returns the previous visible sibling TreeItem in the tree (in the context of a depth-first search) or a null object if there is none.

If wrap is enabled, the method will wrap around to the last visible element in the tree when called on the first visible element, otherwise it returns null.

### get_range {#get_range}

```gdscript
float get_range(column: int) const
```

Returns the value of a CELL_MODE_RANGE column.

### get_range_config {#get_range_config}

```gdscript
Dictionary get_range_config(column: int)
```

Returns a dictionary containing the range parameters for a given column. The keys are "min", "max", "step", and "expr".

### get_structured_text_bidi_override {#get_structured_text_bidi_override}

```gdscript
TextServer.StructuredTextParser get_structured_text_bidi_override(column: int) const
```

Returns the BiDi algorithm override set for this cell.

### get_structured_text_bidi_override_options {#get_structured_text_bidi_override_options}

```gdscript
Array get_structured_text_bidi_override_options(column: int) const
```

Returns the additional BiDi options set for this cell.

### get_suffix {#get_suffix}

```gdscript
String get_suffix(column: int) const
```

Gets the suffix string shown after the column value.

### get_text {#get_text}

```gdscript
String get_text(column: int) const
```

Returns the given column's text.

### get_text_alignment {#get_text_alignment}

```gdscript
HorizontalAlignment get_text_alignment(column: int) const
```

Returns the given column's text alignment.

### get_text_direction {#get_text_direction}

```gdscript
Control.TextDirection get_text_direction(column: int) const
```

Returns item's text base writing direction.

### get_text_overrun_behavior {#get_text_overrun_behavior}

```gdscript
TextServer.OverrunBehavior get_text_overrun_behavior(column: int) const
```

Returns the clipping behavior when the text exceeds the item's bounding rectangle in the given column. By default it is TextServer.OVERRUN_TRIM_ELLIPSIS.

### get_tooltip_text {#get_tooltip_text}

```gdscript
String get_tooltip_text(column: int) const
```

Returns the given column's tooltip text.

### get_tree {#get_tree}

```gdscript
Tree get_tree() const
```

Returns the Tree that owns this TreeItem.

### is_accepting_children {#is_accepting_children}

```gdscript
bool is_accepting_children() const
```

Returns true if this TreeItem is allowed to accept children.

### is_any_collapsed {#is_any_collapsed}

```gdscript
bool is_any_collapsed(only_visible: bool = false)
```

Returns true if this TreeItem, or any of its descendants, is collapsed.

If only_visible is true it ignores non-visible TreeItems.

### is_button_disabled {#is_button_disabled}

```gdscript
bool is_button_disabled(column: int, button_index: int) const
```

Returns true if the button at index button_index for the given column is disabled.

### is_checked {#is_checked}

```gdscript
bool is_checked(column: int) const
```

Returns true if the given column is checked.

### is_collapsed {#is_collapsed}

```gdscript
bool is_collapsed()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_custom_set_as_button {#is_custom_set_as_button}

```gdscript
bool is_custom_set_as_button(column: int) const
```

Returns true if the cell was made into a button with set_custom_as_button().

### is_edit_multiline {#is_edit_multiline}

```gdscript
bool is_edit_multiline(column: int) const
```

Returns true if the given column is multiline editable.

### is_editable {#is_editable}

```gdscript
bool is_editable(column: int)
```

Returns true if the given column is editable.

### is_folding_disabled {#is_folding_disabled}

```gdscript
bool is_folding_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_indeterminate {#is_indeterminate}

```gdscript
bool is_indeterminate(column: int) const
```

Returns true if the given column is indeterminate.

### is_selectable {#is_selectable}

```gdscript
bool is_selectable(column: int) const
```

Returns true if the given column is selectable.

### is_selected {#is_selected}

```gdscript
bool is_selected(column: int)
```

Returns true if the given column is selected.

### is_visible {#is_visible}

```gdscript
bool is_visible()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_visible_in_tree {#is_visible_in_tree}

```gdscript
bool is_visible_in_tree() const
```

Returns true if visible is true and all its ancestors are also visible.

### move_after {#move_after}

```gdscript
void move_after(item: TreeItem)
```

Moves this TreeItem right after the given item.

Note: You can't move to the root or move the root.

### move_before {#move_before}

```gdscript
void move_before(item: TreeItem)
```

Moves this TreeItem right before the given item.

Note: You can't move to the root or move the root.

### propagate_check {#propagate_check}

```gdscript
void propagate_check(column: int, emit_signal: bool = true)
```

Propagates this item's checked status to its children and parents for the given column. It is possible to process the items affected by this method call by connecting to Tree.check_propagated_to_item. The order that the items affected will be processed is as follows: the item invoking this method, children of that item, and finally parents of that item. If emit_signal is false, then Tree.check_propagated_to_item will not be emitted.

### remove_child {#remove_child}

```gdscript
void remove_child(child: TreeItem)
```

Removes the given child TreeItem and all its children from the Tree. Note that it doesn't free the item from memory, so it can be reused later (see add_child()). To completely remove a TreeItem use Object.free().

Note: If you want to move a child from one Tree to another, then instead of removing and adding it manually you can use move_before() or move_after().

### select {#select}

```gdscript
void select(column: int, set_as_cursor: bool = true)
```

Selects the given column. If set_as_cursor is true, the Tree's cursor will be moved to this item (only matters if Tree.select_mode is set to Tree.SELECT_MULTI).

### set_accept_children {#set_accept_children}

```gdscript
void set_accept_children(allowed: bool)
```

Sets TreeItem's ability to accept children.

### set_auto_translate_mode {#set_auto_translate_mode}

```gdscript
void set_auto_translate_mode(column: int, mode: Node.AutoTranslateMode)
```

Sets the given column's auto translate mode to mode.

All columns use Node.AUTO_TRANSLATE_MODE_INHERIT by default, which uses the same auto translate mode as the Tree itself.

### set_autowrap_mode {#set_autowrap_mode}

```gdscript
void set_autowrap_mode(column: int, autowrap_mode: TextServer.AutowrapMode)
```

Sets the autowrap mode in the given column. If set to something other than TextServer.AUTOWRAP_OFF, the text gets wrapped inside the cell's bounding rectangle.

### set_autowrap_trim_flags {#set_autowrap_trim_flags}

```gdscript
void set_autowrap_trim_flags(column: int, flags: bitfield::TextServer.LineBreakFlag)
```

Sets the autowrap trim flags for the given column. These flags control whether leading and trailing spaces are trimmed on wrapped lines. Set to 0 to disable all trimming.

### set_button {#set_button}

```gdscript
void set_button(column: int, button_index: int, button: Texture2D)
```

Sets the given column's button Texture2D at index button_index to button.

### set_button_color {#set_button_color}

```gdscript
void set_button_color(column: int, button_index: int, color: Color)
```

Sets the given column's button color at index button_index to color.

### set_button_description {#set_button_description}

```gdscript
void set_button_description(column: int, button_index: int, description: String)
```

Sets the given column's button description at index button_index for assistive apps.

### set_button_disabled {#set_button_disabled}

```gdscript
void set_button_disabled(column: int, button_index: int, disabled: bool)
```

If true, disables the button at index button_index in the given column.

### set_button_tooltip_text {#set_button_tooltip_text}

```gdscript
void set_button_tooltip_text(column: int, button_index: int, tooltip: String)
```

Sets the tooltip text for the button at index button_index in the given column.

### set_cell_mode {#set_cell_mode}

```gdscript
void set_cell_mode(column: int, mode: TreeItem.TreeCellMode)
```

Sets the given column's cell mode to mode. This determines how the cell is displayed and edited.

### set_checked {#set_checked}

```gdscript
void set_checked(column: int, checked: bool)
```

If checked is true, the given column is checked. Clears column's indeterminate status.

### set_collapsed {#set_collapsed}

```gdscript
void set_collapsed(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collapsed_recursive {#set_collapsed_recursive}

```gdscript
void set_collapsed_recursive(enable: bool)
```

Collapses or uncollapses this TreeItem and all the descendants of this item.

### set_custom_as_button {#set_custom_as_button}

```gdscript
void set_custom_as_button(column: int, enable: bool)
```

Makes a cell with CELL_MODE_CUSTOM display as a non-flat button with a StyleBox.

### set_custom_bg_color {#set_custom_bg_color}

```gdscript
void set_custom_bg_color(column: int, color: Color, just_outline: bool = false)
```

Sets the given column's custom background color and whether to just use it as an outline.

Note: If a custom StyleBox is set, the background color will be drawn behind it.

### set_custom_color {#set_custom_color}

```gdscript
void set_custom_color(column: int, color: Color)
```

Sets the given column's custom color.

### set_custom_draw {#set_custom_draw}

```gdscript
void set_custom_draw(column: int, object: Object, callback: StringName)
```

Deprecated: Use set_custom_draw_callback() instead.

Sets the given column's custom draw callback to the callback method on object.

The method named callback should accept two arguments: the TreeItem that is drawn and its position and size as a Rect2.

### set_custom_draw_callback {#set_custom_draw_callback}

```gdscript
void set_custom_draw_callback(column: int, callback: Callable)
```

Sets the given column's custom draw callback. Use an empty Callable (Callable()) to clear the custom callback. The cell has to be in CELL_MODE_CUSTOM to use this feature.

The callback should accept two arguments: the TreeItem that is drawn and its position and size as a Rect2.

To draw custom content over the native style, please use Tree.get_custom_drawing_canvas_item().

### set_custom_font {#set_custom_font}

```gdscript
void set_custom_font(column: int, font: Font)
```

Sets custom font used to draw text in the given column.

### set_custom_font_size {#set_custom_font_size}

```gdscript
void set_custom_font_size(column: int, font_size: int)
```

Sets custom font size used to draw text in the given column.

### set_custom_minimum_height {#set_custom_minimum_height}

```gdscript
void set_custom_minimum_height(height: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_stylebox {#set_custom_stylebox}

```gdscript
void set_custom_stylebox(column: int, stylebox: StyleBox)
```

Sets the given column's custom StyleBox used to draw the background.

Note: If a custom background color is set, the StyleBox will be drawn in front of it.

### set_description {#set_description}

```gdscript
void set_description(column: int, description: String)
```

Sets the given column's description for assistive apps.

### set_disable_folding {#set_disable_folding}

```gdscript
void set_disable_folding(disable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edit_multiline {#set_edit_multiline}

```gdscript
void set_edit_multiline(column: int, multiline: bool)
```

If multiline is true, the given column is multiline editable.

Note: This option only affects the type of control (LineEdit or TextEdit) that appears when editing the column. You can set multiline values with set_text() even if the column is not multiline editable.

### set_editable {#set_editable}

```gdscript
void set_editable(column: int, enabled: bool)
```

If enabled is true, the given column is editable.

### set_expand_right {#set_expand_right}

```gdscript
void set_expand_right(column: int, enable: bool)
```

If enable is true, the given column is expanded to the right.

### set_icon {#set_icon}

```gdscript
void set_icon(column: int, texture: Texture2D)
```

Sets the given cell's icon Texture2D. If the cell is in CELL_MODE_ICON mode, the icon is displayed in the center of the cell. Otherwise, the icon is displayed before the cell's text. CELL_MODE_RANGE does not display an icon.

### set_icon_max_width {#set_icon_max_width}

```gdscript
void set_icon_max_width(column: int, width: int)
```

Sets the maximum allowed width of the icon in the given column. This limit is applied on top of the default size of the icon and on top of Tree.icon_max_width. The height is adjusted according to the icon's ratio.

### set_icon_modulate {#set_icon_modulate}

```gdscript
void set_icon_modulate(column: int, modulate: Color)
```

Modulates the given column's icon with modulate.

### set_icon_overlay {#set_icon_overlay}

```gdscript
void set_icon_overlay(column: int, texture: Texture2D)
```

Sets the given cell's icon overlay Texture2D. The cell has to be in CELL_MODE_ICON mode, and icon has to be set. Overlay is drawn on top of icon, in the bottom left corner.

### set_icon_region {#set_icon_region}

```gdscript
void set_icon_region(column: int, region: Rect2)
```

Sets the given column's icon's texture region.

### set_indeterminate {#set_indeterminate}

```gdscript
void set_indeterminate(column: int, indeterminate: bool)
```

If indeterminate is true, the given column is marked indeterminate.

Note: If set true from false, then column is cleared of checked status.

### set_language {#set_language}

```gdscript
void set_language(column: int, language: String)
```

Sets the language code of the given column's text to language. This is used for line-breaking and text shaping algorithms. If language is empty, the current locale is used.

### set_metadata {#set_metadata}

```gdscript
void set_metadata(column: int, meta: Variant)
```

Sets the metadata value for the given column, which can be retrieved later using get_metadata(). This can be used, for example, to store a reference to the original data.

### set_range {#set_range}

```gdscript
void set_range(column: int, value: float)
```

Sets the value of a CELL_MODE_RANGE column.

### set_range_config {#set_range_config}

```gdscript
void set_range_config(column: int, min: float, max: float, step: float, expr: bool = false)
```

Sets the range of accepted values for a column. The column must be in the CELL_MODE_RANGE mode.

If expr is true, the edit mode slider will use an exponential scale as with Range.exp_edit.

### set_selectable {#set_selectable}

```gdscript
void set_selectable(column: int, selectable: bool)
```

If selectable is true, the given column is selectable.

### set_structured_text_bidi_override {#set_structured_text_bidi_override}

```gdscript
void set_structured_text_bidi_override(column: int, parser: TextServer.StructuredTextParser)
```

Set BiDi algorithm override for the structured text. Has effect for cells that display text.

### set_structured_text_bidi_override_options {#set_structured_text_bidi_override_options}

```gdscript
void set_structured_text_bidi_override_options(column: int, args: Array)
```

Set additional options for BiDi override. Has effect for cells that display text.

### set_suffix {#set_suffix}

```gdscript
void set_suffix(column: int, text: String)
```

Sets a string to be shown after a column's value (for example, a unit abbreviation).

### set_text {#set_text}

```gdscript
void set_text(column: int, text: String)
```

Sets the given column's text value.

### set_text_alignment {#set_text_alignment}

```gdscript
void set_text_alignment(column: int, text_alignment: HorizontalAlignment)
```

Sets the given column's text alignment to text_alignment.

### set_text_direction {#set_text_direction}

```gdscript
void set_text_direction(column: int, direction: Control.TextDirection)
```

Sets item's text base writing direction.

### set_text_overrun_behavior {#set_text_overrun_behavior}

```gdscript
void set_text_overrun_behavior(column: int, overrun_behavior: TextServer.OverrunBehavior)
```

Sets the clipping behavior when the text exceeds the item's bounding rectangle in the given column.

### set_tooltip_text {#set_tooltip_text}

```gdscript
void set_tooltip_text(column: int, tooltip: String)
```

Sets the given column's tooltip text.

### set_visible {#set_visible}

```gdscript
void set_visible(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### uncollapse_tree {#uncollapse_tree}

```gdscript
void uncollapse_tree()
```

Uncollapses all TreeItems necessary to reveal this TreeItem, i.e. all ancestor TreeItems.

## 属性

### collapsed {#collapsed}

```gdscript
collapsed: bool
```

- void set_collapsed(value: bool)
- bool is_collapsed()

If true, the TreeItem is collapsed.

### visible {#visible}

```gdscript
visible: bool
```

- void set_visible(value: bool)
- bool is_visible()

If true, the TreeItem is visible (default).

Note that if a TreeItem is set to not be visible, none of its children will be visible either.

### disable_folding {#disable_folding}

```gdscript
disable_folding: bool
```

- void set_disable_folding(value: bool)
- bool is_folding_disabled()

If true, folding is disabled for this TreeItem.

### custom_minimum_height {#custom_minimum_height}

```gdscript
custom_minimum_height: int
```

- void set_custom_minimum_height(value: int)
- int get_custom_minimum_height()

The custom minimum height.

## 枚举

### TreeCellMode {#enum-TreeCellMode}

```gdscript
enum TreeCellMode { CELL_MODE_STRING = 0, CELL_MODE_CHECK = 1, CELL_MODE_RANGE = 2, CELL_MODE_ICON = 3, CELL_MODE_CUSTOM = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CELL_MODE_STRING {#CELL_MODE_STRING}

```gdscript
CELL_MODE_STRING = 0
```

Cell shows a string label, optionally with an icon. When editable, the text can be edited using a LineEdit, or a TextEdit popup if set_edit_multiline() is used.

#### CELL_MODE_CHECK {#CELL_MODE_CHECK}

```gdscript
CELL_MODE_CHECK = 1
```

Cell shows a checkbox, optionally with text and an icon. The checkbox can be pressed, released, or indeterminate (via set_indeterminate()). The checkbox can't be clicked unless the cell is editable.

#### CELL_MODE_RANGE {#CELL_MODE_RANGE}

```gdscript
CELL_MODE_RANGE = 2
```

Cell shows a numeric range. When editable, it can be edited using a range slider. Use set_range() to set the value and set_range_config() to configure the range.

This cell can also be used in a text dropdown mode when you assign a text with set_text(). Separate options with a comma, e.g. "Option1,Option2,Option3".

#### CELL_MODE_ICON {#CELL_MODE_ICON}

```gdscript
CELL_MODE_ICON = 3
```

Cell shows an icon. It can't be edited nor display text. The icon is always centered within the cell.

#### CELL_MODE_CUSTOM {#CELL_MODE_CUSTOM}

```gdscript
CELL_MODE_CUSTOM = 4
```

Cell shows as a clickable button. It will display an arrow similar to OptionButton, but doesn't feature a dropdown (for that you can use CELL_MODE_RANGE). Clicking the button emits the Tree.item_edited signal. The button is flat by default, you can use set_custom_as_button() to display it with a StyleBox.

This mode also supports custom drawing using set_custom_draw_callback().

---
