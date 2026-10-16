# ConfirmationDialog

**继承**：[AcceptDialog](class_acceptdialog.md) → [Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **ConfirmationDialog**

> A dialog used for confirmation of actions. This window is similar to AcceptDialog, but pressing its Cancel button can have a different outcome from pressing the OK button. The order of the two buttons varies depending on the host OS.

<details><summary>官方英文描述（中文精译进行中）</summary>

A dialog used for confirmation of actions. This window is similar to AcceptDialog, but pressing its Cancel button can have a different outcome from pressing the OK button. The order of the two buttons varies depending on the host OS.

To get cancel action, you can use:

```
get_cancel_button().pressed.connect(_on_canceled)
```

```
GetCancelButton().Pressed += OnCanceled;
```

Note: AcceptDialog is invisible by default. To make it visible, call one of the popup_* methods from Window on the node, such as Window.popup_centered_clamped().

</details>

## 方法

### get_cancel_button {#get_cancel_button}

```gdscript
Button get_cancel_button()
```

Returns the cancel button.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_cancel_button_text {#get_cancel_button_text}

```gdscript
String get_cancel_button_text() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cancel_button_text {#set_cancel_button_text}

```gdscript
void set_cancel_button_text(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### cancel_button_text {#cancel_button_text}

```gdscript
cancel_button_text: String
```

- void set_cancel_button_text(value: String)
- String get_cancel_button_text()

The text displayed by the cancel button (see get_cancel_button()).
