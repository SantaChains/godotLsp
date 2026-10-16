# CompositorEffect

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CompositorEffect**

> This resource defines a custom rendering effect that can be applied to Viewports through the viewports' Environment. You can implement a callback that is called during rendering at a given stage of the rendering pipeline and allows you to insert additional passes. Note that this callback happens on the rendering thread. CompositorEffect is an abstract base class and must be extended to implement specific rendering logic.

<details><summary>官方英文描述（中文精译进行中）</summary>

This resource defines a custom rendering effect that can be applied to Viewports through the viewports' Environment. You can implement a callback that is called during rendering at a given stage of the rendering pipeline and allows you to insert additional passes. Note that this callback happens on the rendering thread. CompositorEffect is an abstract base class and must be extended to implement specific rendering logic.

</details>

## 方法

### _render_callback {#_render_callback}

```gdscript
void _render_callback(effect_callback_type: int, render_data: RenderData) virtual
```

Implement this function with your custom rendering code. effect_callback_type should always match the effect callback type you've specified in effect_callback_type. render_data provides access to the rendering state, it is only valid during rendering and should not be stored.

### get_access_resolved_color {#get_access_resolved_color}

```gdscript
bool get_access_resolved_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_access_resolved_depth {#get_access_resolved_depth}

```gdscript
bool get_access_resolved_depth() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_effect_callback_type {#get_effect_callback_type}

```gdscript
CompositorEffect.EffectCallbackType get_effect_callback_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enabled {#get_enabled}

```gdscript
bool get_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_needs_motion_vectors {#get_needs_motion_vectors}

```gdscript
bool get_needs_motion_vectors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_needs_normal_roughness {#get_needs_normal_roughness}

```gdscript
bool get_needs_normal_roughness() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_needs_separate_specular {#get_needs_separate_specular}

```gdscript
bool get_needs_separate_specular() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_access_resolved_color {#set_access_resolved_color}

```gdscript
void set_access_resolved_color(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_access_resolved_depth {#set_access_resolved_depth}

```gdscript
void set_access_resolved_depth(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_effect_callback_type {#set_effect_callback_type}

```gdscript
void set_effect_callback_type(effect_callback_type: CompositorEffect.EffectCallbackType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_needs_motion_vectors {#set_needs_motion_vectors}

```gdscript
void set_needs_motion_vectors(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_needs_normal_roughness {#set_needs_normal_roughness}

```gdscript
void set_needs_normal_roughness(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_needs_separate_specular {#set_needs_separate_specular}

```gdscript
void set_needs_separate_specular(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool get_enabled()

If true this rendering effect is applied to any viewport it is added to.

### effect_callback_type {#effect_callback_type}

```gdscript
effect_callback_type: int
```

- void set_effect_callback_type(value: EffectCallbackType)
- EffectCallbackType get_effect_callback_type()

The type of effect that is implemented, determines at what stage of rendering the callback is called.

### access_resolved_color {#access_resolved_color}

```gdscript
access_resolved_color: bool
```

- void set_access_resolved_color(value: bool)
- bool get_access_resolved_color()

If true and MSAA is enabled, this will trigger a color buffer resolve before the effect is run.

Note: In _render_callback(), to access the resolved buffer use:

```gdscript
var render_scene_buffers = render_data.get_render_scene_buffers()
var color_buffer = render_scene_buffers.get_texture("render_buffers", "color")
```

### access_resolved_depth {#access_resolved_depth}

```gdscript
access_resolved_depth: bool
```

- void set_access_resolved_depth(value: bool)
- bool get_access_resolved_depth()

If true and MSAA is enabled, this will trigger a depth buffer resolve before the effect is run.

Note: In _render_callback(), to access the resolved buffer use:

```gdscript
var render_scene_buffers = render_data.get_render_scene_buffers()
var depth_buffer = render_scene_buffers.get_texture("render_buffers", "depth")
```

### needs_motion_vectors {#needs_motion_vectors}

```gdscript
needs_motion_vectors: bool
```

- void set_needs_motion_vectors(value: bool)
- bool get_needs_motion_vectors()

If true this triggers motion vectors being calculated during the opaque render state.

Note: In _render_callback(), to access the motion vector buffer use:

```gdscript
var render_scene_buffers = render_data.get_render_scene_buffers()
var motion_buffer = render_scene_buffers.get_velocity_texture()
```

### needs_normal_roughness {#needs_normal_roughness}

```gdscript
needs_normal_roughness: bool
```

- void set_needs_normal_roughness(value: bool)
- bool get_needs_normal_roughness()

If true this triggers normal and roughness data to be output during our depth pre-pass, only applicable for the Forward+ renderer.

Note: In _render_callback(), to access the roughness buffer use:

```gdscript
var render_scene_buffers = render_data.get_render_scene_buffers()
var roughness_buffer = render_scene_buffers.get_texture("forward_clustered", "normal_roughness")
```

The raw normal and roughness buffer is stored in an optimized format, different than the one available in Spatial shaders. When sampling the buffer, a conversion function must be applied. Use this function, copied from here [https://github.com/godotengine/godot/blob/da5f39889f155658cef7f7ec3cc1abb94e17d815/servers/rendering/renderer_rd/shaders/forward_clustered/scene_forward_clustered_inc.glsl#L334-L341]:

```gdscript
vec4 normal_roughness_compatibility(vec4 p_normal_roughness) {
    float roughness = p_normal_roughness.w;
    if (roughness > 0.5) {
        roughness = 1.0 - roughness;
    }
    roughness /= (127.0 / 255.0);
    return vec4(normalize(p_normal_roughness.xyz * 2.0 - 1.0) * 0.5 + 0.5, roughness);
}
```

### needs_separate_specular {#needs_separate_specular}

```gdscript
needs_separate_specular: bool
```

- void set_needs_separate_specular(value: bool)
- bool get_needs_separate_specular()

If true this triggers specular data being rendered to a separate buffer and combined after effects have been applied, only applicable for the Forward+ renderer.

## 枚举

### EffectCallbackType {#enum-EffectCallbackType}

```gdscript
enum EffectCallbackType { EFFECT_CALLBACK_TYPE_PRE_OPAQUE = 0, EFFECT_CALLBACK_TYPE_POST_OPAQUE = 1, EFFECT_CALLBACK_TYPE_POST_SKY = 2, EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT = 3, EFFECT_CALLBACK_TYPE_POST_TRANSPARENT = 4, EFFECT_CALLBACK_TYPE_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EFFECT_CALLBACK_TYPE_PRE_OPAQUE {#EFFECT_CALLBACK_TYPE_PRE_OPAQUE}

```gdscript
EFFECT_CALLBACK_TYPE_PRE_OPAQUE = 0
```

The callback is called before our opaque rendering pass, but after depth prepass (if applicable).

#### EFFECT_CALLBACK_TYPE_POST_OPAQUE {#EFFECT_CALLBACK_TYPE_POST_OPAQUE}

```gdscript
EFFECT_CALLBACK_TYPE_POST_OPAQUE = 1
```

The callback is called after our opaque rendering pass, but before our sky is rendered.

#### EFFECT_CALLBACK_TYPE_POST_SKY {#EFFECT_CALLBACK_TYPE_POST_SKY}

```gdscript
EFFECT_CALLBACK_TYPE_POST_SKY = 2
```

The callback is called after our sky is rendered, but before our back buffers are created (and if enabled, before subsurface scattering and/or screen space reflections).

#### EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT {#EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT}

```gdscript
EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT = 3
```

The callback is called before our transparent rendering pass, but after our sky is rendered and we've created our back buffers.

#### EFFECT_CALLBACK_TYPE_POST_TRANSPARENT {#EFFECT_CALLBACK_TYPE_POST_TRANSPARENT}

```gdscript
EFFECT_CALLBACK_TYPE_POST_TRANSPARENT = 4
```

The callback is called after our transparent rendering pass, but before any built-in post-processing effects and output to our render target.

#### EFFECT_CALLBACK_TYPE_MAX {#EFFECT_CALLBACK_TYPE_MAX}

```gdscript
EFFECT_CALLBACK_TYPE_MAX = 5
```

Represents the size of the EffectCallbackType enum.

---
