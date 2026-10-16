# RDTextureView

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDTextureView**

> This object is used by RenderingDevice.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object is used by RenderingDevice.

</details>

## 方法

### get_format_override {#get_format_override}

```gdscript
RenderingDevice.DataFormat get_format_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_swizzle_a {#get_swizzle_a}

```gdscript
RenderingDevice.TextureSwizzle get_swizzle_a() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_swizzle_b {#get_swizzle_b}

```gdscript
RenderingDevice.TextureSwizzle get_swizzle_b() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_swizzle_g {#get_swizzle_g}

```gdscript
RenderingDevice.TextureSwizzle get_swizzle_g() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_swizzle_r {#get_swizzle_r}

```gdscript
RenderingDevice.TextureSwizzle get_swizzle_r() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_format_override {#set_format_override}

```gdscript
void set_format_override(p_member: RenderingDevice.DataFormat)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_swizzle_a {#set_swizzle_a}

```gdscript
void set_swizzle_a(p_member: RenderingDevice.TextureSwizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_swizzle_b {#set_swizzle_b}

```gdscript
void set_swizzle_b(p_member: RenderingDevice.TextureSwizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_swizzle_g {#set_swizzle_g}

```gdscript
void set_swizzle_g(p_member: RenderingDevice.TextureSwizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_swizzle_r {#set_swizzle_r}

```gdscript
void set_swizzle_r(p_member: RenderingDevice.TextureSwizzle)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### format_override {#format_override}

```gdscript
format_override: int
```

- void set_format_override(value: DataFormat)
- DataFormat get_format_override()

Optional override for the data format to return sampled values in. The corresponding RDTextureFormat must have had this added as a shareable format. The default value of RenderingDevice.DATA_FORMAT_MAX does not override the format.

### swizzle_r {#swizzle_r}

```gdscript
swizzle_r: int
```

- void set_swizzle_r(value: TextureSwizzle)
- TextureSwizzle get_swizzle_r()

The channel to sample when sampling the red color channel.

### swizzle_g {#swizzle_g}

```gdscript
swizzle_g: int
```

- void set_swizzle_g(value: TextureSwizzle)
- TextureSwizzle get_swizzle_g()

The channel to sample when sampling the green color channel.

### swizzle_b {#swizzle_b}

```gdscript
swizzle_b: int
```

- void set_swizzle_b(value: TextureSwizzle)
- TextureSwizzle get_swizzle_b()

The channel to sample when sampling the blue color channel.

### swizzle_a {#swizzle_a}

```gdscript
swizzle_a: int
```

- void set_swizzle_a(value: TextureSwizzle)
- TextureSwizzle get_swizzle_a()

The channel to sample when sampling the alpha channel.
