# ColorPickerButton

**继承**：[Button](class_button.md) → [BaseButton](class_basebutton.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ColorPickerButton**

> Encapsulates a ColorPicker, making it accessible by pressing a button. Pressing the button will toggle the ColorPicker's visibility.

<details><summary>官方英文描述（中文精译进行中）</summary>

Encapsulates a ColorPicker, making it accessible by pressing a button. Pressing the button will toggle the ColorPicker's visibility.

See also BaseButton which contains common properties and methods associated with this node.

Note: By default, the button may not be wide enough for the color preview swatch to be visible. Make sure to set Control.custom_minimum_size to a big enough value to give the button enough space.

</details>

## 方法

### get_pick_color {#get_pick_color}

```gdscript
Color get_pick_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_picker {#get_picker}

```gdscript
ColorPicker get_picker()
```

Returns the ColorPicker that this node toggles.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_popup {#get_popup}

```gdscript
PopupPanel get_popup()
```

Returns the control's PopupPanel which allows you to connect to popup signals. This allows you to handle events when the ColorPicker is shown or hidden.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their Window.visible property.

### is_editing_alpha {#is_editing_alpha}

```gdscript
bool is_editing_alpha() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editing_intensity {#is_editing_intensity}

```gdscript
bool is_editing_intensity() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edit_alpha {#set_edit_alpha}

```gdscript
void set_edit_alpha(show: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edit_intensity {#set_edit_intensity}

```gdscript
void set_edit_intensity(show: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pick_color {#set_pick_color}

```gdscript
void set_pick_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### color {#color}

```gdscript
color: Color
```

- void set_pick_color(value: Color)
- Color get_pick_color()

The currently selected color.

### edit_alpha {#edit_alpha}

```gdscript
edit_alpha: bool
```

- void set_edit_alpha(value: bool)
- bool is_editing_alpha()

If true, the alpha channel in the displayed ColorPicker will be visible.

### edit_intensity {#edit_intensity}

```gdscript
edit_intensity: bool
```

- void set_edit_intensity(value: bool)
- bool is_editing_intensity()

If true, the intensity slider in the displayed ColorPicker will be visible.

## 信号

### color_changed {#color_changed}

```gdscript
signal color_changed(color: Color)
```

Emitted when the color changes.

### popup_closed {#popup_closed}

```gdscript
signal popup_closed()
```

Emitted when the ColorPicker is closed.

### picker_created {#picker_created}

```gdscript
signal picker_created()
```

Emitted when the ColorPicker is created (the button is pressed for the first time).
