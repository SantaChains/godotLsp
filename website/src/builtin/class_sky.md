# Sky

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Sky**

> The Sky class uses a Material to render a 3D environment's background and the light it emits by updating the reflection/radiance cubemaps.

<details><summary>官方英文描述（中文精译进行中）</summary>

The Sky class uses a Material to render a 3D environment's background and the light it emits by updating the reflection/radiance cubemaps.

</details>

## 方法

### get_material {#get_material}

```gdscript
Material get_material() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_process_mode {#get_process_mode}

```gdscript
Sky.ProcessMode get_process_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_radiance_size {#get_radiance_size}

```gdscript
Sky.RadianceSize get_radiance_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_material {#set_material}

```gdscript
void set_material(material: Material)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_process_mode {#set_process_mode}

```gdscript
void set_process_mode(mode: Sky.ProcessMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_radiance_size {#set_radiance_size}

```gdscript
void set_radiance_size(size: Sky.RadianceSize)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### sky_material {#sky_material}

```gdscript
sky_material: PanoramaSkyMaterial,ProceduralSkyMaterial,PhysicalSkyMaterial,ShaderMaterial
```

- void set_material(value: Material)
- Material get_material()

Material used to draw the background. Can be PanoramaSkyMaterial, ProceduralSkyMaterial, PhysicalSkyMaterial, or even a ShaderMaterial if you want to use your own custom shader.

### process_mode {#process_mode}

```gdscript
process_mode: int
```

- void set_process_mode(value: ProcessMode)
- ProcessMode get_process_mode()

The method for generating the radiance map from the sky. The radiance map is a cubemap with increasingly blurry versions of the sky corresponding to different levels of roughness. Radiance maps can be expensive to calculate.

### radiance_size {#radiance_size}

```gdscript
radiance_size: int
```

- void set_radiance_size(value: RadianceSize)
- RadianceSize get_radiance_size()

The Sky's radiance map size. The higher the radiance map size, the more detailed the lighting from the Sky will be.

Note: Some hardware will have trouble with higher radiance sizes, especially RADIANCE_SIZE_512 and above. Only use such high values on high-end hardware.

## 枚举

### RadianceSize {#enum-RadianceSize}

```gdscript
enum RadianceSize { RADIANCE_SIZE_32 = 0, RADIANCE_SIZE_64 = 1, RADIANCE_SIZE_128 = 2, RADIANCE_SIZE_256 = 3, RADIANCE_SIZE_512 = 4, RADIANCE_SIZE_1024 = 5, RADIANCE_SIZE_2048 = 6, RADIANCE_SIZE_MAX = 7 }
```

RadianceSize RADIANCE_SIZE_32 = 0

Radiance texture size is 32×32 pixels.

RadianceSize RADIANCE_SIZE_64 = 1

Radiance texture size is 64×64 pixels.

RadianceSize RADIANCE_SIZE_128 = 2

Radiance texture size is 128×128 pixels.

RadianceSize RADIANCE_SIZE_256 = 3

Radiance texture size is 256×256 pixels.

RadianceSize RADIANCE_SIZE_512 = 4

Radiance texture size is 512×512 pixels.

RadianceSize RADIANCE_SIZE_1024 = 5

Radiance texture size is 1024×1024 pixels.

RadianceSize RADIANCE_SIZE_2048 = 6

Radiance texture size is 2048×2048 pixels.

RadianceSize RADIANCE_SIZE_MAX = 7

Represents the size of the RadianceSize enum.

#### RADIANCE_SIZE_32 {#RADIANCE_SIZE_32}

```gdscript
RADIANCE_SIZE_32 = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RADIANCE_SIZE_64 {#RADIANCE_SIZE_64}

```gdscript
RADIANCE_SIZE_64 = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RADIANCE_SIZE_128 {#RADIANCE_SIZE_128}

```gdscript
RADIANCE_SIZE_128 = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RADIANCE_SIZE_256 {#RADIANCE_SIZE_256}

```gdscript
RADIANCE_SIZE_256 = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RADIANCE_SIZE_512 {#RADIANCE_SIZE_512}

```gdscript
RADIANCE_SIZE_512 = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RADIANCE_SIZE_1024 {#RADIANCE_SIZE_1024}

```gdscript
RADIANCE_SIZE_1024 = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RADIANCE_SIZE_2048 {#RADIANCE_SIZE_2048}

```gdscript
RADIANCE_SIZE_2048 = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RADIANCE_SIZE_MAX {#RADIANCE_SIZE_MAX}

```gdscript
RADIANCE_SIZE_MAX = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### ProcessMode {#enum-ProcessMode}

```gdscript
enum ProcessMode { PROCESS_MODE_AUTOMATIC = 0, PROCESS_MODE_QUALITY = 1, PROCESS_MODE_INCREMENTAL = 2, PROCESS_MODE_REALTIME = 3 }
```

ProcessMode PROCESS_MODE_AUTOMATIC = 0

Automatically selects the appropriate process mode based on your sky shader. If your shader uses TIME or POSITION, this will use PROCESS_MODE_REALTIME. If your shader uses any of the LIGHT_* variables or any custom uniforms, this uses PROCESS_MODE_INCREMENTAL. Otherwise, this defaults to PROCESS_MODE_QUALITY.

ProcessMode PROCESS_MODE_QUALITY = 1

Uses high quality importance sampling to process the radiance map. In general, this results in much higher quality than PROCESS_MODE_REALTIME but takes much longer to generate. This should not be used if you plan on changing the sky at runtime. If you are finding that the reflection is not blurry enough and is showing sparkles or fireflies, try increasing ProjectSettings.rendering/reflections/sky_reflections/ggx_samples.

ProcessMode PROCESS_MODE_INCREMENTAL = 2

Uses the same high quality importance sampling to process the radiance map as PROCESS_MODE_QUALITY, but updates over several frames. The number of frames is determined by ProjectSettings.rendering/reflections/sky_reflections/roughness_layers. Use this when you need highest quality radiance maps, but have a sky that updates slowly.

ProcessMode PROCESS_MODE_REALTIME = 3

Uses the fast filtering algorithm to process the radiance map. In general this results in lower quality, but substantially faster run times. If you need better quality, but still need to update the sky every frame, consider turning on ProjectSettings.rendering/reflections/sky_reflections/fast_filter_high_quality.

Note: The fast filtering algorithm is limited to 256×256 cubemaps, so radiance_size must be set to RADIANCE_SIZE_256. Otherwise, a warning is printed and the overridden radiance size is ignored.

#### PROCESS_MODE_AUTOMATIC {#PROCESS_MODE_AUTOMATIC}

```gdscript
PROCESS_MODE_AUTOMATIC = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROCESS_MODE_QUALITY {#PROCESS_MODE_QUALITY}

```gdscript
PROCESS_MODE_QUALITY = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROCESS_MODE_INCREMENTAL {#PROCESS_MODE_INCREMENTAL}

```gdscript
PROCESS_MODE_INCREMENTAL = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROCESS_MODE_REALTIME {#PROCESS_MODE_REALTIME}

```gdscript
PROCESS_MODE_REALTIME = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
