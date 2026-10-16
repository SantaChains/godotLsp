# Button

**继承**：[BaseButton](class_basebutton.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Button**

> 按钮控件。支持点击、长按、自定义样式。

<details><summary>官方英文描述</summary>

Button is the standard themed button. It can contain text and an icon, and it will display them according to the current Theme.

Example: Create a button and connect a method that will be called when the button is pressed:

```
func _ready():
    var button = Button.new()
    button.text = "Click me"
    button.pressed.connect(_button_pressed)
    add_child(button)
func _button_pressed():
    print("Hello world!")
```

```
public override void _Ready()
{
    var button = new Button();
    button.Text = "Click me";
    button.Pressed += ButtonPressed;
    AddChild(button);
}
private void ButtonPressed()
{
    GD.Print("Hello world!");
}
```

See also BaseButton which contains common properties and methods associated with this node.

Note: Buttons support multitouch via touch input, allowing multiple buttons to be pressed at the same time. Otherwise, mouse input is used, limiting interaction to one button press at a time.

</details>

## 方法

### get_autowrap_mode {#get_autowrap_mode}

```gdscript
TextServer.AutowrapMode get_autowrap_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_autowrap_trim_flags {#get_autowrap_trim_flags}

```gdscript
bitfield::TextServer.LineBreakFlag get_autowrap_trim_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_button_icon {#get_button_icon}

```gdscript
Texture2D get_button_icon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_clip_text {#get_clip_text}

```gdscript
bool get_clip_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_icon_alignment {#get_icon_alignment}

```gdscript
HorizontalAlignment get_icon_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_language {#get_language}

```gdscript
String get_language() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text {#get_text}

```gdscript
String get_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_alignment {#get_text_alignment}

```gdscript
HorizontalAlignment get_text_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_direction {#get_text_direction}

```gdscript
Control.TextDirection get_text_direction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text_overrun_behavior {#get_text_overrun_behavior}

```gdscript
TextServer.OverrunBehavior get_text_overrun_behavior() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertical_icon_alignment {#get_vertical_icon_alignment}

```gdscript
VerticalAlignment get_vertical_icon_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_expand_icon {#is_expand_icon}

```gdscript
bool is_expand_icon() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_flat {#is_flat}

```gdscript
bool is_flat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autowrap_mode {#set_autowrap_mode}

```gdscript
void set_autowrap_mode(autowrap_mode: TextServer.AutowrapMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_autowrap_trim_flags {#set_autowrap_trim_flags}

```gdscript
void set_autowrap_trim_flags(autowrap_trim_flags: bitfield::TextServer.LineBreakFlag)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_button_icon {#set_button_icon}

```gdscript
void set_button_icon(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_clip_text {#set_clip_text}

```gdscript
void set_clip_text(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_expand_icon {#set_expand_icon}

```gdscript
void set_expand_icon(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flat {#set_flat}

```gdscript
void set_flat(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_icon_alignment {#set_icon_alignment}

```gdscript
void set_icon_alignment(icon_alignment: HorizontalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_language {#set_language}

```gdscript
void set_language(language: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text {#set_text}

```gdscript
void set_text(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_alignment {#set_text_alignment}

```gdscript
void set_text_alignment(alignment: HorizontalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_direction {#set_text_direction}

```gdscript
void set_text_direction(direction: Control.TextDirection)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text_overrun_behavior {#set_text_overrun_behavior}

```gdscript
void set_text_overrun_behavior(overrun_behavior: TextServer.OverrunBehavior)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical_icon_alignment {#set_vertical_icon_alignment}

```gdscript
void set_vertical_icon_alignment(vertical_icon_alignment: VerticalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### text {#text}

```gdscript
text: String
```

- void set_text(value: String)
- String get_text()

The button's text that will be displayed inside the button's area.

### icon {#icon}

```gdscript
icon: Texture2D
```

- void set_button_icon(value: Texture2D)
- Texture2D get_button_icon()

Button's icon, if text is present the icon will be placed before the text.

To edit margin and spacing of the icon, use h_separation theme property and content_margin_* properties of the used StyleBoxes.

### flat {#flat}

```gdscript
flat: bool
```

- void set_flat(value: bool)
- bool is_flat()

Flat buttons don't display decoration.

### alignment {#alignment}

```gdscript
alignment: int
```

- void set_text_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_text_alignment()

Text alignment policy for the button's text.

### text_overrun_behavior {#text_overrun_behavior}

```gdscript
text_overrun_behavior: int
```

- void set_text_overrun_behavior(value: OverrunBehavior)
- OverrunBehavior get_text_overrun_behavior()

Sets the clipping behavior when the text exceeds the node's bounding rectangle.

### autowrap_mode {#autowrap_mode}

```gdscript
autowrap_mode: int
```

- void set_autowrap_mode(value: AutowrapMode)
- AutowrapMode get_autowrap_mode()

If set to something other than TextServer.AUTOWRAP_OFF, the text gets wrapped inside the node's bounding rectangle.

### autowrap_trim_flags {#autowrap_trim_flags}

```gdscript
autowrap_trim_flags: int
```

- void set_autowrap_trim_flags(value: BitField[LineBreakFlag])
- BitField[LineBreakFlag] get_autowrap_trim_flags()

Autowrap space trimming flags. See TextServer.BREAK_TRIM_START_EDGE_SPACES and TextServer.BREAK_TRIM_END_EDGE_SPACES for more info.

### clip_text {#clip_text}

```gdscript
clip_text: bool
```

- void set_clip_text(value: bool)
- bool get_clip_text()

If true, text that is too large to fit the button is clipped horizontally. If false, the button will always be wide enough to hold the text. The text is not vertically clipped, and the button's height is not affected by this property.

### icon_alignment {#icon_alignment}

```gdscript
icon_alignment: int
```

- void set_icon_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_icon_alignment()

Specifies if the icon should be aligned horizontally to the left, right, or center of a button. Uses the same HorizontalAlignment constants as the text alignment. If centered horizontally and vertically, text will draw on top of the icon.

### vertical_icon_alignment {#vertical_icon_alignment}

```gdscript
vertical_icon_alignment: int
```

- void set_vertical_icon_alignment(value: VerticalAlignment)
- VerticalAlignment get_vertical_icon_alignment()

Specifies if the icon should be aligned vertically to the top, bottom, or center of a button. Uses the same VerticalAlignment constants as the text alignment. If centered horizontally and vertically, text will draw on top of the icon.

### expand_icon {#expand_icon}

```gdscript
expand_icon: bool
```

- void set_expand_icon(value: bool)
- bool is_expand_icon()

When enabled, the button's icon will expand/shrink to fit the button's size while keeping its aspect. See also icon_max_width.

### text_direction {#text_direction}

```gdscript
text_direction: int
```

- void set_text_direction(value: TextDirection)
- TextDirection get_text_direction()

Base text writing direction.

### language {#language}

```gdscript
language: String
```

- void set_language(value: String)
- String get_language()

Language code used for line-breaking and text shaping algorithms. If left empty, the current locale is used instead.
