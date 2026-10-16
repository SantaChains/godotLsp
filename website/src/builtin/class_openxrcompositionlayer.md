# OpenXRCompositionLayer

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **OpenXRCompositionLayer**

> Composition layers allow 2D viewports to be displayed inside of the headset by the XR compositor through special projections that retain their quality. This allows for rendering clear text while keeping the layer at a native resolution.

<details><summary>官方英文描述</summary>

Composition layers allow 2D viewports to be displayed inside of the headset by the XR compositor through special projections that retain their quality. This allows for rendering clear text while keeping the layer at a native resolution.

Note: If the OpenXR runtime doesn't support the given composition layer type, a fallback mesh can be generated with a ViewportTexture, in order to emulate the composition layer.

</details>

## 方法

### get_alpha_blend {#get_alpha_blend}

```gdscript
bool get_alpha_blend() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_alpha_swizzle {#get_alpha_swizzle}

```gdscript
OpenXRCompositionLayer.Swizzle get_alpha_swizzle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_android_surface {#get_android_surface}

```gdscript
JavaObject get_android_surface()
```

Returns a JavaObject representing an android.view.Surface if use_android_surface is enabled and OpenXR has created the surface. Otherwise, this will return null.

Note: The surface can only be created during an active OpenXR session. So, if use_android_surface is enabled outside of an OpenXR session, it won't be created until a new session fully starts.

### get_android_surface_size {#get_android_surface_size}

```gdscript
Vector2i get_android_surface_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_blue_swizzle {#get_blue_swizzle}

```gdscript
OpenXRCompositionLayer.Swizzle get_blue_swizzle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_border_color {#get_border_color}

```gdscript
Color get_border_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_hole_punch {#get_enable_hole_punch}

```gdscript
bool get_enable_hole_punch() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_eye_visibility {#get_eye_visibility}

```gdscript
OpenXRCompositionLayer.EyeVisibility get_eye_visibility() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_green_swizzle {#get_green_swizzle}

```gdscript
OpenXRCompositionLayer.Swizzle get_green_swizzle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_horizontal_wrap {#get_horizontal_wrap}

```gdscript
OpenXRCompositionLayer.Wrap get_horizontal_wrap() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_layer_viewport {#get_layer_viewport}

```gdscript
SubViewport get_layer_viewport() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mag_filter {#get_mag_filter}

```gdscript
OpenXRCompositionLayer.Filter get_mag_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_anisotropy {#get_max_anisotropy}

```gdscript
float get_max_anisotropy() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_filter {#get_min_filter}

```gdscript
OpenXRCompositionLayer.Filter get_min_filter() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mipmap_mode {#get_mipmap_mode}

```gdscript
OpenXRCompositionLayer.MipmapMode get_mipmap_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_red_swizzle {#get_red_swizzle}

```gdscript
OpenXRCompositionLayer.Swizzle get_red_swizzle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sort_order {#get_sort_order}

```gdscript
int get_sort_order() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_android_surface {#get_use_android_surface}

```gdscript
bool get_use_android_surface() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertical_wrap {#get_vertical_wrap}

```gdscript
OpenXRCompositionLayer.Wrap get_vertical_wrap() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### intersects_ray {#intersects_ray}

```gdscript
Vector2 intersects_ray(origin: Vector3, direction: Vector3) const
```

Returns UV coordinates where the given ray intersects with the composition layer. origin and direction must be in global space.

Returns Vector2(-1.0, -1.0) if the ray doesn't intersect.

### is_natively_supported {#is_natively_supported}

```gdscript
bool is_natively_supported() const
```

Returns true if the OpenXR runtime natively supports this composition layer type.

Note: This will only return an accurate result after the OpenXR session has started.

### is_protected_content {#is_protected_content}

```gdscript
bool is_protected_content() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_blend {#set_alpha_blend}

```gdscript
void set_alpha_blend(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_swizzle {#set_alpha_swizzle}

```gdscript
void set_alpha_swizzle(mode: OpenXRCompositionLayer.Swizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_android_surface_size {#set_android_surface_size}

```gdscript
void set_android_surface_size(size: Vector2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blue_swizzle {#set_blue_swizzle}

```gdscript
void set_blue_swizzle(mode: OpenXRCompositionLayer.Swizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_border_color {#set_border_color}

```gdscript
void set_border_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_hole_punch {#set_enable_hole_punch}

```gdscript
void set_enable_hole_punch(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_eye_visibility {#set_eye_visibility}

```gdscript
void set_eye_visibility(eye_visibility: OpenXRCompositionLayer.EyeVisibility)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_green_swizzle {#set_green_swizzle}

```gdscript
void set_green_swizzle(mode: OpenXRCompositionLayer.Swizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_horizontal_wrap {#set_horizontal_wrap}

```gdscript
void set_horizontal_wrap(mode: OpenXRCompositionLayer.Wrap)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_layer_viewport {#set_layer_viewport}

```gdscript
void set_layer_viewport(viewport: SubViewport)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mag_filter {#set_mag_filter}

```gdscript
void set_mag_filter(mode: OpenXRCompositionLayer.Filter)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_anisotropy {#set_max_anisotropy}

```gdscript
void set_max_anisotropy(value: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_filter {#set_min_filter}

```gdscript
void set_min_filter(mode: OpenXRCompositionLayer.Filter)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mipmap_mode {#set_mipmap_mode}

```gdscript
void set_mipmap_mode(mode: OpenXRCompositionLayer.MipmapMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_protected_content {#set_protected_content}

```gdscript
void set_protected_content(protected_content: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_red_swizzle {#set_red_swizzle}

```gdscript
void set_red_swizzle(mode: OpenXRCompositionLayer.Swizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sort_order {#set_sort_order}

```gdscript
void set_sort_order(order: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_android_surface {#set_use_android_surface}

```gdscript
void set_use_android_surface(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertical_wrap {#set_vertical_wrap}

```gdscript
void set_vertical_wrap(mode: OpenXRCompositionLayer.Wrap)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### layer_viewport {#layer_viewport}

```gdscript
layer_viewport: Object
```

- void set_layer_viewport(value: SubViewport)
- SubViewport get_layer_viewport()

The SubViewport to render on the composition layer.

### use_android_surface {#use_android_surface}

```gdscript
use_android_surface: bool
```

- void set_use_android_surface(value: bool)
- bool get_use_android_surface()

If enabled, an Android surface will be created (with the dimensions from android_surface_size) which will provide the 2D content for the composition layer, rather than using layer_viewport.

See get_android_surface() for information about how to get the surface so that your application can draw to it.

Note: This will only work in Android builds.

### protected_content {#protected_content}

```gdscript
protected_content: bool
```

- void set_protected_content(value: bool)
- bool is_protected_content()

If enabled, the OpenXR swapchain will be created with the XR_SWAPCHAIN_CREATE_PROTECTED_CONTENT_BIT flag, which will protect its contents from CPU access.

When used with an Android Surface, this may allow DRM content to be presented, and will only take effect when the Surface is first created; later changes to this property will have no effect.

### android_surface_size {#android_surface_size}

```gdscript
android_surface_size: Vector2i
```

- void set_android_surface_size(value: Vector2i)
- Vector2i get_android_surface_size()

The size of the Android surface to create if use_android_surface is enabled.

### sort_order {#sort_order}

```gdscript
sort_order: int
```

- void set_sort_order(value: int)
- int get_sort_order()

The sort order for this composition layer. Higher numbers will be shown in front of lower numbers.

Note: This will have no effect if a fallback mesh is being used.

### alpha_blend {#alpha_blend}

```gdscript
alpha_blend: bool
```

- void set_alpha_blend(value: bool)
- bool get_alpha_blend()

Enables the blending the layer using its alpha channel.

Can be combined with Viewport.transparent_bg to give the layer a transparent background.

### enable_hole_punch {#enable_hole_punch}

```gdscript
enable_hole_punch: bool
```

- void set_enable_hole_punch(value: bool)
- bool get_enable_hole_punch()

Enables a technique called "hole punching", which allows putting the composition layer behind the main projection layer (i.e. setting sort_order to a negative value) while "punching a hole" through everything rendered by Godot so that the layer is still visible.

This can be used to create the illusion that the composition layer exists in the same 3D space as everything rendered by Godot, allowing objects to appear to pass both behind or in front of the composition layer.

### eye_visibility {#eye_visibility}

```gdscript
eye_visibility: int
```

- void set_eye_visibility(value: EyeVisibility)
- EyeVisibility get_eye_visibility()

The eye(s) the composition layer is visible to.

Note: Not all composition layer types or runtimes support restricting visibility to a single eye.

### swapchain_state_min_filter {#swapchain_state_min_filter}

```gdscript
swapchain_state_min_filter: int
```

- void set_min_filter(value: Filter)
- Filter get_min_filter()

The minification filter of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_mag_filter {#swapchain_state_mag_filter}

```gdscript
swapchain_state_mag_filter: int
```

- void set_mag_filter(value: Filter)
- Filter get_mag_filter()

The magnification filter of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_mipmap_mode {#swapchain_state_mipmap_mode}

```gdscript
swapchain_state_mipmap_mode: int
```

- void set_mipmap_mode(value: MipmapMode)
- MipmapMode get_mipmap_mode()

The mipmap mode of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_horizontal_wrap {#swapchain_state_horizontal_wrap}

```gdscript
swapchain_state_horizontal_wrap: int
```

- void set_horizontal_wrap(value: Wrap)
- Wrap get_horizontal_wrap()

The horizontal wrap mode of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_vertical_wrap {#swapchain_state_vertical_wrap}

```gdscript
swapchain_state_vertical_wrap: int
```

- void set_vertical_wrap(value: Wrap)
- Wrap get_vertical_wrap()

The vertical wrap mode of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_red_swizzle {#swapchain_state_red_swizzle}

```gdscript
swapchain_state_red_swizzle: int
```

- void set_red_swizzle(value: Swizzle)
- Swizzle get_red_swizzle()

The swizzle value for the red channel of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_green_swizzle {#swapchain_state_green_swizzle}

```gdscript
swapchain_state_green_swizzle: int
```

- void set_green_swizzle(value: Swizzle)
- Swizzle get_green_swizzle()

The swizzle value for the green channel of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_blue_swizzle {#swapchain_state_blue_swizzle}

```gdscript
swapchain_state_blue_swizzle: int
```

- void set_blue_swizzle(value: Swizzle)
- Swizzle get_blue_swizzle()

The swizzle value for the blue channel of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_alpha_swizzle {#swapchain_state_alpha_swizzle}

```gdscript
swapchain_state_alpha_swizzle: int
```

- void set_alpha_swizzle(value: Swizzle)
- Swizzle get_alpha_swizzle()

The swizzle value for the alpha channel of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_max_anisotropy {#swapchain_state_max_anisotropy}

```gdscript
swapchain_state_max_anisotropy: float
```

- void set_max_anisotropy(value: float)
- float get_max_anisotropy()

The max anisotropy of the swapchain state.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

### swapchain_state_border_color {#swapchain_state_border_color}

```gdscript
swapchain_state_border_color: Color
```

- void set_border_color(value: Color)
- Color get_border_color()

The border color of the swapchain state that is used when the wrap mode clamps to the border.

Note: This property only has an effect on devices that support the OpenXR XR_FB_swapchain_update_state OpenGLES/Vulkan extensions.

## 枚举

### Filter {#enum-Filter}

```gdscript
enum Filter { FILTER_NEAREST = 0, FILTER_LINEAR = 1, FILTER_CUBIC = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILTER_NEAREST {#FILTER_NEAREST}

```gdscript
FILTER_NEAREST = 0
```

Perform nearest-neighbor filtering when sampling the texture.

#### FILTER_LINEAR {#FILTER_LINEAR}

```gdscript
FILTER_LINEAR = 1
```

Perform linear filtering when sampling the texture.

#### FILTER_CUBIC {#FILTER_CUBIC}

```gdscript
FILTER_CUBIC = 2
```

Perform cubic filtering when sampling the texture.

---

### MipmapMode {#enum-MipmapMode}

```gdscript
enum MipmapMode { MIPMAP_MODE_DISABLED = 0, MIPMAP_MODE_NEAREST = 1, MIPMAP_MODE_LINEAR = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MIPMAP_MODE_DISABLED {#MIPMAP_MODE_DISABLED}

```gdscript
MIPMAP_MODE_DISABLED = 0
```

Disable mipmapping.

Note: Mipmapping can only be disabled in the Compatibility renderer.

#### MIPMAP_MODE_NEAREST {#MIPMAP_MODE_NEAREST}

```gdscript
MIPMAP_MODE_NEAREST = 1
```

Use the mipmap of the nearest resolution.

#### MIPMAP_MODE_LINEAR {#MIPMAP_MODE_LINEAR}

```gdscript
MIPMAP_MODE_LINEAR = 2
```

Use linear interpolation of the two mipmaps of the nearest resolution.

---

### Wrap {#enum-Wrap}

```gdscript
enum Wrap { WRAP_CLAMP_TO_BORDER = 0, WRAP_CLAMP_TO_EDGE = 1, WRAP_REPEAT = 2, WRAP_MIRRORED_REPEAT = 3, WRAP_MIRROR_CLAMP_TO_EDGE = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### WRAP_CLAMP_TO_BORDER {#WRAP_CLAMP_TO_BORDER}

```gdscript
WRAP_CLAMP_TO_BORDER = 0
```

Clamp the texture to its specified border color.

#### WRAP_CLAMP_TO_EDGE {#WRAP_CLAMP_TO_EDGE}

```gdscript
WRAP_CLAMP_TO_EDGE = 1
```

Clamp the texture to its edge color.

#### WRAP_REPEAT {#WRAP_REPEAT}

```gdscript
WRAP_REPEAT = 2
```

Repeat the texture infinitely.

#### WRAP_MIRRORED_REPEAT {#WRAP_MIRRORED_REPEAT}

```gdscript
WRAP_MIRRORED_REPEAT = 3
```

Repeat the texture infinitely, mirroring it on each repeat.

#### WRAP_MIRROR_CLAMP_TO_EDGE {#WRAP_MIRROR_CLAMP_TO_EDGE}

```gdscript
WRAP_MIRROR_CLAMP_TO_EDGE = 4
```

Mirror the texture once and then clamp the texture to its edge color.

Note: This wrap mode is not available in the Compatibility renderer.

---

### Swizzle {#enum-Swizzle}

```gdscript
enum Swizzle { SWIZZLE_RED = 0, SWIZZLE_GREEN = 1, SWIZZLE_BLUE = 2, SWIZZLE_ALPHA = 3, SWIZZLE_ZERO = 4, SWIZZLE_ONE = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SWIZZLE_RED {#SWIZZLE_RED}

```gdscript
SWIZZLE_RED = 0
```

Maps a color channel to the value of the red channel.

#### SWIZZLE_GREEN {#SWIZZLE_GREEN}

```gdscript
SWIZZLE_GREEN = 1
```

Maps a color channel to the value of the green channel.

#### SWIZZLE_BLUE {#SWIZZLE_BLUE}

```gdscript
SWIZZLE_BLUE = 2
```

Maps a color channel to the value of the blue channel.

#### SWIZZLE_ALPHA {#SWIZZLE_ALPHA}

```gdscript
SWIZZLE_ALPHA = 3
```

Maps a color channel to the value of the alpha channel.

#### SWIZZLE_ZERO {#SWIZZLE_ZERO}

```gdscript
SWIZZLE_ZERO = 4
```

Maps a color channel to the value of zero.

#### SWIZZLE_ONE {#SWIZZLE_ONE}

```gdscript
SWIZZLE_ONE = 5
```

Maps a color channel to the value of one.

---

### EyeVisibility {#enum-EyeVisibility}

```gdscript
enum EyeVisibility { EYE_VISIBILITY_BOTH = 0, EYE_VISIBILITY_LEFT = 1, EYE_VISIBILITY_RIGHT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EYE_VISIBILITY_BOTH {#EYE_VISIBILITY_BOTH}

```gdscript
EYE_VISIBILITY_BOTH = 0
```

The layer is visible to both the left and right eyes.

#### EYE_VISIBILITY_LEFT {#EYE_VISIBILITY_LEFT}

```gdscript
EYE_VISIBILITY_LEFT = 1
```

The layer is visible only to the left eye.

#### EYE_VISIBILITY_RIGHT {#EYE_VISIBILITY_RIGHT}

```gdscript
EYE_VISIBILITY_RIGHT = 2
```

The layer is visible only to the right eye.

---
