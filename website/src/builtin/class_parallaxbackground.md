# ParallaxBackground

**继承**：[CanvasLayer](class_canvaslayer.md) → [Node](class_node.md) → [Object](class_object.md) → **ParallaxBackground**

> A ParallaxBackground uses one or more ParallaxLayer child nodes to create a parallax effect. Each ParallaxLayer can move at a different speed using ParallaxLayer.motion_offset. This creates an illusion of depth in a 2D game. If not used with a Camera2D, you must manually calculate the scroll_offset.

<details><summary>官方英文描述（中文精译进行中）</summary>

A ParallaxBackground uses one or more ParallaxLayer child nodes to create a parallax effect. Each ParallaxLayer can move at a different speed using ParallaxLayer.motion_offset. This creates an illusion of depth in a 2D game. If not used with a Camera2D, you must manually calculate the scroll_offset.

Note: Each ParallaxBackground is drawn on one specific Viewport and cannot be shared between multiple Viewports, see CanvasLayer.custom_viewport. When using multiple Viewports, for example in a split-screen game, you need create an individual ParallaxBackground for each Viewport you want it to be drawn on.

</details>

## 方法

### get_limit_begin {#get_limit_begin}

```gdscript
Vector2 get_limit_begin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_limit_end {#get_limit_end}

```gdscript
Vector2 get_limit_end() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scroll_base_offset {#get_scroll_base_offset}

```gdscript
Vector2 get_scroll_base_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scroll_base_scale {#get_scroll_base_scale}

```gdscript
Vector2 get_scroll_base_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scroll_offset {#get_scroll_offset}

```gdscript
Vector2 get_scroll_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_ignore_camera_zoom {#is_ignore_camera_zoom}

```gdscript
bool is_ignore_camera_zoom()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ignore_camera_zoom {#set_ignore_camera_zoom}

```gdscript
void set_ignore_camera_zoom(ignore: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_limit_begin {#set_limit_begin}

```gdscript
void set_limit_begin(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_limit_end {#set_limit_end}

```gdscript
void set_limit_end(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_base_offset {#set_scroll_base_offset}

```gdscript
void set_scroll_base_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_base_scale {#set_scroll_base_scale}

```gdscript
void set_scroll_base_scale(scale: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scroll_offset {#set_scroll_offset}

```gdscript
void set_scroll_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### scroll_offset {#scroll_offset}

```gdscript
scroll_offset: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scroll_base_offset {#scroll_base_offset}

```gdscript
scroll_base_offset: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scroll_base_scale {#scroll_base_scale}

```gdscript
scroll_base_scale: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scroll_limit_begin {#scroll_limit_begin}

```gdscript
scroll_limit_begin: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scroll_limit_end {#scroll_limit_end}

```gdscript
scroll_limit_end: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### scroll_ignore_camera_zoom {#scroll_ignore_camera_zoom}

```gdscript
scroll_ignore_camera_zoom: bool
```

- void set_ignore_camera_zoom(value: bool)
- bool is_ignore_camera_zoom()

If true, elements in ParallaxLayer child aren't affected by the zoom level of the camera.
