# ScrollBar

**继承**：[Range](class_range.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ScrollBar**

> Abstract base class for scrollbars, typically used to navigate through content that extends beyond the visible area of a control. Scrollbars are Range-based controls.

<details><summary>官方英文描述（中文精译进行中）</summary>

Abstract base class for scrollbars, typically used to navigate through content that extends beyond the visible area of a control. Scrollbars are Range-based controls.

</details>

## 方法

### get_custom_step {#get_custom_step}

```gdscript
float get_custom_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_step {#set_custom_step}

```gdscript
void set_custom_step(step: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### custom_step {#custom_step}

```gdscript
custom_step: float
```

- void set_custom_step(value: float)
- float get_custom_step()

Overrides the step used when clicking increment and decrement buttons or when using arrow keys when the ScrollBar is focused.

## 信号

### scrolling {#scrolling}

```gdscript
signal scrolling()
```

Emitted when the scrollbar is being scrolled.
