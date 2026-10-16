# MenuButton

**继承**：[Button](class_button.md) → [BaseButton](class_basebutton.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **MenuButton**

> A button that brings up a PopupMenu when clicked. To create new items inside this PopupMenu, use get_popup().add_item("My Item Name"). You can also create them directly from Godot editor's inspector.

<details><summary>官方英文描述</summary>

A button that brings up a PopupMenu when clicked. To create new items inside this PopupMenu, use get_popup().add_item("My Item Name"). You can also create them directly from Godot editor's inspector.

See also BaseButton which contains common properties and methods associated with this node.

</details>

## 方法

### get_item_count {#get_item_count}

```gdscript
int get_item_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_popup {#get_popup}

```gdscript
PopupMenu get_popup() const
```

Returns the PopupMenu contained in this button.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their Window.visible property.

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

### set_item_count {#set_item_count}

```gdscript
void set_item_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_switch_on_hover {#set_switch_on_hover}

```gdscript
void set_switch_on_hover(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### show_popup {#show_popup}

```gdscript
void show_popup()
```

Adjusts popup position and sizing for the MenuButton, then shows the PopupMenu. Prefer this over using get_popup().popup().

## 属性

### switch_on_hover {#switch_on_hover}

```gdscript
switch_on_hover: bool
```

- void set_switch_on_hover(value: bool)
- bool is_switch_on_hover()

If true, when the cursor hovers above another MenuButton within the same parent which also has switch_on_hover enabled, it will close the current MenuButton and open the other one.

### item_count {#item_count}

```gdscript
item_count: int
```

- void set_item_count(value: int)
- int get_item_count()

The number of items currently in the list.

## 信号

### about_to_popup {#about_to_popup}

```gdscript
signal about_to_popup()
```

Emitted when the PopupMenu of this MenuButton is about to show.
