# EditorInterface

**继承**：[Object](class_object.md) → **EditorInterface**

> EditorInterface gives you control over Godot editor's window. It allows customizing the window, saving and (re-)loading scenes, rendering mesh previews, inspecting and editing resources and objects, and provides access to EditorSettings, EditorFileSystem, EditorResourcePreview, ScriptEditor, the editor viewport, and information about scenes.

<details><summary>官方英文描述（中文精译进行中）</summary>

EditorInterface gives you control over Godot editor's window. It allows customizing the window, saving and (re-)loading scenes, rendering mesh previews, inspecting and editing resources and objects, and provides access to EditorSettings, EditorFileSystem, EditorResourcePreview, ScriptEditor, the editor viewport, and information about scenes.

Note: This class shouldn't be instantiated directly. Instead, access the singleton directly by its name.

```
var editor_settings = EditorInterface.get_editor_settings()
```

```
// In C# you can access it via the static Singleton property.
EditorSettings settings = EditorInterface.Singleton.GetEditorSettings();
```

</details>

## 方法

### add_root_node {#add_root_node}

```gdscript
void add_root_node(node: Node)
```

Makes node root of the currently opened scene. Only works if the scene is empty. If the node is a scene instance, an inheriting scene will be created.

### close_scene {#close_scene}

```gdscript
Error close_scene()
```

Closes the currently active scene, discarding any pending changes in the process. Returns @GlobalScope.OK on success or @GlobalScope.ERR_DOES_NOT_EXIST if there is no scene to close.

### edit_node {#edit_node}

```gdscript
void edit_node(node: Node)
```

Edits the given Node. The node will be also selected if it's inside the scene tree.

### edit_resource {#edit_resource}

```gdscript
void edit_resource(resource: Resource)
```

Edits the given Resource. If the resource is a Script you can also edit it with edit_script() to specify the line and column position.

### edit_script {#edit_script}

```gdscript
void edit_script(script: Script, line: int = -1, column: int = 0, grab_focus: bool = true)
```

Edits the given Script. The line and column on which to open the script can also be specified. The script will be open with the user-configured editor for the script's language which may be an external editor.

### get_base_control {#get_base_control}

```gdscript
Control get_base_control() const
```

Returns the main container of Godot editor's window. For example, you can use it to retrieve the size of the container and place your controls accordingly.

Warning: Removing and freeing this node will render the editor useless and may cause a crash.

### get_command_palette {#get_command_palette}

```gdscript
EditorCommandPalette get_command_palette() const
```

Returns the editor's EditorCommandPalette instance.

Warning: Removing and freeing this node will render a part of the editor useless and may cause a crash.

### get_current_directory {#get_current_directory}

```gdscript
String get_current_directory() const
```

Returns the current directory being viewed in the FileSystemDock. If a file is selected, its base directory will be returned using String.get_base_dir() instead.

### get_current_feature_profile {#get_current_feature_profile}

```gdscript
String get_current_feature_profile() const
```

Returns the name of the currently activated feature profile. If the default profile is currently active, an empty string is returned instead.

In order to get a reference to the EditorFeatureProfile, you must load the feature profile using EditorFeatureProfile.load_from_file().

Note: Feature profiles created via the user interface are loaded from the feature_profiles directory, as a file with the .profile extension. The editor configuration folder can be found by using EditorPaths.get_config_dir().

### get_current_path {#get_current_path}

```gdscript
String get_current_path() const
```

Returns the current path being viewed in the FileSystemDock.

### get_edited_scene_root {#get_edited_scene_root}

```gdscript
Node get_edited_scene_root() const
```

Returns the edited (current) scene's root Node.

### get_editor_language {#get_editor_language}

```gdscript
String get_editor_language() const
```

Returns the language currently used for the editor interface.

### get_editor_main_screen {#get_editor_main_screen}

```gdscript
VBoxContainer get_editor_main_screen() const
```

Returns the editor control responsible for main screen plugins and tools. Use it with plugins that implement EditorPlugin._has_main_screen().

