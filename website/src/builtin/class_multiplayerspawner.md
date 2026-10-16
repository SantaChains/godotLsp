# MultiplayerSpawner

**继承**：[Node](class_node.md) → [Object](class_object.md) → **MultiplayerSpawner**

> Spawnable scenes can be configured in the editor or through code (see add_spawnable_scene()).

<details><summary>官方英文描述</summary>

Spawnable scenes can be configured in the editor or through code (see add_spawnable_scene()).

Also supports custom node spawns through spawn(), calling spawn_function on all peers.

Internally, MultiplayerSpawner uses MultiplayerAPI.object_configuration_add() to notify spawns passing the spawned node as the object and itself as the configuration, and MultiplayerAPI.object_configuration_remove() to notify despawns in a similar way.

</details>

## 方法

### add_spawnable_scene {#add_spawnable_scene}

```gdscript
void add_spawnable_scene(path: String)
```

Adds a scene path to spawnable scenes, making it automatically replicated from the multiplayer authority to other peers when added as children of the node pointed by spawn_path.

### clear_spawnable_scenes {#clear_spawnable_scenes}

```gdscript
void clear_spawnable_scenes()
```

Clears all spawnable scenes. Does not despawn existing instances on remote peers.

### get_spawn_function {#get_spawn_function}

```gdscript
Callable get_spawn_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_spawn_limit {#get_spawn_limit}

```gdscript
int get_spawn_limit() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_spawn_path {#get_spawn_path}

```gdscript
NodePath get_spawn_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_spawnable_scene {#get_spawnable_scene}

```gdscript
String get_spawnable_scene(index: int) const
```

Returns the spawnable scene path by index.

### get_spawnable_scene_count {#get_spawnable_scene_count}

```gdscript
int get_spawnable_scene_count() const
```

Returns the count of spawnable scene paths.

### set_spawn_function {#set_spawn_function}

```gdscript
void set_spawn_function(spawn_function: Callable)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_spawn_limit {#set_spawn_limit}

```gdscript
void set_spawn_limit(limit: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_spawn_path {#set_spawn_path}

```gdscript
void set_spawn_path(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### spawn {#spawn}

```gdscript
Node spawn(data: Variant = null)
```

Requests a custom spawn, with data passed to spawn_function on all peers. Returns the locally spawned node instance already inside the scene tree, and added as a child of the node pointed by spawn_path.

Note: Spawnable scenes are spawned automatically. spawn() is only needed for custom spawns.

## 属性

### spawn_path {#spawn_path}

```gdscript
spawn_path: NodePath
```

- void set_spawn_path(value: NodePath)
- NodePath get_spawn_path()

Path to the spawn root. Spawnable scenes that are added as direct children are replicated to other peers.

### spawn_limit {#spawn_limit}

```gdscript
spawn_limit: int
```

- void set_spawn_limit(value: int)
- int get_spawn_limit()

Maximum number of nodes allowed to be spawned by this spawner. Includes both spawnable scenes and custom spawns.

When set to 0 (the default), there is no limit.

### spawn_function {#spawn_function}

```gdscript
spawn_function: Callable
```

- void set_spawn_function(value: Callable)
- Callable get_spawn_function()

Method called on all peers when a custom spawn() is requested by the authority. Will receive the data parameter, and should return a Node that is not in the scene tree.

Note: The returned node should not be added to the scene with Node.add_child(). This is done automatically.

## 信号

### despawned {#despawned}

```gdscript
signal despawned(node: Node)
```

Emitted when a spawnable scene or custom spawn was despawned by the multiplayer authority. Only called on remote peers.

### spawned {#spawned}

```gdscript
signal spawned(node: Node)
```

Emitted when a spawnable scene or custom spawn was spawned by the multiplayer authority. Only called on remote peers.
