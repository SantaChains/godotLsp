# ScriptEditorBase

**继承**：[VBoxContainer](class_vboxcontainer.md) → [BoxContainer](class_boxcontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **ScriptEditorBase**

> Base editor for editing scripts in the ScriptEditor. This does not include documentation items.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base editor for editing scripts in the ScriptEditor. This does not include documentation items.

</details>

## 方法

### add_syntax_highlighter {#add_syntax_highlighter}

```gdscript
void add_syntax_highlighter(highlighter: EditorSyntaxHighlighter)
```

Adds an EditorSyntaxHighlighter to the open script.

### get_base_editor {#get_base_editor}

```gdscript
Control get_base_editor() const
```

Returns the underlying Control used for editing scripts. For text scripts, this is a CodeEdit.

## 信号

### name_changed {#name_changed}

```gdscript
signal name_changed()
```

Emitted after script validation or when the edited resource has changed.

### edited_script_changed {#edited_script_changed}

```gdscript
signal edited_script_changed()
```

Emitted after script validation.

### search_in_files_requested {#search_in_files_requested}

```gdscript
signal search_in_files_requested(text: String)
```

Emitted when the user request to search text in the file system.

### request_save_history {#request_save_history}

```gdscript
signal request_save_history()
```

Emitted when the user contextual goto and the item is in the same script.

### request_help {#request_help}

```gdscript
signal request_help(topic: String)
```

Emitted when the user requests contextual help.

### request_open_script_at_line {#request_open_script_at_line}

```gdscript
signal request_open_script_at_line(script: Object, line: int)
```

Emitted when the user requests to view a specific line of a script, similar to go_to_method.

### go_to_help {#go_to_help}

```gdscript
signal go_to_help(what: String)
```

Emitted when the user requests a specific documentation page.

### request_save_previous_state {#request_save_previous_state}

```gdscript
signal request_save_previous_state(state: Dictionary)
```

Emitted when the user changes current script or moves caret by 10 or more columns within the same script.

### replace_in_files_requested {#replace_in_files_requested}

```gdscript
signal replace_in_files_requested(text: String)
```

Emitted when the user request to find and replace text in the file system.

### go_to_method {#go_to_method}

```gdscript
signal go_to_method(script: Object, method: String)
```

Emitted when the user requests to view a specific method of a script, similar to request_open_script_at_line.
