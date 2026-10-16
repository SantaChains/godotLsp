# OptionButton

**继承**：[Button](class_button.md) → [BaseButton](class_basebutton.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **OptionButton**

> OptionButton is a type of button that brings up a dropdown with selectable items when pressed. The item selected becomes the "current" item and is displayed as the button text.

<details><summary>官方英文描述（中文精译进行中）</summary>

OptionButton is a type of button that brings up a dropdown with selectable items when pressed. The item selected becomes the "current" item and is displayed as the button text.

See also BaseButton which contains common properties and methods associated with this node.

Note: The IDs used for items are limited to signed 32-bit integers, not the full 64 bits of int. These have a range of -2^31 to 2^31 - 1, that is, -2147483648 to 2147483647.

Note: The Button.text and Button.icon properties are set automatically based on the selected item. They shouldn't be changed manually.

</details>

## 方法

### add_icon_item {#add_icon_item}

```gdscript
void add_icon_item(texture: Texture2D, label: String, id: int = -1)
```

Adds an item, with a texture icon, text label and (optionally) id. If no id is passed, the item index will be used as the item's ID. New items are appended at the end.

Note: The item will be selected if there are no other items.

### add_item {#add_item}

```gdscript
void add_item(label: String, id: int = -1)
```

Adds an item, with text label and (optionally) id. If no id is passed, the item index will be used as the item's ID. New items are appended at the end.

Note: The item will be selected if there are no other items.

### add_separator {#add_separator}

```gdscript
void add_separator(text: String = "")
```

Adds a separator to the list of items. Separators help to group items, and can optionally be given a text header. A separator also gets an index assigned, and is appended at the end of the item list.

### clear {#clear}

```gdscript
void clear()
```

Clears all the items in the OptionButton.

### get_allow_reselect {#get_allow_reselect}

```gdscript
bool get_allow_reselect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_item_auto_translate_mode {#get_item_auto_translate_mode}

```gdscript
Node.AutoTranslateMode get_item_auto_translate_mode(idx: int) const
```

Returns the auto translate mode of the item at index idx.

### get_item_count {#get_item_count}

```gdscript
int get_item_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_item_icon {#get_item_icon}

```gdscript
Texture2D get_item_icon(idx: int) const
```

Returns the icon of the item at index idx.

### get_item_id {#get_item_id}

```gdscript
int get_item_id(idx: int) const
```

Returns the ID of the item at index idx.

### get_item_index {#get_item_index}

```gdscript
int get_item_index(id: int) const
```

Returns the index of the item with the given id.

### get_item_metadata {#get_item_metadata}

```gdscript
Variant get_item_metadata(idx: int) const
```

Retrieves the metadata of an item. Metadata may be any type and can be used to store extra information about an item, such as an external string ID.

### get_item_text {#get_item_text}

```gdscript
String get_item_text(idx: int) const
```

Returns the text of the item at index idx.

### get_item_tooltip {#get_item_tooltip}

```gdscript
String get_item_tooltip(idx: int) const
```

Returns the tooltip of the item at index idx.

### get_popup {#get_popup}

```gdscript
PopupMenu get_popup() const
```

Returns the PopupMenu contained in this button.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their Window.visible property.

### get_search_bar_fuzzy_search_max_misses {#get_search_bar_fuzzy_search_max_misses}

```gdscript
int get_search_bar_fuzzy_search_max_misses() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_search_bar_min_item_count {#get_search_bar_min_item_count}

```gdscript
int get_search_bar_min_item_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_selectable_item {#get_selectable_item}

```gdscript
int get_selectable_item(from_last: bool = false) const
```

Returns the index of the first item which is not disabled, or marked as a separator. If from_last is true, the items will be searched in reverse order.

Returns -1 if no item is found.

### get_selected {#get_selected}

```gdscript
int get_selected() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_selected_id {#get_selected_id}

```gdscript
int get_selected_id() const
```

Returns the ID of the selected item, or -1 if no item is selected.

### get_selected_metadata {#get_selected_metadata}

```gdscript
Variant get_selected_metadata() const
```

Gets the metadata of the selected item. Metadata for items can be set using set_item_metadata().

### has_selectable_items {#has_selectable_items}

```gdscript
bool has_selectable_items() const
```

Returns true if this button contains at least one item which is not disabled, or marked as a separator.

### is_fit_to_longest_item {#is_fit_to_longest_item}

```gdscript
bool is_fit_to_longest_item() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_item_disabled {#is_item_disabled}

```gdscript
bool is_item_disabled(idx: int) const
```

Returns true if the item at index idx is disabled.

### is_item_separator {#is_item_separator}

```gdscript
bool is_item_separator(idx: int) const
```

Returns true if the item at index idx is marked as a separator.

### is_search_bar_enabled {#is_search_bar_enabled}

```gdscript
bool is_search_bar_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_search_bar_fuzzy_search_enabled {#is_search_bar_fuzzy_search_enabled}

```gdscript
bool is_search_bar_fuzzy_search_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_item {#remove_item}

```gdscript
void remove_item(idx: int)
```

Removes the item at index idx.

### select {#select}

```gdscript
void select(idx: int)
```

Selects an item by index and makes it the current item. This will work even if the item is disabled.

Passing -1 as the index deselects any currently selected item.

### set_allow_reselect {#set_allow_reselect}

```gdscript
void set_allow_reselect(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_shortcuts {#set_disable_shortcuts}

```gdscript
void set_disable_shortcuts(disabled: bool)
```

If true, shortcuts are disabled and cannot be used to trigger the button.

### set_fit_to_longest_item {#set_fit_to_longest_item}

```gdscript
void set_fit_to_longest_item(fit: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_item_auto_translate_mode {#set_item_auto_translate_mode}

```gdscript
void set_item_auto_translate_mode(idx: int, mode: Node.AutoTranslateMode)
```

Sets the auto translate mode of the item at index idx.

Items use Node.AUTO_TRANSLATE_MODE_INHERIT by default, which uses the same auto translate mode as the OptionButton itself.

### set_item_count {#set_item_count}

```gdscript
void set_item_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_item_disabled {#set_item_disabled}

```gdscript
void set_item_disabled(idx: int, disabled: bool)
```

Sets whether the item at index idx is disabled.

Disabled items are drawn differently in the dropdown and are not selectable by the user. If the current selected item is set as disabled, it will remain selected.

### set_item_icon {#set_item_icon}

```gdscript
void set_item_icon(idx: int, texture: Texture2D)
```

Sets the icon of the item at index idx.

### set_item_id {#set_item_id}

```gdscript
void set_item_id(idx: int, id: int)
```

Sets the ID of the item at index idx.

### set_item_metadata {#set_item_metadata}

```gdscript
void set_item_metadata(idx: int, metadata: Variant)
```

Sets the metadata of an item. Metadata may be of any type and can be used to store extra information about an item, such as an external string ID.

### set_item_text {#set_item_text}

```gdscript
void set_item_text(idx: int, text: String)
```

Sets the text of the item at index idx.

### set_item_tooltip {#set_item_tooltip}

```gdscript
void set_item_tooltip(idx: int, tooltip: String)
```

Sets the tooltip of the item at index idx.

### set_search_bar_enabled {#set_search_bar_enabled}

```gdscript
void set_search_bar_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_search_bar_fuzzy_search_enabled {#set_search_bar_fuzzy_search_enabled}

```gdscript
void set_search_bar_fuzzy_search_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_search_bar_fuzzy_search_max_misses {#set_search_bar_fuzzy_search_max_misses}

```gdscript
void set_search_bar_fuzzy_search_max_misses(max_misses: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_search_bar_min_item_count {#set_search_bar_min_item_count}

```gdscript
void set_search_bar_min_item_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### show_popup {#show_popup}

```gdscript
void show_popup()
```

Adjusts popup position and sizing for the OptionButton, then shows the PopupMenu. Prefer this over using get_popup().popup().

## 属性

### selected {#selected}

```gdscript
selected: int
```

- int get_selected()

The index of the currently selected item, or -1 if no item is selected.

### fit_to_longest_item {#fit_to_longest_item}

```gdscript
fit_to_longest_item: bool
```

- void set_fit_to_longest_item(value: bool)
- bool is_fit_to_longest_item()

If true, minimum size will be determined by the longest item's text, instead of the currently selected one's.

Note: For performance reasons, the minimum size doesn't update immediately when adding, removing or modifying items.

### allow_reselect {#allow_reselect}

```gdscript
allow_reselect: bool
```

- void set_allow_reselect(value: bool)
- bool get_allow_reselect()

If true, the currently selected item can be selected again.

### search_bar_enabled {#search_bar_enabled}

```gdscript
search_bar_enabled: bool
```

- void set_search_bar_enabled(value: bool)
- bool is_search_bar_enabled()

If true, shows a search bar at the top of the PopupMenu for filtering items. See search_bar_min_item_count for dynamically controlling its visibility based on the number of items.

### search_bar_min_item_count {#search_bar_min_item_count}

```gdscript
search_bar_min_item_count: int
```

- void set_search_bar_min_item_count(value: int)
- int get_search_bar_min_item_count()

Sets the minimum number of items required for the PopupMenu search bar to be visible. search_bar_enabled must be true for this to have any effect.

### search_bar_fuzzy_search_enabled {#search_bar_fuzzy_search_enabled}

```gdscript
search_bar_fuzzy_search_enabled: bool
```

- void set_search_bar_fuzzy_search_enabled(value: bool)
- bool is_search_bar_fuzzy_search_enabled()

If true, enables fuzzy searching in the PopupMenu search bar. This allows the search results to include items that almost match the search query, as well items that match the individual characters of the search query, but not in sequence.

Use search_bar_fuzzy_search_max_misses to set the maximum number of mismatches allowed in the search results.

### search_bar_fuzzy_search_max_misses {#search_bar_fuzzy_search_max_misses}

```gdscript
search_bar_fuzzy_search_max_misses: int
```

- void set_search_bar_fuzzy_search_max_misses(value: int)
- int get_search_bar_fuzzy_search_max_misses()

Sets the maximum number of mismatches allowed in each search result when fuzzy searching is enabled for the PopupMenu search bar. Any item with more mismatches will be hidden from the search results.

### item_count {#item_count}

```gdscript
item_count: int
```

- void set_item_count(value: int)
- int get_item_count()

The number of items to select from.

## 信号

### item_selected {#item_selected}

```gdscript
signal item_selected(index: int)
```

Emitted when the current item has been changed by the user. The index of the item selected is passed as argument.

allow_reselect must be enabled to reselect an item.

### item_focused {#item_focused}

```gdscript
signal item_focused(index: int)
```

Emitted when the user navigates to an item using the ProjectSettings.input/ui_up or ProjectSettings.input/ui_down input actions. The index of the item focused is passed as argument.
