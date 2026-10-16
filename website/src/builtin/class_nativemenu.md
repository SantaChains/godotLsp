# NativeMenu

**继承**：[Object](class_object.md) → **NativeMenu**

> NativeMenu handles low-level access to the OS native global menu bar and popup menus.

<details><summary>官方英文描述（中文精译进行中）</summary>

NativeMenu handles low-level access to the OS native global menu bar and popup menus.

Note: This is low-level API, consider using MenuBar with MenuBar.prefer_global_menu set to true, and PopupMenu with PopupMenu.prefer_native_menu set to true.

To create a menu, use create_menu(), add menu items using add_*_item methods. To remove a menu, use free_menu().

```gdscript
var menu
func _menu_callback(item_id):
    if item_id == "ITEM_CUT":
        cut()
    elif item_id == "ITEM_COPY":
        copy()
    elif item_id == "ITEM_PASTE":
        paste()
func _enter_tree():
    # Create new menu and add items:
    menu = NativeMenu.create_menu()
    NativeMenu.add_item(menu, "Cut", _menu_callback, Callable(), "ITEM_CUT")
    NativeMenu.add_item(menu, "Copy", _menu_callback, Callable(), "ITEM_COPY")
    NativeMenu.add_separator(menu)
    NativeMenu.add_item(menu, "Paste", _menu_callback, Callable(), "ITEM_PASTE")
func _on_button_pressed():
    # Show popup menu at mouse position:
    NativeMenu.popup(menu, DisplayServer.mouse_get_position())
func _exit_tree():
    # Remove menu when it's no longer needed:
    NativeMenu.free_menu(menu)
```

</details>

## 方法

### add_check_item {#add_check_item}

```gdscript
int add_check_item(rid: RID, label: String, callback: Callable = Callable(), key_callback: Callable = Callable(), tag: Variant = null, accelerator: Key = 0, index: int = -1)
```

Adds a new checkable item with text label to the global menu rid.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

An accelerator can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The accelerator is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: The callback and key_callback Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to tag.

Note: This method is implemented on macOS and Windows.

Note: On Windows, accelerator and key_callback are ignored.

### add_icon_check_item {#add_icon_check_item}

```gdscript
int add_icon_check_item(rid: RID, icon: Texture2D, label: String, callback: Callable = Callable(), key_callback: Callable = Callable(), tag: Variant = null, accelerator: Key = 0, index: int = -1)
```

Adds a new checkable item with text label and icon icon to the global menu rid.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

An accelerator can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The accelerator is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: The callback and key_callback Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to tag.

Note: This method is implemented on macOS and Windows.

Note: On Windows, accelerator and key_callback are ignored.

### add_icon_item {#add_icon_item}

```gdscript
int add_icon_item(rid: RID, icon: Texture2D, label: String, callback: Callable = Callable(), key_callback: Callable = Callable(), tag: Variant = null, accelerator: Key = 0, index: int = -1)
```

Adds a new item with text label and icon icon to the global menu rid.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

An accelerator can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The accelerator is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: The callback and key_callback Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to tag.

Note: This method is implemented on macOS and Windows.

Note: On Windows, accelerator and key_callback are ignored.

### add_icon_radio_check_item {#add_icon_radio_check_item}

```gdscript
int add_icon_radio_check_item(rid: RID, icon: Texture2D, label: String, callback: Callable = Callable(), key_callback: Callable = Callable(), tag: Variant = null, accelerator: Key = 0, index: int = -1)
```

Adds a new radio-checkable item with text label and icon icon to the global menu rid.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

An accelerator can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The accelerator is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

Note: The callback and key_callback Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to tag.

Note: This method is implemented on macOS and Windows.

Note: On Windows, accelerator and key_callback are ignored.

### add_item {#add_item}

```gdscript
int add_item(rid: RID, label: String, callback: Callable = Callable(), key_callback: Callable = Callable(), tag: Variant = null, accelerator: Key = 0, index: int = -1)
```

Adds a new item with text label to the global menu rid.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

An accelerator can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The accelerator is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: The callback and key_callback Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to tag.

Note: This method is implemented on macOS and Windows.

