# Texture2D

**继承**：[Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Texture2D**

> A texture works by registering an image in the video hardware, which then can be used in 3D models or 2D Sprite2D or GUI Control.

<details><summary>官方英文描述（中文精译进行中）</summary>

A texture works by registering an image in the video hardware, which then can be used in 3D models or 2D Sprite2D or GUI Control.

Textures are often created by loading them from a file. See @GDScript.load().

Texture2D is a base for other resources. It cannot be used directly.

Note: The maximum texture size is 16384×16384 pixels due to graphics hardware limitations. Larger textures may fail to import.

</details>

## 方法

### _draw {#_draw}

```gdscript
void _draw(to_canvas_item: RID, pos: Vector2, modulate: Color, transpose: bool) virtual const
```

Called when the entire Texture2D is requested to be drawn over a CanvasItem, with the top-left offset specified in pos. modulate specifies a multiplier for the colors being drawn, while transpose specifies whether drawing should be performed in column-major order instead of row-major order (resulting in 90-degree clockwise rotation).

Note: This is only used in 2D rendering, not 3D.

### _draw_rect {#_draw_rect}

```gdscript
void _draw_rect(to_canvas_item: RID, rect: Rect2, tile: bool, modulate: Color, transpose: bool) virtual const
```

Called when the Texture2D is requested to be drawn onto CanvasItem's specified rect. modulate specifies a multiplier for the colors being drawn, while transpose specifies whether drawing should be performed in column-major order instead of row-major order (resulting in 90-degree clockwise rotation).

Note: This is only used in 2D rendering, not 3D.

### _draw_rect_region {#_draw_rect_region}

```gdscript
void _draw_rect_region(to_canvas_item: RID, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool) virtual const
```

Called when a part of the Texture2D specified by src_rect's coordinates is requested to be drawn onto CanvasItem's specified rect. modulate specifies a multiplier for the colors being drawn, while transpose specifies whether drawing should be performed in column-major order instead of row-major order (resulting in 90-degree clockwise rotation).

Note: This is only used in 2D rendering, not 3D.

### _get_format {#_get_format}

```gdscript
Image.Format _get_format() virtual const
```

Called when get_format() is called.

### _get_height {#_get_height}

```gdscript
int _get_height() virtual const
```

Called when the Texture2D's height is queried.

### _get_image {#_get_image}

```gdscript
Image _get_image() virtual const
```

Called when get_image() is called.

### _get_mipmap_count {#_get_mipmap_count}

```gdscript
int _get_mipmap_count() virtual const
```

Called when get_mipmap_count() is called.

### _get_width {#_get_width}

```gdscript
int _get_width() virtual const
```

Called when the Texture2D's width is queried.

### _has_alpha {#_has_alpha}

```gdscript
bool _has_alpha() virtual const
```

Called when the presence of an alpha channel in the Texture2D is queried.

### _has_mipmaps {#_has_mipmaps}

```gdscript
bool _has_mipmaps() virtual const
```

Called when has_mipmaps() is called.

### _is_pixel_opaque {#_is_pixel_opaque}

```gdscript
bool _is_pixel_opaque(x: int, y: int) virtual const
```

Called when a pixel's opaque state in the Texture2D is queried at the specified (x, y) position.

### create_placeholder {#create_placeholder}

```gdscript
Resource create_placeholder() const
```

Creates a placeholder version of this resource (PlaceholderTexture2D).

### draw {#draw}

```gdscript
void draw(canvas_item: RID, position: Vector2, modulate: Color = Color(1, 1, 1, 1), transpose: bool = false) const
```

Draws the texture using a CanvasItem with the RenderingServer API at the specified position.

### draw_rect {#draw_rect}

```gdscript
void draw_rect(canvas_item: RID, rect: Rect2, tile: bool, modulate: Color = Color(1, 1, 1, 1), transpose: bool = false) const
```

Draws the texture using a CanvasItem with the RenderingServer API.

### draw_rect_region {#draw_rect_region}

```gdscript
void draw_rect_region(canvas_item: RID, rect: Rect2, src_rect: Rect2, modulate: Color = Color(1, 1, 1, 1), transpose: bool = false, clip_uv: bool = true) const
```

Draws a part of the texture using a CanvasItem with the RenderingServer API.

### get_format {#get_format}

```gdscript
Image.Format get_format() const
```

Returns the image format of the texture.

### get_height {#get_height}

```gdscript
int get_height() const
```

Returns the texture height in pixels.

### get_image {#get_image}

```gdscript
Image get_image() const
```

Returns an Image that is a copy of data from this Texture2D (a new Image is created each time). Images can be accessed and manipulated directly.

Note: This will return null if this Texture2D is invalid.

Note: This will fetch the texture data from the GPU, which might cause performance problems when overused. Avoid calling get_image() every frame, especially on large textures.

### get_mipmap_count {#get_mipmap_count}

```gdscript
int get_mipmap_count() const
```

Returns the number of mipmaps of the texture.

### get_size {#get_size}

```gdscript
Vector2 get_size() const
```

Returns the texture size in pixels.

### get_width {#get_width}

```gdscript
int get_width() const
```

Returns the texture width in pixels.

### has_alpha {#has_alpha}

```gdscript
bool has_alpha() const
```

Returns true if this Texture2D has an alpha channel.

### has_mipmaps {#has_mipmaps}

```gdscript
bool has_mipmaps() const
```

Returns true if the texture has mipmaps.
