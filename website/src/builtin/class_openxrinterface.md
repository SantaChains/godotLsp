# OpenXRInterface

**继承**：[XRInterface](class_xrinterface.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRInterface**

> The OpenXR interface allows Godot to interact with OpenXR runtimes and make it possible to create XR experiences and games.

<details><summary>官方英文描述（中文精译进行中）</summary>

The OpenXR interface allows Godot to interact with OpenXR runtimes and make it possible to create XR experiences and games.

Due to the needs of OpenXR this interface works slightly different than other plugin based XR interfaces. It needs to be initialized when Godot starts. You need to enable OpenXR, settings for this can be found in your games project settings under the XR heading. You do need to mark a viewport for use with XR in order for Godot to know which render result should be output to the headset.

</details>

## 方法

### get_action_sets {#get_action_sets}

```gdscript
Array get_action_sets() const
```

Returns a list of action sets registered with Godot (loaded from the action map at runtime).

### get_available_display_refresh_rates {#get_available_display_refresh_rates}

```gdscript
Array get_available_display_refresh_rates() const
```

Returns a list of display refresh rates supported by the current HMD. Only returned if this feature is supported by the OpenXR runtime and after the interface has been initialized.

### get_display_refresh_rate {#get_display_refresh_rate}

```gdscript
float get_display_refresh_rate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_foveation_dynamic {#get_foveation_dynamic}

```gdscript
bool get_foveation_dynamic() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_foveation_level {#get_foveation_level}

```gdscript
int get_foveation_level() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_foveation_with_subsampled_images {#get_foveation_with_subsampled_images}

```gdscript
bool get_foveation_with_subsampled_images() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hand_joint_angular_velocity {#get_hand_joint_angular_velocity}

```gdscript
Vector3 get_hand_joint_angular_velocity(hand: OpenXRInterface.Hand, joint: OpenXRInterface.HandJoints) const
```

Deprecated: Use XRHandTracker.get_hand_joint_angular_velocity() obtained from XRServer.get_tracker() instead.

If handtracking is enabled, returns the angular velocity of a joint (joint) of a hand (hand) as provided by OpenXR. This is relative to XROrigin3D!

### get_hand_joint_flags {#get_hand_joint_flags}

```gdscript
bitfield::OpenXRInterface.HandJointFlags get_hand_joint_flags(hand: OpenXRInterface.Hand, joint: OpenXRInterface.HandJoints) const
```

Deprecated: Use XRHandTracker.get_hand_joint_flags() obtained from XRServer.get_tracker() instead.

If handtracking is enabled, returns flags that inform us of the validity of the tracking data.

### get_hand_joint_linear_velocity {#get_hand_joint_linear_velocity}

```gdscript
Vector3 get_hand_joint_linear_velocity(hand: OpenXRInterface.Hand, joint: OpenXRInterface.HandJoints) const
```

Deprecated: Use XRHandTracker.get_hand_joint_linear_velocity() obtained from XRServer.get_tracker() instead.

If handtracking is enabled, returns the linear velocity of a joint (joint) of a hand (hand) as provided by OpenXR. This is relative to XROrigin3D without worldscale applied!

### get_hand_joint_position {#get_hand_joint_position}

```gdscript
Vector3 get_hand_joint_position(hand: OpenXRInterface.Hand, joint: OpenXRInterface.HandJoints) const
```

Deprecated: Use XRHandTracker.get_hand_joint_transform() obtained from XRServer.get_tracker() instead.

If handtracking is enabled, returns the position of a joint (joint) of a hand (hand) as provided by OpenXR. This is relative to XROrigin3D without worldscale applied!

### get_hand_joint_radius {#get_hand_joint_radius}

```gdscript
float get_hand_joint_radius(hand: OpenXRInterface.Hand, joint: OpenXRInterface.HandJoints) const
```

Deprecated: Use XRHandTracker.get_hand_joint_radius() obtained from XRServer.get_tracker() instead.

If handtracking is enabled, returns the radius of a joint (joint) of a hand (hand) as provided by OpenXR. This is without worldscale applied!

### get_hand_joint_rotation {#get_hand_joint_rotation}

```gdscript
Quaternion get_hand_joint_rotation(hand: OpenXRInterface.Hand, joint: OpenXRInterface.HandJoints) const
```

Deprecated: Use XRHandTracker.get_hand_joint_transform() obtained from XRServer.get_tracker() instead.

If handtracking is enabled, returns the rotation of a joint (joint) of a hand (hand) as provided by OpenXR.

### get_hand_tracking_source {#get_hand_tracking_source}

```gdscript
OpenXRInterface.HandTrackedSource get_hand_tracking_source(hand: OpenXRInterface.Hand) const
```

Deprecated: Use XRHandTracker.hand_tracking_source obtained from XRServer.get_tracker() instead.

If handtracking is enabled and hand tracking source is supported, gets the source of the hand tracking data for hand.

### get_motion_range {#get_motion_range}

```gdscript
OpenXRInterface.HandMotionRange get_motion_range(hand: OpenXRInterface.Hand) const
```

If handtracking is enabled and motion range is supported, gets the currently configured motion range for hand.

### get_render_target_size_multiplier {#get_render_target_size_multiplier}

```gdscript
float get_render_target_size_multiplier() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_session_state {#get_session_state}

```gdscript
OpenXRInterface.SessionState get_session_state()
```

Returns the current state of our OpenXR session.

### get_vrs_min_radius {#get_vrs_min_radius}

```gdscript
float get_vrs_min_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vrs_strength {#get_vrs_strength}

```gdscript
float get_vrs_strength() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_action_set_active {#is_action_set_active}

```gdscript
bool is_action_set_active(name: String) const
```

Returns true if the given action set is active.

### is_eye_gaze_interaction_supported {#is_eye_gaze_interaction_supported}

```gdscript
bool is_eye_gaze_interaction_supported()
```

Returns the capabilities of the eye gaze interaction extension.

Note: This only returns a valid value after OpenXR has been initialized.

### is_foveation_supported {#is_foveation_supported}

```gdscript
bool is_foveation_supported() const
```

Returns true if OpenXR's foveation extension is supported. The interface must be initialized before this returns a valid value.

Note: When using the Vulkan rendering driver, Viewport.vrs_mode must be set to Viewport.VRS_XR to support foveation.

### is_hand_interaction_supported {#is_hand_interaction_supported}

```gdscript
bool is_hand_interaction_supported() const
```

Returns true if OpenXR's hand interaction profile is supported and enabled.

Note: This only returns a valid value after OpenXR has been initialized.

### is_hand_tracking_supported {#is_hand_tracking_supported}

```gdscript
bool is_hand_tracking_supported()
```

Returns true if OpenXR's hand tracking is supported and enabled.

Note: This only returns a valid value after OpenXR has been initialized.

### is_user_presence_supported {#is_user_presence_supported}

```gdscript
bool is_user_presence_supported() const
```

Returns true if OpenXR's user presence extension is supported and enabled.

Note: This only returns a valid value after OpenXR has been initialized.

### is_user_present {#is_user_present}

```gdscript
bool is_user_present() const
```

Returns true if system has detected the presence of a user in the XR experience.

### set_action_set_active {#set_action_set_active}

```gdscript
void set_action_set_active(name: String, active: bool)
```

Sets the given action set as active or inactive.

### set_cpu_level {#set_cpu_level}

```gdscript
void set_cpu_level(level: OpenXRInterface.PerfSettingsLevel)
```

Sets the CPU performance level of the OpenXR device.

### set_display_refresh_rate {#set_display_refresh_rate}

```gdscript
void set_display_refresh_rate(refresh_rate: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_foveation_dynamic {#set_foveation_dynamic}

```gdscript
void set_foveation_dynamic(foveation_dynamic: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_foveation_level {#set_foveation_level}

```gdscript
void set_foveation_level(foveation_level: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_foveation_with_subsampled_images {#set_foveation_with_subsampled_images}

```gdscript
void set_foveation_with_subsampled_images(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gpu_level {#set_gpu_level}

```gdscript
void set_gpu_level(level: OpenXRInterface.PerfSettingsLevel)
```

Sets the GPU performance level of the OpenXR device.

### set_motion_range {#set_motion_range}

```gdscript
void set_motion_range(hand: OpenXRInterface.Hand, motion_range: OpenXRInterface.HandMotionRange)
```

If handtracking is enabled and motion range is supported, sets the currently configured motion range for hand to motion_range.

### set_render_target_size_multiplier {#set_render_target_size_multiplier}

```gdscript
void set_render_target_size_multiplier(multiplier: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_min_radius {#set_vrs_min_radius}

```gdscript
void set_vrs_min_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_strength {#set_vrs_strength}

```gdscript
void set_vrs_strength(strength: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### display_refresh_rate {#display_refresh_rate}

```gdscript
display_refresh_rate: float
```

- void set_display_refresh_rate(value: float)
- float get_display_refresh_rate()

The display refresh rate for the current HMD. Only functional if this feature is supported by the OpenXR runtime and after the interface has been initialized.

### render_target_size_multiplier {#render_target_size_multiplier}

```gdscript
render_target_size_multiplier: float
```

- void set_render_target_size_multiplier(value: float)
- float get_render_target_size_multiplier()

The render size multiplier for the current HMD. Must be set before the interface has been initialized.

### foveation_level {#foveation_level}

```gdscript
foveation_level: int
```

- void set_foveation_level(value: int)
- int get_foveation_level()

The foveation level, from 0 (off) to 3 (high). The interface must be initialized before this is accessible.

### foveation_dynamic {#foveation_dynamic}

```gdscript
foveation_dynamic: bool
```

- void set_foveation_dynamic(value: bool)
- bool get_foveation_dynamic()

If true, enables dynamic foveation adjustment. The interface must be initialized before this is accessible. If enabled, foveation will automatically be adjusted between low and foveation_level.

### foveation_with_subsampled_images {#foveation_with_subsampled_images}

```gdscript
foveation_with_subsampled_images: bool
```

- void set_foveation_with_subsampled_images(value: bool)
- bool get_foveation_with_subsampled_images()

If true, enables subsampled images with foveation, which can provide a performance boost on Vulkan.

### vrs_min_radius {#vrs_min_radius}

```gdscript
vrs_min_radius: float
```

- void set_vrs_min_radius(value: float)
- float get_vrs_min_radius()

The minimum radius around the focal point where full quality is guaranteed if VRS is used as a percentage of screen size.

Note: Mobile and Forward+ renderers only. Requires Viewport.vrs_mode to be set to Viewport.VRS_XR.

### vrs_strength {#vrs_strength}

```gdscript
vrs_strength: float
```

- void set_vrs_strength(value: float)
- float get_vrs_strength()

The strength used to calculate the VRS density map. The greater this value, the more noticeable VRS is. This improves performance at the cost of quality.

Note: Mobile and Forward+ renderers only. Requires Viewport.vrs_mode to be set to Viewport.VRS_XR.

## 信号

### session_begun {#session_begun}

```gdscript
signal session_begun()
```

Informs our OpenXR session has been started.

### session_stopping {#session_stopping}

```gdscript
signal session_stopping()
```

Informs our OpenXR session is stopping.

### session_synchronized {#session_synchronized}

```gdscript
signal session_synchronized()
```

Informs our OpenXR session has been synchronized.

### session_focussed {#session_focussed}

```gdscript
signal session_focussed()
```

Informs our OpenXR session now has focus, for example output is sent to the HMD and we're receiving XR input.

### session_visible {#session_visible}

```gdscript
signal session_visible()
```

Informs our OpenXR session is now visible, for example output is sent to the HMD but we don't receive XR input.

### session_loss_pending {#session_loss_pending}

```gdscript
signal session_loss_pending()
```

Informs our OpenXR session is in the process of being lost.

### instance_exiting {#instance_exiting}

```gdscript
signal instance_exiting()
```

Informs our OpenXR instance is exiting.

### pose_recentered {#pose_recentered}

```gdscript
signal pose_recentered()
```

Informs the user queued a recenter of the player position.

### refresh_rate_changed {#refresh_rate_changed}

```gdscript
signal refresh_rate_changed(refresh_rate: float)
```

Informs the user the HMD refresh rate has changed.

Note: Only emitted if XR runtime supports the refresh rate extension.

### cpu_level_changed {#cpu_level_changed}

```gdscript
signal cpu_level_changed(sub_domain: int, from_level: int, to_level: int)
```

Informs the device CPU performance level has changed in the specified subdomain.

### gpu_level_changed {#gpu_level_changed}

```gdscript
signal gpu_level_changed(sub_domain: int, from_level: int, to_level: int)
```

Informs the device GPU performance level has changed in the specified subdomain.

### user_presence_changed {#user_presence_changed}

```gdscript
signal user_presence_changed(is_user_present: bool)
```

Signal emitted when the user presence value changes.

Note: This signal will not be emitted during application startup and application shutdown. Developers should assume user presence is gained on startup and lost on shutdown.

## 枚举

### SessionState {#enum-SessionState}

```gdscript
enum SessionState { SESSION_STATE_UNKNOWN = 0, SESSION_STATE_IDLE = 1, SESSION_STATE_READY = 2, SESSION_STATE_SYNCHRONIZED = 3, SESSION_STATE_VISIBLE = 4, SESSION_STATE_FOCUSED = 5, SESSION_STATE_STOPPING = 6, SESSION_STATE_LOSS_PENDING = 7, SESSION_STATE_EXITING = 8 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SESSION_STATE_UNKNOWN {#SESSION_STATE_UNKNOWN}

```gdscript
SESSION_STATE_UNKNOWN = 0
```

The state of the session is unknown, we haven't tried setting up OpenXR yet.

#### SESSION_STATE_IDLE {#SESSION_STATE_IDLE}

```gdscript
SESSION_STATE_IDLE = 1
```

The initial state after the OpenXR session is created or after the session is destroyed.

#### SESSION_STATE_READY {#SESSION_STATE_READY}

```gdscript
SESSION_STATE_READY = 2
```

OpenXR is ready to begin our session. session_begun is emitted when we change to this state.

#### SESSION_STATE_SYNCHRONIZED {#SESSION_STATE_SYNCHRONIZED}

```gdscript
SESSION_STATE_SYNCHRONIZED = 3
```

The application has synched its frame loop with the runtime but we're not rendering anything. session_synchronized is emitted when we change to this state.

#### SESSION_STATE_VISIBLE {#SESSION_STATE_VISIBLE}

```gdscript
SESSION_STATE_VISIBLE = 4
```

The application has synched its frame loop with the runtime and we're rendering output to the user, however we receive no user input. session_visible is emitted when we change to this state.

Note: This is the current state just before we get the focused state, whenever the user opens a system menu, switches to another application, or takes off their headset.

#### SESSION_STATE_FOCUSED {#SESSION_STATE_FOCUSED}

```gdscript
SESSION_STATE_FOCUSED = 5
```

The application has synched its frame loop with the runtime, we're rendering output to the user and we're receiving XR input. session_focussed is emitted when we change to this state.

Note: This is the state OpenXR will be in when the user can fully interact with your game.

#### SESSION_STATE_STOPPING {#SESSION_STATE_STOPPING}

```gdscript
SESSION_STATE_STOPPING = 6
```

Our session is being stopped. session_stopping is emitted when we change to this state.

#### SESSION_STATE_LOSS_PENDING {#SESSION_STATE_LOSS_PENDING}

```gdscript
SESSION_STATE_LOSS_PENDING = 7
```

The session is about to be lost. session_loss_pending is emitted when we change to this state.

#### SESSION_STATE_EXITING {#SESSION_STATE_EXITING}

```gdscript
SESSION_STATE_EXITING = 8
```

The OpenXR instance is about to be destroyed and we're exiting. instance_exiting is emitted when we change to this state.

---

### Hand {#enum-Hand}

```gdscript
enum Hand { HAND_LEFT = 0, HAND_RIGHT = 1, HAND_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_LEFT {#HAND_LEFT}

```gdscript
HAND_LEFT = 0
```

Left hand.

#### HAND_RIGHT {#HAND_RIGHT}

```gdscript
HAND_RIGHT = 1
```

Right hand.

#### HAND_MAX {#HAND_MAX}

```gdscript
HAND_MAX = 2
```

Maximum value for the hand enum.

---

### HandMotionRange {#enum-HandMotionRange}

```gdscript
enum HandMotionRange { HAND_MOTION_RANGE_UNOBSTRUCTED = 0, HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER = 1, HAND_MOTION_RANGE_MAX = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_MOTION_RANGE_UNOBSTRUCTED {#HAND_MOTION_RANGE_UNOBSTRUCTED}

```gdscript
HAND_MOTION_RANGE_UNOBSTRUCTED = 0
```

Full hand range, if user closes their hands, we make a full fist.

#### HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER {#HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER}

```gdscript
HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER = 1
```

Conform to controller, if user closes their hands, the tracked data conforms to the shape of the controller.

#### HAND_MOTION_RANGE_MAX {#HAND_MOTION_RANGE_MAX}

```gdscript
HAND_MOTION_RANGE_MAX = 2
```

Maximum value for the motion range enum.

---

### HandTrackedSource {#enum-HandTrackedSource}

```gdscript
enum HandTrackedSource { HAND_TRACKED_SOURCE_UNKNOWN = 0, HAND_TRACKED_SOURCE_UNOBSTRUCTED = 1, HAND_TRACKED_SOURCE_CONTROLLER = 2, HAND_TRACKED_SOURCE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_TRACKED_SOURCE_UNKNOWN {#HAND_TRACKED_SOURCE_UNKNOWN}

```gdscript
HAND_TRACKED_SOURCE_UNKNOWN = 0
```

The source of hand tracking data is unknown (the extension is likely unsupported).

#### HAND_TRACKED_SOURCE_UNOBSTRUCTED {#HAND_TRACKED_SOURCE_UNOBSTRUCTED}

```gdscript
HAND_TRACKED_SOURCE_UNOBSTRUCTED = 1
```

The source of hand tracking is unobstructed, this means that an accurate method of hand tracking is used, e.g. optical hand tracking, data gloves, etc.

#### HAND_TRACKED_SOURCE_CONTROLLER {#HAND_TRACKED_SOURCE_CONTROLLER}

```gdscript
HAND_TRACKED_SOURCE_CONTROLLER = 2
```

The source of hand tracking is a controller, bone positions are inferred from controller inputs.

#### HAND_TRACKED_SOURCE_MAX {#HAND_TRACKED_SOURCE_MAX}

```gdscript
HAND_TRACKED_SOURCE_MAX = 3
```

Represents the size of the HandTrackedSource enum.

---

### HandJoints {#enum-HandJoints}

```gdscript
enum HandJoints { HAND_JOINT_PALM = 0, HAND_JOINT_WRIST = 1, HAND_JOINT_THUMB_METACARPAL = 2, HAND_JOINT_THUMB_PROXIMAL = 3, HAND_JOINT_THUMB_DISTAL = 4, HAND_JOINT_THUMB_TIP = 5, HAND_JOINT_INDEX_METACARPAL = 6, HAND_JOINT_INDEX_PROXIMAL = 7, HAND_JOINT_INDEX_INTERMEDIATE = 8, HAND_JOINT_INDEX_DISTAL = 9, HAND_JOINT_INDEX_TIP = 10, HAND_JOINT_MIDDLE_METACARPAL = 11, HAND_JOINT_MIDDLE_PROXIMAL = 12, HAND_JOINT_MIDDLE_INTERMEDIATE = 13, HAND_JOINT_MIDDLE_DISTAL = 14, HAND_JOINT_MIDDLE_TIP = 15, HAND_JOINT_RING_METACARPAL = 16, HAND_JOINT_RING_PROXIMAL = 17, HAND_JOINT_RING_INTERMEDIATE = 18, HAND_JOINT_RING_DISTAL = 19, HAND_JOINT_RING_TIP = 20, HAND_JOINT_LITTLE_METACARPAL = 21, HAND_JOINT_LITTLE_PROXIMAL = 22, HAND_JOINT_LITTLE_INTERMEDIATE = 23, HAND_JOINT_LITTLE_DISTAL = 24, HAND_JOINT_LITTLE_TIP = 25, HAND_JOINT_MAX = 26 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_JOINT_PALM {#HAND_JOINT_PALM}

```gdscript
HAND_JOINT_PALM = 0
```

Palm joint.

#### HAND_JOINT_WRIST {#HAND_JOINT_WRIST}

```gdscript
HAND_JOINT_WRIST = 1
```

Wrist joint.

#### HAND_JOINT_THUMB_METACARPAL {#HAND_JOINT_THUMB_METACARPAL}

```gdscript
HAND_JOINT_THUMB_METACARPAL = 2
```

Thumb metacarpal joint.

#### HAND_JOINT_THUMB_PROXIMAL {#HAND_JOINT_THUMB_PROXIMAL}

```gdscript
HAND_JOINT_THUMB_PROXIMAL = 3
```

Thumb proximal joint.

#### HAND_JOINT_THUMB_DISTAL {#HAND_JOINT_THUMB_DISTAL}

```gdscript
HAND_JOINT_THUMB_DISTAL = 4
```

Thumb distal joint.

#### HAND_JOINT_THUMB_TIP {#HAND_JOINT_THUMB_TIP}

```gdscript
HAND_JOINT_THUMB_TIP = 5
```

Thumb tip joint.

#### HAND_JOINT_INDEX_METACARPAL {#HAND_JOINT_INDEX_METACARPAL}

```gdscript
HAND_JOINT_INDEX_METACARPAL = 6
```

Index finger metacarpal joint.

#### HAND_JOINT_INDEX_PROXIMAL {#HAND_JOINT_INDEX_PROXIMAL}

```gdscript
HAND_JOINT_INDEX_PROXIMAL = 7
```

Index finger phalanx proximal joint.

#### HAND_JOINT_INDEX_INTERMEDIATE {#HAND_JOINT_INDEX_INTERMEDIATE}

```gdscript
HAND_JOINT_INDEX_INTERMEDIATE = 8
```

Index finger phalanx intermediate joint.

#### HAND_JOINT_INDEX_DISTAL {#HAND_JOINT_INDEX_DISTAL}

```gdscript
HAND_JOINT_INDEX_DISTAL = 9
```

Index finger phalanx distal joint.

#### HAND_JOINT_INDEX_TIP {#HAND_JOINT_INDEX_TIP}

```gdscript
HAND_JOINT_INDEX_TIP = 10
```

Index finger tip joint.

#### HAND_JOINT_MIDDLE_METACARPAL {#HAND_JOINT_MIDDLE_METACARPAL}

```gdscript
HAND_JOINT_MIDDLE_METACARPAL = 11
```

Middle finger metacarpal joint.

#### HAND_JOINT_MIDDLE_PROXIMAL {#HAND_JOINT_MIDDLE_PROXIMAL}

```gdscript
HAND_JOINT_MIDDLE_PROXIMAL = 12
```

Middle finger phalanx proximal joint.

#### HAND_JOINT_MIDDLE_INTERMEDIATE {#HAND_JOINT_MIDDLE_INTERMEDIATE}

```gdscript
HAND_JOINT_MIDDLE_INTERMEDIATE = 13
```

Middle finger phalanx intermediate joint.

#### HAND_JOINT_MIDDLE_DISTAL {#HAND_JOINT_MIDDLE_DISTAL}

```gdscript
HAND_JOINT_MIDDLE_DISTAL = 14
```

Middle finger phalanx distal joint.

#### HAND_JOINT_MIDDLE_TIP {#HAND_JOINT_MIDDLE_TIP}

```gdscript
HAND_JOINT_MIDDLE_TIP = 15
```

Middle finger tip joint.

#### HAND_JOINT_RING_METACARPAL {#HAND_JOINT_RING_METACARPAL}

```gdscript
HAND_JOINT_RING_METACARPAL = 16
```

Ring finger metacarpal joint.

#### HAND_JOINT_RING_PROXIMAL {#HAND_JOINT_RING_PROXIMAL}

```gdscript
HAND_JOINT_RING_PROXIMAL = 17
```

Ring finger phalanx proximal joint.

#### HAND_JOINT_RING_INTERMEDIATE {#HAND_JOINT_RING_INTERMEDIATE}

```gdscript
HAND_JOINT_RING_INTERMEDIATE = 18
```

Ring finger phalanx intermediate joint.

#### HAND_JOINT_RING_DISTAL {#HAND_JOINT_RING_DISTAL}

```gdscript
HAND_JOINT_RING_DISTAL = 19
```

Ring finger phalanx distal joint.

#### HAND_JOINT_RING_TIP {#HAND_JOINT_RING_TIP}

```gdscript
HAND_JOINT_RING_TIP = 20
```

Ring finger tip joint.

#### HAND_JOINT_LITTLE_METACARPAL {#HAND_JOINT_LITTLE_METACARPAL}

```gdscript
HAND_JOINT_LITTLE_METACARPAL = 21
```

Pinky finger metacarpal joint.

#### HAND_JOINT_LITTLE_PROXIMAL {#HAND_JOINT_LITTLE_PROXIMAL}

```gdscript
HAND_JOINT_LITTLE_PROXIMAL = 22
```

Pinky finger phalanx proximal joint.

#### HAND_JOINT_LITTLE_INTERMEDIATE {#HAND_JOINT_LITTLE_INTERMEDIATE}

```gdscript
HAND_JOINT_LITTLE_INTERMEDIATE = 23
```

Pinky finger phalanx intermediate joint.

#### HAND_JOINT_LITTLE_DISTAL {#HAND_JOINT_LITTLE_DISTAL}

```gdscript
HAND_JOINT_LITTLE_DISTAL = 24
```

Pinky finger phalanx distal joint.

#### HAND_JOINT_LITTLE_TIP {#HAND_JOINT_LITTLE_TIP}

```gdscript
HAND_JOINT_LITTLE_TIP = 25
```

Pinky finger tip joint.

#### HAND_JOINT_MAX {#HAND_JOINT_MAX}

```gdscript
HAND_JOINT_MAX = 26
```

Represents the size of the HandJoints enum.

---

### PerfSettingsLevel {#enum-PerfSettingsLevel}

```gdscript
enum PerfSettingsLevel { PERF_SETTINGS_LEVEL_POWER_SAVINGS = 0, PERF_SETTINGS_LEVEL_SUSTAINED_LOW = 1, PERF_SETTINGS_LEVEL_SUSTAINED_HIGH = 2, PERF_SETTINGS_LEVEL_BOOST = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PERF_SETTINGS_LEVEL_POWER_SAVINGS {#PERF_SETTINGS_LEVEL_POWER_SAVINGS}

```gdscript
PERF_SETTINGS_LEVEL_POWER_SAVINGS = 0
```

The application has entered a non-XR section (head-locked / static screen), during which power savings are to be prioritized.

#### PERF_SETTINGS_LEVEL_SUSTAINED_LOW {#PERF_SETTINGS_LEVEL_SUSTAINED_LOW}

```gdscript
PERF_SETTINGS_LEVEL_SUSTAINED_LOW = 1
```

The application has entered a low and stable complexity section, during which reducing power is more important than occasional late rendering frames.

#### PERF_SETTINGS_LEVEL_SUSTAINED_HIGH {#PERF_SETTINGS_LEVEL_SUSTAINED_HIGH}

```gdscript
PERF_SETTINGS_LEVEL_SUSTAINED_HIGH = 2
```

The application has entered a high or dynamic complexity section, during which the XR Runtime strives for consistent XR compositing and frame rendering within a thermally sustainable range.

#### PERF_SETTINGS_LEVEL_BOOST {#PERF_SETTINGS_LEVEL_BOOST}

```gdscript
PERF_SETTINGS_LEVEL_BOOST = 3
```

The application has entered a section with very high complexity, during which the XR Runtime is allowed to step up beyond the thermally sustainable range.

---

### PerfSettingsSubDomain {#enum-PerfSettingsSubDomain}

```gdscript
enum PerfSettingsSubDomain { PERF_SETTINGS_SUB_DOMAIN_COMPOSITING = 0, PERF_SETTINGS_SUB_DOMAIN_RENDERING = 1, PERF_SETTINGS_SUB_DOMAIN_THERMAL = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PERF_SETTINGS_SUB_DOMAIN_COMPOSITING {#PERF_SETTINGS_SUB_DOMAIN_COMPOSITING}

```gdscript
PERF_SETTINGS_SUB_DOMAIN_COMPOSITING = 0
```

The compositing performance within the runtime has reached a new level.

#### PERF_SETTINGS_SUB_DOMAIN_RENDERING {#PERF_SETTINGS_SUB_DOMAIN_RENDERING}

```gdscript
PERF_SETTINGS_SUB_DOMAIN_RENDERING = 1
```

The application rendering performance has reached a new level.

#### PERF_SETTINGS_SUB_DOMAIN_THERMAL {#PERF_SETTINGS_SUB_DOMAIN_THERMAL}

```gdscript
PERF_SETTINGS_SUB_DOMAIN_THERMAL = 2
```

The temperature of the device has reached a new level.

---

### PerfSettingsNotificationLevel {#enum-PerfSettingsNotificationLevel}

```gdscript
enum PerfSettingsNotificationLevel { PERF_SETTINGS_NOTIF_LEVEL_NORMAL = 0, PERF_SETTINGS_NOTIF_LEVEL_WARNING = 1, PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PERF_SETTINGS_NOTIF_LEVEL_NORMAL {#PERF_SETTINGS_NOTIF_LEVEL_NORMAL}

```gdscript
PERF_SETTINGS_NOTIF_LEVEL_NORMAL = 0
```

The sub-domain has reached a level where no further actions other than currently applied are necessary.

#### PERF_SETTINGS_NOTIF_LEVEL_WARNING {#PERF_SETTINGS_NOTIF_LEVEL_WARNING}

```gdscript
PERF_SETTINGS_NOTIF_LEVEL_WARNING = 1
```

The sub-domain has reached an early warning level where the application should start proactive mitigation actions.

#### PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED {#PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED}

```gdscript
PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED = 2
```

The sub-domain has reached a critical level where the application should start drastic mitigation actions.

---

flags HandJointFlags:

### HandJointFlags {#enum-HandJointFlags}

```gdscript
enum HandJointFlags（位标志） { HAND_JOINT_NONE = 0, HAND_JOINT_ORIENTATION_VALID = 1, HAND_JOINT_ORIENTATION_TRACKED = 2, HAND_JOINT_POSITION_VALID = 4, HAND_JOINT_POSITION_TRACKED = 8, HAND_JOINT_LINEAR_VELOCITY_VALID = 16, HAND_JOINT_ANGULAR_VELOCITY_VALID = 32 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HAND_JOINT_NONE {#HAND_JOINT_NONE}

```gdscript
HAND_JOINT_NONE = 0
```

No flags are set.

#### HAND_JOINT_ORIENTATION_VALID {#HAND_JOINT_ORIENTATION_VALID}

```gdscript
HAND_JOINT_ORIENTATION_VALID = 1
```

If set, the orientation data is valid, otherwise, the orientation data is unreliable and should not be used.

#### HAND_JOINT_ORIENTATION_TRACKED {#HAND_JOINT_ORIENTATION_TRACKED}

```gdscript
HAND_JOINT_ORIENTATION_TRACKED = 2
```

If set, the orientation data comes from tracking data, otherwise, the orientation data contains predicted data.

#### HAND_JOINT_POSITION_VALID {#HAND_JOINT_POSITION_VALID}

```gdscript
HAND_JOINT_POSITION_VALID = 4
```

If set, the positional data is valid, otherwise, the positional data is unreliable and should not be used.

#### HAND_JOINT_POSITION_TRACKED {#HAND_JOINT_POSITION_TRACKED}

```gdscript
HAND_JOINT_POSITION_TRACKED = 8
```

If set, the positional data comes from tracking data, otherwise, the positional data contains predicted data.

#### HAND_JOINT_LINEAR_VELOCITY_VALID {#HAND_JOINT_LINEAR_VELOCITY_VALID}

```gdscript
HAND_JOINT_LINEAR_VELOCITY_VALID = 16
```

If set, our linear velocity data is valid, otherwise, the linear velocity data is unreliable and should not be used.

#### HAND_JOINT_ANGULAR_VELOCITY_VALID {#HAND_JOINT_ANGULAR_VELOCITY_VALID}

```gdscript
HAND_JOINT_ANGULAR_VELOCITY_VALID = 32
```

If set, our angular velocity data is valid, otherwise, the angular velocity data is unreliable and should not be used.

---
