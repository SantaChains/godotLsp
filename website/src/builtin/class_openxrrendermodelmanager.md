# OpenXRRenderModelManager

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **OpenXRRenderModelManager**

> This helper node will automatically manage displaying render models. It will create new OpenXRRenderModel nodes as controllers and other hand held devices are detected, and remove those nodes when they are deactivated.

<details><summary>官方英文描述（中文精译进行中）</summary>

This helper node will automatically manage displaying render models. It will create new OpenXRRenderModel nodes as controllers and other hand held devices are detected, and remove those nodes when they are deactivated.

Note: If you want more control over this logic you can alternatively call OpenXRRenderModelExtension.render_model_get_all() to obtain a list of active render model ids and create OpenXRRenderModel instances for each render model id provided.

</details>

## 方法

### get_make_local_to_pose {#get_make_local_to_pose}

```gdscript
String get_make_local_to_pose() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tracker {#get_tracker}

```gdscript
OpenXRRenderModelManager.RenderModelTracker get_tracker() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_make_local_to_pose {#set_make_local_to_pose}

```gdscript
void set_make_local_to_pose(make_local_to_pose: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tracker {#set_tracker}

```gdscript
void set_tracker(tracker: OpenXRRenderModelManager.RenderModelTracker)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### tracker {#tracker}

```gdscript
tracker: int
```

- void set_tracker(value: RenderModelTracker)
- RenderModelTracker get_tracker()

Limits render models to the specified tracker. Include: 0 = All render models, 1 = Render models not related to a tracker, 2 = Render models related to the left hand tracker, 3 = Render models related to the right hand tracker.

### make_local_to_pose {#make_local_to_pose}

```gdscript
make_local_to_pose: String
```

- void set_make_local_to_pose(value: String)
- String get_make_local_to_pose()

Position render models local to this pose (this will adjust the position of the render models container node).

## 信号

### render_model_added {#render_model_added}

```gdscript
signal render_model_added(render_model: OpenXRRenderModel)
```

Emitted when a render model node is added as a child to this node.

### render_model_removed {#render_model_removed}

```gdscript
signal render_model_removed(render_model: OpenXRRenderModel)
```

Emitted when a render model child node is about to be removed from this node.

## 枚举

### RenderModelTracker {#enum-RenderModelTracker}

```gdscript
enum RenderModelTracker { RENDER_MODEL_TRACKER_ANY = 0, RENDER_MODEL_TRACKER_NONE_SET = 1, RENDER_MODEL_TRACKER_LEFT_HAND = 2, RENDER_MODEL_TRACKER_RIGHT_HAND = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_MODEL_TRACKER_ANY {#RENDER_MODEL_TRACKER_ANY}

```gdscript
RENDER_MODEL_TRACKER_ANY = 0
```

All active render models are shown regardless of what tracker they relate to.

#### RENDER_MODEL_TRACKER_NONE_SET {#RENDER_MODEL_TRACKER_NONE_SET}

```gdscript
RENDER_MODEL_TRACKER_NONE_SET = 1
```

Only active render models are shown that are not related to any tracker we manage.

#### RENDER_MODEL_TRACKER_LEFT_HAND {#RENDER_MODEL_TRACKER_LEFT_HAND}

```gdscript
RENDER_MODEL_TRACKER_LEFT_HAND = 2
```

Only active render models are shown that are related to the left hand tracker.

#### RENDER_MODEL_TRACKER_RIGHT_HAND {#RENDER_MODEL_TRACKER_RIGHT_HAND}

```gdscript
RENDER_MODEL_TRACKER_RIGHT_HAND = 3
```

Only active render models are shown that are related to the right hand tracker.

---
