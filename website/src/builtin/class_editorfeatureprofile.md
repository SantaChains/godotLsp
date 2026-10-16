# EditorFeatureProfile

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorFeatureProfile**

> An editor feature profile can be used to disable specific features of the Godot editor. When disabled, the features won't appear in the editor, which makes the editor less cluttered. This is useful in education settings to reduce confusion or when working in a team. For example, artists and level designers could use a feature profile that disables the script editor to avoid accidentally making changes to files they aren't supposed to edit.

<details><summary>官方英文描述</summary>

An editor feature profile can be used to disable specific features of the Godot editor. When disabled, the features won't appear in the editor, which makes the editor less cluttered. This is useful in education settings to reduce confusion or when working in a team. For example, artists and level designers could use a feature profile that disables the script editor to avoid accidentally making changes to files they aren't supposed to edit.

To manage editor feature profiles visually, use Editor > Manage Feature Profiles... at the top of the editor window.

</details>

## 方法

### get_feature_name {#get_feature_name}

```gdscript
String get_feature_name(feature: EditorFeatureProfile.Feature)
```

Returns the specified feature's human-readable name.

### is_class_disabled {#is_class_disabled}

```gdscript
bool is_class_disabled(class_name: StringName) const
```

Returns true if the class specified by class_name is disabled. When disabled, the class won't appear in the Create New Node dialog.

### is_class_editor_disabled {#is_class_editor_disabled}

```gdscript
bool is_class_editor_disabled(class_name: StringName) const
```

Returns true if editing for the class specified by class_name is disabled. When disabled, the class will still appear in the Create New Node dialog but the Inspector will be read-only when selecting a node that extends the class.

### is_class_property_disabled {#is_class_property_disabled}

```gdscript
bool is_class_property_disabled(class_name: StringName, property: StringName) const
```

Returns true if property is disabled in the class specified by class_name. When a property is disabled, it won't appear in the Inspector when selecting a node that extends the class specified by class_name.

### is_feature_disabled {#is_feature_disabled}

```gdscript
bool is_feature_disabled(feature: EditorFeatureProfile.Feature) const
```

Returns true if the feature is disabled. When a feature is disabled, it will disappear from the editor entirely.

### load_from_file {#load_from_file}

```gdscript
Error load_from_file(path: String)
```

Loads an editor feature profile from a file. The file must follow the JSON format obtained by using the feature profile manager's Export button or the save_to_file() method.

Note: Feature profiles created via the user interface are loaded from the feature_profiles directory, as a file with the .profile extension. The editor configuration folder can be found by using EditorPaths.get_config_dir().

### save_to_file {#save_to_file}

```gdscript
Error save_to_file(path: String)
```

Saves the editor feature profile to a file in JSON format. It can then be imported using the feature profile manager's Import button or the load_from_file() method.

Note: Feature profiles created via the user interface are saved in the feature_profiles directory, as a file with the .profile extension. The editor configuration folder can be found by using EditorPaths.get_config_dir().

### set_disable_class {#set_disable_class}

```gdscript
void set_disable_class(class_name: StringName, disable: bool)
```

If disable is true, disables the class specified by class_name. When disabled, the class won't appear in the Create New Node dialog.

### set_disable_class_editor {#set_disable_class_editor}

```gdscript
void set_disable_class_editor(class_name: StringName, disable: bool)
```

If disable is true, disables editing for the class specified by class_name. When disabled, the class will still appear in the Create New Node dialog but the Inspector will be read-only when selecting a node that extends the class.

### set_disable_class_property {#set_disable_class_property}

```gdscript
void set_disable_class_property(class_name: StringName, property: StringName, disable: bool)
```

If disable is true, disables editing for property in the class specified by class_name. When a property is disabled, it won't appear in the Inspector when selecting a node that extends the class specified by class_name.

### set_disable_feature {#set_disable_feature}

```gdscript
void set_disable_feature(feature: EditorFeatureProfile.Feature, disable: bool)
```

If disable is true, disables the editor feature specified in feature. When a feature is disabled, it will disappear from the editor entirely.

## 枚举

### Feature {#enum-Feature}

```gdscript
enum Feature { FEATURE_3D = 0, FEATURE_SCRIPT = 1, FEATURE_ASSET_LIB = 2, FEATURE_SCENE_TREE = 3, FEATURE_NODE_DOCK = 4, FEATURE_FILESYSTEM_DOCK = 5, FEATURE_IMPORT_DOCK = 6, FEATURE_HISTORY_DOCK = 7, FEATURE_GAME = 8, FEATURE_SIGNALS_DOCK = 9, FEATURE_GROUPS_DOCK = 10, FEATURE_MAX = 11 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEATURE_3D {#FEATURE_3D}

```gdscript
FEATURE_3D = 0
```

The 3D editor. If this feature is disabled, the 3D editor won't display but 3D nodes will still display in the Create New Node dialog.

#### FEATURE_SCRIPT {#FEATURE_SCRIPT}

```gdscript
FEATURE_SCRIPT = 1
```

The Script tab, which contains the script editor and class reference browser. If this feature is disabled, the Script tab won't display.

#### FEATURE_ASSET_LIB {#FEATURE_ASSET_LIB}

```gdscript
FEATURE_ASSET_LIB = 2
```

The Asset Store tab. If this feature is disabled, the Asset Store tab won't display.

#### FEATURE_SCENE_TREE {#FEATURE_SCENE_TREE}

```gdscript
FEATURE_SCENE_TREE = 3
```

Scene tree editing. If this feature is disabled, the Scene tree dock will still be visible but will be read-only.

#### FEATURE_NODE_DOCK {#FEATURE_NODE_DOCK}

```gdscript
FEATURE_NODE_DOCK = 4
```

Deprecated: Replaced by the Signals and Groups docks.

The Node dock. If this feature is disabled, signals and groups won't be visible and modifiable from the editor.

#### FEATURE_FILESYSTEM_DOCK {#FEATURE_FILESYSTEM_DOCK}

```gdscript
FEATURE_FILESYSTEM_DOCK = 5
```

The FileSystem dock. If this feature is disabled, the FileSystem dock won't be visible.

#### FEATURE_IMPORT_DOCK {#FEATURE_IMPORT_DOCK}

```gdscript
FEATURE_IMPORT_DOCK = 6
```

The Import dock. If this feature is disabled, the Import dock won't be visible.

#### FEATURE_HISTORY_DOCK {#FEATURE_HISTORY_DOCK}

```gdscript
FEATURE_HISTORY_DOCK = 7
```

The History dock. If this feature is disabled, the History dock won't be visible.

#### FEATURE_GAME {#FEATURE_GAME}

```gdscript
FEATURE_GAME = 8
```

The Game tab, which allows embedding the game window and selecting nodes by clicking inside of it. If this feature is disabled, the Game tab won't display.

#### FEATURE_SIGNALS_DOCK {#FEATURE_SIGNALS_DOCK}

```gdscript
FEATURE_SIGNALS_DOCK = 9
```

The Signals dock. If this feature is disabled, signals won't be visible and modifiable from the editor.

#### FEATURE_GROUPS_DOCK {#FEATURE_GROUPS_DOCK}

```gdscript
FEATURE_GROUPS_DOCK = 10
```

The Groups dock. If this feature is disabled, groups won't be visible and modifiable from the editor.

#### FEATURE_MAX {#FEATURE_MAX}

```gdscript
FEATURE_MAX = 11
```

Represents the size of the Feature enum.

---
