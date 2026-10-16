# AcceptDialog

**继承**：[Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **AcceptDialog**

> The default use of AcceptDialog is to allow it to only be accepted or closed, with the same result. However, the confirmed and canceled signals allow to make the two actions different, and the add_button() method allows to add custom buttons and actions.

<details><summary>官方英文描述（中文精译进行中）</summary>

The default use of AcceptDialog is to allow it to only be accepted or closed, with the same result. However, the confirmed and canceled signals allow to make the two actions different, and the add_button() method allows to add custom buttons and actions.

Note: AcceptDialog is invisible by default. To make it visible, call one of the popup_* methods from Window on the node, such as Window.popup_centered_clamped().

</details>

## 方法

### add_button {#add_button}

```gdscript
Button add_button(text: String, right: bool = false, action: String = "")
```

Adds a button with label text and a custom action to the dialog and returns the created button.

If action is not empty, pressing the button will emit the custom_action signal with the specified action string.

If true, right will place the button to the right of any sibling buttons.

You can use remove_button() method to remove a button created with this method from the dialog.

### add_cancel_button {#add_cancel_button}

```gdscript
Button add_cancel_button(name: String)
```

Adds a button with label name and a cancel action to the dialog and returns the created button.

You can use remove_button() method to remove a button created with this method from the dialog.

### get_close_on_escape {#get_close_on_escape}

```gdscript
bool get_close_on_escape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hide_on_ok {#get_hide_on_ok}

```gdscript
bool get_hide_on_ok() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_label {#get_label}

```gdscript
Label get_label()
```

Returns the label used for built-in text.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_ok_button {#get_ok_button}

```gdscript
Button get_ok_button()
```

Returns the OK Button instance.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_ok_button_text {#get_ok_button_text}

```gdscript
String get_ok_button_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_text {#get_text}

```gdscript
String get_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_autowrap {#has_autowrap}

```gdscript
bool has_autowrap()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### register_text_enter {#register_text_enter}

```gdscript
void register_text_enter(line_edit: LineEdit)
```

Registers a LineEdit in the dialog. When the enter key is pressed, the dialog will be accepted.

### remove_button {#remove_button}

```gdscript
void remove_button(button: Button)
```

Removes the button from the dialog. Does NOT free the button. The button must be a Button added with add_button() or add_cancel_button() method. After removal, pressing the button will no longer emit this dialog's custom_action or canceled signals.

### set_autowrap {#set_autowrap}

```gdscript
void set_autowrap(autowrap: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_close_on_escape {#set_close_on_escape}

```gdscript
void set_close_on_escape(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hide_on_ok {#set_hide_on_ok}

```gdscript
void set_hide_on_ok(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ok_button_text {#set_ok_button_text}

```gdscript
void set_ok_button_text(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_text {#set_text}

```gdscript
void set_text(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### ok_button_text {#ok_button_text}

```gdscript
ok_button_text: String
```

- void set_ok_button_text(value: String)
- String get_ok_button_text()

The text displayed by the OK button (see get_ok_button()). If empty, a default text will be used.

### dialog_text {#dialog_text}

```gdscript
dialog_text: String
```

- void set_text(value: String)
- String get_text()

The text displayed by the dialog.

### dialog_hide_on_ok {#dialog_hide_on_ok}

```gdscript
dialog_hide_on_ok: bool
```

- void set_hide_on_ok(value: bool)
- bool get_hide_on_ok()

If true, the dialog is hidden when the OK button is pressed. You can set it to false if you want to do e.g. input validation when receiving the confirmed signal, and handle hiding the dialog in your own logic.

Note: Some nodes derived from this class can have a different default value, and potentially their own built-in logic overriding this setting. For example FileDialog defaults to false, and has its own input validation code that is called when you press OK, which eventually hides the dialog if the input is valid. As such, this property can't be used in FileDialog to disable hiding the dialog when pressing OK.

### dialog_close_on_escape {#dialog_close_on_escape}

```gdscript
dialog_close_on_escape: bool
```

- void set_close_on_escape(value: bool)
- bool get_close_on_escape()

If true, the dialog will be hidden when the ui_close_dialog action is pressed (by default, this action is bound to Escape, or Cmd + W on macOS).

### dialog_autowrap {#dialog_autowrap}

```gdscript
dialog_autowrap: bool
```

- void set_autowrap(value: bool)
- bool has_autowrap()

Sets autowrapping for the text in the dialog.

## 信号

### confirmed {#confirmed}

```gdscript
signal confirmed()
```

Emitted when the dialog is accepted, i.e. the OK button is pressed.

### canceled {#canceled}

```gdscript
signal canceled()
```

Emitted when the dialog is closed or the button created with add_cancel_button() is pressed.

### custom_action {#custom_action}

```gdscript
signal custom_action(action: StringName)
```

Emitted when a custom button with an action is pressed. See add_button().
