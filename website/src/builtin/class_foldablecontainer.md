# FoldableContainer

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **FoldableContainer**

> A container that can be expanded/collapsed, with a title that can be filled with controls, such as buttons. This is also called an accordion.

<details><summary>官方英文描述（中文精译进行中）</summary>

A container that can be expanded/collapsed, with a title that can be filled with controls, such as buttons. This is also called an accordion.

The title can be positioned at the top or bottom of the container. The container can be expanded or collapsed by clicking the title or by pressing ui_accept when focused. Child control nodes are hidden when the container is collapsed. Ignores non-control children.

A FoldableContainer can be grouped with other FoldableContainers so that only one of them can be opened at a time; see foldable_group and FoldableGroup.

</details>

## 方法

### add_title_bar_control {#add_title_bar_control}

```gdscript
void add_title_bar_control(control: Control)
```

Adds a Control that will be placed next to the container's title, obscuring the clickable area. Prime usage is adding Button nodes, but it can be any Control.

The control will be added as a child of this container and removed from previous parent if necessary. The controls will be placed aligned to the right, with the first added control being the leftmost one.

### expand {#expand}

```gdscript
void expand()
```

Expands the container and emits folding_changed.

### fold {#fold}

```gdscript
void fold()
```

Folds the container and emits folding_changed.

### get_foldable_group {#get_foldable_group}

```gdscript
FoldableGroup get_foldable_group() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title {#get_title}

```gdscript
String get_title() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title_alignment {#get_title_alignment}

```gdscript
HorizontalAlignment get_title_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title_position {#get_title_position}

```gdscript
FoldableContainer.TitlePosition get_title_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title_text_direction {#get_title_text_direction}

```gdscript
Control.TextDirection get_title_text_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_title_text_overrun_behavior {#get_title_text_overrun_behavior}

```gdscript
TextServer.OverrunBehavior get_title_text_overrun_behavior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_folded {#is_folded}

```gdscript
bool is_folded() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_title_bar_control {#remove_title_bar_control}

```gdscript
void remove_title_bar_control(control: Control)
```

Removes a Control added with add_title_bar_control(). The node is not freed automatically, you need to use Node.queue_free().

### set_foldable_group {#set_foldable_group}

```gdscript
void set_foldable_group(button_group: FoldableGroup)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_folded {#set_folded}

```gdscript
void set_folded(folded: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title {#set_title}

```gdscript
void set_title(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title_alignment {#set_title_alignment}

```gdscript
void set_title_alignment(alignment: HorizontalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title_position {#set_title_position}

```gdscript
void set_title_position(title_position: FoldableContainer.TitlePosition)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title_text_direction {#set_title_text_direction}

```gdscript
void set_title_text_direction(text_direction: Control.TextDirection)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_title_text_overrun_behavior {#set_title_text_overrun_behavior}

```gdscript
void set_title_text_overrun_behavior(overrun_behavior: TextServer.OverrunBehavior)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### folded {#folded}

```gdscript
folded: bool
```

- void set_folded(value: bool)
- bool is_folded()

If true, the container will become folded and will hide all its children.

### title {#title}

```gdscript
title: String
```

- void set_title(value: String)
- String get_title()

The container's title text.

### title_alignment {#title_alignment}

```gdscript
title_alignment: int
```

- void set_title_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_title_alignment()

Title's horizontal text alignment.

### title_position {#title_position}

```gdscript
title_position: int
```

- void set_title_position(value: TitlePosition)
- TitlePosition get_title_position()

Title's position.

### title_text_overrun_behavior {#title_text_overrun_behavior}

```gdscript
title_text_overrun_behavior: int
```

- void set_title_text_overrun_behavior(value: OverrunBehavior)
- OverrunBehavior get_title_text_overrun_behavior()

Defines the behavior of the title when the text is longer than the available space.

### foldable_group {#foldable_group}

```gdscript
foldable_group: FoldableGroup
```

- void set_foldable_group(value: FoldableGroup)
- FoldableGroup get_foldable_group()

The FoldableGroup associated with the container. When multiple FoldableContainer nodes share the same group, only one of them is allowed to be unfolded.

### title_text_direction {#title_text_direction}

```gdscript
title_text_direction: int
```

- void set_title_text_direction(value: TextDirection)
- TextDirection get_title_text_direction()

Title text writing direction.

### language {#language}

```gdscript
language: String
```

- void set_language(value: String)
- String get_language()

Language code used for text shaping algorithms. If left empty, the current locale is used instead.

## 信号

### folding_changed {#folding_changed}

```gdscript
signal folding_changed(is_folded: bool)
```

Emitted when the container is folded/expanded.

## 枚举

### TitlePosition {#enum-TitlePosition}

```gdscript
enum TitlePosition { POSITION_TOP = 0, POSITION_BOTTOM = 1 }
```

TitlePosition POSITION_TOP = 0

Makes the title appear at the top of the container.

TitlePosition POSITION_BOTTOM = 1

Makes the title appear at the bottom of the container. Also makes all StyleBoxes flipped vertically.

#### POSITION_TOP {#POSITION_TOP}

```gdscript
POSITION_TOP = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### POSITION_BOTTOM {#POSITION_BOTTOM}

```gdscript
POSITION_BOTTOM = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
