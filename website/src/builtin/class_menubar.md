# MenuBar

**继承**：[Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **MenuBar**

> A horizontal menu bar that creates a menu for each PopupMenu child. New items are created by adding PopupMenus to this node. Item title is determined by Window.title, or node name if Window.title is empty. Item title can be overridden using set_menu_title().

<details><summary>官方英文描述</summary>

A horizontal menu bar that creates a menu for each PopupMenu child. New items are created by adding PopupMenus to this node. Item title is determined by Window.title, or node name if Window.title is empty. Item title can be overridden using set_menu_title().

</details>

## 方法

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_menu_count {#get_menu_count}

```gdscript
int get_menu_count() const
```

Returns number of menu items.

### get_menu_popup {#get_menu_popup}

```gdscript
PopupMenu get_menu_popup(menu: int) const
```

Returns PopupMenu associated with menu item.

### get_menu_title {#get_menu_title}

```gdscript
String get_menu_title(menu: int) const
```

Returns menu item title.

### get_menu_tooltip {#get_menu_tooltip}

```gdscript
String get_menu_tooltip(menu: int) const
```

Returns menu item tooltip.

### get_start_index {#get_start_index}

```gdscript
int get_start_index() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_direction {#get_text_direction}

```gdscript
Control.TextDirection get_text_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_flat {#is_flat}

```gdscript
bool is_flat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_menu_disabled {#is_menu_disabled}

```gdscript
bool is_menu_disabled(menu: int) const
```

Returns true if the menu item is disabled.

### is_menu_hidden {#is_menu_hidden}

```gdscript
bool is_menu_hidden(menu: int) const
```

Returns true if the menu item is hidden.

### is_native_menu {#is_native_menu}

```gdscript
bool is_native_menu() const
```

Returns true if the current system's global menu is supported and used by this MenuBar.

### is_prefer_global_menu {#is_prefer_global_menu}

```gdscript
bool is_prefer_global_menu() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_switch_on_hover {#is_switch_on_hover}

```gdscript
bool is_switch_on_hover()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_shortcuts {#set_disable_shortcuts}

```gdscript
void set_disable_shortcuts(disabled: bool)
```

If true, shortcuts are disabled and cannot be used to trigger the button.

### set_flat {#set_flat}

```gdscript
void set_flat(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_menu_disabled {#set_menu_disabled}

```gdscript
void set_menu_disabled(menu: int, disabled: bool)
```

If true, menu item is disabled.

### set_menu_hidden {#set_menu_hidden}

```gdscript
void set_menu_hidden(menu: int, hidden: bool)
```

If true, menu item is hidden.

### set_menu_title {#set_menu_title}

```gdscript
void set_menu_title(menu: int, title: String)
```

Sets menu item title.

### set_menu_tooltip {#set_menu_tooltip}

```gdscript
void set_menu_tooltip(menu: int, tooltip: String)
```

Sets menu item tooltip.

### set_prefer_global_menu {#set_prefer_global_menu}

```gdscript
void set_prefer_global_menu(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_start_index {#set_start_index}

```gdscript
void set_start_index(enabled: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_switch_on_hover {#set_switch_on_hover}

```gdscript
void set_switch_on_hover(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_direction {#set_text_direction}

```gdscript
void set_text_direction(direction: Control.TextDirection)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### flat {#flat}

```gdscript
flat: bool
```

- void set_flat(value: bool)
- bool is_flat()

Flat MenuBar don't display item decoration.

### start_index {#start_index}

```gdscript
start_index: int
```

- void set_start_index(value: int)
- int get_start_index()

Position order in the global menu to insert MenuBar items at. All menu items in the MenuBar are always inserted as a continuous range. Menus with lower start_index are inserted first. Menus with start_index equal to -1 are inserted last.

### switch_on_hover {#switch_on_hover}

```gdscript
switch_on_hover: bool
```

- void set_switch_on_hover(value: bool)
- bool is_switch_on_hover()

If true, when the cursor hovers above menu item, it will close the current PopupMenu and open the other one.

### prefer_global_menu {#prefer_global_menu}

```gdscript
prefer_global_menu: bool
```

- void set_prefer_global_menu(value: bool)
- bool is_prefer_global_menu()

If true, MenuBar will use system global menu when supported.

Note: If true and global menu is supported, this node is not displayed, has zero size, and all its child nodes except PopupMenus are inaccessible.

Note: This property overrides the value of the PopupMenu.prefer_native_menu property of the child nodes.

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
