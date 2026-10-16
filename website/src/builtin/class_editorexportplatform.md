# EditorExportPlatform

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorExportPlatform**

> Base resource that provides the functionality of exporting a release build of a project to a platform, from the editor. Stores platform-specific metadata such as the name and supported features of the platform, and performs the exporting of projects, PCK files, and ZIP files. Uses an export template for the platform provided at the time of project exporting.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base resource that provides the functionality of exporting a release build of a project to a platform, from the editor. Stores platform-specific metadata such as the name and supported features of the platform, and performs the exporting of projects, PCK files, and ZIP files. Uses an export template for the platform provided at the time of project exporting.

Used in scripting by EditorExportPlugin to configure platform-specific customization of scenes and resources. See EditorExportPlugin._begin_customize_scenes() and EditorExportPlugin._begin_customize_resources() for more details.

</details>

## 方法

### add_message {#add_message}

```gdscript
void add_message(type: EditorExportPlatform.ExportMessageType, category: String, message: String)
```

Adds a message to the export log that will be displayed when exporting ends.

### clear_messages {#clear_messages}

```gdscript
void clear_messages()
```

Clears the export log.

### create_preset {#create_preset}

```gdscript
EditorExportPreset create_preset()
```

Create a new preset for this platform.

### export_pack {#export_pack}

```gdscript
Error export_pack(preset: EditorExportPreset, debug: bool, path: String, flags: bitfield::EditorExportPlatform.DebugFlags = 0)
```

Creates a PCK archive at path for the specified preset.

### export_pack_patch {#export_pack_patch}

```gdscript
Error export_pack_patch(preset: EditorExportPreset, debug: bool, path: String, patches: PackedStringArray = PackedStringArray(), flags: bitfield::EditorExportPlatform.DebugFlags = 0)
```

Creates a patch PCK archive at path for the specified preset, containing only the files that have changed since the last patch.

Note: patches is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead.

### export_project {#export_project}

```gdscript
Error export_project(preset: EditorExportPreset, debug: bool, path: String, flags: bitfield::EditorExportPlatform.DebugFlags = 0, notify: bool = true)
```

Creates a full project at path for the specified preset. If notify is true, plugins using EditorExportPlugin._export_begin() will be called during the process.

### export_project_files {#export_project_files}

```gdscript
Error export_project_files(preset: EditorExportPreset, debug: bool, save_cb: Callable, shared_cb: Callable = Callable())
```

Exports project files for the specified preset. This method can be used to implement custom export format, other than PCK and ZIP. One of the callbacks is called for each exported file.

save_cb is called for all exported files and have the following arguments: file_path: String, file_data: PackedByteArray, file_index: int, file_count: int, encryption_include_filters: PackedStringArray, encryption_exclude_filters: PackedStringArray, encryption_key: PackedByteArray.

shared_cb is called for exported native shared/static libraries and have the following arguments: file_path: String, tags: PackedStringArray, target_folder: String.

Note: file_index and file_count are intended for progress tracking only and aren't necessarily unique and precise.

### export_zip {#export_zip}

```gdscript
Error export_zip(preset: EditorExportPreset, debug: bool, path: String, flags: bitfield::EditorExportPlatform.DebugFlags = 0)
```

Create a ZIP archive at path for the specified preset.

### export_zip_patch {#export_zip_patch}

```gdscript
Error export_zip_patch(preset: EditorExportPreset, debug: bool, path: String, patches: PackedStringArray = PackedStringArray(), flags: bitfield::EditorExportPlatform.DebugFlags = 0)
```

Create a patch ZIP archive at path for the specified preset, containing only the files that have changed since the last patch.

Note: patches is an optional override of the set of patches defined in the export preset. When empty the patches defined in the export preset will be used instead.

### find_export_template {#find_export_template}

```gdscript
Dictionary find_export_template(template_file_name: String) const
```

Locates export template for the platform, and returns Dictionary with the following keys: path: String and error: String. This method is provided for convenience and custom export platforms aren't required to use it or keep export templates stored in the same way official templates are.

### gen_export_flags {#gen_export_flags}

```gdscript
PackedStringArray gen_export_flags(flags: bitfield::EditorExportPlatform.DebugFlags)
```

Generates array of command line arguments for the default export templates for the debug flags and editor settings.

### get_current_presets {#get_current_presets}

```gdscript
Array get_current_presets() const
```

Returns array of EditorExportPresets for this platform.

### get_forced_export_files {#get_forced_export_files}

```gdscript
static PackedStringArray get_forced_export_files(preset: EditorExportPreset = null)
```

Returns array of core file names that always should be exported regardless of preset config.

### get_internal_export_files {#get_internal_export_files}

```gdscript
Dictionary get_internal_export_files(preset: EditorExportPreset, debug: bool)
```

Returns additional files that should always be exported regardless of preset configuration, and are not part of the project source. The returned Dictionary contains filename keys (String) and their corresponding raw data (PackedByteArray).

### get_message_category {#get_message_category}

```gdscript
String get_message_category(index: int) const
```

Returns the message category for the message with the given index.

### get_message_count {#get_message_count}

```gdscript
int get_message_count() const
```

Returns the number of messages in the export log.

### get_message_text {#get_message_text}

```gdscript
String get_message_text(index: int) const
```

Returns the text for the message with the given index.

### get_message_type {#get_message_type}

```gdscript
EditorExportPlatform.ExportMessageType get_message_type(index: int) const
```

Returns the type for the message with the given index.

### get_os_name {#get_os_name}