Note: This node is a VBoxContainer, which means that if you add a Control child to it, you need to set the child's Control.size_flags_vertical to Control.SIZE_EXPAND_FILL to make it use the full available space.

Warning: Removing and freeing this node will render a part of the editor useless and may cause a crash.

### get_editor_paths {#get_editor_paths}

```gdscript
EditorPaths get_editor_paths() const
```

Returns the EditorPaths singleton.

### get_editor_scale {#get_editor_scale}

```gdscript
float get_editor_scale() const
```

Returns the actual scale of the editor UI (1.0 being 100% scale). This can be used to adjust position and dimensions of the UI added by plugins.

Note: This value is set via the EditorSettings.interface/editor/appearance/display_scale and EditorSettings.interface/editor/appearance/custom_display_scale settings. The editor must be restarted for changes to be properly applied.

### get_editor_settings {#get_editor_settings}

```gdscript
EditorSettings get_editor_settings() const
```

Returns the editor's EditorSettings instance.

### get_editor_theme {#get_editor_theme}

```gdscript
Theme get_editor_theme() const
```

Returns the editor's Theme.

Note: When creating custom editor UI, prefer accessing theme items directly from your GUI nodes using the get_theme_* methods.

### get_editor_toaster {#get_editor_toaster}

```gdscript
EditorToaster get_editor_toaster() const
```

Returns the editor's EditorToaster.

### get_editor_undo_redo {#get_editor_undo_redo}

```gdscript
EditorUndoRedoManager get_editor_undo_redo() const
```

Returns the editor's EditorUndoRedoManager.

### get_editor_viewport_2d {#get_editor_viewport_2d}

```gdscript
SubViewport get_editor_viewport_2d() const
```

Returns the 2D editor SubViewport. It does not have a camera. Instead, the view transforms are done directly and can be accessed with Viewport.global_canvas_transform.

### get_editor_viewport_3d {#get_editor_viewport_3d}

```gdscript
SubViewport get_editor_viewport_3d(idx: int = 0) const
```

Returns the specified 3D editor SubViewport, from 0 to 3. The viewport can be used to access the active editor cameras with Viewport.get_camera_3d().

### get_file_system_dock {#get_file_system_dock}

```gdscript
FileSystemDock get_file_system_dock() const
```

Returns the editor's FileSystemDock instance.

Warning: Removing and freeing this node will render a part of the editor useless and may cause a crash.

### get_inspector {#get_inspector}

```gdscript
EditorInspector get_inspector() const
```

Returns the editor's EditorInspector instance.

Warning: Removing and freeing this node will render a part of the editor useless and may cause a crash.

### get_node_3d_rotate_snap {#get_node_3d_rotate_snap}

```gdscript
float get_node_3d_rotate_snap() const
```

Returns the amount of degrees the 3D editor's rotational snapping is set to.

### get_node_3d_scale_snap {#get_node_3d_scale_snap}

```gdscript
float get_node_3d_scale_snap() const
```

Returns the amount of units the 3D editor's scale snapping is set to.

### get_node_3d_translate_snap {#get_node_3d_translate_snap}

```gdscript
float get_node_3d_translate_snap() const
```

Returns the amount of units the 3D editor's translation snapping is set to.

### get_open_scene_roots {#get_open_scene_roots}

```gdscript
typedarray::Node get_open_scene_roots() const
```

Returns an array with references to the root nodes of the currently opened scenes.

### get_open_scenes {#get_open_scenes}

```gdscript
PackedStringArray get_open_scenes() const
```

Returns an array with the file paths of the currently opened scenes.

### get_playing_scene {#get_playing_scene}

```gdscript
String get_playing_scene() const
```

Returns the name of the scene that is being played. If no scene is currently being played, returns an empty string.

### get_resource_filesystem {#get_resource_filesystem}

```gdscript
EditorFileSystem get_resource_filesystem() const
```

Returns the editor's EditorFileSystem instance.

### get_resource_previewer {#get_resource_previewer}

```gdscript
EditorResourcePreview get_resource_previewer() const
```

Returns the editor's EditorResourcePreview instance.

