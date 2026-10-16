# EditorExportPreset

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorExportPreset**

> Represents the configuration of an export preset, as created by the editor's export dialog. An EditorExportPreset instance is intended to be used a read-only configuration passed to the EditorExportPlatform methods when exporting the project.

<details><summary>官方英文描述</summary>

Represents the configuration of an export preset, as created by the editor's export dialog. An EditorExportPreset instance is intended to be used a read-only configuration passed to the EditorExportPlatform methods when exporting the project.

</details>

## 方法

### are_advanced_options_enabled {#are_advanced_options_enabled}

```gdscript
bool are_advanced_options_enabled() const
```

Returns true if the "Advanced" toggle is enabled in the export dialog.

### get_custom_features {#get_custom_features}

```gdscript
String get_custom_features() const
```

Returns a comma-separated list of custom features added to this preset, as a string. See Feature tags in the documentation for more information.

### get_customized_files {#get_customized_files}

```gdscript
Dictionary get_customized_files() const
```

Returns a dictionary of files selected in the "Resources" tab of the export dialog. The dictionary's keys are file paths, and its values are the corresponding export modes: "strip", "keep", or "remove". See also get_file_export_mode().

### get_customized_files_count {#get_customized_files_count}

```gdscript
int get_customized_files_count() const
```

Returns the number of files selected in the "Resources" tab of the export dialog.

### get_encrypt_directory {#get_encrypt_directory}

```gdscript
bool get_encrypt_directory() const
```

Returns true if PCK directory encryption is enabled in the export dialog.

### get_encrypt_pck {#get_encrypt_pck}

```gdscript
bool get_encrypt_pck() const
```

Returns true if PCK encryption is enabled in the export dialog.

### get_encryption_ex_filter {#get_encryption_ex_filter}

```gdscript
String get_encryption_ex_filter() const
```

Returns file filters to exclude during PCK encryption.

### get_encryption_in_filter {#get_encryption_in_filter}

```gdscript
String get_encryption_in_filter() const
```

Returns file filters to include during PCK encryption.

### get_encryption_key {#get_encryption_key}

```gdscript
String get_encryption_key() const
```

Returns PCK encryption key.

### get_exclude_filter {#get_exclude_filter}

```gdscript
String get_exclude_filter() const
```

Returns file filters to exclude during export.

### get_export_filter {#get_export_filter}

```gdscript
EditorExportPreset.ExportFilter get_export_filter() const
```

Returns export file filter mode selected in the "Resources" tab of the export dialog.

### get_export_path {#get_export_path}

```gdscript
String get_export_path() const
```

Returns export target path.

### get_file_export_mode {#get_file_export_mode}

```gdscript
EditorExportPreset.FileExportMode get_file_export_mode(path: String, default: EditorExportPreset.FileExportMode = 0) const
```

Returns file export mode for the specified file.

### get_files_to_export {#get_files_to_export}

```gdscript
PackedStringArray get_files_to_export() const
```

Returns array of files to export.

### get_include_filter {#get_include_filter}

```gdscript
String get_include_filter() const
```

Returns file filters to include during export.

### get_or_env {#get_or_env}

```gdscript
Variant get_or_env(name: StringName, env_var: String) const
```

Returns export option value or value of environment variable if it is set.

### get_patches {#get_patches}

```gdscript
PackedStringArray get_patches() const
```

Returns the list of packs on which to base a patch export on.

### get_preset_name {#get_preset_name}

```gdscript
String get_preset_name() const
```

Returns this export preset's name.

### get_project_setting {#get_project_setting}

```gdscript
Variant get_project_setting(name: StringName)
```

Returns the value of the setting identified by name using export preset feature tag overrides instead of current OS features.

### get_script_export_mode {#get_script_export_mode}

```gdscript
EditorExportPreset.ScriptExportMode get_script_export_mode() const
```

