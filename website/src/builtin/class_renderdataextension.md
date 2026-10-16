# RenderDataExtension

**继承**：[RenderData](class_renderdata.md) → [Object](class_object.md) → **RenderDataExtension**

> This class allows for a RenderData implementation to be made in GDExtension.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class allows for a RenderData implementation to be made in GDExtension.

</details>

## 方法

### _get_camera_attributes {#_get_camera_attributes}

```gdscript
RID _get_camera_attributes() virtual const
```

Implement this in GDExtension to return the RID for the implementation's camera attributes object.

### _get_environment {#_get_environment}

```gdscript
RID _get_environment() virtual const
```

Implement this in GDExtension to return the RID of the implementation's environment object.

### _get_render_scene_buffers {#_get_render_scene_buffers}

```gdscript
RenderSceneBuffers _get_render_scene_buffers() virtual const
```

Implement this in GDExtension to return the implementation's RenderSceneBuffers object.

### _get_render_scene_data {#_get_render_scene_data}

```gdscript
RenderSceneData _get_render_scene_data() virtual const
```

Implement this in GDExtension to return the implementation's RenderSceneDataExtension object.
