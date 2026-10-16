# RemoteTransform3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **RemoteTransform3D**

> RemoteTransform3D pushes its own Transform3D to another Node3D derived Node (called the remote node) in the scene.

<details><summary>官方英文描述（中文精译进行中）</summary>

RemoteTransform3D pushes its own Transform3D to another Node3D derived Node (called the remote node) in the scene.

It can be set to update another Node's position, rotation and/or scale. It can use either global or local coordinates.

</details>

## 方法

### force_update_cache {#force_update_cache}

```gdscript
void force_update_cache()
```

RemoteTransform3D caches the remote node. It may not notice if the remote node disappears; force_update_cache() forces it to update the cache again.

### get_remote_node {#get_remote_node}

```gdscript
NodePath get_remote_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_update_position {#get_update_position}

```gdscript
bool get_update_position() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_update_rotation {#get_update_rotation}

```gdscript
bool get_update_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_update_scale {#get_update_scale}

```gdscript
bool get_update_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_global_coordinates {#get_use_global_coordinates}

```gdscript
bool get_use_global_coordinates() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_remote_node {#set_remote_node}

```gdscript
void set_remote_node(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_update_position {#set_update_position}

```gdscript
void set_update_position(update_remote_position: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_update_rotation {#set_update_rotation}

```gdscript
void set_update_rotation(update_remote_rotation: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_update_scale {#set_update_scale}

```gdscript
void set_update_scale(update_remote_scale: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_global_coordinates {#set_use_global_coordinates}

```gdscript
void set_use_global_coordinates(use_global_coordinates: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### remote_path {#remote_path}

```gdscript
remote_path: NodePath
```

- void set_remote_node(value: NodePath)
- NodePath get_remote_node()

The NodePath to the remote node, relative to the RemoteTransform3D's position in the scene.

### use_global_coordinates {#use_global_coordinates}

```gdscript
use_global_coordinates: bool
```

- void set_use_global_coordinates(value: bool)
- bool get_use_global_coordinates()

If true, global coordinates are used. If false, local coordinates are used.

### update_position {#update_position}

```gdscript
update_position: bool
```

- void set_update_position(value: bool)
- bool get_update_position()

If true, the remote node's position is updated.

### update_rotation {#update_rotation}

```gdscript
update_rotation: bool
```

- void set_update_rotation(value: bool)
- bool get_update_rotation()

If true, the remote node's rotation is updated.

### update_scale {#update_scale}

```gdscript
update_scale: bool
```

- void set_update_scale(value: bool)
- bool get_update_scale()

If true, the remote node's scale is updated.
