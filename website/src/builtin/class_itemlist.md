# ItemList

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ItemList**

> This control provides a vertical list of selectable items that may be in a single or in multiple columns, with each item having options for text and an icon. Tooltips are supported and may be different for every item in the list.

<details><summary>官方英文描述（中文精译进行中）</summary>

This control provides a vertical list of selectable items that may be in a single or in multiple columns, with each item having options for text and an icon. Tooltips are supported and may be different for every item in the list.

Selectable items in the list may be selected or deselected and multiple selection may be enabled. Selection with right mouse button may also be enabled to allow use of popup context menus. Items may also be "activated" by double-clicking them or by pressing Enter.

Item text only supports single-line strings. Newline characters (e.g. \n) in the string won't produce a newline. Text wrapping is enabled in ICON_MODE_TOP mode, but the column's width is adjusted to fully fit its content by default. You need to set fixed_column_width greater than zero to wrap the text.

All set_* methods allow negative item indices, i.e. -1 to access the last item, -2 to select the second-to-last item, and so on.

Incremental search: Like PopupMenu and Tree, ItemList supports searching within the list while the control is focused. Press a key that matches the first letter of an item's name to select the first item starting with the given letter. After that point, there are two ways to perform incremental search: 1) Press the same key again before the timeout duration to select the next item starting with the same letter. 2) Press letter keys that match the rest of the word before the timeout duration to match to select the item in question directly. Both of these actions will be reset to the beginning of the list if the timeout duration has passed since the last keystroke was registered. You can adjust the timeout duration by changing ProjectSettings.gui/timers/incremental_search_max_interval_msec.

</details>

## 方法

### add_icon_item {#add_icon_item}

```gdscript
int add_icon_item(icon: Texture2D, selectable: bool = true)
```

Adds an item to the item list with no text, only an icon. Returns the index of an added item.

### add_item {#add_item}

```gdscript
int add_item(text: String, icon: Texture2D = null, selectable: bool = true)
```

Adds an item to the item list with specified text. Returns the index of an added item.

Specify an icon, or use null as the icon for a list item with no icon.

If selectable is true, the list item will be selectable.

### center_on_current {#center_on_current}

```gdscript
void center_on_current(center_verically: bool = true, center_horizontally: bool = true)
```

Ensures the currently selected item (the first selected item if multiple selection is enabled) is visible, adjusting the scroll position as necessary to place the item at the center of the list if possible. See also ensure_current_is_visible().

Fails and prints an error if both arguments are false.

### clear {#clear}

```gdscript
void clear()
```

Removes all items from the list.

### deselect {#deselect}

```gdscript
void deselect(idx: int)
```

Ensures the item associated with the specified index is not selected.

### deselect_all {#deselect_all}

```gdscript
void deselect_all()
```

Ensures there are no items selected.

### ensure_current_is_visible {#ensure_current_is_visible}

```gdscript
void ensure_current_is_visible()
```

Ensures the currently selected item (the first selected item if multiple selection is enabled) is visible, adjusting the scroll position as necessary. See also center_on_current().

### force_update_list_size {#force_update_list_size}

```gdscript
void force_update_list_size()
```

Forces an update to the list size based on its items. This happens automatically whenever size of the items, or other relevant settings like auto_height, change. The method can be used to trigger the update ahead of next drawing pass.

### get_allow_reselect {#get_allow_reselect}

