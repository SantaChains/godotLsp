# RDPipelineColorBlendState

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDPipelineColorBlendState**

> This object is used by RenderingDevice.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object is used by RenderingDevice.

</details>

## 方法

### get_attachments {#get_attachments}

```gdscript
typedarray::RDPipelineColorBlendStateAttachment get_attachments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_blend_constant {#get_blend_constant}

```gdscript
Color get_blend_constant() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_enable_logic_op {#get_enable_logic_op}

```gdscript
bool get_enable_logic_op() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_logic_op {#get_logic_op}

```gdscript
RenderingDevice.LogicOperation get_logic_op() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_attachments {#set_attachments}

```gdscript
void set_attachments(attachments: typedarray::RDPipelineColorBlendStateAttachment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_blend_constant {#set_blend_constant}

```gdscript
void set_blend_constant(p_member: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_enable_logic_op {#set_enable_logic_op}

```gdscript
void set_enable_logic_op(p_member: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_logic_op {#set_logic_op}

```gdscript
void set_logic_op(p_member: RenderingDevice.LogicOperation)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### enable_logic_op {#enable_logic_op}

```gdscript
enable_logic_op: bool
```

- void set_enable_logic_op(value: bool)
- bool get_enable_logic_op()

If true, performs the logic operation defined in logic_op.

### logic_op {#logic_op}

```gdscript
logic_op: int
```

- void set_logic_op(value: LogicOperation)
- LogicOperation get_logic_op()

The logic operation to perform for blending. Only effective if enable_logic_op is true.

### blend_constant {#blend_constant}

```gdscript
blend_constant: Color
```

- void set_blend_constant(value: Color)
- Color get_blend_constant()

The constant color to blend with. See also RenderingDevice.draw_list_set_blend_constants().

### attachments {#attachments}

```gdscript
attachments: typedarray::RDPipelineColorBlendStateAttachment
```

- void set_attachments(value: Array[RDPipelineColorBlendStateAttachment])
- Array[RDPipelineColorBlendStateAttachment] get_attachments()

The attachments that are blended together.
