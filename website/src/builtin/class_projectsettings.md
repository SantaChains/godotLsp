# ProjectSettings

**继承**：[Object](class_object.md) → **ProjectSettings**

> Stores variables that can be accessed from everywhere. Use get_setting(), set_setting() or has_setting() to access them. Variables stored in project.godot are also loaded into ProjectSettings, making this object very useful for reading custom game configuration options.

<details><summary>官方英文描述（中文精译进行中）</summary>

Stores variables that can be accessed from everywhere. Use get_setting(), set_setting() or has_setting() to access them. Variables stored in project.godot are also loaded into ProjectSettings, making this object very useful for reading custom game configuration options.

When naming a Project Settings property, use the full path to the setting including the category. For example, "application/config/name" for the project name. Category and property names can be viewed in the Project Settings dialog.

Feature tags: Project settings can be overridden for specific platforms and configurations (debug, release, ...) using feature tags.

Overriding: Any project setting can be overridden by creating a file named override.cfg in the project's root directory. This file is in the same format as project.godot, and can also be written using ConfigFile. This can also be used in exported projects by placing this file in the same directory as the project binary. Overriding will still take the base project settings' feature tags in account. Therefore, make sure to also override the setting with the desired feature tags if you want them to override base project settings on all platforms and configurations.

</details>

## 方法

### add_property_info {#add_property_info}

```gdscript
void add_property_info(hint: Dictionary)
```

Adds a custom property info to a property. The dictionary must contain:

