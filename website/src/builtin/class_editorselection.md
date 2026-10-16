# EditorSelection

**继承**：[Object](class_object.md) → **EditorSelection**

> This object manages the SceneTree selection in the editor.

<details><summary>官方英文描述</summary>

This object manages the SceneTree selection in the editor.

Note: This class shouldn't be instantiated directly. Instead, access the singleton using EditorInterface.get_selection().

</details>

## 方法

### add_node {#add_node}

```gdscript
void add_node(node: Node)
```

Adds a node to the selection.

Note: The newly selected node will not be automatically edited in the inspector. If you want to edit a node, use EditorInterface.edit_node().

### clear {#clear}

```gdscript
void clear()
```

Clear the selection.

### get_selected_nodes {#get_selected_nodes}

```gdscript
typedarray::Node get_selected_nodes()
```

Returns the list of selected nodes.

### get_top_selected_nodes {#get_top_selected_nodes}

```gdscript
typedarray::Node get_top_selected_nodes()
```

Returns the list of top selected nodes only, excluding any children. This is useful for performing transform operations (moving them, rotating, etc.).

For example, if there is a node A with a child B and a sibling C, then selecting all three will cause this method to return only A and C. Changing the global transform of A will affect the global transform of B, so there is no need to change B separately.

### get_transformable_selected_nodes {#get_transformable_selected_nodes}

```gdscript
typedarray::Node get_transformable_selected_nodes()
```

Deprecated: Use get_top_selected_nodes() instead.

Returns the list of top selected nodes only, excluding any children. This is useful for performing transform operations (moving them, rotating, etc.). See get_top_selected_nodes().

### remove_node {#remove_node}

```gdscript
void remove_node(node: Node)
```

Removes a node from the selection.

## 信号

### selection_changed {#selection_changed}

```gdscript
signal selection_changed()
```

Emitted when the selection changes.
