# Resource

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **Resource**

> Resource is the base class for all Godot-specific resource types, serving primarily as data containers. Since they inherit from RefCounted, resources are reference-counted and freed when no longer in use. They can also be nested within other resources, and saved on disk. PackedScene, one of the most common Objects in a Godot project, is also a resource, uniquely capable of storing and instantiating the Nodes it contains as many times as desired.

<details><summary>官方英文描述（中文精译进行中）</summary>

Resource is the base class for all Godot-specific resource types, serving primarily as data containers. Since they inherit from RefCounted, resources are reference-counted and freed when no longer in use. They can also be nested within other resources, and saved on disk. PackedScene, one of the most common Objects in a Godot project, is also a resource, uniquely capable of storing and instantiating the Nodes it contains as many times as desired.

In GDScript, resources can loaded from disk by their resource_path using @GDScript.load() or @GDScript.preload().

The engine keeps a global cache of all loaded resources, referenced by paths (see ResourceLoader.has_cached()). A resource will be cached when loaded for the first time and removed from cache once all references are released. When a resource is cached, subsequent loads using its path will return the cached reference.

Note: In C#, resources will not be freed instantly after they are no longer in use. Instead, garbage collection will run periodically and will free resources that are no longer in use. This means that unused resources will remain in memory for a while before being removed.

</details>

## 方法

### _get_rid {#_get_rid}

```gdscript
RID _get_rid() virtual const
```

Override this method to return a custom RID when get_rid() is called.

### _reset_state {#_reset_state}

```gdscript
void _reset_state() virtual
```

For resources that store state in non-exported properties, such as via Object._validate_property() or Object._get_property_list(), this method must be implemented to clear them.

### _set_path_cache {#_set_path_cache}

```gdscript
void _set_path_cache(path: String) virtual const
```

Override this method to execute additional logic after set_path_cache() is called on this object.

### _setup_local_to_scene {#_setup_local_to_scene}

```gdscript
void _setup_local_to_scene() virtual
```

Override this method to customize the newly duplicated resource created from PackedScene.instantiate(), if the original's resource_local_to_scene is set to true.

Example: Set a random damage value to every local resource from an instantiated scene:

```gdscript
extends Resource
var damage = 0
func _setup_local_to_scene():
    damage = randi_range(10, 40)
```

### copy_from_resource {#copy_from_resource}

```gdscript
Error copy_from_resource(resource: Resource)
```

Copies the data from resource into this resource. Both resources must share the same class.

### duplicate {#duplicate}

```gdscript
Resource duplicate(deep: bool = false) const
```

Duplicates this resource, returning a new resource with its exported or @GlobalScope.PROPERTY_USAGE_STORAGE properties copied from the original.

If deep is false, a shallow copy is returned: nested Array, Dictionary, and Resource properties are not duplicated and are shared with the original resource.

If deep is true, a deep copy is returned: all nested arrays, dictionaries, and packed arrays are also duplicated (recursively). Any Resource found inside will only be duplicated if it's local, like DEEP_DUPLICATE_INTERNAL used with duplicate_deep().

The following exceptions apply:

- Subresource properties with the @GlobalScope.PROPERTY_USAGE_ALWAYS_DUPLICATE flag are always duplicated (recursively or not, depending on deep).
- Subresource properties with the @GlobalScope.PROPERTY_USAGE_NEVER_DUPLICATE flag are never duplicated.

Note: For custom resources, this method will fail if Object._init() has been defined with required parameters.

Note: When duplicating with deep set to true, each resource found, including the one on which this method is called, will be only duplicated once and referenced as many times as needed in the duplicate. For instance, if you are duplicating resource A that happens to have resource B referenced twice, you'll get a new resource A' referencing a new resource B' twice.

### duplicate_deep {#duplicate_deep}

```gdscript
Resource duplicate_deep(deep_subresources_mode: Resource.DeepDuplicateMode = 1) const
```