Note: On Windows, accelerator and key_callback are ignored.

### add_multistate_item {#add_multistate_item}

```gdscript
int add_multistate_item(rid: RID, label: String, max_states: int, default_state: int, callback: Callable = Callable(), key_callback: Callable = Callable(), tag: Variant = null, accelerator: Key = 0, index: int = -1)
```

Adds a new item with text label to the global menu rid.

Contrarily to normal binary items, multistate items can have more than two states, as defined by max_states. Each press or activate of the item will increase the state by one. The default value is defined by default_state.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

An accelerator can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The accelerator is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: By default, there's no indication of the current item state, it should be changed manually.

Note: The callback and key_callback Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to tag.

Note: This method is implemented on macOS and Windows.

Note: On Windows, accelerator and key_callback are ignored.

### add_radio_check_item {#add_radio_check_item}

```gdscript
int add_radio_check_item(rid: RID, label: String, callback: Callable = Callable(), key_callback: Callable = Callable(), tag: Variant = null, accelerator: Key = 0, index: int = -1)
```

Adds a new radio-checkable item with text label to the global menu rid.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

An accelerator can optionally be defined, which is a keyboard shortcut that can be pressed to trigger the menu button even if it's not currently open. The accelerator is generally a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: Radio-checkable items just display a checkmark, but don't have any built-in checking behavior and must be checked/unchecked manually. See set_item_checked() for more info on how to control it.

Note: The callback and key_callback Callables need to accept exactly one Variant parameter, the parameter passed to the Callables will be the value passed to tag.

Note: This method is implemented on macOS and Windows.

Note: On Windows, accelerator and key_callback are ignored.

### add_separator {#add_separator}

```gdscript
int add_separator(rid: RID, index: int = -1)
```

Adds a separator between items to the global menu rid. Separators also occupy an index.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

Note: This method is implemented on macOS and Windows.

### add_submenu_item {#add_submenu_item}

```gdscript
int add_submenu_item(rid: RID, label: String, submenu_rid: RID, tag: Variant = null, index: int = -1)
```

Adds an item that will act as a submenu of the global menu rid. The submenu_rid argument is the RID of the global menu that will be shown when the item is clicked.

Returns index of the inserted item, it's not guaranteed to be the same as index value.

Note: This method is implemented on macOS and Windows.

### clear {#clear}

```gdscript
void clear(rid: RID)
```

Removes all items from the global menu rid.

Note: This method is implemented on macOS and Windows.

### create_menu {#create_menu}

```gdscript
RID create_menu()
```

Creates a new global menu object.

Note: This method is implemented on macOS and Windows.

### find_item_index_with_submenu {#find_item_index_with_submenu}

```gdscript
int find_item_index_with_submenu(rid: RID, submenu_rid: RID) const
```

Returns the index of the item with the submenu specified by submenu_rid. Indices are automatically assigned to each item by the engine.

Note: This method is implemented on macOS and Windows.

### find_item_index_with_tag {#find_item_index_with_tag}

```gdscript
int find_item_index_with_tag(rid: RID, tag: Variant) const
```

Returns the index of the item with the specified tag. Indices are automatically assigned to each item by the engine.

Note: This method is implemented on macOS and Windows.

### find_item_index_with_text {#find_item_index_with_text}

```gdscript
int find_item_index_with_text(rid: RID, text: String) const
```

Returns the index of the item with the specified text. Indices are automatically assigned to each item by the engine.

Note: This method is implemented on macOS and Windows.

### free_menu {#free_menu}

```gdscript
void free_menu(rid: RID)
```

Frees a global menu object created by this NativeMenu.

Note: This method is implemented on macOS and Windows.

### get_item_accelerator {#get_item_accelerator}

```gdscript
Key get_item_accelerator(rid: RID, idx: int) const
```

Returns the accelerator of the item at index idx. Accelerators are special combinations of keys that activate the item, no matter which control is focused.

Note: This method is implemented only on macOS.

### get_item_callback {#get_item_callback}

```gdscript
Callable get_item_callback(rid: RID, idx: int) const
```

Returns the callback of the item at index idx.

Note: This method is implemented on macOS and Windows.

