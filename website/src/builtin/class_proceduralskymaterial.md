# ProceduralSkyMaterial

**继承**：[Material](class_material.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ProceduralSkyMaterial**

> ProceduralSkyMaterial provides a way to create an effective background quickly by defining procedural parameters for the sun, the sky and the ground. The sky and ground are defined by a main color, a color at the horizon, and an easing curve to interpolate between them. Suns are described by a position in the sky, a color, and a max angle from the sun at which the easing curve ends. The max angle therefore defines the size of the sun in the sky.

<details><summary>官方英文描述（中文精译进行中）</summary>

ProceduralSkyMaterial provides a way to create an effective background quickly by defining procedural parameters for the sun, the sky and the ground. The sky and ground are defined by a main color, a color at the horizon, and an easing curve to interpolate between them. Suns are described by a position in the sky, a color, and a max angle from the sun at which the easing curve ends. The max angle therefore defines the size of the sun in the sky.

ProceduralSkyMaterial supports up to 4 suns, using the color, and energy, direction, and angular distance of the first four DirectionalLight3D nodes in the scene. This means that the suns are defined individually by the properties of their corresponding DirectionalLight3Ds and globally by sun_angle_max and sun_curve.

ProceduralSkyMaterial uses a lightweight shader to draw the sky and is therefore suited for real-time updates. This makes it a great option for a sky that is simple and computationally cheap, but unrealistic. If you need a more realistic procedural option, use PhysicalSkyMaterial.

</details>

## 方法

### get_energy_multiplier {#get_energy_multiplier}

```gdscript
float get_energy_multiplier() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ground_bottom_color {#get_ground_bottom_color}

```gdscript
Color get_ground_bottom_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ground_curve {#get_ground_curve}

```gdscript
float get_ground_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ground_energy_multiplier {#get_ground_energy_multiplier}

```gdscript
float get_ground_energy_multiplier() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ground_horizon_color {#get_ground_horizon_color}

```gdscript
Color get_ground_horizon_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sky_cover {#get_sky_cover}

```gdscript
Texture2D get_sky_cover() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sky_cover_modulate {#get_sky_cover_modulate}

```gdscript
Color get_sky_cover_modulate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sky_curve {#get_sky_curve}

```gdscript
float get_sky_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sky_energy_multiplier {#get_sky_energy_multiplier}

```gdscript
float get_sky_energy_multiplier() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sky_horizon_color {#get_sky_horizon_color}

```gdscript
Color get_sky_horizon_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sky_top_color {#get_sky_top_color}

```gdscript
Color get_sky_top_color() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sun_angle_max {#get_sun_angle_max}

```gdscript
float get_sun_angle_max() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sun_curve {#get_sun_curve}

```gdscript
float get_sun_curve() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_use_debanding {#get_use_debanding}

```gdscript
bool get_use_debanding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_energy_multiplier {#set_energy_multiplier}

```gdscript
void set_energy_multiplier(multiplier: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ground_bottom_color {#set_ground_bottom_color}

```gdscript
void set_ground_bottom_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ground_curve {#set_ground_curve}

```gdscript
void set_ground_curve(curve: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ground_energy_multiplier {#set_ground_energy_multiplier}

```gdscript
void set_ground_energy_multiplier(energy: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ground_horizon_color {#set_ground_horizon_color}

```gdscript
void set_ground_horizon_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sky_cover {#set_sky_cover}

```gdscript
void set_sky_cover(sky_cover: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sky_cover_modulate {#set_sky_cover_modulate}

```gdscript
void set_sky_cover_modulate(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sky_curve {#set_sky_curve}

```gdscript
void set_sky_curve(curve: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sky_energy_multiplier {#set_sky_energy_multiplier}

```gdscript
void set_sky_energy_multiplier(multiplier: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sky_horizon_color {#set_sky_horizon_color}

```gdscript
void set_sky_horizon_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sky_top_color {#set_sky_top_color}

```gdscript
void set_sky_top_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sun_angle_max {#set_sun_angle_max}

```gdscript
void set_sun_angle_max(degrees: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sun_curve {#set_sun_curve}

```gdscript
void set_sun_curve(curve: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_debanding {#set_use_debanding}

```gdscript
void set_use_debanding(use_debanding: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### sky_top_color {#sky_top_color}

```gdscript
sky_top_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### sky_horizon_color {#sky_horizon_color}

```gdscript
sky_horizon_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### sky_curve {#sky_curve}

```gdscript
sky_curve: float
```

- void set_sky_curve(value: float)
- float get_sky_curve()

How quickly the sky_horizon_color fades into the sky_top_color.

### sky_energy_multiplier {#sky_energy_multiplier}

```gdscript
sky_energy_multiplier: float
```

- void set_sky_energy_multiplier(value: float)
- float get_sky_energy_multiplier()

Multiplier for sky color. A higher value will make the sky brighter.

### sky_cover {#sky_cover}

```gdscript
sky_cover: Texture2D
```

- void set_sky_cover(value: Texture2D)
- Texture2D get_sky_cover()

The sky cover texture to use. This texture must use an equirectangular projection (similar to PanoramaSkyMaterial). The texture's colors will be added to the existing sky color, and will be multiplied by sky_energy_multiplier and sky_cover_modulate. This is mainly suited to displaying stars at night, but it can also be used to display clouds at day or night (with a non-physically-accurate look).

### sky_cover_modulate {#sky_cover_modulate}

```gdscript
sky_cover_modulate: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ground_bottom_color {#ground_bottom_color}

```gdscript
ground_bottom_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ground_horizon_color {#ground_horizon_color}

```gdscript
ground_horizon_color: Color
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ground_curve {#ground_curve}

```gdscript
ground_curve: float
```

- void set_ground_curve(value: float)
- float get_ground_curve()

How quickly the ground_horizon_color fades into the ground_bottom_color.

### ground_energy_multiplier {#ground_energy_multiplier}

```gdscript
ground_energy_multiplier: float
```

- void set_ground_energy_multiplier(value: float)
- float get_ground_energy_multiplier()

Multiplier for ground color. A higher value will make the ground brighter.

### sun_angle_max {#sun_angle_max}

```gdscript
sun_angle_max: float
```

- void set_sun_angle_max(value: float)
- float get_sun_angle_max()

Distance from center of sun where it fades out completely.

### sun_curve {#sun_curve}

```gdscript
sun_curve: float
```

- void set_sun_curve(value: float)
- float get_sun_curve()

How quickly the sun fades away between the edge of the sun disk and sun_angle_max.

### use_debanding {#use_debanding}

```gdscript
use_debanding: bool
```

- void set_use_debanding(value: bool)
- bool get_use_debanding()

If true, enables debanding. Debanding adds a small amount of noise which helps reduce banding that appears from the smooth changes in color in the sky.

### energy_multiplier {#energy_multiplier}

```gdscript
energy_multiplier: float
```

- void set_energy_multiplier(value: float)
- float get_energy_multiplier()

The sky's overall brightness multiplier. Higher values result in a brighter sky.
