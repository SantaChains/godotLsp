# RDPipelineColorBlendStateAttachment

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDPipelineColorBlendStateAttachment**

> Controls how blending between source and destination fragments is performed when using RenderingDevice.

<details><summary>官方英文描述</summary>

Controls how blending between source and destination fragments is performed when using RenderingDevice.

For reference, this is how common user-facing blend modes are implemented in Godot's 2D renderer:

Mix:

```gdscript
var attachment = RDPipelineColorBlendStateAttachment.new()
attachment.enable_blend = true
attachment.color_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.src_color_blend_factor = RenderingDevice.BLEND_FACTOR_SRC_ALPHA
attachment.dst_color_blend_factor = RenderingDevice.BLEND_FACTOR_ONE_MINUS_SRC_ALPHA
attachment.alpha_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.src_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_ONE
attachment.dst_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_ONE_MINUS_SRC_ALPHA
```

Add:

```gdscript
var attachment = RDPipelineColorBlendStateAttachment.new()
attachment.enable_blend = true
attachment.alpha_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.color_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.src_color_blend_factor = RenderingDevice.BLEND_FACTOR_SRC_ALPHA
attachment.dst_color_blend_factor = RenderingDevice.BLEND_FACTOR_ONE
attachment.src_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_SRC_ALPHA
attachment.dst_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_ONE
```

Subtract:

```gdscript
var attachment = RDPipelineColorBlendStateAttachment.new()
attachment.enable_blend = true
attachment.alpha_blend_op = RenderingDevice.BLEND_OP_REVERSE_SUBTRACT
attachment.color_blend_op = RenderingDevice.BLEND_OP_REVERSE_SUBTRACT
attachment.src_color_blend_factor = RenderingDevice.BLEND_FACTOR_SRC_ALPHA
attachment.dst_color_blend_factor = RenderingDevice.BLEND_FACTOR_ONE
attachment.src_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_SRC_ALPHA
attachment.dst_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_ONE
```

Multiply:

```gdscript
var attachment = RDPipelineColorBlendStateAttachment.new()
attachment.enable_blend = true
attachment.alpha_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.color_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.src_color_blend_factor = RenderingDevice.BLEND_FACTOR_DST_COLOR
attachment.dst_color_blend_factor = RenderingDevice.BLEND_FACTOR_ZERO
attachment.src_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_DST_ALPHA
attachment.dst_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_ZERO
```

Pre-multiplied alpha:

```gdscript
var attachment = RDPipelineColorBlendStateAttachment.new()
attachment.enable_blend = true
attachment.alpha_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.color_blend_op = RenderingDevice.BLEND_OP_ADD
attachment.src_color_blend_factor = RenderingDevice.BLEND_FACTOR_ONE
attachment.dst_color_blend_factor = RenderingDevice.BLEND_FACTOR_ONE_MINUS_SRC_ALPHA
attachment.src_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_ONE
attachment.dst_alpha_blend_factor = RenderingDevice.BLEND_FACTOR_ONE_MINUS_SRC_ALPHA
```

</details>

## 方法

### get_alpha_blend_op {#get_alpha_blend_op}

