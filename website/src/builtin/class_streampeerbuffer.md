# StreamPeerBuffer

**继承**：[StreamPeer](class_streampeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StreamPeerBuffer**

> A data buffer stream peer that uses a byte array as the stream. This object can be used to handle binary data from network sessions. To handle binary data stored in files, FileAccess can be used directly.

<details><summary>官方英文描述</summary>

A data buffer stream peer that uses a byte array as the stream. This object can be used to handle binary data from network sessions. To handle binary data stored in files, FileAccess can be used directly.

A StreamPeerBuffer object keeps an internal cursor which is the offset in bytes to the start of the buffer. Get and put operations are performed at the cursor position and will move the cursor accordingly.

</details>

## 方法

### clear {#clear}

```gdscript
void clear()
```

Clears the data_array and resets the cursor.

### duplicate {#duplicate}

```gdscript
StreamPeerBuffer duplicate() const
```

Returns a new StreamPeerBuffer with the same data_array content.

### get_data_array {#get_data_array}

```gdscript
PackedByteArray get_data_array() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_position {#get_position}

```gdscript
int get_position() const
```

Returns the current cursor position.

### get_size {#get_size}

```gdscript
int get_size() const
```

Returns the size of data_array.

### resize {#resize}

```gdscript
void resize(size: int)
```

Resizes the data_array. This doesn't update the cursor.

### seek {#seek}

```gdscript
void seek(position: int)
```

Moves the cursor to the specified position. position must be a valid index of data_array.

### set_data_array {#set_data_array}

```gdscript
void set_data_array(data: PackedByteArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### data_array {#data_array}

```gdscript
data_array: PackedByteArray
```

- void set_data_array(value: PackedByteArray)
- PackedByteArray get_data_array()

The underlying data buffer. Setting this value resets the cursor.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.
