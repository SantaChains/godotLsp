# PanoramaSkyMaterial

**继承**：[Material](class_material.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PanoramaSkyMaterial**

> A resource referenced in a Sky that is used to draw a background. PanoramaSkyMaterial functions similar to skyboxes in other engines, except it uses an equirectangular sky map instead of a Cubemap.

<details><summary>官方英文描述（中文精译进行中）</summary>

A resource referenced in a Sky that is used to draw a background. PanoramaSkyMaterial functions similar to skyboxes in other engines, except it uses an equirectangular sky map instead of a Cubemap.

Using an HDR panorama is strongly recommended for accurate, high-quality reflections. Godot supports the Radiance HDR (.hdr) and OpenEXR (.exr) image formats for this purpose.

You can use this tool [https://danilw.github.io/GLSL-howto/cubemap_to_panorama_js/cubemap_to_panorama.html] to convert a cubemap to an equirectangular sky map.

</details>

## 方法

### get_energy_multiplier {#get_energy_multiplier}

```gdscript
float get_energy_multiplier() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_panorama {#get_panorama}

```gdscript
Texture2D get_panorama() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_filtering_enabled {#is_filtering_enabled}

```gdscript
bool is_filtering_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_energy_multiplier {#set_energy_multiplier}

```gdscript
void set_energy_multiplier(multiplier: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_filtering_enabled {#set_filtering_enabled}

```gdscript
void set_filtering_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_panorama {#set_panorama}

```gdscript
void set_panorama(texture: Texture2D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### panorama {#panorama}

```gdscript
panorama: Texture2D
```

- void set_panorama(value: Texture2D)
- Texture2D get_panorama()

Texture2D to be applied to the PanoramaSkyMaterial.

### filter {#filter}

```gdscript
filter: bool
```

- void set_filtering_enabled(value: bool)
- bool is_filtering_enabled()

A boolean value to determine if the background texture should be filtered or not.

### energy_multiplier {#energy_multiplier}

```gdscript
energy_multiplier: float
```

- void set_energy_multiplier(value: float)
- float get_energy_multiplier()

The sky's overall brightness multiplier. Higher values result in a brighter sky.
