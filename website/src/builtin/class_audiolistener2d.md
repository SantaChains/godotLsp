# AudioListener2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **AudioListener2D**

> Once added to the scene tree and enabled using make_current(), this node will override the location sounds are heard from. Only one AudioListener2D can be current. Using make_current() will disable the previous AudioListener2D.

<details><summary>官方英文描述（中文精译进行中）</summary>

Once added to the scene tree and enabled using make_current(), this node will override the location sounds are heard from. Only one AudioListener2D can be current. Using make_current() will disable the previous AudioListener2D.

If there is no active AudioListener2D in the current Viewport, center of the screen will be used as a hearing point for the audio. AudioListener2D needs to be inside SceneTree to function.

</details>

## 方法

### clear_current {#clear_current}

```gdscript
void clear_current()
```

Disables the AudioListener2D. If it's not set as current, this method will have no effect.

### is_current {#is_current}

```gdscript
bool is_current() const
```

Returns true if this AudioListener2D is currently active.

### make_current {#make_current}

```gdscript
void make_current()
```

Makes the AudioListener2D active, setting it as the hearing point for the sounds. If there is already another active AudioListener2D, it will be disabled.

This method will have no effect if the AudioListener2D is not added to SceneTree.
