# EditorSceneFormatImporter

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorSceneFormatImporter**

> EditorSceneFormatImporter allows to define an importer script for a third-party 3D format.

<details><summary>官方英文描述（中文精译进行中）</summary>

EditorSceneFormatImporter allows to define an importer script for a third-party 3D format.

To use EditorSceneFormatImporter, register it using the EditorPlugin.add_scene_format_importer_plugin() method first.

</details>

## 方法

### _get_extensions {#_get_extensions}

```gdscript
PackedStringArray _get_extensions() virtual const
```

Return supported file extensions for this scene importer.

### _get_import_options {#_get_import_options}

```gdscript
void _get_import_options(path: String) virtual
```

Override to add general import options. These will appear in the main import dock on the editor. Add options via add_import_option() and add_import_option_advanced().

Note: All EditorSceneFormatImporter and EditorScenePostImportPlugin instances will add options for all files. It is good practice to check the file extension when path is non-empty.

When the user is editing project settings, path will be empty. It is recommended to add all options when path is empty to allow the user to customize Import Defaults.

### _get_option_visibility {#_get_option_visibility}

```gdscript
Variant _get_option_visibility(path: String, for_animation: bool, option: String) virtual const
```

Should return true to show the given option, false to hide the given option, or null to ignore.

### _import_scene {#_import_scene}

```gdscript
Object _import_scene(path: String, flags: int, options: Dictionary) virtual
```

Perform the bulk of the scene import logic here, for example using GLTFDocument or FBXDocument.

### add_import_option {#add_import_option}

```gdscript
void add_import_option(name: String, value: Variant)
```

Add a specific import option (name and default value only). This function can only be called from _get_import_options().

### add_import_option_advanced {#add_import_option_advanced}

```gdscript
void add_import_option_advanced(type: Variant.Type, name: String, default_value: Variant, hint: PropertyHint = 0, hint_string: String = "", usage_flags: int = 6)
```

Add a specific import option. This function can only be called from _get_import_options().

## 枚举

### ImportFlags {#enum-ImportFlags}

```gdscript
enum ImportFlags（位标志） { IMPORT_SCENE = 1, IMPORT_ANIMATION = 2, IMPORT_FAIL_ON_MISSING_DEPENDENCIES = 4, IMPORT_GENERATE_TANGENT_ARRAYS = 8, IMPORT_USE_NAMED_SKIN_BINDS = 16, IMPORT_DISCARD_MESHES_AND_MATERIALS = 32, IMPORT_FORCE_DISABLE_MESH_COMPRESSION = 64 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_SCENE {#IMPORT_SCENE}

```gdscript
IMPORT_SCENE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_ANIMATION {#IMPORT_ANIMATION}

```gdscript
IMPORT_ANIMATION = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_FAIL_ON_MISSING_DEPENDENCIES {#IMPORT_FAIL_ON_MISSING_DEPENDENCIES}

```gdscript
IMPORT_FAIL_ON_MISSING_DEPENDENCIES = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_GENERATE_TANGENT_ARRAYS {#IMPORT_GENERATE_TANGENT_ARRAYS}

```gdscript
IMPORT_GENERATE_TANGENT_ARRAYS = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_USE_NAMED_SKIN_BINDS {#IMPORT_USE_NAMED_SKIN_BINDS}

```gdscript
IMPORT_USE_NAMED_SKIN_BINDS = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_DISCARD_MESHES_AND_MATERIALS {#IMPORT_DISCARD_MESHES_AND_MATERIALS}

```gdscript
IMPORT_DISCARD_MESHES_AND_MATERIALS = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IMPORT_FORCE_DISABLE_MESH_COMPRESSION {#IMPORT_FORCE_DISABLE_MESH_COMPRESSION}

```gdscript
IMPORT_FORCE_DISABLE_MESH_COMPRESSION = 64
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