- "name": String (the property's name)
- "type": int (see Variant.Type)
- optionally "hint": int (see PropertyHint) and "hint_string": String

```
ProjectSettings.set("category/property_name", 0)
var property_info = {
    "name": "category/property_name",
    "type": TYPE_INT,
    "hint": PROPERTY_HINT_ENUM,
    "hint_string": "one,two,three"
}
ProjectSettings.add_property_info(property_info)
```

```
ProjectSettings.Singleton.Set("category/property_name", 0);
var propertyInfo = new Godot.Collections.Dictionary
{
    { "name", "category/propertyName" },
    { "type", (int)Variant.Type.Int },
    { "hint", (int)PropertyHint.Enum },
    { "hint_string", "one,two,three" },
};
ProjectSettings.AddPropertyInfo(propertyInfo);
```

Note: Setting "usage" for the property is not supported. Use set_as_basic(), set_restart_if_changed(), and set_as_internal() to modify usage flags.

### check_changed_settings_in_group {#check_changed_settings_in_group}

```gdscript
bool check_changed_settings_in_group(setting_prefix: String) const
```

Checks if any settings with the prefix setting_prefix exist in the set of changed settings. See also get_changed_settings().

### clear {#clear}

```gdscript
void clear(name: String)
```

Clears the whole configuration (not recommended, may break things).

### get_changed_settings {#get_changed_settings}

```gdscript
PackedStringArray get_changed_settings() const
```

Gets an array of the settings which have been changed since the last save. Note that internally changed_settings is cleared after a successful save, so generally the most appropriate place to use this method is when processing settings_changed.

### get_global_class_list {#get_global_class_list}

```gdscript
typedarray::Dictionary get_global_class_list()
```

Returns an Array of registered global classes. Each global class is represented as a Dictionary that contains the following entries:

- base is a name of the base class;
- class is a name of the registered global class;
- icon is a path to a custom icon of the global class, if it has any;
- language is a name of a programming language in which the global class is written;
- path is a path to a file containing the global class.

Note: Both the script and the icon paths are local to the project filesystem, i.e. they start with res://.

### get_order {#get_order}

```gdscript
int get_order(name: String) const
```

Returns the order of a configuration value (influences when saved to the config file).

### get_setting {#get_setting}

```gdscript
Variant get_setting(name: String, default_value: Variant = null) const
```

Returns the value of the setting identified by name. If the setting doesn't exist and default_value is specified, the value of default_value is returned. Otherwise, null is returned.

```
print(ProjectSettings.get_setting("application/config/name"))
print(ProjectSettings.get_setting("application/config/custom_description", "No description specified."))
```

```
GD.Print(ProjectSettings.GetSetting("application/config/name"));
GD.Print(ProjectSettings.GetSetting("application/config/custom_description", "No description specified."));
```

Note: This method doesn't take potential feature overrides into account automatically. Use get_setting_with_override() to handle seamlessly.

See also has_setting() to check whether a setting exists.

### get_setting_with_override {#get_setting_with_override}

```gdscript
Variant get_setting_with_override(name: StringName) const
```

Similar to get_setting(), but applies feature tag overrides if any exists and is valid.

Example: If the setting override "application/config/name.windows" exists, and the following code is executed on a Windows operating system, the overridden setting is printed instead:

```
print(ProjectSettings.get_setting_with_override("application/config/name"))
```

```
GD.Print(ProjectSettings.GetSettingWithOverride("application/config/name"));
```

### get_setting_with_override_and_custom_features {#get_setting_with_override_and_custom_features}

```gdscript
Variant get_setting_with_override_and_custom_features(name: StringName, features: PackedStringArray) const
```

Similar to get_setting_with_override(), but applies feature tag overrides instead of current OS features.

### globalize_path {#globalize_path}

```gdscript
String globalize_path(path: String) const
```

Returns the absolute, native OS path corresponding to the localized path (starting with res:// or user://). The returned path will vary depending on the operating system and user preferences. See File paths in Godot projects to see what those paths convert to. See also localize_path().

Note: globalize_path() with res:// will not work in an exported project. Instead, prepend the executable's base directory to the path when running from an exported project:

```gdscript
var path = ""
if OS.has_feature("editor"):
    # Running from an editor binary.
    # `path` will contain the absolute path to `hello.txt` located in the project root.
    path = ProjectSettings.globalize_path("res://hello.txt")
else:
    # Running from an exported project.
    # `path` will contain the absolute path to `hello.txt` next to the executable.
    # This is *not* identical to using `ProjectSettings.globalize_path()` with a `res://` path,
    # but is close enough in spirit.
    path = OS.get_executable_path().get_base_dir().path_join("hello.txt")
```

### has_setting {#has_setting}

```gdscript
bool has_setting(name: String) const
```

Returns true if a configuration value is present.

Note: In order to be be detected, custom settings have to be either defined with set_setting(), or exist in the project.godot file. This is especially relevant when using set_initial_value().

### load_resource_pack {#load_resource_pack}

```gdscript
bool load_resource_pack(pack: String, replace_files: bool = true, offset: int = 0)
```

Loads the contents of the .pck or .zip file specified by pack into the resource filesystem (res://). Returns true on success.

Note: If a file from pack shares the same path as a file already in the resource filesystem, any attempts to load that file will use the file from pack unless replace_files is set to false.

Note: The optional offset parameter can be used to specify the offset in bytes to the start of the resource pack. This is only supported for .pck files.

Note: DirAccess will not show changes made to the contents of res:// after calling this function.

### localize_path {#localize_path}

```gdscript
String localize_path(path: String) const
```

Returns the localized path (starting with res://) corresponding to the absolute, native OS path. See also globalize_path().

### save {#save}

```gdscript
Error save()
```

Saves the configuration to the project.godot file.

Note: This method is intended to be used by editor plugins, as modified ProjectSettings can't be loaded back in the running app. If you want to change project settings in exported projects, use save_custom() to save an override.cfg file.

### save_custom {#save_custom}

```gdscript
Error save_custom(file: String)
```

Saves the configuration to a custom file. The file extension must be .godot (to save in text-based ConfigFile format) or .binary (to save in binary format). You can also save an override.cfg file, which is also text, but can be used in exported projects unlike other formats.

### set_as_basic {#set_as_basic}

```gdscript
void set_as_basic(name: String, basic: bool)
```

Defines if the specified setting is considered basic or advanced. Basic settings will always be shown in the project settings. Advanced settings will only be shown if the user enables the "Advanced Settings" option.

### set_as_internal {#set_as_internal}

```gdscript
void set_as_internal(name: String, internal: bool)
```

Defines if the specified setting is considered internal. An internal setting won't show up in the Project Settings dialog. This is mostly useful for addons that need to store their own internal settings without exposing them directly to the user.

### set_initial_value {#set_initial_value}

```gdscript
void set_initial_value(name: String, value: Variant)
```

Sets the specified setting's initial value. This is the value the setting reverts to. The setting should already exist before calling this method. Note that project settings equal to their default value are not saved, so your code needs to account for that.

```gdscript
extends EditorPlugin
const SETTING_NAME = "addons/my_setting"
const SETTING_DEFAULT = 10.0
func _enter_tree():
    if not ProjectSettings.has_setting(SETTING_NAME):
        ProjectSettings.set_setting(SETTING_NAME, SETTING_DEFAULT)
    ProjectSettings.set_initial_value(SETTING_NAME, SETTING_DEFAULT)
```

If you have a project setting defined by an EditorPlugin, but want to use it in a running project, you will need a similar code at runtime.

### set_order {#set_order}

```gdscript
void set_order(name: String, position: int)
```

Sets the order of a configuration value (influences when saved to the config file).

### set_restart_if_changed {#set_restart_if_changed}

```gdscript
void set_restart_if_changed(name: String, restart: bool)
```

Sets whether a setting requires restarting the editor to properly take effect.

Note: This is just a hint to display to the user that the editor must be restarted for changes to take effect. Enabling set_restart_if_changed() does not delay the setting being set when changed.

### set_setting {#set_setting}

```gdscript
void set_setting(name: String, value: Variant)
```

Sets the value of a setting.

```
ProjectSettings.set_setting("application/config/name", "Example")
```

```
ProjectSettings.SetSetting("application/config/name", "Example");
```

This can also be used to erase custom project settings. To do this change the setting value to null.

## 信号

### settings_changed {#settings_changed}

```gdscript
signal settings_changed()
```

Emitted when any setting is changed, up to once per process frame.
