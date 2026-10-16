# VisibleOnScreenEnabler3D

**继承**：[VisibleOnScreenNotifier3D](class_visibleonscreennotifier3d.md) → [VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **VisibleOnScreenEnabler3D**

> VisibleOnScreenEnabler3D contains a box-shaped region of 3D space and a target node. The target node will be automatically enabled (via its Node.process_mode property) when any part of this region becomes visible on the screen, and automatically disabled otherwise. This can for example be used to activate enemies only when the player approaches them.

<details><summary>官方英文描述（中文精译进行中）</summary>

VisibleOnScreenEnabler3D contains a box-shaped region of 3D space and a target node. The target node will be automatically enabled (via its Node.process_mode property) when any part of this region becomes visible on the screen, and automatically disabled otherwise. This can for example be used to activate enemies only when the player approaches them.

See VisibleOnScreenNotifier3D if you only want to be notified when the region is visible on screen.

Note: VisibleOnScreenEnabler3D uses an approximate heuristic that doesn't take walls and other occlusion into account, unless occlusion culling is used. It also won't function unless Node3D.visible is set to true.

</details>

## 方法

### get_enable_mode {#get_enable_mode}

```gdscript
VisibleOnScreenEnabler3D.EnableMode get_enable_mode()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_node_path {#get_enable_node_path}

```gdscript
NodePath get_enable_node_path()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_mode {#set_enable_mode}

```gdscript
void set_enable_mode(mode: VisibleOnScreenEnabler3D.EnableMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_node_path {#set_enable_node_path}

```gdscript
void set_enable_node_path(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### enable_mode {#enable_mode}

```gdscript
enable_mode: int
```

- void set_enable_mode(value: EnableMode)
- EnableMode get_enable_mode()

Determines how the target node is enabled. Corresponds to ProcessMode. When the node is disabled, it always uses Node.PROCESS_MODE_DISABLED.

### enable_node_path {#enable_node_path}

```gdscript
enable_node_path: NodePath
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 枚举

### EnableMode {#enum-EnableMode}

```gdscript
enum EnableMode { ENABLE_MODE_INHERIT = 0, ENABLE_MODE_ALWAYS = 1, ENABLE_MODE_WHEN_PAUSED = 2 }
```

EnableMode ENABLE_MODE_INHERIT = 0

Corresponds to Node.PROCESS_MODE_INHERIT.

EnableMode ENABLE_MODE_ALWAYS = 1

Corresponds to Node.PROCESS_MODE_ALWAYS.

EnableMode ENABLE_MODE_WHEN_PAUSED = 2

Corresponds to Node.PROCESS_MODE_WHEN_PAUSED.

#### ENABLE_MODE_INHERIT {#ENABLE_MODE_INHERIT}

```gdscript
ENABLE_MODE_INHERIT = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENABLE_MODE_ALWAYS {#ENABLE_MODE_ALWAYS}

```gdscript
ENABLE_MODE_ALWAYS = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENABLE_MODE_WHEN_PAUSED {#ENABLE_MODE_WHEN_PAUSED}

```gdscript
ENABLE_MODE_WHEN_PAUSED = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
