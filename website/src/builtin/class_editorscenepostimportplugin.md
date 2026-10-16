# EditorScenePostImportPlugin

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorScenePostImportPlugin**

> This plugin type exists to modify the process of importing scenes, allowing to change the content as well as add importer options at every stage of the process.

<details><summary>官方英文描述（中文精译进行中）</summary>

This plugin type exists to modify the process of importing scenes, allowing to change the content as well as add importer options at every stage of the process.

</details>

## 方法

### _get_import_options {#_get_import_options}

```gdscript
void _get_import_options(path: String) virtual
```

Override to add general import options. These will appear in the main import dock on the editor. Add options via add_import_option() and add_import_option_advanced().

### _get_internal_import_options {#_get_internal_import_options}

```gdscript
void _get_internal_import_options(category: int) virtual
```

Override to add internal import options. These will appear in the 3D scene import dialog. Add options via add_import_option() and add_import_option_advanced().

### _get_internal_option_update_view_required {#_get_internal_option_update_view_required}

```gdscript
Variant _get_internal_option_update_view_required(category: int, option: String) virtual const
```

Should return true if the 3D view of the import dialog needs to update when changing the given option.

### _get_internal_option_visibility {#_get_internal_option_visibility}

```gdscript
Variant _get_internal_option_visibility(category: int, for_animation: bool, option: String) virtual const
```

Should return true to show the given option, false to hide the given option, or null to ignore.

### _get_option_visibility {#_get_option_visibility}

```gdscript
Variant _get_option_visibility(path: String, for_animation: bool, option: String) virtual const
```

Should return true to show the given option, false to hide the given option, or null to ignore.

### _internal_process {#_internal_process}

```gdscript
void _internal_process(category: int, base_node: Node, node: Node, resource: Resource) virtual
```

Process a specific node or resource for a given category.

### _post_process {#_post_process}

```gdscript
void _post_process(scene: Node) virtual
```

Post-process the scene. This function is called after the final scene has been configured.

### _pre_process {#_pre_process}

```gdscript
void _pre_process(scene: Node) virtual
```

Pre-process the scene. This function is called right after the scene format loader loaded the scene and no changes have been made.

Pre-process may be used to adjust internal import options in the "nodes", "meshes", "animations" or "materials" keys inside get_option_value("_subresources").

### add_import_option {#add_import_option}

```gdscript
void add_import_option(name: String, value: Variant)
```

Add a specific import option (name and default value only). This function can only be called from _get_import_options() and _get_internal_import_options().

### add_import_option_advanced {#add_import_option_advanced}

```gdscript
void add_import_option_advanced(type: Variant.Type, name: String, default_value: Variant, hint: PropertyHint = 0, hint_string: String = "", usage_flags: int = 6)
```

Add a specific import option. This function can only be called from _get_import_options() and _get_internal_import_options().

### get_option_value {#get_option_value}

```gdscript
Variant get_option_value(name: StringName) const
```

Query the value of an option. This function can only be called from those querying visibility, or processing.

## 枚举

### InternalImportCategory {#enum-InternalImportCategory}

```gdscript
enum InternalImportCategory { INTERNAL_IMPORT_CATEGORY_NODE = 0, INTERNAL_IMPORT_CATEGORY_MESH_3D_NODE = 1, INTERNAL_IMPORT_CATEGORY_MESH = 2, INTERNAL_IMPORT_CATEGORY_MATERIAL = 3, INTERNAL_IMPORT_CATEGORY_ANIMATION = 4, INTERNAL_IMPORT_CATEGORY_ANIMATION_NODE = 5, INTERNAL_IMPORT_CATEGORY_SKELETON_3D_NODE = 6, INTERNAL_IMPORT_CATEGORY_MAX = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERNAL_IMPORT_CATEGORY_NODE {#INTERNAL_IMPORT_CATEGORY_NODE}

```gdscript
INTERNAL_IMPORT_CATEGORY_NODE = 0
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### INTERNAL_IMPORT_CATEGORY_MESH_3D_NODE {#INTERNAL_IMPORT_CATEGORY_MESH_3D_NODE}

```gdscript
INTERNAL_IMPORT_CATEGORY_MESH_3D_NODE = 1
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### INTERNAL_IMPORT_CATEGORY_MESH {#INTERNAL_IMPORT_CATEGORY_MESH}

```gdscript
INTERNAL_IMPORT_CATEGORY_MESH = 2
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### INTERNAL_IMPORT_CATEGORY_MATERIAL {#INTERNAL_IMPORT_CATEGORY_MATERIAL}

```gdscript
INTERNAL_IMPORT_CATEGORY_MATERIAL = 3
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### INTERNAL_IMPORT_CATEGORY_ANIMATION {#INTERNAL_IMPORT_CATEGORY_ANIMATION}

```gdscript
INTERNAL_IMPORT_CATEGORY_ANIMATION = 4
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### INTERNAL_IMPORT_CATEGORY_ANIMATION_NODE {#INTERNAL_IMPORT_CATEGORY_ANIMATION_NODE}

```gdscript
INTERNAL_IMPORT_CATEGORY_ANIMATION_NODE = 5
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### INTERNAL_IMPORT_CATEGORY_SKELETON_3D_NODE {#INTERNAL_IMPORT_CATEGORY_SKELETON_3D_NODE}

```gdscript
INTERNAL_IMPORT_CATEGORY_SKELETON_3D_NODE = 6
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### INTERNAL_IMPORT_CATEGORY_MAX {#INTERNAL_IMPORT_CATEGORY_MAX}

```gdscript
INTERNAL_IMPORT_CATEGORY_MAX = 7
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

---
