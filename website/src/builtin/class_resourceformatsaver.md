# ResourceFormatSaver

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **ResourceFormatSaver**

> The engine can save resources when you do it from the editor, or when you use the ResourceSaver singleton. This is accomplished thanks to multiple ResourceFormatSavers, each handling its own format and called automatically by the engine.

<details><summary>官方英文描述</summary>

The engine can save resources when you do it from the editor, or when you use the ResourceSaver singleton. This is accomplished thanks to multiple ResourceFormatSavers, each handling its own format and called automatically by the engine.

By default, Godot saves resources as .tres (text-based), .res (binary) or another built-in format, but you can choose to create your own format by extending this class. Be sure to respect the documented return types and values. You should give it a global class name with class_name for it to be registered. Like built-in ResourceFormatSavers, it will be called automatically when saving resources of its recognized type(s). You may also implement a ResourceFormatLoader.

</details>

## 方法

### _get_recognized_extensions {#_get_recognized_extensions}

```gdscript
PackedStringArray _get_recognized_extensions(resource: Resource) virtual const
```

Returns the list of extensions available for saving the resource object, provided it is recognized (see _recognize()).

### _recognize {#_recognize}

```gdscript
bool _recognize(resource: Resource) virtual const
```

Returns whether the given resource object can be saved by this saver.

### _recognize_path {#_recognize_path}

```gdscript
bool _recognize_path(resource: Resource, path: String) virtual const
```

Returns true if this saver handles a given save path and false otherwise.

If this method is not implemented, the default behavior returns whether the path's extension is within the ones provided by _get_recognized_extensions().

### _save {#_save}

```gdscript
Error _save(resource: Resource, path: String, flags: int) virtual
```

Saves the given resource object to a file at the target path. flags is a bitmask composed with SaverFlags constants.

Returns @GlobalScope.OK on success, or an Error constant in case of failure.

### _set_uid {#_set_uid}

```gdscript
Error _set_uid(path: String, uid: int) virtual
```

Sets a new UID for the resource at the given path. Returns @GlobalScope.OK on success, or an Error constant in case of failure.
