# EditorScriptPicker

**继承**：[EditorResourcePicker](class_editorresourcepicker.md) → [HBoxContainer](class_hboxcontainer.md) → [BoxContainer](class_boxcontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **EditorScriptPicker**

> Similar to EditorResourcePicker this Control node is used in the editor's Inspector dock, but only to edit the script property of a Node. Default options for creating new resources of all possible subtypes are replaced with dedicated buttons that open the "Attach Node Script" dialog. Can be used with EditorInspectorPlugin to recreate the same behavior.

<details><summary>官方英文描述（中文精译进行中）</summary>

Similar to EditorResourcePicker this Control node is used in the editor's Inspector dock, but only to edit the script property of a Node. Default options for creating new resources of all possible subtypes are replaced with dedicated buttons that open the "Attach Node Script" dialog. Can be used with EditorInspectorPlugin to recreate the same behavior.

Note: You must set the script_owner for the custom context menu items to work.

</details>

## 方法

### get_script_owner {#get_script_owner}

```gdscript
Node get_script_owner() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_script_owner {#set_script_owner}

```gdscript
void set_script_owner(owner_node: Node)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### script_owner {#script_owner}

```gdscript
script_owner: Node
```

- void set_script_owner(value: Node)
- Node get_script_owner()

The owner Node of the script property that holds the edited resource.
