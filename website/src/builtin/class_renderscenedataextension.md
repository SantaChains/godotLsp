# RenderSceneDataExtension

**继承**：[RenderSceneData](class_renderscenedata.md) → [Object](class_object.md) → **RenderSceneDataExtension**

> This class allows for a RenderSceneData implementation to be made in GDExtension.

<details><summary>官方英文描述</summary>

This class allows for a RenderSceneData implementation to be made in GDExtension.

</details>

## 方法

### _get_cam_projection {#_get_cam_projection}

```gdscript
Projection _get_cam_projection() virtual const
```

Implement this in GDExtension to return the camera Projection.

### _get_cam_transform {#_get_cam_transform}

```gdscript
Transform3D _get_cam_transform() virtual const
```

Implement this in GDExtension to return the camera Transform3D.

### _get_uniform_buffer {#_get_uniform_buffer}

```gdscript
RID _get_uniform_buffer() virtual const
```

Implement this in GDExtension to return the RID of the uniform buffer containing the scene data as a UBO.

### _get_view_count {#_get_view_count}

```gdscript
int _get_view_count() virtual const
```

Implement this in GDExtension to return the view count.

### _get_view_eye_offset {#_get_view_eye_offset}

```gdscript
Vector3 _get_view_eye_offset(view: int) virtual const
```

Implement this in GDExtension to return the eye offset for the given view.

### _get_view_projection {#_get_view_projection}

```gdscript
Projection _get_view_projection(view: int) virtual const
```

Implement this in GDExtension to return the view Projection for the given view.
