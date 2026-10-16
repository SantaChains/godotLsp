# MissingResource

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MissingResource**

> This is an internal editor class intended for keeping data of resources of unknown type (most likely this type was supplied by an extension that is no longer loaded). It can't be manually instantiated or placed in a scene.

<details><summary>官方英文描述</summary>

This is an internal editor class intended for keeping data of resources of unknown type (most likely this type was supplied by an extension that is no longer loaded). It can't be manually instantiated or placed in a scene.

Warning: Ignore missing resources unless you know what you are doing. Existing properties on a missing resource can be freely modified in code, regardless of the type they are intended to be.

</details>

## 方法

### get_original_class {#get_original_class}

```gdscript
String get_original_class() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_recording_properties {#is_recording_properties}

```gdscript
bool is_recording_properties() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_original_class {#set_original_class}

```gdscript
void set_original_class(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_recording_properties {#set_recording_properties}

```gdscript
void set_recording_properties(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### original_class {#original_class}

```gdscript
original_class: String
```

- void set_original_class(value: String)
- String get_original_class()

The name of the class this resource was supposed to be (see Object.get_class()).

### recording_properties {#recording_properties}

```gdscript
recording_properties: bool
```

- void set_recording_properties(value: bool)
- bool is_recording_properties()

If set to true, allows new properties to be added on top of the existing ones with Object.set().
