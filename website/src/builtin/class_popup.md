# Popup

**继承**：[Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **Popup**

> Popup is a base class for contextual windows and panels with fixed position. It's a modal by default (see Window.popup_window) and provides methods for implementing custom popup behavior.

<details><summary>官方英文描述</summary>

Popup is a base class for contextual windows and panels with fixed position. It's a modal by default (see Window.popup_window) and provides methods for implementing custom popup behavior.

Note: Popup is invisible by default. To make it visible, call one of the popup_* methods from Window on the node, such as Window.popup_centered_clamped().

</details>

## 信号

### popup_hide {#popup_hide}

```gdscript
signal popup_hide()
```

Emitted when the popup is hidden.
