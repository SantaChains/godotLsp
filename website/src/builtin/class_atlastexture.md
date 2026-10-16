# AtlasTexture

**继承**：[Texture2D](class_texture2d.md) → [Texture](class_texture.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AtlasTexture**

> Texture2D resource that draws only part of its atlas texture, as defined by the region. An additional margin can also be set, which is useful for small adjustments.

<details><summary>官方英文描述</summary>

Texture2D resource that draws only part of its atlas texture, as defined by the region. An additional margin can also be set, which is useful for small adjustments.

Multiple AtlasTexture resources can be cropped from the same atlas. Packing many smaller textures into a singular large texture helps to optimize video memory costs and render calls.

Note: AtlasTexture cannot be used in an AnimatedTexture, and will not tile properly in nodes such as TextureRect or Sprite2D. To tile an AtlasTexture, modify its region instead.

</details>

## 方法

### get_atlas {#get_atlas}

```gdscript
Texture2D get_atlas() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_margin {#get_margin}

```gdscript
Rect2 get_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_region {#get_region}

```gdscript
Rect2 get_region() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_filter_clip {#has_filter_clip}

```gdscript
bool has_filter_clip() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_atlas {#set_atlas}

```gdscript
void set_atlas(atlas: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filter_clip {#set_filter_clip}

```gdscript
void set_filter_clip(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin {#set_margin}

```gdscript
void set_margin(margin: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_region {#set_region}

```gdscript
void set_region(region: Rect2)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### atlas {#atlas}

```gdscript
atlas: Texture2D
```

- void set_atlas(value: Texture2D)
- Texture2D get_atlas()

The texture that contains the atlas. Can be any type inheriting from Texture2D, including another AtlasTexture.

### region {#region}

```gdscript
region: Rect2
```

- void set_region(value: Rect2)
- Rect2 get_region()

The region used to draw the atlas. If either dimension of the region's size is 0, the value from atlas size will be used for that axis instead.

Note: The image size is always an integer, so the actual region size is rounded down.

### margin {#margin}

```gdscript
margin: Rect2
```

- void set_margin(value: Rect2)
- Rect2 get_margin()

The margin around the region. Useful for small adjustments. If the Rect2.size of this property ("w" and "h" in the editor) is set, the drawn texture is resized to fit within the margin.

### filter_clip {#filter_clip}

```gdscript
filter_clip: bool
```

- void set_filter_clip(value: bool)
- bool has_filter_clip()

If true, the area outside of the region is clipped to avoid bleeding of the surrounding texture pixels.
