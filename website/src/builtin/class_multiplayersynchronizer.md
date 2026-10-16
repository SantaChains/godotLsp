# MultiplayerSynchronizer

**继承**：[Node](class_node.md) → [Object](class_object.md) → **MultiplayerSynchronizer**

> By default, MultiplayerSynchronizer synchronizes configured properties to all peers.

<details><summary>官方英文描述（中文精译进行中）</summary>

By default, MultiplayerSynchronizer synchronizes configured properties to all peers.

Visibility can be handled directly with set_visibility_for() or as-needed with add_visibility_filter() and update_visibility().

MultiplayerSpawners will handle nodes according to visibility of synchronizers as long as the node at root_path was spawned by one.

Internally, MultiplayerSynchronizer uses MultiplayerAPI.object_configuration_add() to notify synchronization start passing the Node at root_path as the object and itself as the configuration, and uses MultiplayerAPI.object_configuration_remove() to notify synchronization end in a similar way.

Note: Synchronization is not supported for Object type properties, like Resource. Properties that are unique to each peer, like the instance IDs of Objects (see Object.get_instance_id()) or RIDs, will also not work in synchronization.

</details>

## 方法

### add_visibility_filter {#add_visibility_filter}

```gdscript
void add_visibility_filter(filter: Callable)
```

Adds a peer visibility filter for this synchronizer.

filter should take a peer ID int and return a bool.

### get_delta_interval {#get_delta_interval}

```gdscript
float get_delta_interval() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_replication_config {#get_replication_config}

```gdscript
SceneReplicationConfig get_replication_config()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_replication_interval {#get_replication_interval}

```gdscript
float get_replication_interval() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_root_path {#get_root_path}

```gdscript
NodePath get_root_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visibility_for {#get_visibility_for}

```gdscript
bool get_visibility_for(peer: int) const
```

Queries the current visibility for peer peer.

### get_visibility_update_mode {#get_visibility_update_mode}

```gdscript
MultiplayerSynchronizer.VisibilityUpdateMode get_visibility_update_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_visibility_public {#is_visibility_public}

```gdscript
bool is_visibility_public() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### remove_visibility_filter {#remove_visibility_filter}

```gdscript
void remove_visibility_filter(filter: Callable)
```

Removes a peer visibility filter from this synchronizer.

### set_delta_interval {#set_delta_interval}

```gdscript
void set_delta_interval(milliseconds: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_replication_config {#set_replication_config}

```gdscript
void set_replication_config(config: SceneReplicationConfig)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_replication_interval {#set_replication_interval}

```gdscript
void set_replication_interval(milliseconds: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_root_path {#set_root_path}

```gdscript
void set_root_path(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visibility_for {#set_visibility_for}

```gdscript
void set_visibility_for(peer: int, visible: bool)
```

Sets the visibility of peer to visible. If peer is 0, the value of public_visibility will be updated instead.

### set_visibility_public {#set_visibility_public}

```gdscript
void set_visibility_public(visible: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visibility_update_mode {#set_visibility_update_mode}

```gdscript
void set_visibility_update_mode(mode: MultiplayerSynchronizer.VisibilityUpdateMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### update_visibility {#update_visibility}

```gdscript
void update_visibility(for_peer: int = 0)
```

Updates the visibility of for_peer according to visibility filters. If for_peer is 0 (the default), all peers' visibilties are updated.

## 属性

### root_path {#root_path}

```gdscript
root_path: NodePath
```

- void set_root_path(value: NodePath)
- NodePath get_root_path()

Node path that replicated properties are relative to.

If root_path was spawned by a MultiplayerSpawner, the node will be also be spawned and despawned based on this synchronizer visibility options.

### replication_interval {#replication_interval}

```gdscript
replication_interval: float
```

- void set_replication_interval(value: float)
- float get_replication_interval()

Time interval between synchronizations. Used when the replication is set to SceneReplicationConfig.REPLICATION_MODE_ALWAYS. If set to 0.0 (the default), synchronizations happen every network process frame.

### delta_interval {#delta_interval}

```gdscript
delta_interval: float
```

- void set_delta_interval(value: float)
- float get_delta_interval()

Time interval between delta synchronizations. Used when the replication is set to SceneReplicationConfig.REPLICATION_MODE_ON_CHANGE. If set to 0.0 (the default), delta synchronizations happen every network process frame.

### replication_config {#replication_config}

```gdscript
replication_config: SceneReplicationConfig
```

- void set_replication_config(value: SceneReplicationConfig)
- SceneReplicationConfig get_replication_config()

Resource containing which properties to synchronize.

### visibility_update_mode {#visibility_update_mode}

```gdscript
visibility_update_mode: int
```

- void set_visibility_update_mode(value: VisibilityUpdateMode)
- VisibilityUpdateMode get_visibility_update_mode()

Specifies when visibility filters are updated.

### public_visibility {#public_visibility}

```gdscript
public_visibility: bool
```

- void set_visibility_public(value: bool)
- bool is_visibility_public()

Whether synchronization should be visible to all peers by default. See set_visibility_for() and add_visibility_filter() for ways of configuring fine-grained visibility options.

## 信号

### synchronized {#synchronized}

```gdscript
signal synchronized()
```

Emitted when a new synchronization state is received by this synchronizer after the properties have been updated.

### delta_synchronized {#delta_synchronized}

```gdscript
signal delta_synchronized()
```

Emitted when a new delta synchronization state is received by this synchronizer after the properties have been updated.

### visibility_changed {#visibility_changed}

```gdscript
signal visibility_changed(for_peer: int)
```

Emitted when visibility of for_peer is updated. See update_visibility().

## 枚举

### VisibilityUpdateMode {#enum-VisibilityUpdateMode}

```gdscript
enum VisibilityUpdateMode { VISIBILITY_PROCESS_IDLE = 0, VISIBILITY_PROCESS_PHYSICS = 1, VISIBILITY_PROCESS_NONE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VISIBILITY_PROCESS_IDLE {#VISIBILITY_PROCESS_IDLE}

```gdscript
VISIBILITY_PROCESS_IDLE = 0
```

Visibility filters are updated during process frames (see Node.NOTIFICATION_INTERNAL_PROCESS).

#### VISIBILITY_PROCESS_PHYSICS {#VISIBILITY_PROCESS_PHYSICS}

```gdscript
VISIBILITY_PROCESS_PHYSICS = 1
```

Visibility filters are updated during physics frames (see Node.NOTIFICATION_INTERNAL_PHYSICS_PROCESS).

#### VISIBILITY_PROCESS_NONE {#VISIBILITY_PROCESS_NONE}

```gdscript
VISIBILITY_PROCESS_NONE = 2
```

Visibility filters are not updated automatically, and must be updated manually by calling update_visibility().

---
