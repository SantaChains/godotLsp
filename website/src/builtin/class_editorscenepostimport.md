# EditorScenePostImport

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorScenePostImport**

> Imported scenes can be automatically modified right after import by setting their Custom Script Import property to a tool script that inherits from this class.

<details><summary>官方英文描述（中文精译进行中）</summary>

Imported scenes can be automatically modified right after import by setting their Custom Script Import property to a tool script that inherits from this class.

The _post_import() callback receives the imported scene's root node and returns the modified version of the scene:

```
@tool # Needed so it runs in editor.
extends EditorScenePostImport
# This sample changes all node names.
# Called right after the scene is imported and gets the root node.
func _post_import(scene):
    # Change all node names to "modified_[oldnodename]"
    iterate(scene)
    return scene # Remember to return the imported scene
func iterate(node):
    if node != null:
        node.name = "modified_" + node.name
        for child in node.get_children():
            iterate(child)
```

```
using Godot;
// This sample changes all node names.
// Called right after the scene is imported and gets the root node.
[Tool]
public partial class NodeRenamer : EditorScenePostImport
{
    public override GodotObject _PostImport(Node scene)
    {
        // Change all node names to "modified_[oldnodename]"
        Iterate(scene);
        return scene; // Remember to return the imported scene
    }
    public void Iterate(Node node)
    {
        if (node != null)
        {
            node.Name = $"modified_{node.Name}";
            foreach (Node child in node.GetChildren())
            {
                Iterate(child);
            }
        }
    }
}
```

</details>

## 方法

### _post_import {#_post_import}

```gdscript
Object _post_import(scene: Node) virtual
```

Called after the scene was imported. This method must return the modified version of the scene.

### get_source_file {#get_source_file}

```gdscript
String get_source_file() const
```

Returns the source file path which got imported (e.g. res://scene.dae).
