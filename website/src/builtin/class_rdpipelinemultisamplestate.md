# RDPipelineMultisampleState

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDPipelineMultisampleState**

> RDPipelineMultisampleState is used to control how multisample or supersample antialiasing is being performed when rendering using RenderingDevice.

<details><summary>官方英文描述</summary>

RDPipelineMultisampleState is used to control how multisample or supersample antialiasing is being performed when rendering using RenderingDevice.

</details>

## 方法

### get_enable_alpha_to_coverage {#get_enable_alpha_to_coverage}

```gdscript
bool get_enable_alpha_to_coverage() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_alpha_to_one {#get_enable_alpha_to_one}

```gdscript
bool get_enable_alpha_to_one() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_sample_shading {#get_enable_sample_shading}

```gdscript
bool get_enable_sample_shading() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_min_sample_shading {#get_min_sample_shading}

```gdscript
float get_min_sample_shading() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sample_count {#get_sample_count}

```gdscript
RenderingDevice.TextureSamples get_sample_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sample_masks {#get_sample_masks}

```gdscript
typedarray::int get_sample_masks() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_alpha_to_coverage {#set_enable_alpha_to_coverage}

```gdscript
void set_enable_alpha_to_coverage(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_alpha_to_one {#set_enable_alpha_to_one}

```gdscript
void set_enable_alpha_to_one(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_sample_shading {#set_enable_sample_shading}

```gdscript
void set_enable_sample_shading(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_min_sample_shading {#set_min_sample_shading}

```gdscript
void set_min_sample_shading(p_member: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sample_count {#set_sample_count}

```gdscript
void set_sample_count(p_member: RenderingDevice.TextureSamples)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sample_masks {#set_sample_masks}

```gdscript
void set_sample_masks(masks: typedarray::int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### sample_count {#sample_count}

```gdscript
sample_count: int
```

- void set_sample_count(value: TextureSamples)
- TextureSamples get_sample_count()

The number of MSAA samples (or SSAA samples if enable_sample_shading is true) to perform. Higher values result in better antialiasing, at the cost of performance.

### enable_sample_shading {#enable_sample_shading}

```gdscript
enable_sample_shading: bool
```

- void set_enable_sample_shading(value: bool)
- bool get_enable_sample_shading()

If true, enables per-sample shading which replaces MSAA by SSAA. This provides higher quality antialiasing that works with transparent (alpha scissor) edges. This has a very high performance cost. See also min_sample_shading. See the per-sample shading Vulkan documentation [https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-sampleshading] for more details.

### min_sample_shading {#min_sample_shading}

```gdscript
min_sample_shading: float
```

- void set_min_sample_shading(value: float)
- float get_min_sample_shading()

The multiplier of sample_count that determines how many samples are performed for each fragment. Must be between 0.0 and 1.0 (inclusive). Only effective if enable_sample_shading is true. If min_sample_shading is 1.0, fragment invocation must only read from the coverage index sample. Tile image access must not be used if enable_sample_shading is not 1.0.

### enable_alpha_to_coverage {#enable_alpha_to_coverage}

```gdscript
enable_alpha_to_coverage: bool
```

- void set_enable_alpha_to_coverage(value: bool)
- bool get_enable_alpha_to_coverage()

If true, alpha to coverage is enabled. This generates a temporary coverage value based on the alpha component of the fragment's first color output. This allows alpha transparency to make use of multisample antialiasing.

### enable_alpha_to_one {#enable_alpha_to_one}

```gdscript
enable_alpha_to_one: bool
```

- void set_enable_alpha_to_one(value: bool)
- bool get_enable_alpha_to_one()

If true, alpha is forced to either 0.0 or 1.0. This allows hardening the edges of antialiased alpha transparencies. Only relevant if enable_alpha_to_coverage is true.

### sample_masks {#sample_masks}

```gdscript
sample_masks: typedarray::int
```

- void set_sample_masks(value: Array[int])
- Array[int] get_sample_masks()

The sample mask array. See the sample mask Vulkan documentation [https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-samplemask] for more details.
