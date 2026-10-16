# Viewport

**继承**：[Node](class_node.md) → [Object](class_object.md) → **Viewport**

> A Viewport creates a different view into the screen, or a sub-view inside another viewport. Child 2D nodes will display on it, and child Camera3D 3D nodes will render on it too.

<details><summary>官方英文描述（中文精译进行中）</summary>

A Viewport creates a different view into the screen, or a sub-view inside another viewport. Child 2D nodes will display on it, and child Camera3D 3D nodes will render on it too.

Optionally, a viewport can have its own 2D or 3D world, so it doesn't share what it draws with other viewports.

Viewports can also choose to be audio listeners, so they generate positional audio depending on a 2D or 3D camera child of it.

Also, viewports can be assigned to different screens in case the devices have multiple screens.

Finally, viewports can also behave as render targets, in which case they will not be visible unless the associated texture is used to draw.

</details>

## 方法

### find_world_2d {#find_world_2d}

```gdscript
World2D find_world_2d() const
```

Returns the first valid World2D for this viewport, searching the world_2d property of itself and any Viewport ancestor.

### find_world_3d {#find_world_3d}

```gdscript
World3D find_world_3d() const
```

Returns the first valid World3D for this viewport, searching the world_3d property of itself and any Viewport ancestor.

### get_anisotropic_filtering_level {#get_anisotropic_filtering_level}

