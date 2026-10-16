# SubViewportContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **SubViewportContainer**

> A container that displays the contents of underlying SubViewport child nodes. It uses the combined size of the SubViewports as minimum size, unless stretch is enabled.

<details><summary>官方英文描述</summary>

A container that displays the contents of underlying SubViewport child nodes. It uses the combined size of the SubViewports as minimum size, unless stretch is enabled.

Note: Changing a SubViewportContainer's Control.scale will cause its contents to appear distorted. To change its visual size without causing distortion, adjust the node's margins instead (if it's not already in a container).

Note: The SubViewportContainer forwards mouse-enter and mouse-exit notifications to its sub-viewports.

</details>

## 方法

### _propagate_input_event {#_propagate_input_event}

```gdscript
bool _propagate_input_event(event: InputEvent) virtual const
```

Experimental: This method may be changed or removed in future versions.

Virtual method to be implemented by the user. If it returns true, the event is propagated to SubViewport children. Propagation doesn't happen if it returns false. If the function is not implemented, all events are propagated to SubViewports.

### get_stretch_shrink {#get_stretch_shrink}

```gdscript
int get_stretch_shrink() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_mouse_target_enabled {#is_mouse_target_enabled}

```gdscript
bool is_mouse_target_enabled()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_stretch_enabled {#is_stretch_enabled}

```gdscript
bool is_stretch_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mouse_target {#set_mouse_target}

```gdscript
void set_mouse_target(amount: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stretch {#set_stretch}

```gdscript
void set_stretch(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stretch_shrink {#set_stretch_shrink}

```gdscript
void set_stretch_shrink(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### stretch {#stretch}

```gdscript
stretch: bool
```

- void set_stretch(value: bool)
- bool is_stretch_enabled()

If true, the sub-viewport will be automatically resized to the control's size.

Note: If true, this will prohibit changing SubViewport.size of its children manually.

### stretch_shrink {#stretch_shrink}

```gdscript
stretch_shrink: int
```

- void set_stretch_shrink(value: int)
- int get_stretch_shrink()

Divides the sub-viewport's effective resolution by this value while preserving its scale. This can be used to speed up rendering.

For example, a 1280×720 sub-viewport with stretch_shrink set to 2 will be rendered at 640×360 while occupying the same size in the container.

Note: stretch must be true for this property to work.

### mouse_target {#mouse_target}

```gdscript
mouse_target: bool
```

- void set_mouse_target(value: bool)
- bool is_mouse_target_enabled()

Configure, if either the SubViewportContainer or alternatively the Control nodes of its SubViewport children should be available as targets of mouse-related functionalities, like identifying the drop target in drag-and-drop operations or cursor shape of hovered Control node.

If false, the Control nodes inside its SubViewport children are considered as targets.

If true, the SubViewportContainer itself will be considered as a target.
