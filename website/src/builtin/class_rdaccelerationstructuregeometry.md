# RDAccelerationStructureGeometry

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **RDAccelerationStructureGeometry**

> RDAccelerationStructureGeometry describes a set of triangles used as raytracing geometry in the RenderingDevice.blas_create() method.

<details><summary>官方英文描述（中文精译进行中）</summary>

RDAccelerationStructureGeometry describes a set of triangles used as raytracing geometry in the RenderingDevice.blas_create() method.

The geometry is always in triangle list form, either indexed or non-indexed. Triangle strips are not supported.

</details>

## 方法

### get_flags {#get_flags}

```gdscript
bitfield::RenderingDevice.AccelerationStructureGeometryFlagBits get_flags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_index_buffer {#get_index_buffer}

```gdscript
RID get_index_buffer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_index_count {#get_index_count}

```gdscript
int get_index_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_index_offset {#get_index_offset}

```gdscript
int get_index_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertex_buffer {#get_vertex_buffer}

```gdscript
RID get_vertex_buffer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertex_count {#get_vertex_count}

```gdscript
int get_vertex_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertex_format {#get_vertex_format}

```gdscript
RenderingDevice.DataFormat get_vertex_format() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertex_offset {#get_vertex_offset}

```gdscript
int get_vertex_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertex_stride {#get_vertex_stride}

```gdscript
int get_vertex_stride() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_flags {#set_flags}

```gdscript
void set_flags(p_member: bitfield::RenderingDevice.AccelerationStructureGeometryFlagBits)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_index_buffer {#set_index_buffer}

```gdscript
void set_index_buffer(p_member: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_index_count {#set_index_count}

```gdscript
void set_index_count(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_index_offset {#set_index_offset}

```gdscript
void set_index_offset(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertex_buffer {#set_vertex_buffer}

```gdscript
void set_vertex_buffer(p_member: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertex_count {#set_vertex_count}

```gdscript
void set_vertex_count(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertex_format {#set_vertex_format}

```gdscript
void set_vertex_format(p_member: RenderingDevice.DataFormat)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertex_offset {#set_vertex_offset}

```gdscript
void set_vertex_offset(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertex_stride {#set_vertex_stride}

```gdscript
void set_vertex_stride(p_member: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### flags {#flags}

```gdscript
flags: int
```

- void set_flags(value: BitField[AccelerationStructureGeometryFlagBits])
- BitField[AccelerationStructureGeometryFlagBits] get_flags()

Flags for the geometry.

### vertex_buffer {#vertex_buffer}

```gdscript
vertex_buffer: RID
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### vertex_offset {#vertex_offset}

```gdscript
vertex_offset: int
```

- void set_vertex_offset(value: int)
- int get_vertex_offset()

Byte offset of the first vertex in vertex_buffer.

### vertex_stride {#vertex_stride}

```gdscript
vertex_stride: int
```

- void set_vertex_stride(value: int)
- int get_vertex_stride()

Number of bytes between each vertex in vertex_buffer.

### vertex_count {#vertex_count}

```gdscript
vertex_count: int
```

- void set_vertex_count(value: int)
- int get_vertex_count()

Number of vertices used by this geometry in vertex_buffer.

### vertex_format {#vertex_format}

```gdscript
vertex_format: int
```

- void set_vertex_format(value: DataFormat)
- DataFormat get_vertex_format()

Format of the vertices in vertex_buffer.

### index_buffer {#index_buffer}

```gdscript
index_buffer: RID
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### index_offset {#index_offset}

```gdscript
index_offset: int
```

- void set_index_offset(value: int)
- int get_index_offset()

Byte offset of the first index in index_buffer.

### index_count {#index_count}

```gdscript
index_count: int
```

- void set_index_count(value: int)
- int get_index_count()

Number of indices used by this geometry in index_buffer.
