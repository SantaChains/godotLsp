# Tree

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Tree**

> A control used to show a set of internal TreeItems in a hierarchical structure. The tree items can be selected, expanded and collapsed. The tree can have multiple columns with custom controls like LineEdits, buttons and popups. It can be useful for structured displays and interactions.

<details><summary>官方英文描述</summary>

A control used to show a set of internal TreeItems in a hierarchical structure. The tree items can be selected, expanded and collapsed. The tree can have multiple columns with custom controls like LineEdits, buttons and popups. It can be useful for structured displays and interactions.

Trees are built via code, using TreeItem objects to create the structure. They have a single root, but multiple roots can be simulated with hide_root:

```
func _ready():
    var tree = Tree.new()
    var root = tree.create_item()
    tree.hide_root = true
    var child1 = tree.create_item(root)
    var child2 = tree.create_item(root)
    var subchild1 = tree.create_item(child1)
    subchild1.set_text(0, "Subchild1")
```

```
public override void _Ready()
{
    var tree = new Tree();
    TreeItem root = tree.CreateItem();
    tree.HideRoot = true;
    TreeItem child1 = tree.CreateItem(root);
    TreeItem child2 = tree.CreateItem(root);
    TreeItem subchild1 = tree.CreateItem(child1);
    subchild1.SetText(0, "Subchild1");
}
```

To iterate over all the TreeItem objects in a Tree object, use TreeItem.get_next() and TreeItem.get_first_child() after getting the root through get_root(). You can use Object.free() on a TreeItem to remove it from the Tree.

Incremental search: Like ItemList and PopupMenu, Tree supports searching within the list while the control is focused. Press a key that matches the first letter of an item's name to select the first item starting with the given letter. After that point, there are two ways to perform incremental search: 1) Press the same key again before the timeout duration to select the next item starting with the same letter. 2) Press letter keys that match the rest of the word before the timeout duration to match to select the item in question directly. Both of these actions will be reset to the beginning of the list if the timeout duration has passed since the last keystroke was registered. You can adjust the timeout duration by changing ProjectSettings.gui/timers/incremental_search_max_interval_msec.

</details>

## 方法

### are_column_titles_visible {#are_column_titles_visible}

