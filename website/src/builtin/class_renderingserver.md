# RenderingServer

**继承**：[Object](class_object.md) → **RenderingServer**

> The rendering server is the API backend for everything visible. The whole scene system mounts on it to display. The rendering server is completely opaque: the internals are entirely implementation-specific and cannot be accessed.

<details><summary>官方英文描述</summary>

The rendering server is the API backend for everything visible. The whole scene system mounts on it to display. The rendering server is completely opaque: the internals are entirely implementation-specific and cannot be accessed.

The rendering server can be used to bypass the scene/Node system entirely. This can improve performance in cases where the scene system is the bottleneck, but won't improve performance otherwise (for instance, if the GPU is already fully utilized).

Resources are created using the *_create functions. These functions return RIDs which are not references to the objects themselves, but opaque pointers towards these objects.

All objects are drawn to a viewport. You can use the Viewport attached to the SceneTree or you can create one yourself with viewport_create(). When using a custom scenario or canvas, the scenario or canvas needs to be attached to the viewport using viewport_set_scenario() or viewport_attach_canvas().

Scenarios: In 3D, all visual objects must be associated with a scenario. The scenario is a visual representation of the world. If accessing the rendering server from a running game, the scenario can be accessed from the scene tree from any Node3D node with Node3D.get_world_3d(). Otherwise, a scenario can be created with scenario_create().

Similarly, in 2D, a canvas is needed to draw all canvas items.

3D: In 3D, all visible objects are comprised of a resource and an instance. A resource can be a mesh, a particle system, a light, or any other 3D object. In order to be visible resources must be attached to an instance using instance_set_base(). The instance must also be attached to the scenario using instance_set_scenario() in order to be visible. RenderingServer methods that don't have a prefix are usually 3D-specific (but not always).

2D: In 2D, all visible objects are some form of canvas item. In order to be visible, a canvas item needs to be the child of a canvas attached to a viewport, or it needs to be the child of another canvas item that is eventually attached to the canvas. 2D-specific RenderingServer methods generally start with canvas_*.

Headless mode: Starting the engine with the --headless command line argument disables all rendering and window management functions. Most functions from RenderingServer will return dummy values in this case.

</details>

## 方法

### area_light_create {#area_light_create}

```gdscript
RID area_light_create()
```

Creates a new area light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most light_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this area light to an instance using instance_set_base() using the returned RID.

Note: The equivalent node is AreaLight3D.

### bake_render_uv2 {#bake_render_uv2}

```gdscript
typedarray::Image bake_render_uv2(base: RID, material_overrides: typedarray::RID, image_size: Vector2i)
```

Bakes the material data of the Mesh passed in the base parameter with optional material_overrides to a set of Images of size image_size. Returns an array of Images containing material properties as specified in BakeChannels.

### call_on_render_thread {#call_on_render_thread}

```gdscript
void call_on_render_thread(callable: Callable)
```

As the RenderingServer actual logic may run on a separate thread, accessing its internals from the main (or any other) thread will result in errors. To make it easier to run code that can safely access the rendering internals (such as RenderingDevice and similar RD classes), push a callable via this function so it will be executed on the render thread.

### camera_attributes_create {#camera_attributes_create}

```gdscript
RID camera_attributes_create()
```

Creates a camera attributes object and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all camera_attributes_ RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is CameraAttributes.

### camera_attributes_set_auto_exposure {#camera_attributes_set_auto_exposure}

```gdscript
void camera_attributes_set_auto_exposure(camera_attributes: RID, enable: bool, min_sensitivity: float, max_sensitivity: float, speed: float, scale: float)
```

Sets the parameters to use with the auto-exposure effect. These parameters take on the same meaning as their counterparts in CameraAttributes and CameraAttributesPractical.

### camera_attributes_set_dof_blur {#camera_attributes_set_dof_blur}

```gdscript
void camera_attributes_set_dof_blur(camera_attributes: RID, far_enable: bool, far_distance: float, far_transition: float, near_enable: bool, near_distance: float, near_transition: float, amount: float)
```

Sets the parameters to use with the DOF blur effect. These parameters take on the same meaning as their counterparts in CameraAttributesPractical.

### camera_attributes_set_dof_blur_bokeh_shape {#camera_attributes_set_dof_blur_bokeh_shape}

```gdscript
void camera_attributes_set_dof_blur_bokeh_shape(shape: RenderingServer.DOFBokehShape)
```

Sets the shape of the DOF bokeh pattern to shape. Different shapes may be used to achieve artistic effect, or to meet performance targets.

### camera_attributes_set_dof_blur_quality {#camera_attributes_set_dof_blur_quality}

```gdscript
void camera_attributes_set_dof_blur_quality(quality: RenderingServer.DOFBlurQuality, use_jitter: bool)
```

Sets the quality level of the DOF blur effect to quality. use_jitter can be used to jitter samples taken during the blur pass to hide artifacts at the cost of looking more fuzzy.

### camera_attributes_set_exposure {#camera_attributes_set_exposure}

```gdscript
void camera_attributes_set_exposure(camera_attributes: RID, multiplier: float, normalization: float)
```

Sets the exposure values that will be used by the renderers. The normalization amount is used to bake a given Exposure Value (EV) into rendering calculations to reduce the dynamic range of the scene.

The normalization factor can be calculated from exposure value (EV100) as follows:

```gdscript
func get_exposure_normalization(ev100: float):
    return 1.0 / (pow(2.0, ev100) * 1.2)
```

The exposure value can be calculated from aperture (in f-stops), shutter speed (in seconds), and sensitivity (in ISO) as follows:

```gdscript
func get_exposure(aperture: float, shutter_speed: float, sensitivity: float):
    return log((aperture * aperture) / shutter_speed * (100.0 / sensitivity)) / log(2)
```

### camera_create {#camera_create}

```gdscript
RID camera_create()
```

Creates a 3D camera and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all camera_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is Camera3D.

### camera_set_camera_attributes {#camera_set_camera_attributes}

```gdscript
void camera_set_camera_attributes(camera: RID, effects: RID)
```

Sets the camera_attributes created with camera_attributes_create() to the given camera.

### camera_set_compositor {#camera_set_compositor}

```gdscript
void camera_set_compositor(camera: RID, compositor: RID)
```

Sets the compositor used by this camera. Equivalent to Camera3D.compositor.

### camera_set_cull_mask {#camera_set_cull_mask}

```gdscript
void camera_set_cull_mask(camera: RID, layers: int)
```

Sets the cull mask associated with this camera. The cull mask describes which 3D layers are rendered by this camera. Equivalent to Camera3D.cull_mask.

### camera_set_environment {#camera_set_environment}

```gdscript
void camera_set_environment(camera: RID, env: RID)
```

Sets the environment used by this camera. Equivalent to Camera3D.environment.

### camera_set_frustum {#camera_set_frustum}

```gdscript
void camera_set_frustum(camera: RID, size: float, offset: Vector2, z_near: float, z_far: float)
```

Sets camera to use frustum projection. This mode allows adjusting the offset argument to create "tilted frustum" effects.

### camera_set_orthogonal {#camera_set_orthogonal}

```gdscript
void camera_set_orthogonal(camera: RID, size: float, z_near: float, z_far: float)
```

Sets camera to use orthogonal projection, also known as orthographic projection. Objects remain the same size on the screen no matter how far away they are.

### camera_set_perspective {#camera_set_perspective}

```gdscript
void camera_set_perspective(camera: RID, fovy_degrees: float, z_near: float, z_far: float)
```

Sets camera to use perspective projection. Objects on the screen becomes smaller when they are far away.

### camera_set_transform {#camera_set_transform}

```gdscript
void camera_set_transform(camera: RID, transform: Transform3D)
```

Sets Transform3D of camera.

### camera_set_use_vertical_aspect {#camera_set_use_vertical_aspect}

```gdscript
void camera_set_use_vertical_aspect(camera: RID, enable: bool)
```

If true, preserves the horizontal aspect ratio which is equivalent to Camera3D.KEEP_WIDTH. If false, preserves the vertical aspect ratio which is equivalent to Camera3D.KEEP_HEIGHT.

### canvas_create {#canvas_create}

```gdscript
RID canvas_create()
```

Creates a canvas and returns the assigned RID. It can be accessed with the RID that is returned. This RID will be used in all canvas_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Canvas has no Resource or Node equivalent.

### canvas_item_add_animation_slice {#canvas_item_add_animation_slice}

```gdscript
void canvas_item_add_animation_slice(item: RID, animation_length: float, slice_begin: float, slice_end: float, offset: float = 0.0)
```

Subsequent drawing commands will be ignored unless they fall within the specified animation slice. This is a faster way to implement animations that loop on background rather than redrawing constantly.

### canvas_item_add_circle {#canvas_item_add_circle}

```gdscript
void canvas_item_add_circle(item: RID, pos: Vector2, radius: float, color: Color, antialiased: bool = false)
```

Draws a circle on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_circle().

### canvas_item_add_clip_ignore {#canvas_item_add_clip_ignore}

```gdscript
void canvas_item_add_clip_ignore(item: RID, ignore: bool)
```

If ignore is true, ignore clipping on items drawn with this canvas item until this is called again with ignore set to false.

### canvas_item_add_ellipse {#canvas_item_add_ellipse}

```gdscript
void canvas_item_add_ellipse(item: RID, pos: Vector2, major: float, minor: float, color: Color, antialiased: bool = false)
```

Draws an ellipse with semi-major axis major and semi-minor axis minor on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_ellipse().

### canvas_item_add_lcd_texture_rect_region {#canvas_item_add_lcd_texture_rect_region}

```gdscript
void canvas_item_add_lcd_texture_rect_region(item: RID, rect: Rect2, texture: RID, src_rect: Rect2, modulate: Color)
```

See also CanvasItem.draw_lcd_texture_rect_region().

### canvas_item_add_line {#canvas_item_add_line}

```gdscript
void canvas_item_add_line(item: RID, from: Vector2, to: Vector2, color: Color, width: float = -1.0, antialiased: bool = false)
```

Draws a line on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_line().

### canvas_item_add_mesh {#canvas_item_add_mesh}

```gdscript
void canvas_item_add_mesh(item: RID, mesh: RID, transform: Transform2D = Transform2D(1, 0, 0, 1, 0, 0), modulate: Color = Color(1, 1, 1, 1), texture: RID = RID())
```

Draws a mesh created with mesh_create() with given transform, modulate color, and texture. This is used internally by MeshInstance2D.

### canvas_item_add_msdf_texture_rect_region {#canvas_item_add_msdf_texture_rect_region}

```gdscript
void canvas_item_add_msdf_texture_rect_region(item: RID, rect: Rect2, texture: RID, src_rect: Rect2, modulate: Color = Color(1, 1, 1, 1), outline_size: int = 0, px_range: float = 1.0, scale: float = 1.0)
```

See also CanvasItem.draw_msdf_texture_rect_region().

### canvas_item_add_multiline {#canvas_item_add_multiline}

```gdscript
void canvas_item_add_multiline(item: RID, points: PackedVector2Array, colors: PackedColorArray, width: float = -1.0, antialiased: bool = false)
```

Draws a 2D multiline on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_multiline() and CanvasItem.draw_multiline_colors().

### canvas_item_add_multimesh {#canvas_item_add_multimesh}

```gdscript
void canvas_item_add_multimesh(item: RID, mesh: RID, texture: RID = RID())
```

Draws a 2D MultiMesh on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_multimesh().

### canvas_item_add_nine_patch {#canvas_item_add_nine_patch}

```gdscript
void canvas_item_add_nine_patch(item: RID, rect: Rect2, source: Rect2, texture: RID, topleft: Vector2, bottomright: Vector2, x_axis_mode: RenderingServer.NinePatchAxisMode = 0, y_axis_mode: RenderingServer.NinePatchAxisMode = 0, draw_center: bool = true, modulate: Color = Color(1, 1, 1, 1))
```

Draws a nine-patch rectangle on the CanvasItem pointed to by the item RID.

### canvas_item_add_particles {#canvas_item_add_particles}

```gdscript
void canvas_item_add_particles(item: RID, particles: RID, texture: RID)
```

Draws particles on the CanvasItem pointed to by the item RID.

### canvas_item_add_polygon {#canvas_item_add_polygon}

```gdscript
void canvas_item_add_polygon(item: RID, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array = PackedVector2Array(), texture: RID = RID())
```

Draws a 2D polygon on the CanvasItem pointed to by the item RID. If you need more flexibility (such as being able to use bones), use canvas_item_add_triangle_array() instead. See also CanvasItem.draw_polygon().

Note: If you frequently redraw the same polygon with a large number of vertices, consider pre-calculating the triangulation with Geometry2D.triangulate_polygon() and using CanvasItem.draw_mesh(), CanvasItem.draw_multimesh(), or canvas_item_add_triangle_array().

### canvas_item_add_polyline {#canvas_item_add_polyline}

```gdscript
void canvas_item_add_polyline(item: RID, points: PackedVector2Array, colors: PackedColorArray, width: float = -1.0, antialiased: bool = false)
```

Draws a 2D polyline on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_polyline() and CanvasItem.draw_polyline_colors().

### canvas_item_add_primitive {#canvas_item_add_primitive}

```gdscript
void canvas_item_add_primitive(item: RID, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array, texture: RID)
```

Draws a 2D primitive on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_primitive().

### canvas_item_add_rect {#canvas_item_add_rect}

```gdscript
void canvas_item_add_rect(item: RID, rect: Rect2, color: Color, antialiased: bool = false)
```

Draws a rectangle on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_rect().

### canvas_item_add_set_transform {#canvas_item_add_set_transform}

```gdscript
void canvas_item_add_set_transform(item: RID, transform: Transform2D)
```

Sets a Transform2D that will be used to transform subsequent canvas item commands.

### canvas_item_add_texture_rect {#canvas_item_add_texture_rect}

```gdscript
void canvas_item_add_texture_rect(item: RID, rect: Rect2, texture: RID, tile: bool = false, modulate: Color = Color(1, 1, 1, 1), transpose: bool = false)
```

Draws a 2D textured rectangle on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_texture_rect() and Texture2D.draw_rect().

### canvas_item_add_texture_rect_region {#canvas_item_add_texture_rect_region}

```gdscript
void canvas_item_add_texture_rect_region(item: RID, rect: Rect2, texture: RID, src_rect: Rect2, modulate: Color = Color(1, 1, 1, 1), transpose: bool = false, clip_uv: bool = true)
```

Draws the specified region of a 2D textured rectangle on the CanvasItem pointed to by the item RID. See also CanvasItem.draw_texture_rect_region() and Texture2D.draw_rect_region().

### canvas_item_add_triangle_array {#canvas_item_add_triangle_array}

```gdscript
void canvas_item_add_triangle_array(item: RID, indices: PackedInt32Array, points: PackedVector2Array, colors: PackedColorArray, uvs: PackedVector2Array = PackedVector2Array(), bones: PackedInt32Array = PackedInt32Array(), weights: PackedFloat32Array = PackedFloat32Array(), texture: RID = RID(), count: int = -1)
```

Draws a triangle array on the CanvasItem pointed to by the item RID. This is internally used by Line2D and StyleBoxFlat for rendering. canvas_item_add_triangle_array() is highly flexible, but more complex to use than canvas_item_add_polygon().

Note: If count is set to a non-negative value, only the first count * 3 indices (corresponding to count triangles) will be drawn. Otherwise, all indices are drawn.

### canvas_item_attach_skeleton {#canvas_item_attach_skeleton}

```gdscript
void canvas_item_attach_skeleton(item: RID, skeleton: RID)
```

Attaches a skeleton to the CanvasItem. Removes the previous skeleton.

### canvas_item_clear {#canvas_item_clear}

```gdscript
void canvas_item_clear(item: RID)
```

Clears the CanvasItem and removes all commands in it.

### canvas_item_create {#canvas_item_create}

```gdscript
RID canvas_item_create()
```

Creates a new CanvasItem instance and returns its RID. It can be accessed with the RID that is returned. This RID will be used in all canvas_item_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is CanvasItem.

### canvas_item_get_instance_shader_parameter {#canvas_item_get_instance_shader_parameter}

```gdscript
Variant canvas_item_get_instance_shader_parameter(instance: RID, parameter: StringName) const
```

Returns the value of the per-instance shader uniform from the specified canvas item instance. Equivalent to CanvasItem.get_instance_shader_parameter().

### canvas_item_get_instance_shader_parameter_default_value {#canvas_item_get_instance_shader_parameter_default_value}

```gdscript
Variant canvas_item_get_instance_shader_parameter_default_value(instance: RID, parameter: StringName) const
```

Returns the default value of the per-instance shader uniform from the specified canvas item instance. Equivalent to CanvasItem.get_instance_shader_parameter().

### canvas_item_get_instance_shader_parameter_list {#canvas_item_get_instance_shader_parameter_list}

```gdscript
typedarray::Dictionary canvas_item_get_instance_shader_parameter_list(instance: RID) const
```

Returns a dictionary of per-instance shader uniform names of the per-instance shader uniform from the specified canvas item instance.

The returned dictionary is in PropertyInfo format, with the keys name, class_name, type, hint, hint_string, and usage.

### canvas_item_reset_physics_interpolation {#canvas_item_reset_physics_interpolation}

```gdscript
void canvas_item_reset_physics_interpolation(item: RID)
```

Prevents physics interpolation for the current physics tick.

This is useful when moving a canvas item to a new location, to give an instantaneous change rather than interpolation from the previous location.

### canvas_item_set_canvas_group_mode {#canvas_item_set_canvas_group_mode}

```gdscript
void canvas_item_set_canvas_group_mode(item: RID, mode: RenderingServer.CanvasGroupMode, clear_margin: float = 5.0, fit_empty: bool = false, fit_margin: float = 0.0, blur_mipmaps: bool = false)
```

Sets the canvas group mode used during 2D rendering for the canvas item specified by the item RID. For faster but more limited clipping, use canvas_item_set_clip() instead.

Note: The equivalent node functionality is found in CanvasGroup and CanvasItem.clip_children.

### canvas_item_set_clip {#canvas_item_set_clip}

```gdscript
void canvas_item_set_clip(item: RID, clip: bool)
```

If clip is true, makes the canvas item specified by the item RID not draw anything outside of its rect's coordinates. This clipping is fast, but works only with axis-aligned rectangles. This means that rotation is ignored by the clipping rectangle. For more advanced clipping shapes, use canvas_item_set_canvas_group_mode() instead.

Note: The equivalent node functionality is found in Label.clip_text, RichTextLabel (always enabled) and more.

### canvas_item_set_copy_to_backbuffer {#canvas_item_set_copy_to_backbuffer}

```gdscript
void canvas_item_set_copy_to_backbuffer(item: RID, enabled: bool, rect: Rect2)
```

Sets the CanvasItem to copy a rect to the backbuffer.

### canvas_item_set_custom_rect {#canvas_item_set_custom_rect}

```gdscript
void canvas_item_set_custom_rect(item: RID, use_custom_rect: bool, rect: Rect2 = Rect2(0, 0, 0, 0))
```

If use_custom_rect is true, sets the custom visibility rectangle (used for culling) to rect for the canvas item specified by item. Setting a custom visibility rect can reduce CPU load when drawing lots of 2D instances. If use_custom_rect is false, automatically computes a visibility rectangle based on the canvas item's draw commands.

### canvas_item_set_default_texture_filter {#canvas_item_set_default_texture_filter}

```gdscript
void canvas_item_set_default_texture_filter(item: RID, filter: RenderingServer.CanvasItemTextureFilter)
```

Sets the default texture filter mode for the canvas item specified by the item RID. Equivalent to CanvasItem.texture_filter.

### canvas_item_set_default_texture_repeat {#canvas_item_set_default_texture_repeat}

```gdscript
void canvas_item_set_default_texture_repeat(item: RID, repeat: RenderingServer.CanvasItemTextureRepeat)
```

Sets the default texture repeat mode for the canvas item specified by the item RID. Equivalent to CanvasItem.texture_repeat.

### canvas_item_set_distance_field_mode {#canvas_item_set_distance_field_mode}

```gdscript
void canvas_item_set_distance_field_mode(item: RID, enabled: bool)
```

If enabled is true, enables multichannel signed distance field rendering mode for the canvas item specified by the item RID. This is meant to be used for font rendering, or with specially generated images using msdfgen [https://github.com/Chlumsky/msdfgen].

### canvas_item_set_draw_behind_parent {#canvas_item_set_draw_behind_parent}

```gdscript
void canvas_item_set_draw_behind_parent(item: RID, enabled: bool)
```

If enabled is true, draws the canvas item specified by the item RID behind its parent. Equivalent to CanvasItem.show_behind_parent.

### canvas_item_set_draw_index {#canvas_item_set_draw_index}

```gdscript
void canvas_item_set_draw_index(item: RID, index: int)
```

Sets the index for the CanvasItem.

### canvas_item_set_instance_shader_parameter {#canvas_item_set_instance_shader_parameter}

```gdscript
void canvas_item_set_instance_shader_parameter(instance: RID, parameter: StringName, value: Variant)
```

Sets the per-instance shader uniform on the specified canvas item instance. Equivalent to CanvasItem.set_instance_shader_parameter().

### canvas_item_set_interpolated {#canvas_item_set_interpolated}

```gdscript
void canvas_item_set_interpolated(item: RID, interpolated: bool)
```

If interpolated is true, turns on physics interpolation for the canvas item.

### canvas_item_set_light_mask {#canvas_item_set_light_mask}

```gdscript
void canvas_item_set_light_mask(item: RID, mask: int)
```

Sets the light mask for the canvas item specified by the item RID. Equivalent to CanvasItem.light_mask.

### canvas_item_set_material {#canvas_item_set_material}

```gdscript
void canvas_item_set_material(item: RID, material: RID)
```

Sets a new material to the canvas item specified by the item RID. Equivalent to CanvasItem.material.

### canvas_item_set_modulate {#canvas_item_set_modulate}

```gdscript
void canvas_item_set_modulate(item: RID, color: Color)
```

Multiplies the color of the canvas item specified by the item RID, while affecting its children. See also canvas_item_set_self_modulate(). Equivalent to CanvasItem.modulate.

### canvas_item_set_parent {#canvas_item_set_parent}

```gdscript
void canvas_item_set_parent(item: RID, parent: RID)
```

Sets a parent CanvasItem to the CanvasItem. The item will inherit transform, modulation and visibility from its parent, like CanvasItem nodes in the scene tree.

### canvas_item_set_self_modulate {#canvas_item_set_self_modulate}

```gdscript
void canvas_item_set_self_modulate(item: RID, color: Color)
```

Multiplies the color of the canvas item specified by the item RID, without affecting its children. See also canvas_item_set_modulate(). Equivalent to CanvasItem.self_modulate.

### canvas_item_set_sort_children_by_y {#canvas_item_set_sort_children_by_y}

```gdscript
void canvas_item_set_sort_children_by_y(item: RID, enabled: bool)
```

If enabled is true, child nodes with the lowest Y position are drawn before those with a higher Y position. Y-sorting only affects children that inherit from the canvas item specified by the item RID, not the canvas item itself. Equivalent to CanvasItem.y_sort_enabled.

### canvas_item_set_transform {#canvas_item_set_transform}

```gdscript
void canvas_item_set_transform(item: RID, transform: Transform2D)
```

Sets the transform of the canvas item specified by the item RID. This affects where and how the item will be drawn. Child canvas items' transforms are multiplied by their parent's transform. Equivalent to Node2D.transform.

### canvas_item_set_use_parent_material {#canvas_item_set_use_parent_material}

```gdscript
void canvas_item_set_use_parent_material(item: RID, enabled: bool)
```

Sets if the CanvasItem uses its parent's material.

### canvas_item_set_visibility_layer {#canvas_item_set_visibility_layer}

```gdscript
void canvas_item_set_visibility_layer(item: RID, visibility_layer: int)
```

Sets the rendering visibility layer associated with this CanvasItem. Only Viewport nodes with a matching rendering mask will render this CanvasItem.

### canvas_item_set_visibility_notifier {#canvas_item_set_visibility_notifier}

```gdscript
void canvas_item_set_visibility_notifier(item: RID, enable: bool, area: Rect2, enter_callable: Callable, exit_callable: Callable)
```

Sets the given CanvasItem as visibility notifier. area defines the area of detecting visibility. enter_callable is called when the CanvasItem enters the screen, exit_callable is called when the CanvasItem exits the screen. If enable is false, the item will no longer function as notifier.

This method can be used to manually mimic VisibleOnScreenNotifier2D.

### canvas_item_set_visible {#canvas_item_set_visible}

```gdscript
void canvas_item_set_visible(item: RID, visible: bool)
```

Sets the visibility of the CanvasItem.

### canvas_item_set_z_as_relative_to_parent {#canvas_item_set_z_as_relative_to_parent}

```gdscript
void canvas_item_set_z_as_relative_to_parent(item: RID, enabled: bool)
```

If this is enabled, the Z index of the parent will be added to the children's Z index.

### canvas_item_set_z_index {#canvas_item_set_z_index}

```gdscript
void canvas_item_set_z_index(item: RID, z_index: int)
```

Sets the CanvasItem's Z index, i.e. its draw order (lower indexes are drawn first).

### canvas_item_transform_physics_interpolation {#canvas_item_transform_physics_interpolation}

```gdscript
void canvas_item_transform_physics_interpolation(item: RID, transform: Transform2D)
```

Transforms both the current and previous stored transform for a canvas item.

This allows transforming a canvas item without creating a "glitch" in the interpolation, which is particularly useful for large worlds utilizing a shifting origin.

### canvas_light_attach_to_canvas {#canvas_light_attach_to_canvas}

```gdscript
void canvas_light_attach_to_canvas(light: RID, canvas: RID)
```

Attaches the canvas light to the canvas. Removes it from its previous canvas.

### canvas_light_create {#canvas_light_create}

```gdscript
RID canvas_light_create()
```

Creates a canvas light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all canvas_light_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is Light2D.

### canvas_light_occluder_attach_to_canvas {#canvas_light_occluder_attach_to_canvas}

```gdscript
void canvas_light_occluder_attach_to_canvas(occluder: RID, canvas: RID)
```

Attaches a light occluder to the canvas. Removes it from its previous canvas.

### canvas_light_occluder_create {#canvas_light_occluder_create}

```gdscript
RID canvas_light_occluder_create()
```

Creates a light occluder and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all canvas_light_occluder_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is LightOccluder2D.

### canvas_light_occluder_reset_physics_interpolation {#canvas_light_occluder_reset_physics_interpolation}

```gdscript
void canvas_light_occluder_reset_physics_interpolation(occluder: RID)
```

Prevents physics interpolation for the current physics tick.

This is useful when moving an occluder to a new location, to give an instantaneous change rather than interpolation from the previous location.

### canvas_light_occluder_set_as_sdf_collision {#canvas_light_occluder_set_as_sdf_collision}

```gdscript
void canvas_light_occluder_set_as_sdf_collision(occluder: RID, enable: bool)
```

Enables or disables using the light occluder as a signed distance field for 2D particle collision.

### canvas_light_occluder_set_enabled {#canvas_light_occluder_set_enabled}

```gdscript
void canvas_light_occluder_set_enabled(occluder: RID, enabled: bool)
```

Enables or disables light occluder.

### canvas_light_occluder_set_interpolated {#canvas_light_occluder_set_interpolated}

```gdscript
void canvas_light_occluder_set_interpolated(occluder: RID, interpolated: bool)
```

If interpolated is true, turns on physics interpolation for the light occluder.

### canvas_light_occluder_set_light_mask {#canvas_light_occluder_set_light_mask}

```gdscript
void canvas_light_occluder_set_light_mask(occluder: RID, mask: int)
```

The light mask. See LightOccluder2D for more information on light masks.

### canvas_light_occluder_set_polygon {#canvas_light_occluder_set_polygon}

```gdscript
void canvas_light_occluder_set_polygon(occluder: RID, polygon: RID)
```

Sets a light occluder's polygon.

### canvas_light_occluder_set_transform {#canvas_light_occluder_set_transform}

```gdscript
void canvas_light_occluder_set_transform(occluder: RID, transform: Transform2D)
```

Sets a light occluder's Transform2D.

### canvas_light_occluder_transform_physics_interpolation {#canvas_light_occluder_transform_physics_interpolation}

```gdscript
void canvas_light_occluder_transform_physics_interpolation(occluder: RID, transform: Transform2D)
```

Transforms both the current and previous stored transform for a light occluder.

This allows transforming an occluder without creating a "glitch" in the interpolation, which is particularly useful for large worlds utilizing a shifting origin.

### canvas_light_reset_physics_interpolation {#canvas_light_reset_physics_interpolation}

```gdscript
void canvas_light_reset_physics_interpolation(light: RID)
```

Prevents physics interpolation for the current physics tick.

This is useful when moving a canvas item to a new location, to give an instantaneous change rather than interpolation from the previous location.

### canvas_light_set_blend_mode {#canvas_light_set_blend_mode}

```gdscript
void canvas_light_set_blend_mode(light: RID, mode: RenderingServer.CanvasLightBlendMode)
```

Sets the blend mode for the given canvas light to mode. Equivalent to Light2D.blend_mode.

### canvas_light_set_color {#canvas_light_set_color}

```gdscript
void canvas_light_set_color(light: RID, color: Color)
```

Sets the color for a light.

### canvas_light_set_enabled {#canvas_light_set_enabled}

```gdscript
void canvas_light_set_enabled(light: RID, enabled: bool)
```

Enables or disables a canvas light.

### canvas_light_set_energy {#canvas_light_set_energy}

```gdscript
void canvas_light_set_energy(light: RID, energy: float)
```

Sets a canvas light's energy.

### canvas_light_set_height {#canvas_light_set_height}

```gdscript
void canvas_light_set_height(light: RID, height: float)
```

Sets a canvas light's height.

### canvas_light_set_interpolated {#canvas_light_set_interpolated}

```gdscript
void canvas_light_set_interpolated(light: RID, interpolated: bool)
```

If interpolated is true, turns on physics interpolation for the canvas light.

### canvas_light_set_item_cull_mask {#canvas_light_set_item_cull_mask}

```gdscript
void canvas_light_set_item_cull_mask(light: RID, mask: int)
```

The light mask. See LightOccluder2D for more information on light masks.

### canvas_light_set_item_shadow_cull_mask {#canvas_light_set_item_shadow_cull_mask}

```gdscript
void canvas_light_set_item_shadow_cull_mask(light: RID, mask: int)
```

The binary mask used to determine which layers this canvas light's shadows affects. See LightOccluder2D for more information on light masks.

### canvas_light_set_layer_range {#canvas_light_set_layer_range}

```gdscript
void canvas_light_set_layer_range(light: RID, min_layer: int, max_layer: int)
```

The layer range that gets rendered with this light.

### canvas_light_set_mode {#canvas_light_set_mode}

```gdscript
void canvas_light_set_mode(light: RID, mode: RenderingServer.CanvasLightMode)
```

Sets the mode of the canvas light.

### canvas_light_set_shadow_color {#canvas_light_set_shadow_color}

```gdscript
void canvas_light_set_shadow_color(light: RID, color: Color)
```

Sets the color of the canvas light's shadow.

### canvas_light_set_shadow_enabled {#canvas_light_set_shadow_enabled}

```gdscript
void canvas_light_set_shadow_enabled(light: RID, enabled: bool)
```

Enables or disables the canvas light's shadow.

### canvas_light_set_shadow_filter {#canvas_light_set_shadow_filter}

```gdscript
void canvas_light_set_shadow_filter(light: RID, filter: RenderingServer.CanvasLightShadowFilter)
```

Sets the canvas light's shadow's filter.

### canvas_light_set_shadow_smooth {#canvas_light_set_shadow_smooth}

```gdscript
void canvas_light_set_shadow_smooth(light: RID, smooth: float)
```

Smoothens the shadow. The lower, the smoother.

### canvas_light_set_texture {#canvas_light_set_texture}

```gdscript
void canvas_light_set_texture(light: RID, texture: RID)
```

Sets the texture to be used by a PointLight2D. Equivalent to PointLight2D.texture.

### canvas_light_set_texture_offset {#canvas_light_set_texture_offset}

```gdscript
void canvas_light_set_texture_offset(light: RID, offset: Vector2)
```

Sets the offset of a PointLight2D's texture. Equivalent to PointLight2D.offset.

### canvas_light_set_texture_scale {#canvas_light_set_texture_scale}

```gdscript
void canvas_light_set_texture_scale(light: RID, scale: float)
```

Sets the scale factor of a PointLight2D's texture. Equivalent to PointLight2D.texture_scale.

### canvas_light_set_transform {#canvas_light_set_transform}

```gdscript
void canvas_light_set_transform(light: RID, transform: Transform2D)
```

Sets the canvas light's Transform2D.

### canvas_light_set_z_range {#canvas_light_set_z_range}

```gdscript
void canvas_light_set_z_range(light: RID, min_z: int, max_z: int)
```

Sets the Z range of objects that will be affected by this light. Equivalent to Light2D.range_z_min and Light2D.range_z_max.

### canvas_light_transform_physics_interpolation {#canvas_light_transform_physics_interpolation}

```gdscript
void canvas_light_transform_physics_interpolation(light: RID, transform: Transform2D)
```

Transforms both the current and previous stored transform for a canvas light.

This allows transforming a light without creating a "glitch" in the interpolation, which is particularly useful for large worlds utilizing a shifting origin.

### canvas_occluder_polygon_create {#canvas_occluder_polygon_create}

```gdscript
RID canvas_occluder_polygon_create()
```

Creates a new light occluder polygon and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all canvas_occluder_polygon_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is OccluderPolygon2D.

### canvas_occluder_polygon_set_cull_mode {#canvas_occluder_polygon_set_cull_mode}

```gdscript
void canvas_occluder_polygon_set_cull_mode(occluder_polygon: RID, mode: RenderingServer.CanvasOccluderPolygonCullMode)
```

Sets an occluder polygon's cull mode.

### canvas_occluder_polygon_set_shape {#canvas_occluder_polygon_set_shape}

```gdscript
void canvas_occluder_polygon_set_shape(occluder_polygon: RID, shape: PackedVector2Array, closed: bool)
```

Sets the shape of the occluder polygon.

### canvas_set_disable_scale {#canvas_set_disable_scale}

```gdscript
void canvas_set_disable_scale(disable: bool)
```

If disable is true, makes 2D rendering ignore the canvas scale defined for each canvas layer. This affects CanvasLayers with the CanvasLayer.follow_viewport_enabled property set to true.

In the editor, this is set to true by default, and set to false when View > Preview Canvas Scale is enabled at the top of the 2D editor viewport.

Note: Setting this to true does not impact the behavior of CanvasLayer.scale, Node2D.scale, or Control.scale.

### canvas_set_item_mirroring {#canvas_set_item_mirroring}

```gdscript
void canvas_set_item_mirroring(canvas: RID, item: RID, mirroring: Vector2)
```

A copy of the canvas item will be drawn with a local offset of the mirroring.

Note: This is equivalent to calling canvas_set_item_repeat() like canvas_set_item_repeat(item, mirroring, 1), with an additional check ensuring canvas is a parent of item.

### canvas_set_item_repeat {#canvas_set_item_repeat}

```gdscript
void canvas_set_item_repeat(item: RID, repeat_size: Vector2, repeat_times: int)
```

A copy of the canvas item will be drawn with a local offset of the repeat_size by the number of times of the repeat_times. As the repeat_times increases, the copies will spread away from the origin texture.

### canvas_set_modulate {#canvas_set_modulate}

```gdscript
void canvas_set_modulate(canvas: RID, color: Color)
```

Modulates all colors in the given canvas.

### canvas_set_shadow_texture_size {#canvas_set_shadow_texture_size}

```gdscript
void canvas_set_shadow_texture_size(size: int)
```

Sets the ProjectSettings.rendering/2d/shadow_atlas/size to use for Light2D shadow rendering (in pixels). The value is rounded up to the nearest power of 2.

### canvas_texture_create {#canvas_texture_create}

```gdscript
RID canvas_texture_create()
```

Creates a canvas texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all canvas_texture_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method. See also texture_2d_create().

Note: The equivalent resource is CanvasTexture and is only meant to be used in 2D rendering, not 3D.

### canvas_texture_set_channel {#canvas_texture_set_channel}

```gdscript
void canvas_texture_set_channel(canvas_texture: RID, channel: RenderingServer.CanvasTextureChannel, texture: RID)
```

Sets the channel's texture for the canvas texture specified by the canvas_texture RID. Equivalent to CanvasTexture.diffuse_texture, CanvasTexture.normal_texture and CanvasTexture.specular_texture.

### canvas_texture_set_shading_parameters {#canvas_texture_set_shading_parameters}

```gdscript
void canvas_texture_set_shading_parameters(canvas_texture: RID, base_color: Color, shininess: float)
```

Sets the base_color and shininess to use for the canvas texture specified by the canvas_texture RID. Equivalent to CanvasTexture.specular_color and CanvasTexture.specular_shininess.

### canvas_texture_set_texture_filter {#canvas_texture_set_texture_filter}

```gdscript
void canvas_texture_set_texture_filter(canvas_texture: RID, filter: RenderingServer.CanvasItemTextureFilter)
```

Sets the texture filter mode to use for the canvas texture specified by the canvas_texture RID.

### canvas_texture_set_texture_repeat {#canvas_texture_set_texture_repeat}

```gdscript
void canvas_texture_set_texture_repeat(canvas_texture: RID, repeat: RenderingServer.CanvasItemTextureRepeat)
```

Sets the texture repeat mode to use for the canvas texture specified by the canvas_texture RID.

### compositor_create {#compositor_create}

```gdscript
RID compositor_create()
```

Creates a new compositor and adds it to the RenderingServer. It can be accessed with the RID that is returned.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

### compositor_effect_create {#compositor_effect_create}

```gdscript
RID compositor_effect_create()
```

Creates a new rendering effect and adds it to the RenderingServer. It can be accessed with the RID that is returned.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

### compositor_effect_set_callback {#compositor_effect_set_callback}

```gdscript
void compositor_effect_set_callback(effect: RID, callback_type: RenderingServer.CompositorEffectCallbackType, callback: Callable)
```

Sets the callback type (callback_type) and callback method(callback) for this rendering effect.

### compositor_effect_set_enabled {#compositor_effect_set_enabled}

```gdscript
void compositor_effect_set_enabled(effect: RID, enabled: bool)
```

Enables/disables this rendering effect.

### compositor_effect_set_flag {#compositor_effect_set_flag}

```gdscript
void compositor_effect_set_flag(effect: RID, flag: RenderingServer.CompositorEffectFlags, set: bool)
```

Sets the flag (flag) for this rendering effect to true or false (set).

### compositor_set_compositor_effects {#compositor_set_compositor_effects}

```gdscript
void compositor_set_compositor_effects(compositor: RID, effects: typedarray::RID)
```

Sets the compositor effects for the specified compositor RID. effects should be an array containing RIDs created with compositor_effect_create().

### create_local_rendering_device {#create_local_rendering_device}

```gdscript
RenderingDevice create_local_rendering_device() const
```

Creates a RenderingDevice that can be used to do draw and compute operations on a separate thread. Cannot draw to the screen nor share data with the global RenderingDevice.

Note: When using the OpenGL rendering driver or when running in headless mode, this function always returns null.

### debug_canvas_item_get_rect {#debug_canvas_item_get_rect}

```gdscript
Rect2 debug_canvas_item_get_rect(item: RID)
```

Returns the bounding rectangle for a canvas item in local space, as calculated by the renderer. This bound is used internally for culling.

Warning: This function is intended for debugging in the editor, and will pass through and return a zero Rect2 in exported projects.

### decal_create {#decal_create}

```gdscript
RID decal_create()
```

Creates a decal and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all decal_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this decal to an instance using instance_set_base() using the returned RID.

Note: The equivalent node is Decal.

### decal_set_albedo_mix {#decal_set_albedo_mix}

```gdscript
void decal_set_albedo_mix(decal: RID, albedo_mix: float)
```

Sets the albedo_mix in the decal specified by the decal RID. Equivalent to Decal.albedo_mix.

### decal_set_cull_mask {#decal_set_cull_mask}

```gdscript
void decal_set_cull_mask(decal: RID, mask: int)
```

Sets the cull mask in the decal specified by the decal RID. Equivalent to Decal.cull_mask.

### decal_set_distance_fade {#decal_set_distance_fade}

```gdscript
void decal_set_distance_fade(decal: RID, enabled: bool, begin: float, length: float)
```

Sets the distance fade parameters in the decal specified by the decal RID. Equivalent to Decal.distance_fade_enabled, Decal.distance_fade_begin and Decal.distance_fade_length.

### decal_set_emission_energy {#decal_set_emission_energy}

```gdscript
void decal_set_emission_energy(decal: RID, energy: float)
```

Sets the emission energy in the decal specified by the decal RID. Equivalent to Decal.emission_energy.

### decal_set_fade {#decal_set_fade}

```gdscript
void decal_set_fade(decal: RID, above: float, below: float)
```

Sets the upper fade (above) and lower fade (below) in the decal specified by the decal RID. Equivalent to Decal.upper_fade and Decal.lower_fade.

### decal_set_modulate {#decal_set_modulate}

```gdscript
void decal_set_modulate(decal: RID, color: Color)
```

Sets the color multiplier in the decal specified by the decal RID to color. Equivalent to Decal.modulate.

### decal_set_normal_fade {#decal_set_normal_fade}

```gdscript
void decal_set_normal_fade(decal: RID, fade: float)
```

Sets the normal fade in the decal specified by the decal RID. Equivalent to Decal.normal_fade.

### decal_set_size {#decal_set_size}

```gdscript
void decal_set_size(decal: RID, size: Vector3)
```

Sets the size of the decal specified by the decal RID. Equivalent to Decal.size.

### decal_set_texture {#decal_set_texture}

```gdscript
void decal_set_texture(decal: RID, type: RenderingServer.DecalTexture, texture: RID)
```

Sets the texture in the given texture type slot for the specified decal. Equivalent to Decal.set_texture().

### decals_set_filter {#decals_set_filter}

```gdscript
void decals_set_filter(filter: RenderingServer.DecalFilter)
```

Sets the texture filter mode to use when rendering decals. This parameter is global and cannot be set on a per-decal basis.

### directional_light_create {#directional_light_create}

```gdscript
RID directional_light_create()
```

Creates a directional light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most light_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this directional light to an instance using instance_set_base() using the returned RID.

Note: The equivalent node is DirectionalLight3D.

### directional_shadow_atlas_set_size {#directional_shadow_atlas_set_size}

```gdscript
void directional_shadow_atlas_set_size(size: int, is_16bits: bool)
```

Sets the size of the directional light shadows in 3D. See also ProjectSettings.rendering/lights_and_shadows/directional_shadow/size. This parameter is global and cannot be set on a per-viewport basis.

### directional_soft_shadow_filter_set_quality {#directional_soft_shadow_filter_set_quality}

```gdscript
void directional_soft_shadow_filter_set_quality(quality: RenderingServer.ShadowQuality)
```

Sets the filter quality for directional light shadows in 3D. See also ProjectSettings.rendering/lights_and_shadows/directional_shadow/soft_shadow_filter_quality. This parameter is global and cannot be set on a per-viewport basis.

### environment_bake_panorama {#environment_bake_panorama}

```gdscript
Image environment_bake_panorama(environment: RID, bake_irradiance: bool, size: Vector2i)
```

Generates and returns an Image containing the radiance map for the specified environment RID's sky. This supports built-in sky material and custom sky shaders. If bake_irradiance is true, the irradiance map is saved instead of the radiance map. The radiance map is used to render reflected light, while the irradiance map is used to render ambient light. See also sky_bake_panorama().

Note: The image is saved using linear encoding without any tonemapping performed, which means it will look too dark if viewed directly in an image editor.

Note: size should be a 2:1 aspect ratio for the generated panorama to have square pixels. For radiance maps, there is no point in using a height greater than Sky.radiance_size, as it won't increase detail. Irradiance maps only contain low-frequency data, so there is usually no point in going past a size of 128×64 pixels when saving an irradiance map.

### environment_create {#environment_create}

```gdscript
RID environment_create()
```

Creates an environment and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all environment_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is Environment.

### environment_glow_set_use_bicubic_upscale {#environment_glow_set_use_bicubic_upscale}

```gdscript
void environment_glow_set_use_bicubic_upscale(enable: bool)
```

If enable is true, enables bicubic upscaling for glow which improves quality at the cost of performance. Equivalent to ProjectSettings.rendering/environment/glow/upscale_mode.

Note: This setting is only effective when using the Forward+ or Mobile rendering methods, as Compatibility uses a different glow implementation.

### environment_set_adjustment {#environment_set_adjustment}

```gdscript
void environment_set_adjustment(env: RID, enable: bool, brightness: float, contrast: float, saturation: float, use_1d_color_correction: bool, color_correction: RID)
```

Sets the values to be used with the "adjustments" post-process effect. See Environment for more details.

### environment_set_ambient_light {#environment_set_ambient_light}

```gdscript
void environment_set_ambient_light(env: RID, color: Color, ambient: RenderingServer.EnvironmentAmbientSource = 0, energy: float = 1.0, sky_contribution: float = 0.0, reflection_source: RenderingServer.EnvironmentReflectionSource = 0)
```

Sets the values to be used for ambient light rendering. See Environment for more details.

### environment_set_background {#environment_set_background}

```gdscript
void environment_set_background(env: RID, bg: RenderingServer.EnvironmentBG)
```

Sets the environment's background mode. Equivalent to Environment.background_mode.

### environment_set_bg_color {#environment_set_bg_color}

```gdscript
void environment_set_bg_color(env: RID, color: Color)
```

Color displayed for clear areas of the scene. Only effective if using the ENV_BG_COLOR background mode.

### environment_set_bg_energy {#environment_set_bg_energy}

```gdscript
void environment_set_bg_energy(env: RID, multiplier: float, exposure_value: float)
```

Sets the intensity of the background color.

### environment_set_camera_id {#environment_set_camera_id}

```gdscript
void environment_set_camera_id(env: RID, id: int)
```

Sets the camera ID to be used as environment background.

### environment_set_canvas_max_layer {#environment_set_canvas_max_layer}

```gdscript
void environment_set_canvas_max_layer(env: RID, max_layer: int)
```

Sets the maximum layer to use if using Canvas background mode.

### environment_set_fog {#environment_set_fog}

```gdscript
void environment_set_fog(env: RID, enable: bool, light_color: Color, light_energy: float, sun_scatter: float, density: float, height: float, height_density: float, aerial_perspective: float, sky_affect: float, fog_mode: RenderingServer.EnvironmentFogMode = 0)
```

Configures fog for the specified environment RID. See fog_* properties in Environment for more information.

### environment_set_fog_depth {#environment_set_fog_depth}

```gdscript
void environment_set_fog_depth(env: RID, curve: float, begin: float, end: float)
```

Configures fog depth for the specified environment RID. Only has an effect when the fog mode of the environment is ENV_FOG_MODE_DEPTH. See fog_depth_* properties in Environment for more information.

### environment_set_glow {#environment_set_glow}

```gdscript
void environment_set_glow(env: RID, enable: bool, levels: PackedFloat32Array, intensity: float, strength: float, mix: float, bloom_threshold: float, blend_mode: RenderingServer.EnvironmentGlowBlendMode, hdr_bleed_threshold: float, hdr_bleed_scale: float, hdr_luminance_cap: float, glow_map_strength: float, glow_map: RID)
```

Configures glow for the specified environment RID. See glow_* properties in Environment for more information.

### environment_set_sdfgi {#environment_set_sdfgi}

```gdscript
void environment_set_sdfgi(env: RID, enable: bool, cascades: int, min_cell_size: float, y_scale: RenderingServer.EnvironmentSDFGIYScale, use_occlusion: bool, bounce_feedback: float, read_sky: bool, energy: float, normal_bias: float, probe_bias: float)
```

Configures signed distance field global illumination for the specified environment RID. See sdfgi_* properties in Environment for more information.

### environment_set_sdfgi_frames_to_converge {#environment_set_sdfgi_frames_to_converge}

```gdscript
void environment_set_sdfgi_frames_to_converge(frames: RenderingServer.EnvironmentSDFGIFramesToConverge)
```

Sets the number of frames to use for converging signed distance field global illumination. Equivalent to ProjectSettings.rendering/global_illumination/sdfgi/frames_to_converge.

### environment_set_sdfgi_frames_to_update_light {#environment_set_sdfgi_frames_to_update_light}

```gdscript
void environment_set_sdfgi_frames_to_update_light(frames: RenderingServer.EnvironmentSDFGIFramesToUpdateLight)
```

Sets the update speed for dynamic lights' indirect lighting when computing signed distance field global illumination. Equivalent to ProjectSettings.rendering/global_illumination/sdfgi/frames_to_update_lights.

### environment_set_sdfgi_ray_count {#environment_set_sdfgi_ray_count}

```gdscript
void environment_set_sdfgi_ray_count(ray_count: RenderingServer.EnvironmentSDFGIRayCount)
```

Sets the number of rays to throw per frame when computing signed distance field global illumination. Equivalent to ProjectSettings.rendering/global_illumination/sdfgi/probe_ray_count.

### environment_set_sky {#environment_set_sky}

```gdscript
void environment_set_sky(env: RID, sky: RID)
```

Sets the Sky to be used as the environment's background when using BGMode sky. Equivalent to Environment.sky.

### environment_set_sky_custom_fov {#environment_set_sky_custom_fov}

```gdscript
void environment_set_sky_custom_fov(env: RID, scale: float)
```

Sets a custom field of view for the background Sky. Equivalent to Environment.sky_custom_fov.

### environment_set_sky_orientation {#environment_set_sky_orientation}

```gdscript
void environment_set_sky_orientation(env: RID, orientation: Basis)
```

Sets the rotation of the background Sky expressed as a Basis. Equivalent to Environment.sky_rotation, where the rotation vector is used to construct the Basis.

### environment_set_ssao {#environment_set_ssao}

```gdscript
void environment_set_ssao(env: RID, enable: bool, radius: float, intensity: float, power: float, detail: float, horizon: float, sharpness: float, light_affect: float, ao_channel_affect: float)
```

Sets the variables to be used with the screen-space ambient occlusion (SSAO) post-process effect. See Environment for more details.

### environment_set_ssao_quality {#environment_set_ssao_quality}

```gdscript
void environment_set_ssao_quality(quality: RenderingServer.EnvironmentSSAOQuality, half_size: bool, adaptive_target: float, blur_passes: int, fadeout_from: float, fadeout_to: float)
```

Sets the quality level of the screen-space ambient occlusion (SSAO) post-process effect. See Environment for more details.

### environment_set_ssil_quality {#environment_set_ssil_quality}

```gdscript
void environment_set_ssil_quality(quality: RenderingServer.EnvironmentSSILQuality, half_size: bool, adaptive_target: float, blur_passes: int, fadeout_from: float, fadeout_to: float)
```

Sets the quality level of the screen-space indirect lighting (SSIL) post-process effect. See Environment for more details.

### environment_set_ssr {#environment_set_ssr}

```gdscript
void environment_set_ssr(env: RID, enable: bool, max_steps: int, fade_in: float, fade_out: float, depth_tolerance: float)
```

Sets the variables to be used with the screen-space reflections (SSR) post-process effect. See Environment for more details.

### environment_set_ssr_half_size {#environment_set_ssr_half_size}

```gdscript
void environment_set_ssr_half_size(half_size: bool)
```

Sets whether screen-space reflections will be rendered at full or half size. Half size is faster, but may look pixelated or cause flickering.

### environment_set_ssr_roughness_quality {#environment_set_ssr_roughness_quality}

```gdscript
void environment_set_ssr_roughness_quality(quality: RenderingServer.EnvironmentSSRRoughnessQuality)
```

Deprecated: This option no longer does anything.

### environment_set_tonemap {#environment_set_tonemap}

```gdscript
void environment_set_tonemap(env: RID, tone_mapper: RenderingServer.EnvironmentToneMapper, exposure: float, white: float)
```

Sets the variables to be used with the "tonemap" post-process effect. See Environment for more details.

### environment_set_tonemap_agx_contrast {#environment_set_tonemap_agx_contrast}

```gdscript
void environment_set_tonemap_agx_contrast(env: RID, agx_contrast: float)
```

See Environment.tonemap_agx_contrast for more details.

### environment_set_volumetric_fog {#environment_set_volumetric_fog}

```gdscript
void environment_set_volumetric_fog(env: RID, enable: bool, density: float, albedo: Color, emission: Color, emission_energy: float, anisotropy: float, length: float, detail_spread: float, gi_inject: float, temporal_reprojection: bool, temporal_reprojection_amount: float, ambient_inject: float, sky_affect: float)
```

Sets the variables to be used with the volumetric fog post-process effect. See Environment for more details.

### environment_set_volumetric_fog_filter_active {#environment_set_volumetric_fog_filter_active}

```gdscript
void environment_set_volumetric_fog_filter_active(active: bool)
```

Enables filtering of the volumetric fog scattering buffer. This results in much smoother volumes with very few under-sampling artifacts.

### environment_set_volumetric_fog_volume_size {#environment_set_volumetric_fog_volume_size}

```gdscript
void environment_set_volumetric_fog_volume_size(size: int, depth: int)
```

Sets the resolution of the volumetric fog's froxel buffer. size is modified by the screen's aspect ratio and then used to set the width and height of the buffer. While depth is directly used to set the depth of the buffer.

### fog_volume_create {#fog_volume_create}

```gdscript
RID fog_volume_create()
```

Creates a new fog volume and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all fog_volume_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is FogVolume.

### fog_volume_set_material {#fog_volume_set_material}

```gdscript
void fog_volume_set_material(fog_volume: RID, material: RID)
```

Sets the Material of the fog volume. Can be either a FogMaterial or a custom ShaderMaterial.

### fog_volume_set_shape {#fog_volume_set_shape}

```gdscript
void fog_volume_set_shape(fog_volume: RID, shape: RenderingServer.FogVolumeShape)
```

Sets the shape of the fog volume to either FOG_VOLUME_SHAPE_ELLIPSOID, FOG_VOLUME_SHAPE_CONE, FOG_VOLUME_SHAPE_CYLINDER, FOG_VOLUME_SHAPE_BOX or FOG_VOLUME_SHAPE_WORLD.

### fog_volume_set_size {#fog_volume_set_size}

```gdscript
void fog_volume_set_size(fog_volume: RID, size: Vector3)
```

Sets the size of the fog volume when shape is FOG_VOLUME_SHAPE_ELLIPSOID, FOG_VOLUME_SHAPE_CONE, FOG_VOLUME_SHAPE_CYLINDER or FOG_VOLUME_SHAPE_BOX.

### force_draw {#force_draw}

```gdscript
void force_draw(swap_buffers: bool = true, frame_step: float = 0.0)
```

Forces redrawing of all viewports at once. Must be called from the main thread.

### force_sync {#force_sync}

```gdscript
void force_sync()
```

Forces a synchronization between the CPU and GPU, which may be required in certain cases. Only call this when needed, as CPU-GPU synchronization has a performance cost.

### free_rid {#free_rid}

```gdscript
void free_rid(rid: RID)
```

Tries to free an object in the RenderingServer. To avoid memory leaks, this should be called after using an object as memory management does not occur automatically when using RenderingServer directly.

### get_current_rendering_driver_name {#get_current_rendering_driver_name}

```gdscript
String get_current_rendering_driver_name() const
```

Returns the name of the current rendering driver. This can be vulkan, d3d12, metal, opengl3, opengl3_es, or opengl3_angle. See also get_current_rendering_method().

When ProjectSettings.rendering/renderer/rendering_method is forward_plus or mobile, the rendering driver is determined by ProjectSettings.rendering/rendering_device/driver.

When ProjectSettings.rendering/renderer/rendering_method is gl_compatibility, the rendering driver is determined by ProjectSettings.rendering/gl_compatibility/driver.

The rendering driver is also determined by the --rendering-driver command line argument that overrides this project setting, or an automatic fallback that is applied depending on the hardware.

### get_current_rendering_method {#get_current_rendering_method}

```gdscript
String get_current_rendering_method() const
```

Returns the name of the current rendering method. This can be forward_plus, mobile, or gl_compatibility. See also get_current_rendering_driver_name().

The rendering method is determined by ProjectSettings.rendering/renderer/rendering_method, the --rendering-method command line argument that overrides this project setting, or an automatic fallback that is applied depending on the hardware.

### get_default_clear_color {#get_default_clear_color}

```gdscript
Color get_default_clear_color()
```

Returns the default clear color which is used when a specific clear color has not been selected. See also set_default_clear_color().

### get_frame_setup_time_cpu {#get_frame_setup_time_cpu}

```gdscript
float get_frame_setup_time_cpu() const
```

Returns the time taken to setup rendering on the CPU in milliseconds. This value is shared across all viewports and does not require viewport_set_measure_render_time() to be enabled on a viewport to be queried. See also viewport_get_measured_render_time_cpu().

### get_rendering_device {#get_rendering_device}

```gdscript
RenderingDevice get_rendering_device() const
```

Returns the global RenderingDevice.

Note: When using the OpenGL rendering driver or when running in headless mode, this function always returns null.

### get_rendering_info {#get_rendering_info}

```gdscript
int get_rendering_info(info: RenderingServer.RenderingInfo)
```

Returns a statistic about the rendering engine which can be used for performance profiling. See also viewport_get_render_info(), which returns information specific to a viewport.

Note: Only 3D rendering is currently taken into account by some of these values, such as the number of draw calls.

Note: Rendering information is not available until at least 2 frames have been rendered by the engine. If rendering information is not available, get_rendering_info() returns 0. To print rendering information in _ready() successfully, use the following:

```gdscript
func _ready():
    for _i in 2:
        await get_tree().process_frame
    print(RenderingServer.get_rendering_info(RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME))
```

### get_shader_parameter_list {#get_shader_parameter_list}

```gdscript
typedarray::Dictionary get_shader_parameter_list(shader: RID) const
```

Returns the parameters of a shader.

### get_test_cube {#get_test_cube}

```gdscript
RID get_test_cube()
```

Returns the RID of the test cube. This mesh will be created and returned on the first call to get_test_cube(), then it will be cached for subsequent calls. See also make_sphere_mesh().

### get_test_texture {#get_test_texture}

```gdscript
RID get_test_texture()
```

Returns the RID of a 256×256 texture with a testing pattern on it (in Image.FORMAT_RGB8 format). This texture will be created and returned on the first call to get_test_texture(), then it will be cached for subsequent calls. See also get_white_texture().

Example: Get the test texture and apply it to a Sprite2D node:

```gdscript
var texture_rid = RenderingServer.get_test_texture()
var texture = ImageTexture.create_from_image(RenderingServer.texture_2d_get(texture_rid))
$Sprite2D.texture = texture
```

### get_video_adapter_api_version {#get_video_adapter_api_version}

```gdscript
String get_video_adapter_api_version() const
```

Returns the version of the graphics video adapter currently in use (e.g. "1.2.189" for Vulkan, "3.3.0 NVIDIA 510.60.02" for OpenGL). This version may be different from the actual latest version supported by the hardware, as Godot may not always request the latest version. See also OS.get_video_adapter_driver_info().

Note: When running a headless or server binary, this function returns an empty string.

### get_video_adapter_name {#get_video_adapter_name}

```gdscript
String get_video_adapter_name() const
```

Returns the name of the video adapter (e.g. "GeForce GTX 1080/PCIe/SSE2").

Note: When running a headless or server binary, this function returns an empty string.

Note: On the web platform, some browsers such as Firefox may report a different, fixed GPU name such as "GeForce GTX 980" (regardless of the user's actual GPU model). This is done to make fingerprinting more difficult.

### get_video_adapter_type {#get_video_adapter_type}

```gdscript
RenderingDevice.DeviceType get_video_adapter_type() const
```

Returns the type of the video adapter. Since dedicated graphics cards from a given generation will usually be significantly faster than integrated graphics made in the same generation, the device type can be used as a basis for automatic graphics settings adjustment. However, this is not always true, so make sure to provide users with a way to manually override graphics settings.

Note: When using the OpenGL rendering driver or when running in headless mode, this function always returns RenderingDevice.DEVICE_TYPE_OTHER.

### get_video_adapter_vendor {#get_video_adapter_vendor}

```gdscript
String get_video_adapter_vendor() const
```

Returns the vendor of the video adapter (e.g. "NVIDIA Corporation").

Note: When running a headless or server binary, this function returns an empty string.

### get_white_texture {#get_white_texture}

```gdscript
RID get_white_texture()
```

Returns the ID of a 4×4 white texture (in Image.FORMAT_RGB8 format). This texture will be created and returned on the first call to get_white_texture(), then it will be cached for subsequent calls. See also get_test_texture().

Example: Get the white texture and apply it to a Sprite2D node:

```gdscript
var texture_rid = RenderingServer.get_white_texture()
var texture = ImageTexture.create_from_image(RenderingServer.texture_2d_get(texture_rid))
$Sprite2D.texture = texture
```

### gi_set_use_half_resolution {#gi_set_use_half_resolution}

```gdscript
void gi_set_use_half_resolution(half_resolution: bool)
```

If half_resolution is true, renders VoxelGI and SDFGI (Environment.sdfgi_enabled) buffers at halved resolution on each axis (e.g. 960×540 when the viewport size is 1920×1080). This improves performance significantly when VoxelGI or SDFGI is enabled, at the cost of artifacts that may be visible on polygon edges. The loss in quality becomes less noticeable as the viewport resolution increases. LightmapGI rendering is not affected by this setting. Equivalent to ProjectSettings.rendering/global_illumination/gi/use_half_resolution.

### global_shader_parameter_add {#global_shader_parameter_add}

```gdscript
void global_shader_parameter_add(name: StringName, type: RenderingServer.GlobalShaderParameterType, default_value: Variant)
```

Creates a new global shader uniform.

Note: Global shader parameter names are case-sensitive.

### global_shader_parameter_get {#global_shader_parameter_get}

```gdscript
Variant global_shader_parameter_get(name: StringName) const
```

Returns the value of the global shader uniform specified by name.

Note: global_shader_parameter_get() has a large performance penalty as the rendering thread needs to synchronize with the calling thread, which is slow. Do not use this method during gameplay to avoid stuttering. If you need to read values in a script after setting them, consider creating an autoload where you store the values you need to query at the same time you're setting them as global parameters.

### global_shader_parameter_get_list {#global_shader_parameter_get_list}

```gdscript
typedarray::StringName global_shader_parameter_get_list() const
```

Returns the list of global shader uniform names.

Note: global_shader_parameter_get() has a large performance penalty as the rendering thread needs to synchronize with the calling thread, which is slow. Do not use this method during gameplay to avoid stuttering. If you need to read values in a script after setting them, consider creating an autoload where you store the values you need to query at the same time you're setting them as global parameters.

### global_shader_parameter_get_type {#global_shader_parameter_get_type}

```gdscript
RenderingServer.GlobalShaderParameterType global_shader_parameter_get_type(name: StringName) const
```

Returns the type associated to the global shader uniform specified by name.

Note: global_shader_parameter_get() has a large performance penalty as the rendering thread needs to synchronize with the calling thread, which is slow. Do not use this method during gameplay to avoid stuttering. If you need to read values in a script after setting them, consider creating an autoload where you store the values you need to query at the same time you're setting them as global parameters.

### global_shader_parameter_remove {#global_shader_parameter_remove}

```gdscript
void global_shader_parameter_remove(name: StringName)
```

Removes the global shader uniform specified by name.

### global_shader_parameter_set {#global_shader_parameter_set}

```gdscript
void global_shader_parameter_set(name: StringName, value: Variant)
```

Sets the global shader uniform name to value.

### global_shader_parameter_set_override {#global_shader_parameter_set_override}

```gdscript
void global_shader_parameter_set_override(name: StringName, value: Variant)
```

Overrides the global shader uniform name with value. Equivalent to the ShaderGlobalsOverride node.

### has_changed {#has_changed}

```gdscript
bool has_changed() const
```

Returns true if changes have been made to the RenderingServer's data. force_draw() is usually called if this happens.

### has_feature {#has_feature}

```gdscript
bool has_feature(feature: RenderingServer.Features) const
```

Deprecated: This method has not been used since Godot 3.0.

This method does nothing and always returns false.

### has_os_feature {#has_os_feature}

```gdscript
bool has_os_feature(feature: String) const
```

Returns true if the OS supports a certain feature. Features might be s3tc, etc, and etc2.

### instance_attach_object_instance_id {#instance_attach_object_instance_id}

```gdscript
void instance_attach_object_instance_id(instance: RID, id: int)
```

Attaches a unique Object ID to instance. Object ID must be attached to instance for proper culling with instances_cull_aabb(), instances_cull_convex(), and instances_cull_ray().

### instance_attach_skeleton {#instance_attach_skeleton}

```gdscript
void instance_attach_skeleton(instance: RID, skeleton: RID)
```

Attaches a skeleton to an instance. Removes the previous skeleton from the instance.

### instance_create {#instance_create}

```gdscript
RID instance_create()
```

Creates a visual instance and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all instance_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

An instance is a way of placing a 3D object in the scenario. Objects like particles, meshes, reflection probes and decals need to be associated with an instance to be visible in the scenario using instance_set_base().

Note: The equivalent node is VisualInstance3D.

### instance_create2 {#instance_create2}

```gdscript
RID instance_create2(base: RID, scenario: RID)
```

Creates a visual instance, adds it to the RenderingServer, and sets both base and scenario. It can be accessed with the RID that is returned. This RID will be used in all instance_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method. This is a shorthand for using instance_create() and setting the base and scenario manually.

### instance_geometry_get_shader_parameter {#instance_geometry_get_shader_parameter}

```gdscript
Variant instance_geometry_get_shader_parameter(instance: RID, parameter: StringName) const
```

Returns the value of the per-instance shader uniform from the specified 3D geometry instance. Equivalent to GeometryInstance3D.get_instance_shader_parameter().

Note: Per-instance shader parameter names are case-sensitive.

### instance_geometry_get_shader_parameter_default_value {#instance_geometry_get_shader_parameter_default_value}

```gdscript
Variant instance_geometry_get_shader_parameter_default_value(instance: RID, parameter: StringName) const
```

Returns the default value of the per-instance shader uniform from the specified 3D geometry instance. Equivalent to GeometryInstance3D.get_instance_shader_parameter().

### instance_geometry_get_shader_parameter_list {#instance_geometry_get_shader_parameter_list}

```gdscript
typedarray::Dictionary instance_geometry_get_shader_parameter_list(instance: RID) const
```

Returns a dictionary of per-instance shader uniform names of the per-instance shader uniform from the specified 3D geometry instance. The returned dictionary is in PropertyInfo format, with the keys name, class_name, type, hint, hint_string and usage. Equivalent to GeometryInstance3D.get_instance_shader_parameter().

### instance_geometry_set_cast_shadows_setting {#instance_geometry_set_cast_shadows_setting}

```gdscript
void instance_geometry_set_cast_shadows_setting(instance: RID, shadow_casting_setting: RenderingServer.ShadowCastingSetting)
```

Sets the shadow casting setting. Equivalent to GeometryInstance3D.cast_shadow.

### instance_geometry_set_flag {#instance_geometry_set_flag}

```gdscript
void instance_geometry_set_flag(instance: RID, flag: RenderingServer.InstanceFlags, enabled: bool)
```

Sets the flag for a given instance to enabled.

### instance_geometry_set_lightmap {#instance_geometry_set_lightmap}

```gdscript
void instance_geometry_set_lightmap(instance: RID, lightmap: RID, lightmap_uv_scale: Rect2, lightmap_slice: int)
```

Sets the lightmap GI instance to use for the specified 3D geometry instance. The lightmap UV scale for the specified instance (equivalent to GeometryInstance3D.gi_lightmap_scale) and lightmap atlas slice must also be specified.

### instance_geometry_set_lod_bias {#instance_geometry_set_lod_bias}

```gdscript
void instance_geometry_set_lod_bias(instance: RID, lod_bias: float)
```

Sets the level of detail bias to use when rendering the specified 3D geometry instance. Higher values result in higher detail from further away. Equivalent to GeometryInstance3D.lod_bias.

### instance_geometry_set_material_overlay {#instance_geometry_set_material_overlay}

```gdscript
void instance_geometry_set_material_overlay(instance: RID, material: RID)
```

Sets a material that will be rendered for all surfaces on top of active materials for the mesh associated with this instance. Equivalent to GeometryInstance3D.material_overlay.

### instance_geometry_set_material_override {#instance_geometry_set_material_override}

```gdscript
void instance_geometry_set_material_override(instance: RID, material: RID)
```

Sets a material that will override the material for all surfaces on the mesh associated with this instance. Equivalent to GeometryInstance3D.material_override.

### instance_geometry_set_shader_parameter {#instance_geometry_set_shader_parameter}

```gdscript
void instance_geometry_set_shader_parameter(instance: RID, parameter: StringName, value: Variant)
```

Sets the per-instance shader uniform on the specified 3D geometry instance. Equivalent to GeometryInstance3D.set_instance_shader_parameter().

### instance_geometry_set_transparency {#instance_geometry_set_transparency}

```gdscript
void instance_geometry_set_transparency(instance: RID, transparency: float)
```

Sets the transparency for the given geometry instance. Equivalent to GeometryInstance3D.transparency.

A transparency of 0.0 is fully opaque, while 1.0 is fully transparent. Values greater than 0.0 (exclusive) will force the geometry's materials to go through the transparent pipeline, which is slower to render and can exhibit rendering issues due to incorrect transparency sorting. However, unlike using a transparent material, setting transparency to a value greater than 0.0 (exclusive) will not disable shadow rendering.

In spatial shaders, 1.0 - transparency is set as the default value of the ALPHA built-in.

Note: transparency is clamped between 0.0 and 1.0, so this property cannot be used to make transparent materials more opaque than they originally are.

### instance_geometry_set_visibility_range {#instance_geometry_set_visibility_range}

```gdscript
void instance_geometry_set_visibility_range(instance: RID, min: float, max: float, min_margin: float, max_margin: float, fade_mode: RenderingServer.VisibilityRangeFadeMode)
```

Sets the visibility range values for the given geometry instance. Equivalent to GeometryInstance3D.visibility_range_begin and related properties.

### instance_set_base {#instance_set_base}

```gdscript
void instance_set_base(instance: RID, base: RID)
```

Sets the base of the instance. A base can be any of the 3D objects that are created in the RenderingServer that can be displayed. For example, any of the light types, mesh, multimesh, particle system, reflection probe, decal, lightmap, voxel GI and visibility notifiers are all types that can be set as the base of an instance in order to be displayed in the scenario.

### instance_set_blend_shape_weight {#instance_set_blend_shape_weight}

```gdscript
void instance_set_blend_shape_weight(instance: RID, shape: int, weight: float)
```

Sets the weight for a given blend shape associated with this instance.

### instance_set_custom_aabb {#instance_set_custom_aabb}

```gdscript
void instance_set_custom_aabb(instance: RID, aabb: AABB)
```

Sets a custom AABB to use when culling objects from the view frustum. Equivalent to setting GeometryInstance3D.custom_aabb.

### instance_set_extra_visibility_margin {#instance_set_extra_visibility_margin}

```gdscript
void instance_set_extra_visibility_margin(instance: RID, margin: float)
```

Sets a margin to increase the size of the AABB when culling objects from the view frustum. This allows you to avoid culling objects that fall outside the view frustum. Equivalent to GeometryInstance3D.extra_cull_margin.

### instance_set_ignore_culling {#instance_set_ignore_culling}

```gdscript
void instance_set_ignore_culling(instance: RID, enabled: bool)
```

If true, ignores all culling on the specified 3D geometry instance, including frustum culling, occlusion culling, and layer culling. This is not the same as GeometryInstance3D.ignore_occlusion_culling, which only ignores occlusion culling but leaves frustum and layer culling intact.

### instance_set_layer_mask {#instance_set_layer_mask}

```gdscript
void instance_set_layer_mask(instance: RID, mask: int)
```

Sets the render layers that this instance will be drawn to. Equivalent to VisualInstance3D.layers.

### instance_set_pivot_data {#instance_set_pivot_data}

```gdscript
void instance_set_pivot_data(instance: RID, sorting_offset: float, use_aabb_center: bool)
```

Sets the sorting offset and switches between using the bounding box or instance origin for depth sorting.

### instance_set_scenario {#instance_set_scenario}

```gdscript
void instance_set_scenario(instance: RID, scenario: RID)
```

Sets the scenario that the instance is in. The scenario is the 3D world that the objects will be displayed in.

### instance_set_surface_override_material {#instance_set_surface_override_material}

```gdscript
void instance_set_surface_override_material(instance: RID, surface: int, material: RID)
```

Sets the override material of a specific surface. Equivalent to MeshInstance3D.set_surface_override_material().

### instance_set_transform {#instance_set_transform}

```gdscript
void instance_set_transform(instance: RID, transform: Transform3D)
```

Sets the world space transform of the instance. Equivalent to Node3D.global_transform.

### instance_set_visibility_parent {#instance_set_visibility_parent}

```gdscript
void instance_set_visibility_parent(instance: RID, parent: RID)
```

Sets the visibility parent for the given instance. Equivalent to Node3D.visibility_parent.

### instance_set_visible {#instance_set_visible}

```gdscript
void instance_set_visible(instance: RID, visible: bool)
```

Sets whether an instance is drawn or not. Equivalent to Node3D.visible.

### instance_teleport {#instance_teleport}

```gdscript
void instance_teleport(instance: RID)
```

Resets motion vectors and other interpolated values. Use this after teleporting a mesh from one position to another to avoid ghosting artifacts.

### instances_cull_aabb {#instances_cull_aabb}

```gdscript
PackedInt64Array instances_cull_aabb(aabb: AABB, scenario: RID = RID()) const
```

Returns an array of object IDs intersecting with the provided AABB. Only 3D nodes that inherit from VisualInstance3D are considered, such as MeshInstance3D or DirectionalLight3D. Use @GlobalScope.instance_from_id() to obtain the actual nodes. A scenario RID must be provided, which is available in the World3D you want to query. This forces an update for all resources queued to update.

Warning: This function is primarily intended for editor usage. For in-game use cases, prefer physics collision.

### instances_cull_convex {#instances_cull_convex}

```gdscript
PackedInt64Array instances_cull_convex(convex: typedarray::Plane, scenario: RID = RID()) const
```

Returns an array of object IDs intersecting with the provided convex shape. Only 3D nodes that inherit from VisualInstance3D are considered, such as MeshInstance3D or DirectionalLight3D. Use @GlobalScope.instance_from_id() to obtain the actual nodes. A scenario RID must be provided, which is available in the World3D you want to query. This forces an update for all resources queued to update.

Warning: This function is primarily intended for editor usage. For in-game use cases, prefer physics collision.

### instances_cull_ray {#instances_cull_ray}

```gdscript
PackedInt64Array instances_cull_ray(from: Vector3, to: Vector3, scenario: RID = RID()) const
```

Returns an array of object IDs intersecting with the provided 3D ray. Only 3D nodes that inherit from VisualInstance3D are considered, such as MeshInstance3D or DirectionalLight3D. Use @GlobalScope.instance_from_id() to obtain the actual nodes. A scenario RID must be provided, which is available in the World3D you want to query. This forces an update for all resources queued to update.

Warning: This function is primarily intended for editor usage. For in-game use cases, prefer physics collision.

### is_on_render_thread {#is_on_render_thread}

```gdscript
bool is_on_render_thread()
```

Returns true if our code is currently executing on the rendering thread.

### is_render_loop_enabled {#is_render_loop_enabled}

```gdscript
bool is_render_loop_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### light_area_set_normalize_energy {#light_area_set_normalize_energy}

```gdscript
void light_area_set_normalize_energy(light: RID, enable: bool)
```

Defines whether the energy of an AreaLight3D is normalized (divided) by its area. If set to true, changing the size does not affect the total energy output. Equivalent to AreaLight3D.area_normalize_energy.

### light_area_set_size {#light_area_set_size}

```gdscript
void light_area_set_size(light: RID, size: Vector2)
```

Sets the extents (width and height) in meters for this area light. Equivalent to AreaLight3D.area_size.

### light_directional_set_blend_splits {#light_directional_set_blend_splits}

```gdscript
void light_directional_set_blend_splits(light: RID, enable: bool)
```

If true, this directional light will blend between shadow map splits resulting in a smoother transition between them. Equivalent to DirectionalLight3D.directional_shadow_blend_splits.

### light_directional_set_shadow_mode {#light_directional_set_shadow_mode}

```gdscript
void light_directional_set_shadow_mode(light: RID, mode: RenderingServer.LightDirectionalShadowMode)
```

Sets the shadow mode for this directional light. Equivalent to DirectionalLight3D.directional_shadow_mode.

### light_directional_set_sky_mode {#light_directional_set_sky_mode}

```gdscript
void light_directional_set_sky_mode(light: RID, mode: RenderingServer.LightDirectionalSkyMode)
```

If true, this light will not be used for anything except sky shaders. Use this for lights that impact your sky shader that you may want to hide from affecting the rest of the scene. For example, you may want to enable this when the sun in your sky shader falls below the horizon.

### light_omni_set_shadow_mode {#light_omni_set_shadow_mode}

```gdscript
void light_omni_set_shadow_mode(light: RID, mode: RenderingServer.LightOmniShadowMode)
```

Sets whether to use a dual paraboloid or a cubemap for the shadow map. Dual paraboloid is faster but may suffer from artifacts. Equivalent to OmniLight3D.omni_shadow_mode.

### light_projectors_set_filter {#light_projectors_set_filter}

```gdscript
void light_projectors_set_filter(filter: RenderingServer.LightProjectorFilter)
```

Sets the texture filter mode to use when rendering light projectors. This parameter is global and cannot be set on a per-light basis.

### light_set_bake_mode {#light_set_bake_mode}

```gdscript
void light_set_bake_mode(light: RID, bake_mode: RenderingServer.LightBakeMode)
```

Sets the bake mode to use for the specified 3D light. Equivalent to Light3D.light_bake_mode.

### light_set_color {#light_set_color}

```gdscript
void light_set_color(light: RID, color: Color)
```

Sets the color of the light. Equivalent to Light3D.light_color.

### light_set_cull_mask {#light_set_cull_mask}

```gdscript
void light_set_cull_mask(light: RID, mask: int)
```

Sets the cull mask for this 3D light. Lights only affect objects in the selected layers. Equivalent to Light3D.light_cull_mask.

### light_set_distance_fade {#light_set_distance_fade}

```gdscript
void light_set_distance_fade(decal: RID, enabled: bool, begin: float, shadow: float, length: float)
```

Sets the distance fade for this 3D light. This acts as a form of level of detail (LOD) and can be used to improve performance. Equivalent to Light3D.distance_fade_enabled, Light3D.distance_fade_begin, Light3D.distance_fade_shadow, and Light3D.distance_fade_length.

### light_set_max_sdfgi_cascade {#light_set_max_sdfgi_cascade}

```gdscript
void light_set_max_sdfgi_cascade(light: RID, cascade: int)
```

Sets the maximum SDFGI cascade in which the 3D light's indirect lighting is rendered. Higher values allow the light to be rendered in SDFGI further away from the camera.

### light_set_negative {#light_set_negative}

```gdscript
void light_set_negative(light: RID, enable: bool)
```

If true, the 3D light will subtract light instead of adding light. Equivalent to Light3D.light_negative.

### light_set_param {#light_set_param}

```gdscript
void light_set_param(light: RID, param: RenderingServer.LightParam, value: float)
```

Sets the specified 3D light parameter. Equivalent to Light3D.set_param().

### light_set_projector {#light_set_projector}

```gdscript
void light_set_projector(light: RID, texture: RID)
```

Sets the projector texture to use for the specified 3D light. Equivalent to Light3D.light_projector.

### light_set_reverse_cull_face_mode {#light_set_reverse_cull_face_mode}

```gdscript
void light_set_reverse_cull_face_mode(light: RID, enabled: bool)
```

If true, reverses the backface culling of the mesh. This can be useful when you have a flat mesh that has a light behind it. If you need to cast a shadow on both sides of the mesh, set the mesh to use double-sided shadows with instance_geometry_set_cast_shadows_setting(). Equivalent to Light3D.shadow_reverse_cull_face.

### light_set_shadow {#light_set_shadow}

```gdscript
void light_set_shadow(light: RID, enabled: bool)
```

If true, light will cast shadows. Equivalent to Light3D.shadow_enabled.

### light_set_shadow_caster_mask {#light_set_shadow_caster_mask}

```gdscript
void light_set_shadow_caster_mask(light: RID, mask: int)
```

Sets the shadow caster mask for this 3D light. Shadows will only be cast using objects in the selected layers. Equivalent to Light3D.shadow_caster_mask.

### lightmap_create {#lightmap_create}

```gdscript
RID lightmap_create()
```

Creates a new lightmap global illumination instance and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all lightmap_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is LightmapGI.

### lightmap_get_probe_capture_bsp_tree {#lightmap_get_probe_capture_bsp_tree}

```gdscript
PackedInt32Array lightmap_get_probe_capture_bsp_tree(lightmap: RID) const
```

Returns the BSP tree data used for accelerating probe lookups. The BSP data is structured as a series of six signed 32-bit values per BSP node in this order: float plane_x, float plane_y, float plane_z, float plane_distance, int32_t over, int32_t under. An empty leaf is denoted by the value -2147483648 (the minimum 32-bit signed integer). See also lightmap_set_probe_capture_data().

### lightmap_get_probe_capture_points {#lightmap_get_probe_capture_points}

```gdscript
PackedVector3Array lightmap_get_probe_capture_points(lightmap: RID) const
```

Returns the local space positions of each lightmap probe capture point. Keep in mind the lightmap instance may have a non-zero transform, which will affect the position of the probe capture points. See also lightmap_set_probe_capture_data().

### lightmap_get_probe_capture_sh {#lightmap_get_probe_capture_sh}

```gdscript
PackedColorArray lightmap_get_probe_capture_sh(lightmap: RID) const
```

Returns the L0, L1, and L2 spherical harmonics [https://en.wikipedia.org/wiki/Spherical_harmonics] data for each lightmap probe capture point. This is specified as 9 Color values per probe, which means the size of the returned data is always 9 times the number of probe points. See also lightmap_set_probe_capture_data().

### lightmap_get_probe_capture_tetrahedra {#lightmap_get_probe_capture_tetrahedra}

```gdscript
PackedInt32Array lightmap_get_probe_capture_tetrahedra(lightmap: RID) const
```

Returns the tetrahedralization data used for interpolating between lightmap probe capture points. Each tetrahedron is specified as a series of 4 numbers, each being an index into the probe capture points array returned by lightmap_get_probe_capture_points(). See also lightmap_set_probe_capture_data().

### lightmap_set_baked_exposure_normalization {#lightmap_set_baked_exposure_normalization}

```gdscript
void lightmap_set_baked_exposure_normalization(lightmap: RID, baked_exposure: float)
```

Used to inform the renderer what exposure normalization value was used while baking the lightmap. This value will be used and modulated at run time to ensure that the lightmap maintains a consistent level of exposure even if the scene-wide exposure normalization is changed at run time. For more information see camera_attributes_set_exposure().

### lightmap_set_probe_bounds {#lightmap_set_probe_bounds}

```gdscript
void lightmap_set_probe_bounds(lightmap: RID, bounds: AABB)
```

Sets the bounds that this lightmap instance should visually affect, both in terms of static lightmap baking and probe-based global illumination.

### lightmap_set_probe_capture_data {#lightmap_set_probe_capture_data}

```gdscript
void lightmap_set_probe_capture_data(lightmap: RID, points: PackedVector3Array, point_sh: PackedColorArray, tetrahedra: PackedInt32Array, bsp_tree: PackedInt32Array)
```

Sets the probe capture data for the given lightmap instance. See lightmap_get_probe_capture_points(), lightmap_get_probe_capture_sh(), lightmap_get_probe_capture_tetrahedra(), and lightmap_get_probe_capture_bsp_tree() for the expected data formats.

### lightmap_set_probe_capture_update_speed {#lightmap_set_probe_capture_update_speed}

```gdscript
void lightmap_set_probe_capture_update_speed(speed: float)
```

The framerate-independent update speed when representing dynamic object lighting from LightmapProbes. Higher values make dynamic object lighting update faster. Higher values can prevent fast-moving objects from having "outdated" indirect lighting displayed on them, at the cost of possible flickering when an object moves from a bright area to a shaded area. See also ProjectSettings.rendering/lightmapping/probe_capture/update_speed.

### lightmap_set_probe_interior {#lightmap_set_probe_interior}

```gdscript
void lightmap_set_probe_interior(lightmap: RID, interior: bool)
```

Sets whether the lightmap instance should be considered as interior (when interior is true). If the lightmap is marked as interior, environment lighting is ignored when baking lightmaps.

### lightmap_set_textures {#lightmap_set_textures}

```gdscript
void lightmap_set_textures(lightmap: RID, light: RID, uses_sh: bool)
```

Set the textures on the given lightmap GI instance to the texture array pointed to by the light RID. If the lightmap texture was baked with LightmapGI.directional set to true, then uses_sh must also be true.

### lightmaps_set_bicubic_filter {#lightmaps_set_bicubic_filter}

```gdscript
void lightmaps_set_bicubic_filter(enable: bool)
```

Toggles whether a bicubic filter should be used when lightmaps are sampled. This smoothens their appearance at a performance cost.

### make_sphere_mesh {#make_sphere_mesh}

```gdscript
RID make_sphere_mesh(latitudes: int, longitudes: int, radius: float)
```

Returns a mesh of a sphere with the given number of horizontal subdivisions, vertical subdivisions and radius. See also get_test_cube().

### material_create {#material_create}

```gdscript
RID material_create()
```

Creates an empty material and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all material_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is Material.

### material_get_param {#material_get_param}

```gdscript
Variant material_get_param(material: RID, parameter: StringName) const
```

Returns the value of a certain material's parameter.

### material_set_next_pass {#material_set_next_pass}

```gdscript
void material_set_next_pass(material: RID, next_material: RID)
```

Sets an object's next material.

### material_set_param {#material_set_param}

```gdscript
void material_set_param(material: RID, parameter: StringName, value: Variant)
```

Sets a material's parameter.

### material_set_render_priority {#material_set_render_priority}

```gdscript
void material_set_render_priority(material: RID, priority: int)
```

Sets a material's render priority.

### material_set_shader {#material_set_shader}

```gdscript
void material_set_shader(shader_material: RID, shader: RID)
```

Sets a shader material's shader.

### material_set_use_debanding {#material_set_use_debanding}

```gdscript
void material_set_use_debanding(enable: bool)
```

When using the Mobile renderer, material_set_use_debanding() can be used to enable or disable the debanding feature of 3D materials (BaseMaterial3D and ShaderMaterial).

material_set_use_debanding() has no effect when using the Compatibility or Forward+ renderer. In Forward+, Viewport debanding can be used instead.

See also ProjectSettings.rendering/anti_aliasing/quality/use_debanding and viewport_set_use_debanding().

### mesh_add_surface {#mesh_add_surface}

```gdscript
void mesh_add_surface(mesh: RID, surface: Dictionary)
```

Creates a new surface on the given mesh. Equivalent to mesh_add_surface_from_arrays(), but takes a single Dictionary argument instead of separate arguments. The dictionary must follow this structure:

```gdscript
{
    # Required:
    "primitive": RenderingServer.PrimitiveType,
    "format": RenderingServer.ArrayFormat,
    "vertex_data": PackedByteArray,
    "vertex_count": int,
    "aabb": AABB,
    # Optional:
    "attribute_data": PackedByteArray,
    "skin_data": PackedByteArray,
    "index_data": PackedByteArray,
    "index_count": int, # Required if `index_data` is specified.
    "uv_scale": Vector4,
    "lods": [
        # Both values are required for each LOD level.
        {
            "edge_length": float,
            "index_data": PackedByteArray,
        },
    ],
    "bone_aabbs": Array[AABB],
    "blend_shape_data": PackedByteArray,
    "material": Material,
}
```

See also mesh_get_surface(), which returns data in the same structure defined above.

### mesh_add_surface_from_arrays {#mesh_add_surface_from_arrays}

```gdscript
void mesh_add_surface_from_arrays(mesh: RID, primitive: RenderingServer.PrimitiveType, arrays: Array, blend_shapes: Array = [], lods: Dictionary = {}, compress_format: bitfield::RenderingServer.ArrayFormat = 0)
```

Creates a new surface on the given mesh. mesh_get_surface_count() will become the surface index for this new surface.

Surfaces are created to be rendered using a primitive, which may be any of the values defined in PrimitiveType.

The arrays argument is an array of arrays. Each of the Mesh.ARRAY_MAX elements contains an array with some of the mesh data for this surface as described by the corresponding member of ArrayType or null if it is not used by the surface. For example, arrays[0] is the array of vertices. That first vertex sub-array is always required; the others are optional. Adding an index array puts this surface into "index mode" where the vertex and other arrays become the sources of data and the index array defines the vertex order. All sub-arrays must have the same length as the vertex array (or be an exact multiple of the vertex array's length, when multiple elements of a sub-array correspond to a single vertex) or be empty, except for Mesh.ARRAY_INDEX if it is used.

The blend_shapes argument is an array of vertex data for each blend shape. Each element is an array of the same structure as arrays, but Mesh.ARRAY_VERTEX, Mesh.ARRAY_NORMAL, and Mesh.ARRAY_TANGENT are set if and only if they are set in arrays and all other entries are null.

The lods argument is a dictionary with float keys and PackedInt32Array values. Each entry in the dictionary represents an LOD level of the surface, where the value is the Mesh.ARRAY_INDEX array to use for the LOD level and the key is roughly proportional to the distance at which the LOD stats being used. I.e., increasing the key of an LOD also increases the distance that the objects has to be from the camera before the LOD is used.

The compress_format argument is the bitwise OR of, as required: One value of ArrayFormat left shifted by ARRAY_FORMAT_CUSTOMn_SHIFT for each custom channel in use, ARRAY_FLAG_USE_DYNAMIC_UPDATE, ARRAY_FLAG_USE_8_BONE_WEIGHTS, or ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY.

See ArrayMesh.add_surface_from_arrays() and ImporterMesh.add_surface() for higher-level equivalents of this method.

Note: When using indices, it is recommended to only use points, lines, or triangles.

### mesh_clear {#mesh_clear}

```gdscript
void mesh_clear(mesh: RID)
```

Removes all surfaces from a mesh.

### mesh_create {#mesh_create}

```gdscript
RID mesh_create()
```

Creates a new mesh and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all mesh_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this mesh to an instance using instance_set_base() using the returned RID.

Note: The equivalent resource is Mesh.

### mesh_create_from_surfaces {#mesh_create_from_surfaces}

```gdscript
RID mesh_create_from_surfaces(surfaces: typedarray::Dictionary, blend_shape_count: int = 0)
```

Creates a new mesh with predefined surfaces for it and adds the mesh to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all mesh_* RenderingServer functions. This method is more efficient for creating meshes with multiple surfaces compared to creating an empty mesh with mesh_create() and adding surfaces one by one with mesh_add_surface().

Each element in the surfaces array must follow the same structure as described in mesh_add_surface(). The blend_shape_count parameter must match the blend shape data defined in all surfaces.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this mesh to an instance using instance_set_base() using the returned RID.

Note: The equivalent resource is Mesh.

### mesh_get_blend_shape_count {#mesh_get_blend_shape_count}

```gdscript
int mesh_get_blend_shape_count(mesh: RID) const
```

Returns a mesh's blend shape count.

### mesh_get_blend_shape_mode {#mesh_get_blend_shape_mode}

```gdscript
RenderingServer.BlendShapeMode mesh_get_blend_shape_mode(mesh: RID) const
```

Returns a mesh's blend shape mode.

### mesh_get_custom_aabb {#mesh_get_custom_aabb}

```gdscript
AABB mesh_get_custom_aabb(mesh: RID) const
```

Returns a mesh's custom aabb.

### mesh_get_surface {#mesh_get_surface}

```gdscript
Dictionary mesh_get_surface(mesh: RID, surface: int)
```

Returns a mesh's surface as a dictionary following the same structure as described in mesh_add_surface().

### mesh_get_surface_count {#mesh_get_surface_count}

```gdscript
int mesh_get_surface_count(mesh: RID) const
```

Returns a mesh's number of surfaces.

### mesh_set_blend_shape_mode {#mesh_set_blend_shape_mode}

```gdscript
void mesh_set_blend_shape_mode(mesh: RID, mode: RenderingServer.BlendShapeMode)
```

Sets a mesh's blend shape mode.

### mesh_set_custom_aabb {#mesh_set_custom_aabb}

```gdscript
void mesh_set_custom_aabb(mesh: RID, aabb: AABB)
```

Sets a mesh's custom aabb.

### mesh_set_shadow_mesh {#mesh_set_shadow_mesh}

```gdscript
void mesh_set_shadow_mesh(mesh: RID, shadow_mesh: RID)
```

Sets an optional second mesh which can be used for rendering shadows and the depth prepass. Can be used to increase performance by supplying a mesh with fused vertices and only vertex position data (without normals, UVs, colors, etc.).

Note: This mesh must have exactly the same vertex positions as the source mesh (including the source mesh's LODs, if present). If vertex positions differ, then the mesh will not draw correctly.

### mesh_surface_get_arrays {#mesh_surface_get_arrays}

```gdscript
Array mesh_surface_get_arrays(mesh: RID, surface: int) const
```

Returns a mesh's surface's buffer arrays.

### mesh_surface_get_blend_shape_arrays {#mesh_surface_get_blend_shape_arrays}

```gdscript
typedarray::Array mesh_surface_get_blend_shape_arrays(mesh: RID, surface: int) const
```

Returns a mesh's surface's arrays for blend shapes.

### mesh_surface_get_format_attribute_stride {#mesh_surface_get_format_attribute_stride}

```gdscript
int mesh_surface_get_format_attribute_stride(format: bitfield::RenderingServer.ArrayFormat, vertex_count: int) const
```

Returns the stride of the attribute buffer for a mesh with given format.

### mesh_surface_get_format_index_stride {#mesh_surface_get_format_index_stride}

```gdscript
int mesh_surface_get_format_index_stride(format: bitfield::RenderingServer.ArrayFormat, vertex_count: int) const
```

Returns the stride of the index buffer for a mesh with the given format.

### mesh_surface_get_format_normal_tangent_stride {#mesh_surface_get_format_normal_tangent_stride}

```gdscript
int mesh_surface_get_format_normal_tangent_stride(format: bitfield::RenderingServer.ArrayFormat, vertex_count: int) const
```

Returns the stride of the combined normals and tangents for a mesh with given format. Note importantly that, while normals and tangents are in the vertex buffer with vertices, they are only interleaved with each other and so have a different stride than vertex positions.

### mesh_surface_get_format_offset {#mesh_surface_get_format_offset}

```gdscript
int mesh_surface_get_format_offset(format: bitfield::RenderingServer.ArrayFormat, vertex_count: int, array_index: int) const
```

Returns the offset of a given attribute by array_index in the start of its respective buffer.

### mesh_surface_get_format_skin_stride {#mesh_surface_get_format_skin_stride}

```gdscript
int mesh_surface_get_format_skin_stride(format: bitfield::RenderingServer.ArrayFormat, vertex_count: int) const
```

Returns the stride of the skin buffer for a mesh with given format.

### mesh_surface_get_format_vertex_stride {#mesh_surface_get_format_vertex_stride}

```gdscript
int mesh_surface_get_format_vertex_stride(format: bitfield::RenderingServer.ArrayFormat, vertex_count: int) const
```

Returns the stride of the vertex positions for a mesh with given format. Note importantly that vertex positions are stored consecutively and are not interleaved with the other attributes in the vertex buffer (normals and tangents).

### mesh_surface_get_material {#mesh_surface_get_material}

```gdscript
RID mesh_surface_get_material(mesh: RID, surface: int) const
```

Returns a mesh's surface's material.

### mesh_surface_remove {#mesh_surface_remove}

```gdscript
void mesh_surface_remove(mesh: RID, surface: int)
```

Removes the surface at the given index from the Mesh, shifting surfaces with higher index down by one.

### mesh_surface_set_material {#mesh_surface_set_material}

```gdscript
void mesh_surface_set_material(mesh: RID, surface: int, material: RID)
```

Sets a mesh's surface's material.

### mesh_surface_update_attribute_region {#mesh_surface_update_attribute_region}

```gdscript
void mesh_surface_update_attribute_region(mesh: RID, surface: int, offset: int, data: PackedByteArray)
```

Updates the attribute buffer of the mesh surface with the given data. The expected data per attribute is 8 or 12 bytes (4 bytes per float, 2 floats per Vector2, and 3 floats per Vector3) depending on if the mesh is using Vector2 or Vector3 vertices. This value can be determined with mesh_surface_get_format_attribute_stride() instead.

The starting point of the updates can be changed with offset. The value of offset should be a multiple of 12 bytes in most cases to align to each attribute.

A PackedVector3Array of attribute locations can be converted into a PackedByteArray using PackedVector3Array.to_byte_array() for use in data.

### mesh_surface_update_index_region {#mesh_surface_update_index_region}

```gdscript
void mesh_surface_update_index_region(mesh: RID, surface: int, offset: int, data: PackedByteArray)
```

Updates the index buffer of the mesh surface with the given data. The expected data are 16 or 32-bit unsigned integers, which can be determined with mesh_surface_get_format_index_stride().

### mesh_surface_update_skin_region {#mesh_surface_update_skin_region}

```gdscript
void mesh_surface_update_skin_region(mesh: RID, surface: int, offset: int, data: PackedByteArray)
```

Updates the skin buffer of the mesh surface with the given data. The expected data per skin is 8 or 12 bytes (4 bytes per float, 2 floats per Vector2, and 3 floats per Vector3) depending on if the mesh is using Vector2 or Vector3 vertices. This value can be determined with mesh_surface_get_format_skin_stride() instead.

The starting point of the updates can be changed with offset. The value of offset should be a multiple of 12 bytes in most cases to align to each skin.

A PackedVector3Array of skin locations can be converted into a PackedByteArray using PackedVector3Array.to_byte_array() for use in data.

### mesh_surface_update_vertex_region {#mesh_surface_update_vertex_region}

```gdscript
void mesh_surface_update_vertex_region(mesh: RID, surface: int, offset: int, data: PackedByteArray)
```

Updates the vertex buffer of the mesh surface with the given data. The expected data per vertex is 8 or 12 bytes (4 bytes per float, 2 floats per Vector2, and 3 floats per Vector3) depending on if the mesh is using Vector2 or Vector3 vertices. This value can be determined with mesh_surface_get_format_vertex_stride() instead.

The starting point of the updates can be changed with offset. The value of offset should be a multiple of 12 bytes in most cases to align to each vertex.

A PackedVector3Array of vertex locations can be converted into a PackedByteArray using PackedVector3Array.to_byte_array() for use in data.

### multimesh_allocate_data {#multimesh_allocate_data}

```gdscript
void multimesh_allocate_data(multimesh: RID, instances: int, transform_format: RenderingServer.MultimeshTransformFormat, color_format: bool = false, custom_data_format: bool = false, use_indirect: bool = false)
```

Sets up the multimesh using the specified data. The number of instances is set by instances. The format of the instance transforms is set by transform_format, which should be set according to whether the multimesh is meant to be rendered in 2D or 3D. If color_format is true, each instance will have a color associated with it. If custom_data_format is true, each instance will have a custom data vector associated with it. If use_indirect is true, an indirect command buffer will be created for this multimesh, allowing the instance count to be modified directly on the GPU. See also multimesh_get_command_buffer_rd_rid().

### multimesh_create {#multimesh_create}

```gdscript
RID multimesh_create()
```

Creates a new multimesh on the RenderingServer and returns an RID handle. This RID will be used in all multimesh_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this multimesh to an instance using instance_set_base() using the returned RID.

Note: The equivalent resource is MultiMesh.

### multimesh_get_aabb {#multimesh_get_aabb}

```gdscript
AABB multimesh_get_aabb(multimesh: RID) const
```

Calculates and returns the axis-aligned bounding box that encloses all instances within the multimesh.

### multimesh_get_buffer {#multimesh_get_buffer}

```gdscript
PackedFloat32Array multimesh_get_buffer(multimesh: RID) const
```

Returns the MultiMesh data (such as instance transforms, colors, etc.). See multimesh_set_buffer() for details on the returned data.

Note: If the buffer is in the engine's internal cache, it will have to be fetched from GPU memory and possibly decompressed. This means multimesh_get_buffer() is potentially a slow operation and should be avoided whenever possible.

### multimesh_get_buffer_rd_rid {#multimesh_get_buffer_rd_rid}

```gdscript
RID multimesh_get_buffer_rd_rid(multimesh: RID) const
```

Returns the RenderingDevice RID handle of the MultiMesh, which can be used as any other buffer on the Rendering Device.

### multimesh_get_command_buffer_rd_rid {#multimesh_get_command_buffer_rd_rid}

```gdscript
RID multimesh_get_command_buffer_rd_rid(multimesh: RID) const
```

Returns the RenderingDevice RID handle of the MultiMesh command buffer. This RID is only valid if use_indirect is set to true when allocating data through multimesh_allocate_data(). It can be used to directly modify the instance count via buffer.

The data structure is dependent on both how many surfaces the mesh contains and whether it is indexed or not, the buffer has 5 integers in it, with the last unused if the mesh is not indexed.

Each of the values in the buffer correspond to these options:

```
Indexed:
  0 - indexCount;
  1 - instanceCount;
  2 - firstIndex;
  3 - vertexOffset;
  4 - firstInstance;
Non-indexed:
  0 - vertexCount;
  1 - instanceCount;
  2 - firstVertex;
  3 - firstInstance;
  4 - unused;
```

### multimesh_get_custom_aabb {#multimesh_get_custom_aabb}

```gdscript
AABB multimesh_get_custom_aabb(multimesh: RID) const
```

Returns the custom AABB defined for this MultiMesh resource.

### multimesh_get_instance_count {#multimesh_get_instance_count}

```gdscript
int multimesh_get_instance_count(multimesh: RID) const
```

Returns the number of instances allocated for this multimesh.

### multimesh_get_mesh {#multimesh_get_mesh}

```gdscript
RID multimesh_get_mesh(multimesh: RID) const
```

Returns the RID of the mesh that will be used in drawing this multimesh.

### multimesh_get_visible_instances {#multimesh_get_visible_instances}

```gdscript
int multimesh_get_visible_instances(multimesh: RID) const
```

Returns the number of visible instances for this multimesh.

### multimesh_instance_get_color {#multimesh_instance_get_color}

```gdscript
Color multimesh_instance_get_color(multimesh: RID, index: int) const
```

Returns the color by which the specified instance will be modulated.

### multimesh_instance_get_custom_data {#multimesh_instance_get_custom_data}

```gdscript
Color multimesh_instance_get_custom_data(multimesh: RID, index: int) const
```

Returns the custom data associated with the specified instance.

### multimesh_instance_get_transform {#multimesh_instance_get_transform}

```gdscript
Transform3D multimesh_instance_get_transform(multimesh: RID, index: int) const
```

Returns the Transform3D of the specified instance.

### multimesh_instance_get_transform_2d {#multimesh_instance_get_transform_2d}

```gdscript
Transform2D multimesh_instance_get_transform_2d(multimesh: RID, index: int) const
```

Returns the Transform2D of the specified instance. For use when the multimesh is set to use 2D transforms.

### multimesh_instance_reset_physics_interpolation {#multimesh_instance_reset_physics_interpolation}

```gdscript
void multimesh_instance_reset_physics_interpolation(multimesh: RID, index: int)
```

Prevents physics interpolation for the specified instance during the current physics tick.

This is useful when moving an instance to a new location, to give an instantaneous change rather than interpolation from the previous location.

### multimesh_instance_set_color {#multimesh_instance_set_color}

```gdscript
void multimesh_instance_set_color(multimesh: RID, index: int, color: Color)
```

Sets the color by which this instance will be modulated. Equivalent to MultiMesh.set_instance_color().

### multimesh_instance_set_custom_data {#multimesh_instance_set_custom_data}

```gdscript
void multimesh_instance_set_custom_data(multimesh: RID, index: int, custom_data: Color)
```

Sets the custom data for this instance. Custom data is passed as a Color, but is interpreted as a vec4 in the shader. Equivalent to MultiMesh.set_instance_custom_data().

### multimesh_instance_set_transform {#multimesh_instance_set_transform}

```gdscript
void multimesh_instance_set_transform(multimesh: RID, index: int, transform: Transform3D)
```

Sets the Transform3D for this instance. Equivalent to MultiMesh.set_instance_transform().

### multimesh_instance_set_transform_2d {#multimesh_instance_set_transform_2d}

```gdscript
void multimesh_instance_set_transform_2d(multimesh: RID, index: int, transform: Transform2D)
```

Sets the Transform2D for this instance. For use when multimesh is used in 2D. Equivalent to MultiMesh.set_instance_transform_2d().

### multimesh_instances_reset_physics_interpolation {#multimesh_instances_reset_physics_interpolation}

```gdscript
void multimesh_instances_reset_physics_interpolation(multimesh: RID)
```

Prevents physics interpolation for all instances during the current physics tick.

This is useful when moving all instances to new locations, to give instantaneous changes rather than interpolation from the previous locations.

### multimesh_set_buffer {#multimesh_set_buffer}

```gdscript
void multimesh_set_buffer(multimesh: RID, buffer: PackedFloat32Array)
```

Set the entire data to use for drawing the multimesh at once to buffer (such as instance transforms and colors). buffer's size must match the number of instances multiplied by the per-instance data size (which depends on the enabled MultiMesh fields). Otherwise, an error message is printed and nothing is rendered. See also multimesh_get_buffer().

The per-instance data size and expected data order is:

```
2D:
  - Position: 8 floats (8 floats for Transform2D)
  - Position + Vertex color: 12 floats (8 floats for Transform2D, 4 floats for Color)
  - Position + Custom data: 12 floats (8 floats for Transform2D, 4 floats of custom data)
  - Position + Vertex color + Custom data: 16 floats (8 floats for Transform2D, 4 floats for Color, 4 floats of custom data)
3D:
  - Position: 12 floats (12 floats for Transform3D)
  - Position + Vertex color: 16 floats (12 floats for Transform3D, 4 floats for Color)
  - Position + Custom data: 16 floats (12 floats for Transform3D, 4 floats of custom data)
  - Position + Vertex color + Custom data: 20 floats (12 floats for Transform3D, 4 floats for Color, 4 floats of custom data)
```

Instance transforms are in row-major order. Specifically:

- For Transform2D the float-order is: (x.x, y.x, padding_float, origin.x, x.y, y.y, padding_float, origin.y).
- For Transform3D the float-order is: (basis.x.x, basis.y.x, basis.z.x, origin.x, basis.x.y, basis.y.y, basis.z.y, origin.y, basis.x.z, basis.y.z, basis.z.z, origin.z).

### multimesh_set_buffer_interpolated {#multimesh_set_buffer_interpolated}

```gdscript
void multimesh_set_buffer_interpolated(multimesh: RID, buffer: PackedFloat32Array, buffer_previous: PackedFloat32Array)
```

Alternative version of multimesh_set_buffer() for use with physics interpolation.

Takes both an array of current data and an array of data for the previous physics tick.

### multimesh_set_custom_aabb {#multimesh_set_custom_aabb}

```gdscript
void multimesh_set_custom_aabb(multimesh: RID, aabb: AABB)
```

Sets the custom AABB for this MultiMesh resource.

### multimesh_set_mesh {#multimesh_set_mesh}

```gdscript
void multimesh_set_mesh(multimesh: RID, mesh: RID)
```

Sets the mesh to be drawn by the multimesh. Equivalent to MultiMesh.mesh.

### multimesh_set_physics_interpolated {#multimesh_set_physics_interpolated}

```gdscript
void multimesh_set_physics_interpolated(multimesh: RID, interpolated: bool)
```

Turns on and off physics interpolation for this MultiMesh resource.

### multimesh_set_physics_interpolation_quality {#multimesh_set_physics_interpolation_quality}

```gdscript
void multimesh_set_physics_interpolation_quality(multimesh: RID, quality: RenderingServer.MultimeshPhysicsInterpolationQuality)
```

Sets the physics interpolation quality for the MultiMesh.

A value of MULTIMESH_INTERP_QUALITY_FAST gives fast but low quality interpolation, a value of MULTIMESH_INTERP_QUALITY_HIGH gives slower but higher quality interpolation.

### multimesh_set_visible_instances {#multimesh_set_visible_instances}

```gdscript
void multimesh_set_visible_instances(multimesh: RID, visible: int)
```

Sets the number of instances visible at a given time. If -1, all instances that have been allocated are drawn. Equivalent to MultiMesh.visible_instance_count.

### occluder_create {#occluder_create}

```gdscript
RID occluder_create()
```

Creates an occluder instance and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all occluder_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is Occluder3D (not to be confused with the OccluderInstance3D node).

### occluder_set_mesh {#occluder_set_mesh}

```gdscript
void occluder_set_mesh(occluder: RID, vertices: PackedVector3Array, indices: PackedInt32Array)
```

Sets the mesh data for the given occluder RID, which controls the shape of the occlusion culling that will be performed.

### omni_light_create {#omni_light_create}

```gdscript
RID omni_light_create()
```

Creates a new omni light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most light_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this omni light to an instance using instance_set_base() using the returned RID.

Note: The equivalent node is OmniLight3D.

### particles_collision_create {#particles_collision_create}

```gdscript
RID particles_collision_create()
```

Creates a new 3D GPU particle collision or attractor and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most particles_collision_* RenderingServer functions.

Note: The equivalent nodes are GPUParticlesCollision3D and GPUParticlesAttractor3D.

### particles_collision_height_field_update {#particles_collision_height_field_update}

```gdscript
void particles_collision_height_field_update(particles_collision: RID)
```

Requests an update for the 3D GPU particle collision heightfield. This may be automatically called by the 3D GPU particle collision heightfield depending on its GPUParticlesCollisionHeightField3D.update_mode.

### particles_collision_set_attractor_attenuation {#particles_collision_set_attractor_attenuation}

```gdscript
void particles_collision_set_attractor_attenuation(particles_collision: RID, curve: float)
```

Sets the attenuation curve for the 3D GPU particles attractor specified by the particles_collision RID. Only used for attractors, not colliders. Equivalent to GPUParticlesAttractor3D.attenuation.

### particles_collision_set_attractor_directionality {#particles_collision_set_attractor_directionality}

```gdscript
void particles_collision_set_attractor_directionality(particles_collision: RID, amount: float)
```

Sets the directionality amount for the 3D GPU particles attractor specified by the particles_collision RID. Only used for attractors, not colliders. Equivalent to GPUParticlesAttractor3D.directionality.

### particles_collision_set_attractor_strength {#particles_collision_set_attractor_strength}

```gdscript
void particles_collision_set_attractor_strength(particles_collision: RID, strength: float)
```

Sets the strength for the 3D GPU particles attractor specified by the particles_collision RID. Only used for attractors, not colliders. Equivalent to GPUParticlesAttractor3D.strength.

### particles_collision_set_box_extents {#particles_collision_set_box_extents}

```gdscript
void particles_collision_set_box_extents(particles_collision: RID, extents: Vector3)
```

Sets the extents for the 3D GPU particles collision by the particles_collision RID. Equivalent to GPUParticlesCollisionBox3D.size, GPUParticlesCollisionSDF3D.size, GPUParticlesCollisionHeightField3D.size, GPUParticlesAttractorBox3D.size or GPUParticlesAttractorVectorField3D.size depending on the particles_collision type.

### particles_collision_set_collision_type {#particles_collision_set_collision_type}

```gdscript
void particles_collision_set_collision_type(particles_collision: RID, type: RenderingServer.ParticlesCollisionType)
```

Sets the collision or attractor shape type for the 3D GPU particles collision or attractor specified by the particles_collision RID.

### particles_collision_set_cull_mask {#particles_collision_set_cull_mask}

```gdscript
void particles_collision_set_cull_mask(particles_collision: RID, mask: int)
```

Sets the cull mask for the 3D GPU particles collision or attractor specified by the particles_collision RID. Equivalent to GPUParticlesCollision3D.cull_mask or GPUParticlesAttractor3D.cull_mask depending on the particles_collision type.

### particles_collision_set_field_texture {#particles_collision_set_field_texture}

```gdscript
void particles_collision_set_field_texture(particles_collision: RID, texture: RID)
```

Sets the signed distance field texture for the 3D GPU particles collision specified by the particles_collision RID. Equivalent to GPUParticlesCollisionSDF3D.texture or GPUParticlesAttractorVectorField3D.texture depending on the particles_collision type.

### particles_collision_set_height_field_mask {#particles_collision_set_height_field_mask}

```gdscript
void particles_collision_set_height_field_mask(particles_collision: RID, mask: int)
```

Sets the heightfield mask for the 3D GPU particles heightfield collision specified by the particles_collision RID. Equivalent to GPUParticlesCollisionHeightField3D.heightfield_mask.

### particles_collision_set_height_field_resolution {#particles_collision_set_height_field_resolution}

```gdscript
void particles_collision_set_height_field_resolution(particles_collision: RID, resolution: RenderingServer.ParticlesCollisionHeightfieldResolution)
```

Sets the heightmap resolution for the 3D GPU particles heightfield collision specified by the particles_collision RID. Equivalent to GPUParticlesCollisionHeightField3D.resolution.

### particles_collision_set_sphere_radius {#particles_collision_set_sphere_radius}

```gdscript
void particles_collision_set_sphere_radius(particles_collision: RID, radius: float)
```

Sets the radius for the 3D GPU particles sphere collision or attractor specified by the particles_collision RID. Equivalent to GPUParticlesCollisionSphere3D.radius or GPUParticlesAttractorSphere3D.radius depending on the particles_collision type.

### particles_create {#particles_create}

```gdscript
RID particles_create()
```

Creates a GPU-based particle system and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all particles_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach these particles to an instance using instance_set_base() using the returned RID.

Note: The equivalent nodes are GPUParticles2D and GPUParticles3D.

Note: All particles_* methods only apply to GPU-based particles, not CPU-based particles. CPUParticles2D and CPUParticles3D do not have equivalent RenderingServer functions available, as these use MultiMeshInstance2D and MultiMeshInstance3D under the hood (see multimesh_* methods).

### particles_emit {#particles_emit}

```gdscript
void particles_emit(particles: RID, transform: Transform3D, velocity: Vector3, color: Color, custom: Color, emit_flags: int)
```

Manually emits particles from the particles instance.

### particles_get_current_aabb {#particles_get_current_aabb}

```gdscript
AABB particles_get_current_aabb(particles: RID)
```

Calculates and returns the axis-aligned bounding box that contains all the particles. Equivalent to GPUParticles3D.capture_aabb().

### particles_get_emitting {#particles_get_emitting}

```gdscript
bool particles_get_emitting(particles: RID)
```

Returns true if particles are currently set to emitting.

### particles_is_inactive {#particles_is_inactive}

```gdscript
bool particles_is_inactive(particles: RID)
```

Returns true if particles are not emitting and particles are set to inactive.

### particles_request_process {#particles_request_process}

```gdscript
void particles_request_process(particles: RID)
```

Add particle system to list of particle systems that need to be updated. Update will take place on the next frame, or on the next call to instances_cull_aabb(), instances_cull_convex(), or instances_cull_ray().

### particles_request_process_time {#particles_request_process_time}

```gdscript
void particles_request_process_time(particles: RID, process_time: float, process_time_residual: float = 0.0)
```

Requests the particles to process for extra process time during a single frame.

process_time defines the time that the particles will process while emitting is on. process_time_residual defines the time that particles will process with emitting turned off for the simulation. When combined with the particles' speed scale set to 0.0, this is useful to be able to seek a particle system timeline.

### particles_restart {#particles_restart}

```gdscript
void particles_restart(particles: RID)
```

Reset the particles on the next update. Equivalent to GPUParticles3D.restart().

### particles_set_amount {#particles_set_amount}

```gdscript
void particles_set_amount(particles: RID, amount: int)
```

Sets the number of particles to be drawn and allocates the memory for them. Equivalent to GPUParticles3D.amount.

### particles_set_amount_ratio {#particles_set_amount_ratio}

```gdscript
void particles_set_amount_ratio(particles: RID, ratio: float)
```

Sets the amount ratio for particles to be emitted. Equivalent to GPUParticles3D.amount_ratio.

### particles_set_collision_base_size {#particles_set_collision_base_size}

```gdscript
void particles_set_collision_base_size(particles: RID, size: float)
```

Sets the base size for particle collision. Equivalent to GPUParticles3D.collision_base_size.

### particles_set_custom_aabb {#particles_set_custom_aabb}

```gdscript
void particles_set_custom_aabb(particles: RID, aabb: AABB)
```

Sets a custom axis-aligned bounding box for the particle system. Equivalent to GPUParticles3D.visibility_aabb.

### particles_set_draw_order {#particles_set_draw_order}

```gdscript
void particles_set_draw_order(particles: RID, order: RenderingServer.ParticlesDrawOrder)
```

Sets the draw order of the particles. Equivalent to GPUParticles3D.draw_order.

### particles_set_draw_pass_mesh {#particles_set_draw_pass_mesh}

```gdscript
void particles_set_draw_pass_mesh(particles: RID, pass: int, mesh: RID)
```

Sets the mesh to be used for the specified draw pass. Equivalent to GPUParticles3D.draw_pass_1, GPUParticles3D.draw_pass_2, GPUParticles3D.draw_pass_3, and GPUParticles3D.draw_pass_4.

### particles_set_draw_passes {#particles_set_draw_passes}

```gdscript
void particles_set_draw_passes(particles: RID, count: int)
```

Sets the number of draw passes to use. Equivalent to GPUParticles3D.draw_passes.

### particles_set_emission_transform {#particles_set_emission_transform}

```gdscript
void particles_set_emission_transform(particles: RID, transform: Transform3D)
```

Sets the Transform3D that will be used by the particles when they first emit.

### particles_set_emitter_velocity {#particles_set_emitter_velocity}

```gdscript
void particles_set_emitter_velocity(particles: RID, velocity: Vector3)
```

Sets the velocity of a particle node, that will be used by ParticleProcessMaterial.inherit_velocity_ratio.

### particles_set_emitting {#particles_set_emitting}

```gdscript
void particles_set_emitting(particles: RID, emitting: bool)
```

If true, particles will emit over time. Setting to false does not reset the particles, but only stops their emission. Equivalent to GPUParticles3D.emitting.

### particles_set_explosiveness_ratio {#particles_set_explosiveness_ratio}

```gdscript
void particles_set_explosiveness_ratio(particles: RID, ratio: float)
```

Sets the explosiveness ratio. Equivalent to GPUParticles3D.explosiveness.

### particles_set_fixed_fps {#particles_set_fixed_fps}

```gdscript
void particles_set_fixed_fps(particles: RID, fps: int)
```

Sets the frame rate that the particle system rendering will be fixed to. Equivalent to GPUParticles3D.fixed_fps.

### particles_set_fractional_delta {#particles_set_fractional_delta}

```gdscript
void particles_set_fractional_delta(particles: RID, enable: bool)
```

If true, uses fractional delta which smooths the movement of the particles. Equivalent to GPUParticles3D.fract_delta.

### particles_set_interp_to_end {#particles_set_interp_to_end}

```gdscript
void particles_set_interp_to_end(particles: RID, factor: float)
```

Sets the value that informs a ParticleProcessMaterial to rush all particles towards the end of their lifetime.

### particles_set_interpolate {#particles_set_interpolate}

```gdscript
void particles_set_interpolate(particles: RID, enable: bool)
```

Sets whether particles should use interpolation between fixed steps. Equivalent to GPUParticles3D.interpolate.

### particles_set_lifetime {#particles_set_lifetime}

```gdscript
void particles_set_lifetime(particles: RID, lifetime: float)
```

Sets the lifetime of each particle in the system. Equivalent to GPUParticles3D.lifetime.

### particles_set_mode {#particles_set_mode}

```gdscript
void particles_set_mode(particles: RID, mode: RenderingServer.ParticlesMode)
```

Sets whether the GPU particles specified by the particles RID should be rendered in 2D or 3D according to mode.

### particles_set_one_shot {#particles_set_one_shot}

```gdscript
void particles_set_one_shot(particles: RID, one_shot: bool)
```

If true, particles will emit once and then stop. Equivalent to GPUParticles3D.one_shot.

### particles_set_pre_process_time {#particles_set_pre_process_time}

```gdscript
void particles_set_pre_process_time(particles: RID, time: float)
```

Sets the preprocess time for the particles' animation. This lets you delay starting an animation until after the particles have begun emitting. Equivalent to GPUParticles3D.preprocess.

### particles_set_process_material {#particles_set_process_material}

```gdscript
void particles_set_process_material(particles: RID, material: RID)
```

Sets the material for processing the particles.

Note: This is not the material used to draw the materials. Equivalent to GPUParticles3D.process_material.

### particles_set_randomness_ratio {#particles_set_randomness_ratio}

```gdscript
void particles_set_randomness_ratio(particles: RID, ratio: float)
```

Sets the emission randomness ratio. This randomizes the emission of particles within their phase. Equivalent to GPUParticles3D.randomness.

### particles_set_speed_scale {#particles_set_speed_scale}

```gdscript
void particles_set_speed_scale(particles: RID, scale: float)
```

Sets the speed scale of the particle system. Equivalent to GPUParticles3D.speed_scale.

### particles_set_subemitter {#particles_set_subemitter}

```gdscript
void particles_set_subemitter(particles: RID, subemitter_particles: RID)
```

Sets the subemitter particles for the particle system. Equivalent to GPUParticles3D.sub_emitter.

### particles_set_trail_bind_poses {#particles_set_trail_bind_poses}

```gdscript
void particles_set_trail_bind_poses(particles: RID, bind_poses: typedarray::Transform3D)
```

Sets the trail bind poses for the particle system. This specified as an array of Transform3Ds representing the bind pose for each draw pass. See GPUParticles3D.draw_skin, Skin.get_bind_count(), and Skin.get_bind_pose(). Set the value for each draw pass to Transform3D.IDENTITY to use the default behavior, which is what built-in trails use (RibbonTrailMesh and TubeTrailMesh).

### particles_set_trails {#particles_set_trails}

```gdscript
void particles_set_trails(particles: RID, enable: bool, length_sec: float)
```

If enable is true, enables trails for the particles with the specified length_sec in seconds. Equivalent to GPUParticles3D.trail_enabled and GPUParticles3D.trail_lifetime.

### particles_set_transform_align {#particles_set_transform_align}

```gdscript
void particles_set_transform_align(particles: RID, align: RenderingServer.ParticlesTransformAlign)
```

Sets the transform alignment for the particle system. Equivalent to GPUParticles3D.transform_align.

### particles_set_transform_align_axis {#particles_set_transform_align_axis}

```gdscript
void particles_set_transform_align_axis(particles: RID, rotation_axis: RenderingServer.ParticlesTransformAlignAxis)
```

Sets which axis to use for transform alignment.

### particles_set_transform_align_channel_filter {#particles_set_transform_align_channel_filter}

```gdscript
void particles_set_transform_align_channel_filter(particles: RID, channel_filter: RenderingServer.ParticlesTransformAlignCustomSrc)
```

When using Z-Billboarding, which CUSTOM channel to read from.

### particles_set_use_local_coordinates {#particles_set_use_local_coordinates}

```gdscript
void particles_set_use_local_coordinates(particles: RID, enable: bool)
```

If true, particles use local coordinates. If false they use global coordinates. Equivalent to GPUParticles3D.local_coords.

### positional_soft_shadow_filter_set_quality {#positional_soft_shadow_filter_set_quality}

```gdscript
void positional_soft_shadow_filter_set_quality(quality: RenderingServer.ShadowQuality)
```

Sets the filter quality for omni and spot light shadows in 3D. See also ProjectSettings.rendering/lights_and_shadows/positional_shadow/soft_shadow_filter_quality. This parameter is global and cannot be set on a per-viewport basis.

### reflection_probe_create {#reflection_probe_create}

```gdscript
RID reflection_probe_create()
```

Creates a reflection probe and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all reflection_probe_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this reflection probe to an instance using instance_set_base() using the returned RID.

Note: The equivalent node is ReflectionProbe.

### reflection_probe_set_ambient_color {#reflection_probe_set_ambient_color}

```gdscript
void reflection_probe_set_ambient_color(probe: RID, color: Color)
```

Sets the reflection probe's custom ambient light color. Equivalent to ReflectionProbe.ambient_color.

### reflection_probe_set_ambient_energy {#reflection_probe_set_ambient_energy}

```gdscript
void reflection_probe_set_ambient_energy(probe: RID, energy: float)
```

Sets the reflection probe's custom ambient light energy. Equivalent to ReflectionProbe.ambient_color_energy.

### reflection_probe_set_ambient_mode {#reflection_probe_set_ambient_mode}

```gdscript
void reflection_probe_set_ambient_mode(probe: RID, mode: RenderingServer.ReflectionProbeAmbientMode)
```

Sets the reflection probe's ambient light mode. Equivalent to ReflectionProbe.ambient_mode.

### reflection_probe_set_as_interior {#reflection_probe_set_as_interior}

```gdscript
void reflection_probe_set_as_interior(probe: RID, enable: bool)
```

If true, reflections will ignore sky contribution. Equivalent to ReflectionProbe.interior.

### reflection_probe_set_blend_distance {#reflection_probe_set_blend_distance}

```gdscript
void reflection_probe_set_blend_distance(probe: RID, blend_distance: float)
```

Sets the distance in meters over which a probe blends into the scene.

### reflection_probe_set_cull_mask {#reflection_probe_set_cull_mask}

```gdscript
void reflection_probe_set_cull_mask(probe: RID, layers: int)
```

Sets the render cull mask for this reflection probe. Only instances with a matching layer will be reflected by this probe. Equivalent to ReflectionProbe.cull_mask.

### reflection_probe_set_enable_box_projection {#reflection_probe_set_enable_box_projection}

```gdscript
void reflection_probe_set_enable_box_projection(probe: RID, enable: bool)
```

If true, uses box projection. This can make reflections look more correct in certain situations. Equivalent to ReflectionProbe.box_projection.

### reflection_probe_set_enable_shadows {#reflection_probe_set_enable_shadows}

```gdscript
void reflection_probe_set_enable_shadows(probe: RID, enable: bool)
```

If true, computes shadows in the reflection probe. This makes the reflection much slower to compute. Equivalent to ReflectionProbe.enable_shadows.

### reflection_probe_set_intensity {#reflection_probe_set_intensity}

```gdscript
void reflection_probe_set_intensity(probe: RID, intensity: float)
```

Sets the intensity of the reflection probe. Intensity modulates the strength of the reflection. Equivalent to ReflectionProbe.intensity.

### reflection_probe_set_max_distance {#reflection_probe_set_max_distance}

```gdscript
void reflection_probe_set_max_distance(probe: RID, distance: float)
```

Sets the max distance away from the probe an object can be before it is culled. Equivalent to ReflectionProbe.max_distance.

### reflection_probe_set_mesh_lod_threshold {#reflection_probe_set_mesh_lod_threshold}

```gdscript
void reflection_probe_set_mesh_lod_threshold(probe: RID, pixels: float)
```

Sets the mesh level of detail to use in the reflection probe rendering. Higher values will use less detailed versions of meshes that have LOD variations generated, which can improve performance. Equivalent to ReflectionProbe.mesh_lod_threshold.

### reflection_probe_set_origin_offset {#reflection_probe_set_origin_offset}

```gdscript
void reflection_probe_set_origin_offset(probe: RID, offset: Vector3)
```

Sets the origin offset to be used when this reflection probe is in box project mode. Equivalent to ReflectionProbe.origin_offset.

### reflection_probe_set_reflection_mask {#reflection_probe_set_reflection_mask}

```gdscript
void reflection_probe_set_reflection_mask(probe: RID, layers: int)
```

Sets the render reflection mask for this reflection probe. Only instances with a matching layer will have reflections applied from this probe. Equivalent to ReflectionProbe.reflection_mask.

### reflection_probe_set_resolution {#reflection_probe_set_resolution}

```gdscript
void reflection_probe_set_resolution(probe: RID, resolution: int)
```

Deprecated: This method has not done anything since Godot 3.

Deprecated. This method does nothing.

### reflection_probe_set_size {#reflection_probe_set_size}

```gdscript
void reflection_probe_set_size(probe: RID, size: Vector3)
```

Sets the size of the area that the reflection probe will capture. Equivalent to ReflectionProbe.size.

### reflection_probe_set_update_mode {#reflection_probe_set_update_mode}

```gdscript
void reflection_probe_set_update_mode(probe: RID, mode: RenderingServer.ReflectionProbeUpdateMode)
```

Sets how often the reflection probe updates. Can either be once or every frame.

### request_frame_drawn_callback {#request_frame_drawn_callback}

```gdscript
void request_frame_drawn_callback(callable: Callable)
```

Schedules a callback to the given callable after a frame has been drawn.

### scenario_create {#scenario_create}

```gdscript
RID scenario_create()
```

Creates a scenario and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all scenario_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

The scenario is the 3D world that all the visual instances exist in.

### scenario_set_camera_attributes {#scenario_set_camera_attributes}

```gdscript
void scenario_set_camera_attributes(scenario: RID, effects: RID)
```

Sets the camera attributes (effects) that will be used with this scenario. See also CameraAttributes.

### scenario_set_compositor {#scenario_set_compositor}

```gdscript
void scenario_set_compositor(scenario: RID, compositor: RID)
```

Sets the compositor (compositor) that will be used with this scenario. See also Compositor.

### scenario_set_environment {#scenario_set_environment}

```gdscript
void scenario_set_environment(scenario: RID, environment: RID)
```

Sets the environment that will be used with this scenario. See also Environment.

### scenario_set_fallback_environment {#scenario_set_fallback_environment}

```gdscript
void scenario_set_fallback_environment(scenario: RID, environment: RID)
```

Sets the fallback environment to be used by this scenario. The fallback environment is used if no environment is set. Internally, this is used by the editor to provide a default environment.

### screen_space_roughness_limiter_set_active {#screen_space_roughness_limiter_set_active}

```gdscript
void screen_space_roughness_limiter_set_active(enable: bool, amount: float, limit: float)
```

Sets the screen-space roughness limiter parameters, such as whether it should be enabled and its thresholds. Equivalent to ProjectSettings.rendering/anti_aliasing/screen_space_roughness_limiter/enabled, ProjectSettings.rendering/anti_aliasing/screen_space_roughness_limiter/amount and ProjectSettings.rendering/anti_aliasing/screen_space_roughness_limiter/limit.

### set_boot_image {#set_boot_image}

```gdscript
void set_boot_image(image: Image, color: Color, scale: bool, use_filter: bool = true)
```

Deprecated: Use set_boot_image_with_stretch() instead.

Sets a boot image. The color defines the background color. The value of scale indicates if the image will be scaled to fit the screen size. If use_filter is true, the image will be scaled with linear interpolation. If use_filter is false, the image will be scaled with nearest-neighbor interpolation.

### set_boot_image_with_stretch {#set_boot_image_with_stretch}

```gdscript
void set_boot_image_with_stretch(image: Image, color: Color, stretch_mode: RenderingServer.SplashStretchMode, use_filter: bool = true)
```

Sets a boot image. The color defines the background color. The value of stretch_mode indicates how the image will be stretched (see SplashStretchMode for possible values). If use_filter is true, the image will be scaled with linear interpolation. If use_filter is false, the image will be scaled with nearest-neighbor interpolation.

### set_debug_generate_wireframes {#set_debug_generate_wireframes}

```gdscript
void set_debug_generate_wireframes(generate: bool)
```

If generate is true, generates debug wireframes for all meshes that are loaded when using the Compatibility renderer. By default, the engine does not generate debug wireframes at runtime, since they slow down loading of assets and take up VRAM.

Note: You must call this method before loading any meshes when using the Compatibility renderer. Otherwise, wireframes will not be used.

### set_default_clear_color {#set_default_clear_color}

```gdscript
void set_default_clear_color(color: Color)
```

Sets the default clear color which is used when a specific clear color has not been selected. See also get_default_clear_color().

### set_render_loop_enabled {#set_render_loop_enabled}

```gdscript
void set_render_loop_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### shader_create {#shader_create}

```gdscript
RID shader_create()
```

Creates an empty shader and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all shader_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is Shader.

### shader_get_code {#shader_get_code}

```gdscript
String shader_get_code(shader: RID) const
```

Returns a shader's source code as a string.

### shader_get_default_texture_parameter {#shader_get_default_texture_parameter}

```gdscript
RID shader_get_default_texture_parameter(shader: RID, name: StringName, index: int = 0) const
```

Returns a default texture from a shader searched by name.

Note: If the sampler array is used use index to access the specified texture.

### shader_get_parameter_default {#shader_get_parameter_default}

```gdscript
Variant shader_get_parameter_default(shader: RID, name: StringName) const
```

Returns the default value for the specified shader uniform. This is usually the value written in the shader source code.

### shader_set_code {#shader_set_code}

```gdscript
void shader_set_code(shader: RID, code: String)
```

Sets the shader's source code (which triggers recompilation after being changed).

### shader_set_default_texture_parameter {#shader_set_default_texture_parameter}

```gdscript
void shader_set_default_texture_parameter(shader: RID, name: StringName, texture: RID, index: int = 0)
```

Sets a shader's default texture. Overwrites the texture given by name.

Note: If the sampler array is used use index to access the specified texture.

### shader_set_path_hint {#shader_set_path_hint}

```gdscript
void shader_set_path_hint(shader: RID, path: String)
```

Sets the path hint for the specified shader. This should generally match the Shader resource's Resource.resource_path.

### skeleton_allocate_data {#skeleton_allocate_data}

```gdscript
void skeleton_allocate_data(skeleton: RID, bones: int, is_2d_skeleton: bool = false)
```

Allocates data for this skeleton using the number of bones specified in bones. If is_2d_skeleton is true, the skeleton will be treated as a 2D skeleton instead of a 3D skeleton. See also skeleton_get_bone_count().

### skeleton_bone_get_transform {#skeleton_bone_get_transform}

```gdscript
Transform3D skeleton_bone_get_transform(skeleton: RID, bone: int) const
```

Returns the Transform3D set for a specific bone of this skeleton.

### skeleton_bone_get_transform_2d {#skeleton_bone_get_transform_2d}

```gdscript
Transform2D skeleton_bone_get_transform_2d(skeleton: RID, bone: int) const
```

Returns the Transform2D set for a specific bone of this skeleton.

### skeleton_bone_set_transform {#skeleton_bone_set_transform}

```gdscript
void skeleton_bone_set_transform(skeleton: RID, bone: int, transform: Transform3D)
```

Sets the Transform3D for a specific bone of this skeleton.

### skeleton_bone_set_transform_2d {#skeleton_bone_set_transform_2d}

```gdscript
void skeleton_bone_set_transform_2d(skeleton: RID, bone: int, transform: Transform2D)
```

Sets the Transform2D for a specific bone of this skeleton.

### skeleton_create {#skeleton_create}

```gdscript
RID skeleton_create()
```

Creates a skeleton and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all skeleton_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

### skeleton_get_bone_count {#skeleton_get_bone_count}

```gdscript
int skeleton_get_bone_count(skeleton: RID) const
```

Returns the number of bones allocated for this skeleton. See also skeleton_allocate_data().

### skeleton_set_base_transform_2d {#skeleton_set_base_transform_2d}

```gdscript
void skeleton_set_base_transform_2d(skeleton: RID, base_transform: Transform2D)
```

Sets the base Transform2D to use for the specified skeleton.

### sky_bake_panorama {#sky_bake_panorama}

```gdscript
Image sky_bake_panorama(sky: RID, energy: float, bake_irradiance: bool, size: Vector2i)
```

Generates and returns an Image containing the radiance map for the specified sky RID. This supports built-in sky material and custom sky shaders. If bake_irradiance is true, the irradiance map is saved instead of the radiance map. The radiance map is used to render reflected light, while the irradiance map is used to render ambient light. See also environment_bake_panorama().

Note: The image is saved using linear encoding without any tonemapping performed, which means it will look too dark if viewed directly in an image editor. energy values above 1.0 can be used to brighten the resulting image.

Note: size should be a 2:1 aspect ratio for the generated panorama to have square pixels. For radiance maps, there is no point in using a height greater than Sky.radiance_size, as it won't increase detail. Irradiance maps only contain low-frequency data, so there is usually no point in going past a size of 128×64 pixels when saving an irradiance map.

### sky_create {#sky_create}

```gdscript
RID sky_create()
```

Creates an empty sky and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all sky_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

### sky_set_material {#sky_set_material}

```gdscript
void sky_set_material(sky: RID, material: RID)
```

Sets the material that the sky uses to render the background, ambient and reflection maps.

### sky_set_mode {#sky_set_mode}

```gdscript
void sky_set_mode(sky: RID, mode: RenderingServer.SkyMode)
```

Sets the process mode of the sky specified by the sky RID. Equivalent to Sky.process_mode.

### sky_set_radiance_size {#sky_set_radiance_size}

```gdscript
void sky_set_radiance_size(sky: RID, radiance_size: int)
```

Sets the radiance_size of the sky specified by the sky RID (in pixels). Equivalent to Sky.radiance_size.

### spot_light_create {#spot_light_create}

```gdscript
RID spot_light_create()
```

Creates a spot light and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID can be used in most light_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this spot light to an instance using instance_set_base() using the returned RID.

### sub_surface_scattering_set_quality {#sub_surface_scattering_set_quality}

```gdscript
void sub_surface_scattering_set_quality(quality: RenderingServer.SubSurfaceScatteringQuality)
```

Sets ProjectSettings.rendering/environment/subsurface_scattering/subsurface_scattering_quality to use when rendering materials that have subsurface scattering enabled.

### sub_surface_scattering_set_scale {#sub_surface_scattering_set_scale}

```gdscript
void sub_surface_scattering_set_scale(scale: float, depth_scale: float)
```

Sets the ProjectSettings.rendering/environment/subsurface_scattering/subsurface_scattering_scale and ProjectSettings.rendering/environment/subsurface_scattering/subsurface_scattering_depth_scale to use when rendering materials that have subsurface scattering enabled.

### texture_2d_create {#texture_2d_create}

```gdscript
RID texture_2d_create(image: Image)
```

Creates a 2-dimensional texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all texture_2d_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is Texture2D.

Note: Not to be confused with RenderingDevice.texture_create(), which creates the graphics API's own texture type as opposed to the Godot-specific Texture2D resource.

### texture_2d_get {#texture_2d_get}

```gdscript
Image texture_2d_get(texture: RID) const
```

Returns an Image instance from the given texture RID.

Example: Get the test texture from get_test_texture() and apply it to a Sprite2D node:

```gdscript
var texture_rid = RenderingServer.get_test_texture()
var texture = ImageTexture.create_from_image(RenderingServer.texture_2d_get(texture_rid))
$Sprite2D.texture = texture
```

### texture_2d_layer_get {#texture_2d_layer_get}

```gdscript
Image texture_2d_layer_get(texture: RID, layer: int) const
```

Returns an Image instance from the given texture RID and layer.

### texture_2d_layered_create {#texture_2d_layered_create}

```gdscript
RID texture_2d_layered_create(layers: typedarray::Image, layered_type: RenderingServer.TextureLayeredType)
```

Creates a 2-dimensional layered texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all texture_2d_layered_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is TextureLayered.

### texture_2d_layered_placeholder_create {#texture_2d_layered_placeholder_create}

```gdscript
RID texture_2d_layered_placeholder_create(layered_type: RenderingServer.TextureLayeredType)
```

Creates a placeholder for a 2-dimensional layered texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all texture_2d_layered_* RenderingServer functions, although it does nothing when used. See also texture_2d_placeholder_create().

Note: The equivalent resource is PlaceholderTextureLayered.

### texture_2d_placeholder_create {#texture_2d_placeholder_create}

```gdscript
RID texture_2d_placeholder_create()
```

Creates a placeholder for a 2-dimensional layered texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all texture_2d_layered_* RenderingServer functions, although it does nothing when used. See also texture_2d_layered_placeholder_create().

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is PlaceholderTexture2D.

### texture_2d_update {#texture_2d_update}

```gdscript
void texture_2d_update(texture: RID, image: Image, layer: int)
```

Updates the texture specified by the texture RID with the data in image. A layer must also be specified, which should be 0 when updating a single-layer texture (Texture2D).

Note: The image must have the same width, height and format as the current texture data. Otherwise, an error will be printed and the original texture won't be modified. If you need to use different width, height or format, use texture_replace() instead.

### texture_3d_create {#texture_3d_create}

```gdscript
RID texture_3d_create(format: Image.Format, width: int, height: int, depth: int, mipmaps: bool, data: typedarray::Image)
```

Note: The equivalent resource is Texture3D.

### texture_3d_get {#texture_3d_get}

```gdscript
typedarray::Image texture_3d_get(texture: RID) const
```

Returns 3D texture data as an array of Images for the specified texture RID.

### texture_3d_placeholder_create {#texture_3d_placeholder_create}

```gdscript
RID texture_3d_placeholder_create()
```

Creates a placeholder for a 3-dimensional texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all texture_3d_* RenderingServer functions, although it does nothing when used.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is PlaceholderTexture3D.

### texture_3d_update {#texture_3d_update}

```gdscript
void texture_3d_update(texture: RID, data: typedarray::Image)
```

Updates the texture specified by the texture RID's data with the data in data. All the texture's layers must be replaced at once.

Note: The texture must have the same width, height, depth and format as the current texture data. Otherwise, an error will be printed and the original texture won't be modified. If you need to use different width, height, depth or format, use texture_replace() instead.

### texture_create_from_native_handle {#texture_create_from_native_handle}

```gdscript
RID texture_create_from_native_handle(type: RenderingServer.TextureType, format: Image.Format, native_handle: int, width: int, height: int, depth: int, layers: int = 1, layered_type: RenderingServer.TextureLayeredType = 0)
```

Creates a texture based on a native handle that was created outside of Godot's renderer.

Note: If using only the rendering device renderer, it's recommend to use RenderingDevice.texture_create_from_extension() together with texture_rd_create(), rather than this method. This way, the texture's format and usage can be controlled more effectively.

### texture_drawable_blit_rect {#texture_drawable_blit_rect}

```gdscript
void texture_drawable_blit_rect(textures: typedarray::RID, rect: Rect2i, material: RID, modulate: Color, source_textures: typedarray::RID, to_mipmap: int = 0)
```

Draws to rect on up to 4 given Drawable textures, using a TextureBlit Shader from material. modulate and up to 4 source_textures are uniforms for the Shader to process with. to_mipmap can specify to perform this draw to a lower mipmap level.

Note: All textures must be the same size and format.

### texture_drawable_create {#texture_drawable_create}

```gdscript
RID texture_drawable_create(width: int, height: int, format: RenderingServer.TextureDrawableFormat, color: Color = Color(1, 1, 1, 1), with_mipmaps: bool = false)
```

Creates a 2-dimensional texture and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all texture_drawable* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent resource is DrawableTexture2D.

### texture_drawable_generate_mipmaps {#texture_drawable_generate_mipmaps}

```gdscript
void texture_drawable_generate_mipmaps(texture: RID)
```

Calculates new MipMaps for the given Drawable texture.

### texture_drawable_get_default_material {#texture_drawable_get_default_material}

```gdscript
RID texture_drawable_get_default_material() const
```

Returns a ShaderMaterial with the default texture_blit Shader.

### texture_get_format {#texture_get_format}

```gdscript
Image.Format texture_get_format(texture: RID) const
```

Returns the format for the texture.

### texture_get_native_handle {#texture_get_native_handle}

```gdscript
int texture_get_native_handle(texture: RID, srgb: bool = false) const
```

Returns the internal graphics handle for this texture object. For use when communicating with third-party APIs mostly with GDExtension.

srgb should be true when the texture uses nonlinear sRGB encoding and false when the texture uses linear encoding.

Note: This function returns a uint64_t which internally maps to a GLuint (OpenGL) or VkImage (Vulkan).

### texture_get_path {#texture_get_path}

```gdscript
String texture_get_path(texture: RID) const
```

Returns the resource path (starting with res:// or uid://) for the specified texture RID. Returns an empty String if the resource is built-in. See also texture_set_path().

### texture_get_rd_texture {#texture_get_rd_texture}

```gdscript
RID texture_get_rd_texture(texture: RID, srgb: bool = false) const
```

Returns a texture RID that can be used with RenderingDevice.

srgb should be true when the texture uses nonlinear sRGB encoding and false when the texture uses linear encoding.

### texture_proxy_create {#texture_proxy_create}

```gdscript
RID texture_proxy_create(base: RID)
```

Deprecated: ProxyTexture was removed in Godot 4.

This method does nothing and always returns an invalid RID.

### texture_proxy_update {#texture_proxy_update}

```gdscript
void texture_proxy_update(texture: RID, proxy_to: RID)
```

Deprecated: ProxyTexture was removed in Godot 4.

This method does nothing.

### texture_rd_create {#texture_rd_create}

```gdscript
RID texture_rd_create(rd_texture: RID, layer_type: RenderingServer.TextureLayeredType = 0)
```

Creates a new texture object based on a texture created directly on the RenderingDevice. If the texture contains layers, layer_type is used to define the layer type.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The RenderingServer's free_rid() won't free the underlying rd_texture, you will want to free the rd_texture using RenderingDevice.free_rid().

### texture_replace {#texture_replace}

```gdscript
void texture_replace(texture: RID, by_texture: RID)
```

Replaces texture's texture data by the texture specified by the by_texture RID, without changing texture's RID.

### texture_set_force_redraw_if_visible {#texture_set_force_redraw_if_visible}

```gdscript
void texture_set_force_redraw_if_visible(texture: RID, enable: bool)
```

Sets whether the texture RID should force redrawing when it's visible on screen when OS.low_processor_usage_mode is true. This is used by AnimatedTexture to force redrawing.

### texture_set_path {#texture_set_path}

```gdscript
void texture_set_path(texture: RID, path: String)
```

Sets the resource path for this texture RID. See also texture_get_path().

Note: This is purely a hint and does not cause the texture to be automatically saved when set to a res:// path.

### texture_set_size_override {#texture_set_size_override}

```gdscript
void texture_set_size_override(texture: RID, width: int, height: int)
```

Sets the size at which the texture should be displayed in 2D, ignoring its original size. This does not rescale the texture data itself, only how it is drawn in 2D. Set width and height to 0 to disable the size override.

### viewport_attach_camera {#viewport_attach_camera}

```gdscript
void viewport_attach_camera(viewport: RID, camera: RID)
```

Sets a viewport's camera.

### viewport_attach_canvas {#viewport_attach_canvas}

```gdscript
void viewport_attach_canvas(viewport: RID, canvas: RID)
```

Sets a viewport's canvas.

### viewport_attach_to_screen {#viewport_attach_to_screen}

```gdscript
void viewport_attach_to_screen(viewport: RID, rect: Rect2 = Rect2(0, 0, 0, 0), screen: int = 0)
```

Copies the viewport to a region of the screen specified by rect. If viewport_set_render_direct_to_screen() is true, then the viewport does not use a framebuffer and the contents of the viewport are rendered directly to screen. However, note that the root viewport is drawn last, therefore it will draw over the screen. Accordingly, you must set the root viewport to an area that does not cover the area that you have attached this viewport to.

For example, you can set the root viewport to not render at all with the following code:

```
func _ready():
    RenderingServer.viewport_attach_to_screen(get_viewport().get_viewport_rid(), Rect2())
    RenderingServer.viewport_attach_to_screen($Viewport.get_viewport_rid(), Rect2(0, 0, 600, 600))
```

Using this can result in significant optimization, especially on lower-end devices. However, it comes at the cost of having to manage your viewports manually. For further optimization, see viewport_set_render_direct_to_screen().

### viewport_create {#viewport_create}

```gdscript
RID viewport_create()
```

Creates an empty viewport and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all viewport_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is Viewport.

### viewport_get_measured_render_time_cpu {#viewport_get_measured_render_time_cpu}

```gdscript
float viewport_get_measured_render_time_cpu(viewport: RID) const
```

Returns the CPU time taken to render the last frame in milliseconds. This only includes time spent in rendering-related operations; scripts' _process functions and other engine subsystems are not included in this readout. To get a complete readout of CPU time spent to render the scene, sum the render times of all viewports that are drawn every frame plus get_frame_setup_time_cpu(). Unlike Engine.get_frames_per_second(), this method will accurately reflect CPU utilization even if framerate is capped via V-Sync or Engine.max_fps. See also viewport_get_measured_render_time_gpu().

Note: Requires measurements to be enabled on the specified viewport using viewport_set_measure_render_time(). Otherwise, this method returns 0.0.

### viewport_get_measured_render_time_gpu {#viewport_get_measured_render_time_gpu}

```gdscript
float viewport_get_measured_render_time_gpu(viewport: RID) const
```

Returns the GPU time taken to render the last frame in milliseconds. To get a complete readout of GPU time spent to render the scene, sum the render times of all viewports that are drawn every frame. Unlike Engine.get_frames_per_second(), this method accurately reflects GPU utilization even if framerate is capped via V-Sync or Engine.max_fps. See also viewport_get_measured_render_time_cpu().

Note: Requires measurements to be enabled on the specified viewport using viewport_set_measure_render_time(). Otherwise, this method returns 0.0.

Note: When GPU utilization is low enough during a certain period of time, GPUs will decrease their power state (which in turn decreases core and memory clock speeds). This can cause the reported GPU time to increase if GPU utilization is kept low enough by a framerate cap (compared to what it would be at the GPU's highest power state). Keep this in mind when benchmarking using viewport_get_measured_render_time_gpu(). This behavior can be overridden in the graphics driver settings at the cost of higher power usage.

### viewport_get_render_info {#viewport_get_render_info}

```gdscript
int viewport_get_render_info(viewport: RID, type: RenderingServer.ViewportRenderInfoType, info: RenderingServer.ViewportRenderInfo)
```

Returns a statistic about the rendering engine which can be used for performance profiling. This is separated into render pass types, each of them having the same infos you can query (different passes will return different values).

See also get_rendering_info(), which returns global information across all viewports.

Note: Viewport rendering information is not available until at least 2 frames have been rendered by the engine. If rendering information is not available, viewport_get_render_info() returns 0. To print rendering information in _ready() successfully, use the following:

```gdscript
func _ready():
    for _i in 2:
        await get_tree().process_frame
    print(
            RenderingServer.viewport_get_render_info(get_viewport().get_viewport_rid(),
            RenderingServer.VIEWPORT_RENDER_INFO_TYPE_VISIBLE,
            RenderingServer.VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME)
    )
```

### viewport_get_render_target {#viewport_get_render_target}

```gdscript
RID viewport_get_render_target(viewport: RID) const
```

Returns the render target for the viewport.

### viewport_get_texture {#viewport_get_texture}

```gdscript
RID viewport_get_texture(viewport: RID) const
```

Returns the viewport's last rendered frame.

### viewport_get_update_mode {#viewport_get_update_mode}

```gdscript
RenderingServer.ViewportUpdateMode viewport_get_update_mode(viewport: RID) const
```

Returns the viewport's update mode.

Warning: Calling this from any thread other than the rendering thread will be detrimental to performance.

### viewport_remove_canvas {#viewport_remove_canvas}

```gdscript
void viewport_remove_canvas(viewport: RID, canvas: RID)
```

Detaches a viewport from a canvas.

### viewport_set_active {#viewport_set_active}

```gdscript
void viewport_set_active(viewport: RID, active: bool)
```

If true, sets the viewport active, else sets it inactive.

### viewport_set_anisotropic_filtering_level {#viewport_set_anisotropic_filtering_level}

```gdscript
void viewport_set_anisotropic_filtering_level(viewport: RID, anisotropic_filtering_level: RenderingServer.ViewportAnisotropicFiltering)
```

Sets the maximum number of samples to take when using anisotropic filtering on textures (as a power of two). A higher sample count will result in sharper textures at oblique angles, but is more expensive to compute. A value of 0 forcibly disables anisotropic filtering, even on materials where it is enabled.

The anisotropic filtering level also affects decals and light projectors if they are configured to use anisotropic filtering. See ProjectSettings.rendering/textures/decals/filter and ProjectSettings.rendering/textures/light_projectors/filter.

Note: In 3D, for this setting to have an effect, set BaseMaterial3D.texture_filter to BaseMaterial3D.TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC or BaseMaterial3D.TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC on materials.

Note: In 2D, for this setting to have an effect, set CanvasItem.texture_filter to CanvasItem.TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC or CanvasItem.TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC on the CanvasItem node displaying the texture (or in CanvasTexture). However, anisotropic filtering is rarely useful in 2D, so only enable it for textures in 2D if it makes a meaningful visual difference.

### viewport_set_canvas_cull_mask {#viewport_set_canvas_cull_mask}

```gdscript
void viewport_set_canvas_cull_mask(viewport: RID, canvas_cull_mask: int)
```

Sets the rendering mask associated with this Viewport. Only CanvasItem nodes with a matching rendering visibility layer will be rendered by this Viewport.

### viewport_set_canvas_stacking {#viewport_set_canvas_stacking}

```gdscript
void viewport_set_canvas_stacking(viewport: RID, canvas: RID, layer: int, sublayer: int)
```

Sets the stacking order for a viewport's canvas.

layer is the actual canvas layer, while sublayer specifies the stacking order of the canvas among those in the same layer.

Note: layer should be between CANVAS_LAYER_MIN and CANVAS_LAYER_MAX (inclusive). Any other value will wrap around.

### viewport_set_canvas_transform {#viewport_set_canvas_transform}

```gdscript
void viewport_set_canvas_transform(viewport: RID, canvas: RID, offset: Transform2D)
```

Sets the transformation of a viewport's canvas.

### viewport_set_clear_mode {#viewport_set_clear_mode}

```gdscript
void viewport_set_clear_mode(viewport: RID, clear_mode: RenderingServer.ViewportClearMode)
```

Sets the clear mode of a viewport.

### viewport_set_debug_draw {#viewport_set_debug_draw}

```gdscript
void viewport_set_debug_draw(viewport: RID, draw: RenderingServer.ViewportDebugDraw)
```

Sets the debug draw mode of a viewport.

### viewport_set_default_canvas_item_texture_filter {#viewport_set_default_canvas_item_texture_filter}

```gdscript
void viewport_set_default_canvas_item_texture_filter(viewport: RID, filter: RenderingServer.CanvasItemTextureFilter)
```

Sets the default texture filtering mode for the specified viewport RID.

### viewport_set_default_canvas_item_texture_repeat {#viewport_set_default_canvas_item_texture_repeat}

```gdscript
void viewport_set_default_canvas_item_texture_repeat(viewport: RID, repeat: RenderingServer.CanvasItemTextureRepeat)
```

Sets the default texture repeat mode for the specified viewport RID.

### viewport_set_disable_2d {#viewport_set_disable_2d}

```gdscript
void viewport_set_disable_2d(viewport: RID, disable: bool)
```

If true, the viewport's canvas (i.e. 2D and GUI elements) is not rendered.

### viewport_set_disable_3d {#viewport_set_disable_3d}

```gdscript
void viewport_set_disable_3d(viewport: RID, disable: bool)
```

If true, the viewport's 3D elements are not rendered.

### viewport_set_environment_mode {#viewport_set_environment_mode}

```gdscript
void viewport_set_environment_mode(viewport: RID, mode: RenderingServer.ViewportEnvironmentMode)
```

Sets the viewport's environment mode which allows enabling or disabling rendering of 3D environment over 2D canvas. When disabled, 2D will not be affected by the environment. When enabled, 2D will be affected by the environment if the environment background mode is ENV_BG_CANVAS. The default behavior is to inherit the setting from the viewport's parent. If the topmost parent is also set to VIEWPORT_ENVIRONMENT_INHERIT, then the behavior will be the same as if it was set to VIEWPORT_ENVIRONMENT_ENABLED.

### viewport_set_fsr_sharpness {#viewport_set_fsr_sharpness}

```gdscript
void viewport_set_fsr_sharpness(viewport: RID, sharpness: float)
```

Determines how sharp the upscaled image will be when using the FSR upscaling mode. Sharpness halves with every whole number. Values go from 0.0 (sharpest) to 2.0. Values above 2.0 won't make a visible difference.

### viewport_set_global_canvas_transform {#viewport_set_global_canvas_transform}

```gdscript
void viewport_set_global_canvas_transform(viewport: RID, transform: Transform2D)
```

Sets the viewport's global transformation matrix.

### viewport_set_measure_render_time {#viewport_set_measure_render_time}

```gdscript
void viewport_set_measure_render_time(viewport: RID, enable: bool)
```

Sets the measurement for the given viewport RID (obtained using Viewport.get_viewport_rid()). Once enabled, viewport_get_measured_render_time_cpu() and viewport_get_measured_render_time_gpu() will return values greater than 0.0 when queried with the given viewport.

### viewport_set_msaa_2d {#viewport_set_msaa_2d}

```gdscript
void viewport_set_msaa_2d(viewport: RID, msaa: RenderingServer.ViewportMSAA)
```

Sets the multisample antialiasing mode for 2D/Canvas on the specified viewport RID. Equivalent to ProjectSettings.rendering/anti_aliasing/quality/msaa_2d or Viewport.msaa_2d.

### viewport_set_msaa_3d {#viewport_set_msaa_3d}

```gdscript
void viewport_set_msaa_3d(viewport: RID, msaa: RenderingServer.ViewportMSAA)
```

Sets the multisample antialiasing mode for 3D on the specified viewport RID. Equivalent to ProjectSettings.rendering/anti_aliasing/quality/msaa_3d or Viewport.msaa_3d.

### viewport_set_occlusion_culling_build_quality {#viewport_set_occlusion_culling_build_quality}

```gdscript
void viewport_set_occlusion_culling_build_quality(quality: RenderingServer.ViewportOcclusionCullingBuildQuality)
```

Sets the ProjectSettings.rendering/occlusion_culling/bvh_build_quality to use for occlusion culling. This parameter is global and cannot be set on a per-viewport basis.

### viewport_set_occlusion_rays_per_thread {#viewport_set_occlusion_rays_per_thread}

```gdscript
void viewport_set_occlusion_rays_per_thread(rays_per_thread: int)
```

Sets the ProjectSettings.rendering/occlusion_culling/occlusion_rays_per_thread to use for occlusion culling. This parameter is global and cannot be set on a per-viewport basis.

### viewport_set_parent_viewport {#viewport_set_parent_viewport}

```gdscript
void viewport_set_parent_viewport(viewport: RID, parent_viewport: RID)
```

Sets the viewport's parent to the viewport specified by the parent_viewport RID.

### viewport_set_positional_shadow_atlas_quadrant_subdivision {#viewport_set_positional_shadow_atlas_quadrant_subdivision}

```gdscript
void viewport_set_positional_shadow_atlas_quadrant_subdivision(viewport: RID, quadrant: int, subdivision: int)
```

Sets the number of subdivisions to use in the specified shadow atlas quadrant for omni and spot shadows. See also Viewport.set_positional_shadow_atlas_quadrant_subdiv().

### viewport_set_positional_shadow_atlas_size {#viewport_set_positional_shadow_atlas_size}

```gdscript
void viewport_set_positional_shadow_atlas_size(viewport: RID, size: int, use_16_bits: bool = false)
```

Sets the size of the shadow atlas's images (used for omni and spot lights) on the viewport specified by the viewport RID. The value is rounded up to the nearest power of 2. If use_16_bits is true, use 16 bits for the omni/spot shadow depth map. Enabling this results in shadows having less precision and may result in shadow acne, but can lead to performance improvements on some devices.

Note: If this is set to 0, no positional shadows will be visible at all. This can improve performance significantly on low-end systems by reducing both the CPU and GPU load (as fewer draw calls are needed to draw the scene without shadows).

### viewport_set_render_direct_to_screen {#viewport_set_render_direct_to_screen}

```gdscript
void viewport_set_render_direct_to_screen(viewport: RID, enabled: bool)
```

If true, render the contents of the viewport directly to screen. This allows a low-level optimization where you can skip drawing a viewport to the root viewport. While this optimization can result in a significant increase in speed (especially on older devices), it comes at a cost of usability. When this is enabled, you cannot read from the viewport or from the screen_texture. You also lose the benefit of certain window settings, such as the various stretch modes. Another consequence to be aware of is that in 2D the rendering happens in window coordinates, so if you have a viewport that is double the size of the window, and you set this, then only the portion that fits within the window will be drawn, no automatic scaling is possible, even if your game scene is significantly larger than the window size.

### viewport_set_scaling_3d_mode {#viewport_set_scaling_3d_mode}

```gdscript
void viewport_set_scaling_3d_mode(viewport: RID, scaling_3d_mode: RenderingServer.ViewportScaling3DMode)
```

Sets the 3D resolution scaling mode. Bilinear scaling renders at different resolution to either undersample or supersample the viewport. FidelityFX Super Resolution 1.0, abbreviated to FSR, is an upscaling technology that produces high quality images at fast framerates by using a spatially aware upscaling algorithm. FSR is slightly more expensive than bilinear, but it produces significantly higher image quality. FSR should be used where possible.

### viewport_set_scaling_3d_scale {#viewport_set_scaling_3d_scale}

```gdscript
void viewport_set_scaling_3d_scale(viewport: RID, scale: float)
```

Scales the 3D render buffer based on the viewport size uses an image filter specified in ViewportScaling3DMode to scale the output image to the full viewport size. Values lower than 1.0 can be used to speed up 3D rendering at the cost of quality (undersampling). Values greater than 1.0 are only valid for bilinear mode and can be used to improve 3D rendering quality at a high performance cost (supersampling). See also ViewportMSAA for multi-sample antialiasing, which is significantly cheaper but only smoothens the edges of polygons.

When using FSR upscaling, AMD recommends exposing the following values as preset options to users "Ultra Quality: 0.77", "Quality: 0.67", "Balanced: 0.59", "Performance: 0.5" instead of exposing the entire scale.

### viewport_set_scenario {#viewport_set_scenario}

```gdscript
void viewport_set_scenario(viewport: RID, scenario: RID)
```

Sets a viewport's scenario. The scenario contains information about environment information, reflection atlas, etc.

### viewport_set_screen_space_aa {#viewport_set_screen_space_aa}

```gdscript
void viewport_set_screen_space_aa(viewport: RID, mode: RenderingServer.ViewportScreenSpaceAA)
```

Sets the viewport's screen-space antialiasing mode. Equivalent to ProjectSettings.rendering/anti_aliasing/quality/screen_space_aa or Viewport.screen_space_aa.

### viewport_set_sdf_oversize_and_scale {#viewport_set_sdf_oversize_and_scale}

```gdscript
void viewport_set_sdf_oversize_and_scale(viewport: RID, oversize: RenderingServer.ViewportSDFOversize, scale: RenderingServer.ViewportSDFScale)
```

Sets the viewport's 2D signed distance field ProjectSettings.rendering/2d/sdf/oversize and ProjectSettings.rendering/2d/sdf/scale. This is used when sampling the signed distance field in CanvasItem shaders as well as GPUParticles2D collision. This is not used by SDFGI in 3D rendering.

### viewport_set_size {#viewport_set_size}

```gdscript
void viewport_set_size(viewport: RID, width: int, height: int, view_count: int = 1)
```

Sets the viewport's width and height in pixels. Optionally the view_count can be set to increase the number of view layers for stereo rendering.

### viewport_set_snap_2d_transforms_to_pixel {#viewport_set_snap_2d_transforms_to_pixel}

```gdscript
void viewport_set_snap_2d_transforms_to_pixel(viewport: RID, enabled: bool)
```

If true, canvas item transforms (i.e. origin position) are snapped to the nearest pixel when rendering. This can lead to a crisper appearance at the cost of less smooth movement, especially when Camera2D smoothing is enabled. Equivalent to ProjectSettings.rendering/2d/snap/snap_2d_transforms_to_pixel.

### viewport_set_snap_2d_vertices_to_pixel {#viewport_set_snap_2d_vertices_to_pixel}

```gdscript
void viewport_set_snap_2d_vertices_to_pixel(viewport: RID, enabled: bool)
```

If true, canvas item vertices (i.e. polygon points) are snapped to the nearest pixel when rendering. This can lead to a crisper appearance at the cost of less smooth movement, especially when Camera2D smoothing is enabled. Equivalent to ProjectSettings.rendering/2d/snap/snap_2d_vertices_to_pixel.

### viewport_set_texture_mipmap_bias {#viewport_set_texture_mipmap_bias}

```gdscript
void viewport_set_texture_mipmap_bias(viewport: RID, mipmap_bias: float)
```

Affects the final texture sharpness by reading from a lower or higher mipmap (also called "texture LOD bias"). Negative values make mipmapped textures sharper but grainier when viewed at a distance, while positive values make mipmapped textures blurrier (even when up close). To get sharper textures at a distance without introducing too much graininess, set this between -0.75 and 0.0. Enabling temporal antialiasing (ProjectSettings.rendering/anti_aliasing/quality/use_taa) can help reduce the graininess visible when using negative mipmap bias.

Note: When the 3D scaling mode is set to FSR 1.0, this value is used to adjust the automatic mipmap bias which is calculated internally based on the scale factor. The formula for this is -log2(1.0 / scale) + mipmap_bias.

Note: This method is only supported in the Forward+ and Mobile renderers, not Compatibility. In Compatibility, this method is always treated as if mipmap_bias was set to 0.0.

### viewport_set_transparent_background {#viewport_set_transparent_background}

```gdscript
void viewport_set_transparent_background(viewport: RID, enabled: bool)
```

If true, the viewport renders its background as transparent.

### viewport_set_update_mode {#viewport_set_update_mode}

```gdscript
void viewport_set_update_mode(viewport: RID, update_mode: RenderingServer.ViewportUpdateMode)
```

Sets when the viewport should be updated.

### viewport_set_use_debanding {#viewport_set_use_debanding}

```gdscript
void viewport_set_use_debanding(viewport: RID, enable: bool)
```

Equivalent to Viewport.use_debanding. See also ProjectSettings.rendering/anti_aliasing/quality/use_debanding.

### viewport_set_use_hdr_2d {#viewport_set_use_hdr_2d}

```gdscript
void viewport_set_use_hdr_2d(viewport: RID, enabled: bool)
```

If true, 2D rendering will use a high dynamic range (HDR) RGBA16 format framebuffer. Additionally, 2D rendering will be performed on linear values and will be converted using the appropriate transfer function immediately before blitting to the screen (if the Viewport is attached to the screen).

Practically speaking, this means that the end result of the Viewport will not be clamped to the 0-1 range and can be used in 3D rendering without color encoding adjustments. This allows 2D rendering to take advantage of effects requiring high dynamic range (e.g. 2D glow) as well as substantially improves the appearance of effects requiring highly detailed gradients. This setting has the same effect as Viewport.use_hdr_2d.

### viewport_set_use_occlusion_culling {#viewport_set_use_occlusion_culling}

```gdscript
void viewport_set_use_occlusion_culling(viewport: RID, enable: bool)
```

If true, enables occlusion culling on the specified viewport. Equivalent to ProjectSettings.rendering/occlusion_culling/use_occlusion_culling.

### viewport_set_use_taa {#viewport_set_use_taa}

```gdscript
void viewport_set_use_taa(viewport: RID, enable: bool)
```

If true, use temporal antialiasing. Equivalent to ProjectSettings.rendering/anti_aliasing/quality/use_taa or Viewport.use_taa.

### viewport_set_use_xr {#viewport_set_use_xr}

```gdscript
void viewport_set_use_xr(viewport: RID, use_xr: bool)
```

If true, the viewport uses augmented or virtual reality technologies. See XRInterface.

### viewport_set_vrs_mode {#viewport_set_vrs_mode}

```gdscript
void viewport_set_vrs_mode(viewport: RID, mode: RenderingServer.ViewportVRSMode)
```

Sets the Variable Rate Shading (VRS) mode for the viewport. If the GPU does not support VRS, this property is ignored. Equivalent to ProjectSettings.rendering/vrs/mode.

### viewport_set_vrs_texture {#viewport_set_vrs_texture}

```gdscript
void viewport_set_vrs_texture(viewport: RID, texture: RID)
```

The texture to use when the VRS mode is set to VIEWPORT_VRS_TEXTURE. Equivalent to ProjectSettings.rendering/vrs/texture.

### viewport_set_vrs_update_mode {#viewport_set_vrs_update_mode}

```gdscript
void viewport_set_vrs_update_mode(viewport: RID, mode: RenderingServer.ViewportVRSUpdateMode)
```

Sets the update mode for Variable Rate Shading (VRS) for the viewport. VRS requires the input texture to be converted to the format usable by the VRS method supported by the hardware. The update mode defines how often this happens. If the GPU does not support VRS, or VRS is not enabled, this property is ignored.

If set to VIEWPORT_VRS_UPDATE_ONCE, the input texture is copied once and the mode is changed to VIEWPORT_VRS_UPDATE_DISABLED.

### visibility_notifier_create {#visibility_notifier_create}

```gdscript
RID visibility_notifier_create()
```

Creates a new 3D visibility notifier object and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all visibility_notifier_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

To place in a scene, attach this notifier to an instance using instance_set_base() using the returned RID.

Note: The equivalent node is VisibleOnScreenNotifier3D.

### visibility_notifier_set_aabb {#visibility_notifier_set_aabb}

```gdscript
void visibility_notifier_set_aabb(notifier: RID, aabb: AABB)
```

Sets the AABB of the specified visibility notifier.

### visibility_notifier_set_callbacks {#visibility_notifier_set_callbacks}

```gdscript
void visibility_notifier_set_callbacks(notifier: RID, enter_callable: Callable, exit_callable: Callable)
```

Sets the methods to be called when the notifier enters or exits the view.

### voxel_gi_allocate_data {#voxel_gi_allocate_data}

```gdscript
void voxel_gi_allocate_data(voxel_gi: RID, to_cell_xform: Transform3D, aabb: AABB, octree_size: Vector3i, octree_cells: PackedByteArray, data_cells: PackedByteArray, distance_field: PackedByteArray, level_counts: PackedInt32Array)
```

Allocates and initializes the voxel GI data for the specified voxel_gi RID. octree_cells must be a multiple of 32. octree_cells must be double the size of data_cells. The allocated data can be retrieved later using the various voxel_gi_get_* methods.

### voxel_gi_create {#voxel_gi_create}

```gdscript
RID voxel_gi_create()
```

Creates a new voxel-based global illumination object and adds it to the RenderingServer. It can be accessed with the RID that is returned. This RID will be used in all voxel_gi_* RenderingServer functions.

Once finished with your RID, you will want to free the RID using the RenderingServer's free_rid() method.

Note: The equivalent node is VoxelGI.

### voxel_gi_get_data_cells {#voxel_gi_get_data_cells}

```gdscript
PackedByteArray voxel_gi_get_data_cells(voxel_gi: RID) const
```

Returns the data cells for the specified voxel GI data instance. See also voxel_gi_allocate_data().

### voxel_gi_get_distance_field {#voxel_gi_get_distance_field}

```gdscript
PackedByteArray voxel_gi_get_distance_field(voxel_gi: RID) const
```

Returns the distance field data for the specified voxel GI data instance. See also voxel_gi_allocate_data().

### voxel_gi_get_level_counts {#voxel_gi_get_level_counts}

```gdscript
PackedInt32Array voxel_gi_get_level_counts(voxel_gi: RID) const
```

Returns the level counts for the specified voxel GI data instance. See also voxel_gi_allocate_data().

### voxel_gi_get_octree_cells {#voxel_gi_get_octree_cells}

```gdscript
PackedByteArray voxel_gi_get_octree_cells(voxel_gi: RID) const
```

Returns the octree cell data for the specified voxel GI data instance. See also voxel_gi_allocate_data().

### voxel_gi_get_octree_size {#voxel_gi_get_octree_size}

```gdscript
Vector3i voxel_gi_get_octree_size(voxel_gi: RID) const
```

Returns the octree size for the specified voxel GI data instance, which corresponds to the number of subdivisions per axis. This can be viewed in the editor by hovering the Bake VoxelGI button at the top of the 3D editor viewport when a VoxelGI node is selected and looking at the Subdivisions field in the tooltip.

### voxel_gi_get_to_cell_xform {#voxel_gi_get_to_cell_xform}

```gdscript
Transform3D voxel_gi_get_to_cell_xform(voxel_gi: RID) const
```

Returns the transform to cell space for the specified voxel GI data instance. See also voxel_gi_allocate_data().

### voxel_gi_set_baked_exposure_normalization {#voxel_gi_set_baked_exposure_normalization}

```gdscript
void voxel_gi_set_baked_exposure_normalization(voxel_gi: RID, baked_exposure: float)
```

Used to inform the renderer what exposure normalization value was used while baking the voxel gi. This value will be used and modulated at run time to ensure that the voxel gi maintains a consistent level of exposure even if the scene-wide exposure normalization is changed at run time. For more information see camera_attributes_set_exposure().

### voxel_gi_set_bias {#voxel_gi_set_bias}

```gdscript
void voxel_gi_set_bias(voxel_gi: RID, bias: float)
```

Sets the VoxelGIData.bias value to use on the specified voxel_gi's RID.

### voxel_gi_set_dynamic_range {#voxel_gi_set_dynamic_range}

```gdscript
void voxel_gi_set_dynamic_range(voxel_gi: RID, range: float)
```

Sets the VoxelGIData.dynamic_range value to use on the specified voxel_gi's RID.

### voxel_gi_set_energy {#voxel_gi_set_energy}

```gdscript
void voxel_gi_set_energy(voxel_gi: RID, energy: float)
```

Sets the VoxelGIData.energy value to use on the specified voxel_gi's RID.

### voxel_gi_set_interior {#voxel_gi_set_interior}

```gdscript
void voxel_gi_set_interior(voxel_gi: RID, enable: bool)
```

Sets the VoxelGIData.interior value to use on the specified voxel_gi's RID.

### voxel_gi_set_normal_bias {#voxel_gi_set_normal_bias}

```gdscript
void voxel_gi_set_normal_bias(voxel_gi: RID, bias: float)
```

Sets the VoxelGIData.normal_bias value to use on the specified voxel_gi's RID.

### voxel_gi_set_propagation {#voxel_gi_set_propagation}

```gdscript
void voxel_gi_set_propagation(voxel_gi: RID, amount: float)
```

Sets the VoxelGIData.propagation value to use on the specified voxel_gi's RID.

### voxel_gi_set_quality {#voxel_gi_set_quality}

```gdscript
void voxel_gi_set_quality(quality: RenderingServer.VoxelGIQuality)
```

Sets the ProjectSettings.rendering/global_illumination/voxel_gi/quality value to use when rendering. This parameter is global and cannot be set on a per-VoxelGI basis.

### voxel_gi_set_use_two_bounces {#voxel_gi_set_use_two_bounces}

```gdscript
void voxel_gi_set_use_two_bounces(voxel_gi: RID, enable: bool)
```

Sets the VoxelGIData.use_two_bounces value to use on the specified voxel_gi's RID.

## 属性

### render_loop_enabled {#render_loop_enabled}

```gdscript
render_loop_enabled: bool
```

- void set_render_loop_enabled(value: bool)
- bool is_render_loop_enabled()

If false, disables rendering completely, but the engine logic is still being processed. You can call force_draw() to draw a frame even with rendering disabled.

## 信号

### frame_pre_draw {#frame_pre_draw}

```gdscript
signal frame_pre_draw()
```

Emitted at the beginning of the frame, before the RenderingServer updates all the Viewports.

### frame_post_draw {#frame_post_draw}

```gdscript
signal frame_post_draw()
```

Emitted at the end of the frame, after the RenderingServer has finished updating all the Viewports.

## 枚举

### TextureType {#enum-TextureType}

```gdscript
enum TextureType { TEXTURE_TYPE_2D = 0, TEXTURE_TYPE_LAYERED = 1, TEXTURE_TYPE_3D = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TEXTURE_TYPE_2D {#TEXTURE_TYPE_2D}

```gdscript
TEXTURE_TYPE_2D = 0
```

2D texture.

#### TEXTURE_TYPE_LAYERED {#TEXTURE_TYPE_LAYERED}

```gdscript
TEXTURE_TYPE_LAYERED = 1
```

Layered texture.

#### TEXTURE_TYPE_3D {#TEXTURE_TYPE_3D}

```gdscript
TEXTURE_TYPE_3D = 2
```

3D texture.

---

### TextureLayeredType {#enum-TextureLayeredType}

```gdscript
enum TextureLayeredType { TEXTURE_LAYERED_2D_ARRAY = 0, TEXTURE_LAYERED_CUBEMAP = 1, TEXTURE_LAYERED_CUBEMAP_ARRAY = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TEXTURE_LAYERED_2D_ARRAY {#TEXTURE_LAYERED_2D_ARRAY}

```gdscript
TEXTURE_LAYERED_2D_ARRAY = 0
```

Array of 2-dimensional textures (see Texture2DArray).

#### TEXTURE_LAYERED_CUBEMAP {#TEXTURE_LAYERED_CUBEMAP}

```gdscript
TEXTURE_LAYERED_CUBEMAP = 1
```

Cubemap texture (see Cubemap).

#### TEXTURE_LAYERED_CUBEMAP_ARRAY {#TEXTURE_LAYERED_CUBEMAP_ARRAY}

```gdscript
TEXTURE_LAYERED_CUBEMAP_ARRAY = 2
```

Array of cubemap textures (see CubemapArray).

---

### CubeMapLayer {#enum-CubeMapLayer}

```gdscript
enum CubeMapLayer { CUBEMAP_LAYER_LEFT = 0, CUBEMAP_LAYER_RIGHT = 1, CUBEMAP_LAYER_BOTTOM = 2, CUBEMAP_LAYER_TOP = 3, CUBEMAP_LAYER_FRONT = 4, CUBEMAP_LAYER_BACK = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CUBEMAP_LAYER_LEFT {#CUBEMAP_LAYER_LEFT}

```gdscript
CUBEMAP_LAYER_LEFT = 0
```

Left face of a Cubemap.

#### CUBEMAP_LAYER_RIGHT {#CUBEMAP_LAYER_RIGHT}

```gdscript
CUBEMAP_LAYER_RIGHT = 1
```

Right face of a Cubemap.

#### CUBEMAP_LAYER_BOTTOM {#CUBEMAP_LAYER_BOTTOM}

```gdscript
CUBEMAP_LAYER_BOTTOM = 2
```

Bottom face of a Cubemap.

#### CUBEMAP_LAYER_TOP {#CUBEMAP_LAYER_TOP}

```gdscript
CUBEMAP_LAYER_TOP = 3
```

Top face of a Cubemap.

#### CUBEMAP_LAYER_FRONT {#CUBEMAP_LAYER_FRONT}

```gdscript
CUBEMAP_LAYER_FRONT = 4
```

Front face of a Cubemap.

#### CUBEMAP_LAYER_BACK {#CUBEMAP_LAYER_BACK}

```gdscript
CUBEMAP_LAYER_BACK = 5
```

Back face of a Cubemap.

---

### TextureDrawableFormat {#enum-TextureDrawableFormat}

```gdscript
enum TextureDrawableFormat { TEXTURE_DRAWABLE_FORMAT_RGBA8 = 0, TEXTURE_DRAWABLE_FORMAT_RGBA8_SRGB = 1, TEXTURE_DRAWABLE_FORMAT_RGBAH = 2, TEXTURE_DRAWABLE_FORMAT_RGBAF = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TEXTURE_DRAWABLE_FORMAT_RGBA8 {#TEXTURE_DRAWABLE_FORMAT_RGBA8}

```gdscript
TEXTURE_DRAWABLE_FORMAT_RGBA8 = 0
```

OpenGL texture format RGBA with four components, each with a bitdepth of 8.

#### TEXTURE_DRAWABLE_FORMAT_RGBA8_SRGB {#TEXTURE_DRAWABLE_FORMAT_RGBA8_SRGB}

```gdscript
TEXTURE_DRAWABLE_FORMAT_RGBA8_SRGB = 1
```

OpenGL texture format RGBA with four components, each with a bitdepth of 8.

When drawn to, an sRGB to linear color space conversion is performed.

#### TEXTURE_DRAWABLE_FORMAT_RGBAH {#TEXTURE_DRAWABLE_FORMAT_RGBAH}

```gdscript
TEXTURE_DRAWABLE_FORMAT_RGBAH = 2
```

OpenGL texture format GL_RGBA16F where there are four components, each a 16-bit "half-precision" floating-point value.

#### TEXTURE_DRAWABLE_FORMAT_RGBAF {#TEXTURE_DRAWABLE_FORMAT_RGBAF}

```gdscript
TEXTURE_DRAWABLE_FORMAT_RGBAF = 3
```

OpenGL texture format GL_RGBA32F where there are four components, each a 32-bit floating-point value.

---

### ShaderMode {#enum-ShaderMode}

```gdscript
enum ShaderMode { SHADER_SPATIAL = 0, SHADER_CANVAS_ITEM = 1, SHADER_PARTICLES = 2, SHADER_SKY = 3, SHADER_FOG = 4, SHADER_TEXTURE_BLIT = 5, SHADER_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHADER_SPATIAL {#SHADER_SPATIAL}

```gdscript
SHADER_SPATIAL = 0
```

Shader is a 3D shader.

#### SHADER_CANVAS_ITEM {#SHADER_CANVAS_ITEM}

```gdscript
SHADER_CANVAS_ITEM = 1
```

Shader is a 2D shader.

#### SHADER_PARTICLES {#SHADER_PARTICLES}

```gdscript
SHADER_PARTICLES = 2
```

Shader is a particle shader (can be used in both 2D and 3D).

#### SHADER_SKY {#SHADER_SKY}

```gdscript
SHADER_SKY = 3
```

Shader is a 3D sky shader.

#### SHADER_FOG {#SHADER_FOG}

```gdscript
SHADER_FOG = 4
```

Shader is a 3D fog shader.

#### SHADER_TEXTURE_BLIT {#SHADER_TEXTURE_BLIT}

```gdscript
SHADER_TEXTURE_BLIT = 5
```

Shader is a texture_blit shader.

#### SHADER_MAX {#SHADER_MAX}

```gdscript
SHADER_MAX = 6
```

Represents the size of the ShaderMode enum.

---

### ArrayType {#enum-ArrayType}

```gdscript
enum ArrayType { ARRAY_VERTEX = 0, ARRAY_NORMAL = 1, ARRAY_TANGENT = 2, ARRAY_COLOR = 3, ARRAY_TEX_UV = 4, ARRAY_TEX_UV2 = 5, ARRAY_CUSTOM0 = 6, ARRAY_CUSTOM1 = 7, ARRAY_CUSTOM2 = 8, ARRAY_CUSTOM3 = 9, ARRAY_BONES = 10, ARRAY_WEIGHTS = 11, ARRAY_INDEX = 12, ARRAY_MAX = 13 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_VERTEX {#ARRAY_VERTEX}

```gdscript
ARRAY_VERTEX = 0
```

Array is a vertex position array.

#### ARRAY_NORMAL {#ARRAY_NORMAL}

```gdscript
ARRAY_NORMAL = 1
```

Array is a normal array.

#### ARRAY_TANGENT {#ARRAY_TANGENT}

```gdscript
ARRAY_TANGENT = 2
```

Array is a tangent array.

#### ARRAY_COLOR {#ARRAY_COLOR}

```gdscript
ARRAY_COLOR = 3
```

Array is a vertex color array.

#### ARRAY_TEX_UV {#ARRAY_TEX_UV}

```gdscript
ARRAY_TEX_UV = 4
```

Array is a UV coordinates array.

#### ARRAY_TEX_UV2 {#ARRAY_TEX_UV2}

```gdscript
ARRAY_TEX_UV2 = 5
```

Array is a UV coordinates array for the second set of UV coordinates.

#### ARRAY_CUSTOM0 {#ARRAY_CUSTOM0}

```gdscript
ARRAY_CUSTOM0 = 6
```

Array is a custom data array for the first set of custom data.

#### ARRAY_CUSTOM1 {#ARRAY_CUSTOM1}

```gdscript
ARRAY_CUSTOM1 = 7
```

Array is a custom data array for the second set of custom data.

#### ARRAY_CUSTOM2 {#ARRAY_CUSTOM2}

```gdscript
ARRAY_CUSTOM2 = 8
```

Array is a custom data array for the third set of custom data.

#### ARRAY_CUSTOM3 {#ARRAY_CUSTOM3}

```gdscript
ARRAY_CUSTOM3 = 9
```

Array is a custom data array for the fourth set of custom data.

#### ARRAY_BONES {#ARRAY_BONES}

```gdscript
ARRAY_BONES = 10
```

Array contains bone information.

#### ARRAY_WEIGHTS {#ARRAY_WEIGHTS}

```gdscript
ARRAY_WEIGHTS = 11
```

Array is weight information.

#### ARRAY_INDEX {#ARRAY_INDEX}

```gdscript
ARRAY_INDEX = 12
```

Array is an index array.

#### ARRAY_MAX {#ARRAY_MAX}

```gdscript
ARRAY_MAX = 13
```

Represents the size of the ArrayType enum.

---

### ArrayCustomFormat {#enum-ArrayCustomFormat}

```gdscript
enum ArrayCustomFormat { ARRAY_CUSTOM_RGBA8_UNORM = 0, ARRAY_CUSTOM_RGBA8_SNORM = 1, ARRAY_CUSTOM_RG_HALF = 2, ARRAY_CUSTOM_RGBA_HALF = 3, ARRAY_CUSTOM_R_FLOAT = 4, ARRAY_CUSTOM_RG_FLOAT = 5, ARRAY_CUSTOM_RGB_FLOAT = 6, ARRAY_CUSTOM_RGBA_FLOAT = 7, ARRAY_CUSTOM_MAX = 8 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_CUSTOM_RGBA8_UNORM {#ARRAY_CUSTOM_RGBA8_UNORM}

```gdscript
ARRAY_CUSTOM_RGBA8_UNORM = 0
```

Custom data array contains 8-bit-per-channel red/green/blue/alpha color data. Values are normalized, unsigned floating-point in the [0.0, 1.0] range.

#### ARRAY_CUSTOM_RGBA8_SNORM {#ARRAY_CUSTOM_RGBA8_SNORM}

```gdscript
ARRAY_CUSTOM_RGBA8_SNORM = 1
```

Custom data array contains 8-bit-per-channel red/green/blue/alpha color data. Values are normalized, signed floating-point in the [-1.0, 1.0] range.

#### ARRAY_CUSTOM_RG_HALF {#ARRAY_CUSTOM_RG_HALF}

```gdscript
ARRAY_CUSTOM_RG_HALF = 2
```

Custom data array contains 16-bit-per-channel red/green color data. Values are floating-point in half precision.

#### ARRAY_CUSTOM_RGBA_HALF {#ARRAY_CUSTOM_RGBA_HALF}

```gdscript
ARRAY_CUSTOM_RGBA_HALF = 3
```

Custom data array contains 16-bit-per-channel red/green/blue/alpha color data. Values are floating-point in half precision.

#### ARRAY_CUSTOM_R_FLOAT {#ARRAY_CUSTOM_R_FLOAT}

```gdscript
ARRAY_CUSTOM_R_FLOAT = 4
```

Custom data array contains 32-bit-per-channel red color data. Values are floating-point in single precision.

#### ARRAY_CUSTOM_RG_FLOAT {#ARRAY_CUSTOM_RG_FLOAT}

```gdscript
ARRAY_CUSTOM_RG_FLOAT = 5
```

Custom data array contains 32-bit-per-channel red/green color data. Values are floating-point in single precision.

#### ARRAY_CUSTOM_RGB_FLOAT {#ARRAY_CUSTOM_RGB_FLOAT}

```gdscript
ARRAY_CUSTOM_RGB_FLOAT = 6
```

Custom data array contains 32-bit-per-channel red/green/blue color data. Values are floating-point in single precision.

#### ARRAY_CUSTOM_RGBA_FLOAT {#ARRAY_CUSTOM_RGBA_FLOAT}

```gdscript
ARRAY_CUSTOM_RGBA_FLOAT = 7
```

Custom data array contains 32-bit-per-channel red/green/blue/alpha color data. Values are floating-point in single precision.

#### ARRAY_CUSTOM_MAX {#ARRAY_CUSTOM_MAX}

```gdscript
ARRAY_CUSTOM_MAX = 8
```

Represents the size of the ArrayCustomFormat enum.

---

flags ArrayFormat:

### ArrayFormat {#enum-ArrayFormat}

```gdscript
enum ArrayFormat（位标志） { ARRAY_FORMAT_VERTEX = 1, ARRAY_FORMAT_NORMAL = 2, ARRAY_FORMAT_TANGENT = 4, ARRAY_FORMAT_COLOR = 8, ARRAY_FORMAT_TEX_UV = 16, ARRAY_FORMAT_TEX_UV2 = 32, ARRAY_FORMAT_CUSTOM0 = 64, ARRAY_FORMAT_CUSTOM1 = 128, ARRAY_FORMAT_CUSTOM2 = 256, ARRAY_FORMAT_CUSTOM3 = 512, ARRAY_FORMAT_BONES = 1024, ARRAY_FORMAT_WEIGHTS = 2048, ARRAY_FORMAT_INDEX = 4096, ARRAY_FORMAT_BLEND_SHAPE_MASK = 7, ARRAY_FORMAT_CUSTOM_BASE = 13, ARRAY_FORMAT_CUSTOM_BITS = 3, ARRAY_FORMAT_CUSTOM0_SHIFT = 13, ARRAY_FORMAT_CUSTOM1_SHIFT = 16, ARRAY_FORMAT_CUSTOM2_SHIFT = 19, ARRAY_FORMAT_CUSTOM3_SHIFT = 22, ARRAY_FORMAT_CUSTOM_MASK = 7, ARRAY_COMPRESS_FLAGS_BASE = 25, ARRAY_FLAG_USE_2D_VERTICES = 33554432, ARRAY_FLAG_USE_DYNAMIC_UPDATE = 67108864, ARRAY_FLAG_USE_8_BONE_WEIGHTS = 134217728, ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY = 268435456, ARRAY_FLAG_COMPRESS_ATTRIBUTES = 536870912, ARRAY_FLAG_FORMAT_VERSION_BASE = 35, ARRAY_FLAG_FORMAT_VERSION_SHIFT = 35, ARRAY_FLAG_FORMAT_VERSION_1 = 0, ARRAY_FLAG_FORMAT_VERSION_2 = 34359738368, ARRAY_FLAG_FORMAT_CURRENT_VERSION = 34359738368, ARRAY_FLAG_FORMAT_VERSION_MASK = 255 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ARRAY_FORMAT_VERTEX {#ARRAY_FORMAT_VERTEX}

```gdscript
ARRAY_FORMAT_VERTEX = 1
```

Flag used to mark a vertex position array.

#### ARRAY_FORMAT_NORMAL {#ARRAY_FORMAT_NORMAL}

```gdscript
ARRAY_FORMAT_NORMAL = 2
```

Flag used to mark a normal array.

#### ARRAY_FORMAT_TANGENT {#ARRAY_FORMAT_TANGENT}

```gdscript
ARRAY_FORMAT_TANGENT = 4
```

Flag used to mark a tangent array.

#### ARRAY_FORMAT_COLOR {#ARRAY_FORMAT_COLOR}

```gdscript
ARRAY_FORMAT_COLOR = 8
```

Flag used to mark a vertex color array.

#### ARRAY_FORMAT_TEX_UV {#ARRAY_FORMAT_TEX_UV}

```gdscript
ARRAY_FORMAT_TEX_UV = 16
```

Flag used to mark a UV coordinates array.

#### ARRAY_FORMAT_TEX_UV2 {#ARRAY_FORMAT_TEX_UV2}

```gdscript
ARRAY_FORMAT_TEX_UV2 = 32
```

Flag used to mark a UV coordinates array for the second UV coordinates.

#### ARRAY_FORMAT_CUSTOM0 {#ARRAY_FORMAT_CUSTOM0}

```gdscript
ARRAY_FORMAT_CUSTOM0 = 64
```

Flag used to mark an array of custom per-vertex data for the first set of custom data.

#### ARRAY_FORMAT_CUSTOM1 {#ARRAY_FORMAT_CUSTOM1}

```gdscript
ARRAY_FORMAT_CUSTOM1 = 128
```

Flag used to mark an array of custom per-vertex data for the second set of custom data.

#### ARRAY_FORMAT_CUSTOM2 {#ARRAY_FORMAT_CUSTOM2}

```gdscript
ARRAY_FORMAT_CUSTOM2 = 256
```

Flag used to mark an array of custom per-vertex data for the third set of custom data.

#### ARRAY_FORMAT_CUSTOM3 {#ARRAY_FORMAT_CUSTOM3}

```gdscript
ARRAY_FORMAT_CUSTOM3 = 512
```

Flag used to mark an array of custom per-vertex data for the fourth set of custom data.

#### ARRAY_FORMAT_BONES {#ARRAY_FORMAT_BONES}

```gdscript
ARRAY_FORMAT_BONES = 1024
```

Flag used to mark a bone information array.

#### ARRAY_FORMAT_WEIGHTS {#ARRAY_FORMAT_WEIGHTS}

```gdscript
ARRAY_FORMAT_WEIGHTS = 2048
```

Flag used to mark a weights array.

#### ARRAY_FORMAT_INDEX {#ARRAY_FORMAT_INDEX}

```gdscript
ARRAY_FORMAT_INDEX = 4096
```

Flag used to mark an index array.

#### ARRAY_FORMAT_BLEND_SHAPE_MASK {#ARRAY_FORMAT_BLEND_SHAPE_MASK}

```gdscript
ARRAY_FORMAT_BLEND_SHAPE_MASK = 7
```

Mask of mesh channels permitted in blend shapes.

#### ARRAY_FORMAT_CUSTOM_BASE {#ARRAY_FORMAT_CUSTOM_BASE}

```gdscript
ARRAY_FORMAT_CUSTOM_BASE = 13
```

Shift of first custom channel.

#### ARRAY_FORMAT_CUSTOM_BITS {#ARRAY_FORMAT_CUSTOM_BITS}

```gdscript
ARRAY_FORMAT_CUSTOM_BITS = 3
```

Number of format bits per custom channel. See ArrayCustomFormat.

#### ARRAY_FORMAT_CUSTOM0_SHIFT {#ARRAY_FORMAT_CUSTOM0_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM0_SHIFT = 13
```

Amount to shift ArrayCustomFormat for custom channel index 0.

#### ARRAY_FORMAT_CUSTOM1_SHIFT {#ARRAY_FORMAT_CUSTOM1_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM1_SHIFT = 16
```

Amount to shift ArrayCustomFormat for custom channel index 1.

#### ARRAY_FORMAT_CUSTOM2_SHIFT {#ARRAY_FORMAT_CUSTOM2_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM2_SHIFT = 19
```

Amount to shift ArrayCustomFormat for custom channel index 2.

#### ARRAY_FORMAT_CUSTOM3_SHIFT {#ARRAY_FORMAT_CUSTOM3_SHIFT}

```gdscript
ARRAY_FORMAT_CUSTOM3_SHIFT = 22
```

Amount to shift ArrayCustomFormat for custom channel index 3.

#### ARRAY_FORMAT_CUSTOM_MASK {#ARRAY_FORMAT_CUSTOM_MASK}

```gdscript
ARRAY_FORMAT_CUSTOM_MASK = 7
```

Mask of custom format bits per custom channel. Must be shifted by one of the SHIFT constants. See ArrayCustomFormat.

#### ARRAY_COMPRESS_FLAGS_BASE {#ARRAY_COMPRESS_FLAGS_BASE}

```gdscript
ARRAY_COMPRESS_FLAGS_BASE = 25
```

Shift of first compress flag. Compress flags should be passed to ArrayMesh.add_surface_from_arrays() and SurfaceTool.commit().

#### ARRAY_FLAG_USE_2D_VERTICES {#ARRAY_FLAG_USE_2D_VERTICES}

```gdscript
ARRAY_FLAG_USE_2D_VERTICES = 33554432
```

Flag used to mark that the array contains 2D vertices.

#### ARRAY_FLAG_USE_DYNAMIC_UPDATE {#ARRAY_FLAG_USE_DYNAMIC_UPDATE}

```gdscript
ARRAY_FLAG_USE_DYNAMIC_UPDATE = 67108864
```

Flag used to mark that the mesh data will use GL_DYNAMIC_DRAW on GLES. Unused on Vulkan.

#### ARRAY_FLAG_USE_8_BONE_WEIGHTS {#ARRAY_FLAG_USE_8_BONE_WEIGHTS}

```gdscript
ARRAY_FLAG_USE_8_BONE_WEIGHTS = 134217728
```

Flag used to mark that the array uses 8 bone weights instead of 4.

#### ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY {#ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY}

```gdscript
ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY = 268435456
```

Flag used to mark that the mesh does not have a vertex array and instead will infer vertex positions in the shader using indices and other information.

#### ARRAY_FLAG_COMPRESS_ATTRIBUTES {#ARRAY_FLAG_COMPRESS_ATTRIBUTES}

```gdscript
ARRAY_FLAG_COMPRESS_ATTRIBUTES = 536870912
```

Flag used to mark that a mesh is using compressed attributes (vertices, normals, tangents, UVs). When this form of compression is enabled, vertex positions will be packed into an RGBA16UNORM attribute and scaled in the vertex shader. The normal and tangent will be packed into an RG16UNORM representing an axis, and a 16-bit float stored in the A-channel of the vertex. UVs will use 16-bit normalized floats instead of full 32-bit signed floats. When using this compression mode you must use either vertices, normals, and tangents or only vertices. You cannot use normals without tangents. Importers will automatically enable this compression if they can.

#### ARRAY_FLAG_FORMAT_VERSION_BASE {#ARRAY_FLAG_FORMAT_VERSION_BASE}

```gdscript
ARRAY_FLAG_FORMAT_VERSION_BASE = 35
```

Flag used to mark the start of the bits used to store the mesh version.

#### ARRAY_FLAG_FORMAT_VERSION_SHIFT {#ARRAY_FLAG_FORMAT_VERSION_SHIFT}

```gdscript
ARRAY_FLAG_FORMAT_VERSION_SHIFT = 35
```

Flag used to shift a mesh format int to bring the version into the lowest digits.

#### ARRAY_FLAG_FORMAT_VERSION_1 {#ARRAY_FLAG_FORMAT_VERSION_1}

```gdscript
ARRAY_FLAG_FORMAT_VERSION_1 = 0
```

Flag used to record the format used by prior mesh versions before the introduction of a version.

#### ARRAY_FLAG_FORMAT_VERSION_2 {#ARRAY_FLAG_FORMAT_VERSION_2}

```gdscript
ARRAY_FLAG_FORMAT_VERSION_2 = 34359738368
```

Flag used to record the second iteration of the mesh version flag. The primary difference between this and ARRAY_FLAG_FORMAT_VERSION_1 is that this version supports ARRAY_FLAG_COMPRESS_ATTRIBUTES and in this version vertex positions are de-interleaved from normals and tangents.

#### ARRAY_FLAG_FORMAT_CURRENT_VERSION {#ARRAY_FLAG_FORMAT_CURRENT_VERSION}

```gdscript
ARRAY_FLAG_FORMAT_CURRENT_VERSION = 34359738368
```

Flag used to record the current version that the engine expects. Currently this is the same as ARRAY_FLAG_FORMAT_VERSION_2.

#### ARRAY_FLAG_FORMAT_VERSION_MASK {#ARRAY_FLAG_FORMAT_VERSION_MASK}

```gdscript
ARRAY_FLAG_FORMAT_VERSION_MASK = 255
```

Flag used to isolate the bits used for mesh version after using ARRAY_FLAG_FORMAT_VERSION_SHIFT to shift them into place.

---

### PrimitiveType {#enum-PrimitiveType}

```gdscript
enum PrimitiveType { PRIMITIVE_POINTS = 0, PRIMITIVE_LINES = 1, PRIMITIVE_LINE_STRIP = 2, PRIMITIVE_TRIANGLES = 3, PRIMITIVE_TRIANGLE_STRIP = 4, PRIMITIVE_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PRIMITIVE_POINTS {#PRIMITIVE_POINTS}

```gdscript
PRIMITIVE_POINTS = 0
```

Primitive to draw consists of points.

#### PRIMITIVE_LINES {#PRIMITIVE_LINES}

```gdscript
PRIMITIVE_LINES = 1
```

Primitive to draw consists of lines.

#### PRIMITIVE_LINE_STRIP {#PRIMITIVE_LINE_STRIP}

```gdscript
PRIMITIVE_LINE_STRIP = 2
```

Primitive to draw consists of a line strip from start to end.

#### PRIMITIVE_TRIANGLES {#PRIMITIVE_TRIANGLES}

```gdscript
PRIMITIVE_TRIANGLES = 3
```

Primitive to draw consists of triangles.

#### PRIMITIVE_TRIANGLE_STRIP {#PRIMITIVE_TRIANGLE_STRIP}

```gdscript
PRIMITIVE_TRIANGLE_STRIP = 4
```

Primitive to draw consists of a triangle strip (the last 3 vertices are always combined to make a triangle).

#### PRIMITIVE_MAX {#PRIMITIVE_MAX}

```gdscript
PRIMITIVE_MAX = 5
```

Represents the size of the PrimitiveType enum.

---

### BlendShapeMode {#enum-BlendShapeMode}

```gdscript
enum BlendShapeMode { BLEND_SHAPE_MODE_NORMALIZED = 0, BLEND_SHAPE_MODE_RELATIVE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BLEND_SHAPE_MODE_NORMALIZED {#BLEND_SHAPE_MODE_NORMALIZED}

```gdscript
BLEND_SHAPE_MODE_NORMALIZED = 0
```

Blend shapes are normalized.

#### BLEND_SHAPE_MODE_RELATIVE {#BLEND_SHAPE_MODE_RELATIVE}

```gdscript
BLEND_SHAPE_MODE_RELATIVE = 1
```

Blend shapes are relative to base weight.

---

### MultimeshTransformFormat {#enum-MultimeshTransformFormat}

```gdscript
enum MultimeshTransformFormat { MULTIMESH_TRANSFORM_2D = 0, MULTIMESH_TRANSFORM_3D = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MULTIMESH_TRANSFORM_2D {#MULTIMESH_TRANSFORM_2D}

```gdscript
MULTIMESH_TRANSFORM_2D = 0
```

Use Transform2D to store MultiMesh transform.

#### MULTIMESH_TRANSFORM_3D {#MULTIMESH_TRANSFORM_3D}

```gdscript
MULTIMESH_TRANSFORM_3D = 1
```

Use Transform3D to store MultiMesh transform.

---

### MultimeshPhysicsInterpolationQuality {#enum-MultimeshPhysicsInterpolationQuality}

```gdscript
enum MultimeshPhysicsInterpolationQuality { MULTIMESH_INTERP_QUALITY_FAST = 0, MULTIMESH_INTERP_QUALITY_HIGH = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MULTIMESH_INTERP_QUALITY_FAST {#MULTIMESH_INTERP_QUALITY_FAST}

```gdscript
MULTIMESH_INTERP_QUALITY_FAST = 0
```

MultiMesh physics interpolation favors speed over quality.

#### MULTIMESH_INTERP_QUALITY_HIGH {#MULTIMESH_INTERP_QUALITY_HIGH}

```gdscript
MULTIMESH_INTERP_QUALITY_HIGH = 1
```

MultiMesh physics interpolation favors quality over speed.

---

### LightProjectorFilter {#enum-LightProjectorFilter}

```gdscript
enum LightProjectorFilter { LIGHT_PROJECTOR_FILTER_NEAREST = 0, LIGHT_PROJECTOR_FILTER_LINEAR = 1, LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS = 2, LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS = 3, LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC = 4, LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_PROJECTOR_FILTER_NEAREST {#LIGHT_PROJECTOR_FILTER_NEAREST}

```gdscript
LIGHT_PROJECTOR_FILTER_NEAREST = 0
```

Nearest-neighbor filter for light projectors (use for pixel art light projectors). No mipmaps are used for rendering, which means light projectors at a distance will look sharp but grainy. This has roughly the same performance cost as using mipmaps.

#### LIGHT_PROJECTOR_FILTER_LINEAR {#LIGHT_PROJECTOR_FILTER_LINEAR}

```gdscript
LIGHT_PROJECTOR_FILTER_LINEAR = 1
```

Linear filter for light projectors (use for non-pixel art light projectors). No mipmaps are used for rendering, which means light projectors at a distance will look smooth but blurry. This has roughly the same performance cost as using mipmaps.

#### LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS {#LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS}

```gdscript
LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS = 2
```

Nearest-neighbor filter for light projectors (use for pixel art light projectors). Isotropic mipmaps are used for rendering, which means light projectors at a distance will look smooth but blurry. This has roughly the same performance cost as not using mipmaps.

#### LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS {#LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS}

```gdscript
LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS = 3
```

Linear filter for light projectors (use for non-pixel art light projectors). Isotropic mipmaps are used for rendering, which means light projectors at a distance will look smooth but blurry. This has roughly the same performance cost as not using mipmaps.

#### LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC {#LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC}

```gdscript
LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC = 4
```

Nearest-neighbor filter for light projectors (use for pixel art light projectors). Anisotropic mipmaps are used for rendering, which means light projectors at a distance will look smooth and sharp when viewed from oblique angles. This looks better compared to isotropic mipmaps, but is slower. The level of anisotropic filtering is defined by ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

#### LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC {#LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC}

```gdscript
LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC = 5
```

Linear filter for light projectors (use for non-pixel art light projectors). Anisotropic mipmaps are used for rendering, which means light projectors at a distance will look smooth and sharp when viewed from oblique angles. This looks better compared to isotropic mipmaps, but is slower. The level of anisotropic filtering is defined by ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

---

### LightType {#enum-LightType}

```gdscript
enum LightType { LIGHT_DIRECTIONAL = 0, LIGHT_OMNI = 1, LIGHT_SPOT = 2, LIGHT_AREA = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_DIRECTIONAL {#LIGHT_DIRECTIONAL}

```gdscript
LIGHT_DIRECTIONAL = 0
```

Directional (sun/moon) light (see DirectionalLight3D).

#### LIGHT_OMNI {#LIGHT_OMNI}

```gdscript
LIGHT_OMNI = 1
```

Omni light (see OmniLight3D).

#### LIGHT_SPOT {#LIGHT_SPOT}

```gdscript
LIGHT_SPOT = 2
```

Spot light (see SpotLight3D).

#### LIGHT_AREA {#LIGHT_AREA}

```gdscript
LIGHT_AREA = 3
```

Area light (see AreaLight3D).

---

### LightParam {#enum-LightParam}

```gdscript
enum LightParam { LIGHT_PARAM_ENERGY = 0, LIGHT_PARAM_INDIRECT_ENERGY = 1, LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY = 2, LIGHT_PARAM_SPECULAR = 3, LIGHT_PARAM_RANGE = 4, LIGHT_PARAM_SIZE = 5, LIGHT_PARAM_ATTENUATION = 6, LIGHT_PARAM_SPOT_ANGLE = 7, LIGHT_PARAM_SPOT_ATTENUATION = 8, LIGHT_PARAM_SHADOW_MAX_DISTANCE = 9, LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET = 10, LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET = 11, LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET = 12, LIGHT_PARAM_SHADOW_FADE_START = 13, LIGHT_PARAM_SHADOW_NORMAL_BIAS = 14, LIGHT_PARAM_SHADOW_BIAS = 15, LIGHT_PARAM_SHADOW_PANCAKE_SIZE = 16, LIGHT_PARAM_SHADOW_OPACITY = 17, LIGHT_PARAM_SHADOW_BLUR = 18, LIGHT_PARAM_TRANSMITTANCE_BIAS = 19, LIGHT_PARAM_INTENSITY = 20, LIGHT_PARAM_MAX = 21 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_PARAM_ENERGY {#LIGHT_PARAM_ENERGY}

```gdscript
LIGHT_PARAM_ENERGY = 0
```

The light's energy multiplier.

#### LIGHT_PARAM_INDIRECT_ENERGY {#LIGHT_PARAM_INDIRECT_ENERGY}

```gdscript
LIGHT_PARAM_INDIRECT_ENERGY = 1
```

The light's indirect energy multiplier (final indirect energy is LIGHT_PARAM_ENERGY * LIGHT_PARAM_INDIRECT_ENERGY).

#### LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY {#LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY}

```gdscript
LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY = 2
```

The light's volumetric fog energy multiplier (final volumetric fog energy is LIGHT_PARAM_ENERGY * LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY).

#### LIGHT_PARAM_SPECULAR {#LIGHT_PARAM_SPECULAR}

```gdscript
LIGHT_PARAM_SPECULAR = 3
```

The light's influence on specularity.

#### LIGHT_PARAM_RANGE {#LIGHT_PARAM_RANGE}

```gdscript
LIGHT_PARAM_RANGE = 4
```

The light's range.

#### LIGHT_PARAM_SIZE {#LIGHT_PARAM_SIZE}

```gdscript
LIGHT_PARAM_SIZE = 5
```

The size of the light when using spot light or omni light. The angular size of the light when using directional light.

#### LIGHT_PARAM_ATTENUATION {#LIGHT_PARAM_ATTENUATION}

```gdscript
LIGHT_PARAM_ATTENUATION = 6
```

The light's attenuation.

#### LIGHT_PARAM_SPOT_ANGLE {#LIGHT_PARAM_SPOT_ANGLE}

```gdscript
LIGHT_PARAM_SPOT_ANGLE = 7
```

The spotlight's angle.

#### LIGHT_PARAM_SPOT_ATTENUATION {#LIGHT_PARAM_SPOT_ATTENUATION}

```gdscript
LIGHT_PARAM_SPOT_ATTENUATION = 8
```

The spotlight's attenuation.

#### LIGHT_PARAM_SHADOW_MAX_DISTANCE {#LIGHT_PARAM_SHADOW_MAX_DISTANCE}

```gdscript
LIGHT_PARAM_SHADOW_MAX_DISTANCE = 9
```

The maximum distance for shadow splits. Increasing this value will make directional shadows visible from further away, at the cost of lower overall shadow detail and performance (since more objects need to be included in the directional shadow rendering).

#### LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET {#LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET}

```gdscript
LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET = 10
```

Proportion of shadow atlas occupied by the first split.

#### LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET {#LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET}

```gdscript
LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET = 11
```

Proportion of shadow atlas occupied by the second split.

#### LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET {#LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET}

```gdscript
LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET = 12
```

Proportion of shadow atlas occupied by the third split. The fourth split occupies the rest.

#### LIGHT_PARAM_SHADOW_FADE_START {#LIGHT_PARAM_SHADOW_FADE_START}

```gdscript
LIGHT_PARAM_SHADOW_FADE_START = 13
```

Proportion of shadow max distance where the shadow will start to fade out.

#### LIGHT_PARAM_SHADOW_NORMAL_BIAS {#LIGHT_PARAM_SHADOW_NORMAL_BIAS}

```gdscript
LIGHT_PARAM_SHADOW_NORMAL_BIAS = 14
```

Normal bias used to offset shadow lookup by object normal. Can be used to fix self-shadowing artifacts.

#### LIGHT_PARAM_SHADOW_BIAS {#LIGHT_PARAM_SHADOW_BIAS}

```gdscript
LIGHT_PARAM_SHADOW_BIAS = 15
```

Bias for the shadow lookup to fix self-shadowing artifacts.

#### LIGHT_PARAM_SHADOW_PANCAKE_SIZE {#LIGHT_PARAM_SHADOW_PANCAKE_SIZE}

```gdscript
LIGHT_PARAM_SHADOW_PANCAKE_SIZE = 16
```

Sets the size of the directional shadow pancake. The pancake offsets the start of the shadow's camera frustum to provide a higher effective depth resolution for the shadow. However, a high pancake size can cause artifacts in the shadows of large objects that are close to the edge of the frustum. Reducing the pancake size can help. Setting the size to 0 turns off the pancaking effect.

#### LIGHT_PARAM_SHADOW_OPACITY {#LIGHT_PARAM_SHADOW_OPACITY}

```gdscript
LIGHT_PARAM_SHADOW_OPACITY = 17
```

The light's shadow opacity. Values lower than 1.0 make the light appear through shadows. This can be used to fake global illumination at a low performance cost.

#### LIGHT_PARAM_SHADOW_BLUR {#LIGHT_PARAM_SHADOW_BLUR}

```gdscript
LIGHT_PARAM_SHADOW_BLUR = 18
```

Blurs the edges of the shadow. Can be used to hide pixel artifacts in low resolution shadow maps. A high value can make shadows appear grainy and can cause other unwanted artifacts. Try to keep as near default as possible.

#### LIGHT_PARAM_TRANSMITTANCE_BIAS {#LIGHT_PARAM_TRANSMITTANCE_BIAS}

```gdscript
LIGHT_PARAM_TRANSMITTANCE_BIAS = 19
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

#### LIGHT_PARAM_INTENSITY {#LIGHT_PARAM_INTENSITY}

```gdscript
LIGHT_PARAM_INTENSITY = 20
```

Constant representing the intensity of the light, measured in Lumens when dealing with a SpotLight3D or OmniLight3D, or measured in Lux with a DirectionalLight3D. Only used when ProjectSettings.rendering/lights_and_shadows/use_physical_light_units is true.

#### LIGHT_PARAM_MAX {#LIGHT_PARAM_MAX}

```gdscript
LIGHT_PARAM_MAX = 21
```

Represents the size of the LightParam enum.

---

### LightBakeMode {#enum-LightBakeMode}

```gdscript
enum LightBakeMode { LIGHT_BAKE_DISABLED = 0, LIGHT_BAKE_STATIC = 1, LIGHT_BAKE_DYNAMIC = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_BAKE_DISABLED {#LIGHT_BAKE_DISABLED}

```gdscript
LIGHT_BAKE_DISABLED = 0
```

Light is ignored when baking. This is the fastest mode, but the light will be taken into account when baking global illumination. This mode should generally be used for dynamic lights that change quickly, as the effect of global illumination is less noticeable on those lights.

#### LIGHT_BAKE_STATIC {#LIGHT_BAKE_STATIC}

```gdscript
LIGHT_BAKE_STATIC = 1
```

Light is taken into account in static baking (VoxelGI, LightmapGI, SDFGI (Environment.sdfgi_enabled)). The light can be moved around or modified, but its global illumination will not update in real-time. This is suitable for subtle changes (such as flickering torches), but generally not large changes such as toggling a light on and off.

#### LIGHT_BAKE_DYNAMIC {#LIGHT_BAKE_DYNAMIC}

```gdscript
LIGHT_BAKE_DYNAMIC = 2
```

Light is taken into account in dynamic baking (VoxelGI and SDFGI (Environment.sdfgi_enabled) only). The light can be moved around or modified with global illumination updating in real-time. The light's global illumination appearance will be slightly different compared to LIGHT_BAKE_STATIC. This has a greater performance cost compared to LIGHT_BAKE_STATIC. When using SDFGI, the update speed of dynamic lights is affected by ProjectSettings.rendering/global_illumination/sdfgi/frames_to_update_lights.

---

### LightOmniShadowMode {#enum-LightOmniShadowMode}

```gdscript
enum LightOmniShadowMode { LIGHT_OMNI_SHADOW_DUAL_PARABOLOID = 0, LIGHT_OMNI_SHADOW_CUBE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_OMNI_SHADOW_DUAL_PARABOLOID {#LIGHT_OMNI_SHADOW_DUAL_PARABOLOID}

```gdscript
LIGHT_OMNI_SHADOW_DUAL_PARABOLOID = 0
```

Use a dual paraboloid shadow map for omni lights.

#### LIGHT_OMNI_SHADOW_CUBE {#LIGHT_OMNI_SHADOW_CUBE}

```gdscript
LIGHT_OMNI_SHADOW_CUBE = 1
```

Use a cubemap shadow map for omni lights. Slower but better quality than dual paraboloid.

---

### LightDirectionalShadowMode {#enum-LightDirectionalShadowMode}

```gdscript
enum LightDirectionalShadowMode { LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL = 0, LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS = 1, LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL {#LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL}

```gdscript
LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL = 0
```

Use orthogonal shadow projection for directional light.

#### LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS {#LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS}

```gdscript
LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS = 1
```

Use 2 splits for shadow projection when using directional light.

#### LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS {#LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS}

```gdscript
LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS = 2
```

Use 4 splits for shadow projection when using directional light.

---

### LightDirectionalSkyMode {#enum-LightDirectionalSkyMode}

```gdscript
enum LightDirectionalSkyMode { LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY = 0, LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY = 1, LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY {#LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY}

```gdscript
LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY = 0
```

Use DirectionalLight3D in both sky rendering and scene lighting.

#### LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY {#LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY}

```gdscript
LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY = 1
```

Only use DirectionalLight3D in scene lighting.

#### LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY {#LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY}

```gdscript
LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY = 2
```

Only use DirectionalLight3D in sky rendering.

---

### ShadowQuality {#enum-ShadowQuality}

```gdscript
enum ShadowQuality { SHADOW_QUALITY_HARD = 0, SHADOW_QUALITY_SOFT_VERY_LOW = 1, SHADOW_QUALITY_SOFT_LOW = 2, SHADOW_QUALITY_SOFT_MEDIUM = 3, SHADOW_QUALITY_SOFT_HIGH = 4, SHADOW_QUALITY_SOFT_ULTRA = 5, SHADOW_QUALITY_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHADOW_QUALITY_HARD {#SHADOW_QUALITY_HARD}

```gdscript
SHADOW_QUALITY_HARD = 0
```

Lowest shadow filtering quality (fastest). Soft shadows are not available with this quality setting, which means the Light3D.shadow_blur property is ignored if Light3D.light_size and Light3D.light_angular_distance is 0.0.

Note: The variable shadow blur performed by Light3D.light_size and Light3D.light_angular_distance is still effective when using hard shadow filtering. In this case, Light3D.shadow_blur is taken into account. However, the results will not be blurred, instead the blur amount is treated as a maximum radius for the penumbra.

#### SHADOW_QUALITY_SOFT_VERY_LOW {#SHADOW_QUALITY_SOFT_VERY_LOW}

```gdscript
SHADOW_QUALITY_SOFT_VERY_LOW = 1
```

Very low shadow filtering quality (faster). When using this quality setting, Light3D.shadow_blur is automatically multiplied by 0.75× to avoid introducing too much noise. This division only applies to lights whose Light3D.light_size or Light3D.light_angular_distance is 0.0).

#### SHADOW_QUALITY_SOFT_LOW {#SHADOW_QUALITY_SOFT_LOW}

```gdscript
SHADOW_QUALITY_SOFT_LOW = 2
```

Low shadow filtering quality (fast).

#### SHADOW_QUALITY_SOFT_MEDIUM {#SHADOW_QUALITY_SOFT_MEDIUM}

```gdscript
SHADOW_QUALITY_SOFT_MEDIUM = 3
```

Medium low shadow filtering quality (average).

#### SHADOW_QUALITY_SOFT_HIGH {#SHADOW_QUALITY_SOFT_HIGH}

```gdscript
SHADOW_QUALITY_SOFT_HIGH = 4
```

High low shadow filtering quality (slow). When using this quality setting, Light3D.shadow_blur is automatically multiplied by 1.5× to better make use of the high sample count. This increased blur also improves the stability of dynamic object shadows. This multiplier only applies to lights whose Light3D.light_size or Light3D.light_angular_distance is 0.0).

#### SHADOW_QUALITY_SOFT_ULTRA {#SHADOW_QUALITY_SOFT_ULTRA}

```gdscript
SHADOW_QUALITY_SOFT_ULTRA = 5
```

Highest low shadow filtering quality (slowest). When using this quality setting, Light3D.shadow_blur is automatically multiplied by 2× to better make use of the high sample count. This increased blur also improves the stability of dynamic object shadows. This multiplier only applies to lights whose Light3D.light_size or Light3D.light_angular_distance is 0.0).

#### SHADOW_QUALITY_MAX {#SHADOW_QUALITY_MAX}

```gdscript
SHADOW_QUALITY_MAX = 6
```

Represents the size of the ShadowQuality enum.

---

### ReflectionProbeUpdateMode {#enum-ReflectionProbeUpdateMode}

```gdscript
enum ReflectionProbeUpdateMode { REFLECTION_PROBE_UPDATE_ONCE = 0, REFLECTION_PROBE_UPDATE_ALWAYS = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### REFLECTION_PROBE_UPDATE_ONCE {#REFLECTION_PROBE_UPDATE_ONCE}

```gdscript
REFLECTION_PROBE_UPDATE_ONCE = 0
```

Reflection probe will update reflections once and then stop.

#### REFLECTION_PROBE_UPDATE_ALWAYS {#REFLECTION_PROBE_UPDATE_ALWAYS}

```gdscript
REFLECTION_PROBE_UPDATE_ALWAYS = 1
```

Reflection probe will update each frame. This mode is necessary to capture moving objects.

---

### ReflectionProbeAmbientMode {#enum-ReflectionProbeAmbientMode}

```gdscript
enum ReflectionProbeAmbientMode { REFLECTION_PROBE_AMBIENT_DISABLED = 0, REFLECTION_PROBE_AMBIENT_ENVIRONMENT = 1, REFLECTION_PROBE_AMBIENT_COLOR = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### REFLECTION_PROBE_AMBIENT_DISABLED {#REFLECTION_PROBE_AMBIENT_DISABLED}

```gdscript
REFLECTION_PROBE_AMBIENT_DISABLED = 0
```

Do not apply any ambient lighting inside the reflection probe's box defined by its size.

#### REFLECTION_PROBE_AMBIENT_ENVIRONMENT {#REFLECTION_PROBE_AMBIENT_ENVIRONMENT}

```gdscript
REFLECTION_PROBE_AMBIENT_ENVIRONMENT = 1
```

Apply automatically-sourced environment lighting inside the reflection probe's box defined by its size.

#### REFLECTION_PROBE_AMBIENT_COLOR {#REFLECTION_PROBE_AMBIENT_COLOR}

```gdscript
REFLECTION_PROBE_AMBIENT_COLOR = 2
```

Apply custom ambient lighting inside the reflection probe's box defined by its size. See reflection_probe_set_ambient_color() and reflection_probe_set_ambient_energy().

---

### DecalTexture {#enum-DecalTexture}

```gdscript
enum DecalTexture { DECAL_TEXTURE_ALBEDO = 0, DECAL_TEXTURE_NORMAL = 1, DECAL_TEXTURE_ORM = 2, DECAL_TEXTURE_EMISSION = 3, DECAL_TEXTURE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DECAL_TEXTURE_ALBEDO {#DECAL_TEXTURE_ALBEDO}

```gdscript
DECAL_TEXTURE_ALBEDO = 0
```

Albedo texture slot in a decal (Decal.texture_albedo).

#### DECAL_TEXTURE_NORMAL {#DECAL_TEXTURE_NORMAL}

```gdscript
DECAL_TEXTURE_NORMAL = 1
```

Normal map texture slot in a decal (Decal.texture_normal).

#### DECAL_TEXTURE_ORM {#DECAL_TEXTURE_ORM}

```gdscript
DECAL_TEXTURE_ORM = 2
```

Occlusion/Roughness/Metallic texture slot in a decal (Decal.texture_orm).

#### DECAL_TEXTURE_EMISSION {#DECAL_TEXTURE_EMISSION}

```gdscript
DECAL_TEXTURE_EMISSION = 3
```

Emission texture slot in a decal (Decal.texture_emission).

#### DECAL_TEXTURE_MAX {#DECAL_TEXTURE_MAX}

```gdscript
DECAL_TEXTURE_MAX = 4
```

Represents the size of the DecalTexture enum.

---

### DecalFilter {#enum-DecalFilter}

```gdscript
enum DecalFilter { DECAL_FILTER_NEAREST = 0, DECAL_FILTER_LINEAR = 1, DECAL_FILTER_NEAREST_MIPMAPS = 2, DECAL_FILTER_LINEAR_MIPMAPS = 3, DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC = 4, DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DECAL_FILTER_NEAREST {#DECAL_FILTER_NEAREST}

```gdscript
DECAL_FILTER_NEAREST = 0
```

Nearest-neighbor filter for decals (use for pixel art decals). No mipmaps are used for rendering, which means decals at a distance will look sharp but grainy. This has roughly the same performance cost as using mipmaps.

#### DECAL_FILTER_LINEAR {#DECAL_FILTER_LINEAR}

```gdscript
DECAL_FILTER_LINEAR = 1
```

Linear filter for decals (use for non-pixel art decals). No mipmaps are used for rendering, which means decals at a distance will look smooth but blurry. This has roughly the same performance cost as using mipmaps.

#### DECAL_FILTER_NEAREST_MIPMAPS {#DECAL_FILTER_NEAREST_MIPMAPS}

```gdscript
DECAL_FILTER_NEAREST_MIPMAPS = 2
```

Nearest-neighbor filter for decals (use for pixel art decals). Isotropic mipmaps are used for rendering, which means decals at a distance will look smooth but blurry. This has roughly the same performance cost as not using mipmaps.

#### DECAL_FILTER_LINEAR_MIPMAPS {#DECAL_FILTER_LINEAR_MIPMAPS}

```gdscript
DECAL_FILTER_LINEAR_MIPMAPS = 3
```

Linear filter for decals (use for non-pixel art decals). Isotropic mipmaps are used for rendering, which means decals at a distance will look smooth but blurry. This has roughly the same performance cost as not using mipmaps.

#### DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC {#DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC}

```gdscript
DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC = 4
```

Nearest-neighbor filter for decals (use for pixel art decals). Anisotropic mipmaps are used for rendering, which means decals at a distance will look smooth and sharp when viewed from oblique angles. This looks better compared to isotropic mipmaps, but is slower. The level of anisotropic filtering is defined by ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

#### DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC {#DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC}

```gdscript
DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC = 5
```

Linear filter for decals (use for non-pixel art decals). Anisotropic mipmaps are used for rendering, which means decals at a distance will look smooth and sharp when viewed from oblique angles. This looks better compared to isotropic mipmaps, but is slower. The level of anisotropic filtering is defined by ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

---

### VoxelGIQuality {#enum-VoxelGIQuality}

```gdscript
enum VoxelGIQuality { VOXEL_GI_QUALITY_LOW = 0, VOXEL_GI_QUALITY_HIGH = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VOXEL_GI_QUALITY_LOW {#VOXEL_GI_QUALITY_LOW}

```gdscript
VOXEL_GI_QUALITY_LOW = 0
```

Low VoxelGI rendering quality using 4 cones.

#### VOXEL_GI_QUALITY_HIGH {#VOXEL_GI_QUALITY_HIGH}

```gdscript
VOXEL_GI_QUALITY_HIGH = 1
```

High VoxelGI rendering quality using 6 cones.

---

### ParticlesMode {#enum-ParticlesMode}

```gdscript
enum ParticlesMode { PARTICLES_MODE_2D = 0, PARTICLES_MODE_3D = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLES_MODE_2D {#PARTICLES_MODE_2D}

```gdscript
PARTICLES_MODE_2D = 0
```

2D particles.

#### PARTICLES_MODE_3D {#PARTICLES_MODE_3D}

```gdscript
PARTICLES_MODE_3D = 1
```

3D particles.

---

### ParticlesTransformAlign {#enum-ParticlesTransformAlign}

```gdscript
enum ParticlesTransformAlign { PARTICLES_TRANSFORM_ALIGN_DISABLED = 0, PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD = 1, PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY = 2, PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY = 3, PARTICLES_TRANSFORM_ALIGN_LOCAL_BILLBOARD = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLES_TRANSFORM_ALIGN_DISABLED {#PARTICLES_TRANSFORM_ALIGN_DISABLED}

```gdscript
PARTICLES_TRANSFORM_ALIGN_DISABLED = 0
```

Do not align particle transforms relative to the camera or velocity.

#### PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD {#PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD}

```gdscript
PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD = 1
```

Align each particle's Z axis to face the camera.

#### PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY {#PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY}

```gdscript
PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY = 2
```

Align each particle's Y axis to the velocity vector.

#### PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY {#PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY}

```gdscript
PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY = 3
```

Align each particle's Z axis to face the camera and Y axis to the velocity vector.

#### PARTICLES_TRANSFORM_ALIGN_LOCAL_BILLBOARD {#PARTICLES_TRANSFORM_ALIGN_LOCAL_BILLBOARD}

```gdscript
PARTICLES_TRANSFORM_ALIGN_LOCAL_BILLBOARD = 4
```

Billboard each particles around a local axis.

---

### ParticlesTransformAlignCustomSrc {#enum-ParticlesTransformAlignCustomSrc}

```gdscript
enum ParticlesTransformAlignCustomSrc { PARTICLES_ALIGN_CHANNEL_FILTER_DISABLED = 0, PARTICLES_ALIGN_CHANNEL_FILTER_X = 1, PARTICLES_ALIGN_CHANNEL_FILTER_Y = 2, PARTICLES_ALIGN_CHANNEL_FILTER_Z = 3, PARTICLES_ALIGN_CHANNEL_FILTER_W = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLES_ALIGN_CHANNEL_FILTER_DISABLED {#PARTICLES_ALIGN_CHANNEL_FILTER_DISABLED}

```gdscript
PARTICLES_ALIGN_CHANNEL_FILTER_DISABLED = 0
```

Do not read from CUSTOM when performing billboarding.

#### PARTICLES_ALIGN_CHANNEL_FILTER_X {#PARTICLES_ALIGN_CHANNEL_FILTER_X}

```gdscript
PARTICLES_ALIGN_CHANNEL_FILTER_X = 1
```

Read from CUSTOM.x when performing billboarding and use it as an angle, in radians.

#### PARTICLES_ALIGN_CHANNEL_FILTER_Y {#PARTICLES_ALIGN_CHANNEL_FILTER_Y}

```gdscript
PARTICLES_ALIGN_CHANNEL_FILTER_Y = 2
```

Read from CUSTOM.y when performing billboarding and use it as an angle, in radians.

#### PARTICLES_ALIGN_CHANNEL_FILTER_Z {#PARTICLES_ALIGN_CHANNEL_FILTER_Z}

```gdscript
PARTICLES_ALIGN_CHANNEL_FILTER_Z = 3
```

Read from CUSTOM.z when performing billboarding and use it as an angle, in radians.

#### PARTICLES_ALIGN_CHANNEL_FILTER_W {#PARTICLES_ALIGN_CHANNEL_FILTER_W}

```gdscript
PARTICLES_ALIGN_CHANNEL_FILTER_W = 4
```

Read from CUSTOM.w when performing billboarding and use it as an angle, in radians.

---

### ParticlesTransformAlignAxis {#enum-ParticlesTransformAlignAxis}

```gdscript
enum ParticlesTransformAlignAxis { PARTICLES_ALIGN_AXIS_X = 0, PARTICLES_ALIGN_AXIS_Y = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLES_ALIGN_AXIS_X {#PARTICLES_ALIGN_AXIS_X}

```gdscript
PARTICLES_ALIGN_AXIS_X = 0
```

Use the X axis for local billboarding.

#### PARTICLES_ALIGN_AXIS_Y {#PARTICLES_ALIGN_AXIS_Y}

```gdscript
PARTICLES_ALIGN_AXIS_Y = 1
```

Use the Y axis for local billboarding.

---

### ParticlesDrawOrder {#enum-ParticlesDrawOrder}

```gdscript
enum ParticlesDrawOrder { PARTICLES_DRAW_ORDER_INDEX = 0, PARTICLES_DRAW_ORDER_LIFETIME = 1, PARTICLES_DRAW_ORDER_REVERSE_LIFETIME = 2, PARTICLES_DRAW_ORDER_VIEW_DEPTH = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLES_DRAW_ORDER_INDEX {#PARTICLES_DRAW_ORDER_INDEX}

```gdscript
PARTICLES_DRAW_ORDER_INDEX = 0
```

Draw particles in the order that they appear in the particles array.

#### PARTICLES_DRAW_ORDER_LIFETIME {#PARTICLES_DRAW_ORDER_LIFETIME}

```gdscript
PARTICLES_DRAW_ORDER_LIFETIME = 1
```

Sort particles based on their lifetime. In other words, the particle with the highest lifetime is drawn at the front.

#### PARTICLES_DRAW_ORDER_REVERSE_LIFETIME {#PARTICLES_DRAW_ORDER_REVERSE_LIFETIME}

```gdscript
PARTICLES_DRAW_ORDER_REVERSE_LIFETIME = 2
```

Sort particles based on the inverse of their lifetime. In other words, the particle with the lowest lifetime is drawn at the front.

#### PARTICLES_DRAW_ORDER_VIEW_DEPTH {#PARTICLES_DRAW_ORDER_VIEW_DEPTH}

```gdscript
PARTICLES_DRAW_ORDER_VIEW_DEPTH = 3
```

Sort particles based on their distance to the camera.

---

### ParticlesCollisionType {#enum-ParticlesCollisionType}

```gdscript
enum ParticlesCollisionType { PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT = 0, PARTICLES_COLLISION_TYPE_BOX_ATTRACT = 1, PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT = 2, PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE = 3, PARTICLES_COLLISION_TYPE_BOX_COLLIDE = 4, PARTICLES_COLLISION_TYPE_SDF_COLLIDE = 5, PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT {#PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT}

```gdscript
PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT = 0
```

Sphere attractor type for GPUParticles3D (see GPUParticlesAttractorSphere3D).

#### PARTICLES_COLLISION_TYPE_BOX_ATTRACT {#PARTICLES_COLLISION_TYPE_BOX_ATTRACT}

```gdscript
PARTICLES_COLLISION_TYPE_BOX_ATTRACT = 1
```

Box attractor type for GPUParticles3D (see GPUParticlesAttractorBox3D).

#### PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT {#PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT}

```gdscript
PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT = 2
```

Vector field attractor type for GPUParticles3D (see GPUParticlesAttractorVectorField3D).

#### PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE {#PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE}

```gdscript
PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE = 3
```

Sphere collision type for GPUParticles3D (see GPUParticlesCollisionSphere3D).

#### PARTICLES_COLLISION_TYPE_BOX_COLLIDE {#PARTICLES_COLLISION_TYPE_BOX_COLLIDE}

```gdscript
PARTICLES_COLLISION_TYPE_BOX_COLLIDE = 4
```

Box collision type for GPUParticles3D (see GPUParticlesCollisionBox3D).

#### PARTICLES_COLLISION_TYPE_SDF_COLLIDE {#PARTICLES_COLLISION_TYPE_SDF_COLLIDE}

```gdscript
PARTICLES_COLLISION_TYPE_SDF_COLLIDE = 5
```

Signed distance field collision type for GPUParticles3D (see GPUParticlesCollisionSDF3D).

#### PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE {#PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE}

```gdscript
PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE = 6
```

Heightfield collision type for GPUParticles3D (see GPUParticlesCollisionHeightField3D).

---

### ParticlesCollisionHeightfieldResolution {#enum-ParticlesCollisionHeightfieldResolution}

```gdscript
enum ParticlesCollisionHeightfieldResolution { PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256 = 0, PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512 = 1, PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024 = 2, PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048 = 3, PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096 = 4, PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192 = 5, PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256 {#PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256}

```gdscript
PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256 = 0
```

256×256 heightfield resolution for GPUParticlesCollisionHeightField3D.

#### PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512 {#PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512}

```gdscript
PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512 = 1
```

512×512 heightfield resolution for GPUParticlesCollisionHeightField3D.

#### PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024 {#PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024}

```gdscript
PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024 = 2
```

1024×1024 heightfield resolution for GPUParticlesCollisionHeightField3D.

#### PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048 {#PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048}

```gdscript
PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048 = 3
```

2048×2048 heightfield resolution for GPUParticlesCollisionHeightField3D.

#### PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096 {#PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096}

```gdscript
PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096 = 4
```

4096×4096 heightfield resolution for GPUParticlesCollisionHeightField3D.

#### PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192 {#PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192}

```gdscript
PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192 = 5
```

8192×8192 heightfield resolution for GPUParticlesCollisionHeightField3D.

#### PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX {#PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX}

```gdscript
PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX = 6
```

Represents the size of the ParticlesCollisionHeightfieldResolution enum.

---

### FogVolumeShape {#enum-FogVolumeShape}

```gdscript
enum FogVolumeShape { FOG_VOLUME_SHAPE_ELLIPSOID = 0, FOG_VOLUME_SHAPE_CONE = 1, FOG_VOLUME_SHAPE_CYLINDER = 2, FOG_VOLUME_SHAPE_BOX = 3, FOG_VOLUME_SHAPE_WORLD = 4, FOG_VOLUME_SHAPE_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FOG_VOLUME_SHAPE_ELLIPSOID {#FOG_VOLUME_SHAPE_ELLIPSOID}

```gdscript
FOG_VOLUME_SHAPE_ELLIPSOID = 0
```

FogVolume will be shaped like an ellipsoid (stretched sphere).

#### FOG_VOLUME_SHAPE_CONE {#FOG_VOLUME_SHAPE_CONE}

```gdscript
FOG_VOLUME_SHAPE_CONE = 1
```

FogVolume will be shaped like a cone pointing upwards (in local coordinates). The cone's angle is set automatically to fill the size. The cone will be adjusted to fit within the size. Rotate the FogVolume node to reorient the cone. Non-uniform scaling via size is not supported (scale the FogVolume node instead).

#### FOG_VOLUME_SHAPE_CYLINDER {#FOG_VOLUME_SHAPE_CYLINDER}

```gdscript
FOG_VOLUME_SHAPE_CYLINDER = 2
```

FogVolume will be shaped like an upright cylinder (in local coordinates). Rotate the FogVolume node to reorient the cylinder. The cylinder will be adjusted to fit within the size. Non-uniform scaling via size is not supported (scale the FogVolume node instead).

#### FOG_VOLUME_SHAPE_BOX {#FOG_VOLUME_SHAPE_BOX}

```gdscript
FOG_VOLUME_SHAPE_BOX = 3
```

FogVolume will be shaped like a box.

#### FOG_VOLUME_SHAPE_WORLD {#FOG_VOLUME_SHAPE_WORLD}

```gdscript
FOG_VOLUME_SHAPE_WORLD = 4
```

FogVolume will have no shape, will cover the whole world and will not be culled.

#### FOG_VOLUME_SHAPE_MAX {#FOG_VOLUME_SHAPE_MAX}

```gdscript
FOG_VOLUME_SHAPE_MAX = 5
```

Represents the size of the FogVolumeShape enum.

---

### ViewportScaling3DMode {#enum-ViewportScaling3DMode}

```gdscript
enum ViewportScaling3DMode { VIEWPORT_SCALING_3D_MODE_BILINEAR = 0, VIEWPORT_SCALING_3D_MODE_FSR = 1, VIEWPORT_SCALING_3D_MODE_FSR2 = 2, VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL = 3, VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL = 4, VIEWPORT_SCALING_3D_MODE_NEAREST = 5, VIEWPORT_SCALING_3D_MODE_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_SCALING_3D_MODE_BILINEAR {#VIEWPORT_SCALING_3D_MODE_BILINEAR}

```gdscript
VIEWPORT_SCALING_3D_MODE_BILINEAR = 0
```

Use bilinear scaling for the viewport's 3D buffer. The amount of scaling can be set using Viewport.scaling_3d_scale. Values less than 1.0 will result in undersampling while values greater than 1.0 will result in supersampling. A value of 1.0 disables scaling.

#### VIEWPORT_SCALING_3D_MODE_FSR {#VIEWPORT_SCALING_3D_MODE_FSR}

```gdscript
VIEWPORT_SCALING_3D_MODE_FSR = 1
```

Use AMD FidelityFX Super Resolution 1.0 upscaling for the viewport's 3D buffer. The amount of scaling can be set using Viewport.scaling_3d_scale. Values less than 1.0 will result in the viewport being upscaled using FSR. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 disables scaling.

#### VIEWPORT_SCALING_3D_MODE_FSR2 {#VIEWPORT_SCALING_3D_MODE_FSR2}

```gdscript
VIEWPORT_SCALING_3D_MODE_FSR2 = 2
```

Use AMD FidelityFX Super Resolution 2.2 upscaling for the viewport's 3D buffer. The amount of scaling can be set using Viewport.scaling_3d_scale. Values less than 1.0 will result in the viewport being upscaled using FSR2. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 will use FSR2 at native resolution as a TAA solution.

#### VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL {#VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL}

```gdscript
VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL = 3
```

Use MetalFX spatial upscaling for the viewport's 3D buffer. The amount of scaling can be set using Viewport.scaling_3d_scale. Values less than 1.0 will result in the viewport being upscaled using MetalFX. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 disables scaling.

Note: Only supported when the Metal rendering driver is in use, which limits this scaling mode to macOS and iOS.

#### VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL {#VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL}

```gdscript
VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL = 4
```

Use MetalFX temporal upscaling for the viewport's 3D buffer. The amount of scaling can be set using Viewport.scaling_3d_scale. Values less than 1.0 will result in the viewport being upscaled using MetalFX. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 will use MetalFX at native resolution as a TAA solution.

Note: Only supported when the Metal rendering driver is in use, which limits this scaling mode to macOS and iOS.

#### VIEWPORT_SCALING_3D_MODE_NEAREST {#VIEWPORT_SCALING_3D_MODE_NEAREST}

```gdscript
VIEWPORT_SCALING_3D_MODE_NEAREST = 5
```

Use nearest-neighbor filtering for the viewport's 3D buffer. This looks crisper than VIEWPORT_SCALING_3D_MODE_BILINEAR and has no additional rendering cost. The amount of scaling can be set using Viewport.scaling_3d_scale. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 disables scaling.

Note: When using the Nearest scaling mode, to avoid uneven pixel scaling, it's highly recommended to use a value equal to an integer divisor with a dividend of 1. For example, it's best to use a scale of 0.5 (1/2), 0.3333 (1/3), 0.25 (1/4), 0.2 (1/5), and so on.

#### VIEWPORT_SCALING_3D_MODE_MAX {#VIEWPORT_SCALING_3D_MODE_MAX}

```gdscript
VIEWPORT_SCALING_3D_MODE_MAX = 6
```

Represents the size of the ViewportScaling3DMode enum.

---

### ViewportUpdateMode {#enum-ViewportUpdateMode}

```gdscript
enum ViewportUpdateMode { VIEWPORT_UPDATE_DISABLED = 0, VIEWPORT_UPDATE_ONCE = 1, VIEWPORT_UPDATE_WHEN_VISIBLE = 2, VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE = 3, VIEWPORT_UPDATE_ALWAYS = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_UPDATE_DISABLED {#VIEWPORT_UPDATE_DISABLED}

```gdscript
VIEWPORT_UPDATE_DISABLED = 0
```

Do not update the viewport's render target.

#### VIEWPORT_UPDATE_ONCE {#VIEWPORT_UPDATE_ONCE}

```gdscript
VIEWPORT_UPDATE_ONCE = 1
```

Update the viewport's render target once, then switch to VIEWPORT_UPDATE_DISABLED.

#### VIEWPORT_UPDATE_WHEN_VISIBLE {#VIEWPORT_UPDATE_WHEN_VISIBLE}

```gdscript
VIEWPORT_UPDATE_WHEN_VISIBLE = 2
```

Update the viewport's render target only when it is visible. This is the default value.

#### VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE {#VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE}

```gdscript
VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE = 3
```

Update the viewport's render target only when its parent is visible.

#### VIEWPORT_UPDATE_ALWAYS {#VIEWPORT_UPDATE_ALWAYS}

```gdscript
VIEWPORT_UPDATE_ALWAYS = 4
```

Always update the viewport's render target.

---

### ViewportClearMode {#enum-ViewportClearMode}

```gdscript
enum ViewportClearMode { VIEWPORT_CLEAR_ALWAYS = 0, VIEWPORT_CLEAR_NEVER = 1, VIEWPORT_CLEAR_ONLY_NEXT_FRAME = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_CLEAR_ALWAYS {#VIEWPORT_CLEAR_ALWAYS}

```gdscript
VIEWPORT_CLEAR_ALWAYS = 0
```

Always clear the viewport's render target before drawing.

#### VIEWPORT_CLEAR_NEVER {#VIEWPORT_CLEAR_NEVER}

```gdscript
VIEWPORT_CLEAR_NEVER = 1
```

Never clear the viewport's render target.

#### VIEWPORT_CLEAR_ONLY_NEXT_FRAME {#VIEWPORT_CLEAR_ONLY_NEXT_FRAME}

```gdscript
VIEWPORT_CLEAR_ONLY_NEXT_FRAME = 2
```

Clear the viewport's render target on the next frame, then switch to VIEWPORT_CLEAR_NEVER.

---

### ViewportEnvironmentMode {#enum-ViewportEnvironmentMode}

```gdscript
enum ViewportEnvironmentMode { VIEWPORT_ENVIRONMENT_DISABLED = 0, VIEWPORT_ENVIRONMENT_ENABLED = 1, VIEWPORT_ENVIRONMENT_INHERIT = 2, VIEWPORT_ENVIRONMENT_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_ENVIRONMENT_DISABLED {#VIEWPORT_ENVIRONMENT_DISABLED}

```gdscript
VIEWPORT_ENVIRONMENT_DISABLED = 0
```

Disable rendering of 3D environment over 2D canvas.

#### VIEWPORT_ENVIRONMENT_ENABLED {#VIEWPORT_ENVIRONMENT_ENABLED}

```gdscript
VIEWPORT_ENVIRONMENT_ENABLED = 1
```

Enable rendering of 3D environment over 2D canvas.

#### VIEWPORT_ENVIRONMENT_INHERIT {#VIEWPORT_ENVIRONMENT_INHERIT}

```gdscript
VIEWPORT_ENVIRONMENT_INHERIT = 2
```

Inherit enable/disable value from parent. If the topmost parent is also set to VIEWPORT_ENVIRONMENT_INHERIT, then this has the same behavior as VIEWPORT_ENVIRONMENT_ENABLED.

#### VIEWPORT_ENVIRONMENT_MAX {#VIEWPORT_ENVIRONMENT_MAX}

```gdscript
VIEWPORT_ENVIRONMENT_MAX = 3
```

Represents the size of the ViewportEnvironmentMode enum.

---

### ViewportSDFOversize {#enum-ViewportSDFOversize}

```gdscript
enum ViewportSDFOversize { VIEWPORT_SDF_OVERSIZE_100_PERCENT = 0, VIEWPORT_SDF_OVERSIZE_120_PERCENT = 1, VIEWPORT_SDF_OVERSIZE_150_PERCENT = 2, VIEWPORT_SDF_OVERSIZE_200_PERCENT = 3, VIEWPORT_SDF_OVERSIZE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_SDF_OVERSIZE_100_PERCENT {#VIEWPORT_SDF_OVERSIZE_100_PERCENT}

```gdscript
VIEWPORT_SDF_OVERSIZE_100_PERCENT = 0
```

Do not oversize the 2D signed distance field. Occluders may disappear when touching the viewport's edges, and GPUParticles3D collision may stop working earlier than intended. This has the lowest GPU requirements.

#### VIEWPORT_SDF_OVERSIZE_120_PERCENT {#VIEWPORT_SDF_OVERSIZE_120_PERCENT}

```gdscript
VIEWPORT_SDF_OVERSIZE_120_PERCENT = 1
```

2D signed distance field covers 20% of the viewport's size outside the viewport on each side (top, right, bottom, left).

#### VIEWPORT_SDF_OVERSIZE_150_PERCENT {#VIEWPORT_SDF_OVERSIZE_150_PERCENT}

```gdscript
VIEWPORT_SDF_OVERSIZE_150_PERCENT = 2
```

2D signed distance field covers 50% of the viewport's size outside the viewport on each side (top, right, bottom, left).

#### VIEWPORT_SDF_OVERSIZE_200_PERCENT {#VIEWPORT_SDF_OVERSIZE_200_PERCENT}

```gdscript
VIEWPORT_SDF_OVERSIZE_200_PERCENT = 3
```

2D signed distance field covers 100% of the viewport's size outside the viewport on each side (top, right, bottom, left). This has the highest GPU requirements.

#### VIEWPORT_SDF_OVERSIZE_MAX {#VIEWPORT_SDF_OVERSIZE_MAX}

```gdscript
VIEWPORT_SDF_OVERSIZE_MAX = 4
```

Represents the size of the ViewportSDFOversize enum.

---

### ViewportSDFScale {#enum-ViewportSDFScale}

```gdscript
enum ViewportSDFScale { VIEWPORT_SDF_SCALE_100_PERCENT = 0, VIEWPORT_SDF_SCALE_50_PERCENT = 1, VIEWPORT_SDF_SCALE_25_PERCENT = 2, VIEWPORT_SDF_SCALE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_SDF_SCALE_100_PERCENT {#VIEWPORT_SDF_SCALE_100_PERCENT}

```gdscript
VIEWPORT_SDF_SCALE_100_PERCENT = 0
```

Full resolution 2D signed distance field scale. This has the highest GPU requirements.

#### VIEWPORT_SDF_SCALE_50_PERCENT {#VIEWPORT_SDF_SCALE_50_PERCENT}

```gdscript
VIEWPORT_SDF_SCALE_50_PERCENT = 1
```

Half resolution 2D signed distance field scale on each axis (25% of the viewport pixel count).

#### VIEWPORT_SDF_SCALE_25_PERCENT {#VIEWPORT_SDF_SCALE_25_PERCENT}

```gdscript
VIEWPORT_SDF_SCALE_25_PERCENT = 2
```

Quarter resolution 2D signed distance field scale on each axis (6.25% of the viewport pixel count). This has the lowest GPU requirements.

#### VIEWPORT_SDF_SCALE_MAX {#VIEWPORT_SDF_SCALE_MAX}

```gdscript
VIEWPORT_SDF_SCALE_MAX = 3
```

Represents the size of the ViewportSDFScale enum.

---

### ViewportMSAA {#enum-ViewportMSAA}

```gdscript
enum ViewportMSAA { VIEWPORT_MSAA_DISABLED = 0, VIEWPORT_MSAA_2X = 1, VIEWPORT_MSAA_4X = 2, VIEWPORT_MSAA_8X = 3, VIEWPORT_MSAA_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_MSAA_DISABLED {#VIEWPORT_MSAA_DISABLED}

```gdscript
VIEWPORT_MSAA_DISABLED = 0
```

Multisample antialiasing for 3D is disabled. This is the default value, and also the fastest setting.

#### VIEWPORT_MSAA_2X {#VIEWPORT_MSAA_2X}

```gdscript
VIEWPORT_MSAA_2X = 1
```

Multisample antialiasing uses 2 samples per pixel for 3D. This has a moderate impact on performance.

#### VIEWPORT_MSAA_4X {#VIEWPORT_MSAA_4X}

```gdscript
VIEWPORT_MSAA_4X = 2
```

Multisample antialiasing uses 4 samples per pixel for 3D. This has a high impact on performance.

#### VIEWPORT_MSAA_8X {#VIEWPORT_MSAA_8X}

```gdscript
VIEWPORT_MSAA_8X = 3
```

Multisample antialiasing uses 8 samples per pixel for 3D. This has a very high impact on performance. Likely unsupported on low-end and older hardware.

#### VIEWPORT_MSAA_MAX {#VIEWPORT_MSAA_MAX}

```gdscript
VIEWPORT_MSAA_MAX = 4
```

Represents the size of the ViewportMSAA enum.

---

### ViewportAnisotropicFiltering {#enum-ViewportAnisotropicFiltering}

```gdscript
enum ViewportAnisotropicFiltering { VIEWPORT_ANISOTROPY_DISABLED = 0, VIEWPORT_ANISOTROPY_2X = 1, VIEWPORT_ANISOTROPY_4X = 2, VIEWPORT_ANISOTROPY_8X = 3, VIEWPORT_ANISOTROPY_16X = 4, VIEWPORT_ANISOTROPY_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_ANISOTROPY_DISABLED {#VIEWPORT_ANISOTROPY_DISABLED}

```gdscript
VIEWPORT_ANISOTROPY_DISABLED = 0
```

Anisotropic filtering is disabled.

#### VIEWPORT_ANISOTROPY_2X {#VIEWPORT_ANISOTROPY_2X}

```gdscript
VIEWPORT_ANISOTROPY_2X = 1
```

Use 2× anisotropic filtering.

#### VIEWPORT_ANISOTROPY_4X {#VIEWPORT_ANISOTROPY_4X}

```gdscript
VIEWPORT_ANISOTROPY_4X = 2
```

Use 4× anisotropic filtering. This is the default value.

#### VIEWPORT_ANISOTROPY_8X {#VIEWPORT_ANISOTROPY_8X}

```gdscript
VIEWPORT_ANISOTROPY_8X = 3
```

Use 8× anisotropic filtering.

#### VIEWPORT_ANISOTROPY_16X {#VIEWPORT_ANISOTROPY_16X}

```gdscript
VIEWPORT_ANISOTROPY_16X = 4
```

Use 16× anisotropic filtering.

#### VIEWPORT_ANISOTROPY_MAX {#VIEWPORT_ANISOTROPY_MAX}

```gdscript
VIEWPORT_ANISOTROPY_MAX = 5
```

Represents the size of the ViewportAnisotropicFiltering enum.

---

### ViewportScreenSpaceAA {#enum-ViewportScreenSpaceAA}

```gdscript
enum ViewportScreenSpaceAA { VIEWPORT_SCREEN_SPACE_AA_DISABLED = 0, VIEWPORT_SCREEN_SPACE_AA_FXAA = 1, VIEWPORT_SCREEN_SPACE_AA_SMAA = 2, VIEWPORT_SCREEN_SPACE_AA_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_SCREEN_SPACE_AA_DISABLED {#VIEWPORT_SCREEN_SPACE_AA_DISABLED}

```gdscript
VIEWPORT_SCREEN_SPACE_AA_DISABLED = 0
```

Do not perform any antialiasing in the full screen post-process.

#### VIEWPORT_SCREEN_SPACE_AA_FXAA {#VIEWPORT_SCREEN_SPACE_AA_FXAA}

```gdscript
VIEWPORT_SCREEN_SPACE_AA_FXAA = 1
```

Use fast approximate antialiasing. FXAA is a popular screen-space antialiasing method, which is fast but will make the image look blurry, especially at lower resolutions. It can still work relatively well at large resolutions such as 1440p and 4K.

#### VIEWPORT_SCREEN_SPACE_AA_SMAA {#VIEWPORT_SCREEN_SPACE_AA_SMAA}

```gdscript
VIEWPORT_SCREEN_SPACE_AA_SMAA = 2
```

Use subpixel morphological antialiasing. SMAA may produce clearer results than FXAA, but at a slightly higher performance cost.

#### VIEWPORT_SCREEN_SPACE_AA_MAX {#VIEWPORT_SCREEN_SPACE_AA_MAX}

```gdscript
VIEWPORT_SCREEN_SPACE_AA_MAX = 3
```

Represents the size of the ViewportScreenSpaceAA enum.

---

### ViewportOcclusionCullingBuildQuality {#enum-ViewportOcclusionCullingBuildQuality}

```gdscript
enum ViewportOcclusionCullingBuildQuality { VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW = 0, VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM = 1, VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW {#VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW}

```gdscript
VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW = 0
```

Low occlusion culling BVH build quality (as defined by Embree). Results in the lowest CPU usage, but least effective culling.

#### VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM {#VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM}

```gdscript
VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM = 1
```

Medium occlusion culling BVH build quality (as defined by Embree).

#### VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH {#VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH}

```gdscript
VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH = 2
```

High occlusion culling BVH build quality (as defined by Embree). Results in the highest CPU usage, but most effective culling.

---

### ViewportRenderInfo {#enum-ViewportRenderInfo}

```gdscript
enum ViewportRenderInfo { VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME = 0, VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME = 1, VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME = 2, VIEWPORT_RENDER_INFO_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME {#VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME}

```gdscript
VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME = 0
```

Number of objects drawn in a single frame.

#### VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME {#VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME}

```gdscript
VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME = 1
```

Number of points, lines, or triangles drawn in a single frame.

#### VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME {#VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME}

```gdscript
VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME = 2
```

Number of draw calls during this frame.

#### VIEWPORT_RENDER_INFO_MAX {#VIEWPORT_RENDER_INFO_MAX}

```gdscript
VIEWPORT_RENDER_INFO_MAX = 3
```

Represents the size of the ViewportRenderInfo enum.

---

### ViewportRenderInfoType {#enum-ViewportRenderInfoType}

```gdscript
enum ViewportRenderInfoType { VIEWPORT_RENDER_INFO_TYPE_VISIBLE = 0, VIEWPORT_RENDER_INFO_TYPE_SHADOW = 1, VIEWPORT_RENDER_INFO_TYPE_CANVAS = 2, VIEWPORT_RENDER_INFO_TYPE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_RENDER_INFO_TYPE_VISIBLE {#VIEWPORT_RENDER_INFO_TYPE_VISIBLE}

```gdscript
VIEWPORT_RENDER_INFO_TYPE_VISIBLE = 0
```

Visible render pass (excluding shadows).

#### VIEWPORT_RENDER_INFO_TYPE_SHADOW {#VIEWPORT_RENDER_INFO_TYPE_SHADOW}

```gdscript
VIEWPORT_RENDER_INFO_TYPE_SHADOW = 1
```

Shadow render pass. Objects will be rendered several times depending on the number of amounts of lights with shadows and the number of directional shadow splits.

#### VIEWPORT_RENDER_INFO_TYPE_CANVAS {#VIEWPORT_RENDER_INFO_TYPE_CANVAS}

```gdscript
VIEWPORT_RENDER_INFO_TYPE_CANVAS = 2
```

Canvas item rendering. This includes all 2D rendering.

#### VIEWPORT_RENDER_INFO_TYPE_MAX {#VIEWPORT_RENDER_INFO_TYPE_MAX}

```gdscript
VIEWPORT_RENDER_INFO_TYPE_MAX = 3
```

Represents the size of the ViewportRenderInfoType enum.

---

### ViewportDebugDraw {#enum-ViewportDebugDraw}

```gdscript
enum ViewportDebugDraw { VIEWPORT_DEBUG_DRAW_DISABLED = 0, VIEWPORT_DEBUG_DRAW_UNSHADED = 1, VIEWPORT_DEBUG_DRAW_LIGHTING = 2, VIEWPORT_DEBUG_DRAW_OVERDRAW = 3, VIEWPORT_DEBUG_DRAW_WIREFRAME = 4, VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER = 5, VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO = 6, VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING = 7, VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION = 8, VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS = 9, VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS = 10, VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE = 11, VIEWPORT_DEBUG_DRAW_SSAO = 12, VIEWPORT_DEBUG_DRAW_SSIL = 13, VIEWPORT_DEBUG_DRAW_PSSM_SPLITS = 14, VIEWPORT_DEBUG_DRAW_DECAL_ATLAS = 15, VIEWPORT_DEBUG_DRAW_SDFGI = 16, VIEWPORT_DEBUG_DRAW_SDFGI_PROBES = 17, VIEWPORT_DEBUG_DRAW_GI_BUFFER = 18, VIEWPORT_DEBUG_DRAW_DISABLE_LOD = 19, VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS = 20, VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS = 21, VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS = 22, VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES = 23, VIEWPORT_DEBUG_DRAW_OCCLUDERS = 24, VIEWPORT_DEBUG_DRAW_MOTION_VECTORS = 25, VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER = 26 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_DEBUG_DRAW_DISABLED {#VIEWPORT_DEBUG_DRAW_DISABLED}

```gdscript
VIEWPORT_DEBUG_DRAW_DISABLED = 0
```

Debug draw is disabled. Default setting.

#### VIEWPORT_DEBUG_DRAW_UNSHADED {#VIEWPORT_DEBUG_DRAW_UNSHADED}

```gdscript
VIEWPORT_DEBUG_DRAW_UNSHADED = 1
```

Objects are displayed without light information.

#### VIEWPORT_DEBUG_DRAW_LIGHTING {#VIEWPORT_DEBUG_DRAW_LIGHTING}

```gdscript
VIEWPORT_DEBUG_DRAW_LIGHTING = 2
```

Objects are displayed with only light information.

Note: When using this debug draw mode, custom shaders are ignored since all materials in the scene temporarily use a debug material. This means the result from custom shader functions (such as vertex displacement) won't be visible anymore when using this debug draw mode.

#### VIEWPORT_DEBUG_DRAW_OVERDRAW {#VIEWPORT_DEBUG_DRAW_OVERDRAW}

```gdscript
VIEWPORT_DEBUG_DRAW_OVERDRAW = 3
```

Objects are displayed semi-transparent with additive blending so you can see where they are drawing over top of one another. A higher overdraw (represented by brighter colors) means you are wasting performance on drawing pixels that are being hidden behind others.

Note: When using this debug draw mode, custom shaders are ignored since all materials in the scene temporarily use a debug material. This means the result from custom shader functions (such as vertex displacement) won't be visible anymore when using this debug draw mode.

#### VIEWPORT_DEBUG_DRAW_WIREFRAME {#VIEWPORT_DEBUG_DRAW_WIREFRAME}

```gdscript
VIEWPORT_DEBUG_DRAW_WIREFRAME = 4
```

Debug draw draws objects in wireframe.

Note: set_debug_generate_wireframes() must be called before loading any meshes for wireframes to be visible when using the Compatibility renderer.

Note: In the Compatibility renderer, backfaces are always visible when using wireframe rendering. In the Forward+ and Mobile renderers, wireframes follow the material's backface culling properties instead.

#### VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER {#VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER}

```gdscript
VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER = 5
```

Normal buffer is drawn instead of regular scene so you can see the per-pixel normals that will be used by post-processing effects.

#### VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO {#VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO}

```gdscript
VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO = 6
```

Objects are displayed with only the albedo value from VoxelGIs. Requires at least one visible VoxelGI node that has been baked to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING {#VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING}

```gdscript
VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING = 7
```

Objects are displayed with only the lighting value from VoxelGIs. Requires at least one visible VoxelGI node that has been baked to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION {#VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION}

```gdscript
VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION = 8
```

Objects are displayed with only the emission color from VoxelGIs. Requires at least one visible VoxelGI node that has been baked to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS {#VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS}

```gdscript
VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS = 9
```

Draws the shadow atlas that stores shadows from OmniLight3Ds and SpotLight3Ds in the upper left quadrant of the Viewport.

#### VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS {#VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS}

```gdscript
VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS = 10
```

Draws the shadow atlas that stores shadows from DirectionalLight3Ds in the upper left quadrant of the Viewport.

The slice of the camera frustum related to the shadow map cascade is superimposed to visualize coverage. The color of each slice matches the colors used for VIEWPORT_DEBUG_DRAW_PSSM_SPLITS. When shadow cascades are blended the overlap is taken into account when drawing the frustum slices.

The last cascade shows all frustum slices to illustrate the coverage of all slices.

#### VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE {#VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE}

```gdscript
VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE = 11
```

Draws the estimated scene luminance. This is a 1×1 texture that is generated when autoexposure is enabled to control the scene's exposure.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### VIEWPORT_DEBUG_DRAW_SSAO {#VIEWPORT_DEBUG_DRAW_SSAO}

```gdscript
VIEWPORT_DEBUG_DRAW_SSAO = 12
```

Draws the screen space ambient occlusion texture instead of the scene so that you can clearly see how it is affecting objects. In order for this display mode to work, you must have Environment.ssao_enabled set in your WorldEnvironment.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_SSIL {#VIEWPORT_DEBUG_DRAW_SSIL}

```gdscript
VIEWPORT_DEBUG_DRAW_SSIL = 13
```

Draws the screen space indirect lighting texture instead of the scene so that you can clearly see how it is affecting objects. In order for this display mode to work, you must have Environment.ssil_enabled set in your WorldEnvironment.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_PSSM_SPLITS {#VIEWPORT_DEBUG_DRAW_PSSM_SPLITS}

```gdscript
VIEWPORT_DEBUG_DRAW_PSSM_SPLITS = 14
```

Colors each PSSM split for the DirectionalLight3Ds in the scene a different color so you can see where the splits are. In order (from closest to furthest from the camera), they are colored red, green, blue, and yellow.

Note: When using this debug draw mode, custom shaders are ignored since all materials in the scene temporarily use a debug material. This means the result from custom shader functions (such as vertex displacement) won't be visible anymore when using this debug draw mode.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### VIEWPORT_DEBUG_DRAW_DECAL_ATLAS {#VIEWPORT_DEBUG_DRAW_DECAL_ATLAS}

```gdscript
VIEWPORT_DEBUG_DRAW_DECAL_ATLAS = 15
```

Draws the decal atlas that stores decal textures from Decals.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### VIEWPORT_DEBUG_DRAW_SDFGI {#VIEWPORT_DEBUG_DRAW_SDFGI}

```gdscript
VIEWPORT_DEBUG_DRAW_SDFGI = 16
```

Draws SDFGI cascade data. This is the data structure that is used to bounce lighting against and create reflections.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_SDFGI_PROBES {#VIEWPORT_DEBUG_DRAW_SDFGI_PROBES}

```gdscript
VIEWPORT_DEBUG_DRAW_SDFGI_PROBES = 17
```

Draws SDFGI probe data. This is the data structure that is used to give indirect lighting dynamic objects moving within the scene.

When in the editor, left-clicking a probe will display additional bright dots that show its occlusion information. A white dot means the light is not occluded at all at the dot's position, while a red dot means the light is fully occluded. Intermediate values are possible.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_GI_BUFFER {#VIEWPORT_DEBUG_DRAW_GI_BUFFER}

```gdscript
VIEWPORT_DEBUG_DRAW_GI_BUFFER = 18
```

Draws the global illumination buffer from VoxelGI or SDFGI. Requires VoxelGI (at least one visible baked VoxelGI node) or SDFGI (Environment.sdfgi_enabled) to be enabled to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_DISABLE_LOD {#VIEWPORT_DEBUG_DRAW_DISABLE_LOD}

```gdscript
VIEWPORT_DEBUG_DRAW_DISABLE_LOD = 19
```

Disable mesh LOD. All meshes are drawn with full detail, which can be used to compare performance.

#### VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS {#VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS}

```gdscript
VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS = 20
```

Draws the OmniLight3D cluster. Clustering determines where lights are positioned in screen-space, which allows the engine to only process these portions of the screen for lighting.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS {#VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS}

```gdscript
VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS = 21
```

Draws the SpotLight3D cluster. Clustering determines where lights are positioned in screen-space, which allows the engine to only process these portions of the screen for lighting.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS {#VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS}

```gdscript
VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS = 22
```

Draws the Decal cluster. Clustering determines where decals are positioned in screen-space, which allows the engine to only process these portions of the screen for decals.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES {#VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES}

```gdscript
VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES = 23
```

Draws the ReflectionProbe cluster. Clustering determines where reflection probes are positioned in screen-space, which allows the engine to only process these portions of the screen for reflection probes.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_OCCLUDERS {#VIEWPORT_DEBUG_DRAW_OCCLUDERS}

```gdscript
VIEWPORT_DEBUG_DRAW_OCCLUDERS = 24
```

Draws the occlusion culling buffer. This low-resolution occlusion culling buffer is rasterized on the CPU and is used to check whether instances are occluded by other objects.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### VIEWPORT_DEBUG_DRAW_MOTION_VECTORS {#VIEWPORT_DEBUG_DRAW_MOTION_VECTORS}

```gdscript
VIEWPORT_DEBUG_DRAW_MOTION_VECTORS = 25
```

Draws the motion vectors buffer. This is used by temporal antialiasing to correct for motion that occurs during gameplay.

Note: Only supported when using the Forward+ rendering method.

#### VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER {#VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER}

```gdscript
VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER = 26
```

Internal buffer is drawn instead of regular scene so you can see the per-pixel output that will be used by post-processing effects.

Note: Only supported when using the Forward+ or Mobile rendering methods.

---

### ViewportVRSMode {#enum-ViewportVRSMode}

```gdscript
enum ViewportVRSMode { VIEWPORT_VRS_DISABLED = 0, VIEWPORT_VRS_TEXTURE = 1, VIEWPORT_VRS_XR = 2, VIEWPORT_VRS_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_VRS_DISABLED {#VIEWPORT_VRS_DISABLED}

```gdscript
VIEWPORT_VRS_DISABLED = 0
```

Variable rate shading is disabled.

#### VIEWPORT_VRS_TEXTURE {#VIEWPORT_VRS_TEXTURE}

```gdscript
VIEWPORT_VRS_TEXTURE = 1
```

Variable rate shading uses a texture. Note, for stereoscopic use a texture atlas with a texture for each view.

#### VIEWPORT_VRS_XR {#VIEWPORT_VRS_XR}

```gdscript
VIEWPORT_VRS_XR = 2
```

Variable rate shading texture is supplied by the primary XRInterface. Note that this may override the update mode.

#### VIEWPORT_VRS_MAX {#VIEWPORT_VRS_MAX}

```gdscript
VIEWPORT_VRS_MAX = 3
```

Represents the size of the ViewportVRSMode enum.

---

### ViewportVRSUpdateMode {#enum-ViewportVRSUpdateMode}

```gdscript
enum ViewportVRSUpdateMode { VIEWPORT_VRS_UPDATE_DISABLED = 0, VIEWPORT_VRS_UPDATE_ONCE = 1, VIEWPORT_VRS_UPDATE_ALWAYS = 2, VIEWPORT_VRS_UPDATE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VIEWPORT_VRS_UPDATE_DISABLED {#VIEWPORT_VRS_UPDATE_DISABLED}

```gdscript
VIEWPORT_VRS_UPDATE_DISABLED = 0
```

The input texture for variable rate shading will not be processed.

#### VIEWPORT_VRS_UPDATE_ONCE {#VIEWPORT_VRS_UPDATE_ONCE}

```gdscript
VIEWPORT_VRS_UPDATE_ONCE = 1
```

The input texture for variable rate shading will be processed once.

#### VIEWPORT_VRS_UPDATE_ALWAYS {#VIEWPORT_VRS_UPDATE_ALWAYS}

```gdscript
VIEWPORT_VRS_UPDATE_ALWAYS = 2
```

The input texture for variable rate shading will be processed each frame.

#### VIEWPORT_VRS_UPDATE_MAX {#VIEWPORT_VRS_UPDATE_MAX}

```gdscript
VIEWPORT_VRS_UPDATE_MAX = 3
```

Represents the size of the ViewportVRSUpdateMode enum.

---

### SkyMode {#enum-SkyMode}

```gdscript
enum SkyMode { SKY_MODE_AUTOMATIC = 0, SKY_MODE_QUALITY = 1, SKY_MODE_INCREMENTAL = 2, SKY_MODE_REALTIME = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SKY_MODE_AUTOMATIC {#SKY_MODE_AUTOMATIC}

```gdscript
SKY_MODE_AUTOMATIC = 0
```

Automatically selects the appropriate process mode based on your sky shader. If your shader uses TIME or POSITION, this will use SKY_MODE_REALTIME. If your shader uses any of the LIGHT_* variables or any custom uniforms, this uses SKY_MODE_INCREMENTAL. Otherwise, this defaults to SKY_MODE_QUALITY.

#### SKY_MODE_QUALITY {#SKY_MODE_QUALITY}

```gdscript
SKY_MODE_QUALITY = 1
```

Uses high quality importance sampling to process the radiance map. In general, this results in much higher quality than SKY_MODE_REALTIME but takes much longer to generate. This should not be used if you plan on changing the sky at runtime. If you are finding that the reflection is not blurry enough and is showing sparkles or fireflies, try increasing ProjectSettings.rendering/reflections/sky_reflections/ggx_samples.

#### SKY_MODE_INCREMENTAL {#SKY_MODE_INCREMENTAL}

```gdscript
SKY_MODE_INCREMENTAL = 2
```

Uses the same high quality importance sampling to process the radiance map as SKY_MODE_QUALITY, but updates over several frames. The number of frames is determined by ProjectSettings.rendering/reflections/sky_reflections/roughness_layers. Use this when you need highest quality radiance maps, but have a sky that updates slowly.

#### SKY_MODE_REALTIME {#SKY_MODE_REALTIME}

```gdscript
SKY_MODE_REALTIME = 3
```

Uses the fast filtering algorithm to process the radiance map. In general this results in lower quality, but substantially faster run times. If you need better quality, but still need to update the sky every frame, consider turning on ProjectSettings.rendering/reflections/sky_reflections/fast_filter_high_quality.

Note: The fast filtering algorithm is limited to 256×256 cubemaps, so sky_set_radiance_size() must be set to 256. Otherwise, a warning is printed and the overridden radiance size is ignored.

---

### CompositorEffectFlags {#enum-CompositorEffectFlags}

```gdscript
enum CompositorEffectFlags { COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR = 1, COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH = 2, COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS = 4, COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS = 8, COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR = 16 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR {#COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR}

```gdscript
COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR = 1
```

The rendering effect requires the color buffer to be resolved if MSAA is enabled.

#### COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH {#COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH}

```gdscript
COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH = 2
```

The rendering effect requires the depth buffer to be resolved if MSAA is enabled.

#### COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS {#COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS}

```gdscript
COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS = 4
```

The rendering effect requires motion vectors to be produced.

#### COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS {#COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS}

```gdscript
COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS = 8
```

The rendering effect requires normals and roughness g-buffer to be produced (Forward+ only).

#### COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR {#COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR}

```gdscript
COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR = 16
```

The rendering effect requires specular data to be separated out (Forward+ only).

---

### CompositorEffectCallbackType {#enum-CompositorEffectCallbackType}

```gdscript
enum CompositorEffectCallbackType { COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE = 0, COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE = 1, COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY = 2, COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT = 3, COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT = 4, COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY = -1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE {#COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE}

```gdscript
COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE = 0
```

The callback is called before our opaque rendering pass, but after depth prepass (if applicable).

#### COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE {#COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE}

```gdscript
COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE = 1
```

The callback is called after our opaque rendering pass, but before our sky is rendered.

#### COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY {#COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY}

```gdscript
COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY = 2
```

The callback is called after our sky is rendered, but before our back buffers are created (and if enabled, before subsurface scattering and/or screen space reflections).

#### COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT {#COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT}

```gdscript
COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT = 3
```

The callback is called before our transparent rendering pass, but after our sky is rendered and we've created our back buffers.

#### COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT {#COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT}

```gdscript
COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT = 4
```

The callback is called after our transparent rendering pass, but before any built-in post-processing effects and output to our render target.

#### COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY {#COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY}

```gdscript
COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY = -1
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

---

### EnvironmentBG {#enum-EnvironmentBG}

```gdscript
enum EnvironmentBG { ENV_BG_CLEAR_COLOR = 0, ENV_BG_COLOR = 1, ENV_BG_SKY = 2, ENV_BG_CANVAS = 3, ENV_BG_KEEP = 4, ENV_BG_CAMERA_FEED = 5, ENV_BG_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_BG_CLEAR_COLOR {#ENV_BG_CLEAR_COLOR}

```gdscript
ENV_BG_CLEAR_COLOR = 0
```

Use the clear color as background.

#### ENV_BG_COLOR {#ENV_BG_COLOR}

```gdscript
ENV_BG_COLOR = 1
```

Use a specified color as the background.

#### ENV_BG_SKY {#ENV_BG_SKY}

```gdscript
ENV_BG_SKY = 2
```

Use a sky resource for the background.

#### ENV_BG_CANVAS {#ENV_BG_CANVAS}

```gdscript
ENV_BG_CANVAS = 3
```

Use a specified canvas layer as the background. This can be useful for instantiating a 2D scene in a 3D world.

#### ENV_BG_KEEP {#ENV_BG_KEEP}

```gdscript
ENV_BG_KEEP = 4
```

Do not clear the background, use whatever was rendered last frame as the background.

#### ENV_BG_CAMERA_FEED {#ENV_BG_CAMERA_FEED}

```gdscript
ENV_BG_CAMERA_FEED = 5
```

Displays a camera feed in the background.

#### ENV_BG_MAX {#ENV_BG_MAX}

```gdscript
ENV_BG_MAX = 6
```

Represents the size of the EnvironmentBG enum.

---

### EnvironmentAmbientSource {#enum-EnvironmentAmbientSource}

```gdscript
enum EnvironmentAmbientSource { ENV_AMBIENT_SOURCE_BG = 0, ENV_AMBIENT_SOURCE_DISABLED = 1, ENV_AMBIENT_SOURCE_COLOR = 2, ENV_AMBIENT_SOURCE_SKY = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_AMBIENT_SOURCE_BG {#ENV_AMBIENT_SOURCE_BG}

```gdscript
ENV_AMBIENT_SOURCE_BG = 0
```

Gather ambient light from whichever source is specified as the background.

#### ENV_AMBIENT_SOURCE_DISABLED {#ENV_AMBIENT_SOURCE_DISABLED}

```gdscript
ENV_AMBIENT_SOURCE_DISABLED = 1
```

Disable ambient light.

#### ENV_AMBIENT_SOURCE_COLOR {#ENV_AMBIENT_SOURCE_COLOR}

```gdscript
ENV_AMBIENT_SOURCE_COLOR = 2
```

Specify a specific Color for ambient light.

#### ENV_AMBIENT_SOURCE_SKY {#ENV_AMBIENT_SOURCE_SKY}

```gdscript
ENV_AMBIENT_SOURCE_SKY = 3
```

Gather ambient light from the Sky regardless of what the background is.

---

### EnvironmentReflectionSource {#enum-EnvironmentReflectionSource}

```gdscript
enum EnvironmentReflectionSource { ENV_REFLECTION_SOURCE_BG = 0, ENV_REFLECTION_SOURCE_DISABLED = 1, ENV_REFLECTION_SOURCE_SKY = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_REFLECTION_SOURCE_BG {#ENV_REFLECTION_SOURCE_BG}

```gdscript
ENV_REFLECTION_SOURCE_BG = 0
```

Use the background for reflections.

#### ENV_REFLECTION_SOURCE_DISABLED {#ENV_REFLECTION_SOURCE_DISABLED}

```gdscript
ENV_REFLECTION_SOURCE_DISABLED = 1
```

Disable reflections.

#### ENV_REFLECTION_SOURCE_SKY {#ENV_REFLECTION_SOURCE_SKY}

```gdscript
ENV_REFLECTION_SOURCE_SKY = 2
```

Use the Sky for reflections regardless of what the background is.

---

### EnvironmentGlowBlendMode {#enum-EnvironmentGlowBlendMode}

```gdscript
enum EnvironmentGlowBlendMode { ENV_GLOW_BLEND_MODE_ADDITIVE = 0, ENV_GLOW_BLEND_MODE_SCREEN = 1, ENV_GLOW_BLEND_MODE_SOFTLIGHT = 2, ENV_GLOW_BLEND_MODE_REPLACE = 3, ENV_GLOW_BLEND_MODE_MIX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_GLOW_BLEND_MODE_ADDITIVE {#ENV_GLOW_BLEND_MODE_ADDITIVE}

```gdscript
ENV_GLOW_BLEND_MODE_ADDITIVE = 0
```

Adds the glow effect to the scene.

#### ENV_GLOW_BLEND_MODE_SCREEN {#ENV_GLOW_BLEND_MODE_SCREEN}

```gdscript
ENV_GLOW_BLEND_MODE_SCREEN = 1
```

Adds the glow effect to the scene after modifying the glow influence based on the scene value; dark values will be highly influenced by glow and bright values will not be influenced by glow. This approach avoids bright values becoming overly bright from the glow effect. Environment.tonemap_white is used to determine the maximum scene value where the glow should have no influence. When Environment.tonemap_mode is set to Environment.TONE_MAPPER_LINEAR and Viewport.use_hdr_2d is true, the parent window's Window.get_output_max_linear_value() will be used as the maximum scene value.

#### ENV_GLOW_BLEND_MODE_SOFTLIGHT {#ENV_GLOW_BLEND_MODE_SOFTLIGHT}

```gdscript
ENV_GLOW_BLEND_MODE_SOFTLIGHT = 2
```

Adds the glow effect to the tonemapped image after modifying the glow influence based on the image value; dark values and bright values will not be influenced by glow and mid-range values will be highly influenced by glow. This approach avoids bright values becoming overly bright from the glow effect. The glow will have the largest influence on image values of 0.25 and will have no influence when applied to image values greater than 1.0.

Note: This blend mode does not support HDR output because expects a maximum output value of 1.0. It is recommended to use a different blend mode when rendering to an HDR screen.

#### ENV_GLOW_BLEND_MODE_REPLACE {#ENV_GLOW_BLEND_MODE_REPLACE}

```gdscript
ENV_GLOW_BLEND_MODE_REPLACE = 3
```

Replaces all pixels' color by the glow effect. This can be used to simulate a full-screen blur effect by tweaking the glow parameters to match the original image's brightness or to preview glow configuration in the editor.

#### ENV_GLOW_BLEND_MODE_MIX {#ENV_GLOW_BLEND_MODE_MIX}

```gdscript
ENV_GLOW_BLEND_MODE_MIX = 4
```

Mixes the glow image with the scene image. Best used with Environment.glow_bloom to avoid darkening the scene.

---

### EnvironmentFogMode {#enum-EnvironmentFogMode}

```gdscript
enum EnvironmentFogMode { ENV_FOG_MODE_EXPONENTIAL = 0, ENV_FOG_MODE_DEPTH = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_FOG_MODE_EXPONENTIAL {#ENV_FOG_MODE_EXPONENTIAL}

```gdscript
ENV_FOG_MODE_EXPONENTIAL = 0
```

Use a physically-based fog model defined primarily by fog density.

#### ENV_FOG_MODE_DEPTH {#ENV_FOG_MODE_DEPTH}

```gdscript
ENV_FOG_MODE_DEPTH = 1
```

Use a simple fog model defined by start and end positions and a custom curve. While not physically accurate, this model can be useful when you need more artistic control.

---

### EnvironmentToneMapper {#enum-EnvironmentToneMapper}

```gdscript
enum EnvironmentToneMapper { ENV_TONE_MAPPER_LINEAR = 0, ENV_TONE_MAPPER_REINHARD = 1, ENV_TONE_MAPPER_FILMIC = 2, ENV_TONE_MAPPER_ACES = 3, ENV_TONE_MAPPER_AGX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_TONE_MAPPER_LINEAR {#ENV_TONE_MAPPER_LINEAR}

```gdscript
ENV_TONE_MAPPER_LINEAR = 0
```

Does not modify color data, resulting in a linear tonemapping curve which unnaturally clips bright values, causing bright lighting to look blown out. The simplest and fastest tonemapper.

#### ENV_TONE_MAPPER_REINHARD {#ENV_TONE_MAPPER_REINHARD}

```gdscript
ENV_TONE_MAPPER_REINHARD = 1
```

A simple tonemapping curve that rolls off bright values to prevent clipping. This results in an image that can appear dull and low contrast. Slower than ENV_TONE_MAPPER_LINEAR.

Note: When Environment.tonemap_white is left at the default value of 1.0, ENV_TONE_MAPPER_REINHARD produces an identical image to ENV_TONE_MAPPER_LINEAR.

#### ENV_TONE_MAPPER_FILMIC {#ENV_TONE_MAPPER_FILMIC}

```gdscript
ENV_TONE_MAPPER_FILMIC = 2
```

Uses a film-like tonemapping curve to prevent clipping of bright values and provide better contrast than ENV_TONE_MAPPER_REINHARD. Slightly slower than ENV_TONE_MAPPER_REINHARD.

Note: This tonemapper does not support HDR output because it produces output in the SDR range. It is recommended to use a different tonemapper when rendering to an HDR screen.

#### ENV_TONE_MAPPER_ACES {#ENV_TONE_MAPPER_ACES}

```gdscript
ENV_TONE_MAPPER_ACES = 3
```

Uses a high-contrast film-like tonemapping curve and desaturates bright values for a more realistic appearance. Slightly slower than ENV_TONE_MAPPER_FILMIC.

Note: This tonemapping operator is called "ACES Fitted" in Godot 3.x.

Note: This tonemapper does not support HDR output because it produces output in the SDR range. It is recommended to use a different tonemapper when rendering to an HDR screen.

#### ENV_TONE_MAPPER_AGX {#ENV_TONE_MAPPER_AGX}

```gdscript
ENV_TONE_MAPPER_AGX = 4
```

Uses an adjustable film-like tonemapping curve and desaturates bright values for a more realistic appearance. Better than other tonemappers at maintaining the hue of colors as they become brighter. The slowest tonemapping option.

---

### EnvironmentSSRRoughnessQuality {#enum-EnvironmentSSRRoughnessQuality}

```gdscript
enum EnvironmentSSRRoughnessQuality { ENV_SSR_ROUGHNESS_QUALITY_DISABLED = 0, ENV_SSR_ROUGHNESS_QUALITY_LOW = 1, ENV_SSR_ROUGHNESS_QUALITY_MEDIUM = 2, ENV_SSR_ROUGHNESS_QUALITY_HIGH = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_SSR_ROUGHNESS_QUALITY_DISABLED {#ENV_SSR_ROUGHNESS_QUALITY_DISABLED}

```gdscript
ENV_SSR_ROUGHNESS_QUALITY_DISABLED = 0
```

Lowest quality of roughness filter for screen-space reflections. Rough materials will not have blurrier screen-space reflections compared to smooth (non-rough) materials. This is the fastest option.

#### ENV_SSR_ROUGHNESS_QUALITY_LOW {#ENV_SSR_ROUGHNESS_QUALITY_LOW}

```gdscript
ENV_SSR_ROUGHNESS_QUALITY_LOW = 1
```

Low quality of roughness filter for screen-space reflections.

#### ENV_SSR_ROUGHNESS_QUALITY_MEDIUM {#ENV_SSR_ROUGHNESS_QUALITY_MEDIUM}

```gdscript
ENV_SSR_ROUGHNESS_QUALITY_MEDIUM = 2
```

Medium quality of roughness filter for screen-space reflections.

#### ENV_SSR_ROUGHNESS_QUALITY_HIGH {#ENV_SSR_ROUGHNESS_QUALITY_HIGH}

```gdscript
ENV_SSR_ROUGHNESS_QUALITY_HIGH = 3
```

High quality of roughness filter for screen-space reflections. This is the slowest option.

---

### EnvironmentSSAOQuality {#enum-EnvironmentSSAOQuality}

```gdscript
enum EnvironmentSSAOQuality { ENV_SSAO_QUALITY_VERY_LOW = 0, ENV_SSAO_QUALITY_LOW = 1, ENV_SSAO_QUALITY_MEDIUM = 2, ENV_SSAO_QUALITY_HIGH = 3, ENV_SSAO_QUALITY_ULTRA = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_SSAO_QUALITY_VERY_LOW {#ENV_SSAO_QUALITY_VERY_LOW}

```gdscript
ENV_SSAO_QUALITY_VERY_LOW = 0
```

Lowest quality of screen-space ambient occlusion.

#### ENV_SSAO_QUALITY_LOW {#ENV_SSAO_QUALITY_LOW}

```gdscript
ENV_SSAO_QUALITY_LOW = 1
```

Low quality screen-space ambient occlusion.

#### ENV_SSAO_QUALITY_MEDIUM {#ENV_SSAO_QUALITY_MEDIUM}

```gdscript
ENV_SSAO_QUALITY_MEDIUM = 2
```

Medium quality screen-space ambient occlusion.

#### ENV_SSAO_QUALITY_HIGH {#ENV_SSAO_QUALITY_HIGH}

```gdscript
ENV_SSAO_QUALITY_HIGH = 3
```

High quality screen-space ambient occlusion.

#### ENV_SSAO_QUALITY_ULTRA {#ENV_SSAO_QUALITY_ULTRA}

```gdscript
ENV_SSAO_QUALITY_ULTRA = 4
```

Highest quality screen-space ambient occlusion. Uses the adaptive target setting which can be dynamically adjusted to smoothly balance performance and visual quality.

---

### EnvironmentSSILQuality {#enum-EnvironmentSSILQuality}

```gdscript
enum EnvironmentSSILQuality { ENV_SSIL_QUALITY_VERY_LOW = 0, ENV_SSIL_QUALITY_LOW = 1, ENV_SSIL_QUALITY_MEDIUM = 2, ENV_SSIL_QUALITY_HIGH = 3, ENV_SSIL_QUALITY_ULTRA = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_SSIL_QUALITY_VERY_LOW {#ENV_SSIL_QUALITY_VERY_LOW}

```gdscript
ENV_SSIL_QUALITY_VERY_LOW = 0
```

Lowest quality of screen-space indirect lighting.

#### ENV_SSIL_QUALITY_LOW {#ENV_SSIL_QUALITY_LOW}

```gdscript
ENV_SSIL_QUALITY_LOW = 1
```

Low quality screen-space indirect lighting.

#### ENV_SSIL_QUALITY_MEDIUM {#ENV_SSIL_QUALITY_MEDIUM}

```gdscript
ENV_SSIL_QUALITY_MEDIUM = 2
```

High quality screen-space indirect lighting.

#### ENV_SSIL_QUALITY_HIGH {#ENV_SSIL_QUALITY_HIGH}

```gdscript
ENV_SSIL_QUALITY_HIGH = 3
```

High quality screen-space indirect lighting.

#### ENV_SSIL_QUALITY_ULTRA {#ENV_SSIL_QUALITY_ULTRA}

```gdscript
ENV_SSIL_QUALITY_ULTRA = 4
```

Highest quality screen-space indirect lighting. Uses the adaptive target setting which can be dynamically adjusted to smoothly balance performance and visual quality.

---

### EnvironmentSDFGIYScale {#enum-EnvironmentSDFGIYScale}

```gdscript
enum EnvironmentSDFGIYScale { ENV_SDFGI_Y_SCALE_50_PERCENT = 0, ENV_SDFGI_Y_SCALE_75_PERCENT = 1, ENV_SDFGI_Y_SCALE_100_PERCENT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_SDFGI_Y_SCALE_50_PERCENT {#ENV_SDFGI_Y_SCALE_50_PERCENT}

```gdscript
ENV_SDFGI_Y_SCALE_50_PERCENT = 0
```

Use 50% scale for SDFGI on the Y (vertical) axis. SDFGI cells will be twice as short as they are wide. This allows providing increased GI detail and reduced light leaking with thin floors and ceilings. This is usually the best choice for scenes that don't feature much verticality.

#### ENV_SDFGI_Y_SCALE_75_PERCENT {#ENV_SDFGI_Y_SCALE_75_PERCENT}

```gdscript
ENV_SDFGI_Y_SCALE_75_PERCENT = 1
```

Use 75% scale for SDFGI on the Y (vertical) axis. This is a balance between the 50% and 100% SDFGI Y scales.

#### ENV_SDFGI_Y_SCALE_100_PERCENT {#ENV_SDFGI_Y_SCALE_100_PERCENT}

```gdscript
ENV_SDFGI_Y_SCALE_100_PERCENT = 2
```

Use 100% scale for SDFGI on the Y (vertical) axis. SDFGI cells will be as tall as they are wide. This is usually the best choice for highly vertical scenes. The downside is that light leaking may become more noticeable with thin floors and ceilings.

---

### EnvironmentSDFGIRayCount {#enum-EnvironmentSDFGIRayCount}

```gdscript
enum EnvironmentSDFGIRayCount { ENV_SDFGI_RAY_COUNT_4 = 0, ENV_SDFGI_RAY_COUNT_8 = 1, ENV_SDFGI_RAY_COUNT_16 = 2, ENV_SDFGI_RAY_COUNT_32 = 3, ENV_SDFGI_RAY_COUNT_64 = 4, ENV_SDFGI_RAY_COUNT_96 = 5, ENV_SDFGI_RAY_COUNT_128 = 6, ENV_SDFGI_RAY_COUNT_MAX = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_SDFGI_RAY_COUNT_4 {#ENV_SDFGI_RAY_COUNT_4}

```gdscript
ENV_SDFGI_RAY_COUNT_4 = 0
```

Throw 4 rays per frame when converging SDFGI. This has the lowest GPU requirements, but creates the most noisy result.

#### ENV_SDFGI_RAY_COUNT_8 {#ENV_SDFGI_RAY_COUNT_8}

```gdscript
ENV_SDFGI_RAY_COUNT_8 = 1
```

Throw 8 rays per frame when converging SDFGI.

#### ENV_SDFGI_RAY_COUNT_16 {#ENV_SDFGI_RAY_COUNT_16}

```gdscript
ENV_SDFGI_RAY_COUNT_16 = 2
```

Throw 16 rays per frame when converging SDFGI.

#### ENV_SDFGI_RAY_COUNT_32 {#ENV_SDFGI_RAY_COUNT_32}

```gdscript
ENV_SDFGI_RAY_COUNT_32 = 3
```

Throw 32 rays per frame when converging SDFGI.

#### ENV_SDFGI_RAY_COUNT_64 {#ENV_SDFGI_RAY_COUNT_64}

```gdscript
ENV_SDFGI_RAY_COUNT_64 = 4
```

Throw 64 rays per frame when converging SDFGI.

#### ENV_SDFGI_RAY_COUNT_96 {#ENV_SDFGI_RAY_COUNT_96}

```gdscript
ENV_SDFGI_RAY_COUNT_96 = 5
```

Throw 96 rays per frame when converging SDFGI. This has high GPU requirements.

#### ENV_SDFGI_RAY_COUNT_128 {#ENV_SDFGI_RAY_COUNT_128}

```gdscript
ENV_SDFGI_RAY_COUNT_128 = 6
```

Throw 128 rays per frame when converging SDFGI. This has very high GPU requirements, but creates the least noisy result.

#### ENV_SDFGI_RAY_COUNT_MAX {#ENV_SDFGI_RAY_COUNT_MAX}

```gdscript
ENV_SDFGI_RAY_COUNT_MAX = 7
```

Represents the size of the EnvironmentSDFGIRayCount enum.

---

### EnvironmentSDFGIFramesToConverge {#enum-EnvironmentSDFGIFramesToConverge}

```gdscript
enum EnvironmentSDFGIFramesToConverge { ENV_SDFGI_CONVERGE_IN_5_FRAMES = 0, ENV_SDFGI_CONVERGE_IN_10_FRAMES = 1, ENV_SDFGI_CONVERGE_IN_15_FRAMES = 2, ENV_SDFGI_CONVERGE_IN_20_FRAMES = 3, ENV_SDFGI_CONVERGE_IN_25_FRAMES = 4, ENV_SDFGI_CONVERGE_IN_30_FRAMES = 5, ENV_SDFGI_CONVERGE_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_SDFGI_CONVERGE_IN_5_FRAMES {#ENV_SDFGI_CONVERGE_IN_5_FRAMES}

```gdscript
ENV_SDFGI_CONVERGE_IN_5_FRAMES = 0
```

Converge SDFGI over 5 frames. This is the most responsive, but creates the most noisy result with a given ray count.

#### ENV_SDFGI_CONVERGE_IN_10_FRAMES {#ENV_SDFGI_CONVERGE_IN_10_FRAMES}

```gdscript
ENV_SDFGI_CONVERGE_IN_10_FRAMES = 1
```

Configure SDFGI to fully converge over 10 frames.

#### ENV_SDFGI_CONVERGE_IN_15_FRAMES {#ENV_SDFGI_CONVERGE_IN_15_FRAMES}

```gdscript
ENV_SDFGI_CONVERGE_IN_15_FRAMES = 2
```

Configure SDFGI to fully converge over 15 frames.

#### ENV_SDFGI_CONVERGE_IN_20_FRAMES {#ENV_SDFGI_CONVERGE_IN_20_FRAMES}

```gdscript
ENV_SDFGI_CONVERGE_IN_20_FRAMES = 3
```

Configure SDFGI to fully converge over 20 frames.

#### ENV_SDFGI_CONVERGE_IN_25_FRAMES {#ENV_SDFGI_CONVERGE_IN_25_FRAMES}

```gdscript
ENV_SDFGI_CONVERGE_IN_25_FRAMES = 4
```

Configure SDFGI to fully converge over 25 frames.

#### ENV_SDFGI_CONVERGE_IN_30_FRAMES {#ENV_SDFGI_CONVERGE_IN_30_FRAMES}

```gdscript
ENV_SDFGI_CONVERGE_IN_30_FRAMES = 5
```

Configure SDFGI to fully converge over 30 frames. This is the least responsive, but creates the least noisy result with a given ray count.

#### ENV_SDFGI_CONVERGE_MAX {#ENV_SDFGI_CONVERGE_MAX}

```gdscript
ENV_SDFGI_CONVERGE_MAX = 6
```

Represents the size of the EnvironmentSDFGIFramesToConverge enum.

---

### EnvironmentSDFGIFramesToUpdateLight {#enum-EnvironmentSDFGIFramesToUpdateLight}

```gdscript
enum EnvironmentSDFGIFramesToUpdateLight { ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME = 0, ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES = 1, ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES = 2, ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES = 3, ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES = 4, ENV_SDFGI_UPDATE_LIGHT_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME {#ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME}

```gdscript
ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME = 0
```

Update indirect light from dynamic lights in SDFGI over 1 frame. This is the most responsive, but has the highest GPU requirements.

#### ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES {#ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES}

```gdscript
ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES = 1
```

Update indirect light from dynamic lights in SDFGI over 2 frames.

#### ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES {#ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES}

```gdscript
ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES = 2
```

Update indirect light from dynamic lights in SDFGI over 4 frames.

#### ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES {#ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES}

```gdscript
ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES = 3
```

Update indirect light from dynamic lights in SDFGI over 8 frames.

#### ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES {#ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES}

```gdscript
ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES = 4
```

Update indirect light from dynamic lights in SDFGI over 16 frames. This is the least responsive, but has the lowest GPU requirements.

#### ENV_SDFGI_UPDATE_LIGHT_MAX {#ENV_SDFGI_UPDATE_LIGHT_MAX}

```gdscript
ENV_SDFGI_UPDATE_LIGHT_MAX = 5
```

Represents the size of the EnvironmentSDFGIFramesToUpdateLight enum.

---

### SubSurfaceScatteringQuality {#enum-SubSurfaceScatteringQuality}

```gdscript
enum SubSurfaceScatteringQuality { SUB_SURFACE_SCATTERING_QUALITY_DISABLED = 0, SUB_SURFACE_SCATTERING_QUALITY_LOW = 1, SUB_SURFACE_SCATTERING_QUALITY_MEDIUM = 2, SUB_SURFACE_SCATTERING_QUALITY_HIGH = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SUB_SURFACE_SCATTERING_QUALITY_DISABLED {#SUB_SURFACE_SCATTERING_QUALITY_DISABLED}

```gdscript
SUB_SURFACE_SCATTERING_QUALITY_DISABLED = 0
```

Disables subsurface scattering entirely, even on materials that have BaseMaterial3D.subsurf_scatter_enabled set to true. This has the lowest GPU requirements.

#### SUB_SURFACE_SCATTERING_QUALITY_LOW {#SUB_SURFACE_SCATTERING_QUALITY_LOW}

```gdscript
SUB_SURFACE_SCATTERING_QUALITY_LOW = 1
```

Low subsurface scattering quality.

#### SUB_SURFACE_SCATTERING_QUALITY_MEDIUM {#SUB_SURFACE_SCATTERING_QUALITY_MEDIUM}

```gdscript
SUB_SURFACE_SCATTERING_QUALITY_MEDIUM = 2
```

Medium subsurface scattering quality.

#### SUB_SURFACE_SCATTERING_QUALITY_HIGH {#SUB_SURFACE_SCATTERING_QUALITY_HIGH}

```gdscript
SUB_SURFACE_SCATTERING_QUALITY_HIGH = 3
```

High subsurface scattering quality. This has the highest GPU requirements.

---

### DOFBokehShape {#enum-DOFBokehShape}

```gdscript
enum DOFBokehShape { DOF_BOKEH_BOX = 0, DOF_BOKEH_HEXAGON = 1, DOF_BOKEH_CIRCLE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOF_BOKEH_BOX {#DOF_BOKEH_BOX}

```gdscript
DOF_BOKEH_BOX = 0
```

Calculate the DOF blur using a box filter. The fastest option, but results in obvious lines in blur pattern.

#### DOF_BOKEH_HEXAGON {#DOF_BOKEH_HEXAGON}

```gdscript
DOF_BOKEH_HEXAGON = 1
```

Calculates DOF blur using a hexagon shaped filter.

#### DOF_BOKEH_CIRCLE {#DOF_BOKEH_CIRCLE}

```gdscript
DOF_BOKEH_CIRCLE = 2
```

Calculates DOF blur using a circle shaped filter. Best quality and most realistic, but slowest. Use only for areas where a lot of performance can be dedicated to post-processing (e.g. cutscenes).

---

### DOFBlurQuality {#enum-DOFBlurQuality}

```gdscript
enum DOFBlurQuality { DOF_BLUR_QUALITY_VERY_LOW = 0, DOF_BLUR_QUALITY_LOW = 1, DOF_BLUR_QUALITY_MEDIUM = 2, DOF_BLUR_QUALITY_HIGH = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOF_BLUR_QUALITY_VERY_LOW {#DOF_BLUR_QUALITY_VERY_LOW}

```gdscript
DOF_BLUR_QUALITY_VERY_LOW = 0
```

Lowest quality DOF blur. This is the fastest setting, but you may be able to see filtering artifacts.

#### DOF_BLUR_QUALITY_LOW {#DOF_BLUR_QUALITY_LOW}

```gdscript
DOF_BLUR_QUALITY_LOW = 1
```

Low quality DOF blur.

#### DOF_BLUR_QUALITY_MEDIUM {#DOF_BLUR_QUALITY_MEDIUM}

```gdscript
DOF_BLUR_QUALITY_MEDIUM = 2
```

Medium quality DOF blur.

#### DOF_BLUR_QUALITY_HIGH {#DOF_BLUR_QUALITY_HIGH}

```gdscript
DOF_BLUR_QUALITY_HIGH = 3
```

Highest quality DOF blur. Results in the smoothest looking blur by taking the most samples, but is also significantly slower.

---

### InstanceType {#enum-InstanceType}

```gdscript
enum InstanceType { INSTANCE_NONE = 0, INSTANCE_MESH = 1, INSTANCE_MULTIMESH = 2, INSTANCE_PARTICLES = 3, INSTANCE_PARTICLES_COLLISION = 4, INSTANCE_LIGHT = 5, INSTANCE_REFLECTION_PROBE = 6, INSTANCE_DECAL = 7, INSTANCE_VOXEL_GI = 8, INSTANCE_LIGHTMAP = 9, INSTANCE_OCCLUDER = 10, INSTANCE_VISIBLITY_NOTIFIER = 11, INSTANCE_FOG_VOLUME = 12, INSTANCE_MAX = 13, INSTANCE_GEOMETRY_MASK = 14 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INSTANCE_NONE {#INSTANCE_NONE}

```gdscript
INSTANCE_NONE = 0
```

The instance does not have a type.

#### INSTANCE_MESH {#INSTANCE_MESH}

```gdscript
INSTANCE_MESH = 1
```

The instance is a mesh.

#### INSTANCE_MULTIMESH {#INSTANCE_MULTIMESH}

```gdscript
INSTANCE_MULTIMESH = 2
```

The instance is a multimesh.

#### INSTANCE_PARTICLES {#INSTANCE_PARTICLES}

```gdscript
INSTANCE_PARTICLES = 3
```

The instance is a particle emitter.

#### INSTANCE_PARTICLES_COLLISION {#INSTANCE_PARTICLES_COLLISION}

```gdscript
INSTANCE_PARTICLES_COLLISION = 4
```

The instance is a GPUParticles collision shape.

#### INSTANCE_LIGHT {#INSTANCE_LIGHT}

```gdscript
INSTANCE_LIGHT = 5
```

The instance is a light.

#### INSTANCE_REFLECTION_PROBE {#INSTANCE_REFLECTION_PROBE}

```gdscript
INSTANCE_REFLECTION_PROBE = 6
```

The instance is a reflection probe.

#### INSTANCE_DECAL {#INSTANCE_DECAL}

```gdscript
INSTANCE_DECAL = 7
```

The instance is a decal.

#### INSTANCE_VOXEL_GI {#INSTANCE_VOXEL_GI}

```gdscript
INSTANCE_VOXEL_GI = 8
```

The instance is a VoxelGI.

#### INSTANCE_LIGHTMAP {#INSTANCE_LIGHTMAP}

```gdscript
INSTANCE_LIGHTMAP = 9
```

The instance is a lightmap.

#### INSTANCE_OCCLUDER {#INSTANCE_OCCLUDER}

```gdscript
INSTANCE_OCCLUDER = 10
```

The instance is an occlusion culling occluder.

#### INSTANCE_VISIBLITY_NOTIFIER {#INSTANCE_VISIBLITY_NOTIFIER}

```gdscript
INSTANCE_VISIBLITY_NOTIFIER = 11
```

The instance is a visible on-screen notifier.

#### INSTANCE_FOG_VOLUME {#INSTANCE_FOG_VOLUME}

```gdscript
INSTANCE_FOG_VOLUME = 12
```

The instance is a fog volume.

#### INSTANCE_MAX {#INSTANCE_MAX}

```gdscript
INSTANCE_MAX = 13
```

Represents the size of the InstanceType enum.

#### INSTANCE_GEOMETRY_MASK {#INSTANCE_GEOMETRY_MASK}

```gdscript
INSTANCE_GEOMETRY_MASK = 14
```

A combination of the flags of geometry instances (mesh, multimesh, immediate and particles).

---

### InstanceFlags {#enum-InstanceFlags}

```gdscript
enum InstanceFlags { INSTANCE_FLAG_USE_BAKED_LIGHT = 0, INSTANCE_FLAG_USE_DYNAMIC_GI = 1, INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE = 2, INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING = 3, INSTANCE_FLAG_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INSTANCE_FLAG_USE_BAKED_LIGHT {#INSTANCE_FLAG_USE_BAKED_LIGHT}

```gdscript
INSTANCE_FLAG_USE_BAKED_LIGHT = 0
```

Allows the instance to be used in baked lighting.

#### INSTANCE_FLAG_USE_DYNAMIC_GI {#INSTANCE_FLAG_USE_DYNAMIC_GI}

```gdscript
INSTANCE_FLAG_USE_DYNAMIC_GI = 1
```

Allows the instance to be used with dynamic global illumination.

#### INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE {#INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE}

```gdscript
INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE = 2
```

When set, manually requests to draw geometry on next frame.

#### INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING {#INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING}

```gdscript
INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING = 3
```

Always draw, even if the instance would be culled by occlusion culling. Does not affect view frustum culling.

#### INSTANCE_FLAG_MAX {#INSTANCE_FLAG_MAX}

```gdscript
INSTANCE_FLAG_MAX = 4
```

Represents the size of the InstanceFlags enum.

---

### ShadowCastingSetting {#enum-ShadowCastingSetting}

```gdscript
enum ShadowCastingSetting { SHADOW_CASTING_SETTING_OFF = 0, SHADOW_CASTING_SETTING_ON = 1, SHADOW_CASTING_SETTING_DOUBLE_SIDED = 2, SHADOW_CASTING_SETTING_SHADOWS_ONLY = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHADOW_CASTING_SETTING_OFF {#SHADOW_CASTING_SETTING_OFF}

```gdscript
SHADOW_CASTING_SETTING_OFF = 0
```

Disable shadows from this instance.

#### SHADOW_CASTING_SETTING_ON {#SHADOW_CASTING_SETTING_ON}

```gdscript
SHADOW_CASTING_SETTING_ON = 1
```

Cast shadows from this instance.

#### SHADOW_CASTING_SETTING_DOUBLE_SIDED {#SHADOW_CASTING_SETTING_DOUBLE_SIDED}

```gdscript
SHADOW_CASTING_SETTING_DOUBLE_SIDED = 2
```

Disable backface culling when rendering the shadow of the object. This is slightly slower but may result in more correct shadows.

#### SHADOW_CASTING_SETTING_SHADOWS_ONLY {#SHADOW_CASTING_SETTING_SHADOWS_ONLY}

```gdscript
SHADOW_CASTING_SETTING_SHADOWS_ONLY = 3
```

Only render the shadows from the object. The object itself will not be drawn.

---

### VisibilityRangeFadeMode {#enum-VisibilityRangeFadeMode}

```gdscript
enum VisibilityRangeFadeMode { VISIBILITY_RANGE_FADE_DISABLED = 0, VISIBILITY_RANGE_FADE_SELF = 1, VISIBILITY_RANGE_FADE_DEPENDENCIES = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VISIBILITY_RANGE_FADE_DISABLED {#VISIBILITY_RANGE_FADE_DISABLED}

```gdscript
VISIBILITY_RANGE_FADE_DISABLED = 0
```

Disable visibility range fading for the given instance.

#### VISIBILITY_RANGE_FADE_SELF {#VISIBILITY_RANGE_FADE_SELF}

```gdscript
VISIBILITY_RANGE_FADE_SELF = 1
```

Fade-out the given instance when it approaches its visibility range limits.

#### VISIBILITY_RANGE_FADE_DEPENDENCIES {#VISIBILITY_RANGE_FADE_DEPENDENCIES}

```gdscript
VISIBILITY_RANGE_FADE_DEPENDENCIES = 2
```

Fade-in the given instance's dependencies when reaching its visibility range limits.

---

### BakeChannels {#enum-BakeChannels}

```gdscript
enum BakeChannels { BAKE_CHANNEL_ALBEDO_ALPHA = 0, BAKE_CHANNEL_NORMAL = 1, BAKE_CHANNEL_ORM = 2, BAKE_CHANNEL_EMISSION = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### BAKE_CHANNEL_ALBEDO_ALPHA {#BAKE_CHANNEL_ALBEDO_ALPHA}

```gdscript
BAKE_CHANNEL_ALBEDO_ALPHA = 0
```

Index of Image in array of Images returned by bake_render_uv2(). Image uses Image.FORMAT_RGBA8 and contains albedo color in the .rgb channels and alpha in the .a channel.

#### BAKE_CHANNEL_NORMAL {#BAKE_CHANNEL_NORMAL}

```gdscript
BAKE_CHANNEL_NORMAL = 1
```

Index of Image in array of Images returned by bake_render_uv2(). Image uses Image.FORMAT_RGBA8 and contains the per-pixel normal of the object in the .rgb channels and nothing in the .a channel. The per-pixel normal is encoded as normal * 0.5 + 0.5.

#### BAKE_CHANNEL_ORM {#BAKE_CHANNEL_ORM}

```gdscript
BAKE_CHANNEL_ORM = 2
```

Index of Image in array of Images returned by bake_render_uv2(). Image uses Image.FORMAT_RGBA8 and contains ambient occlusion (from material and decals only) in the .r channel, roughness in the .g channel, metallic in the .b channel and sub surface scattering amount in the .a channel.

#### BAKE_CHANNEL_EMISSION {#BAKE_CHANNEL_EMISSION}

```gdscript
BAKE_CHANNEL_EMISSION = 3
```

Index of Image in array of Images returned by bake_render_uv2(). Image uses Image.FORMAT_RGBAH and contains emission color in the .rgb channels and nothing in the .a channel.

---

### CanvasTextureChannel {#enum-CanvasTextureChannel}

```gdscript
enum CanvasTextureChannel { CANVAS_TEXTURE_CHANNEL_DIFFUSE = 0, CANVAS_TEXTURE_CHANNEL_NORMAL = 1, CANVAS_TEXTURE_CHANNEL_SPECULAR = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_TEXTURE_CHANNEL_DIFFUSE {#CANVAS_TEXTURE_CHANNEL_DIFFUSE}

```gdscript
CANVAS_TEXTURE_CHANNEL_DIFFUSE = 0
```

Diffuse canvas texture (CanvasTexture.diffuse_texture).

#### CANVAS_TEXTURE_CHANNEL_NORMAL {#CANVAS_TEXTURE_CHANNEL_NORMAL}

```gdscript
CANVAS_TEXTURE_CHANNEL_NORMAL = 1
```

Normal map canvas texture (CanvasTexture.normal_texture).

#### CANVAS_TEXTURE_CHANNEL_SPECULAR {#CANVAS_TEXTURE_CHANNEL_SPECULAR}

```gdscript
CANVAS_TEXTURE_CHANNEL_SPECULAR = 2
```

Specular map canvas texture (CanvasTexture.specular_texture).

---

### NinePatchAxisMode {#enum-NinePatchAxisMode}

```gdscript
enum NinePatchAxisMode { NINE_PATCH_STRETCH = 0, NINE_PATCH_TILE = 1, NINE_PATCH_TILE_FIT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### NINE_PATCH_STRETCH {#NINE_PATCH_STRETCH}

```gdscript
NINE_PATCH_STRETCH = 0
```

The nine patch gets stretched where needed.

#### NINE_PATCH_TILE {#NINE_PATCH_TILE}

```gdscript
NINE_PATCH_TILE = 1
```

The nine patch gets filled with tiles where needed.

#### NINE_PATCH_TILE_FIT {#NINE_PATCH_TILE_FIT}

```gdscript
NINE_PATCH_TILE_FIT = 2
```

The nine patch gets filled with tiles where needed and stretches them a bit if needed.

---

### CanvasItemTextureFilter {#enum-CanvasItemTextureFilter}

```gdscript
enum CanvasItemTextureFilter { CANVAS_ITEM_TEXTURE_FILTER_DEFAULT = 0, CANVAS_ITEM_TEXTURE_FILTER_NEAREST = 1, CANVAS_ITEM_TEXTURE_FILTER_LINEAR = 2, CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS = 3, CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS = 4, CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC = 5, CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC = 6, CANVAS_ITEM_TEXTURE_FILTER_MAX = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_ITEM_TEXTURE_FILTER_DEFAULT {#CANVAS_ITEM_TEXTURE_FILTER_DEFAULT}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_DEFAULT = 0
```

Uses the default filter mode for this Viewport.

#### CANVAS_ITEM_TEXTURE_FILTER_NEAREST {#CANVAS_ITEM_TEXTURE_FILTER_NEAREST}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_NEAREST = 1
```

The texture filter reads from the nearest pixel only. This makes the texture look pixelated from up close, and grainy from a distance (due to mipmaps not being sampled).

#### CANVAS_ITEM_TEXTURE_FILTER_LINEAR {#CANVAS_ITEM_TEXTURE_FILTER_LINEAR}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_LINEAR = 2
```

The texture filter blends between the nearest 4 pixels. This makes the texture look smooth from up close, and grainy from a distance (due to mipmaps not being sampled).

#### CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS {#CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS = 3
```

The texture filter reads from the nearest pixel and blends between the nearest 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true). This makes the texture look pixelated from up close, and smooth from a distance.

Use this for non-pixel art textures that may be viewed at a low scale (e.g. due to Camera2D zoom or sprite scaling), as mipmaps are important to smooth out pixels that are smaller than on-screen pixels.

#### CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS {#CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS = 4
```

The texture filter blends between the nearest 4 pixels and between the nearest 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true). This makes the texture look smooth from up close, and smooth from a distance.

Use this for non-pixel art textures that may be viewed at a low scale (e.g. due to Camera2D zoom or sprite scaling), as mipmaps are important to smooth out pixels that are smaller than on-screen pixels.

#### CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC {#CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC = 5
```

The texture filter reads from the nearest pixel and blends between 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true) based on the angle between the surface and the camera view. This makes the texture look pixelated from up close, and smooth from a distance. Anisotropic filtering improves texture quality on surfaces that are almost in line with the camera, but is slightly slower. The anisotropic filtering level can be changed by adjusting ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

Note: This texture filter is rarely useful in 2D projects. CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS is usually more appropriate in this case.

#### CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC {#CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC = 6
```

The texture filter blends between the nearest 4 pixels and blends between 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true) based on the angle between the surface and the camera view. This makes the texture look smooth from up close, and smooth from a distance. Anisotropic filtering improves texture quality on surfaces that are almost in line with the camera, but is slightly slower. The anisotropic filtering level can be changed by adjusting ProjectSettings.rendering/textures/default_filters/anisotropic_filtering_level.

Note: This texture filter is rarely useful in 2D projects. CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS is usually more appropriate in this case.

#### CANVAS_ITEM_TEXTURE_FILTER_MAX {#CANVAS_ITEM_TEXTURE_FILTER_MAX}

```gdscript
CANVAS_ITEM_TEXTURE_FILTER_MAX = 7
```

Max value for CanvasItemTextureFilter enum.

---

### CanvasItemTextureRepeat {#enum-CanvasItemTextureRepeat}

```gdscript
enum CanvasItemTextureRepeat { CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT = 0, CANVAS_ITEM_TEXTURE_REPEAT_DISABLED = 1, CANVAS_ITEM_TEXTURE_REPEAT_ENABLED = 2, CANVAS_ITEM_TEXTURE_REPEAT_MIRROR = 3, CANVAS_ITEM_TEXTURE_REPEAT_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT {#CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT}

```gdscript
CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT = 0
```

Uses the default repeat mode for this Viewport.

#### CANVAS_ITEM_TEXTURE_REPEAT_DISABLED {#CANVAS_ITEM_TEXTURE_REPEAT_DISABLED}

```gdscript
CANVAS_ITEM_TEXTURE_REPEAT_DISABLED = 1
```

Disables textures repeating. Instead, when reading UVs outside the 0-1 range, the value will be clamped to the edge of the texture, resulting in a stretched out look at the borders of the texture.

#### CANVAS_ITEM_TEXTURE_REPEAT_ENABLED {#CANVAS_ITEM_TEXTURE_REPEAT_ENABLED}

```gdscript
CANVAS_ITEM_TEXTURE_REPEAT_ENABLED = 2
```

Enables the texture to repeat when UV coordinates are outside the 0-1 range. If using one of the linear filtering modes, this can result in artifacts at the edges of a texture when the sampler filters across the edges of the texture.

#### CANVAS_ITEM_TEXTURE_REPEAT_MIRROR {#CANVAS_ITEM_TEXTURE_REPEAT_MIRROR}

```gdscript
CANVAS_ITEM_TEXTURE_REPEAT_MIRROR = 3
```

Flip the texture when repeating so that the edge lines up instead of abruptly changing.

#### CANVAS_ITEM_TEXTURE_REPEAT_MAX {#CANVAS_ITEM_TEXTURE_REPEAT_MAX}

```gdscript
CANVAS_ITEM_TEXTURE_REPEAT_MAX = 4
```

Max value for CanvasItemTextureRepeat enum.

---

### CanvasGroupMode {#enum-CanvasGroupMode}

```gdscript
enum CanvasGroupMode { CANVAS_GROUP_MODE_DISABLED = 0, CANVAS_GROUP_MODE_CLIP_ONLY = 1, CANVAS_GROUP_MODE_CLIP_AND_DRAW = 2, CANVAS_GROUP_MODE_TRANSPARENT = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_GROUP_MODE_DISABLED {#CANVAS_GROUP_MODE_DISABLED}

```gdscript
CANVAS_GROUP_MODE_DISABLED = 0
```

Child draws over parent and is not clipped.

#### CANVAS_GROUP_MODE_CLIP_ONLY {#CANVAS_GROUP_MODE_CLIP_ONLY}

```gdscript
CANVAS_GROUP_MODE_CLIP_ONLY = 1
```

Parent is used for the purposes of clipping only. Child is clipped to the parent's visible area, parent is not drawn.

#### CANVAS_GROUP_MODE_CLIP_AND_DRAW {#CANVAS_GROUP_MODE_CLIP_AND_DRAW}

```gdscript
CANVAS_GROUP_MODE_CLIP_AND_DRAW = 2
```

Parent is used for clipping child, but parent is also drawn underneath child as normal before clipping child to its visible area.

#### CANVAS_GROUP_MODE_TRANSPARENT {#CANVAS_GROUP_MODE_TRANSPARENT}

```gdscript
CANVAS_GROUP_MODE_TRANSPARENT = 3
```

There is currently no description for this enum. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

---

### CanvasLightMode {#enum-CanvasLightMode}

```gdscript
enum CanvasLightMode { CANVAS_LIGHT_MODE_POINT = 0, CANVAS_LIGHT_MODE_DIRECTIONAL = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_LIGHT_MODE_POINT {#CANVAS_LIGHT_MODE_POINT}

```gdscript
CANVAS_LIGHT_MODE_POINT = 0
```

2D point light (see PointLight2D).

#### CANVAS_LIGHT_MODE_DIRECTIONAL {#CANVAS_LIGHT_MODE_DIRECTIONAL}

```gdscript
CANVAS_LIGHT_MODE_DIRECTIONAL = 1
```

2D directional (sun/moon) light (see DirectionalLight2D).

---

### CanvasLightBlendMode {#enum-CanvasLightBlendMode}

```gdscript
enum CanvasLightBlendMode { CANVAS_LIGHT_BLEND_MODE_ADD = 0, CANVAS_LIGHT_BLEND_MODE_SUB = 1, CANVAS_LIGHT_BLEND_MODE_MIX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_LIGHT_BLEND_MODE_ADD {#CANVAS_LIGHT_BLEND_MODE_ADD}

```gdscript
CANVAS_LIGHT_BLEND_MODE_ADD = 0
```

Adds light color additive to the canvas.

#### CANVAS_LIGHT_BLEND_MODE_SUB {#CANVAS_LIGHT_BLEND_MODE_SUB}

```gdscript
CANVAS_LIGHT_BLEND_MODE_SUB = 1
```

Adds light color subtractive to the canvas.

#### CANVAS_LIGHT_BLEND_MODE_MIX {#CANVAS_LIGHT_BLEND_MODE_MIX}

```gdscript
CANVAS_LIGHT_BLEND_MODE_MIX = 2
```

The light adds color depending on transparency.

---

### CanvasLightShadowFilter {#enum-CanvasLightShadowFilter}

```gdscript
enum CanvasLightShadowFilter { CANVAS_LIGHT_FILTER_NONE = 0, CANVAS_LIGHT_FILTER_PCF5 = 1, CANVAS_LIGHT_FILTER_PCF13 = 2, CANVAS_LIGHT_FILTER_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_LIGHT_FILTER_NONE {#CANVAS_LIGHT_FILTER_NONE}

```gdscript
CANVAS_LIGHT_FILTER_NONE = 0
```

Do not apply a filter to canvas light shadows.

#### CANVAS_LIGHT_FILTER_PCF5 {#CANVAS_LIGHT_FILTER_PCF5}

```gdscript
CANVAS_LIGHT_FILTER_PCF5 = 1
```

Use PCF5 filtering to filter canvas light shadows.

#### CANVAS_LIGHT_FILTER_PCF13 {#CANVAS_LIGHT_FILTER_PCF13}

```gdscript
CANVAS_LIGHT_FILTER_PCF13 = 2
```

Use PCF13 filtering to filter canvas light shadows.

#### CANVAS_LIGHT_FILTER_MAX {#CANVAS_LIGHT_FILTER_MAX}

```gdscript
CANVAS_LIGHT_FILTER_MAX = 3
```

Max value of the CanvasLightShadowFilter enum.

---

### CanvasOccluderPolygonCullMode {#enum-CanvasOccluderPolygonCullMode}

```gdscript
enum CanvasOccluderPolygonCullMode { CANVAS_OCCLUDER_POLYGON_CULL_DISABLED = 0, CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE = 1, CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CANVAS_OCCLUDER_POLYGON_CULL_DISABLED {#CANVAS_OCCLUDER_POLYGON_CULL_DISABLED}

```gdscript
CANVAS_OCCLUDER_POLYGON_CULL_DISABLED = 0
```

Culling of the canvas occluder is disabled.

#### CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE {#CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE}

```gdscript
CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE = 1
```

Culling of the canvas occluder is clockwise.

#### CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE {#CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE}

```gdscript
CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE = 2
```

Culling of the canvas occluder is counterclockwise.

---

### GlobalShaderParameterType {#enum-GlobalShaderParameterType}

```gdscript
enum GlobalShaderParameterType { GLOBAL_VAR_TYPE_BOOL = 0, GLOBAL_VAR_TYPE_BVEC2 = 1, GLOBAL_VAR_TYPE_BVEC3 = 2, GLOBAL_VAR_TYPE_BVEC4 = 3, GLOBAL_VAR_TYPE_INT = 4, GLOBAL_VAR_TYPE_IVEC2 = 5, GLOBAL_VAR_TYPE_IVEC3 = 6, GLOBAL_VAR_TYPE_IVEC4 = 7, GLOBAL_VAR_TYPE_RECT2I = 8, GLOBAL_VAR_TYPE_UINT = 9, GLOBAL_VAR_TYPE_UVEC2 = 10, GLOBAL_VAR_TYPE_UVEC3 = 11, GLOBAL_VAR_TYPE_UVEC4 = 12, GLOBAL_VAR_TYPE_FLOAT = 13, GLOBAL_VAR_TYPE_VEC2 = 14, GLOBAL_VAR_TYPE_VEC3 = 15, GLOBAL_VAR_TYPE_VEC4 = 16, GLOBAL_VAR_TYPE_COLOR = 17, GLOBAL_VAR_TYPE_RECT2 = 18, GLOBAL_VAR_TYPE_MAT2 = 19, GLOBAL_VAR_TYPE_MAT3 = 20, GLOBAL_VAR_TYPE_MAT4 = 21, GLOBAL_VAR_TYPE_TRANSFORM_2D = 22, GLOBAL_VAR_TYPE_TRANSFORM = 23, GLOBAL_VAR_TYPE_SAMPLER2D = 24, GLOBAL_VAR_TYPE_SAMPLER2DARRAY = 25, GLOBAL_VAR_TYPE_SAMPLER3D = 26, GLOBAL_VAR_TYPE_SAMPLERCUBE = 27, GLOBAL_VAR_TYPE_SAMPLEREXT = 28, GLOBAL_VAR_TYPE_MAX = 29 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### GLOBAL_VAR_TYPE_BOOL {#GLOBAL_VAR_TYPE_BOOL}

```gdscript
GLOBAL_VAR_TYPE_BOOL = 0
```

Boolean global shader parameter (global uniform bool ...).

#### GLOBAL_VAR_TYPE_BVEC2 {#GLOBAL_VAR_TYPE_BVEC2}

```gdscript
GLOBAL_VAR_TYPE_BVEC2 = 1
```

2-dimensional boolean vector global shader parameter (global uniform bvec2 ...).

#### GLOBAL_VAR_TYPE_BVEC3 {#GLOBAL_VAR_TYPE_BVEC3}

```gdscript
GLOBAL_VAR_TYPE_BVEC3 = 2
```

3-dimensional boolean vector global shader parameter (global uniform bvec3 ...).

#### GLOBAL_VAR_TYPE_BVEC4 {#GLOBAL_VAR_TYPE_BVEC4}

```gdscript
GLOBAL_VAR_TYPE_BVEC4 = 3
```

4-dimensional boolean vector global shader parameter (global uniform bvec4 ...).

#### GLOBAL_VAR_TYPE_INT {#GLOBAL_VAR_TYPE_INT}

```gdscript
GLOBAL_VAR_TYPE_INT = 4
```

Integer global shader parameter (global uniform int ...).

#### GLOBAL_VAR_TYPE_IVEC2 {#GLOBAL_VAR_TYPE_IVEC2}

```gdscript
GLOBAL_VAR_TYPE_IVEC2 = 5
```

2-dimensional integer vector global shader parameter (global uniform ivec2 ...).

#### GLOBAL_VAR_TYPE_IVEC3 {#GLOBAL_VAR_TYPE_IVEC3}

```gdscript
GLOBAL_VAR_TYPE_IVEC3 = 6
```

3-dimensional integer vector global shader parameter (global uniform ivec3 ...).

#### GLOBAL_VAR_TYPE_IVEC4 {#GLOBAL_VAR_TYPE_IVEC4}

```gdscript
GLOBAL_VAR_TYPE_IVEC4 = 7
```

4-dimensional integer vector global shader parameter (global uniform ivec4 ...).

#### GLOBAL_VAR_TYPE_RECT2I {#GLOBAL_VAR_TYPE_RECT2I}

```gdscript
GLOBAL_VAR_TYPE_RECT2I = 8
```

2-dimensional integer rectangle global shader parameter (global uniform ivec4 ...). Equivalent to GLOBAL_VAR_TYPE_IVEC4 in shader code, but exposed as a Rect2i in the editor UI.

#### GLOBAL_VAR_TYPE_UINT {#GLOBAL_VAR_TYPE_UINT}

```gdscript
GLOBAL_VAR_TYPE_UINT = 9
```

Unsigned integer global shader parameter (global uniform uint ...).

#### GLOBAL_VAR_TYPE_UVEC2 {#GLOBAL_VAR_TYPE_UVEC2}

```gdscript
GLOBAL_VAR_TYPE_UVEC2 = 10
```

2-dimensional unsigned integer vector global shader parameter (global uniform uvec2 ...).

#### GLOBAL_VAR_TYPE_UVEC3 {#GLOBAL_VAR_TYPE_UVEC3}

```gdscript
GLOBAL_VAR_TYPE_UVEC3 = 11
```

3-dimensional unsigned integer vector global shader parameter (global uniform uvec3 ...).

#### GLOBAL_VAR_TYPE_UVEC4 {#GLOBAL_VAR_TYPE_UVEC4}

```gdscript
GLOBAL_VAR_TYPE_UVEC4 = 12
```

4-dimensional unsigned integer vector global shader parameter (global uniform uvec4 ...).

#### GLOBAL_VAR_TYPE_FLOAT {#GLOBAL_VAR_TYPE_FLOAT}

```gdscript
GLOBAL_VAR_TYPE_FLOAT = 13
```

Single-precision floating-point global shader parameter (global uniform float ...).

#### GLOBAL_VAR_TYPE_VEC2 {#GLOBAL_VAR_TYPE_VEC2}

```gdscript
GLOBAL_VAR_TYPE_VEC2 = 14
```

2-dimensional floating-point vector global shader parameter (global uniform vec2 ...).

#### GLOBAL_VAR_TYPE_VEC3 {#GLOBAL_VAR_TYPE_VEC3}

```gdscript
GLOBAL_VAR_TYPE_VEC3 = 15
```

3-dimensional floating-point vector global shader parameter (global uniform vec3 ...).

#### GLOBAL_VAR_TYPE_VEC4 {#GLOBAL_VAR_TYPE_VEC4}

```gdscript
GLOBAL_VAR_TYPE_VEC4 = 16
```

4-dimensional floating-point vector global shader parameter (global uniform vec4 ...).

#### GLOBAL_VAR_TYPE_COLOR {#GLOBAL_VAR_TYPE_COLOR}

```gdscript
GLOBAL_VAR_TYPE_COLOR = 17
```

Color global shader parameter (global uniform vec4 ...). Equivalent to GLOBAL_VAR_TYPE_VEC4 in shader code, but exposed as a Color in the editor UI.

#### GLOBAL_VAR_TYPE_RECT2 {#GLOBAL_VAR_TYPE_RECT2}

```gdscript
GLOBAL_VAR_TYPE_RECT2 = 18
```

2-dimensional floating-point rectangle global shader parameter (global uniform vec4 ...). Equivalent to GLOBAL_VAR_TYPE_VEC4 in shader code, but exposed as a Rect2 in the editor UI.

#### GLOBAL_VAR_TYPE_MAT2 {#GLOBAL_VAR_TYPE_MAT2}

```gdscript
GLOBAL_VAR_TYPE_MAT2 = 19
```

2×2 matrix global shader parameter (global uniform mat2 ...). Exposed as a PackedInt32Array in the editor UI.

#### GLOBAL_VAR_TYPE_MAT3 {#GLOBAL_VAR_TYPE_MAT3}

```gdscript
GLOBAL_VAR_TYPE_MAT3 = 20
```

3×3 matrix global shader parameter (global uniform mat3 ...). Exposed as a Basis in the editor UI.

#### GLOBAL_VAR_TYPE_MAT4 {#GLOBAL_VAR_TYPE_MAT4}

```gdscript
GLOBAL_VAR_TYPE_MAT4 = 21
```

4×4 matrix global shader parameter (global uniform mat4 ...). Exposed as a Projection in the editor UI.

#### GLOBAL_VAR_TYPE_TRANSFORM_2D {#GLOBAL_VAR_TYPE_TRANSFORM_2D}

```gdscript
GLOBAL_VAR_TYPE_TRANSFORM_2D = 22
```

2-dimensional transform global shader parameter (global uniform mat2x3 ...). Exposed as a Transform2D in the editor UI.

#### GLOBAL_VAR_TYPE_TRANSFORM {#GLOBAL_VAR_TYPE_TRANSFORM}

```gdscript
GLOBAL_VAR_TYPE_TRANSFORM = 23
```

3-dimensional transform global shader parameter (global uniform mat3x4 ...). Exposed as a Transform3D in the editor UI.

#### GLOBAL_VAR_TYPE_SAMPLER2D {#GLOBAL_VAR_TYPE_SAMPLER2D}

```gdscript
GLOBAL_VAR_TYPE_SAMPLER2D = 24
```

2D sampler global shader parameter (global uniform sampler2D ...). Exposed as a Texture2D in the editor UI.

#### GLOBAL_VAR_TYPE_SAMPLER2DARRAY {#GLOBAL_VAR_TYPE_SAMPLER2DARRAY}

```gdscript
GLOBAL_VAR_TYPE_SAMPLER2DARRAY = 25
```

2D sampler array global shader parameter (global uniform sampler2DArray ...). Exposed as a Texture2DArray in the editor UI.

#### GLOBAL_VAR_TYPE_SAMPLER3D {#GLOBAL_VAR_TYPE_SAMPLER3D}

```gdscript
GLOBAL_VAR_TYPE_SAMPLER3D = 26
```

3D sampler global shader parameter (global uniform sampler3D ...). Exposed as a Texture3D in the editor UI.

#### GLOBAL_VAR_TYPE_SAMPLERCUBE {#GLOBAL_VAR_TYPE_SAMPLERCUBE}

```gdscript
GLOBAL_VAR_TYPE_SAMPLERCUBE = 27
```

Cubemap sampler global shader parameter (global uniform samplerCube ...). Exposed as a Cubemap in the editor UI.

#### GLOBAL_VAR_TYPE_SAMPLEREXT {#GLOBAL_VAR_TYPE_SAMPLEREXT}

```gdscript
GLOBAL_VAR_TYPE_SAMPLEREXT = 28
```

External sampler global shader parameter (global uniform samplerExternalOES ...). Exposed as an ExternalTexture in the editor UI.

#### GLOBAL_VAR_TYPE_MAX {#GLOBAL_VAR_TYPE_MAX}

```gdscript
GLOBAL_VAR_TYPE_MAX = 29
```

Represents the size of the GlobalShaderParameterType enum.

---

### RenderingInfo {#enum-RenderingInfo}

```gdscript
enum RenderingInfo { RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME = 0, RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME = 1, RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME = 2, RENDERING_INFO_TEXTURE_MEM_USED = 3, RENDERING_INFO_BUFFER_MEM_USED = 4, RENDERING_INFO_VIDEO_MEM_USED = 5, RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS = 6, RENDERING_INFO_PIPELINE_COMPILATIONS_MESH = 7, RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE = 8, RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW = 9, RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION = 10 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME {#RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME}

```gdscript
RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME = 0
```

Number of objects rendered in the current 3D scene. This varies depending on camera position and rotation.

#### RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME {#RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME}

```gdscript
RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME = 1
```

Number of points, lines, or triangles rendered in the current 3D scene. This varies depending on camera position and rotation.

#### RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME {#RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME}

```gdscript
RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME = 2
```

Number of draw calls performed to render in the current 3D scene. This varies depending on camera position and rotation.

#### RENDERING_INFO_TEXTURE_MEM_USED {#RENDERING_INFO_TEXTURE_MEM_USED}

```gdscript
RENDERING_INFO_TEXTURE_MEM_USED = 3
```

Texture memory used (in bytes).

#### RENDERING_INFO_BUFFER_MEM_USED {#RENDERING_INFO_BUFFER_MEM_USED}

```gdscript
RENDERING_INFO_BUFFER_MEM_USED = 4
```

Buffer memory used (in bytes). This includes vertex data, uniform buffers, and many miscellaneous buffer types used internally.

#### RENDERING_INFO_VIDEO_MEM_USED {#RENDERING_INFO_VIDEO_MEM_USED}

```gdscript
RENDERING_INFO_VIDEO_MEM_USED = 5
```

Video memory used (in bytes). When using the Forward+ or Mobile renderers, this is always greater than the sum of RENDERING_INFO_TEXTURE_MEM_USED and RENDERING_INFO_BUFFER_MEM_USED, since there is miscellaneous data not accounted for by those two metrics. When using the Compatibility renderer, this is equal to the sum of RENDERING_INFO_TEXTURE_MEM_USED and RENDERING_INFO_BUFFER_MEM_USED.

#### RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS {#RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS}

```gdscript
RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS = 6
```

Number of pipeline compilations that were triggered by the 2D canvas renderer.

#### RENDERING_INFO_PIPELINE_COMPILATIONS_MESH {#RENDERING_INFO_PIPELINE_COMPILATIONS_MESH}

```gdscript
RENDERING_INFO_PIPELINE_COMPILATIONS_MESH = 7
```

Number of pipeline compilations that were triggered by loading meshes. These compilations will show up as longer loading times the first time a user runs the game and the pipeline is required.

#### RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE {#RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE}

```gdscript
RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE = 8
```

Number of pipeline compilations that were triggered by building the surface cache before rendering the scene. These compilations will show up as a stutter when loading a scene the first time a user runs the game and the pipeline is required.

#### RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW {#RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW}

```gdscript
RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW = 9
```

Number of pipeline compilations that were triggered while drawing the scene. These compilations will show up as stutters during gameplay the first time a user runs the game and the pipeline is required.

#### RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION {#RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION}

```gdscript
RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION = 10
```

Number of pipeline compilations that were triggered to optimize the current scene. These compilations are done in the background and should not cause any stutters whatsoever.

---

### PipelineSource {#enum-PipelineSource}

```gdscript
enum PipelineSource { PIPELINE_SOURCE_CANVAS = 0, PIPELINE_SOURCE_MESH = 1, PIPELINE_SOURCE_SURFACE = 2, PIPELINE_SOURCE_DRAW = 3, PIPELINE_SOURCE_SPECIALIZATION = 4, PIPELINE_SOURCE_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PIPELINE_SOURCE_CANVAS {#PIPELINE_SOURCE_CANVAS}

```gdscript
PIPELINE_SOURCE_CANVAS = 0
```

Pipeline compilation that was triggered by the 2D canvas renderer.

#### PIPELINE_SOURCE_MESH {#PIPELINE_SOURCE_MESH}

```gdscript
PIPELINE_SOURCE_MESH = 1
```

Pipeline compilation that was triggered by loading a mesh.

#### PIPELINE_SOURCE_SURFACE {#PIPELINE_SOURCE_SURFACE}

```gdscript
PIPELINE_SOURCE_SURFACE = 2
```

Pipeline compilation that was triggered by building the surface cache before rendering the scene.

#### PIPELINE_SOURCE_DRAW {#PIPELINE_SOURCE_DRAW}

```gdscript
PIPELINE_SOURCE_DRAW = 3
```

Pipeline compilation that was triggered while drawing the scene.

#### PIPELINE_SOURCE_SPECIALIZATION {#PIPELINE_SOURCE_SPECIALIZATION}

```gdscript
PIPELINE_SOURCE_SPECIALIZATION = 4
```

Pipeline compilation that was triggered to optimize the current scene.

#### PIPELINE_SOURCE_MAX {#PIPELINE_SOURCE_MAX}

```gdscript
PIPELINE_SOURCE_MAX = 5
```

Represents the size of the PipelineSource enum.

---

### SplashStretchMode {#enum-SplashStretchMode}

```gdscript
enum SplashStretchMode { SPLASH_STRETCH_MODE_DISABLED = 0, SPLASH_STRETCH_MODE_KEEP = 1, SPLASH_STRETCH_MODE_KEEP_WIDTH = 2, SPLASH_STRETCH_MODE_KEEP_HEIGHT = 3, SPLASH_STRETCH_MODE_COVER = 4, SPLASH_STRETCH_MODE_IGNORE = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SPLASH_STRETCH_MODE_DISABLED {#SPLASH_STRETCH_MODE_DISABLED}

```gdscript
SPLASH_STRETCH_MODE_DISABLED = 0
```

No stretching is applied.

#### SPLASH_STRETCH_MODE_KEEP {#SPLASH_STRETCH_MODE_KEEP}

```gdscript
SPLASH_STRETCH_MODE_KEEP = 1
```

Stretches image to fullscreen while preserving aspect ratio.

#### SPLASH_STRETCH_MODE_KEEP_WIDTH {#SPLASH_STRETCH_MODE_KEEP_WIDTH}

```gdscript
SPLASH_STRETCH_MODE_KEEP_WIDTH = 2
```

Stretches the height of the image based on the width of the screen.

#### SPLASH_STRETCH_MODE_KEEP_HEIGHT {#SPLASH_STRETCH_MODE_KEEP_HEIGHT}

```gdscript
SPLASH_STRETCH_MODE_KEEP_HEIGHT = 3
```

Stretches the width of the image based on the height of the screen.

#### SPLASH_STRETCH_MODE_COVER {#SPLASH_STRETCH_MODE_COVER}

```gdscript
SPLASH_STRETCH_MODE_COVER = 4
```

Stretches the image to cover the entire screen while preserving aspect ratio.

#### SPLASH_STRETCH_MODE_IGNORE {#SPLASH_STRETCH_MODE_IGNORE}

```gdscript
SPLASH_STRETCH_MODE_IGNORE = 5
```

Stretches the image to cover the entire screen but doesn't preserve aspect ratio.

---

### Features {#enum-Features}

```gdscript
enum Features { FEATURE_SHADERS = 0, FEATURE_MULTITHREADED = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FEATURE_SHADERS {#FEATURE_SHADERS}

```gdscript
FEATURE_SHADERS = 0
```

Deprecated: This constant has not been used since Godot 3.0.

#### FEATURE_MULTITHREADED {#FEATURE_MULTITHREADED}

```gdscript
FEATURE_MULTITHREADED = 1
```

Deprecated: This constant has not been used since Godot 3.0.

---

## 常量

### NO_INDEX_ARRAY {#NO_INDEX_ARRAY}

```gdscript
const NO_INDEX_ARRAY = -1
```

Marks an error that shows that the index array is empty.

ARRAY_WEIGHTS_SIZE = 4 

Number of weights/bones per vertex.

CANVAS_ITEM_Z_MIN = -4096 

The minimum Z-layer for canvas items.

CANVAS_ITEM_Z_MAX = 4096 

The maximum Z-layer for canvas items.

CANVAS_LAYER_MIN = -2147483648 

The minimum canvas layer.

CANVAS_LAYER_MAX = 2147483647 

The maximum canvas layer.

MAX_GLOW_LEVELS = 7 

The maximum number of glow levels that can be used with the glow post-processing effect.

MAX_CURSORS = 8 

Deprecated: This constant is not used by the engine.

MAX_2D_DIRECTIONAL_LIGHTS = 8 

The maximum number of directional lights that can be rendered at a given time in 2D.

MAX_MESH_SURFACES = 256 

The maximum number of surfaces a mesh can have.

MATERIAL_RENDER_PRIORITY_MIN = -128 

The minimum renderpriority of all materials.

MATERIAL_RENDER_PRIORITY_MAX = 127 

The maximum renderpriority of all materials.

ARRAY_CUSTOM_COUNT = 4 

The number of custom data arrays available (ARRAY_CUSTOM0, ARRAY_CUSTOM1, ARRAY_CUSTOM2, ARRAY_CUSTOM3).

PARTICLES_EMIT_FLAG_POSITION = 1 

Particle starts at the specified position.

PARTICLES_EMIT_FLAG_ROTATION_SCALE = 2 

Particle starts with specified rotation and scale.

PARTICLES_EMIT_FLAG_VELOCITY = 4 

Particle starts with the specified velocity vector, which defines the emission direction and speed.

PARTICLES_EMIT_FLAG_COLOR = 8 

Particle starts with specified color.

PARTICLES_EMIT_FLAG_CUSTOM = 16 

Particle starts with specified CUSTOM data.

### ARRAY_WEIGHTS_SIZE {#ARRAY_WEIGHTS_SIZE}

```gdscript
const ARRAY_WEIGHTS_SIZE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CANVAS_ITEM_Z_MIN {#CANVAS_ITEM_Z_MIN}

```gdscript
const CANVAS_ITEM_Z_MIN = -4096
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CANVAS_ITEM_Z_MAX {#CANVAS_ITEM_Z_MAX}

```gdscript
const CANVAS_ITEM_Z_MAX = 4096
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CANVAS_LAYER_MIN {#CANVAS_LAYER_MIN}

```gdscript
const CANVAS_LAYER_MIN = -2147483648
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CANVAS_LAYER_MAX {#CANVAS_LAYER_MAX}

```gdscript
const CANVAS_LAYER_MAX = 2147483647
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MAX_GLOW_LEVELS {#MAX_GLOW_LEVELS}

```gdscript
const MAX_GLOW_LEVELS = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MAX_CURSORS {#MAX_CURSORS}

```gdscript
const MAX_CURSORS = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MAX_2D_DIRECTIONAL_LIGHTS {#MAX_2D_DIRECTIONAL_LIGHTS}

```gdscript
const MAX_2D_DIRECTIONAL_LIGHTS = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MAX_MESH_SURFACES {#MAX_MESH_SURFACES}

```gdscript
const MAX_MESH_SURFACES = 256
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MATERIAL_RENDER_PRIORITY_MIN {#MATERIAL_RENDER_PRIORITY_MIN}

```gdscript
const MATERIAL_RENDER_PRIORITY_MIN = -128
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MATERIAL_RENDER_PRIORITY_MAX {#MATERIAL_RENDER_PRIORITY_MAX}

```gdscript
const MATERIAL_RENDER_PRIORITY_MAX = 127
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ARRAY_CUSTOM_COUNT {#ARRAY_CUSTOM_COUNT}

```gdscript
const ARRAY_CUSTOM_COUNT = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### PARTICLES_EMIT_FLAG_POSITION {#PARTICLES_EMIT_FLAG_POSITION}

```gdscript
const PARTICLES_EMIT_FLAG_POSITION = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### PARTICLES_EMIT_FLAG_ROTATION_SCALE {#PARTICLES_EMIT_FLAG_ROTATION_SCALE}

```gdscript
const PARTICLES_EMIT_FLAG_ROTATION_SCALE = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### PARTICLES_EMIT_FLAG_VELOCITY {#PARTICLES_EMIT_FLAG_VELOCITY}

```gdscript
const PARTICLES_EMIT_FLAG_VELOCITY = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### PARTICLES_EMIT_FLAG_COLOR {#PARTICLES_EMIT_FLAG_COLOR}

```gdscript
const PARTICLES_EMIT_FLAG_COLOR = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### PARTICLES_EMIT_FLAG_CUSTOM {#PARTICLES_EMIT_FLAG_CUSTOM}

```gdscript
const PARTICLES_EMIT_FLAG_CUSTOM = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