### get_item_count {#get_item_count}

```gdscript
int get_item_count(rid: RID) const
```

Returns number of items in the global menu rid.

Note: This method is implemented on macOS and Windows.

### get_item_icon {#get_item_icon}

```gdscript
Texture2D get_item_icon(rid: RID, idx: int) const
```

Returns the icon of the item at index idx.

Note: This method is implemented on macOS and Windows.

### get_item_indentation_level {#get_item_indentation_level}

```gdscript
int get_item_indentation_level(rid: RID, idx: int) const
```

Returns the horizontal offset of the item at the given idx.

Note: This method is implemented only on macOS.

### get_item_key_callback {#get_item_key_callback}

```gdscript
Callable get_item_key_callback(rid: RID, idx: int) const
```

Returns the callback of the item accelerator at index idx.

Note: This method is implemented only on macOS.

### get_item_max_states {#get_item_max_states}

```gdscript
int get_item_max_states(rid: RID, idx: int) const
```

Returns number of states of a multistate item. See add_multistate_item() for details.

Note: This method is implemented on macOS and Windows.

### get_item_state {#get_item_state}

```gdscript
int get_item_state(rid: RID, idx: int) const
```

Returns the state of a multistate item. See add_multistate_item() for details.

Note: This method is implemented on macOS and Windows.

### get_item_submenu {#get_item_submenu}

```gdscript
RID get_item_submenu(rid: RID, idx: int) const
```

Returns the submenu ID of the item at index idx. See add_submenu_item() for more info on how to add a submenu.

Note: This method is implemented on macOS and Windows.

### get_item_tag {#get_item_tag}

```gdscript
Variant get_item_tag(rid: RID, idx: int) const
```

Returns the metadata of the specified item, which might be of any type. You can set it with set_item_tag(), which provides a simple way of assigning context data to items.

Note: This method is implemented on macOS and Windows.

### get_item_text {#get_item_text}

```gdscript
String get_item_text(rid: RID, idx: int) const
```

Returns the text of the item at index idx.

Note: This method is implemented on macOS and Windows.

### get_item_tooltip {#get_item_tooltip}

```gdscript
String get_item_tooltip(rid: RID, idx: int) const
```

Returns the tooltip associated with the specified index idx.

Note: This method is implemented only on macOS.

### get_minimum_width {#get_minimum_width}

```gdscript
float get_minimum_width(rid: RID) const
```

Returns global menu minimum width.

Note: This method is implemented only on macOS.

### get_popup_close_callback {#get_popup_close_callback}

```gdscript
Callable get_popup_close_callback(rid: RID) const
```

Returns global menu close callback.

Note: This method is implemented on macOS and Windows.

### get_popup_open_callback {#get_popup_open_callback}

```gdscript
Callable get_popup_open_callback(rid: RID) const
```

Returns global menu open callback.

Note: This method is implemented only on macOS.

### get_size {#get_size}

```gdscript
Vector2 get_size(rid: RID) const
```

Returns global menu size.

Note: This method is implemented on macOS and Windows.

### get_system_menu {#get_system_menu}

```gdscript
RID get_system_menu(menu_id: NativeMenu.SystemMenus) const
```

Returns RID of a special system menu.

Note: This method is implemented only on macOS.

### get_system_menu_name {#get_system_menu_name}

```gdscript
String get_system_menu_name(menu_id: NativeMenu.SystemMenus) const
```

Returns readable name of a special system menu.

Note: This method is implemented only on macOS.

### get_system_menu_text {#get_system_menu_text}

```gdscript
String get_system_menu_text(menu_id: NativeMenu.SystemMenus) const
```

Returns the text of the system menu item.

Note: This method is implemented on macOS.

### has_feature {#has_feature}

```gdscript
bool has_feature(feature: NativeMenu.Feature) const
```

Returns true if the specified feature is supported by the current NativeMenu, false otherwise.

Note: This method is implemented on macOS and Windows.

### has_menu {#has_menu}

```gdscript
bool has_menu(rid: RID) const
```

Returns true if rid is valid global menu.

Note: This method is implemented on macOS and Windows.

