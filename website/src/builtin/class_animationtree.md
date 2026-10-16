# AnimationTree

**继承**：[AnimationMixer](class_animationmixer.md) → [Node](class_node.md) → [Object](class_object.md) → **AnimationTree**

> A node used for advanced animation transitions in an AnimationPlayer.

<details><summary>官方英文描述（中文精译进行中）</summary>

A node used for advanced animation transitions in an AnimationPlayer.

Note: When linked with an AnimationPlayer, several properties and methods of the corresponding AnimationPlayer will not function as expected. Playback and transitions should be handled using only the AnimationTree and its constituent AnimationNode(s). The AnimationPlayer node should be used solely for adding, deleting, and editing animations.

</details>

## 方法

### get_advance_expression_base_node {#get_advance_expression_base_node}

```gdscript
NodePath get_advance_expression_base_node() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_animation_player {#get_animation_player}

```gdscript
NodePath get_animation_player() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_callback {#get_process_callback}

```gdscript
AnimationTree.AnimationProcessCallback get_process_callback() const
```

Deprecated: Use AnimationMixer.callback_mode_process instead.

Returns the process notification in which to update animations.

### get_tree_root {#get_tree_root}

```gdscript
AnimationRootNode get_tree_root() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_advance_expression_base_node {#set_advance_expression_base_node}

```gdscript
void set_advance_expression_base_node(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_animation_player {#set_animation_player}

```gdscript
void set_animation_player(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_callback {#set_process_callback}

```gdscript
void set_process_callback(mode: AnimationTree.AnimationProcessCallback)
```

Deprecated: Use AnimationMixer.callback_mode_process instead.

Sets the process notification in which to update animations.

### set_tree_root {#set_tree_root}

```gdscript
void set_tree_root(animation_node: AnimationRootNode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### tree_root {#tree_root}

```gdscript
tree_root: AnimationRootNode
```

- void set_tree_root(value: AnimationRootNode)
- AnimationRootNode get_tree_root()

The root animation node of this AnimationTree. See AnimationRootNode.

### advance_expression_base_node {#advance_expression_base_node}

```gdscript
advance_expression_base_node: NodePath
```

- void set_advance_expression_base_node(value: NodePath)
- NodePath get_advance_expression_base_node()

The path to the Node used to evaluate the AnimationNode Expression if one is not explicitly specified internally.

### anim_player {#anim_player}

```gdscript
anim_player: NodePath
```

- void set_animation_player(value: NodePath)
- NodePath get_animation_player()

The path to the AnimationPlayer used for animating.

## 信号

### animation_player_changed {#animation_player_changed}

```gdscript
signal animation_player_changed()
```

Emitted when the anim_player is changed.

## 枚举

### AnimationProcessCallback {#enum-AnimationProcessCallback}

```gdscript
enum AnimationProcessCallback { ANIMATION_PROCESS_PHYSICS = 0, ANIMATION_PROCESS_IDLE = 1, ANIMATION_PROCESS_MANUAL = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ANIMATION_PROCESS_PHYSICS {#ANIMATION_PROCESS_PHYSICS}

```gdscript
ANIMATION_PROCESS_PHYSICS = 0
```

Deprecated: See AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS.

#### ANIMATION_PROCESS_IDLE {#ANIMATION_PROCESS_IDLE}

```gdscript
ANIMATION_PROCESS_IDLE = 1
```

Deprecated: See AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_IDLE.

#### ANIMATION_PROCESS_MANUAL {#ANIMATION_PROCESS_MANUAL}

```gdscript
ANIMATION_PROCESS_MANUAL = 2
```

Deprecated: See AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_MANUAL.

---