Duplicates this resource, deeply, like duplicate() when passing true, with extra control over how subresources are handled.

### emit_changed {#emit_changed}

```gdscript
void emit_changed()
```

Emits the changed signal. This method is called automatically for some built-in resources.

Note: For custom resources, it's recommended to call this method whenever a meaningful change occurs, such as a modified property. This ensures that custom Objects depending on the resource are properly updated.

```gdscript
var damage:
    set(new_value):
        if damage != new_value:
            damage = new_value
            emit_changed()
```

### generate_scene_unique_id {#generate_scene_unique_id}

```gdscript
static String generate_scene_unique_id()
```

Generates a unique identifier for a resource to be contained inside a PackedScene, based on the current date, time, and a random value. The returned string is only composed of letters (a to y) and numbers (0 to 8). See also resource_scene_unique_id.

### get_id_for_path {#get_id_for_path}

```gdscript
String get_id_for_path(path: String) const
```

From the internal cache for scene-unique IDs, returns the ID of this resource for the scene at path. If there is no entry, an empty string is returned. Useful to keep scene-unique IDs the same when implementing a VCS-friendly custom resource format by extending ResourceFormatLoader and ResourceFormatSaver.

Note: This method is only implemented when running in an editor context. At runtime, it returns an empty string.

### get_local_scene {#get_local_scene}

```gdscript
Node get_local_scene() const
```

If resource_local_to_scene is set to true and the resource has been loaded from a PackedScene instantiation, returns the root Node of the scene where this resource is used. Otherwise, returns null.

### get_name {#get_name}

