# GDExtension

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GDExtension**

> The GDExtension resource type represents a shared library [https://en.wikipedia.org/wiki/Shared_library] which can expand the functionality of the engine. The GDExtensionManager singleton is responsible for loading, reloading, and unloading GDExtension resources.

<details><summary>官方英文描述（中文精译进行中）</summary>

The GDExtension resource type represents a shared library [https://en.wikipedia.org/wiki/Shared_library] which can expand the functionality of the engine. The GDExtensionManager singleton is responsible for loading, reloading, and unloading GDExtension resources.

Note: GDExtension itself is not a scripting language and has no relation to GDScript resources.

</details>

## 方法

### get_minimum_library_initialization_level {#get_minimum_library_initialization_level}

```gdscript
GDExtension.InitializationLevel get_minimum_library_initialization_level() const
```

Returns the lowest level required for this extension to be properly initialized (see the InitializationLevel enum).

### is_library_open {#is_library_open}

```gdscript
bool is_library_open() const
```

Returns true if this extension's library has been opened.

## 枚举

### InitializationLevel {#enum-InitializationLevel}

```gdscript
enum InitializationLevel { INITIALIZATION_LEVEL_CORE = 0, INITIALIZATION_LEVEL_SERVERS = 1, INITIALIZATION_LEVEL_SCENE = 2, INITIALIZATION_LEVEL_EDITOR = 3 }
```

InitializationLevel INITIALIZATION_LEVEL_CORE = 0

The library is initialized at the same time as the core features of the engine.

InitializationLevel INITIALIZATION_LEVEL_SERVERS = 1

The library is initialized at the same time as the engine's servers (such as RenderingServer or PhysicsServer3D).

InitializationLevel INITIALIZATION_LEVEL_SCENE = 2

The library is initialized at the same time as the engine's scene-related classes.

InitializationLevel INITIALIZATION_LEVEL_EDITOR = 3

The library is initialized at the same time as the engine's editor classes. Only happens when loading the GDExtension in the editor.

#### INITIALIZATION_LEVEL_CORE {#INITIALIZATION_LEVEL_CORE}

```gdscript
INITIALIZATION_LEVEL_CORE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INITIALIZATION_LEVEL_SERVERS {#INITIALIZATION_LEVEL_SERVERS}

```gdscript
INITIALIZATION_LEVEL_SERVERS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INITIALIZATION_LEVEL_SCENE {#INITIALIZATION_LEVEL_SCENE}

```gdscript
INITIALIZATION_LEVEL_SCENE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INITIALIZATION_LEVEL_EDITOR {#INITIALIZATION_LEVEL_EDITOR}

```gdscript
INITIALIZATION_LEVEL_EDITOR = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
