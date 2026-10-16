# StatusIndicator

**继承**：[Node](class_node.md) → [Object](class_object.md) → **StatusIndicator**

## 方法

### get_icon {#get_icon}

```gdscript
Texture2D get_icon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_menu {#get_menu}

```gdscript
NodePath get_menu() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rect {#get_rect}

```gdscript
Rect2 get_rect() const
```

Returns the status indicator rectangle in screen coordinates. If this status indicator is not visible, returns an empty Rect2.

### get_tooltip {#get_tooltip}

```gdscript
String get_tooltip() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_visible {#is_visible}

```gdscript
bool is_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_icon {#set_icon}

```gdscript
void set_icon(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_menu {#set_menu}

```gdscript
void set_menu(menu: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tooltip {#set_tooltip}

```gdscript
void set_tooltip(tooltip: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visible {#set_visible}

```gdscript
void set_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### tooltip {#tooltip}

```gdscript
tooltip: String
```

- void set_tooltip(value: String)
- String get_tooltip()

Status indicator tooltip.

### icon {#icon}

```gdscript
icon: Texture2D
```

- void set_icon(value: Texture2D)
- Texture2D get_icon()

Status indicator icon.

### menu {#menu}

```gdscript
menu: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### visible {#visible}

```gdscript
visible: bool
```

- void set_visible(value: bool)
- bool is_visible()

If true, the status indicator is visible.

## 信号

### pressed {#pressed}

```gdscript
signal pressed(mouse_button: int, mouse_position: Vector2i)
```

Emitted when the status indicator is pressed.
