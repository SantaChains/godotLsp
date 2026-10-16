# OpenXRBindingModifierEditor

**继承**：[PanelContainer](class_panelcontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **OpenXRBindingModifierEditor**

> This is the default binding modifier editor used in the OpenXR action map.

<details><summary>官方英文描述</summary>

This is the default binding modifier editor used in the OpenXR action map.

</details>

## 方法

### get_binding_modifier {#get_binding_modifier}

```gdscript
OpenXRBindingModifier get_binding_modifier() const
```

Returns the OpenXRBindingModifier currently being edited.

### setup {#setup}

```gdscript
void setup(action_map: OpenXRActionMap, binding_modifier: OpenXRBindingModifier)
```

Setup this editor for the provided action_map and binding_modifier.

## 信号

### binding_modifier_removed {#binding_modifier_removed}

```gdscript
signal binding_modifier_removed(binding_modifier_editor: Object)
```

Signal emitted when the user presses the delete binding modifier button for this modifier.
