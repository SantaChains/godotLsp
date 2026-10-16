# GLTFBufferView

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFBufferView**

> GLTFBufferView is a data structure representing a glTF bufferView that would be found in the "bufferViews" array. A buffer is a blob of binary data. A buffer view is a slice of a buffer that can be used to identify and extract data from the buffer.

<details><summary>官方英文描述（中文精译进行中）</summary>

GLTFBufferView is a data structure representing a glTF bufferView that would be found in the "bufferViews" array. A buffer is a blob of binary data. A buffer view is a slice of a buffer that can be used to identify and extract data from the buffer.

Most custom uses of buffers only need to use the buffer, byte_length, and byte_offset. The byte_stride and indices properties are for more advanced use cases such as interleaved mesh data encoded for the GPU.

</details>

## 方法

### from_dictionary {#from_dictionary}

```gdscript
static GLTFBufferView from_dictionary(dictionary: Dictionary)
```

Creates a new GLTFBufferView instance by parsing the given Dictionary.

### get_buffer {#get_buffer}

```gdscript
int get_buffer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_byte_length {#get_byte_length}

```gdscript
int get_byte_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_byte_offset {#get_byte_offset}

```gdscript
int get_byte_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_byte_stride {#get_byte_stride}

```gdscript
int get_byte_stride() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_indices {#get_indices}

```gdscript
bool get_indices() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_vertex_attributes {#get_vertex_attributes}

```gdscript
bool get_vertex_attributes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### load_buffer_view_data {#load_buffer_view_data}

```gdscript
PackedByteArray load_buffer_view_data(state: GLTFState) const
```

Loads the buffer view data from the buffer referenced by this buffer view in the given GLTFState. Interleaved data with a byte stride is not yet supported by this method. The data is returned as a PackedByteArray.

### set_buffer {#set_buffer}

```gdscript
void set_buffer(buffer: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_byte_length {#set_byte_length}

```gdscript
void set_byte_length(byte_length: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_byte_offset {#set_byte_offset}

```gdscript
void set_byte_offset(byte_offset: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_byte_stride {#set_byte_stride}

```gdscript
void set_byte_stride(byte_stride: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_indices {#set_indices}

```gdscript
void set_indices(indices: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_vertex_attributes {#set_vertex_attributes}

```gdscript
void set_vertex_attributes(is_attributes: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to_dictionary {#to_dictionary}

```gdscript
Dictionary to_dictionary() const
```

Serializes this GLTFBufferView instance into a Dictionary.

## 属性

### buffer {#buffer}

```gdscript
buffer: int
```

- void set_buffer(value: int)
- int get_buffer()

The index of the buffer this buffer view is referencing. If -1, this buffer view is not referencing any buffer.

### byte_offset {#byte_offset}

```gdscript
byte_offset: int
```

- void set_byte_offset(value: int)
- int get_byte_offset()

The offset, in bytes, from the start of the buffer to the start of this buffer view.

### byte_length {#byte_length}

```gdscript
byte_length: int
```

- void set_byte_length(value: int)
- int get_byte_length()

The length, in bytes, of this buffer view. If 0, this buffer view is empty.

### byte_stride {#byte_stride}

```gdscript
byte_stride: int
```

- void set_byte_stride(value: int)
- int get_byte_stride()

The stride, in bytes, between interleaved data. If -1, this buffer view is not interleaved.

### indices {#indices}

```gdscript
indices: bool
```

- void set_indices(value: bool)
- bool get_indices()

true if the GLTFBufferView's OpenGL GPU buffer type is an ELEMENT_ARRAY_BUFFER used for vertex indices (integer constant 34963). false if the buffer type is any other value. See Buffers, BufferViews, and Accessors [https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_005_BuffersBufferViewsAccessors.md] for possible values. This property is set on import and used on export.

### vertex_attributes {#vertex_attributes}

```gdscript
vertex_attributes: bool
```

- void set_vertex_attributes(value: bool)
- bool get_vertex_attributes()

true if the GLTFBufferView's OpenGL GPU buffer type is an ARRAY_BUFFER used for vertex attributes (integer constant 34962). false if the buffer type is any other value. See Buffers, BufferViews, and Accessors [https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_005_BuffersBufferViewsAccessors.md] for possible values. This property is set on import and used on export.
