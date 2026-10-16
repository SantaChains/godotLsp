# XRInterface

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **XRInterface**

> This class needs to be implemented to make an AR or VR platform available to Godot and these should be implemented as C++ modules or GDExtension modules. Part of the interface is exposed to GDScript so you can detect, enable and configure an AR or VR platform.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class needs to be implemented to make an AR or VR platform available to Godot and these should be implemented as C++ modules or GDExtension modules. Part of the interface is exposed to GDScript so you can detect, enable and configure an AR or VR platform.

Interfaces should be written in such a way that simply enabling them will give us a working setup. You can query the available interfaces through XRServer.

</details>

## 方法

### get_anchor_detection_is_enabled {#get_anchor_detection_is_enabled}

```gdscript
bool get_anchor_detection_is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_camera_feed_id {#get_camera_feed_id}

```gdscript
int get_camera_feed_id()
```

If this is an AR interface that requires displaying a camera feed as the background, this method returns the feed ID in the CameraServer for this interface.

### get_capabilities {#get_capabilities}

```gdscript
int get_capabilities() const
```

Returns a combination of Capabilities flags providing information about the capabilities of this interface.

### get_environment_blend_mode {#get_environment_blend_mode}

```gdscript
XRInterface.EnvironmentBlendMode get_environment_blend_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_name {#get_name}

```gdscript
StringName get_name() const
```

Returns the name of this interface ("OpenXR", "OpenVR", "OpenHMD", "ARKit", etc.).

### get_play_area {#get_play_area}

```gdscript
PackedVector3Array get_play_area() const
```

Returns an array of vectors that represent the physical play area mapped to the virtual space around the XROrigin3D point. The points form a convex polygon that can be used to react to or visualize the play area. This returns an empty array if this feature is not supported or if the information is not yet available.

### get_play_area_mode {#get_play_area_mode}

```gdscript
XRInterface.PlayAreaMode get_play_area_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_projection_for_view {#get_projection_for_view}

```gdscript
Projection get_projection_for_view(view: int, aspect: float, near: float, far: float)
```

Returns the projection matrix for a view/eye.

### get_render_target_size {#get_render_target_size}

```gdscript
Vector2 get_render_target_size()
```

Returns the resolution at which we should render our intermediate results before things like lens distortion are applied by the VR platform.

### get_supported_environment_blend_modes {#get_supported_environment_blend_modes}

```gdscript
Array get_supported_environment_blend_modes()
```

Returns the an array of supported environment blend modes, see EnvironmentBlendMode.

### get_system_info {#get_system_info}

```gdscript
Dictionary get_system_info()
```

Returns a Dictionary with extra system info. Interfaces are expected to return XRRuntimeName and XRRuntimeVersion providing info about the used XR runtime. Additional entries may be provided specific to an interface.

Note:This information may only be available after initialize() was successfully called.

### get_tracking_status {#get_tracking_status}

```gdscript
XRInterface.TrackingStatus get_tracking_status() const
```

If supported, returns the status of our tracking. This will allow you to provide feedback to the user whether there are issues with positional tracking.

### get_transform_for_view {#get_transform_for_view}

```gdscript
Transform3D get_transform_for_view(view: int, cam_transform: Transform3D)
```

Returns the transform for a view/eye.

view is the view/eye index.

cam_transform is the transform that maps device coordinates to scene coordinates, typically the Node3D.global_transform of the current XROrigin3D.

### get_view_count {#get_view_count}

```gdscript
int get_view_count()
```

Returns the number of views that need to be rendered for this device. 1 for Monoscopic, 2 for Stereoscopic.

### initialize {#initialize}

```gdscript
bool initialize()
```

Call this to initialize this interface. The first interface that is initialized is identified as the primary interface and it will be used for rendering output.

After initializing the interface you want to use you then need to enable the AR/VR mode of a viewport and rendering should commence.

Note: You must enable the XR mode on the main viewport for any device that uses the main output of Godot, such as for mobile VR.

If you do this for a platform that handles its own output (such as OpenVR) Godot will show just one eye without distortion on screen. Alternatively, you can add a separate viewport node to your scene and enable AR/VR on that viewport. It will be used to output to the HMD, leaving you free to do anything you like in the main window, such as using a separate camera as a spectator camera or rendering something completely different.

While currently not used, you can activate additional interfaces. You may wish to do this if you want to track controllers from other platforms. However, at this point in time only one interface can render to an HMD.

### is_initialized {#is_initialized}

```gdscript
bool is_initialized() const
```

Returns true if this interface has been initialized.

### is_passthrough_enabled {#is_passthrough_enabled}

```gdscript
bool is_passthrough_enabled()
```

Deprecated: Check if environment_blend_mode is XR_ENV_BLEND_MODE_ALPHA_BLEND, instead.

Returns true if passthrough is enabled.

### is_passthrough_supported {#is_passthrough_supported}

```gdscript
bool is_passthrough_supported()
```

Deprecated: Check that XR_ENV_BLEND_MODE_ALPHA_BLEND is supported using get_supported_environment_blend_modes(), instead.

Returns true if this interface supports passthrough.

### is_primary {#is_primary}

```gdscript
bool is_primary()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_anchor_detection_is_enabled {#set_anchor_detection_is_enabled}

```gdscript
void set_anchor_detection_is_enabled(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment_blend_mode {#set_environment_blend_mode}

```gdscript
bool set_environment_blend_mode(mode: XRInterface.EnvironmentBlendMode)
```

Sets the active environment blend mode.

mode is the environment blend mode starting with the next frame.

Note: Not all runtimes support all environment blend modes, so it is important to check this at startup. For example:

```gdscript
func _ready():
    var xr_interface = XRServer.find_interface("OpenXR")
    if xr_interface and xr_interface.is_initialized():
        var vp = get_viewport()
        vp.use_xr = true
        var acceptable_modes = [XRInterface.XR_ENV_BLEND_MODE_OPAQUE, XRInterface.XR_ENV_BLEND_MODE_ADDITIVE]
        var modes = xr_interface.get_supported_environment_blend_modes()
        for mode in acceptable_modes:
            if mode in modes:
                xr_interface.set_environment_blend_mode(mode)
                break
```

### set_play_area_mode {#set_play_area_mode}

```gdscript
bool set_play_area_mode(mode: XRInterface.PlayAreaMode)
```

Sets the active play area mode, will return false if the mode can't be used with this interface.

Note: Changing this after the interface has already been initialized can be jarring for the player, so it's recommended to recenter on the HMD with XRServer.center_on_hmd() (if switching to XR_PLAY_AREA_STAGE) or make the switch during a scene change.

### set_primary {#set_primary}

```gdscript
void set_primary(primary: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### start_passthrough {#start_passthrough}

```gdscript
bool start_passthrough()
```

Deprecated: Set the environment_blend_mode to XR_ENV_BLEND_MODE_ALPHA_BLEND, instead.

Starts passthrough, will return false if passthrough couldn't be started.

Note: The viewport used for XR must have a transparent background, otherwise passthrough may not properly render.

### stop_passthrough {#stop_passthrough}

```gdscript
void stop_passthrough()
```

Deprecated: Set the environment_blend_mode to XR_ENV_BLEND_MODE_OPAQUE, instead.

Stops passthrough.

### supports_play_area_mode {#supports_play_area_mode}

```gdscript
bool supports_play_area_mode(mode: XRInterface.PlayAreaMode)
```

Call this to find out if a given play area mode is supported by this interface.

### trigger_haptic_pulse {#trigger_haptic_pulse}

```gdscript
void trigger_haptic_pulse(action_name: String, tracker_name: StringName, frequency: float, amplitude: float, duration_sec: float, delay_sec: float)
```

Triggers a haptic pulse on a device associated with this interface.

action_name is the name of the action for this pulse.

tracker_name is optional and can be used to direct the pulse to a specific device provided that device is bound to this haptic.

frequency is the frequency of the pulse, set to 0.0 to have the system use a default frequency.

amplitude is the amplitude of the pulse between 0.0 and 1.0.

duration_sec is the duration of the pulse in seconds.

delay_sec is a delay in seconds before the pulse is given.

### uninitialize {#uninitialize}

```gdscript
void uninitialize()
```

Turns the interface off.

## 属性

### interface_is_primary {#interface_is_primary}

```gdscript
interface_is_primary: bool
```

- void set_primary(value: bool)
- bool is_primary()

true if this is the primary interface.

### xr_play_area_mode {#xr_play_area_mode}

```gdscript
xr_play_area_mode: int
```

- bool set_play_area_mode(mode: PlayAreaMode)
- PlayAreaMode get_play_area_mode()

The play area mode for this interface.

### environment_blend_mode {#environment_blend_mode}

```gdscript
environment_blend_mode: int
```

- bool set_environment_blend_mode(mode: EnvironmentBlendMode)
- EnvironmentBlendMode get_environment_blend_mode()

Specify how XR should blend in the environment. This is specific to certain AR and passthrough devices where camera images are blended in by the XR compositor.

### ar_is_anchor_detection_enabled {#ar_is_anchor_detection_enabled}

```gdscript
ar_is_anchor_detection_enabled: bool
```

- void set_anchor_detection_is_enabled(value: bool)
- bool get_anchor_detection_is_enabled()

On an AR interface, true if anchor detection is enabled.

## 信号

### play_area_changed {#play_area_changed}

```gdscript
signal play_area_changed(mode: int)
```

Emitted when the play area is changed. This can be a result of the player resetting the boundary or entering a new play area, the player changing the play area mode, the world scale changing or the player resetting their headset orientation.

## 枚举

### Capabilities {#enum-Capabilities}

```gdscript
enum Capabilities { XR_NONE = 0, XR_MONO = 1, XR_STEREO = 2, XR_QUAD = 4, XR_VR = 8, XR_AR = 16, XR_EXTERNAL = 32 }
```

Capabilities XR_NONE = 0

No XR capabilities.

Capabilities XR_MONO = 1

This interface can work with normal rendering output (non-HMD based AR).

Capabilities XR_STEREO = 2

This interface supports stereoscopic rendering.

Capabilities XR_QUAD = 4

This interface supports quad rendering (not yet supported by Godot).

Capabilities XR_VR = 8

This interface supports VR.

Capabilities XR_AR = 16

This interface supports AR (video background and real world tracking).

Capabilities XR_EXTERNAL = 32

This interface outputs to an external device. If the main viewport is used, the on screen output is an unmodified buffer of either the left or right eye (stretched if the viewport size is not changed to the same aspect ratio of get_render_target_size()). Using a separate viewport node frees up the main viewport for other purposes.

#### XR_NONE {#XR_NONE}

```gdscript
XR_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_MONO {#XR_MONO}

```gdscript
XR_MONO = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_STEREO {#XR_STEREO}

```gdscript
XR_STEREO = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_QUAD {#XR_QUAD}

```gdscript
XR_QUAD = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_VR {#XR_VR}

```gdscript
XR_VR = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_AR {#XR_AR}

```gdscript
XR_AR = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_EXTERNAL {#XR_EXTERNAL}

```gdscript
XR_EXTERNAL = 32
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### TrackingStatus {#enum-TrackingStatus}

```gdscript
enum TrackingStatus { XR_NORMAL_TRACKING = 0, XR_EXCESSIVE_MOTION = 1, XR_INSUFFICIENT_FEATURES = 2, XR_UNKNOWN_TRACKING = 3, XR_NOT_TRACKING = 4 }
```

TrackingStatus XR_NORMAL_TRACKING = 0

Tracking is behaving as expected.

TrackingStatus XR_EXCESSIVE_MOTION = 1

Tracking is hindered by excessive motion (the player is moving faster than tracking can keep up).

TrackingStatus XR_INSUFFICIENT_FEATURES = 2

Tracking is hindered by insufficient features, it's too dark (for camera-based tracking), player is blocked, etc.

TrackingStatus XR_UNKNOWN_TRACKING = 3

We don't know the status of the tracking or this interface does not provide feedback.

TrackingStatus XR_NOT_TRACKING = 4

Tracking is not functional (camera not plugged in or obscured, lighthouses turned off, etc.).

#### XR_NORMAL_TRACKING {#XR_NORMAL_TRACKING}

```gdscript
XR_NORMAL_TRACKING = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_EXCESSIVE_MOTION {#XR_EXCESSIVE_MOTION}

```gdscript
XR_EXCESSIVE_MOTION = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_INSUFFICIENT_FEATURES {#XR_INSUFFICIENT_FEATURES}

```gdscript
XR_INSUFFICIENT_FEATURES = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_UNKNOWN_TRACKING {#XR_UNKNOWN_TRACKING}

```gdscript
XR_UNKNOWN_TRACKING = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_NOT_TRACKING {#XR_NOT_TRACKING}

```gdscript
XR_NOT_TRACKING = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### PlayAreaMode {#enum-PlayAreaMode}

```gdscript
enum PlayAreaMode { XR_PLAY_AREA_UNKNOWN = 0, XR_PLAY_AREA_3DOF = 1, XR_PLAY_AREA_SITTING = 2, XR_PLAY_AREA_ROOMSCALE = 3, XR_PLAY_AREA_STAGE = 4, XR_PLAY_AREA_CUSTOM = 2147483647 }
```

PlayAreaMode XR_PLAY_AREA_UNKNOWN = 0

Play area mode not set or not available.

PlayAreaMode XR_PLAY_AREA_3DOF = 1

Play area only supports orientation tracking, no positional tracking, area will center around player.

PlayAreaMode XR_PLAY_AREA_SITTING = 2

Player is in seated position, limited positional tracking, fixed guardian around player.

PlayAreaMode XR_PLAY_AREA_ROOMSCALE = 3

Player is free to move around, full positional tracking.

PlayAreaMode XR_PLAY_AREA_STAGE = 4

Same as XR_PLAY_AREA_ROOMSCALE but origin point is fixed to the center of the physical space. In this mode, system-level recentering may be disabled, requiring the use of XRServer.center_on_hmd().

PlayAreaMode XR_PLAY_AREA_CUSTOM = 2147483647

Custom play area set by a GDExtension.

#### XR_PLAY_AREA_UNKNOWN {#XR_PLAY_AREA_UNKNOWN}

```gdscript
XR_PLAY_AREA_UNKNOWN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_PLAY_AREA_3DOF {#XR_PLAY_AREA_3DOF}

```gdscript
XR_PLAY_AREA_3DOF = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_PLAY_AREA_SITTING {#XR_PLAY_AREA_SITTING}

```gdscript
XR_PLAY_AREA_SITTING = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_PLAY_AREA_ROOMSCALE {#XR_PLAY_AREA_ROOMSCALE}

```gdscript
XR_PLAY_AREA_ROOMSCALE = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_PLAY_AREA_STAGE {#XR_PLAY_AREA_STAGE}

```gdscript
XR_PLAY_AREA_STAGE = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_PLAY_AREA_CUSTOM {#XR_PLAY_AREA_CUSTOM}

```gdscript
XR_PLAY_AREA_CUSTOM = 2147483647
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### EnvironmentBlendMode {#enum-EnvironmentBlendMode}

```gdscript
enum EnvironmentBlendMode { XR_ENV_BLEND_MODE_OPAQUE = 0, XR_ENV_BLEND_MODE_ADDITIVE = 1, XR_ENV_BLEND_MODE_ALPHA_BLEND = 2 }
```

EnvironmentBlendMode XR_ENV_BLEND_MODE_OPAQUE = 0

Opaque blend mode. This is typically used for VR devices.

EnvironmentBlendMode XR_ENV_BLEND_MODE_ADDITIVE = 1

Additive blend mode. This is typically used for AR devices or VR devices with passthrough.

EnvironmentBlendMode XR_ENV_BLEND_MODE_ALPHA_BLEND = 2

Alpha blend mode. This is typically used for AR or VR devices with passthrough capabilities. The alpha channel controls how much of the passthrough is visible. Alpha of 0.0 means the passthrough is visible and this pixel works in ADDITIVE mode. Alpha of 1.0 means that the passthrough is not visible and this pixel works in OPAQUE mode.

#### XR_ENV_BLEND_MODE_OPAQUE {#XR_ENV_BLEND_MODE_OPAQUE}

```gdscript
XR_ENV_BLEND_MODE_OPAQUE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_ENV_BLEND_MODE_ADDITIVE {#XR_ENV_BLEND_MODE_ADDITIVE}

```gdscript
XR_ENV_BLEND_MODE_ADDITIVE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_ENV_BLEND_MODE_ALPHA_BLEND {#XR_ENV_BLEND_MODE_ALPHA_BLEND}

```gdscript
XR_ENV_BLEND_MODE_ALPHA_BLEND = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### VRSTextureFormat {#enum-VRSTextureFormat}

```gdscript
enum VRSTextureFormat { XR_VRS_TEXTURE_FORMAT_UNIFIED = 0, XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE = 1, XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP = 2 }
```

VRSTextureFormat XR_VRS_TEXTURE_FORMAT_UNIFIED = 0

The texture format is the same as returned by XRVRS.make_vrs_texture().

VRSTextureFormat XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE = 1

The texture format is the same as expected by the Vulkan VK_KHR_fragment_shading_rate extension.

VRSTextureFormat XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP = 2

The texture format is the same as expected by the Vulkan VK_EXT_fragment_density_map extension.

#### XR_VRS_TEXTURE_FORMAT_UNIFIED {#XR_VRS_TEXTURE_FORMAT_UNIFIED}

```gdscript
XR_VRS_TEXTURE_FORMAT_UNIFIED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE {#XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE}

```gdscript
XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP {#XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP}

```gdscript
XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
