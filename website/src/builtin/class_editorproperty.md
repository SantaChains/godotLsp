# EditorProperty

**继承**：[Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorProperty**

> A custom control for editing properties that can be added to the EditorInspector. It is added via EditorInspectorPlugin.

<details><summary>官方英文描述</summary>

A custom control for editing properties that can be added to the EditorInspector. It is added via EditorInspectorPlugin.

</details>

## 方法

### _set_read_only {#_set_read_only}

```gdscript
void _set_read_only(read_only: bool) virtual
```

Called when the read-only status of the property is changed. It may be used to change custom controls into a read-only or modifiable state.

### _update_property {#_update_property}

```gdscript
void _update_property() virtual
```

When this virtual function is called, you must update your editor.

### add_focusable {#add_focusable}

```gdscript
void add_focusable(control: Control)
```

If any of the controls added can gain keyboard focus, add it here. This ensures that focus will be restored if the inspector is refreshed.

### deselect {#deselect}

```gdscript
void deselect()
```

Draw property as not selected. Used by the inspector.

### emit_changed {#emit_changed}

```gdscript
void emit_changed(property: StringName, value: Variant, field: StringName = &"", changing: bool = false)
```

If one or several properties have changed, this must be called. field is used in case your editor can modify fields separately (as an example, Vector3.x). The changing argument avoids the editor requesting this property to be refreshed (leave as false if unsure).

### get_edited_object {#get_edited_object}

```gdscript
Object get_edited_object()
```

Returns the edited object.

Note: This method could return null if the editor has not yet been associated with a property. However, in _update_property() and _set_read_only(), this value is guaranteed to be non-null.

### get_edited_property {#get_edited_property}

```gdscript
StringName get_edited_property() const
```

Returns the edited property. If your editor is for a single property (added via EditorInspectorPlugin._parse_property()), then this will return the property.

Note: This method could return null if the editor has not yet been associated with a property. However, in _update_property() and _set_read_only(), this value is guaranteed to be non-null.

### get_label {#get_label}

```gdscript
String get_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_name_split_ratio {#get_name_split_ratio}

```gdscript
float get_name_split_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_checkable {#is_checkable}

```gdscript
bool is_checkable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_checked {#is_checked}

```gdscript
bool is_checked() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_deletable {#is_deletable}

```gdscript
bool is_deletable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draw_background {#is_draw_background}

```gdscript
bool is_draw_background() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draw_label {#is_draw_label}

```gdscript
bool is_draw_label() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_draw_warning {#is_draw_warning}

```gdscript
bool is_draw_warning() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_keying {#is_keying}

```gdscript
bool is_keying() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_read_only {#is_read_only}

```gdscript
bool is_read_only() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_selectable {#is_selectable}

```gdscript
bool is_selectable() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_selected {#is_selected}

```gdscript
bool is_selected() const
```

Returns true if property is drawn as selected. Used by the inspector.

### is_using_folding {#is_using_folding}

```gdscript
bool is_using_folding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### select {#select}

```gdscript
void select(focusable: int = -1)
```

Draw property as selected. Used by the inspector.

### set_bottom_editor {#set_bottom_editor}

```gdscript
void set_bottom_editor(editor: Control)
```

Puts the editor control below the property label. The control must be previously added using Node.add_child().

### set_checkable {#set_checkable}

```gdscript
void set_checkable(checkable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_checked {#set_checked}

```gdscript
void set_checked(checked: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_deletable {#set_deletable}

```gdscript
void set_deletable(deletable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_background {#set_draw_background}

```gdscript
void set_draw_background(draw_background: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_label {#set_draw_label}

```gdscript
void set_draw_label(draw_label: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_draw_warning {#set_draw_warning}

```gdscript
void set_draw_warning(draw_warning: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keying {#set_keying}

```gdscript
void set_keying(keying: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_label {#set_label}

```gdscript
void set_label(text: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_label_reference {#set_label_reference}

```gdscript
void set_label_reference(control: Control)
```

Used by the inspector, set to a control that will be used as a reference to calculate the size of the label.

### set_name_split_ratio {#set_name_split_ratio}

```gdscript
void set_name_split_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_object_and_property {#set_object_and_property}

```gdscript
void set_object_and_property(object: Object, property: StringName)
```

Assigns object and property to edit.

### set_read_only {#set_read_only}

```gdscript
void set_read_only(read_only: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_selectable {#set_selectable}

```gdscript
void set_selectable(selectable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_folding {#set_use_folding}

```gdscript
void set_use_folding(use_folding: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### update_property {#update_property}

```gdscript
void update_property()
```

Forces a refresh of the property display.

## 属性

### label {#label}

```gdscript
label: String
```

- void set_label(value: String)
- String get_label()

Set this property to change the label (if you want to show one).

### read_only {#read_only}

```gdscript
read_only: bool
```

- void set_read_only(value: bool)
- bool is_read_only()

Used by the inspector, set to true when the property is read-only.

### draw_label {#draw_label}

```gdscript
draw_label: bool
```

- void set_draw_label(value: bool)
- bool is_draw_label()

Used by the inspector, set to true when the property label is drawn.

### draw_background {#draw_background}

```gdscript
draw_background: bool
```

- void set_draw_background(value: bool)
- bool is_draw_background()

Used by the inspector, set to true when the property background is drawn.

### checkable {#checkable}

```gdscript
checkable: bool
```

- void set_checkable(value: bool)
- bool is_checkable()

Used by the inspector, set to true when the property is checkable.

### checked {#checked}

```gdscript
checked: bool
```

- void set_checked(value: bool)
- bool is_checked()

Used by the inspector, set to true when the property is checked.

### draw_warning {#draw_warning}

```gdscript
draw_warning: bool
```

- void set_draw_warning(value: bool)
- bool is_draw_warning()

Used by the inspector, set to true when the property is drawn with the editor theme's warning color. This is used for editable children's properties.

### keying {#keying}

```gdscript
keying: bool
```

- void set_keying(value: bool)
- bool is_keying()

Used by the inspector, set to true when the property can add keys for animation.

### deletable {#deletable}

```gdscript
deletable: bool
```

- void set_deletable(value: bool)
- bool is_deletable()

Used by the inspector, set to true when the property can be deleted by the user.

### selectable {#selectable}

```gdscript
selectable: bool
```

- void set_selectable(value: bool)
- bool is_selectable()

Used by the inspector, set to true when the property is selectable.

### use_folding {#use_folding}

```gdscript
use_folding: bool
```

- void set_use_folding(value: bool)
- bool is_using_folding()

Used by the inspector, set to true when the property is using folding.

### name_split_ratio {#name_split_ratio}

```gdscript
name_split_ratio: float
```

- void set_name_split_ratio(value: float)
- float get_name_split_ratio()

Space distribution ratio between the label and the editing field.

## 信号

### property_changed {#property_changed}

```gdscript
signal property_changed(property: StringName, value: Variant, field: StringName, changing: bool)
```

Do not emit this manually, use the emit_changed() method instead.

### multiple_properties_changed {#multiple_properties_changed}

```gdscript
signal multiple_properties_changed(properties: PackedStringArray, value: Array)
```

Emit it if you want multiple properties modified at the same time. Do not use if added via EditorInspectorPlugin._parse_property().

### property_keyed {#property_keyed}

```gdscript
signal property_keyed(property: StringName)
```

Emit it if you want to add this value as an animation key (check for keying being enabled first).

### property_deleted {#property_deleted}

```gdscript
signal property_deleted(property: StringName)
```

Emitted when a property was deleted. Used internally.

### property_keyed_with_value {#property_keyed_with_value}

```gdscript
signal property_keyed_with_value(property: StringName, value: Variant)
```

Emit it if you want to key a property with a single value.

### property_checked {#property_checked}

```gdscript
signal property_checked(property: StringName, checked: bool)
```

Emitted when a property was checked. Used internally.

### property_overridden {#property_overridden}

```gdscript
signal property_overridden()
```

Emitted when a setting override for the current project is requested.

### property_favorited {#property_favorited}

```gdscript
signal property_favorited(property: StringName, favorited: bool)
```

Emit it if you want to mark a property as favorited, making it appear at the top of the inspector.

### property_pinned {#property_pinned}

```gdscript
signal property_pinned(property: StringName, pinned: bool)
```

Emit it if you want to mark (or unmark) the value of a property for being saved regardless of being equal to the default value.

The default value is the one the property will get when the node is just instantiated and can come from an ancestor scene in the inheritance/instantiation chain, a script or a builtin class.

### property_can_revert_changed {#property_can_revert_changed}

```gdscript
signal property_can_revert_changed(property: StringName, can_revert: bool)
```

Emitted when the revertability (i.e., whether it has a non-default value and thus is displayed with a revert icon) of a property has changed.

### resource_selected {#resource_selected}

```gdscript
signal resource_selected(path: String, resource: Resource)
```

If you want a sub-resource to be edited, emit this signal with the resource.

### object_id_selected {#object_id_selected}

```gdscript
signal object_id_selected(property: StringName, id: int)
```

Used by sub-inspectors. Emit it if what was selected was an Object ID.

### selected {#selected}

```gdscript
signal selected(path: String, focusable_idx: int)
```

Emitted when selected. Used internally.
