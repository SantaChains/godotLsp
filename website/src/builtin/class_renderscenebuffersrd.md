# RenderSceneBuffersRD

**继承**：[RenderSceneBuffers](class_renderscenebuffers.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **RenderSceneBuffersRD**

> This object manages all 3D rendering buffers for the rendering device based renderers. An instance of this object is created for every viewport that has 3D rendering enabled. See also RenderSceneBuffers.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object manages all 3D rendering buffers for the rendering device based renderers. An instance of this object is created for every viewport that has 3D rendering enabled. See also RenderSceneBuffers.

All buffers are organized in contexts. The default context is called render_buffers and can contain amongst others the color buffer, depth buffer, velocity buffers, VRS density map and MSAA variants of these buffers.

Buffers are only guaranteed to exist during rendering of the viewport.

Note: This is an internal rendering server object. Do not instantiate this class from a script.

</details>

## 方法

### clear_context {#clear_context}

```gdscript
void clear_context(context: StringName)
```

Frees all buffers related to this context.

### create_texture {#create_texture}

```gdscript
RID create_texture(context: StringName, name: StringName, data_format: RenderingDevice.DataFormat, usage_bits: int, texture_samples: RenderingDevice.TextureSamples, size: Vector2i, layers: int, mipmaps: int, unique: bool, discardable: bool)
```

Create a new texture with the given definition and cache this under the given name. Will return the existing texture if it already exists.

### create_texture_from_format {#create_texture_from_format}

```gdscript
RID create_texture_from_format(context: StringName, name: StringName, format: RDTextureFormat, view: RDTextureView, unique: bool)
```

Create a new texture using the given format and view and cache this under the given name. Will return the existing texture if it already exists.

### create_texture_view {#create_texture_view}

```gdscript
RID create_texture_view(context: StringName, name: StringName, view_name: StringName, view: RDTextureView)
```

Create a new texture view for an existing texture and cache this under the given view_name. Will return the existing texture view if it already exists. Will error if the source texture doesn't exist.

### get_color_layer {#get_color_layer}

```gdscript
RID get_color_layer(layer: int, msaa: bool = false)
```

Returns the specified layer from the color texture we are rendering 3D content to.

If msaa is true and MSAA is enabled, this returns the MSAA variant of the buffer.

### get_color_texture {#get_color_texture}

```gdscript
RID get_color_texture(msaa: bool = false)
```

Returns the color texture we are rendering 3D content to. If multiview is used this will be a texture array with all views.

If msaa is true and MSAA is enabled, this returns the MSAA variant of the buffer.

### get_depth_layer {#get_depth_layer}

```gdscript
RID get_depth_layer(layer: int, msaa: bool = false)
```

Returns the specified layer from the depth texture we are rendering 3D content to.

If msaa is true and MSAA is enabled, this returns the MSAA variant of the buffer.

### get_depth_texture {#get_depth_texture}

```gdscript
RID get_depth_texture(msaa: bool = false)
```

Returns the depth texture we are rendering 3D content to. If multiview is used this will be a texture array with all views.

If msaa is true and MSAA is enabled, this returns the MSAA variant of the buffer.

### get_fsr_sharpness {#get_fsr_sharpness}

```gdscript
float get_fsr_sharpness() const
```

Returns the FSR sharpness value used while rendering the 3D content (if get_scaling_3d_mode() is an FSR mode).

### get_internal_size {#get_internal_size}

```gdscript
Vector2i get_internal_size() const
```

Returns the internal size of the render buffer (size before upscaling) with which textures are created by default.

### get_msaa_3d {#get_msaa_3d}

```gdscript
RenderingServer.ViewportMSAA get_msaa_3d() const
```

Returns the applied 3D MSAA mode for this viewport.

### get_render_target {#get_render_target}

```gdscript
RID get_render_target() const
```

Returns the render target associated with this buffers object.

### get_scaling_3d_mode {#get_scaling_3d_mode}

```gdscript
RenderingServer.ViewportScaling3DMode get_scaling_3d_mode() const
```

Returns the scaling mode used for upscaling.

### get_screen_space_aa {#get_screen_space_aa}

```gdscript
RenderingServer.ViewportScreenSpaceAA get_screen_space_aa() const
```

Returns the screen-space antialiasing method applied.

### get_target_size {#get_target_size}

```gdscript
Vector2i get_target_size() const
```

Returns the target size of the render buffer (size after upscaling).

### get_texture {#get_texture}

```gdscript
RID get_texture(context: StringName, name: StringName) const
```

Returns a cached texture with this name.

### get_texture_format {#get_texture_format}

```gdscript
RDTextureFormat get_texture_format(context: StringName, name: StringName) const
```

Returns the texture format information with which a cached texture was created.

### get_texture_samples {#get_texture_samples}

```gdscript
RenderingDevice.TextureSamples get_texture_samples() const
```

Returns the number of MSAA samples used.

### get_texture_slice {#get_texture_slice}

```gdscript
RID get_texture_slice(context: StringName, name: StringName, layer: int, mipmap: int, layers: int, mipmaps: int)
```

Returns a specific slice (layer or mipmap) for a cached texture.

### get_texture_slice_size {#get_texture_slice_size}

```gdscript
Vector2i get_texture_slice_size(context: StringName, name: StringName, mipmap: int)
```

Returns the texture size of a given slice of a cached texture.

### get_texture_slice_view {#get_texture_slice_view}

```gdscript
RID get_texture_slice_view(context: StringName, name: StringName, layer: int, mipmap: int, layers: int, mipmaps: int, view: RDTextureView)
```

Returns a specific view of a slice (layer or mipmap) for a cached texture.

### get_use_debanding {#get_use_debanding}

```gdscript
bool get_use_debanding() const
```

Returns true if debanding is enabled.

### get_use_taa {#get_use_taa}

```gdscript
bool get_use_taa() const
```

Returns true if TAA is enabled.

### get_velocity_layer {#get_velocity_layer}

```gdscript
RID get_velocity_layer(layer: int, msaa: bool = false)
```

Returns the specified layer from the velocity texture we are rendering 3D content to.

### get_velocity_texture {#get_velocity_texture}

```gdscript
RID get_velocity_texture(msaa: bool = false)
```

Returns the velocity texture we are rendering 3D content to. If multiview is used this will be a texture array with all views.

If msaa is true and MSAA is enabled, this returns the MSAA variant of the buffer.

### get_view_count {#get_view_count}

```gdscript
int get_view_count() const
```

Returns the view count for the associated viewport.

### has_texture {#has_texture}

```gdscript
bool has_texture(context: StringName, name: StringName) const
```

Returns true if a cached texture exists for this name.