```gdscript
bool are_column_titles_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### clear {#clear}

```gdscript
void clear()
```

Clears the tree. This removes all items.

Prints an error and does not allow clearing the tree if called during mouse selection.

### create_item {#create_item}

```gdscript
TreeItem create_item(parent: TreeItem = null, index: int = -1)
```

Creates an item in the tree and adds it as a child of parent, which can be either a valid TreeItem or null.

If parent is null, the root item will be the parent, or the new item will be the root itself if the tree is empty.

The new item will be the index-th child of parent, or it will be the last child if there are not enough siblings.

Prints an error and returns null if called during mouse selection, or if the parent does not belong to this tree.

### deselect_all {#deselect_all}

```gdscript
void deselect_all()
```

Deselects all tree items (rows and columns). In SELECT_MULTI mode also removes selection cursor.

### edit_selected {#edit_selected}

```gdscript
bool edit_selected(force_edit: bool = false)
```

Edits the selected tree item as if it was clicked.

Either the item must be set editable with TreeItem.set_editable() or force_edit must be true.

Returns true if the item could be edited. Fails if no item is selected.

### ensure_cursor_is_visible {#ensure_cursor_is_visible}

```gdscript
void ensure_cursor_is_visible()
```

Makes the currently focused cell visible.

This will scroll the tree if necessary. In SELECT_ROW mode, this will not do horizontal scrolling, as all the cells in the selected row is focused logically.

Note: Despite the name of this method, the focus cursor itself is only visible in SELECT_MULTI mode.

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

### get_button_id_at_position {#get_button_id_at_position}

```gdscript
int get_button_id_at_position(position: Vector2) const
```

Returns the button ID at position, or -1 if no button is there.

### get_column_at_position {#get_column_at_position}

```gdscript
int get_column_at_position(position: Vector2) const
```

Returns the column index at position, or -1 if no item is there.

### get_column_expand_ratio {#get_column_expand_ratio}

```gdscript
int get_column_expand_ratio(column: int) const
```

Returns the expand ratio assigned to the column.

### get_column_title {#get_column_title}

```gdscript
String get_column_title(column: int) const
```

Returns the column's title.

### get_column_title_alignment {#get_column_title_alignment}

```gdscript
HorizontalAlignment get_column_title_alignment(column: int) const
```

Returns the column title alignment.

### get_column_title_direction {#get_column_title_direction}

```gdscript
Control.TextDirection get_column_title_direction(column: int) const
```

Returns column title base writing direction.

### get_column_title_language {#get_column_title_language}

```gdscript
String get_column_title_language(column: int) const
```

Returns column title language code.

### get_column_title_tooltip_text {#get_column_title_tooltip_text}

```gdscript
String get_column_title_tooltip_text(column: int) const
```

Returns the column title's tooltip text.

### get_column_width {#get_column_width}

```gdscript
int get_column_width(column: int) const
```

Returns the column's width in pixels.

### get_columns {#get_columns}

```gdscript
int get_columns() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_drawing_canvas_item {#get_custom_drawing_canvas_item}

```gdscript
RID get_custom_drawing_canvas_item() const
```

Returns the internal canvas item designated for custom drawing. See TreeItem.set_custom_draw_callback().

Note: This canvas item clears automatically on each Tree draw call.

### get_custom_popup_rect {#get_custom_popup_rect}

```gdscript
Rect2 get_custom_popup_rect() const
```

Returns the rectangle for custom popups. Helper to create custom cell controls that display a popup. See TreeItem.set_cell_mode().

### get_drop_mode_flags {#get_drop_mode_flags}

```gdscript
int get_drop_mode_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drop_section_at_position {#get_drop_section_at_position}

```gdscript
int get_drop_section_at_position(position: Vector2) const
```

Returns the drop section at position, as permitted by enabled DropModeFlags.

- -1 if the position is above the item. Typically used to insert as the item's previous sibling.
- 0 if the position is on the item. Typically used to insert as the item's last child.
- 1 if the position is below the item, when the item has no children. Typically used to insert as the item's next sibling. If the item does have children, this section is still reachable by hovering to the left of the item's collapse arrow, and below.
- 2 if the position is below the item, when the item has children. Typically used to insert as the item's first child.
- -100 if the position is not over any item, or no DropModeFlags are set.

See DropModeFlags for a description of each drop region. To get the item which the returned drop section refers to, use get_item_at_position().

### get_edited {#get_edited}

```gdscript
TreeItem get_edited() const
```

Returns the currently edited item. Can be used with item_edited to get the item that was modified.

```
func _ready():
    $Tree.item_edited.connect(on_Tree_item_edited)
func on_Tree_item_edited():
    print($Tree.get_edited()) # This item just got edited (e.g. checked).
```

```
public override void _Ready()
{
    GetNode("Tree").ItemEdited += OnTreeItemEdited;
}
public void OnTreeItemEdited()
{
    GD.Print(GetNode("Tree").GetEdited()); // This item just got edited (e.g. checked).
}
```

### get_edited_column {#get_edited_column}

```gdscript
int get_edited_column() const
```

Returns the column for the currently edited item.

### get_item_area_rect {#get_item_area_rect}

```gdscript
Rect2 get_item_area_rect(item: TreeItem, column: int = -1, button_index: int = -1) const
```

Returns the rectangle area for the specified TreeItem. If column is specified, only get the position and size of that column, otherwise get the rectangle containing all columns. If a button index is specified, the rectangle of that button will be returned.

### get_item_at_position {#get_item_at_position}

```gdscript
TreeItem get_item_at_position(position: Vector2) const
```

Returns the tree item at the specified position (relative to the tree origin position).

### get_next_selected {#get_next_selected}

```gdscript
TreeItem get_next_selected(from: TreeItem)
```

Returns the next selected TreeItem after the given one, or null if the end is reached.

If from is null, this returns the first selected item.

### get_pressed_button {#get_pressed_button}

```gdscript
int get_pressed_button() const
```

Returns the last pressed button's index.

### get_root {#get_root}

```gdscript
TreeItem get_root() const
```

Returns the tree's root item, or null if the tree is empty.

### get_scroll {#get_scroll}

```gdscript
Vector2 get_scroll() const
```

Returns the current scrolling position.

### get_scroll_hint_mode {#get_scroll_hint_mode}

```gdscript
Tree.ScrollHintMode get_scroll_hint_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_select_mode {#get_select_mode}

```gdscript
Tree.SelectMode get_select_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_selected {#get_selected}

```gdscript
TreeItem get_selected() const
```

Returns the currently focused item, or null if no item is focused.

In SELECT_ROW and SELECT_SINGLE modes, the focused item is same as the selected item. In SELECT_MULTI mode, the focused item is the item under the focus cursor, not necessarily selected.

To get the currently selected item(s), use get_next_selected().

### get_selected_column {#get_selected_column}

```gdscript
int get_selected_column() const
```

Returns the currently focused column, or -1 if no column is focused.

In SELECT_SINGLE mode, the focused column is the selected column. In SELECT_ROW mode, the focused column is always 0 if any item is selected. In SELECT_MULTI mode, the focused column is the column under the focus cursor, and there are not necessarily any column selected.

To tell whether a column of an item is selected, use TreeItem.is_selected().

### is_auto_tooltip_enabled {#is_auto_tooltip_enabled}

```gdscript
bool is_auto_tooltip_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_column_clipping_content {#is_column_clipping_content}

```gdscript
bool is_column_clipping_content(column: int) const
```

Returns true if the column has enabled clipping (see set_column_clip_content()).

### is_column_expanding {#is_column_expanding}

```gdscript
bool is_column_expanding(column: int) const
```

Returns true if the column has enabled expanding (see set_column_expand()).

### is_drag_unfolding_enabled {#is_drag_unfolding_enabled}

```gdscript
bool is_drag_unfolding_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_folding_hidden {#is_folding_hidden}

```gdscript
bool is_folding_hidden() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_h_scroll_enabled {#is_h_scroll_enabled}

```gdscript
bool is_h_scroll_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_recursive_folding_enabled {#is_recursive_folding_enabled}

```gdscript
bool is_recursive_folding_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_root_hidden {#is_root_hidden}

```gdscript
bool is_root_hidden() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scroll_hint_tiled {#is_scroll_hint_tiled}

```gdscript
bool is_scroll_hint_tiled()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_v_scroll_enabled {#is_v_scroll_enabled}

```gdscript
bool is_v_scroll_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scroll_to_item {#scroll_to_item}

```gdscript
void scroll_to_item(item: TreeItem, center_on_item: bool = false)
```

Causes the Tree to jump to the specified TreeItem.

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

### set_auto_tooltip {#set_auto_tooltip}

```gdscript
void set_auto_tooltip(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_column_clip_content {#set_column_clip_content}

```gdscript
void set_column_clip_content(column: int, enable: bool)
```

Allows to enable clipping for column's content, making the content size ignored.

### set_column_custom_minimum_width {#set_column_custom_minimum_width}

```gdscript
void set_column_custom_minimum_width(column: int, min_width: int)
```

Overrides the calculated minimum width of a column. It can be set to 0 to restore the default behavior. Columns that have the "Expand" flag will use their "min_width" in a similar fashion to Control.size_flags_stretch_ratio.

### set_column_expand {#set_column_expand}

```gdscript
void set_column_expand(column: int, expand: bool)
```

If true, the column will have the "Expand" flag of Control. Columns that have the "Expand" flag will use their expand ratio in a similar fashion to Control.size_flags_stretch_ratio (see set_column_expand_ratio()).

### set_column_expand_ratio {#set_column_expand_ratio}

```gdscript
void set_column_expand_ratio(column: int, ratio: int)
```

Sets the relative expand ratio for a column. See set_column_expand().

### set_column_title {#set_column_title}

```gdscript
void set_column_title(column: int, title: String)
```

Sets the title of a column.

### set_column_title_alignment {#set_column_title_alignment}

```gdscript
void set_column_title_alignment(column: int, title_alignment: HorizontalAlignment)
```

Sets the column title alignment. Note that @GlobalScope.HORIZONTAL_ALIGNMENT_FILL is not supported for column titles.

### set_column_title_direction {#set_column_title_direction}

```gdscript
void set_column_title_direction(column: int, direction: Control.TextDirection)
```

Sets column title base writing direction.

### set_column_title_language {#set_column_title_language}

```gdscript
void set_column_title_language(column: int, language: String)
```

Sets the language code of the given column's title to language. This is used for line-breaking and text shaping algorithms. If language is empty, the current locale is used.

### set_column_title_tooltip_text {#set_column_title_tooltip_text}

```gdscript
void set_column_title_tooltip_text(column: int, tooltip_text: String)
```

Sets the column title's tooltip text.

### set_column_titles_visible {#set_column_titles_visible}

```gdscript
void set_column_titles_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_columns {#set_columns}

```gdscript
void set_columns(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drop_mode_flags {#set_drop_mode_flags}

```gdscript
void set_drop_mode_flags(flags: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_drag_unfolding {#set_enable_drag_unfolding}

```gdscript
void set_enable_drag_unfolding(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_recursive_folding {#set_enable_recursive_folding}

```gdscript
void set_enable_recursive_folding(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_h_scroll_enabled {#set_h_scroll_enabled}

```gdscript
void set_h_scroll_enabled(h_scroll: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hide_folding {#set_hide_folding}

```gdscript
void set_hide_folding(hide: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hide_root {#set_hide_root}

```gdscript
void set_hide_root(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_hint_mode {#set_scroll_hint_mode}

```gdscript
void set_scroll_hint_mode(scroll_hint_mode: Tree.ScrollHintMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_select_mode {#set_select_mode}

```gdscript
void set_select_mode(mode: Tree.SelectMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selected {#set_selected}

```gdscript
void set_selected(item: TreeItem, column: int)
```

Selects the specified TreeItem and column.

### set_tile_scroll_hint {#set_tile_scroll_hint}

```gdscript
void set_tile_scroll_hint(tile_scroll_hint: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_scroll_enabled {#set_v_scroll_enabled}

```gdscript
void set_v_scroll_enabled(h_scroll: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### columns {#columns}

```gdscript
columns: int
```

- void set_columns(value: int)
- int get_columns()

The number of columns.

Prints an error and does not allow setting the columns during mouse selection.

### column_titles_visible {#column_titles_visible}

```gdscript
column_titles_visible: bool
```

- void set_column_titles_visible(value: bool)
- bool are_column_titles_visible()

If true, column titles are visible.

### allow_reselect {#allow_reselect}

```gdscript
allow_reselect: bool
```

- void set_allow_reselect(value: bool)
- bool get_allow_reselect()

If true, the currently selected cell may be selected again.

### allow_rmb_select {#allow_rmb_select}

```gdscript
allow_rmb_select: bool
```

- void set_allow_rmb_select(value: bool)
- bool get_allow_rmb_select()

If true, a right mouse button click can select items.

### allow_search {#allow_search}

```gdscript
allow_search: bool
```

- void set_allow_search(value: bool)
- bool get_allow_search()

If true, allows navigating the Tree with letter keys through incremental search.

### hide_folding {#hide_folding}

```gdscript
hide_folding: bool
```

- void set_hide_folding(value: bool)
- bool is_folding_hidden()

If true, the folding arrow is hidden.

### enable_recursive_folding {#enable_recursive_folding}

```gdscript
enable_recursive_folding: bool
```

- void set_enable_recursive_folding(value: bool)
- bool is_recursive_folding_enabled()

If true, recursive folding is enabled for this Tree. Holding down Shift while clicking the fold arrow or using ui_right/ui_left shortcuts collapses or uncollapses the TreeItem and all its descendants.

### enable_drag_unfolding {#enable_drag_unfolding}

```gdscript
enable_drag_unfolding: bool
```

- void set_enable_drag_unfolding(value: bool)
- bool is_drag_unfolding_enabled()

If true, tree items will unfold when hovered over during a drag-and-drop. The delay for when this happens is dictated by dragging_unfold_wait_msec.

### hide_root {#hide_root}

```gdscript
hide_root: bool
```

- void set_hide_root(value: bool)
- bool is_root_hidden()

If true, the tree's root is hidden.

### drop_mode_flags {#drop_mode_flags}

```gdscript
drop_mode_flags: int
```

- void set_drop_mode_flags(value: int)
- int get_drop_mode_flags()

The drop mode as an OR combination of flags. See DropModeFlags constants. Once dropping is done, reverts to DROP_MODE_DISABLED. Setting this during Control._can_drop_data() is recommended.

This controls the drop sections, i.e. the decision and drawing of possible drop locations based on the mouse position.

### select_mode {#select_mode}

```gdscript
select_mode: int
```

- void set_select_mode(value: SelectMode)
- SelectMode get_select_mode()

Allows single or multiple selection. See the SelectMode constants.

### auto_tooltip {#auto_tooltip}

```gdscript
auto_tooltip: bool
```

- void set_auto_tooltip(value: bool)
- bool is_auto_tooltip_enabled()

If true, tree items with no tooltip assigned display their text as their tooltip. See also TreeItem.get_tooltip_text() and TreeItem.get_button_tooltip_text().

### scroll_horizontal_enabled {#scroll_horizontal_enabled}

```gdscript
scroll_horizontal_enabled: bool
```

- void set_h_scroll_enabled(value: bool)
- bool is_h_scroll_enabled()

If true, enables horizontal scrolling.

### scroll_vertical_enabled {#scroll_vertical_enabled}

```gdscript
scroll_vertical_enabled: bool
```

- void set_v_scroll_enabled(value: bool)
- bool is_v_scroll_enabled()

If true, enables vertical scrolling.

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

## 信号

### item_selected {#item_selected}

```gdscript
signal item_selected()
```

Emitted when an item is selected.

### cell_selected {#cell_selected}

```gdscript
signal cell_selected()
```

Emitted when a cell is selected.

### multi_selected {#multi_selected}

```gdscript
signal multi_selected(item: TreeItem, column: int, selected: bool)
```

Emitted instead of item_selected if select_mode is set to SELECT_MULTI.

### item_mouse_selected {#item_mouse_selected}

```gdscript
signal item_mouse_selected(mouse_position: Vector2, mouse_button_index: int)
```

Emitted when an item is selected with a mouse button.

### empty_clicked {#empty_clicked}

```gdscript
signal empty_clicked(click_position: Vector2, mouse_button_index: int)
```

Emitted when a mouse button is clicked in the empty space of the tree.

### item_edited {#item_edited}

```gdscript
signal item_edited()
```

Emitted when an item is edited.

### custom_item_clicked {#custom_item_clicked}

```gdscript
signal custom_item_clicked(mouse_button_index: int)
```

Emitted when an item with TreeItem.CELL_MODE_CUSTOM is clicked with a mouse button.

### item_icon_double_clicked {#item_icon_double_clicked}

```gdscript
signal item_icon_double_clicked()
```

Emitted when an item's icon is double-clicked. For a signal that emits when any part of the item is double-clicked, see item_activated.

### item_collapsed {#item_collapsed}

```gdscript
signal item_collapsed(item: TreeItem)
```

Emitted when an item is expanded or collapsed by clicking on the folding arrow or through code.

Note: Despite its name, this signal is also emitted when an item is expanded.

### check_propagated_to_item {#check_propagated_to_item}

```gdscript
signal check_propagated_to_item(item: TreeItem, column: int)
```

Emitted when TreeItem.propagate_check() is called. Connect to this signal to process the items that are affected when TreeItem.propagate_check() is invoked. The order that the items affected will be processed is as follows: the item that invoked the method, children of that item, and finally parents of that item.

### button_clicked {#button_clicked}

```gdscript
signal button_clicked(item: TreeItem, column: int, id: int, mouse_button_index: int)
```

Emitted when a button on the tree was pressed (see TreeItem.add_button()).

### custom_popup_edited {#custom_popup_edited}

```gdscript
signal custom_popup_edited(arrow_clicked: bool)
```

Emitted when a cell with the TreeItem.CELL_MODE_CUSTOM is clicked to be edited.

### item_activated {#item_activated}

```gdscript
signal item_activated()
```

Emitted when an item is double-clicked, or selected with a ui_accept input event (e.g. using Enter or Space on the keyboard).

### column_title_clicked {#column_title_clicked}

```gdscript
signal column_title_clicked(column: int, mouse_button_index: int)
```

Emitted when a column's title is clicked with either @GlobalScope.MOUSE_BUTTON_LEFT or @GlobalScope.MOUSE_BUTTON_RIGHT.

### nothing_selected {#nothing_selected}

```gdscript
signal nothing_selected()
```

Emitted when a left mouse button click does not select any item.

## 枚举

### SelectMode {#enum-SelectMode}

```gdscript
enum SelectMode { SELECT_SINGLE = 0, SELECT_ROW = 1, SELECT_MULTI = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SELECT_SINGLE {#SELECT_SINGLE}

```gdscript
SELECT_SINGLE = 0
```

Allows selection of a single cell at a time. From the perspective of items, only a single item is allowed to be selected. And there is only one column selected in the selected item.

The focus cursor is always hidden in this mode, but it is positioned at the current selection, making the currently selected item the currently focused item.

#### SELECT_ROW {#SELECT_ROW}

```gdscript
SELECT_ROW = 1
```

Allows selection of a single row at a time. From the perspective of items, only a single items is allowed to be selected. And all the columns are selected in the selected item.

The focus cursor is always hidden in this mode, but it is positioned at the first column of the current selection, making the currently selected item the currently focused item.

#### SELECT_MULTI {#SELECT_MULTI}

```gdscript
SELECT_MULTI = 2
```

Allows selection of multiple cells at the same time. From the perspective of items, multiple items are allowed to be selected. And there can be multiple columns selected in each selected item.

The focus cursor is visible in this mode, the item or column under the cursor is not necessarily selected.

---

### DropModeFlags {#enum-DropModeFlags}

```gdscript
enum DropModeFlags { DROP_MODE_DISABLED = 0, DROP_MODE_ON_ITEM = 1, DROP_MODE_INBETWEEN = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DROP_MODE_DISABLED {#DROP_MODE_DISABLED}

```gdscript
DROP_MODE_DISABLED = 0
```

Disables all drop sections.

Note: This is the default flag, it has no effect when combined with other flags.

#### DROP_MODE_ON_ITEM {#DROP_MODE_ON_ITEM}

```gdscript
DROP_MODE_ON_ITEM = 1
```

Enables the "on item" drop section. This drop section covers the entire item.

When combined with DROP_MODE_INBETWEEN, this drop section halves in height and stays centered vertically.

#### DROP_MODE_INBETWEEN {#DROP_MODE_INBETWEEN}

```gdscript
DROP_MODE_INBETWEEN = 2
```

Enables "above item" and "below item" drop sections. The "above item" drop section covers the top half of the item, while the "below item" drop section covers the bottom half, and extends downward to the left of any children.

When combined with DROP_MODE_ON_ITEM, these drop sections halve in height and stay at the top and bottom respectively.

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
