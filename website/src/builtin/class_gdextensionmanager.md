# GDExtensionManager

**继承**：[Object](class_object.md) → **GDExtensionManager**

> The GDExtensionManager loads, initializes, and keeps track of all available GDExtension libraries in the project.

<details><summary>官方英文描述（中文精译进行中）</summary>

The GDExtensionManager loads, initializes, and keeps track of all available GDExtension libraries in the project.

Note: Do not worry about GDExtension unless you know what you are doing.

</details>

## 方法

### get_extension {#get_extension}

```gdscript
GDExtension get_extension(path: String)
```

Returns the GDExtension at the given file path, or null if it has not been loaded or does not exist.

### get_loaded_extensions {#get_loaded_extensions}

```gdscript
PackedStringArray get_loaded_extensions() const
```

Returns the file paths of all currently loaded extensions.

### is_extension_loaded {#is_extension_loaded}

```gdscript
bool is_extension_loaded(path: String) const
```

Returns true if the extension at the given file path has already been loaded successfully. See also get_loaded_extensions().

### load_extension {#load_extension}

```gdscript
GDExtensionManager.LoadStatus load_extension(path: String)
```

Loads an extension by absolute file path. The path needs to point to a valid GDExtension. Returns LOAD_STATUS_OK if successful.

### load_extension_from_function {#load_extension_from_function}

```gdscript
GDExtensionManager.LoadStatus load_extension_from_function(path: String, init_func: const GDExtensionInitializationFunction*)
```

Loads the extension already in address space via the given path and initialization function. The path needs to be unique and start with "libgodot://". Returns LOAD_STATUS_OK if successful.

### reload_extension {#reload_extension}

```gdscript
GDExtensionManager.LoadStatus reload_extension(path: String)
```

Reloads the extension at the given file path. The path needs to point to a valid GDExtension, otherwise this method may return either LOAD_STATUS_NOT_LOADED or LOAD_STATUS_FAILED.

Note: You can only reload extensions in the editor. In release builds, this method always fails and returns LOAD_STATUS_FAILED.

### unload_extension {#unload_extension}

```gdscript
GDExtensionManager.LoadStatus unload_extension(path: String)
```

Unloads an extension by file path. The path needs to point to an already loaded GDExtension, otherwise this method returns LOAD_STATUS_NOT_LOADED.

## 信号

### extensions_reloaded {#extensions_reloaded}

```gdscript
signal extensions_reloaded()
```

Emitted after the editor has finished reloading one or more extensions.

### extension_loaded {#extension_loaded}

```gdscript
signal extension_loaded(extension: GDExtension)
```

Emitted after the editor has finished loading a new extension.

Note: This signal is only emitted in editor builds.

### extension_unloading {#extension_unloading}

```gdscript
signal extension_unloading(extension: GDExtension)
```

Emitted before the editor starts unloading an extension.

Note: This signal is only emitted in editor builds.

## 枚举

### LoadStatus {#enum-LoadStatus}

```gdscript
enum LoadStatus { LOAD_STATUS_OK = 0, LOAD_STATUS_FAILED = 1, LOAD_STATUS_ALREADY_LOADED = 2, LOAD_STATUS_NOT_LOADED = 3, LOAD_STATUS_NEEDS_RESTART = 4 }
```

LoadStatus LOAD_STATUS_OK = 0

The extension has loaded successfully.

LoadStatus LOAD_STATUS_FAILED = 1

The extension has failed to load, possibly because it does not exist or has missing dependencies.

LoadStatus LOAD_STATUS_ALREADY_LOADED = 2

The extension has already been loaded.

LoadStatus LOAD_STATUS_NOT_LOADED = 3

The extension has not been loaded.

LoadStatus LOAD_STATUS_NEEDS_RESTART = 4

The extension requires the application to restart to fully load.

#### LOAD_STATUS_OK {#LOAD_STATUS_OK}

```gdscript
LOAD_STATUS_OK = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOAD_STATUS_FAILED {#LOAD_STATUS_FAILED}

```gdscript
LOAD_STATUS_FAILED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOAD_STATUS_ALREADY_LOADED {#LOAD_STATUS_ALREADY_LOADED}

```gdscript
LOAD_STATUS_ALREADY_LOADED = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOAD_STATUS_NOT_LOADED {#LOAD_STATUS_NOT_LOADED}

```gdscript
LOAD_STATUS_NOT_LOADED = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOAD_STATUS_NEEDS_RESTART {#LOAD_STATUS_NEEDS_RESTART}

```gdscript
LOAD_STATUS_NEEDS_RESTART = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