### has_system_menu {#has_system_menu}

```gdscript
bool has_system_menu(menu_id: NativeMenu.SystemMenus) const
```

Returns true if a special system menu is supported.

Note: This method is implemented only on macOS.

### is_item_checkable {#is_item_checkable}

```gdscript
bool is_item_checkable(rid: RID, idx: int) const
```

Returns true if the item at index idx is checkable in some way, i.e. if it has a checkbox or radio button.

Note: This method is implemented on macOS and Windows.

### is_item_checked {#is_item_checked}

```gdscript
bool is_item_checked(rid: RID, idx: int) const
```

Returns true if the item at index idx is checked.

Note: This method is implemented on macOS and Windows.

### is_item_disabled {#is_item_disabled}

```gdscript
bool is_item_disabled(rid: RID, idx: int) const
```

Returns true if the item at index idx is disabled. When it is disabled it can't be selected, or its action invoked.

See set_item_disabled() for more info on how to disable an item.

Note: This method is implemented on macOS and Windows.

### is_item_hidden {#is_item_hidden}

```gdscript
bool is_item_hidden(rid: RID, idx: int) const
```

Returns true if the item at index idx is hidden.

See set_item_hidden() for more info on how to hide an item.

Note: This method is implemented only on macOS.

### is_item_radio_checkable {#is_item_radio_checkable}

```gdscript
bool is_item_radio_checkable(rid: RID, idx: int) const
```

Returns true if the item at index idx has radio button-style checkability.

Note: This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups.

Note: This method is implemented on macOS and Windows.

### is_opened {#is_opened}

```gdscript
bool is_opened(rid: RID) const
```

Returns true if the menu is currently opened.

Note: This method is implemented only on macOS.

### is_system_menu {#is_system_menu}

```gdscript
bool is_system_menu(rid: RID) const
```

Return true is global menu is a special system menu.

Note: This method is implemented only on macOS.

### popup {#popup}

```gdscript
void popup(rid: RID, position: Vector2i)
```

Shows the global menu at position in the screen coordinates.

Note: This method is implemented on macOS and Windows.

### remove_item {#remove_item}

```gdscript
void remove_item(rid: RID, idx: int)
```

Removes the item at index idx from the global menu rid.

Note: The indices of items after the removed item will be shifted by one.

Note: This method is implemented on macOS and Windows.

### set_interface_direction {#set_interface_direction}

```gdscript
void set_interface_direction(rid: RID, is_rtl: bool)
```

Sets the menu text layout direction from right-to-left if is_rtl is true.

Note: This method is implemented on macOS and Windows.

### set_item_accelerator {#set_item_accelerator}

```gdscript
void set_item_accelerator(rid: RID, idx: int, keycode: Key)
```

Sets the accelerator of the item at index idx. keycode can be a single Key, or a combination of KeyModifierMasks and Keys using bitwise OR such as KEY_MASK_CTRL | KEY_A (Ctrl + A).

Note: This method is implemented only on macOS.

### set_item_callback {#set_item_callback}

```gdscript
void set_item_callback(rid: RID, idx: int, callback: Callable)
```

Sets the callback of the item at index idx. Callback is emitted when an item is pressed.

Note: The callback Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the tag parameter when the menu item was created.

Note: This method is implemented on macOS and Windows.

### set_item_checkable {#set_item_checkable}

```gdscript
void set_item_checkable(rid: RID, idx: int, checkable: bool)
```

Sets whether the item at index idx has a checkbox. If false, sets the type of the item to plain text.

Note: This method is implemented on macOS and Windows.

### set_item_checked {#set_item_checked}

```gdscript
void set_item_checked(rid: RID, idx: int, checked: bool)
```

Sets the checkstate status of the item at index idx.

Note: This method is implemented on macOS and Windows.

### set_item_disabled {#set_item_disabled}

```gdscript
void set_item_disabled(rid: RID, idx: int, disabled: bool)
```

Enables/disables the item at index idx. When it is disabled, it can't be selected and its action can't be invoked.

Note: This method is implemented on macOS and Windows.

### set_item_hidden {#set_item_hidden}

