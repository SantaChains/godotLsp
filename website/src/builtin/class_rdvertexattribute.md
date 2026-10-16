# RDVertexAttribute

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDVertexAttribute**

> This object is used by RenderingDevice.

<details><summary>官方英文描述（中文精译进行中）</summary>

This object is used by RenderingDevice.

</details>

## 方法

### get_binding {#get_binding}

```gdscript
int get_binding() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_format {#get_format}

```gdscript
RenderingDevice.DataFormat get_format() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_frequency {#get_frequency}

```gdscript
RenderingDevice.VertexFrequency get_frequency() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_location {#get_location}

```gdscript
int get_location() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_offset {#get_offset}

```gdscript
int get_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stride {#get_stride}

```gdscript
int get_stride() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_binding {#set_binding}

```gdscript
void set_binding(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_format {#set_format}

```gdscript
void set_format(p_member: RenderingDevice.DataFormat)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_frequency {#set_frequency}

```gdscript
void set_frequency(p_member: RenderingDevice.VertexFrequency)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_location {#set_location}

```gdscript
void set_location(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_offset {#set_offset}

```gdscript
void set_offset(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stride {#set_stride}

```gdscript
void set_stride(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### binding {#binding}

```gdscript
binding: int
```

- void set_binding(value: int)
- int get_binding()

The index of the buffer in the vertex buffer array to bind this vertex attribute. When set to -1, it defaults to the index of the attribute.

Note: You cannot mix binding explicitly assigned attributes with implicitly assigned ones (i.e. -1). Either all attributes must have their binding set to -1, or all must have explicit bindings.

### location {#location}

```gdscript
location: int
```

- void set_location(value: int)
- int get_location()

The location in the shader that this attribute is bound to.

### offset {#offset}

```gdscript
offset: int
```

- void set_offset(value: int)
- int get_offset()

The number of bytes between the start of the vertex buffer and the first instance of this attribute.

### format {#format}

```gdscript
format: int
```

- void set_format(value: DataFormat)
- DataFormat get_format()

The way that this attribute's data is interpreted when sent to a shader.

### stride {#stride}

```gdscript
stride: int
```

- void set_stride(value: int)
- int get_stride()

The number of bytes between the starts of consecutive instances of this attribute.

### frequency {#frequency}

```gdscript
frequency: int
```

- void set_frequency(value: VertexFrequency)
- VertexFrequency get_frequency()

The rate at which this attribute is pulled from its vertex buffer.
