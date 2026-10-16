# FileDialog

**继承**：[ConfirmationDialog](class_confirmationdialog.md) → [AcceptDialog](class_acceptdialog.md) → [Window](class_window.md) → [Viewport](class_viewport.md) → [Node](class_node.md) → [Object](class_object.md) → **FileDialog**

> FileDialog is a preset dialog used to choose files and directories in the filesystem. It supports filter masks. FileDialog automatically sets its window title according to the file_mode. If you want to use a custom title, disable this by setting mode_overrides_title to false.

<details><summary>官方英文描述（中文精译进行中）</summary>

FileDialog is a preset dialog used to choose files and directories in the filesystem. It supports filter masks. FileDialog automatically sets its window title according to the file_mode. If you want to use a custom title, disable this by setting mode_overrides_title to false.

Note: FileDialog is invisible by default. To make it visible, call one of the popup_* methods from Window on the node, such as Window.popup_centered_clamped().

</details>

## 方法

### add_filter {#add_filter}

```gdscript
void add_filter(filter: String, description: String = "", mime_type: String = "")
```

Adds a comma-separated file extension filter and comma-separated MIME type mime_type option to the FileDialog with an optional description, which restricts what files can be picked.

A filter should be of the form "filename.extension", where filename and extension can be * to match any string. Filters starting with . (i.e. empty filenames) are not allowed.

For example, a filter of "*.png, *.jpg", a mime_type of image/png, image/jpeg, and a description of "Images" results in filter text "Images (*.png, *.jpg)".

Note: Embedded file dialogs and Windows file dialogs support only file extensions, while Android, Linux, and macOS file dialogs also support MIME types.

### add_option {#add_option}

```gdscript
void add_option(name: String, values: PackedStringArray, default_value_index: int)
```

Adds an additional OptionButton to the file dialog. If values is empty, a CheckBox is added instead.

default_value_index should be an index of the value in the values. If values is empty it should be either 1 (checked), or 0 (unchecked).

### clear_filename_filter {#clear_filename_filter}

```gdscript
void clear_filename_filter()
```

Clear the filter for file names.

### clear_filters {#clear_filters}

```gdscript
void clear_filters()
```

Clear all the added filters in the dialog.

### deselect_all {#deselect_all}

```gdscript
void deselect_all()
```

Clear all currently selected items in the dialog.

### get_access {#get_access}

