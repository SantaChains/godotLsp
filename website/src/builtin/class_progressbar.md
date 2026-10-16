# ProgressBar

**继承**：[Range](class_range.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ProgressBar**

> A control used for visual representation of a percentage. Shows the fill percentage in the center. Can also be used to show indeterminate progress. For more fill modes, use TextureProgressBar instead.

<details><summary>官方英文描述（中文精译进行中）</summary>

A control used for visual representation of a percentage. Shows the fill percentage in the center. Can also be used to show indeterminate progress. For more fill modes, use TextureProgressBar instead.

</details>

## 方法

### get_fill_mode {#get_fill_mode}

```gdscript
int get_fill_mode()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editor_preview_indeterminate_enabled {#is_editor_preview_indeterminate_enabled}

```gdscript
bool is_editor_preview_indeterminate_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_indeterminate {#is_indeterminate}

```gdscript
bool is_indeterminate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_percentage_shown {#is_percentage_shown}

```gdscript
bool is_percentage_shown() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_editor_preview_indeterminate {#set_editor_preview_indeterminate}

```gdscript
void set_editor_preview_indeterminate(preview_indeterminate: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fill_mode {#set_fill_mode}

```gdscript
void set_fill_mode(mode: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_indeterminate {#set_indeterminate}

```gdscript
void set_indeterminate(indeterminate: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_percentage {#set_show_percentage}

```gdscript
void set_show_percentage(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### fill_mode {#fill_mode}

```gdscript
fill_mode: int
```

- void set_fill_mode(value: int)
- int get_fill_mode()

The fill direction. See FillMode for possible values.

### show_percentage {#show_percentage}

```gdscript
show_percentage: bool
```

- void set_show_percentage(value: bool)
- bool is_percentage_shown()

If true, the fill percentage is displayed on the bar.

### indeterminate {#indeterminate}

```gdscript
indeterminate: bool
```

- void set_indeterminate(value: bool)
- bool is_indeterminate()

When set to true, the progress bar indicates that something is happening with an animation, but does not show the fill percentage or value.

### editor_preview_indeterminate {#editor_preview_indeterminate}

```gdscript
editor_preview_indeterminate: bool
```

- void set_editor_preview_indeterminate(value: bool)
- bool is_editor_preview_indeterminate_enabled()

If false, the indeterminate animation will be paused in the editor.

## 枚举

### FillMode {#enum-FillMode}

```gdscript
enum FillMode { FILL_BEGIN_TO_END = 0, FILL_END_TO_BEGIN = 1, FILL_TOP_TO_BOTTOM = 2, FILL_BOTTOM_TO_TOP = 3 }
```

FillMode FILL_BEGIN_TO_END = 0

The progress bar fills from begin to end horizontally, according to the language direction. If Control.is_layout_rtl() returns false, it fills from left to right, and if it returns true, it fills from right to left.

FillMode FILL_END_TO_BEGIN = 1

The progress bar fills from end to begin horizontally, according to the language direction. If Control.is_layout_rtl() returns false, it fills from right to left, and if it returns true, it fills from left to right.

FillMode FILL_TOP_TO_BOTTOM = 2

The progress fills from top to bottom.

FillMode FILL_BOTTOM_TO_TOP = 3

The progress fills from bottom to top.

#### FILL_BEGIN_TO_END {#FILL_BEGIN_TO_END}

```gdscript
FILL_BEGIN_TO_END = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILL_END_TO_BEGIN {#FILL_END_TO_BEGIN}

```gdscript
FILL_END_TO_BEGIN = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILL_TOP_TO_BOTTOM {#FILL_TOP_TO_BOTTOM}

```gdscript
FILL_TOP_TO_BOTTOM = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILL_BOTTOM_TO_TOP {#FILL_BOTTOM_TO_TOP}

```gdscript
FILL_BOTTOM_TO_TOP = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
