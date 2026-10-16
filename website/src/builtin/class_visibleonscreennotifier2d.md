# VisibleOnScreenNotifier2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **VisibleOnScreenNotifier2D**

> VisibleOnScreenNotifier2D represents a rectangular region of 2D space. When any part of this region becomes visible on screen or in a viewport, it will emit a screen_entered signal, and likewise it will emit a screen_exited signal when no part of it remains visible.

<details><summary>官方英文描述（中文精译进行中）</summary>

VisibleOnScreenNotifier2D represents a rectangular region of 2D space. When any part of this region becomes visible on screen or in a viewport, it will emit a screen_entered signal, and likewise it will emit a screen_exited signal when no part of it remains visible.

If you want a node to be enabled automatically when this region is visible on screen, use VisibleOnScreenEnabler2D.

Note: VisibleOnScreenNotifier2D uses the render culling code to determine whether it's visible on screen, so it won't function unless CanvasItem.visible is set to true.

</details>

## 方法

### get_rect {#get_rect}

```gdscript
Rect2 get_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_on_screen {#is_on_screen}

```gdscript
bool is_on_screen() const
```

If true, the bounding rectangle is on the screen.

Note: It takes one frame for the VisibleOnScreenNotifier2D's visibility to be determined once added to the scene tree, so this method will always return false right after it is instantiated, before the draw pass.

### is_showing_rect {#is_showing_rect}

```gdscript
bool is_showing_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rect {#set_rect}

```gdscript
void set_rect(rect: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_rect {#set_show_rect}

```gdscript
void set_show_rect(show_rect: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### rect {#rect}

```gdscript
rect: Rect2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### show_rect {#show_rect}

```gdscript
show_rect: bool
```

- void set_show_rect(value: bool)
- bool is_showing_rect()

If true, shows the rectangle area of rect in the editor with a translucent magenta fill. Unlike changing the visibility of the VisibleOnScreenNotifier2D, this does not affect the screen culling detection.

## 信号

### screen_entered {#screen_entered}

```gdscript
signal screen_entered()
```

Emitted when the VisibleOnScreenNotifier2D enters the screen.

### screen_exited {#screen_exited}

```gdscript
signal screen_exited()
```

Emitted when the VisibleOnScreenNotifier2D exits the screen.
