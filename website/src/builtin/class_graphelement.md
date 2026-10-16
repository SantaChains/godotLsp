# GraphElement

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **GraphElement**

> GraphElement allows to create custom elements for a GraphEdit graph. By default such elements can be selected, resized, and repositioned, but they cannot be connected. For a graph element that allows for connections see GraphNode.

<details><summary>官方英文描述</summary>

GraphElement allows to create custom elements for a GraphEdit graph. By default such elements can be selected, resized, and repositioned, but they cannot be connected. For a graph element that allows for connections see GraphNode.

</details>

## 方法

### get_position_offset {#get_position_offset}

```gdscript
Vector2 get_position_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draggable {#is_draggable}

```gdscript
bool is_draggable()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_resizable {#is_resizable}

```gdscript
bool is_resizable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scaling_menus {#is_scaling_menus}

```gdscript
bool is_scaling_menus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_selectable {#is_selectable}

```gdscript
bool is_selectable()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_selected {#is_selected}

```gdscript
bool is_selected()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draggable {#set_draggable}

```gdscript
void set_draggable(draggable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position_offset {#set_position_offset}

```gdscript
void set_position_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_resizable {#set_resizable}

```gdscript
void set_resizable(resizable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scaling_menus {#set_scaling_menus}

```gdscript
void set_scaling_menus(scaling_menus: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selectable {#set_selectable}

```gdscript
void set_selectable(selectable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selected {#set_selected}

```gdscript
void set_selected(selected: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### position_offset {#position_offset}

```gdscript
position_offset: Vector2
```

- void set_position_offset(value: Vector2)
- Vector2 get_position_offset()

The offset of the GraphElement, relative to the scroll offset of the GraphEdit.

### resizable {#resizable}

```gdscript
resizable: bool
```

- void set_resizable(value: bool)
- bool is_resizable()

If true, the user can resize the GraphElement.

Note: Dragging the handle will only emit the resize_request and resize_end signals, the GraphElement needs to be resized manually.

### draggable {#draggable}

```gdscript
draggable: bool
```

- void set_draggable(value: bool)
- bool is_draggable()

If true, the user can drag the GraphElement.

### selectable {#selectable}

```gdscript
selectable: bool
```

- void set_selectable(value: bool)
- bool is_selectable()

If true, the user can select the GraphElement.

### selected {#selected}

```gdscript
selected: bool
```

- void set_selected(value: bool)
- bool is_selected()

If true, the GraphElement is selected.

### scaling_menus {#scaling_menus}

```gdscript
scaling_menus: bool
```

- void set_scaling_menus(value: bool)
- bool is_scaling_menus()

If true, PopupMenus that are descendants of the GraphElement are scaled with the GraphEdit zoom.

## 信号

### node_selected {#node_selected}

```gdscript
signal node_selected()
```

Emitted when the GraphElement is selected.

### node_deselected {#node_deselected}

```gdscript
signal node_deselected()
```

Emitted when the GraphElement is deselected.

### raise_request {#raise_request}

```gdscript
signal raise_request()
```

Emitted when displaying the GraphElement over other ones is requested. Happens on focusing (clicking into) the GraphElement.

### delete_request {#delete_request}

```gdscript
signal delete_request()
```

Emitted when removing the GraphElement is requested.

### resize_request {#resize_request}

```gdscript
signal resize_request(new_size: Vector2)
```

Emitted when resizing the GraphElement is requested. Happens on dragging the resizer handle (see resizable).

### resize_end {#resize_end}

```gdscript
signal resize_end(new_size: Vector2)
```

Emitted when releasing the mouse button after dragging the resizer handle (see resizable).

### dragged {#dragged}

```gdscript
signal dragged(from: Vector2, to: Vector2)
```

Emitted when the GraphElement is dragged.

### position_offset_changed {#position_offset_changed}

```gdscript
signal position_offset_changed()
```

Emitted when the GraphElement is moved.