```gdscript
void set_item_hidden(rid: RID, idx: int, hidden: bool)
```

Hides/shows the item at index idx. When it is hidden, an item does not appear in a menu and its action cannot be invoked.

Note: This method is implemented only on macOS.

### set_item_hover_callbacks {#set_item_hover_callbacks}

```gdscript
void set_item_hover_callbacks(rid: RID, idx: int, callback: Callable)
```

Sets the callback of the item at index idx. The callback is emitted when an item is hovered.

Note: The callback Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the tag parameter when the menu item was created.

Note: This method is implemented only on macOS.

### set_item_icon {#set_item_icon}

```gdscript
void set_item_icon(rid: RID, idx: int, icon: Texture2D)
```

Replaces the Texture2D icon of the specified idx.

Note: This method is implemented on macOS and Windows.

Note: This method is not supported by macOS Dock menu items.

### set_item_indentation_level {#set_item_indentation_level}

```gdscript
void set_item_indentation_level(rid: RID, idx: int, level: int)
```

Sets the horizontal offset of the item at the given idx.

Note: This method is implemented only on macOS.

### set_item_index {#set_item_index}

```gdscript
int set_item_index(rid: RID, idx: int, target_idx: int)
```

Changes the index of the item at index idx to be at index target_idx. This can be used to move an item above other items.

Returns the new index of the moved item, it's not guaranteed to be the same as target_idx.

Note: The indices of any items between index idx and index target_idx will be shifted by one.

Note: This method is implemented on macOS and Windows.

### set_item_key_callback {#set_item_key_callback}

```gdscript
void set_item_key_callback(rid: RID, idx: int, key_callback: Callable)
```

Sets the callback of the item at index idx. Callback is emitted when its accelerator is activated.

Note: The key_callback Callable needs to accept exactly one Variant parameter, the parameter passed to the Callable will be the value passed to the tag parameter when the menu item was created.

Note: This method is implemented only on macOS.

### set_item_max_states {#set_item_max_states}

```gdscript
void set_item_max_states(rid: RID, idx: int, max_states: int)
```

Sets number of state of a multistate item. See add_multistate_item() for details.

Note: This method is implemented on macOS and Windows.

### set_item_radio_checkable {#set_item_radio_checkable}

```gdscript
void set_item_radio_checkable(rid: RID, idx: int, checkable: bool)
```

Sets the type of the item at the specified index idx to radio button. If false, sets the type of the item to plain text.

Note: This is purely cosmetic; you must add the logic for checking/unchecking items in radio groups.

Note: This method is implemented on macOS and Windows.

### set_item_state {#set_item_state}

```gdscript
void set_item_state(rid: RID, idx: int, state: int)
```

Sets the state of a multistate item. See add_multistate_item() for details.

Note: This method is implemented on macOS and Windows.

### set_item_submenu {#set_item_submenu}

```gdscript
void set_item_submenu(rid: RID, idx: int, submenu_rid: RID)
```

Sets the submenu RID of the item at index idx. The submenu is a global menu that would be shown when the item is clicked.

Note: This method is implemented on macOS and Windows.

### set_item_tag {#set_item_tag}

```gdscript
void set_item_tag(rid: RID, idx: int, tag: Variant)
```

Sets the metadata of an item, which may be of any type. You can later get it with get_item_tag(), which provides a simple way of assigning context data to items.

Note: This method is implemented on macOS and Windows.

### set_item_text {#set_item_text}

```gdscript
void set_item_text(rid: RID, idx: int, text: String)
```

Sets the text of the item at index idx.

Note: This method is implemented on macOS and Windows.

### set_item_tooltip {#set_item_tooltip}

```gdscript
void set_item_tooltip(rid: RID, idx: int, tooltip: String)
```

Sets the String tooltip of the item at the specified index idx.

Note: This method is implemented only on macOS.

### set_minimum_width {#set_minimum_width}

```gdscript
void set_minimum_width(rid: RID, width: float)
```

Sets the minimum width of the global menu.

Note: This method is implemented only on macOS.

### set_popup_close_callback {#set_popup_close_callback}

```gdscript
void set_popup_close_callback(rid: RID, callback: Callable)
```

