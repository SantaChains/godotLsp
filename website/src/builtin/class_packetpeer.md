# PacketPeer

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **PacketPeer**

> PacketPeer is an abstraction and base class for packet-based protocols (such as UDP). It provides an API for sending and receiving packets both as raw data or variables. This makes it easy to transfer data over a protocol, without having to encode data as low-level bytes or having to worry about network ordering.

<details><summary>官方英文描述</summary>

PacketPeer is an abstraction and base class for packet-based protocols (such as UDP). It provides an API for sending and receiving packets both as raw data or variables. This makes it easy to transfer data over a protocol, without having to encode data as low-level bytes or having to worry about network ordering.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### get_available_packet_count {#get_available_packet_count}

```gdscript
int get_available_packet_count() const
```

Returns the number of packets currently available in the ring-buffer.

### get_encode_buffer_max_size {#get_encode_buffer_max_size}

```gdscript
int get_encode_buffer_max_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_packet {#get_packet}

```gdscript
PackedByteArray get_packet()
```

Gets a raw packet.

### get_packet_error {#get_packet_error}

```gdscript
Error get_packet_error() const
```

Returns the error state of the last packet received (via get_packet() and get_var()).

### get_var {#get_var}

```gdscript
Variant get_var(allow_objects: bool = false)
```

Gets a Variant. If allow_objects is true, decoding objects is allowed.

Internally, this uses the same decoding mechanism as the @GlobalScope.bytes_to_var() method.

Warning: Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution.

### put_packet {#put_packet}

```gdscript
Error put_packet(buffer: PackedByteArray)
```

Sends a raw packet.

### put_var {#put_var}

```gdscript
Error put_var(var: Variant, full_objects: bool = false)
```

Sends a Variant as a packet. If full_objects is true, encoding objects is allowed (and can potentially include code).

Internally, this uses the same encoding mechanism as the @GlobalScope.var_to_bytes() method.

### set_encode_buffer_max_size {#set_encode_buffer_max_size}

```gdscript
void set_encode_buffer_max_size(max_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### encode_buffer_max_size {#encode_buffer_max_size}

```gdscript
encode_buffer_max_size: int
```

- void set_encode_buffer_max_size(value: int)
- int get_encode_buffer_max_size()

Maximum buffer size allowed when encoding Variants. Raise this value to support heavier memory allocations.

The put_var() method allocates memory on the stack, and the buffer used will grow automatically to the closest power of two to match the size of the Variant. If the Variant is bigger than encode_buffer_max_size, the method will error out with @GlobalScope.ERR_OUT_OF_MEMORY.