```gdscript
bool get_allow_reselect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_allow_rmb_select {#get_allow_rmb_select}

```gdscript
bool get_allow_rmb_select() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_allow_search {#get_allow_search}

```gdscript
bool get_allow_search() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fixed_column_width {#get_fixed_column_width}

```gdscript
int get_fixed_column_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fixed_icon_size {#get_fixed_icon_size}

```gdscript
Vector2i get_fixed_icon_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_h_scroll_bar {#get_h_scroll_bar}

```gdscript
HScrollBar get_h_scroll_bar()
```

Returns the horizontal scrollbar.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_icon_mode {#get_icon_mode}

```gdscript
ItemList.IconMode get_icon_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_icon_scale {#get_icon_scale}

```gdscript
float get_icon_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_item_at_position {#get_item_at_position}

```gdscript
int get_item_at_position(position: Vector2, exact: bool = false) const
```

Returns the item index at the given position.

When there is no item at that point, -1 will be returned if exact is true, and the closest item index will be returned otherwise.

Note: The returned value is unreliable if called right after modifying the ItemList, before it redraws in the next frame.

### get_item_auto_translate_mode {#get_item_auto_translate_mode}

```gdscript
Node.AutoTranslateMode get_item_auto_translate_mode(idx: int) const
```

Returns item's auto translate mode.

### get_item_count {#get_item_count}

```gdscript
int get_item_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_item_custom_bg_color {#get_item_custom_bg_color}

```gdscript
Color get_item_custom_bg_color(idx: int) const
```

Returns the custom background color of the item specified by idx index.

### get_item_custom_fg_color {#get_item_custom_fg_color}

```gdscript
Color get_item_custom_fg_color(idx: int) const
```

Returns the custom foreground color of the item specified by idx index.

### get_item_icon {#get_item_icon}

```gdscript
Texture2D get_item_icon(idx: int) const
```

Returns the icon associated with the specified index.

### get_item_icon_modulate {#get_item_icon_modulate}

```gdscript
Color get_item_icon_modulate(idx: int) const
```

Returns a Color modulating item's icon at the specified index.

### get_item_icon_region {#get_item_icon_region}

```gdscript
Rect2 get_item_icon_region(idx: int) const
```

Returns the region of item's icon used. The whole icon will be used if the region has no area.

### get_item_language {#get_item_language}

```gdscript
String get_item_language(idx: int) const
```

Returns item's text language code.

### get_item_metadata {#get_item_metadata}

```gdscript
Variant get_item_metadata(idx: int) const
```

Returns the metadata value of the specified index.

### get_item_rect {#get_item_rect}

```gdscript
Rect2 get_item_rect(idx: int, expand: bool = true) const
```

Returns the position and size of the item with the specified index, in the coordinate system of the ItemList node. If expand is true the last column expands to fill the rest of the row.

Note: The returned value is unreliable if called right after modifying the ItemList, before it redraws in the next frame.

### get_item_text {#get_item_text}

```gdscript
String get_item_text(idx: int) const
```

Returns the text associated with the specified index.

### get_item_text_direction {#get_item_text_direction}

```gdscript
Control.TextDirection get_item_text_direction(idx: int) const
```

Returns item's text base writing direction.

### get_item_tooltip {#get_item_tooltip}

```gdscript
String get_item_tooltip(idx: int) const
```

Returns the tooltip hint associated with the specified index.

### get_max_columns {#get_max_columns}

```gdscript
int get_max_columns() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_text_lines {#get_max_text_lines}

```gdscript
int get_max_text_lines() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scroll_hint_mode {#get_scroll_hint_mode}

```gdscript
ItemList.ScrollHintMode get_scroll_hint_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_select_mode {#get_select_mode}

```gdscript
ItemList.SelectMode get_select_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_selected_items {#get_selected_items}

```gdscript
PackedInt32Array get_selected_items()
```

Returns an array with the indexes of the selected items.

### get_text_overrun_behavior {#get_text_overrun_behavior}

```gdscript
TextServer.OverrunBehavior get_text_overrun_behavior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_v_scroll_bar {#get_v_scroll_bar}

```gdscript
VScrollBar get_v_scroll_bar()
```

Returns the vertical scrollbar.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### has_auto_height {#has_auto_height}

```gdscript
bool has_auto_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_auto_width {#has_auto_width}

```gdscript
bool has_auto_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_wraparound_items {#has_wraparound_items}

```gdscript
bool has_wraparound_items() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_anything_selected {#is_anything_selected}

```gdscript
bool is_anything_selected()
```

Returns true if one or more items are selected.

### is_item_disabled {#is_item_disabled}

```gdscript
bool is_item_disabled(idx: int) const
```

Returns true if the item at the specified index is disabled.

### is_item_icon_transposed {#is_item_icon_transposed}

```gdscript
bool is_item_icon_transposed(idx: int) const
```

Returns true if the item icon will be drawn transposed, i.e. the X and Y axes are swapped.

### is_item_selectable {#is_item_selectable}

```gdscript
bool is_item_selectable(idx: int) const
```

Returns true if the item at the specified index is selectable.

### is_item_tooltip_enabled {#is_item_tooltip_enabled}

```gdscript
bool is_item_tooltip_enabled(idx: int) const
```

Returns true if the tooltip is enabled for specified item index.

### is_same_column_width {#is_same_column_width}

```gdscript
bool is_same_column_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scroll_hint_tiled {#is_scroll_hint_tiled}

```gdscript
bool is_scroll_hint_tiled()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_selected {#is_selected}

```gdscript
bool is_selected(idx: int) const
```

Returns true if the item at the specified index is currently selected.

### move_item {#move_item}

```gdscript
void move_item(from_idx: int, to_idx: int)
```

Moves item from index from_idx to to_idx.

### remove_item {#remove_item}

```gdscript
void remove_item(idx: int)
```

Removes the item specified by idx index from the list.

### select {#select}

```gdscript
void select(idx: int, single: bool = true)
```

Selects the item at the specified index.

Note: This method does not trigger the item selection signal.

### set_allow_reselect {#set_allow_reselect}

```gdscript
void set_allow_reselect(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_rmb_select {#set_allow_rmb_select}

```gdscript
void set_allow_rmb_select(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_allow_search {#set_allow_search}

```gdscript
void set_allow_search(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_height {#set_auto_height}

```gdscript
void set_auto_height(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auto_width {#set_auto_width}

```gdscript
void set_auto_width(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fixed_column_width {#set_fixed_column_width}

```gdscript
void set_fixed_column_width(width: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fixed_icon_size {#set_fixed_icon_size}

```gdscript
void set_fixed_icon_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_icon_mode {#set_icon_mode}

```gdscript
void set_icon_mode(mode: ItemList.IconMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_icon_scale {#set_icon_scale}

```gdscript
void set_icon_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_item_auto_translate_mode {#set_item_auto_translate_mode}

```gdscript
void set_item_auto_translate_mode(idx: int, mode: Node.AutoTranslateMode)
```

Sets the auto translate mode of the item associated with the specified index.

Items use Node.AUTO_TRANSLATE_MODE_INHERIT by default, which uses the same auto translate mode as the ItemList itself.

### set_item_count {#set_item_count}

```gdscript
void set_item_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_item_custom_bg_color {#set_item_custom_bg_color}

```gdscript
void set_item_custom_bg_color(idx: int, custom_bg_color: Color)
```

Sets the background color of the item specified by idx index to the specified Color.

### set_item_custom_fg_color {#set_item_custom_fg_color}

```gdscript
void set_item_custom_fg_color(idx: int, custom_fg_color: Color)
```

Sets the foreground color of the item specified by idx index to the specified Color.

### set_item_disabled {#set_item_disabled}

```gdscript
void set_item_disabled(idx: int, disabled: bool)
```

Disables (or enables) the item at the specified index.

Disabled items cannot be selected and do not trigger activation signals (when double-clicking or pressing Enter).

### set_item_icon {#set_item_icon}

```gdscript
void set_item_icon(idx: int, icon: Texture2D)
```

Sets (or replaces) the icon's Texture2D associated with the specified index.

### set_item_icon_modulate {#set_item_icon_modulate}

```gdscript
void set_item_icon_modulate(idx: int, modulate: Color)
```

Sets a modulating Color of the item associated with the specified index.

### set_item_icon_region {#set_item_icon_region}

```gdscript
void set_item_icon_region(idx: int, rect: Rect2)
```

Sets the region of item's icon used. The whole icon will be used if the region has no area.

### set_item_icon_transposed {#set_item_icon_transposed}

```gdscript
void set_item_icon_transposed(idx: int, transposed: bool)
```

Sets whether the item icon will be drawn transposed.

### set_item_language {#set_item_language}

```gdscript
void set_item_language(idx: int, language: String)
```

Sets the language code of the text for the item at the given index to language. This is used for line-breaking and text shaping algorithms. If language is empty, the current locale is used.

### set_item_metadata {#set_item_metadata}

```gdscript
void set_item_metadata(idx: int, metadata: Variant)
```

Sets a value (of any type) to be stored with the item associated with the specified index.

### set_item_selectable {#set_item_selectable}

```gdscript
void set_item_selectable(idx: int, selectable: bool)
```

Allows or disallows selection of the item associated with the specified index.

### set_item_text {#set_item_text}

```gdscript
void set_item_text(idx: int, text: String)
```

Sets text of the item associated with the specified index.

### set_item_text_direction {#set_item_text_direction}

```gdscript
void set_item_text_direction(idx: int, direction: Control.TextDirection)
```

Sets item's text base writing direction.

### set_item_tooltip {#set_item_tooltip}

```gdscript
void set_item_tooltip(idx: int, tooltip: String)
```

Sets the tooltip hint for the item associated with the specified index.

### set_item_tooltip_enabled {#set_item_tooltip_enabled}

```gdscript
void set_item_tooltip_enabled(idx: int, enable: bool)
```

Sets whether the tooltip hint is enabled for specified item index.

### set_max_columns {#set_max_columns}

```gdscript
void set_max_columns(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_text_lines {#set_max_text_lines}

```gdscript
void set_max_text_lines(lines: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_same_column_width {#set_same_column_width}

```gdscript
void set_same_column_width(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_hint_mode {#set_scroll_hint_mode}

```gdscript
void set_scroll_hint_mode(scroll_hint_mode: ItemList.ScrollHintMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_select_mode {#set_select_mode}

```gdscript
void set_select_mode(mode: ItemList.SelectMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_overrun_behavior {#set_text_overrun_behavior}

```gdscript
void set_text_overrun_behavior(overrun_behavior: TextServer.OverrunBehavior)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tile_scroll_hint {#set_tile_scroll_hint}

```gdscript
void set_tile_scroll_hint(tile_scroll_hint: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wraparound_items {#set_wraparound_items}

```gdscript
void set_wraparound_items(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### sort_items_by_text {#sort_items_by_text}

```gdscript
void sort_items_by_text()
```

Sorts items in the list by their text.

## 属性

### select_mode {#select_mode}

```gdscript
select_mode: int
```

- void set_select_mode(value: SelectMode)
- SelectMode get_select_mode()

Allows single or multiple item selection. See the SelectMode constants.

### allow_reselect {#allow_reselect}

```gdscript
allow_reselect: bool
```

- void set_allow_reselect(value: bool)
- bool get_allow_reselect()

If true, the currently selected item can be selected again.

### allow_rmb_select {#allow_rmb_select}

```gdscript
allow_rmb_select: bool
```

- void set_allow_rmb_select(value: bool)
- bool get_allow_rmb_select()

If true, right mouse button click can select items.

### allow_search {#allow_search}

```gdscript
allow_search: bool
```

- void set_allow_search(value: bool)
- bool get_allow_search()

If true, allows navigating the ItemList with letter keys through incremental search.

### max_text_lines {#max_text_lines}

```gdscript
max_text_lines: int
```

- void set_max_text_lines(value: int)
- int get_max_text_lines()

Maximum lines of text allowed in each item. Space will be reserved even when there is not enough lines of text to display.

Note: This property takes effect only when icon_mode is ICON_MODE_TOP. To make the text wrap, fixed_column_width should be greater than zero.

### auto_width {#auto_width}

```gdscript
auto_width: bool
```

- void set_auto_width(value: bool)
- bool has_auto_width()

If true, the control will automatically resize the width to fit its content.

### auto_height {#auto_height}

```gdscript
auto_height: bool
```

- void set_auto_height(value: bool)
- bool has_auto_height()

If true, the control will automatically resize the height to fit its content.

### text_overrun_behavior {#text_overrun_behavior}

```gdscript
text_overrun_behavior: int
```

- void set_text_overrun_behavior(value: OverrunBehavior)
- OverrunBehavior get_text_overrun_behavior()

The clipping behavior when the text exceeds an item's bounding rectangle.

### wraparound_items {#wraparound_items}

```gdscript
wraparound_items: bool
```

- void set_wraparound_items(value: bool)
- bool has_wraparound_items()

If true, the control will automatically move items into a new row to fit its content. See also HFlowContainer for this behavior.

If false, the control will add a horizontal scrollbar to make all items visible.

### scroll_hint_mode {#scroll_hint_mode}

```gdscript
scroll_hint_mode: int
```

- void set_scroll_hint_mode(value: ScrollHintMode)
- ScrollHintMode get_scroll_hint_mode()

The way which scroll hints (indicators that show that the content can still be scrolled in a certain direction) will be shown.

### tile_scroll_hint {#tile_scroll_hint}

```gdscript
tile_scroll_hint: bool
```

- void set_tile_scroll_hint(value: bool)
- bool is_scroll_hint_tiled()

If true, the scroll hint texture will be tiled instead of stretched. See scroll_hint_mode.

### item_count {#item_count}

```gdscript
item_count: int
```

- void set_item_count(value: int)
- int get_item_count()

The number of items currently in the list.

### max_columns {#max_columns}

```gdscript
max_columns: int
```

- void set_max_columns(value: int)
- int get_max_columns()

Maximum columns the list will have.

If greater than zero, the content will be split among the specified columns.

A value of zero means unlimited columns, i.e. all items will be put in the same row.

### same_column_width {#same_column_width}

```gdscript
same_column_width: bool
```

- void set_same_column_width(value: bool)
- bool is_same_column_width()

Whether all columns will have the same width.

If true, the width is equal to the largest column width of all columns.

### fixed_column_width {#fixed_column_width}

```gdscript
fixed_column_width: int
```

- void set_fixed_column_width(value: int)
- int get_fixed_column_width()

The width all columns will be adjusted to.

A value of zero disables the adjustment, each item will have a width equal to the width of its content and the columns will have an uneven width.

### icon_mode {#icon_mode}

```gdscript
icon_mode: int
```

- void set_icon_mode(value: IconMode)
- IconMode get_icon_mode()

The icon position, whether above or to the left of the text. See the IconMode constants.

### icon_scale {#icon_scale}

```gdscript
icon_scale: float
```

- void set_icon_scale(value: float)
- float get_icon_scale()

The scale of icon applied after fixed_icon_size and transposing takes effect.

### fixed_icon_size {#fixed_icon_size}

```gdscript
fixed_icon_size: Vector2i
```

- void set_fixed_icon_size(value: Vector2i)
- Vector2i get_fixed_icon_size()

The size all icons will be adjusted to.

If either X or Y component is not greater than zero, icon size won't be affected.

## 信号

### item_selected {#item_selected}

```gdscript
signal item_selected(index: int)
```

Emitted when specified item has been selected. Only applicable in single selection mode.

allow_reselect must be enabled to reselect an item.

### empty_clicked {#empty_clicked}

```gdscript
signal empty_clicked(at_position: Vector2, mouse_button_index: int)
```

Emitted when any mouse click is issued within the rect of the list but on empty space.

at_position is the click position in this control's local coordinate system.

### item_clicked {#item_clicked}

```gdscript
signal item_clicked(index: int, at_position: Vector2, mouse_button_index: int)
```

Emitted when specified list item has been clicked with any mouse button.

at_position is the click position in this control's local coordinate system.

### multi_selected {#multi_selected}

```gdscript
signal multi_selected(index: int, selected: bool)
```

Emitted when a multiple selection is altered on a list allowing multiple selection.

### item_activated {#item_activated}

```gdscript
signal item_activated(index: int)
```

Emitted when specified list item is activated via double-clicking or by pressing Enter.

## 枚举

### IconMode {#enum-IconMode}

```gdscript
enum IconMode { ICON_MODE_TOP = 0, ICON_MODE_LEFT = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ICON_MODE_TOP {#ICON_MODE_TOP}

```gdscript
ICON_MODE_TOP = 0
```

Icon is drawn above the text.

#### ICON_MODE_LEFT {#ICON_MODE_LEFT}

```gdscript
ICON_MODE_LEFT = 1
```

Icon is drawn to the left of the text.

---

### SelectMode {#enum-SelectMode}

```gdscript
enum SelectMode { SELECT_SINGLE = 0, SELECT_MULTI = 1, SELECT_TOGGLE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SELECT_SINGLE {#SELECT_SINGLE}

```gdscript
SELECT_SINGLE = 0
```

Only allow selecting a single item.

#### SELECT_MULTI {#SELECT_MULTI}

```gdscript
SELECT_MULTI = 1
```

Allows selecting multiple items by holding Ctrl or Shift.

#### SELECT_TOGGLE {#SELECT_TOGGLE}

```gdscript
SELECT_TOGGLE = 2
```

Allows selecting multiple items by toggling them on and off.

---

### ScrollHintMode {#enum-ScrollHintMode}

```gdscript
enum ScrollHintMode { SCROLL_HINT_MODE_DISABLED = 0, SCROLL_HINT_MODE_BOTH = 1, SCROLL_HINT_MODE_TOP = 2, SCROLL_HINT_MODE_BOTTOM = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCROLL_HINT_MODE_DISABLED {#SCROLL_HINT_MODE_DISABLED}

```gdscript
SCROLL_HINT_MODE_DISABLED = 0
```

Scroll hints will never be shown.

#### SCROLL_HINT_MODE_BOTH {#SCROLL_HINT_MODE_BOTH}

```gdscript
SCROLL_HINT_MODE_BOTH = 1
```

Scroll hints will be shown at the top and bottom.

#### SCROLL_HINT_MODE_TOP {#SCROLL_HINT_MODE_TOP}

```gdscript
SCROLL_HINT_MODE_TOP = 2
```

Only the top scroll hint will be shown.

#### SCROLL_HINT_MODE_BOTTOM {#SCROLL_HINT_MODE_BOTTOM}

```gdscript
SCROLL_HINT_MODE_BOTTOM = 3
```

Only the bottom scroll hint will be shown.

---