```gdscript
String get_name() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_path {#get_path}

```gdscript
String get_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rid {#get_rid}

```gdscript
RID get_rid() const
```

Returns the RID of this resource (or an empty RID). Many resources (such as Texture2D, Mesh, and so on) are high-level abstractions of resources stored in a specialized server (DisplayServer, RenderingServer, etc.), so this function will return the original RID.

### get_scene_unique_id {#get_scene_unique_id}

```gdscript
String get_scene_unique_id() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_built_in {#is_built_in}

```gdscript
bool is_built_in() const
```

Returns true if the resource is saved on disk as a part of another resource's file.

### is_local_to_scene {#is_local_to_scene}

```gdscript
bool is_local_to_scene() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### reset_state {#reset_state}

```gdscript
void reset_state()
```

Makes the resource clear its non-exported properties. See also _reset_state(). Useful when implementing a custom resource format by extending ResourceFormatLoader and ResourceFormatSaver.

### set_id_for_path {#set_id_for_path}

```gdscript
void set_id_for_path(path: String, id: String)
```

In the internal cache for scene-unique IDs, sets the ID of this resource to id for the scene at path. If id is empty, the cache entry for path is cleared. Useful to keep scene-unique IDs the same when implementing a VCS-friendly custom resource format by extending ResourceFormatLoader and ResourceFormatSaver.

Note: This method is only implemented when running in an editor context.

### set_local_to_scene {#set_local_to_scene}

```gdscript
void set_local_to_scene(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_name {#set_name}

```gdscript
void set_name(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path {#set_path}

```gdscript
void set_path(path: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_path_cache {#set_path_cache}

```gdscript
void set_path_cache(path: String)
```

Sets the resource's path to path without involving the resource cache. Useful for handling CacheMode values when implementing a custom resource format by extending ResourceFormatLoader and ResourceFormatSaver.

### set_scene_unique_id {#set_scene_unique_id}

```gdscript
void set_scene_unique_id(id: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### setup_local_to_scene {#setup_local_to_scene}

```gdscript
void setup_local_to_scene()
```

Deprecated: This method should only be called internally.

Calls _setup_local_to_scene(). If resource_local_to_scene is set to true, this method is automatically called from PackedScene.instantiate() by the newly duplicated resource within the scene instance.

### take_over_path {#take_over_path}

```gdscript
void take_over_path(path: String)
```

Sets the resource_path to path, potentially overriding an existing cache entry for this path. Further attempts to load an overridden resource by path will instead return this resource.

## 属性

### resource_local_to_scene {#resource_local_to_scene}

```gdscript
resource_local_to_scene: bool
```

- void set_local_to_scene(value: bool)
- bool is_local_to_scene()

If true, the resource is duplicated for each instance of all scenes using it. At run-time, the resource can be modified in one scene without affecting other instances (see PackedScene.instantiate()).

Note: Changing this property at run-time has no effect on already created duplicate resources.

### resource_path {#resource_path}

```gdscript
resource_path: String
```

- void set_path(value: String)
- String get_path()

The unique path to this resource. If it has been saved to disk, the value will be its filepath. If the resource is exclusively contained within a scene, the value will be the PackedScene's filepath, followed by a unique identifier.

Note: Setting this property manually may fail if a resource with the same path has already been previously loaded. If necessary, use take_over_path().

### resource_name {#resource_name}

```gdscript
resource_name: String
```

- void set_name(value: String)
- String get_name()

An optional name for this resource. When defined, its value is displayed to represent the resource in the Inspector dock. For built-in scripts, the name is displayed as part of the tab name in the script editor.

Note: Some resource formats do not support resource names. You can still set the name in the editor or via code, but it will be lost when the resource is reloaded. For example, only built-in scripts can have a resource name, while scripts stored in separate files cannot.

### resource_scene_unique_id {#resource_scene_unique_id}

```gdscript
resource_scene_unique_id: String
```

- void set_scene_unique_id(value: String)
- String get_scene_unique_id()

A unique identifier relative to this resource's scene. If left empty, the ID is automatically generated when this resource is saved inside a PackedScene. If the resource is not inside a scene, this property is empty by default.

Note: When the PackedScene is saved, if multiple resources in the same scene use the same ID, only the earliest resource in the scene hierarchy keeps the original ID. The other resources are assigned new IDs from generate_scene_unique_id().

Note: Setting this property does not emit the changed signal.

Warning: When setting, the ID must only consist of letters, numbers, and underscores. Otherwise, it will fail and default to a randomly generated ID.

## 信号

### changed {#changed}

```gdscript
signal changed()
```

Emitted when the resource changes, usually when one of its properties is modified. See also emit_changed().

Note: This signal is not emitted automatically for properties of custom resources. If necessary, a setter needs to be created to emit the signal.

### setup_local_to_scene_requested {#setup_local_to_scene_requested}

```gdscript
signal setup_local_to_scene_requested()
```

Deprecated: This signal is only emitted when the resource is created. Override _setup_local_to_scene() instead.

Emitted by a newly duplicated resource with resource_local_to_scene set to true.

## 枚举

### DeepDuplicateMode {#enum-DeepDuplicateMode}

```gdscript
enum DeepDuplicateMode { DEEP_DUPLICATE_NONE = 0, DEEP_DUPLICATE_INTERNAL = 1, DEEP_DUPLICATE_ALL = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEEP_DUPLICATE_NONE {#DEEP_DUPLICATE_NONE}

```gdscript
DEEP_DUPLICATE_NONE = 0
```

No subresources at all are duplicated. This is useful even in a deep duplication to have all the arrays and dictionaries duplicated but still pointing to the original resources.

#### DEEP_DUPLICATE_INTERNAL {#DEEP_DUPLICATE_INTERNAL}

```gdscript
DEEP_DUPLICATE_INTERNAL = 1
```

Only subresources without a path or with a scene-local path will be duplicated.

#### DEEP_DUPLICATE_ALL {#DEEP_DUPLICATE_ALL}

```gdscript
DEEP_DUPLICATE_ALL = 2
```

Every subresource found will be duplicated, even if it has a non-local path. In other words, even potentially big resources stored separately will be duplicated.

---
