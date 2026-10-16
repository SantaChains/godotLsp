# FlowContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **FlowContainer**

> A container that arranges its child controls horizontally or vertically and wraps them around at the borders. This is similar to how text in a book wraps around when no more words can fit on a line.

<details><summary>官方英文描述（中文精译进行中）</summary>

A container that arranges its child controls horizontally or vertically and wraps them around at the borders. This is similar to how text in a book wraps around when no more words can fit on a line.

</details>

## 方法

### get_alignment {#get_alignment}

```gdscript
FlowContainer.AlignmentMode get_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_last_wrap_alignment {#get_last_wrap_alignment}

```gdscript
FlowContainer.LastWrapAlignmentMode get_last_wrap_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_count {#get_line_count}

```gdscript
int get_line_count() const
```

Returns the current line count.

### is_reverse_fill {#is_reverse_fill}

```gdscript
bool is_reverse_fill() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_vertical {#is_vertical}

```gdscript
bool is_vertical() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alignment {#set_alignment}

```gdscript
void set_alignment(alignment: FlowContainer.AlignmentMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_last_wrap_alignment {#set_last_wrap_alignment}

```gdscript
void set_last_wrap_alignment(last_wrap_alignment: FlowContainer.LastWrapAlignmentMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_reverse_fill {#set_reverse_fill}

```gdscript
void set_reverse_fill(reverse_fill: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical {#set_vertical}

```gdscript
void set_vertical(vertical: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### alignment {#alignment}

```gdscript
alignment: int
```

- void set_alignment(value: AlignmentMode)
- AlignmentMode get_alignment()

The alignment of the container's children (must be one of ALIGNMENT_BEGIN, ALIGNMENT_CENTER, or ALIGNMENT_END).

### last_wrap_alignment {#last_wrap_alignment}

```gdscript
last_wrap_alignment: int
```

- void set_last_wrap_alignment(value: LastWrapAlignmentMode)
- LastWrapAlignmentMode get_last_wrap_alignment()

The wrap behavior of the last, partially filled row or column (must be one of LAST_WRAP_ALIGNMENT_INHERIT, LAST_WRAP_ALIGNMENT_BEGIN, LAST_WRAP_ALIGNMENT_CENTER, or LAST_WRAP_ALIGNMENT_END).

### vertical {#vertical}

```gdscript
vertical: bool
```

- void set_vertical(value: bool)
- bool is_vertical()

If true, the FlowContainer will arrange its children vertically, rather than horizontally.

Can't be changed when using HFlowContainer and VFlowContainer.

### reverse_fill {#reverse_fill}

```gdscript
reverse_fill: bool
```

- void set_reverse_fill(value: bool)
- bool is_reverse_fill()

If true, reverses fill direction. Horizontal FlowContainers will fill rows bottom to top, vertical FlowContainers will fill columns right to left.

When using a vertical FlowContainer with a right to left Control.layout_direction, columns will fill left to right instead.

## 枚举

### AlignmentMode {#enum-AlignmentMode}

```gdscript
enum AlignmentMode { ALIGNMENT_BEGIN = 0, ALIGNMENT_CENTER = 1, ALIGNMENT_END = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ALIGNMENT_BEGIN {#ALIGNMENT_BEGIN}

```gdscript
ALIGNMENT_BEGIN = 0
```

The child controls will be arranged at the beginning of the container, i.e. top if orientation is vertical, left if orientation is horizontal (right for RTL layout).

#### ALIGNMENT_CENTER {#ALIGNMENT_CENTER}

```gdscript
ALIGNMENT_CENTER = 1
```

The child controls will be centered in the container.

#### ALIGNMENT_END {#ALIGNMENT_END}

```gdscript
ALIGNMENT_END = 2
```

The child controls will be arranged at the end of the container, i.e. bottom if orientation is vertical, right if orientation is horizontal (left for RTL layout).

---

### LastWrapAlignmentMode {#enum-LastWrapAlignmentMode}

```gdscript
enum LastWrapAlignmentMode { LAST_WRAP_ALIGNMENT_INHERIT = 0, LAST_WRAP_ALIGNMENT_BEGIN = 1, LAST_WRAP_ALIGNMENT_CENTER = 2, LAST_WRAP_ALIGNMENT_END = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LAST_WRAP_ALIGNMENT_INHERIT {#LAST_WRAP_ALIGNMENT_INHERIT}

```gdscript
LAST_WRAP_ALIGNMENT_INHERIT = 0
```

The last partially filled row or column will wrap aligned to the previous row or column in accordance with alignment.

#### LAST_WRAP_ALIGNMENT_BEGIN {#LAST_WRAP_ALIGNMENT_BEGIN}

```gdscript
LAST_WRAP_ALIGNMENT_BEGIN = 1
```

The last partially filled row or column will wrap aligned to the beginning of the previous row or column.

#### LAST_WRAP_ALIGNMENT_CENTER {#LAST_WRAP_ALIGNMENT_CENTER}

```gdscript
LAST_WRAP_ALIGNMENT_CENTER = 2
```

The last partially filled row or column will wrap aligned to the center of the previous row or column.

#### LAST_WRAP_ALIGNMENT_END {#LAST_WRAP_ALIGNMENT_END}

```gdscript
LAST_WRAP_ALIGNMENT_END = 3
```

The last partially filled row or column will wrap aligned to the end of the previous row or column.

---