Returns the export mode used by GDScript files. 0 for "Text", 1 for "Binary tokens", and 2 for "Compressed binary tokens (smaller files)".

### get_version {#get_version}

```gdscript
String get_version(name: StringName, windows_version: bool) const
```

Returns the preset's version number, or fall back to the ProjectSettings.application/config/version project setting if set to an empty string.

If windows_version is true, formats the returned version number to be compatible with Windows executable metadata.

### has {#has}

```gdscript
bool has(property: StringName) const
```

Returns true if the preset has the property named property.

### has_export_file {#has_export_file}

```gdscript
bool has_export_file(path: String)
```

Returns true if the file at the specified path will be exported.

### is_dedicated_server {#is_dedicated_server}

```gdscript
bool is_dedicated_server() const
```

Returns true if the dedicated server export mode is selected in the export dialog.

### is_runnable {#is_runnable}

```gdscript
bool is_runnable() const
```

Returns true if the "Runnable" toggle is enabled in the export dialog.

## 枚举

### ExportFilter {#enum-ExportFilter}

```gdscript
enum ExportFilter { EXPORT_ALL_RESOURCES = 0, EXPORT_SELECTED_SCENES = 1, EXPORT_SELECTED_RESOURCES = 2, EXCLUDE_SELECTED_RESOURCES = 3, EXPORT_CUSTOMIZED = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPORT_ALL_RESOURCES {#EXPORT_ALL_RESOURCES}

```gdscript
EXPORT_ALL_RESOURCES = 0
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### EXPORT_SELECTED_SCENES {#EXPORT_SELECTED_SCENES}

```gdscript
EXPORT_SELECTED_SCENES = 1
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### EXPORT_SELECTED_RESOURCES {#EXPORT_SELECTED_RESOURCES}

```gdscript
EXPORT_SELECTED_RESOURCES = 2
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### EXCLUDE_SELECTED_RESOURCES {#EXCLUDE_SELECTED_RESOURCES}

```gdscript
EXCLUDE_SELECTED_RESOURCES = 3
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### EXPORT_CUSTOMIZED {#EXPORT_CUSTOMIZED}

```gdscript
EXPORT_CUSTOMIZED = 4
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

---

### FileExportMode {#enum-FileExportMode}

```gdscript
enum FileExportMode { MODE_FILE_NOT_CUSTOMIZED = 0, MODE_FILE_STRIP = 1, MODE_FILE_KEEP = 2, MODE_FILE_REMOVE = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_FILE_NOT_CUSTOMIZED {#MODE_FILE_NOT_CUSTOMIZED}

```gdscript
MODE_FILE_NOT_CUSTOMIZED = 0
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### MODE_FILE_STRIP {#MODE_FILE_STRIP}

```gdscript
MODE_FILE_STRIP = 1
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### MODE_FILE_KEEP {#MODE_FILE_KEEP}

```gdscript
MODE_FILE_KEEP = 2
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### MODE_FILE_REMOVE {#MODE_FILE_REMOVE}

```gdscript
MODE_FILE_REMOVE = 3
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

---

### ScriptExportMode {#enum-ScriptExportMode}

```gdscript
enum ScriptExportMode { MODE_SCRIPT_TEXT = 0, MODE_SCRIPT_BINARY_TOKENS = 1, MODE_SCRIPT_BINARY_TOKENS_COMPRESSED = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_SCRIPT_TEXT {#MODE_SCRIPT_TEXT}

```gdscript
MODE_SCRIPT_TEXT = 0
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### MODE_SCRIPT_BINARY_TOKENS {#MODE_SCRIPT_BINARY_TOKENS}

```gdscript
MODE_SCRIPT_BINARY_TOKENS = 1
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### MODE_SCRIPT_BINARY_TOKENS_COMPRESSED {#MODE_SCRIPT_BINARY_TOKENS_COMPRESSED}

```gdscript
MODE_SCRIPT_BINARY_TOKENS_COMPRESSED = 2
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

---
