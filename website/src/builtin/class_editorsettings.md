# EditorSettings

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorSettings**

> Object that holds the project-independent editor settings. These settings are generally visible in the Editor > Editor Settings menu.

<details><summary>官方英文描述（中文精译进行中）</summary>

Object that holds the project-independent editor settings. These settings are generally visible in the Editor > Editor Settings menu.

Property names use slash delimiters to distinguish sections. Setting values can be of any Variant type. It's recommended to use snake_case for editor settings to be consistent with the Godot editor itself.

Editor settings are saved automatically when changed.

Accessing the settings can be done using the following methods, such as:

```
var settings = EditorInterface.get_editor_settings()
# `settings.set("some/property", 10)` also works as this class overrides `_set()` internally.
settings.set_setting("some/property", 10)
# `settings.get("some/property")` also works as this class overrides `_get()` internally.
settings.get_setting("some/property")
var list_of_settings = settings.get_property_list()
```

```
EditorSettings settings = EditorInterface.Singleton.GetEditorSettings();
// `settings.set("some/property", value)` also works as this class overrides `_set()` internally.
settings.SetSetting("some/property", Value);
// `settings.get("some/property", value)` also works as this class overrides `_get()` internally.
settings.GetSetting("some/property");
Godot.Collections.Array listOfSettings = settings.GetPropertyList();
```

Note: This class shouldn't be instantiated directly. Instead, access the singleton using EditorInterface.get_editor_settings().

</details>

## 方法

### add_property_info {#add_property_info}

```gdscript
void add_property_info(info: Dictionary)
```

Adds a custom property info to a property. The dictionary must contain:

- name: String (the name of the property)
- type: int (see Variant.Type)
- optionally hint: int (see PropertyHint) and hint_string: String

```
var settings = EditorInterface.get_editor_settings()
settings.set("category/property_name", 0)
var property_info = {
    "name": "category/property_name",
    "type": TYPE_INT,
    "hint": PROPERTY_HINT_ENUM,
    "hint_string": "one,two,three"
}
settings.add_property_info(property_info)
```

```
var settings = GetEditorInterface().GetEditorSettings();
settings.Set("category/property_name", 0);
var propertyInfo = new Godot.Collections.Dictionary
{
    { "name", "category/propertyName" },
    { "type", Variant.Type.Int },
    { "hint", PropertyHint.Enum },
    { "hint_string", "one,two,three" },
};
settings.AddPropertyInfo(propertyInfo);
```

### add_shortcut {#add_shortcut}

```gdscript
void add_shortcut(path: String, shortcut: Shortcut)
```

Adds a shortcut whose path is specified by path.

The path determines how the shortcut is organized and displayed in the editor's shortcut settings. The path format affects the display as follows:

- "name" (no slash): Creates a category named name with the shortcut displayed as name.
- "category/name" (single slash): Displays as name in the category section.
- "category/name/extra" (multiple slashes): Extra path components are ignored, so this behaves the same as "category/name".

Note: Shortcuts are only saved to the editor settings if they differ from their original/default state. This means empty shortcuts that were originally empty will not persist between editor sessions and must be re-added. If a shortcut with the same path already exists, this method will update it with the new shortcut instead of creating a duplicate.

```gdscript
# Add a custom shortcut for a plugin action.
var my_shortcut = Shortcut.new()
var input_event = InputEventKey.new()
input_event.keycode = KEY_F5
input_event.ctrl_pressed = true
my_shortcut.events.append(input_event)
# This will appear under the "My Plugin" category as "Reload Data".
EditorInterface.get_editor_settings().add_shortcut("my_plugin/reload_data", my_shortcut)
# This will appear under the "Test Action" category as "Test Action".
EditorInterface.get_editor_settings().add_shortcut("test_action", my_shortcut)
```

### check_changed_settings_in_group {#check_changed_settings_in_group}

```gdscript
bool check_changed_settings_in_group(setting_prefix: String) const
```

Checks if any settings with the prefix setting_prefix exist in the set of changed settings. See also get_changed_settings().

### erase {#erase}

```gdscript
void erase(property: String)
```

Erases the setting whose name is specified by property.

### get_changed_settings {#get_changed_settings}