```gdscript
String get_os_name() const
```

Returns the name of the export operating system handled by this EditorExportPlatform class, as a friendly string. Possible return values are Windows, Linux, macOS, Android, iOS, and Web.

### get_worst_message_type {#get_worst_message_type}

```gdscript
EditorExportPlatform.ExportMessageType get_worst_message_type() const
```

Returns most severe message type currently present in the export log.

### save_pack {#save_pack}

```gdscript
Dictionary save_pack(preset: EditorExportPreset, debug: bool, path: String, embed: bool = false)
```

Saves PCK archive and returns Dictionary with the following keys: result: Error, so_files: Array (array of the shared/static objects which contains dictionaries with the following keys: path: String, tags: PackedStringArray, and target_folder: String).

If embed is true, PCK content is appended to the end of path file and return Dictionary additionally include following keys: embedded_start: int (embedded PCK offset) and embedded_size: int (embedded PCK size).

### save_pack_patch {#save_pack_patch}

```gdscript
Dictionary save_pack_patch(preset: EditorExportPreset, debug: bool, path: String)
```

Saves patch PCK archive and returns Dictionary with the following keys: result: Error, so_files: Array (array of the shared/static objects which contains dictionaries with the following keys: path: String, tags: PackedStringArray, and target_folder: String).

### save_zip {#save_zip}

```gdscript
Dictionary save_zip(preset: EditorExportPreset, debug: bool, path: String)
```

Saves ZIP archive and returns Dictionary with the following keys: result: Error, so_files: Array (array of the shared/static objects which contains dictionaries with the following keys: path: String, tags: PackedStringArray, and target_folder: String).

### save_zip_patch {#save_zip_patch}

```gdscript
Dictionary save_zip_patch(preset: EditorExportPreset, debug: bool, path: String)
```

Saves patch ZIP archive and returns Dictionary with the following keys: result: Error, so_files: Array (array of the shared/static objects which contains dictionaries with the following keys: path: String, tags: PackedStringArray, and target_folder: String).

### ssh_push_to_remote {#ssh_push_to_remote}

```gdscript
Error ssh_push_to_remote(host: String, port: String, scp_args: PackedStringArray, src_file: String, dst_file: String) const
```

Uploads specified file over SCP protocol to the remote host.

### ssh_run_on_remote {#ssh_run_on_remote}

```gdscript
Error ssh_run_on_remote(host: String, port: String, ssh_arg: PackedStringArray, cmd_args: String, output: Array = [], port_fwd: int = -1) const
```

Executes specified command on the remote host via SSH protocol and returns command output in the output.

### ssh_run_on_remote_no_wait {#ssh_run_on_remote_no_wait}

```gdscript
int ssh_run_on_remote_no_wait(host: String, port: String, ssh_args: PackedStringArray, cmd_args: String, port_fwd: int = -1) const
```

Executes specified command on the remote host via SSH protocol and returns process ID (on the remote host) without waiting for command to finish.

## 枚举

### ExportMessageType {#enum-ExportMessageType}

```gdscript
enum ExportMessageType { EXPORT_MESSAGE_NONE = 0, EXPORT_MESSAGE_INFO = 1, EXPORT_MESSAGE_WARNING = 2, EXPORT_MESSAGE_ERROR = 3 }
```

ExportMessageType EXPORT_MESSAGE_NONE = 0

Invalid message type used as the default value when no type is specified.

ExportMessageType EXPORT_MESSAGE_INFO = 1

Message type for informational messages that have no effect on the export.

ExportMessageType EXPORT_MESSAGE_WARNING = 2

Message type for warning messages that should be addressed but still allow to complete the export.

ExportMessageType EXPORT_MESSAGE_ERROR = 3

Message type for error messages that must be addressed and fail the export.

#### EXPORT_MESSAGE_NONE {#EXPORT_MESSAGE_NONE}

```gdscript
EXPORT_MESSAGE_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPORT_MESSAGE_INFO {#EXPORT_MESSAGE_INFO}

```gdscript
EXPORT_MESSAGE_INFO = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPORT_MESSAGE_WARNING {#EXPORT_MESSAGE_WARNING}

```gdscript
EXPORT_MESSAGE_WARNING = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EXPORT_MESSAGE_ERROR {#EXPORT_MESSAGE_ERROR}

```gdscript
EXPORT_MESSAGE_ERROR = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DebugFlags {#enum-DebugFlags}

```gdscript
enum DebugFlags（位标志） { DEBUG_FLAG_DUMB_CLIENT = 1, DEBUG_FLAG_REMOTE_DEBUG = 2, DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST = 4, DEBUG_FLAG_VIEW_COLLISIONS = 8, DEBUG_FLAG_VIEW_NAVIGATION = 16 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_FLAG_DUMB_CLIENT {#DEBUG_FLAG_DUMB_CLIENT}

```gdscript
DEBUG_FLAG_DUMB_CLIENT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_FLAG_REMOTE_DEBUG {#DEBUG_FLAG_REMOTE_DEBUG}

```gdscript
DEBUG_FLAG_REMOTE_DEBUG = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST {#DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST}

```gdscript
DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_FLAG_VIEW_COLLISIONS {#DEBUG_FLAG_VIEW_COLLISIONS}

```gdscript
DEBUG_FLAG_VIEW_COLLISIONS = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_FLAG_VIEW_NAVIGATION {#DEBUG_FLAG_VIEW_NAVIGATION}

```gdscript
DEBUG_FLAG_VIEW_NAVIGATION = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
