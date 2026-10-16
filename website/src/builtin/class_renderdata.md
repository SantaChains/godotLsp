# RenderData

**继承**：[Object](class_object.md) → **RenderData**

> Abstract render data object, exists for the duration of rendering a single viewport. See also RenderDataRD, RenderSceneData, and RenderSceneDataRD.

<details><summary>官方英文描述</summary>

Abstract render data object, exists for the duration of rendering a single viewport. See also RenderDataRD, RenderSceneData, and RenderSceneDataRD.

Note: This is an internal rendering server object. Do not instantiate this class from a script.

</details>

## 方法

### get_camera_attributes {#get_camera_attributes}

```gdscript
RID get_camera_attributes() const
```

Returns the RID of the camera attributes object in the RenderingServer being used to render this viewport.

### get_environment {#get_environment}

```gdscript
RID get_environment() const
```

Returns the RID of the environment object in the RenderingServer being used to render this viewport.

### get_render_scene_buffers {#get_render_scene_buffers}

```gdscript
RenderSceneBuffers get_render_scene_buffers() const
```

Returns the RenderSceneBuffers object managing the scene buffers for rendering this viewport.

### get_render_scene_data {#get_render_scene_data}

```gdscript
RenderSceneData get_render_scene_data() const
```

Returns the RenderSceneData object managing this frames scene data.