```gdscript
Viewport.AnisotropicFiltering get_anisotropic_filtering_level() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_audio_listener_2d {#get_audio_listener_2d}

```gdscript
AudioListener2D get_audio_listener_2d() const
```

Returns the currently active 2D audio listener. Returns null if there are no active 2D audio listeners, in which case the active 2D camera will be treated as listener.

### get_audio_listener_3d {#get_audio_listener_3d}

```gdscript
AudioListener3D get_audio_listener_3d() const
```

Returns the currently active 3D audio listener. Returns null if there are no active 3D audio listeners, in which case the active 3D camera will be treated as listener.

### get_camera_2d {#get_camera_2d}

```gdscript
Camera2D get_camera_2d() const
```

Returns the currently active 2D camera. Returns null if there are no active cameras.

Note: If called while the Camera Override system is active in editor, this will return the internally managed override camera. It is therefore advised to avoid caching the return value, or to check that the cached value is still a valid instance and is the current camera before use. See @GlobalScope.is_instance_valid() and Camera2D.is_current().

### get_camera_3d {#get_camera_3d}

```gdscript
Camera3D get_camera_3d() const
```

Returns the currently active 3D camera. Returns null if there are no active cameras.

Note: If called while the Camera Override system is active in editor, this will return the internally managed override camera. It is therefore advised to avoid caching the return value, or to check that the cached value is a valid instance and is the current camera before use. See @GlobalScope.is_instance_valid() and Camera3D.current.

### get_canvas_cull_mask {#get_canvas_cull_mask}

```gdscript
int get_canvas_cull_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_canvas_cull_mask_bit {#get_canvas_cull_mask_bit}

```gdscript
bool get_canvas_cull_mask_bit(layer: int) const
```

Returns an individual bit on the rendering layer mask.

### get_canvas_transform {#get_canvas_transform}

```gdscript
Transform2D get_canvas_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_debug_draw {#get_debug_draw}

```gdscript
Viewport.DebugDraw get_debug_draw() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_canvas_item_texture_filter {#get_default_canvas_item_texture_filter}

```gdscript
Viewport.DefaultCanvasItemTextureFilter get_default_canvas_item_texture_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_default_canvas_item_texture_repeat {#get_default_canvas_item_texture_repeat}

```gdscript
Viewport.DefaultCanvasItemTextureRepeat get_default_canvas_item_texture_repeat() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_threshold {#get_drag_threshold}

```gdscript
int get_drag_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_embedded_subwindows {#get_embedded_subwindows}

```gdscript
typedarray::Window get_embedded_subwindows() const
```

Returns a list of the visible embedded Windows inside the viewport.

Note: Windows inside other viewports will not be listed.

### get_final_transform {#get_final_transform}

```gdscript
Transform2D get_final_transform() const
```

Returns the transform from the viewport's coordinate system to the embedder's coordinate system.

### get_fsr_sharpness {#get_fsr_sharpness}

```gdscript
float get_fsr_sharpness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_global_canvas_transform {#get_global_canvas_transform}

```gdscript
Transform2D get_global_canvas_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mesh_lod_threshold {#get_mesh_lod_threshold}

```gdscript
float get_mesh_lod_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mouse_position {#get_mouse_position}

```gdscript
Vector2 get_mouse_position() const
```

Returns the mouse's position in this Viewport using the coordinate system of this Viewport.

### get_msaa_2d {#get_msaa_2d}

```gdscript
Viewport.MSAA get_msaa_2d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_msaa_3d {#get_msaa_3d}

```gdscript
Viewport.MSAA get_msaa_3d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_oversampling {#get_oversampling}

```gdscript
float get_oversampling() const
```

Returns viewport oversampling factor.

### get_oversampling_override {#get_oversampling_override}

```gdscript
float get_oversampling_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_object_picking {#get_physics_object_picking}

```gdscript
bool get_physics_object_picking()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_object_picking_first_only {#get_physics_object_picking_first_only}

```gdscript
bool get_physics_object_picking_first_only()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_object_picking_sort {#get_physics_object_picking_sort}

```gdscript
bool get_physics_object_picking_sort()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_positional_shadow_atlas_16_bits {#get_positional_shadow_atlas_16_bits}

```gdscript
bool get_positional_shadow_atlas_16_bits() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_positional_shadow_atlas_quadrant_subdiv {#get_positional_shadow_atlas_quadrant_subdiv}

```gdscript
Viewport.PositionalShadowAtlasQuadrantSubdiv get_positional_shadow_atlas_quadrant_subdiv(quadrant: int) const
```

Returns the positional shadow atlas quadrant subdivision of the specified quadrant.

### get_positional_shadow_atlas_size {#get_positional_shadow_atlas_size}

```gdscript
int get_positional_shadow_atlas_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_render_info {#get_render_info}

```gdscript
int get_render_info(type: Viewport.RenderInfoType, info: Viewport.RenderInfo)
```

Returns rendering statistics of the given type.

### get_scaling_3d_mode {#get_scaling_3d_mode}

```gdscript
Viewport.Scaling3DMode get_scaling_3d_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scaling_3d_scale {#get_scaling_3d_scale}

```gdscript
float get_scaling_3d_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_screen_space_aa {#get_screen_space_aa}

```gdscript
Viewport.ScreenSpaceAA get_screen_space_aa() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_screen_transform {#get_screen_transform}

```gdscript
Transform2D get_screen_transform() const
```

Returns the transform from the Viewport's coordinates to the screen coordinates of the containing window manager window.

### get_sdf_oversize {#get_sdf_oversize}

```gdscript
Viewport.SDFOversize get_sdf_oversize() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sdf_scale {#get_sdf_scale}

```gdscript
Viewport.SDFScale get_sdf_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stretch_transform {#get_stretch_transform}

```gdscript
Transform2D get_stretch_transform() const
```

Returns the automatically computed 2D stretch transform, taking the Viewport's stretch settings into account. The final value is multiplied by Window.content_scale_factor, but only for the root viewport. If this method is called on a SubViewport (e.g., in a scene tree with SubViewportContainer and SubViewport), the scale factor of the root window will not be applied. Using Transform2D.get_scale() on the returned value, this can be used to compensate for scaling when zooming a Camera2D node, or to scale down a TextureRect to be pixel-perfect regardless of the automatically computed scale factor.

Note: Due to how pixel scaling works, the returned transform's X and Y scale may differ slightly, even when Window.content_scale_aspect is set to a mode that preserves the pixels' aspect ratio. If Window.content_scale_aspect is Window.CONTENT_SCALE_ASPECT_IGNORE, the X and Y scale may differ significantly.

### get_texture {#get_texture}

```gdscript
ViewportTexture get_texture() const
```

Returns the viewport's texture.

Note: When trying to store the current texture (e.g. in a file), it might be completely black or outdated if used too early, especially when used in e.g. Node._ready(). To make sure the texture you get is correct, you can await RenderingServer.frame_post_draw signal.

```
func _ready():
    await RenderingServer.frame_post_draw
    $Viewport.get_texture().get_image().save_png("user://Screenshot.png")
```

```
public async override void _Ready()
{
    await ToSignal(RenderingServer.Singleton, RenderingServer.SignalName.FramePostDraw);
    var viewport = GetNode("Viewport");
    viewport.GetTexture().GetImage().SavePng("user://Screenshot.png");
}
```

Note: When use_hdr_2d is true the returned texture will be an HDR image using linear encoding.

### get_texture_mipmap_bias {#get_texture_mipmap_bias}

```gdscript
float get_texture_mipmap_bias() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_viewport_rid {#get_viewport_rid}

```gdscript
RID get_viewport_rid() const
```

Returns the viewport's RID from the RenderingServer.

### get_visible_rect {#get_visible_rect}

```gdscript
Rect2 get_visible_rect() const
```

Returns the visible rectangle in global screen coordinates.

### get_vrs_mode {#get_vrs_mode}

```gdscript
Viewport.VRSMode get_vrs_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vrs_texture {#get_vrs_texture}

```gdscript
Texture2D get_vrs_texture() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vrs_update_mode {#get_vrs_update_mode}

```gdscript
Viewport.VRSUpdateMode get_vrs_update_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_world_2d {#get_world_2d}

```gdscript
World2D get_world_2d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_world_3d {#get_world_3d}

```gdscript
World3D get_world_3d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### gui_cancel_drag {#gui_cancel_drag}

```gdscript
void gui_cancel_drag()
```

Cancels the drag operation that was previously started through Control._get_drag_data() or forced with Control.force_drag().

### gui_get_drag_data {#gui_get_drag_data}

```gdscript
Variant gui_get_drag_data() const
```

Returns the drag data from the GUI, that was previously returned by Control._get_drag_data().

### gui_get_drag_description {#gui_get_drag_description}

```gdscript
String gui_get_drag_description() const
```

Returns the human-readable description of the drag data, used for assistive apps.

### gui_get_focus_owner {#gui_get_focus_owner}

```gdscript
Control gui_get_focus_owner() const
```

Returns the currently focused Control within this viewport. If no Control is focused, returns null.

### gui_get_hovered_control {#gui_get_hovered_control}

```gdscript
Control gui_get_hovered_control() const
```

Returns the Control that the mouse is currently hovering over in this viewport. If no Control has the cursor, returns null.

Typically the leaf Control node or deepest level of the subtree which claims hover. This is very useful when used together with Node.is_ancestor_of() to find if the mouse is within a control tree.

### gui_is_drag_successful {#gui_is_drag_successful}

```gdscript
bool gui_is_drag_successful() const
```

Returns true if the drag operation is successful.

### gui_is_dragging {#gui_is_dragging}

```gdscript
bool gui_is_dragging() const
```

Returns true if a drag operation is currently ongoing and where the drop action could happen in this viewport.

Alternative to Node.NOTIFICATION_DRAG_BEGIN and Node.NOTIFICATION_DRAG_END when you prefer polling the value.

### gui_release_focus {#gui_release_focus}

```gdscript
void gui_release_focus()
```

Removes the focus from the currently focused Control within this viewport. If no Control has the focus, does nothing.

### gui_set_drag_description {#gui_set_drag_description}

```gdscript
void gui_set_drag_description(description: String)
```

Sets the human-readable description of the drag data to description, used for assistive apps.

### has_transparent_background {#has_transparent_background}

```gdscript
bool has_transparent_background() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_3d_disabled {#is_3d_disabled}

```gdscript
bool is_3d_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_audio_listener_2d {#is_audio_listener_2d}

```gdscript
bool is_audio_listener_2d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_audio_listener_3d {#is_audio_listener_3d}

```gdscript
bool is_audio_listener_3d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_embedding_subwindows {#is_embedding_subwindows}

```gdscript
bool is_embedding_subwindows() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_handling_input_locally {#is_handling_input_locally}

```gdscript
bool is_handling_input_locally() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_input_disabled {#is_input_disabled}

```gdscript
bool is_input_disabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_input_handled {#is_input_handled}

```gdscript
bool is_input_handled() const
```

Returns whether the current InputEvent has been handled. Input events are not handled until set_input_as_handled() has been called during the lifetime of an InputEvent.

This is usually done as part of input handling methods like Node._input(), Control._gui_input() or others, as well as in corresponding signal handlers.

If handle_input_locally is set to false, this method will try finding the first parent viewport that is set to handle input locally, and return its value for is_input_handled() instead.

### is_snap_2d_transforms_to_pixel_enabled {#is_snap_2d_transforms_to_pixel_enabled}

```gdscript
bool is_snap_2d_transforms_to_pixel_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_snap_2d_vertices_to_pixel_enabled {#is_snap_2d_vertices_to_pixel_enabled}

```gdscript
bool is_snap_2d_vertices_to_pixel_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_snap_controls_to_pixels_enabled {#is_snap_controls_to_pixels_enabled}

```gdscript
bool is_snap_controls_to_pixels_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_debanding {#is_using_debanding}

```gdscript
bool is_using_debanding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_hdr_2d {#is_using_hdr_2d}

```gdscript
bool is_using_hdr_2d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_occlusion_culling {#is_using_occlusion_culling}

```gdscript
bool is_using_occlusion_culling() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_oversampling {#is_using_oversampling}

```gdscript
bool is_using_oversampling() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_own_world_3d {#is_using_own_world_3d}

```gdscript
bool is_using_own_world_3d() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_taa {#is_using_taa}

```gdscript
bool is_using_taa() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_xr {#is_using_xr}

```gdscript
bool is_using_xr() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### notify_mouse_entered {#notify_mouse_entered}

```gdscript
void notify_mouse_entered()
```

Inform the Viewport that the mouse has entered its area. Use this function before sending an InputEventMouseButton or InputEventMouseMotion to the Viewport with push_input(). See also notify_mouse_exited().

Note: In most cases, it is not necessary to call this function because SubViewport nodes that are children of SubViewportContainer are notified automatically. This is only necessary when interacting with viewports in non-default ways, for example as textures in TextureRect or with an Area3D that forwards input events.

### notify_mouse_exited {#notify_mouse_exited}

```gdscript
void notify_mouse_exited()
```

Inform the Viewport that the mouse has left its area. Use this function when the node that displays the viewport notices the mouse has left the area of the displayed viewport. See also notify_mouse_entered().

Note: In most cases, it is not necessary to call this function because SubViewport nodes that are children of SubViewportContainer are notified automatically. This is only necessary when interacting with viewports in non-default ways, for example as textures in TextureRect or with an Area3D that forwards input events.

### push_input {#push_input}

```gdscript
void push_input(event: InputEvent, in_local_coords: bool = false)
```

Triggers the given event in this Viewport. This can be used to pass an InputEvent between viewports, or to locally apply inputs that were sent over the network or saved to a file.

If in_local_coords is false, the event's position is in the embedder's coordinates and will be converted to viewport coordinates. If in_local_coords is true, the event's position is in viewport coordinates.

While this method serves a similar purpose as Input.parse_input_event(), it does not remap the specified event based on project settings like ProjectSettings.input_devices/pointing/emulate_touch_from_mouse.

Calling this method will propagate calls to child nodes for following methods in the given order:

- Node._input()
- Control._gui_input() for Control nodes
- Node._shortcut_input()
- Node._unhandled_key_input()
- Node._unhandled_input()

If an earlier method marks the input as handled via set_input_as_handled(), any later method in this list will not be called.

If none of the methods handle the event and physics_object_picking is true, the event is used for physics object picking.

### push_text_input {#push_text_input}

```gdscript
void push_text_input(text: String)
```

Helper method which calls the set_text() method on the currently focused Control, provided that it is defined (e.g. if the focused Control is Button or LineEdit).

### push_unhandled_input {#push_unhandled_input}

```gdscript
void push_unhandled_input(event: InputEvent, in_local_coords: bool = false)
```

Deprecated: Use push_input() instead.

Triggers the given event in this Viewport. This can be used to pass an InputEvent between viewports, or to locally apply inputs that were sent over the network or saved to a file.

If in_local_coords is false, the event's position is in the embedder's coordinates and will be converted to viewport coordinates. If in_local_coords is true, the event's position is in viewport coordinates.

Calling this method will propagate calls to child nodes for following methods in the given order:

- Node._shortcut_input()
- Node._unhandled_key_input()
- Node._unhandled_input()

If an earlier method marks the input as handled via set_input_as_handled(), any later method in this list will not be called.

If none of the methods handle the event and physics_object_picking is true, the event is used for physics object picking.

Note: This method doesn't propagate input events to embedded Windows or SubViewports.

### set_anisotropic_filtering_level {#set_anisotropic_filtering_level}

```gdscript
void set_anisotropic_filtering_level(anisotropic_filtering_level: Viewport.AnisotropicFiltering)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_as_audio_listener_2d {#set_as_audio_listener_2d}

```gdscript
void set_as_audio_listener_2d(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_as_audio_listener_3d {#set_as_audio_listener_3d}

```gdscript
void set_as_audio_listener_3d(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_canvas_cull_mask {#set_canvas_cull_mask}

```gdscript
void set_canvas_cull_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_canvas_cull_mask_bit {#set_canvas_cull_mask_bit}

```gdscript
void set_canvas_cull_mask_bit(layer: int, enable: bool)
```

Set/clear individual bits on the rendering layer mask. This simplifies editing this Viewport's layers.

### set_canvas_transform {#set_canvas_transform}

```gdscript
void set_canvas_transform(xform: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_debug_draw {#set_debug_draw}

```gdscript
void set_debug_draw(debug_draw: Viewport.DebugDraw)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_canvas_item_texture_filter {#set_default_canvas_item_texture_filter}

```gdscript
void set_default_canvas_item_texture_filter(mode: Viewport.DefaultCanvasItemTextureFilter)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_default_canvas_item_texture_repeat {#set_default_canvas_item_texture_repeat}

```gdscript
void set_default_canvas_item_texture_repeat(mode: Viewport.DefaultCanvasItemTextureRepeat)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_3d {#set_disable_3d}

```gdscript
void set_disable_3d(disable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_disable_input {#set_disable_input}

```gdscript
void set_disable_input(disable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_threshold {#set_drag_threshold}

```gdscript
void set_drag_threshold(threshold: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_embedding_subwindows {#set_embedding_subwindows}

```gdscript
void set_embedding_subwindows(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fsr_sharpness {#set_fsr_sharpness}

```gdscript
void set_fsr_sharpness(fsr_sharpness: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_global_canvas_transform {#set_global_canvas_transform}

```gdscript
void set_global_canvas_transform(xform: Transform2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_handle_input_locally {#set_handle_input_locally}

```gdscript
void set_handle_input_locally(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_as_handled {#set_input_as_handled}

```gdscript
void set_input_as_handled()
```

Stops the input from propagating further up the SceneTree.

Note: This does not affect the methods in Input, only the way events are propagated.

### set_mesh_lod_threshold {#set_mesh_lod_threshold}

```gdscript
void set_mesh_lod_threshold(pixels: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_msaa_2d {#set_msaa_2d}

```gdscript
void set_msaa_2d(msaa: Viewport.MSAA)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_msaa_3d {#set_msaa_3d}

```gdscript
void set_msaa_3d(msaa: Viewport.MSAA)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_oversampling_override {#set_oversampling_override}

```gdscript
void set_oversampling_override(oversampling: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_object_picking {#set_physics_object_picking}

```gdscript
void set_physics_object_picking(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_object_picking_first_only {#set_physics_object_picking_first_only}

```gdscript
void set_physics_object_picking_first_only(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_object_picking_sort {#set_physics_object_picking_sort}

```gdscript
void set_physics_object_picking_sort(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_positional_shadow_atlas_16_bits {#set_positional_shadow_atlas_16_bits}

```gdscript
void set_positional_shadow_atlas_16_bits(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_positional_shadow_atlas_quadrant_subdiv {#set_positional_shadow_atlas_quadrant_subdiv}

```gdscript
void set_positional_shadow_atlas_quadrant_subdiv(quadrant: int, subdiv: Viewport.PositionalShadowAtlasQuadrantSubdiv)
```

Sets the number of subdivisions to use in the specified quadrant. A higher number of subdivisions allows you to have more shadows in the scene at once, but reduces the quality of the shadows. A good practice is to have quadrants with a varying number of subdivisions and to have as few subdivisions as possible.

### set_positional_shadow_atlas_size {#set_positional_shadow_atlas_size}

```gdscript
void set_positional_shadow_atlas_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scaling_3d_mode {#set_scaling_3d_mode}

```gdscript
void set_scaling_3d_mode(scaling_3d_mode: Viewport.Scaling3DMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_scaling_3d_scale {#set_scaling_3d_scale}

```gdscript
void set_scaling_3d_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_screen_space_aa {#set_screen_space_aa}

```gdscript
void set_screen_space_aa(screen_space_aa: Viewport.ScreenSpaceAA)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sdf_oversize {#set_sdf_oversize}

```gdscript
void set_sdf_oversize(oversize: Viewport.SDFOversize)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sdf_scale {#set_sdf_scale}

```gdscript
void set_sdf_scale(scale: Viewport.SDFScale)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_snap_2d_transforms_to_pixel {#set_snap_2d_transforms_to_pixel}

```gdscript
void set_snap_2d_transforms_to_pixel(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_snap_2d_vertices_to_pixel {#set_snap_2d_vertices_to_pixel}

```gdscript
void set_snap_2d_vertices_to_pixel(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_snap_controls_to_pixels {#set_snap_controls_to_pixels}

```gdscript
void set_snap_controls_to_pixels(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_texture_mipmap_bias {#set_texture_mipmap_bias}

```gdscript
void set_texture_mipmap_bias(texture_mipmap_bias: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transparent_background {#set_transparent_background}

```gdscript
void set_transparent_background(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_debanding {#set_use_debanding}

```gdscript
void set_use_debanding(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_hdr_2d {#set_use_hdr_2d}

```gdscript
void set_use_hdr_2d(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_occlusion_culling {#set_use_occlusion_culling}

```gdscript
void set_use_occlusion_culling(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_oversampling {#set_use_oversampling}

```gdscript
void set_use_oversampling(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_own_world_3d {#set_use_own_world_3d}

```gdscript
void set_use_own_world_3d(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_taa {#set_use_taa}

```gdscript
void set_use_taa(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_xr {#set_use_xr}

```gdscript
void set_use_xr(use: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_mode {#set_vrs_mode}

```gdscript
void set_vrs_mode(mode: Viewport.VRSMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_texture {#set_vrs_texture}

```gdscript
void set_vrs_texture(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_update_mode {#set_vrs_update_mode}

```gdscript
void set_vrs_update_mode(mode: Viewport.VRSUpdateMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_world_2d {#set_world_2d}

```gdscript
void set_world_2d(world_2d: World2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_world_3d {#set_world_3d}

```gdscript
void set_world_3d(world_3d: World3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### update_mouse_cursor_state {#update_mouse_cursor_state}

```gdscript
void update_mouse_cursor_state()
```

Force instantly updating the display based on the current mouse cursor position. This includes updating the mouse cursor shape and sending necessary Control.mouse_entered, CollisionObject2D.mouse_entered, CollisionObject3D.mouse_entered and Window.mouse_entered signals and their respective mouse_exited counterparts.

### warp_mouse {#warp_mouse}

```gdscript
void warp_mouse(position: Vector2)
```

Moves the mouse pointer to the specified position in this Viewport using the coordinate system of this Viewport.

Note: warp_mouse() is only supported on Windows, macOS and Linux. It has no effect on Android, iOS and Web.

## 属性

### disable_3d {#disable_3d}

```gdscript
disable_3d: bool
```

- void set_disable_3d(value: bool)
- bool is_3d_disabled()

Disable 3D rendering (but keep 2D rendering).

### use_xr {#use_xr}

```gdscript
use_xr: bool
```

- void set_use_xr(value: bool)
- bool is_using_xr()

If true, the viewport will use the primary XR interface to render XR output. When applicable this can result in a stereoscopic image and the resulting render being output to a headset.

### own_world_3d {#own_world_3d}

```gdscript
own_world_3d: bool
```

- void set_use_own_world_3d(value: bool)
- bool is_using_own_world_3d()

If true, the viewport will use a unique copy of the World3D defined in world_3d.

### world_3d {#world_3d}

```gdscript
world_3d: World3D
```

- void set_world_3d(value: World3D)
- World3D get_world_3d()

The custom World3D which can be used as 3D environment source.

### world_2d {#world_2d}

```gdscript
world_2d: World2D
```

- void set_world_2d(value: World2D)
- World2D get_world_2d()

The custom World2D which can be used as 2D environment source.

### transparent_bg {#transparent_bg}

```gdscript
transparent_bg: bool
```

- void set_transparent_background(value: bool)
- bool has_transparent_background()

If true, the viewport should render its background as transparent.

Note: Due to technical limitations, certain rendering features are disabled when a viewport has a transparent background. This currently applies to screen-space reflections, subsurface scattering, and depth of field.

### handle_input_locally {#handle_input_locally}

```gdscript
handle_input_locally: bool
```

- void set_handle_input_locally(value: bool)
- bool is_handling_input_locally()

If true, this viewport will mark incoming input events as handled by itself. If false, this is instead done by the first parent viewport that is set to handle input locally.

A SubViewportContainer will automatically set this property to false for the Viewport contained inside of it.

See also set_input_as_handled() and is_input_handled().

### snap_2d_transforms_to_pixel {#snap_2d_transforms_to_pixel}

```gdscript
snap_2d_transforms_to_pixel: bool
```

- void set_snap_2d_transforms_to_pixel(value: bool)
- bool is_snap_2d_transforms_to_pixel_enabled()

If true, CanvasItem nodes will internally snap to full pixels. Their position can still be sub-pixel, but the decimals will not have effect. This can lead to a crisper appearance at the cost of less smooth movement, especially when Camera2D smoothing is enabled.

### snap_2d_vertices_to_pixel {#snap_2d_vertices_to_pixel}

```gdscript
snap_2d_vertices_to_pixel: bool
```

- void set_snap_2d_vertices_to_pixel(value: bool)
- bool is_snap_2d_vertices_to_pixel_enabled()

If true, vertices of CanvasItem nodes will snap to full pixels. Only affects the final vertex positions, not the transforms. This can lead to a crisper appearance at the cost of less smooth movement, especially when Camera2D smoothing is enabled.

### msaa_2d {#msaa_2d}

```gdscript
msaa_2d: int
```

- void set_msaa_2d(value: MSAA)
- MSAA get_msaa_2d()

The multisample antialiasing mode for 2D/Canvas rendering. A higher number results in smoother edges at the cost of significantly worse performance. A value of MSAA_2X or MSAA_4X is best unless targeting very high-end systems. This has no effect on shader-induced aliasing or texture aliasing.

See also ProjectSettings.rendering/anti_aliasing/quality/msaa_2d and RenderingServer.viewport_set_msaa_2d().

### msaa_3d {#msaa_3d}

```gdscript
msaa_3d: int
```

- void set_msaa_3d(value: MSAA)
- MSAA get_msaa_3d()

The multisample antialiasing mode for 3D rendering. A higher number results in smoother edges at the cost of significantly worse performance. A value of MSAA_2X or MSAA_4X is best unless targeting very high-end systems. See also bilinear scaling 3D scaling_3d_mode for supersampling, which provides higher quality but is much more expensive. This has no effect on shader-induced aliasing or texture aliasing.

See also ProjectSettings.rendering/anti_aliasing/quality/msaa_3d and RenderingServer.viewport_set_msaa_3d().

### screen_space_aa {#screen_space_aa}

```gdscript
screen_space_aa: int
```

- void set_screen_space_aa(value: ScreenSpaceAA)
- ScreenSpaceAA get_screen_space_aa()

Sets the screen-space antialiasing method used. Screen-space antialiasing works by selectively blurring edges in a post-process shader. It differs from MSAA which takes multiple coverage samples while rendering objects. Screen-space AA methods are typically faster than MSAA and will smooth out specular aliasing, but tend to make scenes appear blurry.

See also ProjectSettings.rendering/anti_aliasing/quality/screen_space_aa and RenderingServer.viewport_set_screen_space_aa().

### use_taa {#use_taa}

```gdscript
use_taa: bool
```

- void set_use_taa(value: bool)
- bool is_using_taa()

Enables temporal antialiasing for this viewport. TAA works by jittering the camera and accumulating the images of the last rendered frames, motion vector rendering is used to account for camera and object motion.

Note: The implementation is not complete yet, some visual instances such as particles and skinned meshes may show artifacts.

See also ProjectSettings.rendering/anti_aliasing/quality/use_taa and RenderingServer.viewport_set_use_taa().

### use_debanding {#use_debanding}

```gdscript
use_debanding: bool
```

- void set_use_debanding(value: bool)
- bool is_using_debanding()

When using the Mobile or Forward+ renderers, set use_debanding to enable or disable the debanding feature of this Viewport. If use_hdr_2d is false, 2D rendering is not affected by debanding unless the Environment.background_mode is Environment.BG_CANVAS. If use_hdr_2d is true, debanding will only be applied if this is the root Viewport and will affect all 2D and 3D rendering, including canvas items.

use_debanding has no effect when using the Compatibility rendering method. The Mobile renderer can also use material debanding, which can be set with RenderingServer.material_set_use_debanding() or configured with ProjectSettings.rendering/anti_aliasing/quality/use_debanding.

See also ProjectSettings.rendering/anti_aliasing/quality/use_debanding, RenderingServer.material_set_use_debanding(), and RenderingServer.viewport_set_use_debanding().

### use_occlusion_culling {#use_occlusion_culling}

```gdscript
use_occlusion_culling: bool
```

- void set_use_occlusion_culling(value: bool)
- bool is_using_occlusion_culling()

If true, OccluderInstance3D nodes will be usable for occlusion culling in 3D for this viewport. For the root viewport, ProjectSettings.rendering/occlusion_culling/use_occlusion_culling must be set to true instead.

Note: Enabling occlusion culling has a cost on the CPU. Only enable occlusion culling if you actually plan to use it, and think whether your scene can actually benefit from occlusion culling. Large, open scenes with few or no objects blocking the view will generally not benefit much from occlusion culling. Large open scenes generally benefit more from mesh LOD and visibility ranges (GeometryInstance3D.visibility_range_begin and GeometryInstance3D.visibility_range_end) compared to occlusion culling.

Note: Due to memory constraints, occlusion culling is not supported by default in Web export templates. It can be enabled by compiling custom Web export templates with module_raycast_enabled=yes.

### mesh_lod_threshold {#mesh_lod_threshold}

```gdscript
mesh_lod_threshold: float
```

- void set_mesh_lod_threshold(value: float)
- float get_mesh_lod_threshold()

The automatic LOD bias to use for meshes rendered within the Viewport (this is analogous to ReflectionProbe.mesh_lod_threshold). Higher values will use less detailed versions of meshes that have LOD variations generated. If set to 0.0, automatic LOD is disabled. Increase mesh_lod_threshold to improve performance at the cost of geometry detail.

To control this property on the root viewport, set the ProjectSettings.rendering/mesh_lod/lod_change/threshold_pixels project setting.

Note: Depending on the mesh's attributes (vertex colors, blend shapes, ...), a mesh may have fewer levels of LOD generated to avoid visible distortion of the mesh once it is affected by vertex colors or blend shapes. Meshes with a very low vertex count will also not have any LODs generated, which means this setting will not affect them at all. In general, this setting makes the largest impact on static meshes with a high vertex count.

Note: mesh_lod_threshold does not affect GeometryInstance3D visibility ranges (also known as "manual" LOD or hierarchical LOD).

### debug_draw {#debug_draw}

```gdscript
debug_draw: int
```

- void set_debug_draw(value: DebugDraw)
- DebugDraw get_debug_draw()

The overlay mode for test rendered geometry in debug purposes.

### use_hdr_2d {#use_hdr_2d}

```gdscript
use_hdr_2d: bool
```

- void set_use_hdr_2d(value: bool)
- bool is_using_hdr_2d()

If true, 2D rendering will use a high dynamic range (HDR) RGBA16 format framebuffer. Additionally, 2D rendering will be performed on linear values and will be converted using the appropriate transfer function immediately before blitting to the screen (if the Viewport is attached to the screen).

Practically speaking, this means that the end result of the Viewport will not be clamped to the 0-1 range and can be used in 3D rendering without color encoding adjustments. This allows 2D rendering to take advantage of effects requiring high dynamic range (e.g. 2D glow) as well as substantially improves the appearance of effects requiring highly detailed gradients.

### scaling_3d_mode {#scaling_3d_mode}

```gdscript
scaling_3d_mode: int
```

- void set_scaling_3d_mode(value: Scaling3DMode)
- Scaling3DMode get_scaling_3d_mode()

Sets scaling 3D mode. Bilinear scaling renders at different resolution to either undersample or supersample the viewport. FidelityFX Super Resolution 1.0, abbreviated to FSR, is an upscaling technology that produces high quality images at fast framerates by using a spatially aware upscaling algorithm. FSR is slightly more expensive than bilinear, but it produces significantly higher image quality. FSR should be used where possible.

To control this property on the root viewport, set the ProjectSettings.rendering/scaling_3d/mode project setting.

### scaling_3d_scale {#scaling_3d_scale}

```gdscript
scaling_3d_scale: float
```

- void set_scaling_3d_scale(value: float)
- float get_scaling_3d_scale()

Scales the 3D render buffer based on the viewport size uses an image filter specified in ProjectSettings.rendering/scaling_3d/mode to scale the output image to the full viewport size. Values lower than 1.0 can be used to speed up 3D rendering at the cost of quality (undersampling). Values greater than 1.0 are only valid for bilinear mode and can be used to improve 3D rendering quality at a high performance cost (supersampling). See also ProjectSettings.rendering/anti_aliasing/quality/msaa_3d for multi-sample antialiasing, which is significantly cheaper but only smooths the edges of polygons.

When using FSR upscaling, AMD recommends exposing the following values as preset options to users "Ultra Quality: 0.77", "Quality: 0.67", "Balanced: 0.59", "Performance: 0.5" instead of exposing the entire scale.

To control this property on the root viewport, set the ProjectSettings.rendering/scaling_3d/scale project setting.

### texture_mipmap_bias {#texture_mipmap_bias}

```gdscript
texture_mipmap_bias: float
```

- void set_texture_mipmap_bias(value: float)
- float get_texture_mipmap_bias()

Affects the final texture sharpness by reading from a lower or higher mipmap (also called "texture LOD bias"). Negative values make mipmapped textures sharper but grainier when viewed at a distance, while positive values make mipmapped textures blurrier (even when up close).

Enabling temporal antialiasing (use_taa) will automatically apply a -0.5 offset to this value, while enabling FXAA (screen_space_aa) will automatically apply a -0.25 offset to this value. If both TAA and FXAA are enabled at the same time, an offset of -0.75 is applied to this value.

To control this property on the root viewport, set the ProjectSettings.rendering/textures/default_filters/texture_mipmap_bias project setting.

Note: If scaling_3d_scale is lower than 1.0 (exclusive), texture_mipmap_bias is used to adjust the automatic mipmap bias which is calculated internally based on the scale factor. The formula for this is log2(scaling_3d_scale) + mipmap_bias.

Note: This property is only supported in the Forward+ and Mobile renderers, not Compatibility. In Compatibility, this property is always treated as if it was set to 0.0.

### anisotropic_filtering_level {#anisotropic_filtering_level}

```gdscript
anisotropic_filtering_level: int
```

- void set_anisotropic_filtering_level(value: AnisotropicFiltering)
- AnisotropicFiltering get_anisotropic_filtering_level()

Sets the maximum number of samples to take when using anisotropic filtering on textures (as a power of two). A higher sample count will result in sharper textures at oblique angles, but is more expensive to compute. A value of 0 forcibly disables anisotropic filtering, even on materials where it is enabled.

The anisotropic filtering level also affects decals and light projectors if they are configured to use anisotropic filtering. See ProjectSettings.rendering/textures/decals/filter and ProjectSettings.rendering/textures/light_projectors/filter.

Note: In 3D, for this setting to have an effect, set BaseMaterial3D.texture_filter to BaseMaterial3D.TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC or BaseMaterial3D.TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC on materials.

Note: In 2D, for this setting to have an effect, set CanvasItem.texture_filter to CanvasItem.TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC or CanvasItem.TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC on the CanvasItem node displaying the texture (or in CanvasTexture). However, anisotropic filtering is rarely useful in 2D, so only enable it for textures in 2D if it makes a meaningful visual difference.

### fsr_sharpness {#fsr_sharpness}

```gdscript
fsr_sharpness: float
```

- void set_fsr_sharpness(value: float)
- float get_fsr_sharpness()

Determines how sharp the upscaled image will be when using the FSR upscaling mode. Sharpness halves with every whole number. Values go from 0.0 (sharpest) to 2.0. Values above 2.0 won't make a visible difference.

To control this property on the root viewport, set the ProjectSettings.rendering/scaling_3d/fsr_sharpness project setting.

### vrs_mode {#vrs_mode}

```gdscript
vrs_mode: int
```

- void set_vrs_mode(value: VRSMode)
- VRSMode get_vrs_mode()

The Variable Rate Shading (VRS) mode that is used for this viewport. Note, if hardware does not support VRS this property is ignored.

### vrs_update_mode {#vrs_update_mode}

```gdscript
vrs_update_mode: int
```

- void set_vrs_update_mode(value: VRSUpdateMode)
- VRSUpdateMode get_vrs_update_mode()

Sets the update mode for Variable Rate Shading (VRS) for the viewport. VRS requires the input texture to be converted to the format usable by the VRS method supported by the hardware. The update mode defines how often this happens. If the GPU does not support VRS, or VRS is not enabled, this property is ignored.

### vrs_texture {#vrs_texture}

```gdscript
vrs_texture: Texture2D
```

- void set_vrs_texture(value: Texture2D)
- Texture2D get_vrs_texture()

Texture to use when vrs_mode is set to VRS_TEXTURE.

The texture must use a lossless compression format so that colors can be matched precisely. The following VRS densities are mapped to various colors, with brighter colors representing a lower level of shading precision:

```
- 1×1 = rgb(0, 0, 0)     - #000000
- 1×2 = rgb(0, 85, 0)    - #005500
- 2×1 = rgb(85, 0, 0)    - #550000
- 2×2 = rgb(85, 85, 0)   - #555500
- 2×4 = rgb(85, 170, 0)  - #55aa00
- 4×2 = rgb(170, 85, 0)  - #aa5500
- 4×4 = rgb(170, 170, 0) - #aaaa00
- 4×8 = rgb(170, 255, 0) - #aaff00 - Not supported on most hardware
- 8×4 = rgb(255, 170, 0) - #ffaa00 - Not supported on most hardware
- 8×8 = rgb(255, 255, 0) - #ffff00 - Not supported on most hardware
```

### canvas_item_default_texture_filter {#canvas_item_default_texture_filter}

```gdscript
canvas_item_default_texture_filter: int
```

- void set_default_canvas_item_texture_filter(value: DefaultCanvasItemTextureFilter)
- DefaultCanvasItemTextureFilter get_default_canvas_item_texture_filter()

The default filter mode used by CanvasItem nodes in this viewport.

### canvas_item_default_texture_repeat {#canvas_item_default_texture_repeat}

```gdscript
canvas_item_default_texture_repeat: int
```

- void set_default_canvas_item_texture_repeat(value: DefaultCanvasItemTextureRepeat)
- DefaultCanvasItemTextureRepeat get_default_canvas_item_texture_repeat()

The default repeat mode used by CanvasItem nodes in this viewport.

### audio_listener_enable_2d {#audio_listener_enable_2d}

```gdscript
audio_listener_enable_2d: bool
```

- void set_as_audio_listener_2d(value: bool)
- bool is_audio_listener_2d()

If true, the viewport will process 2D audio streams.

### audio_listener_enable_3d {#audio_listener_enable_3d}

```gdscript
audio_listener_enable_3d: bool
```

- void set_as_audio_listener_3d(value: bool)
- bool is_audio_listener_3d()

If true, the viewport will process 3D audio streams.

### physics_object_picking {#physics_object_picking}

```gdscript
physics_object_picking: bool
```

- void set_physics_object_picking(value: bool)
- bool get_physics_object_picking()

If true, the objects rendered by viewport become subjects of mouse picking process.

Note: The number of simultaneously pickable objects is limited to 64 and they are selected in a non-deterministic order, which can be different in each picking process.

### physics_object_picking_sort {#physics_object_picking_sort}

```gdscript
physics_object_picking_sort: bool
```

- void set_physics_object_picking_sort(value: bool)
- bool get_physics_object_picking_sort()

If true, objects receive mouse picking events sorted primarily by their CanvasItem.z_index and secondarily by their position in the scene tree. If false, the order is undetermined.

Note: This setting is disabled by default because of its potential expensive computational cost.

Note: Sorting happens after selecting the pickable objects. Because of the limitation of 64 simultaneously pickable objects, it is not guaranteed that the object with the highest CanvasItem.z_index receives the picking event.

### physics_object_picking_first_only {#physics_object_picking_first_only}

```gdscript
physics_object_picking_first_only: bool
```

- void set_physics_object_picking_first_only(value: bool)
- bool get_physics_object_picking_first_only()

If true, the input_event signal will only be sent to one physics object in the mouse picking process. If you want to get the top object only, you must also enable physics_object_picking_sort.

If false, an input_event signal will be sent to all physics objects in the mouse picking process.

This applies to 2D CanvasItem object picking only.

### gui_disable_input {#gui_disable_input}

```gdscript
gui_disable_input: bool
```

- void set_disable_input(value: bool)
- bool is_input_disabled()

If true, the viewport will not receive input events.

### gui_snap_controls_to_pixels {#gui_snap_controls_to_pixels}

```gdscript
gui_snap_controls_to_pixels: bool
```

- void set_snap_controls_to_pixels(value: bool)
- bool is_snap_controls_to_pixels_enabled()

If true, the GUI controls on the viewport will lay pixel perfectly.

### gui_embed_subwindows {#gui_embed_subwindows}

```gdscript
gui_embed_subwindows: bool
```

- void set_embedding_subwindows(value: bool)
- bool is_embedding_subwindows()

If true, sub-windows (popups and dialogs) will be embedded inside application window as control-like nodes. If false, they will appear as separate windows handled by the operating system.

### gui_drag_threshold {#gui_drag_threshold}

```gdscript
gui_drag_threshold: int
```

- void set_drag_threshold(value: int)
- int get_drag_threshold()

The minimum distance the mouse cursor must move while pressed before a drag operation begins.

### sdf_oversize {#sdf_oversize}

```gdscript
sdf_oversize: int
```

- void set_sdf_oversize(value: SDFOversize)
- SDFOversize get_sdf_oversize()

Controls how much of the original viewport's size should be covered by the 2D signed distance field. This SDF can be sampled in CanvasItem shaders and is also used for GPUParticles2D collision. Higher values allow portions of occluders located outside the viewport to still be taken into account in the generated signed distance field, at the cost of performance. If you notice particles falling through LightOccluder2Ds as the occluders leave the viewport, increase this setting.

The percentage is added on each axis and on both sides. For example, with the default SDF_OVERSIZE_120_PERCENT, the signed distance field will cover 20% of the viewport's size outside the viewport on each side (top, right, bottom, left).

### sdf_scale {#sdf_scale}

```gdscript
sdf_scale: int
```

- void set_sdf_scale(value: SDFScale)
- SDFScale get_sdf_scale()

The resolution scale to use for the 2D signed distance field. Higher values lead to a more precise and more stable signed distance field as the camera moves, at the cost of performance.

### positional_shadow_atlas_size {#positional_shadow_atlas_size}

```gdscript
positional_shadow_atlas_size: int
```

- void set_positional_shadow_atlas_size(value: int)
- int get_positional_shadow_atlas_size()

The shadow atlas' resolution (used for omni and spot lights). The value is rounded up to the nearest power of 2.

Note: If this is set to 0, no positional shadows will be visible at all. This can improve performance significantly on low-end systems by reducing both the CPU and GPU load (as fewer draw calls are needed to draw the scene without shadows).

### positional_shadow_atlas_16_bits {#positional_shadow_atlas_16_bits}

```gdscript
positional_shadow_atlas_16_bits: bool
```

- void set_positional_shadow_atlas_16_bits(value: bool)
- bool get_positional_shadow_atlas_16_bits()

Use 16 bits for the omni/spot shadow depth map. Enabling this results in shadows having less precision and may result in shadow acne, but can lead to performance improvements on some devices.

### positional_shadow_atlas_quad_0 {#positional_shadow_atlas_quad_0}

```gdscript
positional_shadow_atlas_quad_0: int
```

- void set_positional_shadow_atlas_quadrant_subdiv(quadrant: int, subdiv: PositionalShadowAtlasQuadrantSubdiv)
- PositionalShadowAtlasQuadrantSubdiv get_positional_shadow_atlas_quadrant_subdiv(quadrant: int) const

The subdivision amount of the first quadrant on the shadow atlas.

### positional_shadow_atlas_quad_1 {#positional_shadow_atlas_quad_1}

```gdscript
positional_shadow_atlas_quad_1: int
```

- void set_positional_shadow_atlas_quadrant_subdiv(quadrant: int, subdiv: PositionalShadowAtlasQuadrantSubdiv)
- PositionalShadowAtlasQuadrantSubdiv get_positional_shadow_atlas_quadrant_subdiv(quadrant: int) const

The subdivision amount of the second quadrant on the shadow atlas.

### positional_shadow_atlas_quad_2 {#positional_shadow_atlas_quad_2}

```gdscript
positional_shadow_atlas_quad_2: int
```

- void set_positional_shadow_atlas_quadrant_subdiv(quadrant: int, subdiv: PositionalShadowAtlasQuadrantSubdiv)
- PositionalShadowAtlasQuadrantSubdiv get_positional_shadow_atlas_quadrant_subdiv(quadrant: int) const

The subdivision amount of the third quadrant on the shadow atlas.

### positional_shadow_atlas_quad_3 {#positional_shadow_atlas_quad_3}

```gdscript
positional_shadow_atlas_quad_3: int
```

- void set_positional_shadow_atlas_quadrant_subdiv(quadrant: int, subdiv: PositionalShadowAtlasQuadrantSubdiv)
- PositionalShadowAtlasQuadrantSubdiv get_positional_shadow_atlas_quadrant_subdiv(quadrant: int) const

The subdivision amount of the fourth quadrant on the shadow atlas.

### canvas_transform {#canvas_transform}

```gdscript
canvas_transform: Transform2D
```

- void set_canvas_transform(value: Transform2D)
- Transform2D get_canvas_transform()

The canvas transform of the viewport, useful for changing the on-screen positions of all child CanvasItems. This is relative to the global canvas transform of the viewport.

### global_canvas_transform {#global_canvas_transform}

```gdscript
global_canvas_transform: Transform2D
```

- void set_global_canvas_transform(value: Transform2D)
- Transform2D get_global_canvas_transform()

The global canvas transform of the viewport. The canvas transform is relative to this.

### canvas_cull_mask {#canvas_cull_mask}

```gdscript
canvas_cull_mask: int
```

- void set_canvas_cull_mask(value: int)
- int get_canvas_cull_mask()

The rendering layers in which this Viewport renders CanvasItem nodes.

Note: A CanvasItem does not inherit its parents' visibility layers. See CanvasItem.visibility_layer's description for details.

### oversampling {#oversampling}

```gdscript
oversampling: bool
```

- void set_use_oversampling(value: bool)
- bool is_using_oversampling()

If true and one of the following conditions are true: SubViewport.size_2d_override_stretch and SubViewport.size_2d_override are set, Window.content_scale_factor is set and scaling is enabled, oversampling_override is set, font and DPITexture oversampling are enabled.

### oversampling_override {#oversampling_override}

```gdscript
oversampling_override: float
```

- void set_oversampling_override(value: float)
- float get_oversampling_override()

If greater than zero, this value is used as the font oversampling factor, otherwise oversampling is equal to viewport scale.

## 信号

### size_changed {#size_changed}

```gdscript
signal size_changed()
```

Emitted when the size of the viewport is changed, whether by resizing of window, or some other means.

### gui_focus_changed {#gui_focus_changed}

```gdscript
signal gui_focus_changed(node: Control)
```

Emitted when a Control node grabs keyboard focus.

Note: A Control node losing focus doesn't cause this signal to be emitted.

## 枚举

### PositionalShadowAtlasQuadrantSubdiv {#enum-PositionalShadowAtlasQuadrantSubdiv}

```gdscript
enum PositionalShadowAtlasQuadrantSubdiv { SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED = 0, SHADOW_ATLAS_QUADRANT_SUBDIV_1 = 1, SHADOW_ATLAS_QUADRANT_SUBDIV_4 = 2, SHADOW_ATLAS_QUADRANT_SUBDIV_16 = 3, SHADOW_ATLAS_QUADRANT_SUBDIV_64 = 4, SHADOW_ATLAS_QUADRANT_SUBDIV_256 = 5, SHADOW_ATLAS_QUADRANT_SUBDIV_1024 = 6, SHADOW_ATLAS_QUADRANT_SUBDIV_MAX = 7 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED {#SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED = 0
```

This quadrant will not be used.

#### SHADOW_ATLAS_QUADRANT_SUBDIV_1 {#SHADOW_ATLAS_QUADRANT_SUBDIV_1}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_1 = 1
```

This quadrant will only be used by one shadow map.

#### SHADOW_ATLAS_QUADRANT_SUBDIV_4 {#SHADOW_ATLAS_QUADRANT_SUBDIV_4}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_4 = 2
```

This quadrant will be split in 4 and used by up to 4 shadow maps.

#### SHADOW_ATLAS_QUADRANT_SUBDIV_16 {#SHADOW_ATLAS_QUADRANT_SUBDIV_16}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_16 = 3
```

This quadrant will be split 16 ways and used by up to 16 shadow maps.

#### SHADOW_ATLAS_QUADRANT_SUBDIV_64 {#SHADOW_ATLAS_QUADRANT_SUBDIV_64}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_64 = 4
```

This quadrant will be split 64 ways and used by up to 64 shadow maps.

#### SHADOW_ATLAS_QUADRANT_SUBDIV_256 {#SHADOW_ATLAS_QUADRANT_SUBDIV_256}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_256 = 5
```

This quadrant will be split 256 ways and used by up to 256 shadow maps. Unless the positional_shadow_atlas_size is very high, the shadows in this quadrant will be very low resolution.

#### SHADOW_ATLAS_QUADRANT_SUBDIV_1024 {#SHADOW_ATLAS_QUADRANT_SUBDIV_1024}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_1024 = 6
```

This quadrant will be split 1024 ways and used by up to 1024 shadow maps. Unless the positional_shadow_atlas_size is very high, the shadows in this quadrant will be very low resolution.

#### SHADOW_ATLAS_QUADRANT_SUBDIV_MAX {#SHADOW_ATLAS_QUADRANT_SUBDIV_MAX}

```gdscript
SHADOW_ATLAS_QUADRANT_SUBDIV_MAX = 7
```

Represents the size of the PositionalShadowAtlasQuadrantSubdiv enum.

---

### Scaling3DMode {#enum-Scaling3DMode}

```gdscript
enum Scaling3DMode { SCALING_3D_MODE_BILINEAR = 0, SCALING_3D_MODE_FSR = 1, SCALING_3D_MODE_FSR2 = 2, SCALING_3D_MODE_METALFX_SPATIAL = 3, SCALING_3D_MODE_METALFX_TEMPORAL = 4, SCALING_3D_MODE_NEAREST = 5, SCALING_3D_MODE_MAX = 6 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCALING_3D_MODE_BILINEAR {#SCALING_3D_MODE_BILINEAR}

```gdscript
SCALING_3D_MODE_BILINEAR = 0
```

Use bilinear scaling for the viewport's 3D buffer. The amount of scaling can be set using scaling_3d_scale. Values less than 1.0 will result in undersampling while values greater than 1.0 will result in supersampling. A value of 1.0 disables scaling.

#### SCALING_3D_MODE_FSR {#SCALING_3D_MODE_FSR}

```gdscript
SCALING_3D_MODE_FSR = 1
```

Use AMD FidelityFX Super Resolution 1.0 upscaling for the viewport's 3D buffer. The amount of scaling can be set using scaling_3d_scale. Values less than 1.0 will result in the viewport being upscaled using FSR. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 disables scaling.

#### SCALING_3D_MODE_FSR2 {#SCALING_3D_MODE_FSR2}

```gdscript
SCALING_3D_MODE_FSR2 = 2
```

Use AMD FidelityFX Super Resolution 2.2 upscaling for the viewport's 3D buffer. The amount of scaling can be set using scaling_3d_scale. Values less than 1.0 will result in the viewport being upscaled using FSR2. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 will use FSR2 at native resolution as a TAA solution.

#### SCALING_3D_MODE_METALFX_SPATIAL {#SCALING_3D_MODE_METALFX_SPATIAL}

```gdscript
SCALING_3D_MODE_METALFX_SPATIAL = 3
```

Use the MetalFX spatial upscaler [https://developer.apple.com/documentation/metalfx/mtlfxspatialscaler#overview] for the viewport's 3D buffer.

The amount of scaling can be set using scaling_3d_scale.

Values less than 1.0 will result in the viewport being upscaled using MetalFX. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 disables scaling.

More information: MetalFX [https://developer.apple.com/documentation/metalfx].

Note: Only supported when the Metal rendering driver is in use, which limits this scaling mode to macOS and iOS.

#### SCALING_3D_MODE_METALFX_TEMPORAL {#SCALING_3D_MODE_METALFX_TEMPORAL}

```gdscript
SCALING_3D_MODE_METALFX_TEMPORAL = 4
```

Use the MetalFX temporal upscaler [https://developer.apple.com/documentation/metalfx/mtlfxtemporalscaler#overview] for the viewport's 3D buffer.

The amount of scaling can be set using scaling_3d_scale. To determine the minimum input scale, use the RenderingDevice.limit_get() method with RenderingDevice.LIMIT_METALFX_TEMPORAL_SCALER_MIN_SCALE.

Values less than 1.0 will result in the viewport being upscaled using MetalFX. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 will use MetalFX at native resolution as a TAA solution.

More information: MetalFX [https://developer.apple.com/documentation/metalfx].

Note: Only supported when the Metal rendering driver is in use, which limits this scaling mode to macOS and iOS.

#### SCALING_3D_MODE_NEAREST {#SCALING_3D_MODE_NEAREST}

```gdscript
SCALING_3D_MODE_NEAREST = 5
```

Use nearest-neighbor filtering for the viewport's 3D buffer. This looks crisper than SCALING_3D_MODE_BILINEAR and has no additional rendering cost. The amount of scaling can be set using scaling_3d_scale. Values greater than 1.0 are not supported and bilinear downsampling will be used instead. A value of 1.0 disables scaling.

Note: When using the Nearest scaling mode, to avoid uneven pixel scaling, it's highly recommended to use a value equal to an integer divisor with a dividend of 1. For example, it's best to use a scale of 0.5 (1/2), 0.3333 (1/3), 0.25 (1/4), 0.2 (1/5), and so on.

#### SCALING_3D_MODE_MAX {#SCALING_3D_MODE_MAX}

```gdscript
SCALING_3D_MODE_MAX = 6
```

Represents the size of the Scaling3DMode enum.

---

### MSAA {#enum-MSAA}

```gdscript
enum MSAA { MSAA_DISABLED = 0, MSAA_2X = 1, MSAA_4X = 2, MSAA_8X = 3, MSAA_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MSAA_DISABLED {#MSAA_DISABLED}

```gdscript
MSAA_DISABLED = 0
```

Multisample antialiasing mode disabled. This is the default value, and is also the fastest setting.

#### MSAA_2X {#MSAA_2X}

```gdscript
MSAA_2X = 1
```

Use 2× Multisample Antialiasing. This has a moderate performance cost. It helps reduce aliasing noticeably, but 4× MSAA still looks substantially better.

#### MSAA_4X {#MSAA_4X}

```gdscript
MSAA_4X = 2
```

Use 4× Multisample Antialiasing. This has a significant performance cost, and is generally a good compromise between performance and quality.

#### MSAA_8X {#MSAA_8X}

```gdscript
MSAA_8X = 3
```

Use 8× Multisample Antialiasing. This has a very high performance cost. The difference between 4× and 8× MSAA may not always be visible in real gameplay conditions. Likely unsupported on low-end and older hardware.

#### MSAA_MAX {#MSAA_MAX}

```gdscript
MSAA_MAX = 4
```

Represents the size of the MSAA enum.

---

### AnisotropicFiltering {#enum-AnisotropicFiltering}

```gdscript
enum AnisotropicFiltering { ANISOTROPY_DISABLED = 0, ANISOTROPY_2X = 1, ANISOTROPY_4X = 2, ANISOTROPY_8X = 3, ANISOTROPY_16X = 4, ANISOTROPY_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ANISOTROPY_DISABLED {#ANISOTROPY_DISABLED}

```gdscript
ANISOTROPY_DISABLED = 0
```

Anisotropic filtering is disabled.

#### ANISOTROPY_2X {#ANISOTROPY_2X}

```gdscript
ANISOTROPY_2X = 1
```

Use 2× anisotropic filtering.

#### ANISOTROPY_4X {#ANISOTROPY_4X}

```gdscript
ANISOTROPY_4X = 2
```

Use 4× anisotropic filtering. This is the default value.

#### ANISOTROPY_8X {#ANISOTROPY_8X}

```gdscript
ANISOTROPY_8X = 3
```

Use 8× anisotropic filtering.

#### ANISOTROPY_16X {#ANISOTROPY_16X}

```gdscript
ANISOTROPY_16X = 4
```

Use 16× anisotropic filtering.

#### ANISOTROPY_MAX {#ANISOTROPY_MAX}

```gdscript
ANISOTROPY_MAX = 5
```

Represents the size of the AnisotropicFiltering enum.

---

### ScreenSpaceAA {#enum-ScreenSpaceAA}

```gdscript
enum ScreenSpaceAA { SCREEN_SPACE_AA_DISABLED = 0, SCREEN_SPACE_AA_FXAA = 1, SCREEN_SPACE_AA_SMAA = 2, SCREEN_SPACE_AA_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SCREEN_SPACE_AA_DISABLED {#SCREEN_SPACE_AA_DISABLED}

```gdscript
SCREEN_SPACE_AA_DISABLED = 0
```

Do not perform any antialiasing in the full screen post-process.

#### SCREEN_SPACE_AA_FXAA {#SCREEN_SPACE_AA_FXAA}

```gdscript
SCREEN_SPACE_AA_FXAA = 1
```

Use fast approximate antialiasing. FXAA is a popular screen-space antialiasing method, which is fast but will make the image look blurry, especially at lower resolutions. It can still work relatively well at large resolutions such as 1440p and 4K.

#### SCREEN_SPACE_AA_SMAA {#SCREEN_SPACE_AA_SMAA}

```gdscript
SCREEN_SPACE_AA_SMAA = 2
```

Use subpixel morphological antialiasing. SMAA may produce clearer results than FXAA, but at a slightly higher performance cost.

#### SCREEN_SPACE_AA_MAX {#SCREEN_SPACE_AA_MAX}

```gdscript
SCREEN_SPACE_AA_MAX = 3
```

Represents the size of the ScreenSpaceAA enum.

---

### RenderInfo {#enum-RenderInfo}

```gdscript
enum RenderInfo { RENDER_INFO_OBJECTS_IN_FRAME = 0, RENDER_INFO_PRIMITIVES_IN_FRAME = 1, RENDER_INFO_DRAW_CALLS_IN_FRAME = 2, RENDER_INFO_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_INFO_OBJECTS_IN_FRAME {#RENDER_INFO_OBJECTS_IN_FRAME}

```gdscript
RENDER_INFO_OBJECTS_IN_FRAME = 0
```

Amount of objects in frame.

#### RENDER_INFO_PRIMITIVES_IN_FRAME {#RENDER_INFO_PRIMITIVES_IN_FRAME}

```gdscript
RENDER_INFO_PRIMITIVES_IN_FRAME = 1
```

Amount of vertices in frame.

#### RENDER_INFO_DRAW_CALLS_IN_FRAME {#RENDER_INFO_DRAW_CALLS_IN_FRAME}

```gdscript
RENDER_INFO_DRAW_CALLS_IN_FRAME = 2
```

Amount of draw calls in frame.

#### RENDER_INFO_MAX {#RENDER_INFO_MAX}

```gdscript
RENDER_INFO_MAX = 3
```

Represents the size of the RenderInfo enum.

---

### RenderInfoType {#enum-RenderInfoType}

```gdscript
enum RenderInfoType { RENDER_INFO_TYPE_VISIBLE = 0, RENDER_INFO_TYPE_SHADOW = 1, RENDER_INFO_TYPE_CANVAS = 2, RENDER_INFO_TYPE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RENDER_INFO_TYPE_VISIBLE {#RENDER_INFO_TYPE_VISIBLE}

```gdscript
RENDER_INFO_TYPE_VISIBLE = 0
```

Visible render pass (excluding shadows).

#### RENDER_INFO_TYPE_SHADOW {#RENDER_INFO_TYPE_SHADOW}

```gdscript
RENDER_INFO_TYPE_SHADOW = 1
```

Shadow render pass. Objects will be rendered several times depending on the number of amounts of lights with shadows and the number of directional shadow splits.

#### RENDER_INFO_TYPE_CANVAS {#RENDER_INFO_TYPE_CANVAS}

```gdscript
RENDER_INFO_TYPE_CANVAS = 2
```

Canvas item rendering. This includes all 2D rendering.

#### RENDER_INFO_TYPE_MAX {#RENDER_INFO_TYPE_MAX}

```gdscript
RENDER_INFO_TYPE_MAX = 3
```

Represents the size of the RenderInfoType enum.

---

### DebugDraw {#enum-DebugDraw}

```gdscript
enum DebugDraw { DEBUG_DRAW_DISABLED = 0, DEBUG_DRAW_UNSHADED = 1, DEBUG_DRAW_LIGHTING = 2, DEBUG_DRAW_OVERDRAW = 3, DEBUG_DRAW_WIREFRAME = 4, DEBUG_DRAW_NORMAL_BUFFER = 5, DEBUG_DRAW_VOXEL_GI_ALBEDO = 6, DEBUG_DRAW_VOXEL_GI_LIGHTING = 7, DEBUG_DRAW_VOXEL_GI_EMISSION = 8, DEBUG_DRAW_SHADOW_ATLAS = 9, DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS = 10, DEBUG_DRAW_SCENE_LUMINANCE = 11, DEBUG_DRAW_SSAO = 12, DEBUG_DRAW_SSIL = 13, DEBUG_DRAW_PSSM_SPLITS = 14, DEBUG_DRAW_DECAL_ATLAS = 15, DEBUG_DRAW_SDFGI = 16, DEBUG_DRAW_SDFGI_PROBES = 17, DEBUG_DRAW_GI_BUFFER = 18, DEBUG_DRAW_DISABLE_LOD = 19, DEBUG_DRAW_CLUSTER_OMNI_LIGHTS = 20, DEBUG_DRAW_CLUSTER_SPOT_LIGHTS = 21, DEBUG_DRAW_CLUSTER_DECALS = 22, DEBUG_DRAW_CLUSTER_REFLECTION_PROBES = 23, DEBUG_DRAW_OCCLUDERS = 24, DEBUG_DRAW_MOTION_VECTORS = 25, DEBUG_DRAW_INTERNAL_BUFFER = 26, DEBUG_DRAW_CLUSTER_AREA_LIGHTS = 27, DEBUG_DRAW_AREA_LIGHT_ATLAS = 28 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEBUG_DRAW_DISABLED {#DEBUG_DRAW_DISABLED}

```gdscript
DEBUG_DRAW_DISABLED = 0
```

Objects are displayed normally.

#### DEBUG_DRAW_UNSHADED {#DEBUG_DRAW_UNSHADED}

```gdscript
DEBUG_DRAW_UNSHADED = 1
```

Objects are displayed without light information.

#### DEBUG_DRAW_LIGHTING {#DEBUG_DRAW_LIGHTING}

```gdscript
DEBUG_DRAW_LIGHTING = 2
```

Objects are displayed without textures and only with lighting information.

Note: When using this debug draw mode, custom shaders are ignored since all materials in the scene temporarily use a debug material. This means the result from custom shader functions (such as vertex displacement) won't be visible anymore when using this debug draw mode.

#### DEBUG_DRAW_OVERDRAW {#DEBUG_DRAW_OVERDRAW}

```gdscript
DEBUG_DRAW_OVERDRAW = 3
```

Objects are displayed semi-transparent with additive blending so you can see where they are drawing over top of one another. A higher overdraw means you are wasting performance on drawing pixels that are being hidden behind others.

Note: When using this debug draw mode, custom shaders are ignored since all materials in the scene temporarily use a debug material. This means the result from custom shader functions (such as vertex displacement) won't be visible anymore when using this debug draw mode.

#### DEBUG_DRAW_WIREFRAME {#DEBUG_DRAW_WIREFRAME}

```gdscript
DEBUG_DRAW_WIREFRAME = 4
```

Objects are displayed as wireframe models.

Note: RenderingServer.set_debug_generate_wireframes() must be called before loading any meshes for wireframes to be visible when using the Compatibility renderer.

Note: In the Compatibility renderer, backfaces are always visible when using wireframe rendering. In the Forward+ and Mobile renderers, wireframes follow the material's backface culling properties instead.

#### DEBUG_DRAW_NORMAL_BUFFER {#DEBUG_DRAW_NORMAL_BUFFER}

```gdscript
DEBUG_DRAW_NORMAL_BUFFER = 5
```

Objects are displayed without lighting information and their textures replaced by normal mapping.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_VOXEL_GI_ALBEDO {#DEBUG_DRAW_VOXEL_GI_ALBEDO}

```gdscript
DEBUG_DRAW_VOXEL_GI_ALBEDO = 6
```

Objects are displayed with only the albedo value from VoxelGIs. Requires at least one visible VoxelGI node that has been baked to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_VOXEL_GI_LIGHTING {#DEBUG_DRAW_VOXEL_GI_LIGHTING}

```gdscript
DEBUG_DRAW_VOXEL_GI_LIGHTING = 7
```

Objects are displayed with only the lighting value from VoxelGIs. Requires at least one visible VoxelGI node that has been baked to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_VOXEL_GI_EMISSION {#DEBUG_DRAW_VOXEL_GI_EMISSION}

```gdscript
DEBUG_DRAW_VOXEL_GI_EMISSION = 8
```

Objects are displayed with only the emission color from VoxelGIs. Requires at least one visible VoxelGI node that has been baked to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_SHADOW_ATLAS {#DEBUG_DRAW_SHADOW_ATLAS}

```gdscript
DEBUG_DRAW_SHADOW_ATLAS = 9
```

Draws the shadow atlas that stores shadows from OmniLight3Ds and SpotLight3Ds in the upper left quadrant of the Viewport.

#### DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS {#DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS}

```gdscript
DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS = 10
```

Draws the shadow atlas that stores shadows from DirectionalLight3Ds in the upper left quadrant of the Viewport.

#### DEBUG_DRAW_SCENE_LUMINANCE {#DEBUG_DRAW_SCENE_LUMINANCE}

```gdscript
DEBUG_DRAW_SCENE_LUMINANCE = 11
```

Draws the scene luminance buffer (if available) in the upper left quadrant of the Viewport.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### DEBUG_DRAW_SSAO {#DEBUG_DRAW_SSAO}

```gdscript
DEBUG_DRAW_SSAO = 12
```

Draws the screen-space ambient occlusion texture instead of the scene so that you can clearly see how it is affecting objects. In order for this display mode to work, you must have Environment.ssao_enabled set in your WorldEnvironment.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_SSIL {#DEBUG_DRAW_SSIL}

```gdscript
DEBUG_DRAW_SSIL = 13
```

Draws the screen-space indirect lighting texture instead of the scene so that you can clearly see how it is affecting objects. In order for this display mode to work, you must have Environment.ssil_enabled set in your WorldEnvironment.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_PSSM_SPLITS {#DEBUG_DRAW_PSSM_SPLITS}

```gdscript
DEBUG_DRAW_PSSM_SPLITS = 14
```

Colors each PSSM split for the DirectionalLight3Ds in the scene a different color so you can see where the splits are. In order (from closest to furthest from the camera), they are colored red, green, blue, and yellow.

Note: When using this debug draw mode, custom shaders are ignored since all materials in the scene temporarily use a debug material. This means the result from custom shader functions (such as vertex displacement) won't be visible anymore when using this debug draw mode.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### DEBUG_DRAW_DECAL_ATLAS {#DEBUG_DRAW_DECAL_ATLAS}

```gdscript
DEBUG_DRAW_DECAL_ATLAS = 15
```

Draws the decal atlas used by Decals and light projector textures in the upper left quadrant of the Viewport.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### DEBUG_DRAW_SDFGI {#DEBUG_DRAW_SDFGI}

```gdscript
DEBUG_DRAW_SDFGI = 16
```

Draws the cascades used to render signed distance field global illumination (SDFGI).

Does nothing if the current environment's Environment.sdfgi_enabled is false.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_SDFGI_PROBES {#DEBUG_DRAW_SDFGI_PROBES}

```gdscript
DEBUG_DRAW_SDFGI_PROBES = 17
```

Draws the probes used for signed distance field global illumination (SDFGI).

When in the editor, left-clicking a probe will display additional bright dots that show its occlusion information. A white dot means the light is not occluded at all at the dot's position, while a red dot means the light is fully occluded. Intermediate values are possible.

Does nothing if the current environment's Environment.sdfgi_enabled is false.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_GI_BUFFER {#DEBUG_DRAW_GI_BUFFER}

```gdscript
DEBUG_DRAW_GI_BUFFER = 18
```

Draws the buffer used for global illumination from VoxelGI or SDFGI. Requires VoxelGI (at least one visible baked VoxelGI node) or SDFGI (Environment.sdfgi_enabled) to be enabled to have a visible effect.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_DISABLE_LOD {#DEBUG_DRAW_DISABLE_LOD}

```gdscript
DEBUG_DRAW_DISABLE_LOD = 19
```

Draws all of the objects at their highest polycount regardless of their distance from the camera. No low level of detail (LOD) is applied.

#### DEBUG_DRAW_CLUSTER_OMNI_LIGHTS {#DEBUG_DRAW_CLUSTER_OMNI_LIGHTS}

```gdscript
DEBUG_DRAW_CLUSTER_OMNI_LIGHTS = 20
```

Draws the cluster used by OmniLight3D nodes to optimize light rendering.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_CLUSTER_SPOT_LIGHTS {#DEBUG_DRAW_CLUSTER_SPOT_LIGHTS}

```gdscript
DEBUG_DRAW_CLUSTER_SPOT_LIGHTS = 21
```

Draws the cluster used by SpotLight3D nodes to optimize light rendering.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_CLUSTER_DECALS {#DEBUG_DRAW_CLUSTER_DECALS}

```gdscript
DEBUG_DRAW_CLUSTER_DECALS = 22
```

Draws the cluster used by Decal nodes to optimize decal rendering.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_CLUSTER_REFLECTION_PROBES {#DEBUG_DRAW_CLUSTER_REFLECTION_PROBES}

```gdscript
DEBUG_DRAW_CLUSTER_REFLECTION_PROBES = 23
```

Draws the cluster used by ReflectionProbe nodes to optimize reflection probes.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_OCCLUDERS {#DEBUG_DRAW_OCCLUDERS}

```gdscript
DEBUG_DRAW_OCCLUDERS = 24
```

Draws the buffer used for occlusion culling.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### DEBUG_DRAW_MOTION_VECTORS {#DEBUG_DRAW_MOTION_VECTORS}

```gdscript
DEBUG_DRAW_MOTION_VECTORS = 25
```

Draws vector lines over the viewport to indicate the movement of pixels between frames.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_INTERNAL_BUFFER {#DEBUG_DRAW_INTERNAL_BUFFER}

```gdscript
DEBUG_DRAW_INTERNAL_BUFFER = 26
```

Draws the internal resolution buffer of the scene in linear colorspace before tonemapping or post-processing is applied.

Note: Only supported when using the Forward+ or Mobile rendering methods.

#### DEBUG_DRAW_CLUSTER_AREA_LIGHTS {#DEBUG_DRAW_CLUSTER_AREA_LIGHTS}

```gdscript
DEBUG_DRAW_CLUSTER_AREA_LIGHTS = 27
```

Draws the cluster used by AreaLight3D nodes to optimize light rendering.

Note: Only supported when using the Forward+ rendering method.

#### DEBUG_DRAW_AREA_LIGHT_ATLAS {#DEBUG_DRAW_AREA_LIGHT_ATLAS}

```gdscript
DEBUG_DRAW_AREA_LIGHT_ATLAS = 28
```

Draws the atlas used by AreaLight3D nodes in the upper left quadrant of the Viewport.

Note: Only supported when using the Forward+ or Mobile rendering method.

---

### DefaultCanvasItemTextureFilter {#enum-DefaultCanvasItemTextureFilter}

```gdscript
enum DefaultCanvasItemTextureFilter { DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST = 0, DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR = 1, DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS = 2, DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS = 3, DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_PARENT_NODE = 4, DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST {#DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST = 0
```

The texture filter reads from the nearest pixel only. This makes the texture look pixelated from up close, and grainy from a distance (due to mipmaps not being sampled).

#### DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR {#DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR = 1
```

The texture filter blends between the nearest 4 pixels. This makes the texture look smooth from up close, and grainy from a distance (due to mipmaps not being sampled).

#### DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS {#DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS = 2
```

The texture filter blends between the nearest 4 pixels and between the nearest 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true). This makes the texture look smooth from up close, and smooth from a distance.

Use this for non-pixel art textures that may be viewed at a low scale (e.g. due to Camera2D zoom or sprite scaling), as mipmaps are important to smooth out pixels that are smaller than on-screen pixels.

#### DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS {#DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS = 3
```

The texture filter reads from the nearest pixel and blends between the nearest 2 mipmaps (or uses the nearest mipmap if ProjectSettings.rendering/textures/default_filters/use_nearest_mipmap_filter is true). This makes the texture look pixelated from up close, and smooth from a distance.

Use this for non-pixel art textures that may be viewed at a low scale (e.g. due to Camera2D zoom or sprite scaling), as mipmaps are important to smooth out pixels that are smaller than on-screen pixels.

#### DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_PARENT_NODE {#DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_PARENT_NODE}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_PARENT_NODE = 4
```

The Viewport will inherit the filter from its parent CanvasItem or Viewport.

#### DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX {#DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX = 5
```

Represents the size of the DefaultCanvasItemTextureFilter enum.

---

### DefaultCanvasItemTextureRepeat {#enum-DefaultCanvasItemTextureRepeat}

```gdscript
enum DefaultCanvasItemTextureRepeat { DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED = 0, DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED = 1, DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR = 2, DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_PARENT_NODE = 3, DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED {#DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED = 0
```

Disables textures repeating. Instead, when reading UVs outside the 0-1 range, the value will be clamped to the edge of the texture, resulting in a stretched out look at the borders of the texture.

#### DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED {#DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED = 1
```

Enables the texture to repeat when UV coordinates are outside the 0-1 range. If using one of the linear filtering modes, this can result in artifacts at the edges of a texture when the sampler filters across the edges of the texture.

#### DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR {#DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR = 2
```

Flip the texture when repeating so that the edge lines up instead of abruptly changing.

#### DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_PARENT_NODE {#DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_PARENT_NODE}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_PARENT_NODE = 3
```

The Viewport will inherit the repeat mode from its parent CanvasItem or Viewport.

#### DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX {#DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX}

```gdscript
DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX = 4
```

Represents the size of the DefaultCanvasItemTextureRepeat enum.

---

### SDFOversize {#enum-SDFOversize}

```gdscript
enum SDFOversize { SDF_OVERSIZE_100_PERCENT = 0, SDF_OVERSIZE_120_PERCENT = 1, SDF_OVERSIZE_150_PERCENT = 2, SDF_OVERSIZE_200_PERCENT = 3, SDF_OVERSIZE_MAX = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SDF_OVERSIZE_100_PERCENT {#SDF_OVERSIZE_100_PERCENT}

```gdscript
SDF_OVERSIZE_100_PERCENT = 0
```

The signed distance field only covers the viewport's own rectangle.

#### SDF_OVERSIZE_120_PERCENT {#SDF_OVERSIZE_120_PERCENT}

```gdscript
SDF_OVERSIZE_120_PERCENT = 1
```

The signed distance field is expanded to cover 20% of the viewport's size around the borders.

#### SDF_OVERSIZE_150_PERCENT {#SDF_OVERSIZE_150_PERCENT}

```gdscript
SDF_OVERSIZE_150_PERCENT = 2
```

The signed distance field is expanded to cover 50% of the viewport's size around the borders.

#### SDF_OVERSIZE_200_PERCENT {#SDF_OVERSIZE_200_PERCENT}

```gdscript
SDF_OVERSIZE_200_PERCENT = 3
```

The signed distance field is expanded to cover 100% (double) of the viewport's size around the borders.

#### SDF_OVERSIZE_MAX {#SDF_OVERSIZE_MAX}

```gdscript
SDF_OVERSIZE_MAX = 4
```

Represents the size of the SDFOversize enum.

---

### SDFScale {#enum-SDFScale}

```gdscript
enum SDFScale { SDF_SCALE_100_PERCENT = 0, SDF_SCALE_50_PERCENT = 1, SDF_SCALE_25_PERCENT = 2, SDF_SCALE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SDF_SCALE_100_PERCENT {#SDF_SCALE_100_PERCENT}

```gdscript
SDF_SCALE_100_PERCENT = 0
```

The signed distance field is rendered at full resolution.

#### SDF_SCALE_50_PERCENT {#SDF_SCALE_50_PERCENT}

```gdscript
SDF_SCALE_50_PERCENT = 1
```

The signed distance field is rendered at half the resolution of this viewport.

#### SDF_SCALE_25_PERCENT {#SDF_SCALE_25_PERCENT}

```gdscript
SDF_SCALE_25_PERCENT = 2
```

The signed distance field is rendered at a quarter the resolution of this viewport.

#### SDF_SCALE_MAX {#SDF_SCALE_MAX}

```gdscript
SDF_SCALE_MAX = 3
```

Represents the size of the SDFScale enum.

---

### VRSMode {#enum-VRSMode}

```gdscript
enum VRSMode { VRS_DISABLED = 0, VRS_TEXTURE = 1, VRS_XR = 2, VRS_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VRS_DISABLED {#VRS_DISABLED}

```gdscript
VRS_DISABLED = 0
```

Variable Rate Shading is disabled.

#### VRS_TEXTURE {#VRS_TEXTURE}

```gdscript
VRS_TEXTURE = 1
```

Variable Rate Shading uses a texture. Note, for stereoscopic use a texture atlas with a texture for each view.

#### VRS_XR {#VRS_XR}

```gdscript
VRS_XR = 2
```

Variable Rate Shading's texture is supplied by the primary XRInterface.

#### VRS_MAX {#VRS_MAX}

```gdscript
VRS_MAX = 3
```

Represents the size of the VRSMode enum.

---

### VRSUpdateMode {#enum-VRSUpdateMode}

```gdscript
enum VRSUpdateMode { VRS_UPDATE_DISABLED = 0, VRS_UPDATE_ONCE = 1, VRS_UPDATE_ALWAYS = 2, VRS_UPDATE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VRS_UPDATE_DISABLED {#VRS_UPDATE_DISABLED}

```gdscript
VRS_UPDATE_DISABLED = 0
```

The input texture for variable rate shading will not be processed.

#### VRS_UPDATE_ONCE {#VRS_UPDATE_ONCE}

```gdscript
VRS_UPDATE_ONCE = 1
```

The input texture for variable rate shading will be processed once.

#### VRS_UPDATE_ALWAYS {#VRS_UPDATE_ALWAYS}

```gdscript
VRS_UPDATE_ALWAYS = 2
```

The input texture for variable rate shading will be processed each frame.

#### VRS_UPDATE_MAX {#VRS_UPDATE_MAX}

```gdscript
VRS_UPDATE_MAX = 3
```

Represents the size of the VRSUpdateMode enum.

---
