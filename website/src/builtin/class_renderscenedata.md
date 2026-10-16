# RenderSceneData

**继承**：[Object](class_object.md) → **RenderSceneData**

> Abstract scene data object, exists for the duration of rendering a single viewport. See also RenderSceneDataRD, RenderData, and RenderDataRD.

<details><summary>官方英文描述（中文精译进行中）</summary>

Abstract scene data object, exists for the duration of rendering a single viewport. See also RenderSceneDataRD, RenderData, and RenderDataRD.

Note: This is an internal rendering server object. Do not instantiate this class from a script.

</details>

## 方法

### get_cam_projection {#get_cam_projection}

```gdscript
Projection get_cam_projection() const
```

Returns the camera projection used to render this frame.

Note: If more than one view is rendered, this will return a combined projection.

### get_cam_transform {#get_cam_transform}

```gdscript
Transform3D get_cam_transform() const
```

Returns the camera transform used to render this frame.

Note: If more than one view is rendered, this will return a centered transform.

### get_uniform_buffer {#get_uniform_buffer}

```gdscript
RID get_uniform_buffer() const
```

Return the RID of the uniform buffer containing the scene data as a UBO.

### get_view_count {#get_view_count}

```gdscript
int get_view_count() const
```

Returns the number of views being rendered.

### get_view_eye_offset {#get_view_eye_offset}

```gdscript
Vector3 get_view_eye_offset(view: int) const
```

Returns the eye offset per view used to render this frame. This is the offset between our camera transform and the eye transform.

### get_view_projection {#get_view_projection}

```gdscript
Projection get_view_projection(view: int) const
```

Returns the view projection per view used to render this frame.

Note: If a single view is rendered, this returns the camera projection. If more than one view is rendered, this will return a projection for the given view including the eye offset.
