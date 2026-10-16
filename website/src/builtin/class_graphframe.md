# GraphFrame

**继承**：[GraphElement](class_graphelement.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **GraphFrame**

> GraphFrame is a special GraphElement to which other GraphElements can be attached. It can be configured to automatically resize to enclose all attached GraphElements. If the frame is moved, all the attached GraphElements inside it will be moved as well.

<details><summary>官方英文描述</summary>

GraphFrame is a special GraphElement to which other GraphElements can be attached. It can be configured to automatically resize to enclose all attached GraphElements. If the frame is moved, all the attached GraphElements inside it will be moved as well.

A GraphFrame is always kept behind the connection layer and other GraphElements inside a GraphEdit.

</details>

## 方法

### get_autoshrink_margin {#get_autoshrink_margin}

```gdscript
int get_autoshrink_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_margin {#get_drag_margin}

```gdscript
int get_drag_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tint_color {#get_tint_color}

```gdscript
Color get_tint_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title {#get_title}

```gdscript
String get_title() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_titlebar_hbox {#get_titlebar_hbox}

```gdscript
HBoxContainer get_titlebar_hbox()
```

Returns the HBoxContainer used for the title bar, only containing a Label for displaying the title by default.

This can be used to add custom controls to the title bar such as option or close buttons.

### is_autoshrink_enabled {#is_autoshrink_enabled}

```gdscript
bool is_autoshrink_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_tint_color_enabled {#is_tint_color_enabled}

```gdscript
bool is_tint_color_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autoshrink_enabled {#set_autoshrink_enabled}

```gdscript
void set_autoshrink_enabled(shrink: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autoshrink_margin {#set_autoshrink_margin}

```gdscript
void set_autoshrink_margin(autoshrink_margin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_margin {#set_drag_margin}

```gdscript
void set_drag_margin(drag_margin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tint_color {#set_tint_color}

```gdscript
void set_tint_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tint_color_enabled {#set_tint_color_enabled}

```gdscript
void set_tint_color_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title {#set_title}

```gdscript
void set_title(title: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### title {#title}

```gdscript
title: String
```

- void set_title(value: String)
- String get_title()

Title of the frame.

### autoshrink_enabled {#autoshrink_enabled}

```gdscript
autoshrink_enabled: bool
```

- void set_autoshrink_enabled(value: bool)
- bool is_autoshrink_enabled()

If true, the frame's rect will be adjusted automatically to enclose all attached GraphElements.

### autoshrink_margin {#autoshrink_margin}

```gdscript
autoshrink_margin: int
```

- void set_autoshrink_margin(value: int)
- int get_autoshrink_margin()

The margin around the attached nodes that is used to calculate the size of the frame when autoshrink_enabled is true.

### drag_margin {#drag_margin}

```gdscript
drag_margin: int
```

- void set_drag_margin(value: int)
- int get_drag_margin()

The margin inside the frame that can be used to drag the frame.

### tint_color_enabled {#tint_color_enabled}

```gdscript
tint_color_enabled: bool
```

- void set_tint_color_enabled(value: bool)
- bool is_tint_color_enabled()

If true, the tint color will be used to tint the frame.

### tint_color {#tint_color}

```gdscript
tint_color: Color
```

- void set_tint_color(value: Color)
- Color get_tint_color()

The color of the frame when tint_color_enabled is true.

## 信号

### autoshrink_changed {#autoshrink_changed}

```gdscript
signal autoshrink_changed()
```

Emitted when autoshrink_enabled or autoshrink_margin changes.
