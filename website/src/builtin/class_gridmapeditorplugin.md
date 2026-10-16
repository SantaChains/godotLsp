# GridMapEditorPlugin

**继承**：[EditorPlugin](class_editorplugin.md) → [Node](class_node.md) → [Object](class_object.md) → **GridMapEditorPlugin**

> GridMapEditorPlugin provides access to the GridMap editor functionality.

<details><summary>官方英文描述</summary>

GridMapEditorPlugin provides access to the GridMap editor functionality.

</details>

## 方法

### clear_selection {#clear_selection}

```gdscript
void clear_selection()
```

Deselects any currently selected cells.

### get_current_grid_map {#get_current_grid_map}

```gdscript
GridMap get_current_grid_map() const
```

Returns the GridMap node currently edited by the grid map editor.

### get_selected_cells {#get_selected_cells}

```gdscript
Array get_selected_cells() const
```

Returns an array of Vector3is with the selected cells' coordinates.

### get_selected_palette_item {#get_selected_palette_item}

```gdscript
int get_selected_palette_item() const
```

Returns the index of the selected MeshLibrary item in the grid map editor's palette or -1 if no item is selected.

Note: The indices might not be in the same order as they appear in the editor's interface.

### get_selection {#get_selection}

```gdscript
AABB get_selection() const
```

Returns the cell coordinate bounds of the current selection. Use has_selection() to check if there is an active selection.

### has_selection {#has_selection}

```gdscript
bool has_selection() const
```

Returns true if there are selected cells.

### set_selected_palette_item {#set_selected_palette_item}

```gdscript
void set_selected_palette_item(item: int) const
```

Selects the MeshLibrary item with the given index in the grid map editor's palette. If a negative index is given, no item will be selected. If a value greater than the last index is given, the last item will be selected.

Note: The indices might not be in the same order as they appear in the editor's interface.

### set_selection {#set_selection}

```gdscript
void set_selection(begin: Vector3i, end: Vector3i)
```

Selects the cells inside the given bounds from begin to end.
