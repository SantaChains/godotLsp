# ResourceSaver

**继承**：[Object](class_object.md) → **ResourceSaver**

> A singleton for saving resource types to the filesystem.

<details><summary>官方英文描述</summary>

A singleton for saving resource types to the filesystem.

It uses the many ResourceFormatSaver classes registered in the engine (either built-in or from a plugin) to save resource data to text-based (e.g. .tres or .tscn) or binary files (e.g. .res or .scn).

</details>

## 方法

### add_resource_format_saver {#add_resource_format_saver}

```gdscript
void add_resource_format_saver(format_saver: ResourceFormatSaver, at_front: bool = false)
```

Registers a new ResourceFormatSaver. The ResourceSaver will use the ResourceFormatSaver as described in save().

This method is performed implicitly for ResourceFormatSavers written in GDScript (see ResourceFormatSaver for more information).

### get_recognized_extensions {#get_recognized_extensions}

```gdscript
PackedStringArray get_recognized_extensions(type: Resource)
```

Returns the list of extensions available for saving a resource of a given type.

### get_resource_id_for_path {#get_resource_id_for_path}

```gdscript
int get_resource_id_for_path(path: String, generate: bool = false)
```

Returns the resource ID for the given path. If generate is true, a new resource ID will be generated if one for the path is not found. If generate is false and the path is not found, ResourceUID.INVALID_ID is returned.

### remove_resource_format_saver {#remove_resource_format_saver}

```gdscript
void remove_resource_format_saver(format_saver: ResourceFormatSaver)
```

Unregisters the given ResourceFormatSaver.

### save {#save}

```gdscript
Error save(resource: Resource, path: String = "", flags: bitfield::ResourceSaver.SaverFlags = 0)
```

Saves a resource to disk to the given path, using a ResourceFormatSaver that recognizes the resource object. If path is empty, ResourceSaver will try to use Resource.resource_path.

The flags bitmask can be specified to customize the save behavior.

Returns @GlobalScope.OK on success.

Note: When the project is running, any generated UID associated with the resource will not be saved as the required code is only executed in editor mode.

### set_uid {#set_uid}

```gdscript
Error set_uid(resource: String, uid: int)
```

Sets the UID of the given resource path to uid. You can generate a new UID using ResourceUID.create_id().

Since resources will normally get a UID automatically, this method is only useful in very specific cases.

## 枚举

### SaverFlags {#enum-SaverFlags}

```gdscript
enum SaverFlags（位标志） { FLAG_NONE = 0, FLAG_RELATIVE_PATHS = 1, FLAG_BUNDLE_RESOURCES = 2, FLAG_CHANGE_PATH = 4, FLAG_OMIT_EDITOR_PROPERTIES = 8, FLAG_SAVE_BIG_ENDIAN = 16, FLAG_COMPRESS = 32, FLAG_REPLACE_SUBRESOURCE_PATHS = 64 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_NONE {#FLAG_NONE}

```gdscript
FLAG_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_RELATIVE_PATHS {#FLAG_RELATIVE_PATHS}

```gdscript
FLAG_RELATIVE_PATHS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_BUNDLE_RESOURCES {#FLAG_BUNDLE_RESOURCES}

```gdscript
FLAG_BUNDLE_RESOURCES = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_CHANGE_PATH {#FLAG_CHANGE_PATH}

```gdscript
FLAG_CHANGE_PATH = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_OMIT_EDITOR_PROPERTIES {#FLAG_OMIT_EDITOR_PROPERTIES}

```gdscript
FLAG_OMIT_EDITOR_PROPERTIES = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_SAVE_BIG_ENDIAN {#FLAG_SAVE_BIG_ENDIAN}

```gdscript
FLAG_SAVE_BIG_ENDIAN = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_COMPRESS {#FLAG_COMPRESS}

```gdscript
FLAG_COMPRESS = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FLAG_REPLACE_SUBRESOURCE_PATHS {#FLAG_REPLACE_SUBRESOURCE_PATHS}

```gdscript
FLAG_REPLACE_SUBRESOURCE_PATHS = 64
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
