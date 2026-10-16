# SpinBox

**继承**：[Range](class_range.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **SpinBox**

> SpinBox is a numerical input text field. It allows entering integers and floating-point numbers. The SpinBox also has up and down buttons that can be clicked to increase or decrease the value. The value can also be changed by dragging the mouse up or down over the SpinBox's arrows.

<details><summary>官方英文描述（中文精译进行中）</summary>

SpinBox is a numerical input text field. It allows entering integers and floating-point numbers. The SpinBox also has up and down buttons that can be clicked to increase or decrease the value. The value can also be changed by dragging the mouse up or down over the SpinBox's arrows.

Additionally, mathematical expressions can be entered. These are evaluated when the user presses Enter while editing the SpinBox's text field. This uses the Expression class to parse and evaluate the expression. The result of the expression is then set as the value of the SpinBox. Some examples of valid expressions are 5 + 2 * 3, pow(2, 4), and PI + sin(0.5). Expressions are case-sensitive.

Example: Create a SpinBox, disable its context menu and set its text alignment to right.

```
var spin_box = SpinBox.new()
add_child(spin_box)
var line_edit = spin_box.get_line_edit()
line_edit.context_menu_enabled = false
spin_box.horizontal_alignment = LineEdit.HORIZONTAL_ALIGNMENT_RIGHT
```

```
var spinBox = new SpinBox();
AddChild(spinBox);
var lineEdit = spinBox.GetLineEdit();
lineEdit.ContextMenuEnabled = false;
spinBox.AlignHorizontal = LineEdit.HorizontalAlignEnum.Right;
```

See Range class for more options over the SpinBox.

Note: With the SpinBox's context menu disabled, you can right-click the bottom half of the spinbox to set the value to its minimum, while right-clicking the top half sets the value to its maximum.

Note: SpinBox relies on an underlying LineEdit node. To theme a SpinBox's background, add theme items for LineEdit and customize them. The LineEdit has the SpinBoxInnerLineEdit theme variation, so that you can give it a distinct appearance from regular LineEdits.

Note: If you want to implement drag and drop for the underlying LineEdit, you can use Control.set_drag_forwarding() on the node returned by get_line_edit().

</details>

## 方法

### apply {#apply}

```gdscript
void apply()
```

Applies the current value of this SpinBox. This is equivalent to pressing Enter while editing the LineEdit used by the SpinBox. This will cause LineEdit.text_submitted to be emitted and its currently contained expression to be evaluated.

### get_custom_arrow_step {#get_custom_arrow_step}

```gdscript
float get_custom_arrow_step() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_horizontal_alignment {#get_horizontal_alignment}

```gdscript
HorizontalAlignment get_horizontal_alignment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_edit {#get_line_edit}

```gdscript
LineEdit get_line_edit()
```

Returns the LineEdit instance from this SpinBox. You can use it to access properties and methods of LineEdit.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_prefix {#get_prefix}

```gdscript
String get_prefix() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_suffix {#get_suffix}

```gdscript
String get_suffix() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_update_on_text_changed {#get_update_on_text_changed}

```gdscript
bool get_update_on_text_changed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_custom_arrow_rounding {#is_custom_arrow_rounding}

```gdscript
bool is_custom_arrow_rounding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editable {#is_editable}

```gdscript
bool is_editable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_select_all_on_focus {#is_select_all_on_focus}

```gdscript
bool is_select_all_on_focus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_arrow_round {#set_custom_arrow_round}

```gdscript
void set_custom_arrow_round(round: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_arrow_step {#set_custom_arrow_step}

```gdscript
void set_custom_arrow_step(arrow_step: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_editable {#set_editable}

```gdscript
void set_editable(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_horizontal_alignment {#set_horizontal_alignment}

```gdscript
void set_horizontal_alignment(alignment: HorizontalAlignment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_prefix {#set_prefix}

```gdscript
void set_prefix(prefix: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_select_all_on_focus {#set_select_all_on_focus}

```gdscript
void set_select_all_on_focus(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_suffix {#set_suffix}

```gdscript
void set_suffix(suffix: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_update_on_text_changed {#set_update_on_text_changed}

```gdscript
void set_update_on_text_changed(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### alignment {#alignment}

```gdscript
alignment: int
```

- void set_horizontal_alignment(value: HorizontalAlignment)
- HorizontalAlignment get_horizontal_alignment()

Changes the alignment of the underlying LineEdit.

### editable {#editable}

```gdscript
editable: bool
```

- void set_editable(value: bool)
- bool is_editable()

If true, the SpinBox will be editable. Otherwise, it will be read only.

### update_on_text_changed {#update_on_text_changed}

```gdscript
update_on_text_changed: bool
```

- void set_update_on_text_changed(value: bool)
- bool get_update_on_text_changed()

Sets the value of the Range for this SpinBox when the LineEdit text is changed instead of submitted. See LineEdit.text_changed and LineEdit.text_submitted.

Note: If set to true, this will interfere with entering mathematical expressions in the SpinBox. The SpinBox will try to evaluate the expression as you type, which means symbols like a trailing + are removed immediately by the expression being evaluated.

### prefix {#prefix}

```gdscript
prefix: String
```

- void set_prefix(value: String)
- String get_prefix()

Adds the specified prefix string before the numerical value of the SpinBox.

### suffix {#suffix}

```gdscript
suffix: String
```

- void set_suffix(value: String)
- String get_suffix()

Adds the specified suffix string after the numerical value of the SpinBox.

### custom_arrow_step {#custom_arrow_step}

```gdscript
custom_arrow_step: float
```

- void set_custom_arrow_step(value: float)
- float get_custom_arrow_step()

If not 0, sets the step when interacting with the arrow buttons of the SpinBox.

Note: Range.value will still be rounded to a multiple of Range.step.

### custom_arrow_round {#custom_arrow_round}

```gdscript
custom_arrow_round: bool
```

- void set_custom_arrow_round(value: bool)
- bool is_custom_arrow_rounding()

If true, the value will be rounded to a multiple of custom_arrow_step when interacting with the arrow buttons. Otherwise, increments the value by custom_arrow_step and then rounds it according to Range.step.

### select_all_on_focus {#select_all_on_focus}

```gdscript
select_all_on_focus: bool
```

- void set_select_all_on_focus(value: bool)
- bool is_select_all_on_focus()

If true, the SpinBox will select the whole text when the LineEdit gains focus. Clicking the up and down arrows won't trigger this behavior.
