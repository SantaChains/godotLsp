# FileSystemDock

**继承**：[EditorDock](class_editordock.md) → [MarginContainer](class_margincontainer.md) → [Container](class_container.md) → [Control](class_control.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **FileSystemDock**

> This class is available only in EditorPlugins and can't be instantiated. You can access it using EditorInterface.get_file_system_dock().

<details><summary>官方英文描述（中文精译进行中）</summary>

This class is available only in EditorPlugins and can't be instantiated. You can access it using EditorInterface.get_file_system_dock().

While FileSystemDock doesn't expose any methods for file manipulation, it can listen for various file-related signals.

</details>

## 方法

### add_resource_tooltip_plugin {#add_resource_tooltip_plugin}

```gdscript
void add_resource_tooltip_plugin(plugin: EditorResourceTooltipPlugin)
```

Registers a new EditorResourceTooltipPlugin.

### navigate_to_path {#navigate_to_path}

```gdscript
void navigate_to_path(path: String)
```

Sets the given path as currently selected, ensuring that the selected file/directory is visible.

### remove_resource_tooltip_plugin {#remove_resource_tooltip_plugin}

```gdscript
void remove_resource_tooltip_plugin(plugin: EditorResourceTooltipPlugin)
```

Removes an EditorResourceTooltipPlugin. Fails if the plugin wasn't previously added.

## 信号

### inherit {#inherit}

```gdscript
signal inherit(file: String)
```

Emitted when a new scene is created that inherits the scene at file path.

### instantiate {#instantiate}

```gdscript
signal instantiate(files: PackedStringArray)
```

Emitted when the given scenes are being instantiated in the editor.

### resource_removed {#resource_removed}

```gdscript
signal resource_removed(resource: Resource)
```

Emitted when an external resource had its file removed.

### file_removed {#file_removed}

```gdscript
signal file_removed(file: String)
```

Emitted when the given file was removed.

### folder_removed {#folder_removed}

```gdscript
signal folder_removed(folder: String)
```

Emitted when the given folder was removed.

### files_moved {#files_moved}

```gdscript
signal files_moved(old_file: String, new_file: String)
```

Emitted when a file is moved from old_file path to new_file path.

### folder_moved {#folder_moved}

```gdscript
signal folder_moved(old_folder: String, new_folder: String)
```

Emitted when a folder is moved from old_folder path to new_folder path.

### folder_color_changed {#folder_color_changed}

```gdscript
signal folder_color_changed()
```

Emitted when folders change color.

### selection_changed {#selection_changed}

```gdscript
signal selection_changed()
```

Emitted when the selection changes. Use EditorInterface.get_selected_paths() in the connected method to get the selected paths.

### display_mode_changed {#display_mode_changed}

```gdscript
signal display_mode_changed()
```

Emitted when the user switches file display mode or split mode.
