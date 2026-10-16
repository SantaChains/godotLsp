# EditorFileDialog

**继承**：[FileDialog](class_filedialog.md) → [ConfirmationDialog](class_confirmationdialog.md) → [AcceptDialog](class_acceptdialog.md) → [Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorFileDialog**

> EditorFileDialog is a FileDialog tweaked to work in the editor. It automatically handles favorite and recent directory lists, and synchronizes some properties with their corresponding editor settings.

<details><summary>官方英文描述（中文精译进行中）</summary>

EditorFileDialog is a FileDialog tweaked to work in the editor. It automatically handles favorite and recent directory lists, and synchronizes some properties with their corresponding editor settings.

EditorFileDialog will automatically show a native dialog based on the EditorSettings.interface/editor/appearance/use_native_file_dialogs editor setting and ignores FileDialog.use_native_dialog.

Note: EditorFileDialog is invisible by default. To make it visible, call one of the popup_* methods from Window on the node, such as Window.popup_centered_clamped().

Note: On Linux and macOS, sandboxed apps always use native dialogs to access the host file system.

</details>

## 方法

### add_side_menu {#add_side_menu}

```gdscript
void add_side_menu(menu: Control, title: String = "")
```

Deprecated: This feature is no longer supported.

This method is kept for compatibility and does nothing. As an alternative, you can display another dialog after showing the file dialog.

### is_overwrite_warning_disabled {#is_overwrite_warning_disabled}

```gdscript
bool is_overwrite_warning_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_overwrite_warning {#set_disable_overwrite_warning}

```gdscript
void set_disable_overwrite_warning(disable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### disable_overwrite_warning {#disable_overwrite_warning}

```gdscript
disable_overwrite_warning: bool
```

- void set_disable_overwrite_warning(value: bool)
- bool is_overwrite_warning_disabled()

Deprecated: Use FileDialog.overwrite_warning_enabled instead.

If true, the EditorFileDialog will not warn the user before overwriting files.
