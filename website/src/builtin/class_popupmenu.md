# PopupMenu

**继承**：[Popup](class_popup.md) → [Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **PopupMenu**

> PopupMenu is a modal window used to display a list of options. Useful for toolbars and context menus.

<details><summary>官方英文描述（中文精译进行中）</summary>

PopupMenu is a modal window used to display a list of options. Useful for toolbars and context menus.

The size of a PopupMenu can be limited by using Window.max_size. If the height of the list of items is larger than the maximum height of the PopupMenu, a ScrollContainer within the popup will allow the user to scroll the contents. If no maximum size is set, or if it is set to 0, the PopupMenu height will be limited by its parent rect.

All set_* methods allow negative item indices, i.e. -1 to access the last item, -2 to select the second-to-last item, and so on.

Incremental search: Like ItemList and Tree, PopupMenu supports searching within the list while the control is focused. Press a key that matches the first letter of an item's name to select the first item starting with the given letter. After that point, there are two ways to perform incremental search: 1) Press the same key again before the timeout duration to select the next item starting with the same letter. 2) Press letter keys that match the rest of the word before the timeout duration to match to select the item in question directly. Both of these actions will be reset to the beginning of the list if the timeout duration has passed since the last keystroke was registered. You can adjust the timeout duration by changing ProjectSettings.gui/timers/incremental_search_max_interval_msec.

Note: PopupMenu is invisible by default. To make it visible, call one of the popup_* methods from Window on the node, such as Window.popup_centered_clamped().

Note: The ID values used for items are limited to 32 bits, not full 64 bits of int. This has a range of -2^32 to 2^32 - 1, i.e. -2147483648 to 2147483647.

</details>

## 方法

### activate_item_by_event {#activate_item_by_event}

```gdscript
bool activate_item_by_event(event: InputEvent, for_global_only: bool = false)
```

Checks the provided event against the PopupMenu's shortcuts and accelerators, and activates the first item with matching events. If for_global_only is true, only shortcuts and accelerators with global set to true will be called.

Returns true if an item was successfully activated.

Note: Certain Controls, such as MenuButton, will call this method automatically.

### add_check_item {#add_check_item}

```gdscript
void add_check_item(label: String, id: int = -1, accel: Key = 0)
```

Adds a new checkable item with text label.

An id can optionally be provided, as well as an accelerator (accel). If no id is provided, one will be created from the index. If no accel is provided, then the default value of 0 (corresponding to @GlobalScope.KEY_NONE) will be assigned to the item (which means it won't have any accelerator). See get_item_accelerator() for more info on accelerators.

Note: Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

### add_check_shortcut {#add_check_shortcut}

```gdscript
void add_check_shortcut(shortcut: Shortcut, id: int = -1, global: bool = false)
```

Adds a new checkable item and assigns the specified Shortcut to it. Sets the label of the checkbox to the Shortcut's name.

An id can optionally be provided. If no id is provided, one will be created from the index.

Note: Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

### add_icon_check_item {#add_icon_check_item}

```gdscript
void add_icon_check_item(texture: Texture2D, label: String, id: int = -1, accel: Key = 0)
```

Adds a new checkable item with text label and icon texture.

An id can optionally be provided, as well as an accelerator (accel). If no id is provided, one will be created from the index. If no accel is provided, then the default value of 0 (corresponding to @GlobalScope.KEY_NONE) will be assigned to the item (which means it won't have any accelerator). See get_item_accelerator() for more info on accelerators.

Note: Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

### add_icon_check_shortcut {#add_icon_check_shortcut}

```gdscript
void add_icon_check_shortcut(texture: Texture2D, shortcut: Shortcut, id: int = -1, global: bool = false)
```

Adds a new checkable item and assigns the specified Shortcut and icon texture to it. Sets the label of the checkbox to the Shortcut's name.

An id can optionally be provided. If no id is provided, one will be created from the index.

Note: Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

### add_icon_item {#add_icon_item}

```gdscript
void add_icon_item(texture: Texture2D, label: String, id: int = -1, accel: Key = 0)
```

Adds a new item with text label and icon texture.

An id can optionally be provided, as well as an accelerator (accel). If no id is provided, one will be created from the index. If no accel is provided, then the default value of 0 (corresponding to @GlobalScope.KEY_NONE) will be assigned to the item (which means it won't have any accelerator). See get_item_accelerator() for more info on accelerators.

### add_icon_radio_check_item {#add_icon_radio_check_item}

```gdscript
void add_icon_radio_check_item(texture: Texture2D, label: String, id: int = -1, accel: Key = 0)
```

Same as add_icon_check_item(), but uses a radio check button.

### add_icon_radio_check_shortcut {#add_icon_radio_check_shortcut}

```gdscript
void add_icon_radio_check_shortcut(texture: Texture2D, shortcut: Shortcut, id: int = -1, global: bool = false)
```

Same as add_icon_check_shortcut(), but uses a radio check button.

### add_icon_shortcut {#add_icon_shortcut}

```gdscript
void add_icon_shortcut(texture: Texture2D, shortcut: Shortcut, id: int = -1, global: bool = false, allow_echo: bool = false)
```

Adds a new item and assigns the specified Shortcut and icon texture to it. Sets the label of the checkbox to the Shortcut's name.

An id can optionally be provided. If no id is provided, one will be created from the index.

If allow_echo is true, the shortcut can be activated with echo events.

### add_item {#add_item}

```gdscript
void add_item(label: String, id: int = -1, accel: Key = 0)
```

Adds a new item with text label.

An id can optionally be provided, as well as an accelerator (accel). If no id is provided, one will be created from the index. If no accel is provided, then the default value of 0 (corresponding to @GlobalScope.KEY_NONE) will be assigned to the item (which means it won't have any accelerator). See get_item_accelerator() for more info on accelerators.

Note: The provided id is used only in id_pressed and id_focused signals. It's not related to the index arguments in e.g. set_item_checked().

### add_multistate_item {#add_multistate_item}

```gdscript
void add_multistate_item(label: String, max_states: int, default_state: int = 0, id: int = -1, accel: Key = 0)
```

Adds a new multistate item with text label.

Contrarily to normal binary items, multistate items can have more than two states, as defined by max_states. The default value is defined by default_state.

An id can optionally be provided, as well as an accelerator (accel). If no id is provided, one will be created from the index. If no accel is provided, then the default value of 0 (corresponding to @GlobalScope.KEY_NONE) will be assigned to the item (which means it won't have any accelerator). See get_item_accelerator() for more info on accelerators.

```gdscript
func _ready():
    add_multistate_item("Item", 3, 0)
    index_pressed.connect(func(index: int):
            toggle_item_multistate(index)
            match get_item_multistate(index):
                0:
                    print("First state")
                1:
                    print("Second state")
                2:
                    print("Third state")
        )
```

Note: Multistate items don't update their state automatically and must be done manually. See toggle_item_multistate(), set_item_multistate() and get_item_multistate() for more info on how to control it.

### add_radio_check_item {#add_radio_check_item}

```gdscript
void add_radio_check_item(label: String, id: int = -1, accel: Key = 0)
```

Adds a new radio check button with text label.

An id can optionally be provided, as well as an accelerator (accel). If no id is provided, one will be created from the index. If no accel is provided, then the default value of 0 (corresponding to @GlobalScope.KEY_NONE) will be assigned to the item (which means it won't have any accelerator). See get_item_accelerator() for more info on accelerators.

Note: Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

### add_radio_check_shortcut {#add_radio_check_shortcut}

```gdscript
void add_radio_check_shortcut(shortcut: Shortcut, id: int = -1, global: bool = false)
```

Adds a new radio check button and assigns a Shortcut to it. Sets the label of the checkbox to the Shortcut's name.

An id can optionally be provided. If no id is provided, one will be created from the index.

Note: Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

### add_separator {#add_separator}

```gdscript
void add_separator(label: String = "", id: int = -1)
```

Adds a separator between items. Separators also occupy an index, which you can set by using the id parameter.

A label can optionally be provided, which will appear at the center of the separator.

### add_shortcut {#add_shortcut}

```gdscript
void add_shortcut(shortcut: Shortcut, id: int = -1, global: bool = false, allow_echo: bool = false)
```

Adds a Shortcut.

An id can optionally be provided. If no id is provided, one will be created from the index.

If allow_echo is true, the shortcut can be activated with echo events.

### add_submenu_item {#add_submenu_item}

```gdscript
void add_submenu_item(label: String, submenu: String, id: int = -1)
```

Deprecated: Prefer using add_submenu_node_item() instead.

Adds an item that will act as a submenu of the parent PopupMenu node when clicked. The submenu argument must be the name of an existing PopupMenu that has been added as a child to this node. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the ui_select or ui_right input actions.

An id can optionally be provided. If no id is provided, one will be created from the index.

### add_submenu_node_item {#add_submenu_node_item}

```gdscript
void add_submenu_node_item(label: String, submenu: PopupMenu, id: int = -1)
```

Adds an item that will act as a submenu of the parent PopupMenu node when clicked. This submenu will be shown when the item is clicked, hovered for long enough, or activated using the ui_select or ui_right input actions.

submenu must be either child of this PopupMenu or has no parent node (in which case it will be automatically added as a child). If the submenu popup has another parent, this method will fail.

An id can optionally be provided. If no id is provided, one will be created from the index.

### clear {#clear}

```gdscript
void clear(free_submenus: bool = false)
```

Removes all items from the PopupMenu. If free_submenus is true, the submenu nodes are automatically freed.

### get_allow_search {#get_allow_search}

```gdscript
bool get_allow_search() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_focused_item {#get_focused_item}

```gdscript
int get_focused_item() const
```

Returns the index of the currently focused item. Returns -1 if no item is focused.

### get_item_accelerator {#get_item_accelerator}

```gdscript
Key get_item_accelerator(index: int) const
```

Returns the accelerator of the item at the given index. An accelerator is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The return value is an integer which is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A). If no accelerator is defined for the specified index, get_item_accelerator() returns 0 (corresponding to @GlobalScope.KEY_NONE).

### get_item_auto_translate_mode {#get_item_auto_translate_mode}

```gdscript
Node.AutoTranslateMode get_item_auto_translate_mode(index: int) const
```

Returns the auto translate mode of the item at the given index.

### get_item_count {#get_item_count}

```gdscript
int get_item_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_item_icon {#get_item_icon}

```gdscript
Texture2D get_item_icon(index: int) const
```

Returns the icon of the item at the given index.

### get_item_icon_max_width {#get_item_icon_max_width}

```gdscript
int get_item_icon_max_width(index: int) const
```

Returns the maximum allowed width of the icon for the item at the given index.

### get_item_icon_modulate {#get_item_icon_modulate}

```gdscript
Color get_item_icon_modulate(index: int) const
```

Returns a Color modulating the item's icon at the given index.

### get_item_id {#get_item_id}

```gdscript
int get_item_id(index: int) const
```

Returns the ID of the item at the given index.

### get_item_indent {#get_item_indent}

```gdscript
int get_item_indent(index: int) const
```

Returns the horizontal offset of the item at the given index.

### get_item_index {#get_item_index}

```gdscript
int get_item_index(id: int) const
```

Returns the index of the item containing the specified id. The index is automatically assigned to each item by the engine when added and represents the order items will be displayed.

### get_item_language {#get_item_language}

```gdscript
String get_item_language(index: int) const
```

Returns item's text language code.

### get_item_metadata {#get_item_metadata}

```gdscript
Variant get_item_metadata(index: int) const
```

Returns the metadata of the specified item, which might be of any type. You can set it with set_item_metadata(), which provides a simple way of assigning context data to items.

### get_item_multistate {#get_item_multistate}

```gdscript
int get_item_multistate(index: int) const
```

Returns the state of the item at the given index.

### get_item_multistate_max {#get_item_multistate_max}

```gdscript
int get_item_multistate_max(index: int) const
```

Returns the max states of the item at the given index.

### get_item_shortcut {#get_item_shortcut}

```gdscript
Shortcut get_item_shortcut(index: int) const
```

Returns the Shortcut associated with the item at the given index.

### get_item_submenu {#get_item_submenu}

```gdscript
String get_item_submenu(index: int) const
```

Deprecated: Prefer using get_item_submenu_node() instead.

Returns the submenu name of the item at the given index. See add_submenu_item() for more info on how to add a submenu.

### get_item_submenu_node {#get_item_submenu_node}

```gdscript
PopupMenu get_item_submenu_node(index: int) const
```

Returns the submenu of the item at the given index, or null if no submenu was added. See add_submenu_node_item() for more info on how to add a submenu.

### get_item_text {#get_item_text}

```gdscript
String get_item_text(index: int) const
```

Returns the text of the item at the given index.

### get_item_text_direction {#get_item_text_direction}

```gdscript
Control.TextDirection get_item_text_direction(index: int) const
```

Returns item's text base writing direction.

### get_item_tooltip {#get_item_tooltip}

```gdscript
String get_item_tooltip(index: int) const
```

Returns the tooltip associated with the item at the given index.

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

### get_shrink_height {#get_shrink_height}

```gdscript
bool get_shrink_height() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shrink_width {#get_shrink_width}

```gdscript
bool get_shrink_width() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_submenu_popup_delay {#get_submenu_popup_delay}

```gdscript
float get_submenu_popup_delay() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_system_menu {#get_system_menu}

```gdscript
NativeMenu.SystemMenus get_system_menu() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hide_on_checkable_item_selection {#is_hide_on_checkable_item_selection}

```gdscript
bool is_hide_on_checkable_item_selection() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hide_on_item_selection {#is_hide_on_item_selection}

```gdscript
bool is_hide_on_item_selection() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_hide_on_state_item_selection {#is_hide_on_state_item_selection}

```gdscript
bool is_hide_on_state_item_selection() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_item_checkable {#is_item_checkable}

```gdscript
bool is_item_checkable(index: int) const
```

Returns true if the item at the given index is checkable in some way, i.e. if it has a checkbox or radio button.

Note: Checkable items just display a checkmark or radio button, but don't have any built-in checking behavior and must be checked/unchecked manually.

### is_item_checked {#is_item_checked}

```gdscript
bool is_item_checked(index: int) const
```

Returns true if the item at the given index is checked.

### is_item_disabled {#is_item_disabled}

```gdscript
bool is_item_disabled(index: int) const
```

Returns true if the item at the given index is disabled. When it is disabled it can't be selected, or its action invoked.

See set_item_disabled() for more info on how to disable an item.

### is_item_radio_checkable {#is_item_radio_checkable}

```gdscript
bool is_item_radio_checkable(index: int) const
```

Returns true if the item at the given index has radio button-style checkability.

Note: This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups.

### is_item_separator {#is_item_separator}

```gdscript
bool is_item_separator(index: int) const
```

Returns true if the item is a separator. If it is, it will be displayed as a line. See add_separator() for more info on how to add a separator.

### is_item_shortcut_disabled {#is_item_shortcut_disabled}

```gdscript
bool is_item_shortcut_disabled(index: int) const
```

Returns true if the specified item's shortcut is disabled.

### is_native_menu {#is_native_menu}

```gdscript
bool is_native_menu() const
```

Returns true if the system native menu is supported and currently used by this PopupMenu.

### is_prefer_native_menu {#is_prefer_native_menu}

```gdscript
bool is_prefer_native_menu() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

### is_system_menu {#is_system_menu}

```gdscript
bool is_system_menu() const
```

Returns true if the menu is bound to the special system menu.

### remove_item {#remove_item}

```gdscript
void remove_item(index: int)
```

Removes the item at the given index from the menu.

Note: The indices of items after the removed item will be shifted by one.

### scroll_to_item {#scroll_to_item}

```gdscript
void scroll_to_item(index: int)
```

Moves the scroll view to make the item at the given index visible.

### set_allow_search {#set_allow_search}

```gdscript
void set_allow_search(allow: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_focused_item {#set_focused_item}

```gdscript
void set_focused_item(index: int)
```

Sets the currently focused item as the given index.

Passing -1 as the index makes so that no item is focused.

### set_hide_on_checkable_item_selection {#set_hide_on_checkable_item_selection}

```gdscript
void set_hide_on_checkable_item_selection(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hide_on_item_selection {#set_hide_on_item_selection}

```gdscript
void set_hide_on_item_selection(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hide_on_state_item_selection {#set_hide_on_state_item_selection}

```gdscript
void set_hide_on_state_item_selection(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_item_accelerator {#set_item_accelerator}

```gdscript
void set_item_accelerator(index: int, accel: Key)
```

Sets the accelerator of the item at the given index. An accelerator is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. accel is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

### set_item_as_checkable {#set_item_as_checkable}

```gdscript
void set_item_as_checkable(index: int, enable: bool)
```

Sets whether the item at the given index has a checkbox. If false, sets the type of the item to plain text.

Note: Checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually.

### set_item_as_radio_checkable {#set_item_as_radio_checkable}

```gdscript
void set_item_as_radio_checkable(index: int, enable: bool)
```

Sets the type of the item at the given index to radio button. If false, sets the type of the item to plain text.

### set_item_as_separator {#set_item_as_separator}

```gdscript
void set_item_as_separator(index: int, enable: bool)
```

Mark the item at the given index as a separator, which means that it would be displayed as a line. If false, sets the type of the item to plain text.

### set_item_auto_translate_mode {#set_item_auto_translate_mode}

```gdscript
void set_item_auto_translate_mode(index: int, mode: Node.AutoTranslateMode)
```

Sets the auto translate mode of the item at the given index.

Items use Node.AUTO_TRANSLATE_MODE_INHERIT by default, which uses the same auto translate mode as the PopupMenu itself.

### set_item_checked {#set_item_checked}

```gdscript
void set_item_checked(index: int, checked: bool)
```

Sets the checkstate status of the item at the given index.

### set_item_count {#set_item_count}

```gdscript
void set_item_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_item_disabled {#set_item_disabled}

```gdscript
void set_item_disabled(index: int, disabled: bool)
```

Enables/disables the item at the given index. When it is disabled, it can't be selected and its action can't be invoked.

### set_item_icon {#set_item_icon}

```gdscript
void set_item_icon(index: int, icon: Texture2D)
```

Replaces the Texture2D icon of the item at the given index.

### set_item_icon_max_width {#set_item_icon_max_width}

```gdscript
void set_item_icon_max_width(index: int, width: int)
```

Sets the maximum allowed width of the icon for the item at the given index. This limit is applied on top of the default size of the icon and on top of icon_max_width. The height is adjusted according to the icon's ratio.

### set_item_icon_modulate {#set_item_icon_modulate}

```gdscript
void set_item_icon_modulate(index: int, modulate: Color)
```

Sets a modulating Color of the item's icon at the given index.

### set_item_id {#set_item_id}

```gdscript
void set_item_id(index: int, id: int)
```

Sets the id of the item at the given index.

The id is used in id_pressed and id_focused signals.

### set_item_indent {#set_item_indent}

```gdscript
void set_item_indent(index: int, indent: int)
```

Sets the horizontal offset of the item at the given index.

### set_item_index {#set_item_index}

```gdscript
void set_item_index(index: int, target_index: int)
```

Changes the index of the item at index index to be at index target_index. This can be used to move an item above other items. The moved item will keep the same ID, even if it was generated from the original index.

Note: The indices of any items between index index and index target_index will be shifted by one.

### set_item_language {#set_item_language}

```gdscript
void set_item_language(index: int, language: String)
```

Sets the language code of the text for the item at the given index to language. This is used for line-breaking and text shaping algorithms. If language is empty, the current locale is used.

### set_item_metadata {#set_item_metadata}

```gdscript
void set_item_metadata(index: int, metadata: Variant)
```

Sets the metadata of an item, which may be of any type. You can later get it with get_item_metadata(), which provides a simple way of assigning context data to items.

### set_item_multistate {#set_item_multistate}

```gdscript
void set_item_multistate(index: int, state: int)
```

Sets the state of a multistate item. See add_multistate_item() for details.

### set_item_multistate_max {#set_item_multistate_max}

```gdscript
void set_item_multistate_max(index: int, max_states: int)
```

Sets the max states of a multistate item. See add_multistate_item() for details.

### set_item_shortcut {#set_item_shortcut}

```gdscript
void set_item_shortcut(index: int, shortcut: Shortcut, global: bool = false)
```

Sets a Shortcut for the item at the given index.

### set_item_shortcut_disabled {#set_item_shortcut_disabled}

```gdscript
void set_item_shortcut_disabled(index: int, disabled: bool)
```

Disables the Shortcut of the item at the given index.

### set_item_submenu {#set_item_submenu}

```gdscript
void set_item_submenu(index: int, submenu: String)
```

Deprecated: Prefer using set_item_submenu_node() instead.

Sets the submenu of the item at the given index. The submenu is the name of a child PopupMenu node that would be shown when the item is clicked.

### set_item_submenu_node {#set_item_submenu_node}

```gdscript
void set_item_submenu_node(index: int, submenu: PopupMenu)
```

Sets the submenu of the item at the given index. The submenu is a PopupMenu node that would be shown when the item is clicked. It must either be a child of this PopupMenu or has no parent (in which case it will be automatically added as a child). If the submenu popup has another parent, this method will fail.

### set_item_text {#set_item_text}

```gdscript
void set_item_text(index: int, text: String)
```

Sets the text of the item at the given index.

### set_item_text_direction {#set_item_text_direction}

```gdscript
void set_item_text_direction(index: int, direction: Control.TextDirection)
```

Sets item's text base writing direction.

### set_item_tooltip {#set_item_tooltip}

```gdscript
void set_item_tooltip(index: int, tooltip: String)
```

Sets the String tooltip of the item at the given index.

### set_prefer_native_menu {#set_prefer_native_menu}

```gdscript
void set_prefer_native_menu(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

### set_shrink_height {#set_shrink_height}

```gdscript
void set_shrink_height(shrink: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shrink_width {#set_shrink_width}

```gdscript
void set_shrink_width(shrink: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_submenu_popup_delay {#set_submenu_popup_delay}

```gdscript
void set_submenu_popup_delay(seconds: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_system_menu {#set_system_menu}

```gdscript
void set_system_menu(system_menu_id: NativeMenu.SystemMenus)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### toggle_item_checked {#toggle_item_checked}

```gdscript
void toggle_item_checked(index: int)
```

Toggles the check state of the item at the given index.

### toggle_item_multistate {#toggle_item_multistate}

```gdscript
void toggle_item_multistate(index: int)
```

Cycle to the next state of a multistate item. See add_multistate_item() for details.

## 属性

### hide_on_item_selection {#hide_on_item_selection}

```gdscript
hide_on_item_selection: bool
```

- void set_hide_on_item_selection(value: bool)
- bool is_hide_on_item_selection()

If true, hides the PopupMenu when an item is selected.

### hide_on_checkable_item_selection {#hide_on_checkable_item_selection}

```gdscript
hide_on_checkable_item_selection: bool
```

- void set_hide_on_checkable_item_selection(value: bool)
- bool is_hide_on_checkable_item_selection()

If true, hides the PopupMenu when a checkbox or radio button is selected.

### hide_on_state_item_selection {#hide_on_state_item_selection}

```gdscript
hide_on_state_item_selection: bool
```

- void set_hide_on_state_item_selection(value: bool)
- bool is_hide_on_state_item_selection()

If true, hides the PopupMenu when a state item is selected.

### submenu_popup_delay {#submenu_popup_delay}

```gdscript
submenu_popup_delay: float
```

- void set_submenu_popup_delay(value: float)
- float get_submenu_popup_delay()

Sets the delay time in seconds for the submenu item to popup on mouse hovering. If the popup menu is added as a child of another (acting as a submenu), it will inherit the delay time of the parent menu item.

Note: If the mouse is exiting a submenu item with an open submenu and enters a different submenu item, the submenu popup delay time is affected by the direction of the mouse movement toward the open submenu. If the mouse is moving toward the submenu, the open submenu will wait approximately 0.5 seconds before closing, which then allows the hovered submenu item to open. This additional delay allows the mouse time to move to the open submenu across other menu items without prematurely closing. If the mouse is not moving toward the open submenu, for example in a downward direction, the open submenu will close immediately.

### allow_search {#allow_search}

```gdscript
allow_search: bool
```

- void set_allow_search(value: bool)
- bool get_allow_search()

If true, allows navigating PopupMenu with letter keys.

### system_menu_id {#system_menu_id}

```gdscript
system_menu_id: int
```

- void set_system_menu(value: SystemMenus)
- SystemMenus get_system_menu()

If set to one of the values of SystemMenus, this PopupMenu is bound to the special system menu. Only one PopupMenu can be bound to each special menu at a time.

### prefer_native_menu {#prefer_native_menu}

```gdscript
prefer_native_menu: bool
```

- void set_prefer_native_menu(value: bool)
- bool is_prefer_native_menu()

If true, MenuBar will use native menu when supported.

Note: If PopupMenu is linked to StatusIndicator, MenuBar, or another PopupMenu item it can use native menu regardless of this property, use is_native_menu() to check it.

### shrink_height {#shrink_height}

```gdscript
shrink_height: bool
```

- void set_shrink_height(value: bool)
- bool get_shrink_height()

If true, shrinks PopupMenu to minimum height when it's shown.

### shrink_width {#shrink_width}

```gdscript
shrink_width: bool
```

- void set_shrink_width(value: bool)
- bool get_shrink_width()

If true, shrinks PopupMenu to minimum width when it's shown.

### search_bar_enabled {#search_bar_enabled}

```gdscript
search_bar_enabled: bool
```

- void set_search_bar_enabled(value: bool)
- bool is_search_bar_enabled()

If true, shows a search bar at the top of the PopupMenu for filtering items. See search_bar_min_item_count for dynamically controlling its visibility based on the number of items.

Note: When enabled, allow_search is ignored.

### search_bar_min_item_count {#search_bar_min_item_count}

```gdscript
search_bar_min_item_count: int
```

- void set_search_bar_min_item_count(value: int)
- int get_search_bar_min_item_count()

Sets the minimum number of items required for the search bar to be visible. search_bar_enabled must be true for this to have any effect. Separator items are not counted.

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

The number of items currently in the list.

## 信号

### id_pressed {#id_pressed}

```gdscript
signal id_pressed(id: int)
```

Emitted when an item of some id is pressed. Also emitted when its accelerator is activated on macOS.

Note: If id is negative (either explicitly or due to overflow), this will return the corresponding index instead.

### id_focused {#id_focused}

```gdscript
signal id_focused(id: int)
```

Emitted when the user navigated to an item of some id using the ProjectSettings.input/ui_up or ProjectSettings.input/ui_down input action.

### index_pressed {#index_pressed}

```gdscript
signal index_pressed(index: int)
```

Emitted when an item of some index is pressed. Also emitted when its accelerator is activated on macOS.

### menu_changed {#menu_changed}

```gdscript
signal menu_changed()
```

Emitted when any item is added, modified or removed.