### get_script_editor {#get_script_editor}

```gdscript
ScriptEditor get_script_editor() const
```

Returns the editor's ScriptEditor instance.

Warning: Removing and freeing this node will render a part of the editor useless and may cause a crash.

### get_selected_paths {#get_selected_paths}

```gdscript
PackedStringArray get_selected_paths() const
```

Returns an array containing the paths of the currently selected files (and directories) in the FileSystemDock.

### get_selection {#get_selection}

```gdscript
EditorSelection get_selection() const
```

Returns the editor's EditorSelection instance.

### get_unsaved_scenes {#get_unsaved_scenes}

```gdscript
PackedStringArray get_unsaved_scenes() const
```

Returns an array of file paths of currently unsaved scenes.

### inspect_object {#inspect_object}

```gdscript
void inspect_object(object: Object, for_property: String = "", inspector_only: bool = false)
```

Shows the given property on the given object in the editor's Inspector dock. If inspector_only is true, plugins will not attempt to edit object.

### is_distraction_free_mode_enabled {#is_distraction_free_mode_enabled}

```gdscript
bool is_distraction_free_mode_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_movie_maker_enabled {#is_movie_maker_enabled}

```gdscript
bool is_movie_maker_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_multi_window_enabled {#is_multi_window_enabled}

```gdscript
bool is_multi_window_enabled() const
```

Returns true if multiple window support is enabled in the editor. Multiple window support is enabled if all of these statements are true:

- EditorSettings.interface/multi_window/enable is true.
- EditorSettings.interface/editor/display/single_window_mode is false.
- Viewport.gui_embed_subwindows is false. This is forced to true on platforms that don't support multiple windows such as Web, or when the --single-window command line argument is used.

### is_node_3d_snap_enabled {#is_node_3d_snap_enabled}

```gdscript
bool is_node_3d_snap_enabled() const
```

Returns true if the 3D editor currently has snapping mode enabled, and false otherwise.

### is_object_edited {#is_object_edited}

```gdscript
bool is_object_edited(object: Object) const
```

Returns true if the object has been marked as edited through set_object_edited().

### is_playing_scene {#is_playing_scene}

```gdscript
bool is_playing_scene() const
```

Returns true if a scene is currently being played, false otherwise. Paused scenes are considered as being played.

### is_plugin_enabled {#is_plugin_enabled}

```gdscript
bool is_plugin_enabled(plugin: String) const
```

Returns true if the specified plugin is enabled. The plugin name is the same as its directory name.

### make_mesh_previews {#make_mesh_previews}

```gdscript
typedarray::Texture2D make_mesh_previews(meshes: typedarray::Mesh, preview_size: int)
```

Returns mesh previews rendered at the given size as an Array of Texture2Ds.

### mark_scene_as_unsaved {#mark_scene_as_unsaved}

```gdscript
void mark_scene_as_unsaved()
```

Marks the current scene tab as unsaved.

### open_scene_from_path {#open_scene_from_path}

```gdscript
void open_scene_from_path(scene_filepath: String, set_inherited: bool = false)
```

Opens the scene at the given path. If set_inherited is true, creates a new inherited scene.

### play_current_scene {#play_current_scene}

```gdscript
void play_current_scene()
```

Plays the currently active scene.

### play_custom_scene {#play_custom_scene}

```gdscript
void play_custom_scene(scene_filepath: String)
```

Plays the scene specified by its filepath.

### play_main_scene {#play_main_scene}

```gdscript
void play_main_scene()
```

Plays the main scene.

### popup_create_dialog {#popup_create_dialog}

```gdscript
void popup_create_dialog(callback: Callable, base_type: StringName = "", current_type: String = "", dialog_title: String = "", type_blocklist: typedarray::StringName = Array[StringName]([]))
```

Experimental: This method may be changed or removed in future versions.

Pops up an editor dialog for creating an object.

The callback must take a single argument of type String, which will contain the type name of the selected object (or the script path of the type, if the type is created from a script), or be an empty string if no item is selected.

The base_type specifies the base type of objects to display. For example, if you set this to "Resource", all types derived from Resource will display in the create dialog.

The current_type will be passed in the search box of the create dialog, and the specified type can be immediately selected when the dialog pops up. If the current_type is not derived from base_type, there will be no result of the type in the dialog.

The dialog_title allows you to define a custom title for the dialog. This is useful if you want to accurately hint the usage of the dialog. If the dialog_title is an empty string, the dialog will use "Create New 'Base Type'" as the default title.

The type_blocklist contains a list of type names, and the types in the blocklist will be hidden from the create dialog.

Note: Trying to list the base type in the type_blocklist will hide all types derived from the base type from the create dialog.

### popup_dialog {#popup_dialog}

```gdscript
void popup_dialog(dialog: Window, rect: Rect2i = Rect2i(0, 0, 0, 0))
```

Pops up the dialog in the editor UI with Window.popup_exclusive(). The dialog must have no current parent, otherwise the method fails.

See also Window.set_unparent_when_invisible().

### popup_dialog_centered {#popup_dialog_centered}

```gdscript
void popup_dialog_centered(dialog: Window, minsize: Vector2i = Vector2i(0, 0))
```

Pops up the dialog in the editor UI with Window.popup_exclusive_centered(). The dialog must have no current parent, otherwise the method fails.

See also Window.set_unparent_when_invisible().

### popup_dialog_centered_clamped {#popup_dialog_centered_clamped}

```gdscript
void popup_dialog_centered_clamped(dialog: Window, minsize: Vector2i = Vector2i(0, 0), fallback_ratio: float = 0.75)
```

Pops up the dialog in the editor UI with Window.popup_exclusive_centered_clamped(). The dialog must have no current parent, otherwise the method fails.

See also Window.set_unparent_when_invisible().

### popup_dialog_centered_ratio {#popup_dialog_centered_ratio}

```gdscript
void popup_dialog_centered_ratio(dialog: Window, ratio: float = 0.8)
```

Pops up the dialog in the editor UI with Window.popup_exclusive_centered_ratio(). The dialog must have no current parent, otherwise the method fails.

See also Window.set_unparent_when_invisible().

### popup_method_selector {#popup_method_selector}

```gdscript
void popup_method_selector(object: Object, callback: Callable, current_value: String = "")
```

Pops up an editor dialog for selecting a method from object. The callback must take a single argument of type String which will contain the name of the selected method or be empty if the dialog is canceled. If current_value is provided, the method will be selected automatically in the method list, if it exists.

### popup_node_selector {#popup_node_selector}

```gdscript
void popup_node_selector(callback: Callable, valid_types: typedarray::StringName = Array[StringName]([]), current_value: Node = null)
```

Pops up an editor dialog for selecting a Node from the edited scene. The callback must take a single argument of type NodePath. It is called on the selected NodePath or the empty path ^"" if the dialog is canceled. If valid_types is provided, the dialog will only show Nodes that match one of the listed Node types. If current_value is provided, the Node will be automatically selected in the tree, if it exists.

Example: Display the node selection dialog as soon as this node is added to the tree for the first time:

```gdscript
func _ready():
    if Engine.is_editor_hint():
        EditorInterface.popup_node_selector(_on_node_selected, ["Button"])
func _on_node_selected(node_path):
    if node_path.is_empty():
        print("node selection canceled")
    else:
        print("selected ", node_path)
```

### popup_property_selector {#popup_property_selector}

```gdscript
void popup_property_selector(object: Object, callback: Callable, type_filter: PackedInt32Array = PackedInt32Array(), current_value: String = "")
```

Pops up an editor dialog for selecting properties from object. The callback must take a single argument of type NodePath. It is called on the selected property path (see NodePath.get_as_property_path()) or the empty path ^"" if the dialog is canceled. If type_filter is provided, the dialog will only show properties that match one of the listed Variant.Type values. If current_value is provided, the property will be selected automatically in the property list, if it exists.

```gdscript
func _ready():
    if Engine.is_editor_hint():
        EditorInterface.popup_property_selector(this, _on_property_selected, [TYPE_INT])
func _on_property_selected(property_path):
    if property_path.is_empty():
        print("property selection canceled")
    else:
        print("selected ", property_path)
```

### popup_quick_open {#popup_quick_open}

```gdscript
void popup_quick_open(callback: Callable, base_types: typedarray::StringName = Array[StringName]([]))
```

Pops up an editor dialog for quick selecting a resource file. The callback must take a single argument of type String which will contain the path of the selected resource or be empty if the dialog is canceled. If base_types is provided, the dialog will only show resources that match these types. Only types deriving from Resource are supported.

### reload_scene_from_path {#reload_scene_from_path}

```gdscript
void reload_scene_from_path(scene_filepath: String)
```

Reloads the scene at the given path. Fails if the scene is not open.

### restart_editor {#restart_editor}

```gdscript
void restart_editor(save: bool = true)
```

Restarts the editor. This closes the editor and then opens the same project. If save is true, the project will be saved before restarting.

### save_all_scenes {#save_all_scenes}

```gdscript
void save_all_scenes()
```

Saves all opened scenes in the editor.

### save_scene {#save_scene}

```gdscript
Error save_scene()
```

Saves the currently active scene. Returns either @GlobalScope.OK or @GlobalScope.ERR_CANT_CREATE.

### save_scene_as {#save_scene_as}

```gdscript
void save_scene_as(path: String, with_preview: bool = true)
```

Saves the currently active scene as a file at path.

### select_file {#select_file}

```gdscript
void select_file(file: String)
```

Selects the file, with the path provided by file, in the FileSystem dock.

### set_current_feature_profile {#set_current_feature_profile}

```gdscript
void set_current_feature_profile(profile_name: String)
```

Selects and activates the specified feature profile with the given profile_name. Set profile_name to an empty string to reset to the default feature profile.

A feature profile can be created programmatically using the EditorFeatureProfile class.

Note: The feature profile that gets activated must be located in the feature_profiles directory, as a file with the .profile extension. If a profile could not be found, an error occurs. The editor configuration folder can be found by using EditorPaths.get_config_dir().

### set_distraction_free_mode {#set_distraction_free_mode}

```gdscript
void set_distraction_free_mode(enter: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_main_screen_editor {#set_main_screen_editor}

```gdscript
void set_main_screen_editor(name: String)
```

Sets the editor's current main screen to the one specified in name. name must match the title of the tab in question exactly (e.g. 2D, 3D, Script, Game, or Asset Store for default tabs).

### set_movie_maker_enabled {#set_movie_maker_enabled}

```gdscript
void set_movie_maker_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_object_edited {#set_object_edited}

```gdscript
void set_object_edited(object: Object, edited: bool)
```

If edited is true, the object is marked as edited.

Note: This is primarily used by the editor for Resource based objects to track their modified state. For example, any changes to an open scene, a resource in the inspector, or an edited script will cause this method to be called with true. Saving the scene, script, or resource resets the edited state by calling this method with false.

Note: Each call to this method increments the object's edited version. This is used to track changes in the editor and to trigger when thumbnails should be regenerated for resources.

### set_plugin_enabled {#set_plugin_enabled}

```gdscript
void set_plugin_enabled(plugin: String, enabled: bool)
```

Sets the enabled status of a plugin. The plugin name is the same as its directory name.

### stop_playing_scene {#stop_playing_scene}

```gdscript
void stop_playing_scene()
```

Stops the scene that is currently playing.

## 属性

### distraction_free_mode {#distraction_free_mode}

```gdscript
distraction_free_mode: bool
```

- void set_distraction_free_mode(value: bool)
- bool is_distraction_free_mode_enabled()

If true, enables distraction-free mode which hides side docks to increase the space available for the main view.

### movie_maker_enabled {#movie_maker_enabled}

```gdscript
movie_maker_enabled: bool
```

- void set_movie_maker_enabled(value: bool)
- bool is_movie_maker_enabled()

If true, the Movie Maker mode is enabled in the editor. See MovieWriter for more information.
