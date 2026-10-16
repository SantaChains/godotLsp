# EditorResourcePicker

**继承**：[HBoxContainer](class_hboxcontainer.md) → [BoxContainer](class_boxcontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorResourcePicker**

> This Control node is used in the editor's Inspector dock to allow editing of Resource type properties. It provides options for creating, loading, saving and converting resources. Can be used with EditorInspectorPlugin to recreate the same behavior.

<details><summary>官方英文描述（中文精译进行中）</summary>

This Control node is used in the editor's Inspector dock to allow editing of Resource type properties. It provides options for creating, loading, saving and converting resources. Can be used with EditorInspectorPlugin to recreate the same behavior.

Note: This Control does not include any editor for the resource, as editing is controlled by the Inspector dock itself or sub-Inspectors.

</details>

## 方法

### _handle_menu_selected {#_handle_menu_selected}

```gdscript
bool _handle_menu_selected(id: int) virtual
```

This virtual method can be implemented to handle context menu items not handled by default. See _set_create_options().

### _set_create_options {#_set_create_options}

```gdscript
void _set_create_options(menu_node: Object) virtual
```

This virtual method is called when updating the context menu of an editable EditorResourcePicker. Implement this method to override the "New" items section with your own options. menu_node is a reference to the PopupMenu node.

Note: Implement _handle_menu_selected() to handle these custom items.

Note: Relevant built-in options ("Load", "Copy", "Paste", etc.) are automatically added to the menu_node afterwards, using their hard-coded IDs starting from 0. Custom options need to use non-colliding IDs to be handled properly. Using id = 100 + custom_option_index is safe (this is what the default items in the "New" section use).

### get_allowed_types {#get_allowed_types}

```gdscript
PackedStringArray get_allowed_types() const
```

Returns a list of all allowed types and subtypes corresponding to the base_type. If the base_type is empty, an empty list is returned.

### get_base_type {#get_base_type}

```gdscript
String get_base_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_edited_resource {#get_edited_resource}

```gdscript
Resource get_edited_resource()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_editable {#is_editable}

```gdscript
bool is_editable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_toggle_mode {#is_toggle_mode}

```gdscript
bool is_toggle_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_base_type {#set_base_type}

```gdscript
void set_base_type(base_type: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_editable {#set_editable}

```gdscript
void set_editable(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_edited_resource {#set_edited_resource}

```gdscript
void set_edited_resource(resource: Resource)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_toggle_mode {#set_toggle_mode}

```gdscript
void set_toggle_mode(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_toggle_pressed {#set_toggle_pressed}

```gdscript
void set_toggle_pressed(pressed: bool)
```

Sets the toggle mode state for the main button. Works only if toggle_mode is set to true.

## 属性

### base_type {#base_type}

```gdscript
base_type: String
```

- void set_base_type(value: String)
- String get_base_type()

The base type of allowed resource types. Can be a comma-separated list of several options.

### edited_resource {#edited_resource}

```gdscript
edited_resource: Resource
```

- void set_edited_resource(value: Resource)
- Resource get_edited_resource()

The edited resource value.

### editable {#editable}

```gdscript
editable: bool
```

- void set_editable(value: bool)
- bool is_editable()

If true, the value can be selected and edited.

### toggle_mode {#toggle_mode}

```gdscript
toggle_mode: bool
```

- void set_toggle_mode(value: bool)
- bool is_toggle_mode()

If true, the main button with the resource preview works in the toggle mode. Use set_toggle_pressed() to manually set the state.

## 信号

### resource_selected {#resource_selected}

```gdscript
signal resource_selected(resource: Resource, inspect: bool)
```

Emitted when the resource value was set and user clicked to edit it. When inspect is true, the signal was caused by the context menu "Edit" or "Inspect" option.

### resource_changed {#resource_changed}

```gdscript
signal resource_changed(resource: Resource)
```

Emitted when the value of the edited resource was changed.
