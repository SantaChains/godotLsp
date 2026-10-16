# OpenXRRenderModel

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **OpenXRRenderModel**

> This node will display an OpenXR render model by accessing the associated GLTF and processes all animation data (if supported by the XR runtime).

<details><summary>官方英文描述（中文精译进行中）</summary>

This node will display an OpenXR render model by accessing the associated GLTF and processes all animation data (if supported by the XR runtime).

Render models were introduced to allow showing the correct model for the controller (or other device) the user has in hand, since the OpenXR action map does not provide information about the hardware used by the user. Note that while the controller (or device) can be somewhat inferred by the bound action map profile, this is a dangerous approach as the user may be using hardware not known at time of development and OpenXR will simply simulate an available interaction profile.

</details>

## 方法

### get_render_model {#get_render_model}

```gdscript
RID get_render_model() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_top_level_path {#get_top_level_path}

```gdscript
String get_top_level_path() const
```

Returns the top level path related to this render model.

### set_render_model {#set_render_model}

```gdscript
void set_render_model(render_model: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### render_model {#render_model}

```gdscript
render_model: RID
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 信号

### render_model_top_level_path_changed {#render_model_top_level_path_changed}

```gdscript
signal render_model_top_level_path_changed()
```

Emitted when the top level path of this render model has changed.
