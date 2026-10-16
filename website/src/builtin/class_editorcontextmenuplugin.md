# EditorContextMenuPlugin

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorContextMenuPlugin**

> EditorContextMenuPlugin allows for the addition of custom options in the editor's context menu.

<details><summary>官方英文描述</summary>

EditorContextMenuPlugin allows for the addition of custom options in the editor's context menu.

Currently, context menus are supported for three commonly used areas: the file system, scene tree, and editor script list panel.

</details>

## 方法

### _popup_menu {#_popup_menu}

```gdscript
void _popup_menu(paths: PackedStringArray) virtual
```

Called when creating a context menu, custom options can be added by using the add_context_menu_item() or add_context_menu_item_from_shortcut() functions. paths contains currently selected paths (depending on menu), which can be used to conditionally add options.

### add_context_menu_item {#add_context_menu_item}

```gdscript
void add_context_menu_item(name: String, callback: Callable, icon: Texture2D = null)
```

Add custom option to the context menu of the plugin's specified slot. When the option is activated, callback will be called. Callback should take single Array argument; array contents depend on context menu slot.

```gdscript
func _popup_menu(paths):
    add_context_menu_item("File Custom options", handle, ICON)
```

If you want to assign shortcut to the menu item, use add_context_menu_item_from_shortcut() instead.

### add_context_menu_item_from_shortcut {#add_context_menu_item_from_shortcut}

```gdscript
void add_context_menu_item_from_shortcut(name: String, shortcut: Shortcut, icon: Texture2D = null)
```

Add custom option to the context menu of the plugin's specified slot. The option will have the shortcut assigned and reuse its callback. The shortcut has to be registered beforehand with add_menu_shortcut().

```gdscript
func _init():
    add_menu_shortcut(SHORTCUT, handle)
func _popup_menu(paths):
    add_context_menu_item_from_shortcut("File Custom options", SHORTCUT, ICON)
```

### add_context_submenu_item {#add_context_submenu_item}

```gdscript
void add_context_submenu_item(name: String, menu: PopupMenu, icon: Texture2D = null)
```

Add a submenu to the context menu of the plugin's specified slot. The submenu is not automatically handled, you need to connect to its signals yourself. Also the submenu is freed on every popup, so provide a new PopupMenu every time.

```gdscript
func _popup_menu(paths):
    var popup_menu = PopupMenu.new()
    popup_menu.add_item("Blue")
    popup_menu.add_item("White")
    popup_menu.id_pressed.connect(_on_color_submenu_option)
    add_context_submenu_item("Set Node Color", popup_menu)
```

### add_menu_shortcut {#add_menu_shortcut}

```gdscript
void add_menu_shortcut(shortcut: Shortcut, callback: Callable)
```

Registers a shortcut associated with the plugin's context menu. This method should be called once (e.g. in plugin's Object._init()). callback will be called when user presses the specified shortcut while the menu's context is in effect (e.g. FileSystem dock is focused). Callback should take single Array argument; array contents depend on context menu slot.

```gdscript
func _init():
    add_menu_shortcut(SHORTCUT, handle)
```

## 枚举

### ContextMenuSlot {#enum-ContextMenuSlot}

```gdscript
enum ContextMenuSlot { CONTEXT_SLOT_SCENE_TREE = 0, CONTEXT_SLOT_FILESYSTEM = 1, CONTEXT_SLOT_SCRIPT_EDITOR = 2, CONTEXT_SLOT_FILESYSTEM_CREATE = 3, CONTEXT_SLOT_SCRIPT_EDITOR_CODE = 4, CONTEXT_SLOT_SCENE_TABS = 5, CONTEXT_SLOT_2D_EDITOR = 6, CONTEXT_SLOT_INSPECTOR_PROPERTY = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONTEXT_SLOT_SCENE_TREE {#CONTEXT_SLOT_SCENE_TREE}

```gdscript
CONTEXT_SLOT_SCENE_TREE = 0
```

Context menu of Scene dock. _popup_menu() will be called with a list of paths to currently selected nodes, while option callback will receive the list of currently selected nodes.

#### CONTEXT_SLOT_FILESYSTEM {#CONTEXT_SLOT_FILESYSTEM}

```gdscript
CONTEXT_SLOT_FILESYSTEM = 1
```

Context menu of FileSystem dock. _popup_menu() and option callback will be called with list of paths of the currently selected files.

#### CONTEXT_SLOT_SCRIPT_EDITOR {#CONTEXT_SLOT_SCRIPT_EDITOR}

```gdscript
CONTEXT_SLOT_SCRIPT_EDITOR = 2
```

Context menu of Script editor's script tabs. _popup_menu() will be called with the path to the currently edited script, while option callback will receive reference to that script.

#### CONTEXT_SLOT_FILESYSTEM_CREATE {#CONTEXT_SLOT_FILESYSTEM_CREATE}

```gdscript
CONTEXT_SLOT_FILESYSTEM_CREATE = 3
```

The "Create..." submenu of FileSystem dock's context menu, or the "New" section of the main context menu when empty space is clicked. _popup_menu() and option callback will be called with the path of the currently selected folder. When clicking the empty space, the list of paths for popup method will be empty.

```gdscript
func _popup_menu(paths):
    if paths.is_empty():
        add_context_menu_item("New Image File...", create_image)
    else:
        add_context_menu_item("Image File...", create_image)
```

#### CONTEXT_SLOT_SCRIPT_EDITOR_CODE {#CONTEXT_SLOT_SCRIPT_EDITOR_CODE}

```gdscript
CONTEXT_SLOT_SCRIPT_EDITOR_CODE = 4
```

Context menu of Script editor's code editor. _popup_menu() will be called with the path to the CodeEdit node. You can fetch it using this code:

```gdscript
func _popup_menu(paths):
    var code_edit = Engine.get_main_loop().root.get_node(paths[0]);
```

The option callback will receive reference to that node. You can use CodeEdit methods to perform symbol lookups etc.

#### CONTEXT_SLOT_SCENE_TABS {#CONTEXT_SLOT_SCENE_TABS}

```gdscript
CONTEXT_SLOT_SCENE_TABS = 5
```

Context menu of scene tabs. _popup_menu() will be called with the path of the clicked scene, or empty PackedStringArray if the menu was opened on empty space. The option callback will receive the path of the clicked scene, or empty String if none was clicked.

#### CONTEXT_SLOT_2D_EDITOR {#CONTEXT_SLOT_2D_EDITOR}

```gdscript
CONTEXT_SLOT_2D_EDITOR = 6
```

Context menu of 2D editor's basic right-click menu. _popup_menu() will be called with paths to all CanvasItem nodes under the cursor. You can fetch them using this code:

```gdscript
func _popup_menu(paths):
    var canvas_item = Engine.get_main_loop().root.get_node(paths[0]); # Replace 0 with the desired index.
```

The paths array is empty if there weren't any nodes under cursor. The option callback will receive a typed array of CanvasItem nodes.

#### CONTEXT_SLOT_INSPECTOR_PROPERTY {#CONTEXT_SLOT_INSPECTOR_PROPERTY}

```gdscript
CONTEXT_SLOT_INSPECTOR_PROPERTY = 7
```

Context menu of the inspectors right-click menu. _popup_menu() will be called with an array of two items: The first will be the object's ID, the second will be the property name. An object can be retrieved from it's ID via @GlobalScope.instance_from_id() after converting it to an int. The option callback will receive the EditorProperty directly.

---
