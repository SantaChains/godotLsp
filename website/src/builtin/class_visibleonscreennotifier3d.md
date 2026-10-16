# VisibleOnScreenNotifier3D

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **VisibleOnScreenNotifier3D**

> VisibleOnScreenNotifier3D represents a box-shaped region of 3D space. When any part of this region becomes visible on screen or in a Camera3D's view, it will emit a screen_entered signal, and likewise it will emit a screen_exited signal when no part of it remains visible.

<details><summary>官方英文描述（中文精译进行中）</summary>

VisibleOnScreenNotifier3D represents a box-shaped region of 3D space. When any part of this region becomes visible on screen or in a Camera3D's view, it will emit a screen_entered signal, and likewise it will emit a screen_exited signal when no part of it remains visible.

If you want a node to be enabled automatically when this region is visible on screen, use VisibleOnScreenEnabler3D.

Note: VisibleOnScreenNotifier3D uses an approximate heuristic that doesn't take walls and other occlusion into account, unless occlusion culling is used. It also won't function unless Node3D.visible is set to true.

</details>

## 方法

### is_on_screen {#is_on_screen}

```gdscript
bool is_on_screen() const
```

Returns true if the bounding box is on the screen.

Note: It takes one frame for the VisibleOnScreenNotifier3D's visibility to be assessed once added to the scene tree, so this method will always return false right after it is instantiated.

### set_aabb {#set_aabb}

```gdscript
void set_aabb(rect: AABB)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### aabb {#aabb}

```gdscript
aabb: AABB
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 信号

### screen_entered {#screen_entered}

```gdscript
signal screen_entered()
```

Emitted when the VisibleOnScreenNotifier3D enters the screen.

### screen_exited {#screen_exited}

```gdscript
signal screen_exited()
```

Emitted when the VisibleOnScreenNotifier3D exits the screen.
