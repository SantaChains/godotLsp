# Slider

**继承**：[Range](class_range.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Slider**

> Abstract base class for sliders, used to adjust a value by moving a grabber along a horizontal or vertical axis. Sliders are Range-based controls.

<details><summary>官方英文描述（中文精译进行中）</summary>

Abstract base class for sliders, used to adjust a value by moving a grabber along a horizontal or vertical axis. Sliders are Range-based controls.

</details>

## 方法

### get_ticks {#get_ticks}

```gdscript
int get_ticks() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ticks_on_borders {#get_ticks_on_borders}

```gdscript
bool get_ticks_on_borders() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ticks_position {#get_ticks_position}

```gdscript
Slider.TickPosition get_ticks_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editable {#is_editable}

```gdscript
bool is_editable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_scrollable {#is_scrollable}

```gdscript
bool is_scrollable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_editable {#set_editable}

```gdscript
void set_editable(editable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scrollable {#set_scrollable}

```gdscript
void set_scrollable(scrollable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ticks {#set_ticks}

```gdscript
void set_ticks(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ticks_on_borders {#set_ticks_on_borders}

```gdscript
void set_ticks_on_borders(ticks_on_border: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ticks_position {#set_ticks_position}

```gdscript
void set_ticks_position(ticks_on_border: Slider.TickPosition)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### editable {#editable}

```gdscript
editable: bool
```

- void set_editable(value: bool)
- bool is_editable()

If true, the slider can be interacted with. If false, the value can be changed only by code.

### scrollable {#scrollable}

```gdscript
scrollable: bool
```

- void set_scrollable(value: bool)
- bool is_scrollable()

If true, the value can be changed using the mouse wheel.

### tick_count {#tick_count}

```gdscript
tick_count: int
```

- void set_ticks(value: int)
- int get_ticks()

Number of ticks displayed on the slider, including border ticks. Ticks are uniformly-distributed value markers.

### ticks_on_borders {#ticks_on_borders}

```gdscript
ticks_on_borders: bool
```

- void set_ticks_on_borders(value: bool)
- bool get_ticks_on_borders()

If true, the slider will display ticks for minimum and maximum values.

### ticks_position {#ticks_position}

```gdscript
ticks_position: int
```

- void set_ticks_position(value: TickPosition)
- TickPosition get_ticks_position()

Sets the position of the ticks. See TickPosition for details.

## 信号

### drag_started {#drag_started}

```gdscript
signal drag_started()
```

Emitted when the grabber starts being dragged. This is emitted before the corresponding Range.value_changed signal.

### drag_ended {#drag_ended}

```gdscript
signal drag_ended(value_changed: bool)
```

Emitted when the grabber stops being dragged. If value_changed is true, Range.value is different from the value when the dragging was started.

## 枚举

### TickPosition {#enum-TickPosition}

```gdscript
enum TickPosition { TICK_POSITION_BOTTOM_RIGHT = 0, TICK_POSITION_TOP_LEFT = 1, TICK_POSITION_BOTH = 2, TICK_POSITION_CENTER = 3 }
```

TickPosition TICK_POSITION_BOTTOM_RIGHT = 0

Places the ticks at the bottom of the HSlider, or right of the VSlider.

TickPosition TICK_POSITION_TOP_LEFT = 1

Places the ticks at the top of the HSlider, or left of the VSlider.

TickPosition TICK_POSITION_BOTH = 2

Places the ticks at the both sides of the slider.

TickPosition TICK_POSITION_CENTER = 3

Places the ticks at the center of the slider.

#### TICK_POSITION_BOTTOM_RIGHT {#TICK_POSITION_BOTTOM_RIGHT}

```gdscript
TICK_POSITION_BOTTOM_RIGHT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TICK_POSITION_TOP_LEFT {#TICK_POSITION_TOP_LEFT}

```gdscript
TICK_POSITION_TOP_LEFT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TICK_POSITION_BOTH {#TICK_POSITION_BOTH}

```gdscript
TICK_POSITION_BOTH = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TICK_POSITION_CENTER {#TICK_POSITION_CENTER}

```gdscript
TICK_POSITION_CENTER = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
