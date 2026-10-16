# SceneState

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **SceneState**

> Maintains a list of resources, nodes, exported and overridden properties, and built-in scripts associated with a scene. They cannot be modified from a SceneState, only accessed. Useful for peeking into what a PackedScene contains without instantiating it.

<details><summary>官方英文描述（中文精译进行中）</summary>

Maintains a list of resources, nodes, exported and overridden properties, and built-in scripts associated with a scene. They cannot be modified from a SceneState, only accessed. Useful for peeking into what a PackedScene contains without instantiating it.

This class cannot be instantiated directly, it is retrieved for a given scene as the result of PackedScene.get_state().

</details>

## 方法

### get_base_scene_state {#get_base_scene_state}

```gdscript
SceneState get_base_scene_state() const
```

Returns the SceneState of the scene that this scene inherits from, or null if it doesn't inherit from any scene.

### get_connection_binds {#get_connection_binds}

```gdscript
Array get_connection_binds(idx: int) const
```

Returns the list of bound parameters for the signal at idx.

### get_connection_count {#get_connection_count}

```gdscript
int get_connection_count() const
```

Returns the number of signal connections in the scene.

The idx argument used to query connection metadata in other get_connection_* methods in the interval [0, get_connection_count() - 1].

### get_connection_flags {#get_connection_flags}

```gdscript
int get_connection_flags(idx: int) const
```

Returns the connection flags for the signal at idx. See ConnectFlags constants.

### get_connection_method {#get_connection_method}

```gdscript
StringName get_connection_method(idx: int) const
```

Returns the method connected to the signal at idx.

### get_connection_signal {#get_connection_signal}

```gdscript
StringName get_connection_signal(idx: int) const
```

Returns the name of the signal at idx.

### get_connection_source {#get_connection_source}

```gdscript
NodePath get_connection_source(idx: int) const
```

Returns the path to the node that owns the signal at idx, relative to the root node.

### get_connection_target {#get_connection_target}

```gdscript
NodePath get_connection_target(idx: int) const
```

Returns the path to the node that owns the method connected to the signal at idx, relative to the root node.

### get_connection_unbinds {#get_connection_unbinds}

```gdscript
int get_connection_unbinds(idx: int) const
```

Returns the number of unbound parameters for the signal at idx.

### get_node_count {#get_node_count}

```gdscript
int get_node_count() const
```

Returns the number of nodes in the scene.

The idx argument used to query node data in other get_node_* methods in the interval [0, get_node_count() - 1].

### get_node_groups {#get_node_groups}

```gdscript
PackedStringArray get_node_groups(idx: int) const
```

Returns the list of group names associated with the node at idx.

### get_node_index {#get_node_index}

```gdscript
int get_node_index(idx: int) const
```

Returns the node's index, which is its position relative to its siblings. This is only relevant and saved in scenes for cases where new nodes are added to an instantiated or inherited scene among siblings from the base scene. Despite the name, this index is not related to the idx argument used here and in other methods.

### get_node_instance {#get_node_instance}

```gdscript
PackedScene get_node_instance(idx: int) const
```

Returns a PackedScene for the node at idx (i.e. the whole branch starting at this node, with its child nodes and resources), or null if the node is not an instance.

### get_node_instance_placeholder {#get_node_instance_placeholder}

```gdscript
String get_node_instance_placeholder(idx: int) const
```

Returns the path to the represented scene file if the node at idx is an InstancePlaceholder.

### get_node_name {#get_node_name}

```gdscript
StringName get_node_name(idx: int) const
```

Returns the name of the node at idx.

### get_node_owner_path {#get_node_owner_path}

```gdscript
NodePath get_node_owner_path(idx: int) const
```

Returns the path to the owner of the node at idx, relative to the root node.

### get_node_path {#get_node_path}

```gdscript
NodePath get_node_path(idx: int, for_parent: bool = false) const
```

Returns the path to the node at idx.

If for_parent is true, returns the path of the idx node's parent instead.

### get_node_property_count {#get_node_property_count}

```gdscript
int get_node_property_count(idx: int) const
```

Returns the number of exported or overridden properties for the node at idx.

The prop_idx argument used to query node property data in other get_node_property_* methods in the interval [0, get_node_property_count() - 1].

### get_node_property_name {#get_node_property_name}

```gdscript
StringName get_node_property_name(idx: int, prop_idx: int) const
```

Returns the name of the property at prop_idx for the node at idx.

### get_node_property_value {#get_node_property_value}

```gdscript
Variant get_node_property_value(idx: int, prop_idx: int) const
```

Returns the value of the property at prop_idx for the node at idx.

### get_node_type {#get_node_type}

```gdscript
StringName get_node_type(idx: int) const
```

Returns the type of the node at idx.

### get_path {#get_path}

```gdscript
String get_path() const
```

Returns the resource path to the represented PackedScene.

### is_node_instance_placeholder {#is_node_instance_placeholder}

```gdscript
bool is_node_instance_placeholder(idx: int) const
```

Returns true if the node at idx is an InstancePlaceholder.

## 枚举

### GenEditState {#enum-GenEditState}

```gdscript
enum GenEditState { GEN_EDIT_STATE_DISABLED = 0, GEN_EDIT_STATE_INSTANCE = 1, GEN_EDIT_STATE_MAIN = 2, GEN_EDIT_STATE_MAIN_INHERITED = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GEN_EDIT_STATE_DISABLED {#GEN_EDIT_STATE_DISABLED}

```gdscript
GEN_EDIT_STATE_DISABLED = 0
```

If passed to PackedScene.instantiate(), blocks edits to the scene state.

#### GEN_EDIT_STATE_INSTANCE {#GEN_EDIT_STATE_INSTANCE}

```gdscript
GEN_EDIT_STATE_INSTANCE = 1
```

If passed to PackedScene.instantiate(), provides inherited scene resources to the local scene.

Note: Only available in editor builds.

#### GEN_EDIT_STATE_MAIN {#GEN_EDIT_STATE_MAIN}

```gdscript
GEN_EDIT_STATE_MAIN = 2
```

If passed to PackedScene.instantiate(), provides local scene resources to the local scene. Only the main scene should receive the main edit state.

Note: Only available in editor builds.

#### GEN_EDIT_STATE_MAIN_INHERITED {#GEN_EDIT_STATE_MAIN_INHERITED}

```gdscript
GEN_EDIT_STATE_MAIN_INHERITED = 3
```

If passed to PackedScene.instantiate(), it's similar to GEN_EDIT_STATE_MAIN, but for the case where the scene is being instantiated to be the base of another one.

Note: Only available in editor builds.

---
