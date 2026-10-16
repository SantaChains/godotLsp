# SceneReplicationConfig

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SceneReplicationConfig**

## 方法

### add_property {#add_property}

```gdscript
void add_property(path: NodePath, index: int = -1)
```

Adds the property identified by the given path to the list of the properties being synchronized, optionally passing an index.

Note: For details on restrictions and limitations on property synchronization, see MultiplayerSynchronizer.

### get_properties {#get_properties}

```gdscript
typedarray::NodePath get_properties() const
```

Returns a list of synchronized property NodePaths.

### has_property {#has_property}

```gdscript
bool has_property(path: NodePath) const
```

Returns true if the given path is configured for synchronization.

### property_get_index {#property_get_index}

```gdscript
int property_get_index(path: NodePath) const
```

Finds the index of the given path.

### property_get_replication_mode {#property_get_replication_mode}

```gdscript
SceneReplicationConfig.ReplicationMode property_get_replication_mode(path: NodePath)
```

Returns the replication mode for the property identified by the given path.

### property_get_spawn {#property_get_spawn}

```gdscript
bool property_get_spawn(path: NodePath)
```

Returns true if the property identified by the given path is configured to be synchronized on spawn.

### property_get_sync {#property_get_sync}

```gdscript
bool property_get_sync(path: NodePath)
```

Deprecated: Use property_get_replication_mode() instead.

Returns true if the property identified by the given path is configured to be synchronized on process.

### property_get_watch {#property_get_watch}

```gdscript
bool property_get_watch(path: NodePath)
```

Deprecated: Use property_get_replication_mode() instead.

Returns true if the property identified by the given path is configured to be reliably synchronized when changes are detected on process.

### property_set_replication_mode {#property_set_replication_mode}

```gdscript
void property_set_replication_mode(path: NodePath, mode: SceneReplicationConfig.ReplicationMode)
```

Sets the synchronization mode for the property identified by the given path.

### property_set_spawn {#property_set_spawn}

```gdscript
void property_set_spawn(path: NodePath, enabled: bool)
```

Sets whether the property identified by the given path is configured to be synchronized on spawn.

### property_set_sync {#property_set_sync}

```gdscript
void property_set_sync(path: NodePath, enabled: bool)
```

Deprecated: Use property_set_replication_mode() with REPLICATION_MODE_ALWAYS instead.

Sets whether the property identified by the given path is configured to be synchronized on process.

### property_set_watch {#property_set_watch}

```gdscript
void property_set_watch(path: NodePath, enabled: bool)
```

Deprecated: Use property_set_replication_mode() with REPLICATION_MODE_ON_CHANGE instead.

Sets whether the property identified by the given path is configured to be reliably synchronized when changes are detected on process.

### remove_property {#remove_property}

```gdscript
void remove_property(path: NodePath)
```

Removes the property identified by the given path from the configuration.

## 枚举

### ReplicationMode {#enum-ReplicationMode}

```gdscript
enum ReplicationMode { REPLICATION_MODE_NEVER = 0, REPLICATION_MODE_ALWAYS = 1, REPLICATION_MODE_ON_CHANGE = 2 }
```

ReplicationMode REPLICATION_MODE_NEVER = 0

Do not keep the given property synchronized.

ReplicationMode REPLICATION_MODE_ALWAYS = 1

Replicate the given property on process by constantly sending updates using unreliable transfer mode.

ReplicationMode REPLICATION_MODE_ON_CHANGE = 2

Replicate the given property on process by sending updates using reliable transfer mode when its value changes.

#### REPLICATION_MODE_NEVER {#REPLICATION_MODE_NEVER}

```gdscript
REPLICATION_MODE_NEVER = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### REPLICATION_MODE_ALWAYS {#REPLICATION_MODE_ALWAYS}

```gdscript
REPLICATION_MODE_ALWAYS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### REPLICATION_MODE_ON_CHANGE {#REPLICATION_MODE_ON_CHANGE}

```gdscript
REPLICATION_MODE_ON_CHANGE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
