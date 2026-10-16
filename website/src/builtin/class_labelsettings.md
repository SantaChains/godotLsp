# LabelSettings

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **LabelSettings**

> LabelSettings is a resource that provides common settings to customize the text in a Label. It will take priority over the properties defined in Control.theme. The resource can be shared between multiple labels and changed on the fly, so it's convenient and flexible way to setup text style.

<details><summary>官方英文描述（中文精译进行中）</summary>

LabelSettings is a resource that provides common settings to customize the text in a Label. It will take priority over the properties defined in Control.theme. The resource can be shared between multiple labels and changed on the fly, so it's convenient and flexible way to setup text style.

</details>

## 方法

### add_stacked_outline {#add_stacked_outline}

```gdscript
void add_stacked_outline(index: int = -1)
```

Adds a new stacked outline to the label at the given index. If index is -1, the new stacked outline will be added at the end of the list.

### add_stacked_shadow {#add_stacked_shadow}

```gdscript
void add_stacked_shadow(index: int = -1)
```

Adds a new stacked shadow to the label at the given index. If index is -1, the new stacked shadow will be added at the end of the list.

### get_font {#get_font}

```gdscript
Font get_font() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_font_color {#get_font_color}

```gdscript
Color get_font_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_font_size {#get_font_size}

```gdscript
int get_font_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_spacing {#get_line_spacing}

```gdscript
float get_line_spacing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outline_color {#get_outline_color}

```gdscript
Color get_outline_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outline_size {#get_outline_size}

```gdscript
int get_outline_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_paragraph_spacing {#get_paragraph_spacing}

```gdscript
float get_paragraph_spacing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadow_color {#get_shadow_color}

```gdscript
Color get_shadow_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadow_offset {#get_shadow_offset}

```gdscript
Vector2 get_shadow_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shadow_size {#get_shadow_size}

```gdscript
int get_shadow_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stacked_outline_color {#get_stacked_outline_color}

```gdscript
Color get_stacked_outline_color(index: int) const
```

Returns the color of the stacked outline at index.

### get_stacked_outline_count {#get_stacked_outline_count}

```gdscript
int get_stacked_outline_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stacked_outline_size {#get_stacked_outline_size}

```gdscript
int get_stacked_outline_size(index: int) const
```

Returns the size of the stacked outline at index.

### get_stacked_shadow_color {#get_stacked_shadow_color}

```gdscript
Color get_stacked_shadow_color(index: int) const
```

Returns the color of the stacked shadow at index.

### get_stacked_shadow_count {#get_stacked_shadow_count}

```gdscript
int get_stacked_shadow_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stacked_shadow_offset {#get_stacked_shadow_offset}

```gdscript
Vector2 get_stacked_shadow_offset(index: int) const
```

Returns the offset of the stacked shadow at index.

### get_stacked_shadow_outline_size {#get_stacked_shadow_outline_size}

```gdscript
int get_stacked_shadow_outline_size(index: int) const
```

Returns the outline size of the stacked shadow at index.

### move_stacked_outline {#move_stacked_outline}

```gdscript
void move_stacked_outline(from_index: int, to_position: int)
```

Moves the stacked outline at index from_index to the given position to_position in the array.

### move_stacked_shadow {#move_stacked_shadow}

```gdscript
void move_stacked_shadow(from_index: int, to_position: int)
```

Moves the stacked shadow at index from_index to the given position to_position in the array.

### remove_stacked_outline {#remove_stacked_outline}

```gdscript
void remove_stacked_outline(index: int)
```

Removes the stacked outline at index index.

### remove_stacked_shadow {#remove_stacked_shadow}

```gdscript
void remove_stacked_shadow(index: int)
```

Removes the stacked shadow at index index.

### set_font {#set_font}

```gdscript
void set_font(font: Font)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font_color {#set_font_color}

```gdscript
void set_font_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_font_size {#set_font_size}

```gdscript
void set_font_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_line_spacing {#set_line_spacing}

```gdscript
void set_line_spacing(spacing: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outline_color {#set_outline_color}

```gdscript
void set_outline_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outline_size {#set_outline_size}

```gdscript
void set_outline_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_paragraph_spacing {#set_paragraph_spacing}

```gdscript
void set_paragraph_spacing(spacing: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_color {#set_shadow_color}

```gdscript
void set_shadow_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_offset {#set_shadow_offset}

```gdscript
void set_shadow_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shadow_size {#set_shadow_size}

```gdscript
void set_shadow_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stacked_outline_color {#set_stacked_outline_color}

```gdscript
void set_stacked_outline_color(index: int, color: Color)
```

Sets the color of the stacked outline identified by the given index to color.

### set_stacked_outline_count {#set_stacked_outline_count}

```gdscript
void set_stacked_outline_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stacked_outline_size {#set_stacked_outline_size}

```gdscript
void set_stacked_outline_size(index: int, size: int)
```

Sets the size of the stacked outline identified by the given index to size.

### set_stacked_shadow_color {#set_stacked_shadow_color}

```gdscript
void set_stacked_shadow_color(index: int, color: Color)
```

Sets the color of the stacked shadow identified by the given index to color.

### set_stacked_shadow_count {#set_stacked_shadow_count}

```gdscript
void set_stacked_shadow_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stacked_shadow_offset {#set_stacked_shadow_offset}

```gdscript
void set_stacked_shadow_offset(index: int, offset: Vector2)
```

Sets the offset of the stacked shadow identified by the given index to offset.

### set_stacked_shadow_outline_size {#set_stacked_shadow_outline_size}

```gdscript
void set_stacked_shadow_outline_size(index: int, size: int)
```

Sets the outline size of the stacked shadow identified by the given index to size.

## 属性

### line_spacing {#line_spacing}

```gdscript
line_spacing: float
```

- void set_line_spacing(value: float)
- float get_line_spacing()

Additional vertical spacing between lines (in pixels), spacing is added to line descent. This value can be negative.

### paragraph_spacing {#paragraph_spacing}

```gdscript
paragraph_spacing: float
```

- void set_paragraph_spacing(value: float)
- float get_paragraph_spacing()

Vertical space between paragraphs. Added on top of line_spacing.

### font {#font}

```gdscript
font: Font
```

- void set_font(value: Font)
- Font get_font()

Font used for the text.

### font_size {#font_size}

```gdscript
font_size: int
```

- void set_font_size(value: int)
- int get_font_size()

Size of the text.

### font_color {#font_color}

```gdscript
font_color: Color
```

- void set_font_color(value: Color)
- Color get_font_color()

Color of the text.

### outline_size {#outline_size}

```gdscript
outline_size: int
```

- void set_outline_size(value: int)
- int get_outline_size()

Text outline size.

### outline_color {#outline_color}

```gdscript
outline_color: Color
```

- void set_outline_color(value: Color)
- Color get_outline_color()

The color of the outline.

### shadow_size {#shadow_size}

```gdscript
shadow_size: int
```

- void set_shadow_size(value: int)
- int get_shadow_size()

Size of the shadow effect.

### shadow_color {#shadow_color}

```gdscript
shadow_color: Color
```

- void set_shadow_color(value: Color)
- Color get_shadow_color()

Color of the shadow effect. If alpha is 0, no shadow will be drawn.

### shadow_offset {#shadow_offset}

```gdscript
shadow_offset: Vector2
```

- void set_shadow_offset(value: Vector2)
- Vector2 get_shadow_offset()

Offset of the shadow effect, in pixels.

### stacked_outline_count {#stacked_outline_count}

```gdscript
stacked_outline_count: int
```

- void set_stacked_outline_count(value: int)
- int get_stacked_outline_count()

The number of stacked outlines.

### stacked_shadow_count {#stacked_shadow_count}

```gdscript
stacked_shadow_count: int
```

- void set_stacked_shadow_count(value: int)
- int get_stacked_shadow_count()

The number of stacked shadows.
