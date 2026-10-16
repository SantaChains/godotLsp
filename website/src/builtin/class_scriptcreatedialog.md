# ScriptCreateDialog

**继承**：[ConfirmationDialog](class_confirmationdialog.md) → [AcceptDialog](class_acceptdialog.md) → [Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **ScriptCreateDialog**

> The ScriptCreateDialog creates script files according to a given template for a given scripting language. The standard use is to configure its fields prior to calling one of the Window.popup() methods.

<details><summary>官方英文描述（中文精译进行中）</summary>

The ScriptCreateDialog creates script files according to a given template for a given scripting language. The standard use is to configure its fields prior to calling one of the Window.popup() methods.

```
func _ready():
    var dialog = ScriptCreateDialog.new();
    dialog.config("Node", "res://new_node.gd") # For in-engine types.
    dialog.config("\"res://base_node.gd\"", "res://derived_node.gd") # For script types.
    dialog.popup_centered()
```

```
public override void _Ready()
{
    var dialog = new ScriptCreateDialog();
    dialog.Config("Node", "res://NewNode.cs"); // For in-engine types.
    dialog.Config("\"res://BaseNode.cs\"", "res://DerivedNode.cs"); // For script types.
    dialog.PopupCentered();
}
```

</details>

## 方法

### config {#config}

```gdscript
void config(inherits: String, path: String, built_in_enabled: bool = true, load_enabled: bool = true)
```

Prefills required fields to configure the ScriptCreateDialog for use.

## 信号

### script_created {#script_created}

```gdscript
signal script_created(script: Script)
```

Emitted when the user clicks the OK button.
