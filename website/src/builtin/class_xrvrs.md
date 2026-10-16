# XRVRS

**继承**：[Object](class_object.md) → **XRVRS**

> This class is used by various XR interfaces to generate VRS textures that can be used to speed up rendering.

<details><summary>官方英文描述</summary>

This class is used by various XR interfaces to generate VRS textures that can be used to speed up rendering.

</details>

## 方法

### get_vrs_min_radius {#get_vrs_min_radius}

```gdscript
float get_vrs_min_radius() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vrs_render_region {#get_vrs_render_region}

```gdscript
Rect2i get_vrs_render_region() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vrs_strength {#get_vrs_strength}

```gdscript
float get_vrs_strength() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### make_vrs_texture {#make_vrs_texture}

```gdscript
RID make_vrs_texture(target_size: Vector2, eye_foci: PackedVector2Array)
```

Generates the VRS texture based on a render target_size adjusted by our VRS tile size. For each eyes focal point passed in eye_foci a layer is created. Focal point should be in NDC.

The result will be cached, requesting a VRS texture with unchanged parameters and settings will return the cached RID.

### set_vrs_min_radius {#set_vrs_min_radius}

```gdscript
void set_vrs_min_radius(radius: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_render_region {#set_vrs_render_region}

```gdscript
void set_vrs_render_region(render_region: Rect2i)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vrs_strength {#set_vrs_strength}

```gdscript
void set_vrs_strength(strength: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### vrs_min_radius {#vrs_min_radius}

```gdscript
vrs_min_radius: float
```

- void set_vrs_min_radius(value: float)
- float get_vrs_min_radius()

The minimum radius around the focal point where full quality is guaranteed if VRS is used as a percentage of screen size.

### vrs_strength {#vrs_strength}

```gdscript
vrs_strength: float
```

- void set_vrs_strength(value: float)
- float get_vrs_strength()

The strength used to calculate the VRS density map. The greater this value, the more noticeable VRS is.

### vrs_render_region {#vrs_render_region}

```gdscript
vrs_render_region: Rect2i
```

- void set_vrs_render_region(value: Rect2i)
- Rect2i get_vrs_render_region()

The render region that the VRS texture will be scaled to when generated.