```gdscript
PackedStringArray get_changed_settings() const
```

Gets an array of the settings which have been changed since the last save. Note that internally changed_settings is cleared after a successful save, so generally the most appropriate place to use this method is when processing NOTIFICATION_EDITOR_SETTINGS_CHANGED.

### get_favorites {#get_favorites}

```gdscript
PackedStringArray get_favorites() const
```

Returns the list of favorite files and directories for this project.

### get_project_metadata {#get_project_metadata}

```gdscript
Variant get_project_metadata(section: String, key: String, default: Variant = null) const
```

Returns project-specific metadata for the section and key specified. If the metadata doesn't exist, default will be returned instead. See also set_project_metadata().

### get_recent_dirs {#get_recent_dirs}

```gdscript
PackedStringArray get_recent_dirs() const
```

Returns the list of recently visited folders in the file dialog for this project.

### get_setting {#get_setting}

```gdscript
Variant get_setting(name: String) const
```

Returns the value of the setting specified by name. This is equivalent to using Object.get() on the EditorSettings instance.

### get_shortcut {#get_shortcut}

```gdscript
Shortcut get_shortcut(path: String) const
```

Returns the shortcut specified by path. Tries to find a built-in action if no shortcut with the provided path is found in the shortcut list. If found, adds it to the list and returns it, otherwise returns null.

### get_shortcut_list {#get_shortcut_list}

```gdscript
PackedStringArray get_shortcut_list()
```

Returns the list of stored shortcut paths.

### has_setting {#has_setting}

```gdscript
bool has_setting(name: String) const
```

Returns true if the setting specified by name exists, false otherwise.

### has_shortcut {#has_shortcut}

```gdscript
bool has_shortcut(path: String) const
```

Returns true if the shortcut specified by path exists, false otherwise.

### is_shortcut {#is_shortcut}

```gdscript
bool is_shortcut(path: String, event: InputEvent) const
```

Returns true if the shortcut specified by path matches the event specified by event, false otherwise.

### mark_setting_changed {#mark_setting_changed}

```gdscript
void mark_setting_changed(setting: String)
```

Marks the passed editor setting as being changed, see get_changed_settings(). Only settings which exist (see has_setting()) will be accepted.

### remove_shortcut {#remove_shortcut}

```gdscript
void remove_shortcut(path: String)
```

Removes the shortcut specified by path.

### set_builtin_action_override {#set_builtin_action_override}

```gdscript
void set_builtin_action_override(name: String, actions_list: typedarray::InputEvent)
```

Overrides the built-in editor action name with the input actions defined in actions_list.

### set_favorites {#set_favorites}

```gdscript
void set_favorites(dirs: PackedStringArray)
```

Sets the list of favorite files and directories for this project.

### set_initial_value {#set_initial_value}

```gdscript
void set_initial_value(name: StringName, value: Variant, update_current: bool)
```

Sets the initial value of the setting specified by name to value. This is used to provide a value for the Revert button in the Editor Settings. If update_current is true, the setting is reset to value as well.

### set_project_metadata {#set_project_metadata}

```gdscript
void set_project_metadata(section: String, key: String, data: Variant)
```

Sets project-specific metadata with the section, key and data specified. This metadata is stored outside the project folder and therefore won't be checked into version control. See also get_project_metadata().

### set_recent_dirs {#set_recent_dirs}

```gdscript
void set_recent_dirs(dirs: PackedStringArray)
```

Sets the list of recently visited folders in the file dialog for this project.

### set_setting {#set_setting}

```gdscript
void set_setting(name: String, value: Variant)
```

Sets the value of the setting specified by name. This is equivalent to using Object.set() on the EditorSettings instance.

## 信号

### settings_changed {#settings_changed}

```gdscript
signal settings_changed()
```

Emitted after any editor setting has changed.

## 常量

### NOTIFICATION_EDITOR_SETTINGS_CHANGED {#NOTIFICATION_EDITOR_SETTINGS_CHANGED}

```gdscript
const NOTIFICATION_EDITOR_SETTINGS_CHANGED = 10000
```

Emitted after any editor setting has changed. It's used by various editor plugins to update their visuals on theme changes or logic on configuration changes.
