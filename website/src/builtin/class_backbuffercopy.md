# BackBufferCopy

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **BackBufferCopy**

> Node for back-buffering the currently-displayed screen. The region defined in the BackBufferCopy node is buffered with the content of the screen it covers, or the entire screen according to the copy_mode. It can be accessed in shader scripts using the screen texture (i.e. a uniform sampler with hint_screen_texture).

<details><summary>官方英文描述</summary>

Node for back-buffering the currently-displayed screen. The region defined in the BackBufferCopy node is buffered with the content of the screen it covers, or the entire screen according to the copy_mode. It can be accessed in shader scripts using the screen texture (i.e. a uniform sampler with hint_screen_texture).

Note: Since this node inherits from Node2D (and not Control), anchors and margins won't apply to child Control-derived nodes. This can be problematic when resizing the window. To avoid this, add Control-derived nodes as siblings to the BackBufferCopy node instead of adding them as children.

</details>

## 方法

### get_copy_mode {#get_copy_mode}

```gdscript
BackBufferCopy.CopyMode get_copy_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rect {#get_rect}

```gdscript
Rect2 get_rect() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_copy_mode {#set_copy_mode}

```gdscript
void set_copy_mode(copy_mode: BackBufferCopy.CopyMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rect {#set_rect}

```gdscript
void set_rect(rect: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### copy_mode {#copy_mode}

```gdscript
copy_mode: int
```

- void set_copy_mode(value: CopyMode)
- CopyMode get_copy_mode()

Buffer mode.

### rect {#rect}

```gdscript
rect: Rect2
```

- void set_rect(value: Rect2)
- Rect2 get_rect()

The area covered by the BackBufferCopy. Only used if copy_mode is COPY_MODE_RECT.

## 枚举

### CopyMode {#enum-CopyMode}

```gdscript
enum CopyMode { COPY_MODE_DISABLED = 0, COPY_MODE_RECT = 1, COPY_MODE_VIEWPORT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COPY_MODE_DISABLED {#COPY_MODE_DISABLED}

```gdscript
COPY_MODE_DISABLED = 0
```

Disables the buffering mode. This means the BackBufferCopy node will directly use the portion of screen it covers.

#### COPY_MODE_RECT {#COPY_MODE_RECT}

```gdscript
COPY_MODE_RECT = 1
```

BackBufferCopy buffers a rectangular region.

#### COPY_MODE_VIEWPORT {#COPY_MODE_VIEWPORT}

```gdscript
COPY_MODE_VIEWPORT = 2
```

BackBufferCopy buffers the entire screen.

---