Registers callable to emit when the menu is about to show.

Note: The OS can simulate menu opening to track menu item changes and global shortcuts, in which case the corresponding close callback is not triggered. Use is_opened() to check if the menu is currently opened.

Note: This method is implemented on macOS and Windows.

### set_popup_open_callback {#set_popup_open_callback}

```gdscript
void set_popup_open_callback(rid: RID, callback: Callable)
```

Registers callable to emit after the menu is closed.

Note: This method is implemented only on macOS.

### set_system_menu_text {#set_system_menu_text}

```gdscript
void set_system_menu_text(menu_id: NativeMenu.SystemMenus, name: String)
```

Sets the text of the system menu item.

Note: This method is implemented on macOS.

## 枚举

### Feature {#enum-Feature}

```gdscript
enum Feature { FEATURE_GLOBAL_MENU = 0, FEATURE_POPUP_MENU = 1, FEATURE_OPEN_CLOSE_CALLBACK = 2, FEATURE_HOVER_CALLBACK = 3, FEATURE_KEY_CALLBACK = 4 }
```

Feature FEATURE_GLOBAL_MENU = 0

NativeMenu supports native global main menu.

Feature FEATURE_POPUP_MENU = 1

NativeMenu supports native popup menus.

Feature FEATURE_OPEN_CLOSE_CALLBACK = 2

NativeMenu supports menu open and close callbacks.

Feature FEATURE_HOVER_CALLBACK = 3

NativeMenu supports menu item hover callback.

Feature FEATURE_KEY_CALLBACK = 4

NativeMenu supports menu item accelerator/key callback.

#### FEATURE_GLOBAL_MENU {#FEATURE_GLOBAL_MENU}

```gdscript
FEATURE_GLOBAL_MENU = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEATURE_POPUP_MENU {#FEATURE_POPUP_MENU}

```gdscript
FEATURE_POPUP_MENU = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEATURE_OPEN_CLOSE_CALLBACK {#FEATURE_OPEN_CLOSE_CALLBACK}

```gdscript
FEATURE_OPEN_CLOSE_CALLBACK = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEATURE_HOVER_CALLBACK {#FEATURE_HOVER_CALLBACK}

```gdscript
FEATURE_HOVER_CALLBACK = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEATURE_KEY_CALLBACK {#FEATURE_KEY_CALLBACK}

```gdscript
FEATURE_KEY_CALLBACK = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### SystemMenus {#enum-SystemMenus}

```gdscript
enum SystemMenus { INVALID_MENU_ID = 0, MAIN_MENU_ID = 1, APPLICATION_MENU_ID = 2, WINDOW_MENU_ID = 3, HELP_MENU_ID = 4, DOCK_MENU_ID = 5 }
```

SystemMenus INVALID_MENU_ID = 0

Invalid special system menu ID.

SystemMenus MAIN_MENU_ID = 1

Global main menu ID.

SystemMenus APPLICATION_MENU_ID = 2

Application (first menu after "Apple" menu on macOS) menu ID.

SystemMenus WINDOW_MENU_ID = 3

"Window" menu ID (on macOS this menu includes standard window control items and a list of open windows).

SystemMenus HELP_MENU_ID = 4

"Help" menu ID (on macOS this menu includes help search bar).

SystemMenus DOCK_MENU_ID = 5

Dock icon right-click menu ID (on macOS this menu include standard application control items and a list of open windows).

#### INVALID_MENU_ID {#INVALID_MENU_ID}

```gdscript
INVALID_MENU_ID = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MAIN_MENU_ID {#MAIN_MENU_ID}

```gdscript
MAIN_MENU_ID = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### APPLICATION_MENU_ID {#APPLICATION_MENU_ID}

```gdscript
APPLICATION_MENU_ID = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### WINDOW_MENU_ID {#WINDOW_MENU_ID}

```gdscript
WINDOW_MENU_ID = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HELP_MENU_ID {#HELP_MENU_ID}

```gdscript
HELP_MENU_ID = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOCK_MENU_ID {#DOCK_MENU_ID}

```gdscript
DOCK_MENU_ID = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
