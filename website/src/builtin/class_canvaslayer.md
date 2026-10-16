# CanvasLayer

**继承**：[Node](class_node.md) → [Object](class_object.md) → **CanvasLayer**

> CanvasItem-derived nodes that are direct or indirect children of a CanvasLayer will be drawn in that layer. The layer is a numeric index that defines the draw order. The default 2D scene renders with index 0, so a CanvasLayer with index -1 will be drawn below, and a CanvasLayer with index 1 will be drawn above. This order will hold regardless of the CanvasItem.z_index of the nodes within each layer.

<details><summary>官方英文描述（中文精译进行中）</summary>

CanvasItem-derived nodes that are direct or indirect children of a CanvasLayer will be drawn in that layer. The layer is a numeric index that defines the draw order. The default 2D scene renders with index 0, so a CanvasLayer with index -1 will be drawn below, and a CanvasLayer with index 1 will be drawn above. This order will hold regardless of the CanvasItem.z_index of the nodes within each layer.

CanvasLayers can be hidden and they can also optionally follow the viewport. This makes them useful for HUDs like health bar overlays (on layers 1 and higher) or backgrounds (on layers -1 and lower).

Note: Embedded Windows are placed on layer 1024. CanvasItems on layers 1025 and higher appear in front of embedded windows.

Note: Each CanvasLayer is drawn on one specific Viewport and cannot be shared between multiple Viewports, see custom_viewport. When using multiple Viewports, for example in a split-screen game, you need to create an individual CanvasLayer for each Viewport you want it to be drawn on.

</details>

## 方法

### get_canvas {#get_canvas}

```gdscript
RID get_canvas() const
```

Returns the RID of the canvas used by this layer.

### get_custom_viewport {#get_custom_viewport}

```gdscript
Node get_custom_viewport() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_final_transform {#get_final_transform}

```gdscript
Transform2D get_final_transform() const
```

Returns the transform from the CanvasLayers coordinate system to the Viewports coordinate system.

### get_follow_viewport_scale {#get_follow_viewport_scale}

```gdscript
float get_follow_viewport_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_layer {#get_layer}

```gdscript
int get_layer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset {#get_offset}

```gdscript
Vector2 get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation {#get_rotation}

```gdscript
float get_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scale {#get_scale}

```gdscript
Vector2 get_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transform {#get_transform}

```gdscript
Transform2D get_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### hide {#hide}

```gdscript
void hide()
```

Hides any CanvasItem under this CanvasLayer. This is equivalent to setting visible to false.

### is_following_viewport {#is_following_viewport}

```gdscript
bool is_following_viewport() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_visible {#is_visible}

```gdscript
bool is_visible() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_viewport {#set_custom_viewport}

```gdscript
void set_custom_viewport(viewport: Node)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_follow_viewport {#set_follow_viewport}

```gdscript
void set_follow_viewport(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_follow_viewport_scale {#set_follow_viewport_scale}

```gdscript
void set_follow_viewport_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_layer {#set_layer}

```gdscript
void set_layer(layer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation {#set_rotation}

```gdscript
void set_rotation(radians: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scale {#set_scale}

```gdscript
void set_scale(scale: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transform {#set_transform}

```gdscript
void set_transform(transform: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visible {#set_visible}

```gdscript
void set_visible(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### show {#show}

```gdscript
void show()
```

Shows any CanvasItem under this CanvasLayer. This is equivalent to setting visible to true.

## 属性

### layer {#layer}

```gdscript
layer: int
```

- void set_layer(value: int)
- int get_layer()

Layer index for draw order. Lower values are drawn behind higher values.

Note: If multiple CanvasLayers have the same layer index, CanvasItem children of one CanvasLayer are drawn behind the CanvasItem children of the other CanvasLayer. Which CanvasLayer is drawn in front is non-deterministic.

Note: The layer index should be between RenderingServer.CANVAS_LAYER_MIN and RenderingServer.CANVAS_LAYER_MAX (inclusive). Any other value will wrap around.

### visible {#visible}

```gdscript
visible: bool
```

- void set_visible(value: bool)
- bool is_visible()

If false, any CanvasItem under this CanvasLayer will be hidden.

Unlike CanvasItem.visible, visibility of a CanvasLayer isn't propagated to underlying layers.

### offset {#offset}

```gdscript
offset: Vector2
```

- void set_offset(value: Vector2)
- Vector2 get_offset()

The layer's base offset.

### rotation {#rotation}

```gdscript
rotation: float
```

- void set_rotation(value: float)
- float get_rotation()

The layer's rotation in radians.

### scale {#scale}

```gdscript
scale: Vector2
```

- void set_scale(value: Vector2)
- Vector2 get_scale()

The layer's scale.

### transform {#transform}

```gdscript
transform: Transform2D
```

- void set_transform(value: Transform2D)
- Transform2D get_transform()

The layer's transform.

### custom_viewport {#custom_viewport}

```gdscript
custom_viewport: Viewport
```

- void set_custom_viewport(value: Node)
- Node get_custom_viewport()

The custom Viewport node assigned to the CanvasLayer. If null, uses the default viewport instead.

### follow_viewport_enabled {#follow_viewport_enabled}

```gdscript
follow_viewport_enabled: bool
```

- void set_follow_viewport(value: bool)
- bool is_following_viewport()

If enabled, the CanvasLayer maintains its position in world space. If disabled, the CanvasLayer stays in a fixed position on the screen.

Together with follow_viewport_scale, this can be used for a pseudo-3D effect.

### follow_viewport_scale {#follow_viewport_scale}

```gdscript
follow_viewport_scale: float
```

- void set_follow_viewport_scale(value: float)
- float get_follow_viewport_scale()

Scales the layer when using follow_viewport_enabled. Layers moving into the foreground should have increasing scales, while layers moving into the background should have decreasing scales.

## 信号

### visibility_changed {#visibility_changed}

```gdscript
signal visibility_changed()
```

Emitted when visibility of the layer is changed. See visible.
