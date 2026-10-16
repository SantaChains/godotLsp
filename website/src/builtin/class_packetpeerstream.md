# PacketPeerStream

**继承**：[PacketPeer](class_packetpeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PacketPeerStream**

> PacketStreamPeer provides a wrapper for working using packets over a stream. This allows for using packet based code with StreamPeers. PacketPeerStream implements a custom protocol over the StreamPeer, so the user should not read or write to the wrapped StreamPeer directly.

<details><summary>官方英文描述（中文精译进行中）</summary>

PacketStreamPeer provides a wrapper for working using packets over a stream. This allows for using packet based code with StreamPeers. PacketPeerStream implements a custom protocol over the StreamPeer, so the user should not read or write to the wrapped StreamPeer directly.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### get_input_buffer_max_size {#get_input_buffer_max_size}

```gdscript
int get_input_buffer_max_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_output_buffer_max_size {#get_output_buffer_max_size}

```gdscript
int get_output_buffer_max_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream_peer {#get_stream_peer}

```gdscript
StreamPeer get_stream_peer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_buffer_max_size {#set_input_buffer_max_size}

```gdscript
void set_input_buffer_max_size(max_size_bytes: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_output_buffer_max_size {#set_output_buffer_max_size}

```gdscript
void set_output_buffer_max_size(max_size_bytes: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stream_peer {#set_stream_peer}

```gdscript
void set_stream_peer(peer: StreamPeer)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### input_buffer_max_size {#input_buffer_max_size}

```gdscript
input_buffer_max_size: int
```

- void set_input_buffer_max_size(value: int)
- int get_input_buffer_max_size()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### output_buffer_max_size {#output_buffer_max_size}

```gdscript
output_buffer_max_size: int
```

- void set_output_buffer_max_size(value: int)
- int get_output_buffer_max_size()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### stream_peer {#stream_peer}

```gdscript
stream_peer: StreamPeer
```

- void set_stream_peer(value: StreamPeer)
- StreamPeer get_stream_peer()

The wrapped StreamPeer object.
