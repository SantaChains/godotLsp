# RDFramebufferPass

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDFramebufferPass**

> This class contains the list of attachment descriptions for a framebuffer pass. Each points with an index to a previously supplied list of texture attachments.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class contains the list of attachment descriptions for a framebuffer pass. Each points with an index to a previously supplied list of texture attachments.

Multipass framebuffers can optimize some configurations in mobile. On desktop, they provide little to no advantage.

This object is used by RenderingDevice.

</details>

## 方法

### get_color_attachments {#get_color_attachments}

```gdscript
PackedInt32Array get_color_attachments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_depth_attachment {#get_depth_attachment}

```gdscript
int get_depth_attachment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_input_attachments {#get_input_attachments}

```gdscript
PackedInt32Array get_input_attachments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_preserve_attachments {#get_preserve_attachments}

```gdscript
PackedInt32Array get_preserve_attachments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_resolve_attachments {#get_resolve_attachments}

```gdscript
PackedInt32Array get_resolve_attachments() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_color_attachments {#set_color_attachments}

```gdscript
void set_color_attachments(p_member: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_depth_attachment {#set_depth_attachment}

```gdscript
void set_depth_attachment(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_attachments {#set_input_attachments}

```gdscript
void set_input_attachments(p_member: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_preserve_attachments {#set_preserve_attachments}

```gdscript
void set_preserve_attachments(p_member: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_resolve_attachments {#set_resolve_attachments}

```gdscript
void set_resolve_attachments(p_member: PackedInt32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### color_attachments {#color_attachments}

```gdscript
color_attachments: PackedInt32Array
```

- void set_color_attachments(value: PackedInt32Array)
- PackedInt32Array get_color_attachments()

Color attachments in order starting from 0. If this attachment is not used by the shader, pass ATTACHMENT_UNUSED to skip.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### input_attachments {#input_attachments}

```gdscript
input_attachments: PackedInt32Array
```

- void set_input_attachments(value: PackedInt32Array)
- PackedInt32Array get_input_attachments()

Used for multipass framebuffers (more than one render pass). Converts an attachment to an input. Make sure to also supply it properly in the RDUniform for the uniform set.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### resolve_attachments {#resolve_attachments}

```gdscript
resolve_attachments: PackedInt32Array
```

- void set_resolve_attachments(value: PackedInt32Array)
- PackedInt32Array get_resolve_attachments()

If the color attachments are multisampled, non-multisampled resolve attachments can be provided.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### preserve_attachments {#preserve_attachments}

```gdscript
preserve_attachments: PackedInt32Array
```

- void set_preserve_attachments(value: PackedInt32Array)
- PackedInt32Array get_preserve_attachments()

Attachments to preserve in this pass (otherwise they are erased).

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt32Array for more details.

### depth_attachment {#depth_attachment}

```gdscript
depth_attachment: int
```

- void set_depth_attachment(value: int)
- int get_depth_attachment()

Depth attachment. ATTACHMENT_UNUSED should be used if no depth buffer is required for this pass.

## 常量

### ATTACHMENT_UNUSED {#ATTACHMENT_UNUSED}

```gdscript
const ATTACHMENT_UNUSED = -1
```

Attachment is unused.
