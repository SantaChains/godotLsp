# BoxContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **BoxContainer**

> A container that arranges its child controls horizontally or vertically, rearranging them automatically when their minimum size changes.

<details><summary>官方英文描述</summary>

A container that arranges its child controls horizontally or vertically, rearranging them automatically when their minimum size changes.

</details>

## 方法

### add_spacer {#add_spacer}

```gdscript
Control add_spacer(begin: bool)
```

Adds a Control node to the box as a spacer. If begin is true, it will insert the Control node in front of all other children.

### get_alignment {#get_alignment}

```gdscript
BoxContainer.AlignmentMode get_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_vertical {#is_vertical}

```gdscript
bool is_vertical() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alignment {#set_alignment}

```gdscript
void set_alignment(alignment: BoxContainer.AlignmentMode)
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

### vertical {#vertical}

```gdscript
vertical: bool
```

- void set_vertical(value: bool)
- bool is_vertical()

If true, the BoxContainer will arrange its children vertically, rather than horizontally.

Can't be changed when using HBoxContainer and VBoxContainer.

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
