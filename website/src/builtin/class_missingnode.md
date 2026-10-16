# MissingNode

**继承**：[Node](class_node.md) → [Object](class_object.md) → **MissingNode**

> This is an internal editor class intended for keeping data of nodes of unknown type (most likely this type was supplied by an extension that is no longer loaded). It can't be manually instantiated or placed in a scene.

<details><summary>官方英文描述</summary>

This is an internal editor class intended for keeping data of nodes of unknown type (most likely this type was supplied by an extension that is no longer loaded). It can't be manually instantiated or placed in a scene.

Warning: Ignore missing nodes unless you know what you are doing. Existing properties on a missing node can be freely modified in code, regardless of the type they are intended to be.

</details>

## 方法

### get_original_class {#get_original_class}

```gdscript
String get_original_class() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_original_scene {#get_original_scene}

```gdscript
String get_original_scene() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_recording_properties {#is_recording_properties}

```gdscript
bool is_recording_properties() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_recording_signals {#is_recording_signals}

```gdscript
bool is_recording_signals() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_original_class {#set_original_class}

```gdscript
void set_original_class(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_original_scene {#set_original_scene}

```gdscript
void set_original_scene(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_recording_properties {#set_recording_properties}

```gdscript
void set_recording_properties(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_recording_signals {#set_recording_signals}

```gdscript
void set_recording_signals(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### original_class {#original_class}

```gdscript
original_class: String
```

- void set_original_class(value: String)
- String get_original_class()

The name of the class this node was supposed to be (see Object.get_class()).

### original_scene {#original_scene}

```gdscript
original_scene: String
```

- void set_original_scene(value: String)
- String get_original_scene()

Returns the path of the scene this node was instance of originally.

### recording_properties {#recording_properties}

```gdscript
recording_properties: bool
```

- void set_recording_properties(value: bool)
- bool is_recording_properties()

If true, allows new properties to be set along with existing ones. If false, only existing properties' values can be set, and new properties cannot be added.

### recording_signals {#recording_signals}

```gdscript
recording_signals: bool
```

- void set_recording_signals(value: bool)
- bool is_recording_signals()

If true, allows new signals to be connected to along with existing ones. If false, only existing signals can be connected to, and new signals cannot be added.