```gdscript
FileDialog.Access get_access() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_current_dir {#get_current_dir}

```gdscript
String get_current_dir() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_current_file {#get_current_file}

```gdscript
String get_current_file() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_current_path {#get_current_path}

```gdscript
String get_current_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_display_mode {#get_display_mode}

```gdscript
FileDialog.DisplayMode get_display_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_favorite_list {#get_favorite_list}

```gdscript
static PackedStringArray get_favorite_list()
```

Returns the list of favorite directories, which is shared by all FileDialog nodes. Useful to store the list of favorites between project sessions. This method can be called only from the main thread.

### get_file_mode {#get_file_mode}

```gdscript
FileDialog.FileMode get_file_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filename_filter {#get_filename_filter}

```gdscript
String get_filename_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_filters {#get_filters}

```gdscript
PackedStringArray get_filters() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_line_edit {#get_line_edit}

```gdscript
LineEdit get_line_edit()
```

Returns the LineEdit for the selected file.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

### get_option_count {#get_option_count}

```gdscript
int get_option_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_option_default {#get_option_default}

```gdscript
int get_option_default(option: int) const
```

Returns the default value index of the OptionButton or CheckBox with index option.

### get_option_name {#get_option_name}

```gdscript
String get_option_name(option: int) const
```

Returns the name of the OptionButton or CheckBox with index option.

### get_option_values {#get_option_values}

```gdscript
PackedStringArray get_option_values(option: int) const
```

Returns an array of values of the OptionButton with index option.

### get_recent_list {#get_recent_list}

```gdscript
static PackedStringArray get_recent_list()
```

Returns the list of recent directories, which is shared by all FileDialog nodes. Useful to store the list of recents between project sessions. This method can be called only from the main thread.

### get_root_subfolder {#get_root_subfolder}

```gdscript
String get_root_subfolder() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_selected_options {#get_selected_options}

```gdscript
Dictionary get_selected_options() const
```

Returns a Dictionary with the selected values of the additional OptionButtons and/or CheckBoxes. Dictionary keys are names and values are selected value indices.

### get_use_native_dialog {#get_use_native_dialog}

```gdscript
bool get_use_native_dialog() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vbox {#get_vbox}

```gdscript
VBoxContainer get_vbox()
```

Returns the vertical box container of the dialog, custom controls can be added to it.

Warning: This is a required internal node, removing and freeing it may cause a crash. If you wish to hide it or any of its children, use their CanvasItem.visible property.

Note: Changes to this node are ignored by native file dialogs, use add_option() to add custom elements to the dialog instead.

### invalidate {#invalidate}

```gdscript
void invalidate()
```

Invalidates and updates this dialog's content list.

Note: This method does nothing on native file dialogs.

### is_customization_flag_enabled {#is_customization_flag_enabled}

```gdscript
bool is_customization_flag_enabled(flag: FileDialog.Customization) const
```

Returns true if the provided flag is enabled.

### is_mode_overriding_title {#is_mode_overriding_title}

```gdscript
bool is_mode_overriding_title() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_showing_hidden_files {#is_showing_hidden_files}

```gdscript
bool is_showing_hidden_files() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### popup_file_dialog {#popup_file_dialog}

```gdscript
void popup_file_dialog()
```

Shows the FileDialog using the default size and position for file dialogs, and selects the file name if there is a current file.

### set_access {#set_access}

```gdscript
void set_access(access: FileDialog.Access)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_current_dir {#set_current_dir}

```gdscript
void set_current_dir(dir: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_current_file {#set_current_file}

```gdscript
void set_current_file(file: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_current_path {#set_current_path}

```gdscript
void set_current_path(path: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_customization_flag_enabled {#set_customization_flag_enabled}

```gdscript
void set_customization_flag_enabled(flag: FileDialog.Customization, enabled: bool)
```

Sets the specified customization flag, allowing to customize the features available in this FileDialog.

### set_display_mode {#set_display_mode}

```gdscript
void set_display_mode(mode: FileDialog.DisplayMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_favorite_list {#set_favorite_list}

```gdscript
static void set_favorite_list(favorites: PackedStringArray)
```

Sets the list of favorite directories, which is shared by all FileDialog nodes. Useful to restore the list of favorites saved with get_favorite_list(). This method can be called only from the main thread.

Note: FileDialog will update its internal ItemList of favorites when its visibility changes. Be sure to call this method earlier if you want your changes to have effect.

### set_file_mode {#set_file_mode}

```gdscript
void set_file_mode(mode: FileDialog.FileMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filename_filter {#set_filename_filter}

```gdscript
void set_filename_filter(filter: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filters {#set_filters}

```gdscript
void set_filters(filters: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_get_icon_callback {#set_get_icon_callback}

```gdscript
static void set_get_icon_callback(callback: Callable)
```

Sets the callback used by the FileDialog nodes to get a file icon, when DISPLAY_LIST mode is used. The callback should take a single String argument (file path), and return a Texture2D. If an invalid texture is returned, the file icon will be used instead.

### set_get_thumbnail_callback {#set_get_thumbnail_callback}

```gdscript
static void set_get_thumbnail_callback(callback: Callable)
```

Sets the callback used by the FileDialog nodes to get a file icon, when DISPLAY_THUMBNAILS mode is used. The callback should take a single String argument (file path), and return a Texture2D. If an invalid texture is returned, the file_thumbnail icon will be used instead.

Thumbnails are usually more complex and may take a while to load. To avoid stalling the application, you can use ImageTexture to asynchronously create the thumbnail.

```gdscript
func _ready():
    FileDialog.set_get_thumbnail_callback(thumbnail_method)
func thumbnail_method(path):
    var image_texture = ImageTexture.new()
    make_thumbnail_async(path, image_texture)
    return image_texture
func make_thumbnail_async(path, image_texture):
    var thumbnail_texture = await generate_thumbnail(path) # Some method that generates a thumbnail.
    image_texture.set_image(thumbnail_texture.get_image())
```

### set_mode_overrides_title {#set_mode_overrides_title}

```gdscript
void set_mode_overrides_title(override: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_option_count {#set_option_count}

```gdscript
void set_option_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_option_default {#set_option_default}

```gdscript
void set_option_default(option: int, default_value_index: int)
```

Sets the default value index of the OptionButton or CheckBox with index option.

### set_option_name {#set_option_name}

```gdscript
void set_option_name(option: int, name: String)
```

Sets the name of the OptionButton or CheckBox with index option.

### set_option_values {#set_option_values}

```gdscript
void set_option_values(option: int, values: PackedStringArray)
```

Sets the option values of the OptionButton with index option.

### set_recent_list {#set_recent_list}

```gdscript
static void set_recent_list(recents: PackedStringArray)
```

Sets the list of recent directories, which is shared by all FileDialog nodes. Useful to restore the list of recents saved with set_recent_list(). This method can be called only from the main thread.

Note: FileDialog will update its internal ItemList of recent directories when its visibility changes. Be sure to call this method earlier if you want your changes to have effect.

### set_root_subfolder {#set_root_subfolder}

```gdscript
void set_root_subfolder(dir: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_show_hidden_files {#set_show_hidden_files}

```gdscript
void set_show_hidden_files(show: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_native_dialog {#set_use_native_dialog}

```gdscript
void set_use_native_dialog(native: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mode_overrides_title {#mode_overrides_title}

```gdscript
mode_overrides_title: bool
```

- void set_mode_overrides_title(value: bool)
- bool is_mode_overriding_title()

If true, changing the file_mode property will set the window title accordingly (e.g. setting file_mode to FILE_MODE_OPEN_FILE will change the window title to "Open a File").

### file_mode {#file_mode}

```gdscript
file_mode: int
```

- void set_file_mode(value: FileMode)
- FileMode get_file_mode()

The dialog's open or save mode, which affects the selection behavior.

### display_mode {#display_mode}

```gdscript
display_mode: int
```

- void set_display_mode(value: DisplayMode)
- DisplayMode get_display_mode()

Display mode of the dialog's file list.

### access {#access}

```gdscript
access: int
```

- void set_access(value: Access)
- Access get_access()

The file system access scope.

Warning: In Web builds, FileDialog cannot access the host file system. In sandboxed Linux and macOS environments, use_native_dialog is automatically used to allow limited access to host file system.

### root_subfolder {#root_subfolder}

```gdscript
root_subfolder: String
```

- void set_root_subfolder(value: String)
- String get_root_subfolder()

If non-empty, the given sub-folder will be "root" of this FileDialog, i.e. user won't be able to go to its parent directory.

Note: This property is ignored by native file dialogs.

### filters {#filters}

```gdscript
filters: PackedStringArray
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### filename_filter {#filename_filter}

```gdscript
filename_filter: String
```

- void set_filename_filter(value: String)
- String get_filename_filter()

The filter for file names (case-insensitive). When set to a non-empty string, only files that contains the substring will be shown. filename_filter can be edited by the user with the filter button at the top of the file dialog.

See also filters, which should be used to restrict the file types that can be selected instead of filename_filter which is meant to be set by the user.

### show_hidden_files {#show_hidden_files}

```gdscript
show_hidden_files: bool
```

- void set_show_hidden_files(value: bool)
- bool is_showing_hidden_files()

If true, the dialog will show hidden files.

Note: This property is ignored by native file dialogs on Android and Linux.

### use_native_dialog {#use_native_dialog}

```gdscript
use_native_dialog: bool
```

- void set_use_native_dialog(value: bool)
- bool get_use_native_dialog()

If true, and if supported by the current DisplayServer, OS native dialog will be used instead of custom one.

Note: On Android, it is only supported when using ACCESS_FILESYSTEM. For access mode ACCESS_RESOURCES and ACCESS_USERDATA, the system will fall back to custom FileDialog.

Note: On Linux and macOS, sandboxed apps always use native dialogs to access the host file system.

Note: On macOS, sandboxed apps will save security-scoped bookmarks to retain access to the opened folders across multiple sessions. Use OS.get_granted_permissions() to get a list of saved bookmarks.

Note: Native dialogs are isolated from the base process, file dialog properties can't be modified once the dialog is shown.

Note: This property is ignored in EditorFileDialog.

### option_count {#option_count}

```gdscript
option_count: int
```

- void set_option_count(value: int)
- int get_option_count()

The number of additional OptionButtons and CheckBoxes in the dialog.

### hidden_files_toggle_enabled {#hidden_files_toggle_enabled}

```gdscript
hidden_files_toggle_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, shows the toggle hidden files button.

### file_filter_toggle_enabled {#file_filter_toggle_enabled}

```gdscript
file_filter_toggle_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, shows the toggle file filter button.

### file_sort_options_enabled {#file_sort_options_enabled}

```gdscript
file_sort_options_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, shows the file sorting options button.

### folder_creation_enabled {#folder_creation_enabled}

```gdscript
folder_creation_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, shows the button for creating new directories (when using FILE_MODE_OPEN_DIR, FILE_MODE_OPEN_ANY, or FILE_MODE_SAVE_FILE), and the context menu will have the "New Folder..." option.

### favorites_enabled {#favorites_enabled}

```gdscript
favorites_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, shows the toggle favorite button and favorite list on the left side of the dialog.

### recent_list_enabled {#recent_list_enabled}

```gdscript
recent_list_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, shows the recent directories list on the left side of the dialog.

### layout_toggle_enabled {#layout_toggle_enabled}

```gdscript
layout_toggle_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, shows the layout switch buttons (list/thumbnails).

### overwrite_warning_enabled {#overwrite_warning_enabled}

```gdscript
overwrite_warning_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, the FileDialog will warn the user before overwriting files in save mode.

### deleting_enabled {#deleting_enabled}

```gdscript
deleting_enabled: bool
```

- void set_customization_flag_enabled(flag: Customization, enabled: bool)
- bool is_customization_flag_enabled(flag: Customization) const

If true, the context menu will show the "Delete" option, which allows moving files and folders to trash.

### current_dir {#current_dir}

```gdscript
current_dir: String
```

- void set_current_dir(value: String)
- String get_current_dir()

The current working directory of the file dialog.

Note: For native file dialogs, this property is only treated as a hint and may not be respected by specific OS implementations.

### current_file {#current_file}

```gdscript
current_file: String
```

- void set_current_file(value: String)
- String get_current_file()

The currently selected file of the file dialog.

### current_path {#current_path}

```gdscript
current_path: String
```

- void set_current_path(value: String)
- String get_current_path()

The currently selected file path of the file dialog.

## 信号

### file_selected {#file_selected}

```gdscript
signal file_selected(path: String)
```

Emitted when the user selects a file by double-clicking it or pressing the OK button.

### files_selected {#files_selected}

```gdscript
signal files_selected(paths: PackedStringArray)
```

Emitted when the user selects multiple files.

### dir_selected {#dir_selected}

```gdscript
signal dir_selected(dir: String)
```

Emitted when the user selects a directory.

### filename_filter_changed {#filename_filter_changed}

```gdscript
signal filename_filter_changed(filter: String)
```

Emitted when the filter for file names changes.

## 枚举

### FileMode {#enum-FileMode}

```gdscript
enum FileMode { FILE_MODE_OPEN_FILE = 0, FILE_MODE_OPEN_FILES = 1, FILE_MODE_OPEN_DIR = 2, FILE_MODE_OPEN_ANY = 3, FILE_MODE_SAVE_FILE = 4 }
```

FileMode FILE_MODE_OPEN_FILE = 0

The dialog allows selecting one, and only one file.

FileMode FILE_MODE_OPEN_FILES = 1

The dialog allows selecting multiple files.

FileMode FILE_MODE_OPEN_DIR = 2

The dialog only allows selecting a directory, disallowing the selection of any file.

FileMode FILE_MODE_OPEN_ANY = 3

The dialog allows selecting one file or directory.

FileMode FILE_MODE_SAVE_FILE = 4

The dialog will warn when a file exists.

#### FILE_MODE_OPEN_FILE {#FILE_MODE_OPEN_FILE}

```gdscript
FILE_MODE_OPEN_FILE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILE_MODE_OPEN_FILES {#FILE_MODE_OPEN_FILES}

```gdscript
FILE_MODE_OPEN_FILES = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILE_MODE_OPEN_DIR {#FILE_MODE_OPEN_DIR}

```gdscript
FILE_MODE_OPEN_DIR = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILE_MODE_OPEN_ANY {#FILE_MODE_OPEN_ANY}

```gdscript
FILE_MODE_OPEN_ANY = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILE_MODE_SAVE_FILE {#FILE_MODE_SAVE_FILE}

```gdscript
FILE_MODE_SAVE_FILE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Access {#enum-Access}

```gdscript
enum Access { ACCESS_RESOURCES = 0, ACCESS_USERDATA = 1, ACCESS_FILESYSTEM = 2 }
```

Access ACCESS_RESOURCES = 0

The dialog only allows accessing files under the Resource path (res://).

Access ACCESS_USERDATA = 1

The dialog only allows accessing files under user data path (user://).

Access ACCESS_FILESYSTEM = 2

The dialog allows accessing files on the whole file system.

#### ACCESS_RESOURCES {#ACCESS_RESOURCES}

```gdscript
ACCESS_RESOURCES = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ACCESS_USERDATA {#ACCESS_USERDATA}

```gdscript
ACCESS_USERDATA = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ACCESS_FILESYSTEM {#ACCESS_FILESYSTEM}

```gdscript
ACCESS_FILESYSTEM = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DisplayMode {#enum-DisplayMode}

```gdscript
enum DisplayMode { DISPLAY_THUMBNAILS = 0, DISPLAY_LIST = 1 }
```

DisplayMode DISPLAY_THUMBNAILS = 0

The dialog displays files as a grid of thumbnails. Use thumbnail_size to adjust their size.

DisplayMode DISPLAY_LIST = 1

The dialog displays files as a list of filenames.

#### DISPLAY_THUMBNAILS {#DISPLAY_THUMBNAILS}

```gdscript
DISPLAY_THUMBNAILS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DISPLAY_LIST {#DISPLAY_LIST}

```gdscript
DISPLAY_LIST = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### Customization {#enum-Customization}

```gdscript
enum Customization { CUSTOMIZATION_HIDDEN_FILES = 0, CUSTOMIZATION_CREATE_FOLDER = 1, CUSTOMIZATION_FILE_FILTER = 2, CUSTOMIZATION_FILE_SORT = 3, CUSTOMIZATION_FAVORITES = 4, CUSTOMIZATION_RECENT = 5, CUSTOMIZATION_LAYOUT = 6, CUSTOMIZATION_OVERWRITE_WARNING = 7, CUSTOMIZATION_DELETE = 8 }
```

Customization CUSTOMIZATION_HIDDEN_FILES = 0

Toggles visibility of the favorite button, and the favorite list on the left side of the dialog.

Equivalent to hidden_files_toggle_enabled.

Customization CUSTOMIZATION_CREATE_FOLDER = 1

If enabled, shows the button for creating new directories (when using FILE_MODE_OPEN_DIR, FILE_MODE_OPEN_ANY, or FILE_MODE_SAVE_FILE).

Equivalent to folder_creation_enabled.

Customization CUSTOMIZATION_FILE_FILTER = 2

If enabled, shows the toggle file filter button.

Equivalent to file_filter_toggle_enabled.

Customization CUSTOMIZATION_FILE_SORT = 3

If enabled, shows the file sorting options button.

Equivalent to file_sort_options_enabled.

Customization CUSTOMIZATION_FAVORITES = 4

If enabled, shows the toggle favorite button and favorite list on the left side of the dialog.

Equivalent to favorites_enabled.

Customization CUSTOMIZATION_RECENT = 5

If enabled, shows the recent directories list on the left side of the dialog.

Equivalent to recent_list_enabled.

Customization CUSTOMIZATION_LAYOUT = 6

If enabled, shows the layout switch buttons (list/thumbnails).

Equivalent to layout_toggle_enabled.

Customization CUSTOMIZATION_OVERWRITE_WARNING = 7

If enabled, the FileDialog will warn the user before overwriting files in save mode.

Equivalent to overwrite_warning_enabled.

Customization CUSTOMIZATION_DELETE = 8

If enabled, the context menu will show the "Delete" option, which allows moving files and folders to trash.

Equivalent to deleting_enabled.

#### CUSTOMIZATION_HIDDEN_FILES {#CUSTOMIZATION_HIDDEN_FILES}

```gdscript
CUSTOMIZATION_HIDDEN_FILES = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_CREATE_FOLDER {#CUSTOMIZATION_CREATE_FOLDER}

```gdscript
CUSTOMIZATION_CREATE_FOLDER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_FILE_FILTER {#CUSTOMIZATION_FILE_FILTER}

```gdscript
CUSTOMIZATION_FILE_FILTER = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_FILE_SORT {#CUSTOMIZATION_FILE_SORT}

```gdscript
CUSTOMIZATION_FILE_SORT = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_FAVORITES {#CUSTOMIZATION_FAVORITES}

```gdscript
CUSTOMIZATION_FAVORITES = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_RECENT {#CUSTOMIZATION_RECENT}

```gdscript
CUSTOMIZATION_RECENT = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_LAYOUT {#CUSTOMIZATION_LAYOUT}

```gdscript
CUSTOMIZATION_LAYOUT = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_OVERWRITE_WARNING {#CUSTOMIZATION_OVERWRITE_WARNING}

```gdscript
CUSTOMIZATION_OVERWRITE_WARNING = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUSTOMIZATION_DELETE {#CUSTOMIZATION_DELETE}

```gdscript
CUSTOMIZATION_DELETE = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
