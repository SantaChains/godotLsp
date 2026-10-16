# Camera3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **Camera3D**

> Camera3D is a special node that displays what is visible from its current location. Cameras register themselves in the nearest Viewport node (when ascending the tree). Only one camera can be active per viewport. If no viewport is available ascending the tree, the camera will register in the global viewport. In other words, a camera just provides 3D display capabilities to a Viewport, and, without one, a scene registered in that Viewport (or higher viewports) can't be displayed.

<details><summary>官方英文描述（中文精译进行中）</summary>

Camera3D is a special node that displays what is visible from its current location. Cameras register themselves in the nearest Viewport node (when ascending the tree). Only one camera can be active per viewport. If no viewport is available ascending the tree, the camera will register in the global viewport. In other words, a camera just provides 3D display capabilities to a Viewport, and, without one, a scene registered in that Viewport (or higher viewports) can't be displayed.

</details>

## 方法

### clear_current {#clear_current}

```gdscript
void clear_current(enable_next: bool = true)
```

If this is the current camera, remove it from being current. If enable_next is true, request to make the next camera current, if any.

### get_attributes {#get_attributes}

```gdscript
CameraAttributes get_attributes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_camera_projection {#get_camera_projection}

```gdscript
Projection get_camera_projection() const
```

Returns the projection matrix that this camera uses to render to its associated viewport. The camera must be part of the scene tree to function.

### get_camera_rid {#get_camera_rid}

```gdscript
RID get_camera_rid() const
```

Returns the camera's RID from the RenderingServer.

### get_camera_transform {#get_camera_transform}

```gdscript
Transform3D get_camera_transform() const
```

Returns the transform of the camera plus the vertical (v_offset) and horizontal (h_offset) offsets; and any other adjustments made to the position and orientation of the camera by subclassed cameras such as XRCamera3D.

### get_compositor {#get_compositor}

```gdscript
Compositor get_compositor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cull_mask {#get_cull_mask}

```gdscript
int get_cull_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_cull_mask_value {#get_cull_mask_value}

```gdscript
bool get_cull_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the cull_mask is enabled, given a layer_number between 1 and 20.

### get_doppler_tracking {#get_doppler_tracking}

```gdscript
Camera3D.DopplerTracking get_doppler_tracking() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_environment {#get_environment}

```gdscript
Environment get_environment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_far {#get_far}

```gdscript
float get_far() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fov {#get_fov}

```gdscript
float get_fov() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_frustum {#get_frustum}

```gdscript
typedarray::Plane get_frustum() const
```

Returns the camera's frustum planes in world space units as an array of Planes in the following order: near, far, left, top, right, bottom. Not to be confused with frustum_offset.

### get_frustum_offset {#get_frustum_offset}

```gdscript
Vector2 get_frustum_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_h_offset {#get_h_offset}

```gdscript
float get_h_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_keep_aspect_mode {#get_keep_aspect_mode}

```gdscript
Camera3D.KeepAspect get_keep_aspect_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_near {#get_near}

```gdscript
float get_near() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_projection {#get_projection}

```gdscript
Camera3D.ProjectionType get_projection() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pyramid_shape_rid {#get_pyramid_shape_rid}

```gdscript
RID get_pyramid_shape_rid()
```

Returns the RID of a pyramid shape encompassing the camera's view frustum, ignoring the camera's near plane. The tip of the pyramid represents the position of the camera.

### get_size {#get_size}

```gdscript
float get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_v_offset {#get_v_offset}

```gdscript
float get_v_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_current {#is_current}

```gdscript
bool is_current() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_position_behind {#is_position_behind}

```gdscript
bool is_position_behind(world_point: Vector3) const
```

Returns true if the given position is behind the camera (the blue part of the linked diagram). See this diagram [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/camera3d_position_frustum.png] for an overview of position query methods.

Note: A position which returns false may still be outside the camera's field of view.

### is_position_in_frustum {#is_position_in_frustum}

```gdscript
bool is_position_in_frustum(world_point: Vector3) const
```

Returns true if the given position is inside the camera's frustum (the green part of the linked diagram). See this diagram [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/camera3d_position_frustum.png] for an overview of position query methods.

### make_current {#make_current}

```gdscript
void make_current()
```

Makes this camera the current camera for the Viewport (see class description). If the camera node is outside the scene tree, it will attempt to become current once it's added.

### project_local_ray_normal {#project_local_ray_normal}

```gdscript
Vector3 project_local_ray_normal(screen_point: Vector2) const
```

Returns a normal vector from the screen point location directed along the camera. Orthogonal cameras are normalized. Perspective cameras account for perspective, screen width/height, etc.

### project_position {#project_position}

```gdscript
Vector3 project_position(screen_point: Vector2, z_depth: float) const
```

Returns the 3D point in world space that maps to the given 2D coordinate in the Viewport rectangle on a plane that is the given z_depth distance into the scene away from the camera.

### project_ray_normal {#project_ray_normal}

```gdscript
Vector3 project_ray_normal(screen_point: Vector2) const
```

Returns a normal vector in world space, that is the result of projecting a point on the Viewport rectangle by the inverse camera projection. This is useful for casting rays in the form of (origin, normal) for object intersection or picking.

### project_ray_origin {#project_ray_origin}

```gdscript
Vector3 project_ray_origin(screen_point: Vector2) const
```

Returns a 3D position in world space, that is the result of projecting a point on the Viewport rectangle by the inverse camera projection. This is useful for casting rays in the form of (origin, normal) for object intersection or picking.

### set_attributes {#set_attributes}

```gdscript
void set_attributes(env: CameraAttributes)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_compositor {#set_compositor}

```gdscript
void set_compositor(compositor: Compositor)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cull_mask {#set_cull_mask}

```gdscript
void set_cull_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cull_mask_value {#set_cull_mask_value}

```gdscript
void set_cull_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the cull_mask, given a layer_number between 1 and 20.

### set_current {#set_current}

```gdscript
void set_current(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_doppler_tracking {#set_doppler_tracking}

```gdscript
void set_doppler_tracking(mode: Camera3D.DopplerTracking)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment {#set_environment}

```gdscript
void set_environment(env: Environment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_far {#set_far}

```gdscript
void set_far(far: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fov {#set_fov}

```gdscript
void set_fov(fov: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_frustum {#set_frustum}

```gdscript
void set_frustum(size: float, offset: Vector2, z_near: float, z_far: float)
```

Sets the camera projection to frustum mode (see PROJECTION_FRUSTUM), by specifying a size, an offset, and the z_near and z_far clip planes in world space units. The size parameter represents the size of the near plane, either its width or height depending on the value of keep_aspect. See also frustum_offset.

### set_frustum_offset {#set_frustum_offset}

```gdscript
void set_frustum_offset(offset: Vector2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_h_offset {#set_h_offset}

```gdscript
void set_h_offset(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keep_aspect_mode {#set_keep_aspect_mode}

```gdscript
void set_keep_aspect_mode(mode: Camera3D.KeepAspect)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_near {#set_near}

```gdscript
void set_near(near: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_orthogonal {#set_orthogonal}

```gdscript
void set_orthogonal(size: float, z_near: float, z_far: float)
```

Sets the camera projection to orthogonal mode (see PROJECTION_ORTHOGONAL), by specifying a size, and the z_near and z_far clip planes in world space units.

As a hint, 3D games that look 2D often use this projection, with size specified in pixels.

### set_perspective {#set_perspective}

```gdscript
void set_perspective(fov: float, z_near: float, z_far: float)
```

Sets the camera projection to perspective mode (see PROJECTION_PERSPECTIVE), by specifying a fov (field of view) angle in degrees, and the z_near and z_far clip planes in world space units.

### set_projection {#set_projection}

```gdscript
void set_projection(mode: Camera3D.ProjectionType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_v_offset {#set_v_offset}

```gdscript
void set_v_offset(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### unproject_position {#unproject_position}

```gdscript
Vector2 unproject_position(world_point: Vector3) const
```

Returns the 2D coordinate in the Viewport rectangle that maps to the given 3D point in world space.

Note: When using this to position GUI elements over a 3D viewport, use is_position_behind() to prevent them from appearing if the 3D point is behind the camera:

```gdscript
# This code block is part of a script that inherits from Node3D.
# `control` is a reference to a node inheriting from Control.
control.visible = not get_viewport().get_camera_3d().is_position_behind(global_transform.origin)
control.position = get_viewport().get_camera_3d().unproject_position(global_transform.origin)
```

## 属性

### keep_aspect {#keep_aspect}

```gdscript
keep_aspect: int
```

- void set_keep_aspect_mode(value: KeepAspect)
- KeepAspect get_keep_aspect_mode()

The axis to lock during fov/size adjustments. Can be either KEEP_WIDTH or KEEP_HEIGHT.

### cull_mask {#cull_mask}

```gdscript
cull_mask: int
```

- void set_cull_mask(value: int)
- int get_cull_mask()

The culling mask that describes which VisualInstance3D.layers are rendered by this camera. By default, all 20 user-visible layers are rendered.

Note: Since the cull_mask allows for 32 layers to be stored in total, there are an additional 12 layers that are only used internally by the engine and aren't exposed in the editor. Setting cull_mask using a script allows you to toggle those reserved layers, which can be useful for editor plugins.

To adjust cull_mask more easily using a script, use get_cull_mask_value() and set_cull_mask_value().

Note: VoxelGI, SDFGI and LightmapGI will always take all layers into account to determine what contributes to global illumination. If this is an issue, set GeometryInstance3D.gi_mode to GeometryInstance3D.GI_MODE_DISABLED for meshes and Light3D.light_bake_mode to Light3D.BAKE_DISABLED for lights to exclude them from global illumination.

### environment {#environment}

```gdscript
environment: Environment
```

- void set_environment(value: Environment)
- Environment get_environment()

The Environment to use for this camera.

### attributes {#attributes}

```gdscript
attributes: CameraAttributesPractical,CameraAttributesPhysical
```

- void set_attributes(value: CameraAttributes)
- CameraAttributes get_attributes()

The CameraAttributes to use for this camera.

### compositor {#compositor}

```gdscript
compositor: Compositor
```

- void set_compositor(value: Compositor)
- Compositor get_compositor()

The Compositor to use for this camera.

### h_offset {#h_offset}

```gdscript
h_offset: float
```

- void set_h_offset(value: float)
- float get_h_offset()

The horizontal (X) offset of the camera viewport.

### v_offset {#v_offset}

```gdscript
v_offset: float
```

- void set_v_offset(value: float)
- float get_v_offset()

The vertical (Y) offset of the camera viewport.

### doppler_tracking {#doppler_tracking}

```gdscript
doppler_tracking: int
```

- void set_doppler_tracking(value: DopplerTracking)
- DopplerTracking get_doppler_tracking()

If not DOPPLER_TRACKING_DISABLED, this camera will simulate the Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] for objects changed in particular _process methods.

Note: The Doppler effect will only be heard on AudioStreamPlayer3Ds if AudioStreamPlayer3D.doppler_tracking is not set to AudioStreamPlayer3D.DOPPLER_TRACKING_DISABLED.

### projection {#projection}

```gdscript
projection: int
```

- void set_projection(value: ProjectionType)
- ProjectionType get_projection()

The camera's projection mode. In PROJECTION_PERSPECTIVE mode, objects' Z distance from the camera's local space scales their perceived size.

### current {#current}

```gdscript
current: bool
```

- void set_current(value: bool)
- bool is_current()

If true, the ancestor Viewport is currently using this camera.

If multiple cameras are in the scene, one will always be made current. For example, if two Camera3D nodes are present in the scene and only one is current, setting one camera's current to false will cause the other camera to be made current.

### fov {#fov}

```gdscript
fov: float
```

- void set_fov(value: float)
- float get_fov()

The camera's field of view angle (in degrees). Only applicable in perspective mode. Since keep_aspect locks one axis, fov sets the other axis' field of view angle.

For reference, the default vertical field of view value (75.0) is equivalent to a horizontal FOV of:

- ~91.31 degrees in a 4:3 viewport
- ~101.67 degrees in a 16:10 viewport
- ~107.51 degrees in a 16:9 viewport
- ~121.63 degrees in a 21:9 viewport

### size {#size}

```gdscript
size: float
```

- void set_size(value: float)
- float get_size()

The camera's size in meters measured as the diameter of the width or height, depending on keep_aspect. Only applicable in orthogonal and frustum modes.

### frustum_offset {#frustum_offset}

```gdscript
frustum_offset: Vector2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### near {#near}

```gdscript
near: float
```

- void set_near(value: float)
- float get_near()

The distance to the near culling boundary for this camera relative to its local Z axis. Lower values allow the camera to see objects more up close to its origin, at the cost of lower precision across the entire range. Values lower than the default can lead to increased Z-fighting.

### far {#far}

```gdscript
far: float
```

- void set_far(value: float)
- float get_far()

The distance to the far culling boundary for this camera relative to its local Z axis. Higher values allow the camera to see further away, while decreasing far can improve performance if it results in objects being partially or fully culled.

## 枚举

### ProjectionType {#enum-ProjectionType}

```gdscript
enum ProjectionType { PROJECTION_PERSPECTIVE = 0, PROJECTION_ORTHOGONAL = 1, PROJECTION_FRUSTUM = 2 }
```

ProjectionType PROJECTION_PERSPECTIVE = 0

Perspective projection. Objects on the screen becomes smaller when they are far away.

ProjectionType PROJECTION_ORTHOGONAL = 1

Orthogonal projection, also known as orthographic projection. Objects remain the same size on the screen no matter how far away they are.

ProjectionType PROJECTION_FRUSTUM = 2

Frustum projection. This mode allows adjusting frustum_offset to create "tilted frustum" effects.

#### PROJECTION_PERSPECTIVE {#PROJECTION_PERSPECTIVE}

```gdscript
PROJECTION_PERSPECTIVE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROJECTION_ORTHOGONAL {#PROJECTION_ORTHOGONAL}

```gdscript
PROJECTION_ORTHOGONAL = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROJECTION_FRUSTUM {#PROJECTION_FRUSTUM}

```gdscript
PROJECTION_FRUSTUM = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### KeepAspect {#enum-KeepAspect}

```gdscript
enum KeepAspect { KEEP_WIDTH = 0, KEEP_HEIGHT = 1 }
```

KeepAspect KEEP_WIDTH = 0

Preserves the horizontal aspect ratio; also known as Vert- scaling. This is usually the best option for projects running in portrait mode, as taller aspect ratios will benefit from a wider vertical FOV.

KeepAspect KEEP_HEIGHT = 1

Preserves the vertical aspect ratio; also known as Hor+ scaling. This is usually the best option for projects running in landscape mode, as wider aspect ratios will automatically benefit from a wider horizontal FOV.

#### KEEP_WIDTH {#KEEP_WIDTH}

```gdscript
KEEP_WIDTH = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KEEP_HEIGHT {#KEEP_HEIGHT}

```gdscript
KEEP_HEIGHT = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DopplerTracking {#enum-DopplerTracking}

```gdscript
enum DopplerTracking { DOPPLER_TRACKING_DISABLED = 0, DOPPLER_TRACKING_IDLE_STEP = 1, DOPPLER_TRACKING_PHYSICS_STEP = 2 }
```

DopplerTracking DOPPLER_TRACKING_DISABLED = 0

Disables Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] simulation (default).

DopplerTracking DOPPLER_TRACKING_IDLE_STEP = 1

Simulate Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] by tracking positions of objects that are changed in _process. Changes in the relative velocity of this camera compared to those objects affect how audio is perceived (changing the audio's AudioStreamPlayer3D.pitch_scale).

DopplerTracking DOPPLER_TRACKING_PHYSICS_STEP = 2

Simulate Doppler effect [https://en.wikipedia.org/wiki/Doppler_effect] by tracking positions of objects that are changed in _physics_process. Changes in the relative velocity of this camera compared to those objects affect how audio is perceived (changing the audio's AudioStreamPlayer3D.pitch_scale).

#### DOPPLER_TRACKING_DISABLED {#DOPPLER_TRACKING_DISABLED}

```gdscript
DOPPLER_TRACKING_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOPPLER_TRACKING_IDLE_STEP {#DOPPLER_TRACKING_IDLE_STEP}

```gdscript
DOPPLER_TRACKING_IDLE_STEP = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOPPLER_TRACKING_PHYSICS_STEP {#DOPPLER_TRACKING_PHYSICS_STEP}

```gdscript
DOPPLER_TRACKING_PHYSICS_STEP = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