```gdscript
RenderingDevice.BlendOperation get_alpha_blend_op() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_color_blend_op {#get_color_blend_op}

```gdscript
RenderingDevice.BlendOperation get_color_blend_op() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dst_alpha_blend_factor {#get_dst_alpha_blend_factor}

```gdscript
RenderingDevice.BlendFactor get_dst_alpha_blend_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dst_color_blend_factor {#get_dst_color_blend_factor}

```gdscript
RenderingDevice.BlendFactor get_dst_color_blend_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_blend {#get_enable_blend}

```gdscript
bool get_enable_blend() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_src_alpha_blend_factor {#get_src_alpha_blend_factor}

```gdscript
RenderingDevice.BlendFactor get_src_alpha_blend_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_src_color_blend_factor {#get_src_color_blend_factor}

```gdscript
RenderingDevice.BlendFactor get_src_color_blend_factor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_write_a {#get_write_a}

```gdscript
bool get_write_a() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_write_b {#get_write_b}

```gdscript
bool get_write_b() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_write_g {#get_write_g}

```gdscript
bool get_write_g() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_write_r {#get_write_r}

```gdscript
bool get_write_r() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_alpha_blend_op {#set_alpha_blend_op}

```gdscript
void set_alpha_blend_op(p_member: RenderingDevice.BlendOperation)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_as_mix {#set_as_mix}

```gdscript
void set_as_mix()
```

Convenience method to perform standard mix blending with straight (non-premultiplied) alpha. This sets enable_blend to true, src_color_blend_factor to RenderingDevice.BLEND_FACTOR_SRC_ALPHA, dst_color_blend_factor to RenderingDevice.BLEND_FACTOR_ONE_MINUS_SRC_ALPHA, src_alpha_blend_factor to RenderingDevice.BLEND_FACTOR_SRC_ALPHA and dst_alpha_blend_factor to RenderingDevice.BLEND_FACTOR_ONE_MINUS_SRC_ALPHA.

### set_color_blend_op {#set_color_blend_op}

```gdscript
void set_color_blend_op(p_member: RenderingDevice.BlendOperation)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dst_alpha_blend_factor {#set_dst_alpha_blend_factor}

```gdscript
void set_dst_alpha_blend_factor(p_member: RenderingDevice.BlendFactor)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dst_color_blend_factor {#set_dst_color_blend_factor}

```gdscript
void set_dst_color_blend_factor(p_member: RenderingDevice.BlendFactor)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_blend {#set_enable_blend}

```gdscript
void set_enable_blend(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_src_alpha_blend_factor {#set_src_alpha_blend_factor}

```gdscript
void set_src_alpha_blend_factor(p_member: RenderingDevice.BlendFactor)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_src_color_blend_factor {#set_src_color_blend_factor}

```gdscript
void set_src_color_blend_factor(p_member: RenderingDevice.BlendFactor)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_write_a {#set_write_a}

```gdscript
void set_write_a(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_write_b {#set_write_b}

```gdscript
void set_write_b(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_write_g {#set_write_g}

```gdscript
void set_write_g(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_write_r {#set_write_r}

```gdscript
void set_write_r(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### enable_blend {#enable_blend}

```gdscript
enable_blend: bool
```

- void set_enable_blend(value: bool)
- bool get_enable_blend()

If true, performs blending between the source and destination according to the factors defined in src_color_blend_factor, dst_color_blend_factor, src_alpha_blend_factor and dst_alpha_blend_factor. The blend modes color_blend_op and alpha_blend_op are also taken into account, with write_r, write_g, write_b and write_a controlling the output.

### src_color_blend_factor {#src_color_blend_factor}

```gdscript
src_color_blend_factor: int
```

- void set_src_color_blend_factor(value: BlendFactor)
- BlendFactor get_src_color_blend_factor()

Controls how the blend factor for the color channels is determined based on the source's fragments.

### dst_color_blend_factor {#dst_color_blend_factor}

```gdscript
dst_color_blend_factor: int
```

- void set_dst_color_blend_factor(value: BlendFactor)
- BlendFactor get_dst_color_blend_factor()

Controls how the blend factor for the color channels is determined based on the destination's fragments.

### color_blend_op {#color_blend_op}

```gdscript
color_blend_op: int
```

- void set_color_blend_op(value: BlendOperation)
- BlendOperation get_color_blend_op()

The blend mode to use for the red/green/blue color channels.

### src_alpha_blend_factor {#src_alpha_blend_factor}

```gdscript
src_alpha_blend_factor: int
```

- void set_src_alpha_blend_factor(value: BlendFactor)
- BlendFactor get_src_alpha_blend_factor()

Controls how the blend factor for the alpha channel is determined based on the source's fragments.

### dst_alpha_blend_factor {#dst_alpha_blend_factor}

```gdscript
dst_alpha_blend_factor: int
```

- void set_dst_alpha_blend_factor(value: BlendFactor)
- BlendFactor get_dst_alpha_blend_factor()

Controls how the blend factor for the alpha channel is determined based on the destination's fragments.

### alpha_blend_op {#alpha_blend_op}

```gdscript
alpha_blend_op: int
```

- void set_alpha_blend_op(value: BlendOperation)
- BlendOperation get_alpha_blend_op()

The blend mode to use for the alpha channel.

### write_r {#write_r}

```gdscript
write_r: bool
```

- void set_write_r(value: bool)
- bool get_write_r()

If true, writes the new red color channel to the final result.

### write_g {#write_g}

```gdscript
write_g: bool
```

- void set_write_g(value: bool)
- bool get_write_g()

If true, writes the new green color channel to the final result.

### write_b {#write_b}

```gdscript
write_b: bool
```

- void set_write_b(value: bool)
- bool get_write_b()

If true, writes the new blue color channel to the final result.

### write_a {#write_a}

```gdscript
write_a: bool
```

- void set_write_a(value: bool)
- bool get_write_a()

If true, writes the new alpha channel to the final result.
