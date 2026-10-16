# Camera2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **Camera2D**

> 2D 相机节点。控制视口显示范围。

<details><summary>官方英文描述</summary>

Camera node for 2D scenes. It forces the screen (current layer) to scroll following this node. This makes it easier (and faster) to program scrollable scenes than manually changing the position of CanvasItem-based nodes.

Cameras register themselves in the nearest Viewport node (when ascending the tree). Only one camera can be active per viewport. If no viewport is available ascending the tree, the camera will register in the global viewport.

This node is intended to be a simple helper to get things going quickly, but more functionality may be desired to change how the camera works. To make your own custom camera node, inherit it from Node2D and change the transform of the canvas by setting Viewport.canvas_transform in Viewport (you can obtain the current Viewport by using Node.get_viewport()).

Note that the Camera2D node's Node2D.global_position doesn't represent the actual position of the screen, which may differ due to applied smoothing or limits. You can use get_screen_center_position() to get the real position. Same for the node's Node2D.global_rotation which may be different due to applied rotation smoothing. You can use get_screen_rotation() to get the current rotation of the screen.

</details>

## 方法

### align {#align}

```gdscript
void align()
```

Aligns the camera to the tracked node.

Note: Calling force_update_scroll() after this method is not required.

### force_update_scroll {#force_update_scroll}

```gdscript
void force_update_scroll()
```

Forces the camera to update scroll immediately.

### get_anchor_mode {#get_anchor_mode}

