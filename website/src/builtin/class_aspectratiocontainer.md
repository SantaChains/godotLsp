# AspectRatioContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **AspectRatioContainer**

> A container type that arranges its child controls in a way that preserves their proportions automatically when the container is resized. Useful when a container has a dynamic size and the child nodes must adjust their sizes accordingly without losing their aspect ratios.

<details><summary>官方英文描述（中文精译进行中）</summary>

A container type that arranges its child controls in a way that preserves their proportions automatically when the container is resized. Useful when a container has a dynamic size and the child nodes must adjust their sizes accordingly without losing their aspect ratios.

</details>

## 方法

### get_alignment_horizontal {#get_alignment_horizontal}

```gdscript
AspectRatioContainer.AlignmentMode get_alignment_horizontal() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_alignment_vertical {#get_alignment_vertical}

```gdscript
AspectRatioContainer.AlignmentMode get_alignment_vertical() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ratio {#get_ratio}

```gdscript
float get_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stretch_mode {#get_stretch_mode}

```gdscript
AspectRatioContainer.StretchMode get_stretch_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alignment_horizontal {#set_alignment_horizontal}

```gdscript
void set_alignment_horizontal(alignment_horizontal: AspectRatioContainer.AlignmentMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alignment_vertical {#set_alignment_vertical}

```gdscript
void set_alignment_vertical(alignment_vertical: AspectRatioContainer.AlignmentMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ratio {#set_ratio}

```gdscript
void set_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stretch_mode {#set_stretch_mode}

```gdscript
void set_stretch_mode(stretch_mode: AspectRatioContainer.StretchMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### ratio {#ratio}

```gdscript
ratio: float
```

- void set_ratio(value: float)
- float get_ratio()

The aspect ratio to enforce on child controls. This is the width divided by the height. The ratio depends on the stretch_mode.

### stretch_mode {#stretch_mode}

```gdscript
stretch_mode: int
```

- void set_stretch_mode(value: StretchMode)
- StretchMode get_stretch_mode()

The stretch mode used to align child controls.

### alignment_horizontal {#alignment_horizontal}

```gdscript
alignment_horizontal: int
```

- void set_alignment_horizontal(value: AlignmentMode)
- AlignmentMode get_alignment_horizontal()

Specifies the horizontal relative position of child controls.

### alignment_vertical {#alignment_vertical}

```gdscript
alignment_vertical: int
```

- void set_alignment_vertical(value: AlignmentMode)
- AlignmentMode get_alignment_vertical()

Specifies the vertical relative position of child controls.

## 枚举

### StretchMode {#enum-StretchMode}

```gdscript
enum StretchMode { STRETCH_WIDTH_CONTROLS_HEIGHT = 0, STRETCH_HEIGHT_CONTROLS_WIDTH = 1, STRETCH_FIT = 2, STRETCH_COVER = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STRETCH_WIDTH_CONTROLS_HEIGHT {#STRETCH_WIDTH_CONTROLS_HEIGHT}

```gdscript
STRETCH_WIDTH_CONTROLS_HEIGHT = 0
```

The height of child controls is automatically adjusted based on the width of the container.

#### STRETCH_HEIGHT_CONTROLS_WIDTH {#STRETCH_HEIGHT_CONTROLS_WIDTH}

```gdscript
STRETCH_HEIGHT_CONTROLS_WIDTH = 1
```

The width of child controls is automatically adjusted based on the height of the container.

#### STRETCH_FIT {#STRETCH_FIT}

```gdscript
STRETCH_FIT = 2
```

The bounding rectangle of child controls is automatically adjusted to fit inside the container while keeping the aspect ratio.

#### STRETCH_COVER {#STRETCH_COVER}

```gdscript
STRETCH_COVER = 3
```

The width and height of child controls is automatically adjusted to make their bounding rectangle cover the entire area of the container while keeping the aspect ratio.

When the bounding rectangle of child controls exceed the container's size and Control.clip_contents is enabled, this allows to show only the container's area restricted by its own bounding rectangle.

---

### AlignmentMode {#enum-AlignmentMode}

```gdscript
enum AlignmentMode { ALIGNMENT_BEGIN = 0, ALIGNMENT_CENTER = 1, ALIGNMENT_END = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ALIGNMENT_BEGIN {#ALIGNMENT_BEGIN}

```gdscript
ALIGNMENT_BEGIN = 0
```

Aligns child controls with the beginning (left or top) of the container.

#### ALIGNMENT_CENTER {#ALIGNMENT_CENTER}

```gdscript
ALIGNMENT_CENTER = 1
```

Aligns child controls with the center of the container.

#### ALIGNMENT_END {#ALIGNMENT_END}

```gdscript
ALIGNMENT_END = 2
```

Aligns child controls with the end (right or bottom) of the container.

---