```gdscript
Camera2D.AnchorMode get_anchor_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_viewport {#get_custom_viewport}

```gdscript
Node get_custom_viewport() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_horizontal_offset {#get_drag_horizontal_offset}

```gdscript
float get_drag_horizontal_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_drag_margin {#get_drag_margin}

```gdscript
float get_drag_margin(margin: Side) const
```

Returns the specified Side's margin. See also drag_bottom_margin, drag_top_margin, drag_left_margin, and drag_right_margin.

### get_drag_vertical_offset {#get_drag_vertical_offset}

```gdscript
float get_drag_vertical_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_limit {#get_limit}

```gdscript
int get_limit(margin: Side) const
```

Returns the camera limit for the specified Side. See also limit_bottom, limit_top, limit_left, and limit_right.

### get_offset {#get_offset}

```gdscript
Vector2 get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position_smoothing_speed {#get_position_smoothing_speed}

```gdscript
float get_position_smoothing_speed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_callback {#get_process_callback}

```gdscript
Camera2D.Camera2DProcessCallback get_process_callback() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rotation_smoothing_speed {#get_rotation_smoothing_speed}

```gdscript
float get_rotation_smoothing_speed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_screen_center_position {#get_screen_center_position}

```gdscript
Vector2 get_screen_center_position() const
```

Returns the center of the screen from this camera's point of view, in global coordinates.

Note: The exact targeted position of the camera may be different. See get_target_position().

### get_screen_rotation {#get_screen_rotation}

```gdscript
float get_screen_rotation() const
```

Returns the current screen rotation from this camera's point of view.

Note: The screen rotation can be different from Node2D.global_rotation if the camera is rotating smoothly due to rotation_smoothing_enabled.

### get_target_position {#get_target_position}

```gdscript
Vector2 get_target_position() const
```

Returns this camera's target position, in global coordinates.

Note: The returned value is not the same as Node2D.global_position, as it is affected by the drag properties. It is also not the same as the current position if position_smoothing_enabled is true (see get_screen_center_position()).

### get_zoom {#get_zoom}

```gdscript
Vector2 get_zoom() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_current {#is_current}

```gdscript
bool is_current() const
```

Returns true if this Camera2D is the active camera (see Viewport.get_camera_2d()).

### is_drag_horizontal_enabled {#is_drag_horizontal_enabled}

```gdscript
bool is_drag_horizontal_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_drag_vertical_enabled {#is_drag_vertical_enabled}

```gdscript
bool is_drag_vertical_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_enabled {#is_enabled}

```gdscript
bool is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_ignoring_rotation {#is_ignoring_rotation}

```gdscript
bool is_ignoring_rotation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_limit_drawing_enabled {#is_limit_drawing_enabled}

```gdscript
bool is_limit_drawing_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_limit_enabled {#is_limit_enabled}

```gdscript
bool is_limit_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_limit_smoothing_enabled {#is_limit_smoothing_enabled}

```gdscript
bool is_limit_smoothing_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_margin_drawing_enabled {#is_margin_drawing_enabled}

```gdscript
bool is_margin_drawing_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_position_smoothing_enabled {#is_position_smoothing_enabled}

```gdscript
bool is_position_smoothing_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_rotation_smoothing_enabled {#is_rotation_smoothing_enabled}

```gdscript
bool is_rotation_smoothing_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_screen_drawing_enabled {#is_screen_drawing_enabled}

```gdscript
bool is_screen_drawing_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### make_current {#make_current}

```gdscript
void make_current()
```

Forces this Camera2D to become the current active one. enabled must be true.

### reset_smoothing {#reset_smoothing}

```gdscript
void reset_smoothing()
```

Sets the camera's position immediately to its current smoothing destination.

This method has no effect if position_smoothing_enabled is false.

### set_anchor_mode {#set_anchor_mode}

```gdscript
void set_anchor_mode(anchor_mode: Camera2D.AnchorMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_custom_viewport {#set_custom_viewport}

```gdscript
void set_custom_viewport(viewport: Node)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_horizontal_enabled {#set_drag_horizontal_enabled}

```gdscript
void set_drag_horizontal_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_horizontal_offset {#set_drag_horizontal_offset}

```gdscript
void set_drag_horizontal_offset(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_margin {#set_drag_margin}

```gdscript
void set_drag_margin(margin: Side, drag_margin: float)
```

Sets the specified Side's margin. See also drag_bottom_margin, drag_top_margin, drag_left_margin, and drag_right_margin.

### set_drag_vertical_enabled {#set_drag_vertical_enabled}

```gdscript
void set_drag_vertical_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drag_vertical_offset {#set_drag_vertical_offset}

```gdscript
void set_drag_vertical_offset(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ignore_rotation {#set_ignore_rotation}

```gdscript
void set_ignore_rotation(ignore: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_limit {#set_limit}

```gdscript
void set_limit(margin: Side, limit: int)
```

Sets the camera limit for the specified Side. See also limit_bottom, limit_top, limit_left, and limit_right.

### set_limit_drawing_enabled {#set_limit_drawing_enabled}

```gdscript
void set_limit_drawing_enabled(limit_drawing_enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_limit_enabled {#set_limit_enabled}

```gdscript
void set_limit_enabled(limit_enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_limit_smoothing_enabled {#set_limit_smoothing_enabled}

```gdscript
void set_limit_smoothing_enabled(limit_smoothing_enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin_drawing_enabled {#set_margin_drawing_enabled}

```gdscript
void set_margin_drawing_enabled(margin_drawing_enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position_smoothing_enabled {#set_position_smoothing_enabled}

```gdscript
void set_position_smoothing_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_position_smoothing_speed {#set_position_smoothing_speed}

```gdscript
void set_position_smoothing_speed(position_smoothing_speed: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_callback {#set_process_callback}

```gdscript
void set_process_callback(mode: Camera2D.Camera2DProcessCallback)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_smoothing_enabled {#set_rotation_smoothing_enabled}

```gdscript
void set_rotation_smoothing_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rotation_smoothing_speed {#set_rotation_smoothing_speed}

```gdscript
void set_rotation_smoothing_speed(speed: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_screen_drawing_enabled {#set_screen_drawing_enabled}

```gdscript
void set_screen_drawing_enabled(screen_drawing_enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_zoom {#set_zoom}

```gdscript
void set_zoom(zoom: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### offset {#offset}

```gdscript
offset: Vector2
```

- void set_offset(value: Vector2)
- Vector2 get_offset()

The camera's relative offset. Useful for looking around or camera shake animations. The offsetted camera can go past the limits defined in limit_top, limit_bottom, limit_left and limit_right.

### anchor_mode {#anchor_mode}

```gdscript
anchor_mode: int
```

- void set_anchor_mode(value: AnchorMode)
- AnchorMode get_anchor_mode()

The Camera2D's anchor point.

### ignore_rotation {#ignore_rotation}

```gdscript
ignore_rotation: bool
```

- void set_ignore_rotation(value: bool)
- bool is_ignoring_rotation()

If true, the camera's rendered view is not affected by its Node2D.rotation and Node2D.global_rotation.

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

Controls whether the camera can be active or not. If true, the Camera2D will become the main camera when it enters the scene tree and there is no active camera currently (see Viewport.get_camera_2d()).

When the camera is currently active and enabled is set to false, the next enabled Camera2D in the scene tree will become active.

### zoom {#zoom}

```gdscript
zoom: Vector2
```

- void set_zoom(value: Vector2)
- Vector2 get_zoom()

The camera's zoom. Higher values are more zoomed in. For example, a zoom of Vector2(2.0, 2.0) will be twice as zoomed in on each axis (the view covers an area four times smaller). In contrast, a zoom of Vector2(0.5, 0.5) will be twice as zoomed out on each axis (the view covers an area four times larger). The X and Y components should generally always be set to the same value, unless you wish to stretch the camera view.

Note: FontFile.oversampling does not take Camera2D zoom into account. This means that zooming in/out will cause bitmap fonts and rasterized (non-MSDF) dynamic fonts to appear blurry or pixelated unless the font is part of a CanvasLayer that makes it ignore camera zoom. To ensure text remains crisp regardless of zoom, you can enable MSDF font rendering by enabling ProjectSettings.gui/theme/default_font_multichannel_signed_distance_field (applies to the default project font only), or enabling Multichannel Signed Distance Field in the import options of a DynamicFont for custom fonts. On system fonts, SystemFont.multichannel_signed_distance_field can be enabled in the inspector.

### custom_viewport {#custom_viewport}

```gdscript
custom_viewport: Viewport
```

- void set_custom_viewport(value: Node)
- Node get_custom_viewport()

The custom Viewport node attached to the Camera2D. If null or not a Viewport, uses the default viewport instead.

### process_callback {#process_callback}

```gdscript
process_callback: int
```

- void set_process_callback(value: Camera2DProcessCallback)
- Camera2DProcessCallback get_process_callback()

The camera's process callback.

### limit_enabled {#limit_enabled}

```gdscript
limit_enabled: bool
```

- void set_limit_enabled(value: bool)
- bool is_limit_enabled()

If true, the limits will be enabled. Disabling this will allow the camera to focus anywhere, when the four limit_* properties will not work.

### limit_left {#limit_left}

```gdscript
limit_left: int
```

- void set_limit(margin: Side, limit: int)
- int get_limit(margin: Side) const

Left scroll limit in pixels. The camera stops moving when reaching this value, but offset can push the view past the limit.

### limit_top {#limit_top}

```gdscript
limit_top: int
```

- void set_limit(margin: Side, limit: int)
- int get_limit(margin: Side) const

Top scroll limit in pixels. The camera stops moving when reaching this value, but offset can push the view past the limit.

### limit_right {#limit_right}

```gdscript
limit_right: int
```

- void set_limit(margin: Side, limit: int)
- int get_limit(margin: Side) const

Right scroll limit in pixels. The camera stops moving when reaching this value, but offset can push the view past the limit.

### limit_bottom {#limit_bottom}

```gdscript
limit_bottom: int
```

- void set_limit(margin: Side, limit: int)
- int get_limit(margin: Side) const

Bottom scroll limit in pixels. The camera stops moving when reaching this value, but offset can push the view past the limit.

### limit_smoothed {#limit_smoothed}

```gdscript
limit_smoothed: bool
```

- void set_limit_smoothing_enabled(value: bool)
- bool is_limit_smoothing_enabled()

If true, the camera smoothly stops when reaches its limits.

This property has no effect if position_smoothing_enabled is false.

Note: To immediately update the camera's position to be within limits without smoothing, even with this setting enabled, invoke reset_smoothing().

### position_smoothing_enabled {#position_smoothing_enabled}

```gdscript
position_smoothing_enabled: bool
```

- void set_position_smoothing_enabled(value: bool)
- bool is_position_smoothing_enabled()

If true, the camera's view smoothly moves towards its target position at position_smoothing_speed.

### position_smoothing_speed {#position_smoothing_speed}

```gdscript
position_smoothing_speed: float
```

- void set_position_smoothing_speed(value: float)
- float get_position_smoothing_speed()

Speed in pixels per second of the camera's smoothing effect when position_smoothing_enabled is true.

### rotation_smoothing_enabled {#rotation_smoothing_enabled}

```gdscript
rotation_smoothing_enabled: bool
```

- void set_rotation_smoothing_enabled(value: bool)
- bool is_rotation_smoothing_enabled()

If true, the camera's view smoothly rotates, via asymptotic smoothing, to align with its target rotation at rotation_smoothing_speed.

Note: This property has no effect if ignore_rotation is true.

### rotation_smoothing_speed {#rotation_smoothing_speed}

```gdscript
rotation_smoothing_speed: float
```

- void set_rotation_smoothing_speed(value: float)
- float get_rotation_smoothing_speed()

The angular, asymptotic speed of the camera's rotation smoothing effect when rotation_smoothing_enabled is true.

### drag_horizontal_enabled {#drag_horizontal_enabled}

```gdscript
drag_horizontal_enabled: bool
```

- void set_drag_horizontal_enabled(value: bool)
- bool is_drag_horizontal_enabled()

If true, the camera only moves when reaching the horizontal (left and right) drag margins. If false, the camera moves horizontally regardless of margins.

### drag_vertical_enabled {#drag_vertical_enabled}

```gdscript
drag_vertical_enabled: bool
```

- void set_drag_vertical_enabled(value: bool)
- bool is_drag_vertical_enabled()

If true, the camera only moves when reaching the vertical (top and bottom) drag margins. If false, the camera moves vertically regardless of the drag margins.

### drag_horizontal_offset {#drag_horizontal_offset}

```gdscript
drag_horizontal_offset: float
```

- void set_drag_horizontal_offset(value: float)
- float get_drag_horizontal_offset()

The relative horizontal drag offset of the camera between the right (-1) and left (1) drag margins.

Note: Used to set the initial horizontal drag offset; determine the current offset; or force the current offset. It's not automatically updated when drag_horizontal_enabled is true or the drag margins are changed.

### drag_vertical_offset {#drag_vertical_offset}

```gdscript
drag_vertical_offset: float
```

- void set_drag_vertical_offset(value: float)
- float get_drag_vertical_offset()

The relative vertical drag offset of the camera between the bottom (-1) and top (1) drag margins.

Note: Used to set the initial vertical drag offset; determine the current offset; or force the current offset. It's not automatically updated when drag_vertical_enabled is true or the drag margins are changed.

### drag_left_margin {#drag_left_margin}

```gdscript
drag_left_margin: float
```

- void set_drag_margin(margin: Side, drag_margin: float)
- float get_drag_margin(margin: Side) const

Left margin needed to drag the camera. A value of 1 makes the camera move only when reaching the left edge of the screen.

### drag_top_margin {#drag_top_margin}

```gdscript
drag_top_margin: float
```

- void set_drag_margin(margin: Side, drag_margin: float)
- float get_drag_margin(margin: Side) const

Top margin needed to drag the camera. A value of 1 makes the camera move only when reaching the top edge of the screen.

### drag_right_margin {#drag_right_margin}

```gdscript
drag_right_margin: float
```

- void set_drag_margin(margin: Side, drag_margin: float)
- float get_drag_margin(margin: Side) const

Right margin needed to drag the camera. A value of 1 makes the camera move only when reaching the right edge of the screen.

### drag_bottom_margin {#drag_bottom_margin}

```gdscript
drag_bottom_margin: float
```

- void set_drag_margin(margin: Side, drag_margin: float)
- float get_drag_margin(margin: Side) const

Bottom margin needed to drag the camera. A value of 1 makes the camera move only when reaching the bottom edge of the screen.

### editor_draw_screen {#editor_draw_screen}

```gdscript
editor_draw_screen: bool
```

- void set_screen_drawing_enabled(value: bool)
- bool is_screen_drawing_enabled()

If true, draws the camera's screen rectangle in the editor.

### editor_draw_limits {#editor_draw_limits}

```gdscript
editor_draw_limits: bool
```

- void set_limit_drawing_enabled(value: bool)
- bool is_limit_drawing_enabled()

If true, draws the camera's limits rectangle in the editor.

### editor_draw_drag_margin {#editor_draw_drag_margin}

```gdscript
editor_draw_drag_margin: bool
```

- void set_margin_drawing_enabled(value: bool)
- bool is_margin_drawing_enabled()

If true, draws the camera's drag margin rectangle in the editor.

## 枚举

### AnchorMode {#enum-AnchorMode}

```gdscript
enum AnchorMode { ANCHOR_MODE_FIXED_TOP_LEFT = 0, ANCHOR_MODE_DRAG_CENTER = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ANCHOR_MODE_FIXED_TOP_LEFT {#ANCHOR_MODE_FIXED_TOP_LEFT}

```gdscript
ANCHOR_MODE_FIXED_TOP_LEFT = 0
```

The camera's position is fixed so that the top-left corner is always at the origin.

#### ANCHOR_MODE_DRAG_CENTER {#ANCHOR_MODE_DRAG_CENTER}

```gdscript
ANCHOR_MODE_DRAG_CENTER = 1
```

The camera's position takes into account vertical/horizontal offsets and the screen size.

---

### Camera2DProcessCallback {#enum-Camera2DProcessCallback}

```gdscript
enum Camera2DProcessCallback { CAMERA2D_PROCESS_PHYSICS = 0, CAMERA2D_PROCESS_IDLE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CAMERA2D_PROCESS_PHYSICS {#CAMERA2D_PROCESS_PHYSICS}

```gdscript
CAMERA2D_PROCESS_PHYSICS = 0
```

The camera updates during physics frames (see Node.NOTIFICATION_INTERNAL_PHYSICS_PROCESS).

#### CAMERA2D_PROCESS_IDLE {#CAMERA2D_PROCESS_IDLE}

```gdscript
CAMERA2D_PROCESS_IDLE = 1
```

The camera updates during process frames (see Node.NOTIFICATION_INTERNAL_PROCESS).

---
