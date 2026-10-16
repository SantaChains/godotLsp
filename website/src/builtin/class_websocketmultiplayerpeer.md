# WebSocketMultiplayerPeer

**继承**：[MultiplayerPeer](class_multiplayerpeer.md) → [PacketPeer](class_packetpeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **WebSocketMultiplayerPeer**

> Base class for WebSocket server and client, allowing them to be used as multiplayer peer for the MultiplayerAPI.

<details><summary>官方英文描述</summary>

Base class for WebSocket server and client, allowing them to be used as multiplayer peer for the MultiplayerAPI.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### create_client {#create_client}

```gdscript
Error create_client(url: String, tls_client_options: TLSOptions = null)
```

Starts a new multiplayer client connecting to the given url. TLS certificates will be verified against the hostname when connecting using the wss:// protocol. You can pass the optional tls_client_options parameter to customize the trusted certification authorities, or disable the common name verification. See TLSOptions.client() and TLSOptions.client_unsafe().

Note: It is recommended to specify the scheme part of the URL, i.e. the url should start with either ws:// or wss://.

### create_server {#create_server}

```gdscript
Error create_server(port: int, bind_address: String = "*", tls_server_options: TLSOptions = null)
```

Starts a new multiplayer server listening on the given port. You can optionally specify a bind_address, and provide valid tls_server_options to use TLS. See TLSOptions.server().

### get_handshake_headers {#get_handshake_headers}

```gdscript
PackedStringArray get_handshake_headers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_handshake_timeout {#get_handshake_timeout}

```gdscript
float get_handshake_timeout() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inbound_buffer_size {#get_inbound_buffer_size}

```gdscript
int get_inbound_buffer_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_queued_packets {#get_max_queued_packets}

```gdscript
int get_max_queued_packets() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outbound_buffer_size {#get_outbound_buffer_size}

```gdscript
int get_outbound_buffer_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_peer {#get_peer}

```gdscript
WebSocketPeer get_peer(peer_id: int) const
```

Returns the WebSocketPeer associated to the given peer_id.

### get_peer_address {#get_peer_address}

```gdscript
String get_peer_address(id: int) const
```

Returns the IP address of the given peer.

### get_peer_port {#get_peer_port}

```gdscript
int get_peer_port(id: int) const
```

Returns the remote port of the given peer.

### get_supported_protocols {#get_supported_protocols}

```gdscript
PackedStringArray get_supported_protocols() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_handshake_headers {#set_handshake_headers}

```gdscript
void set_handshake_headers(protocols: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_handshake_timeout {#set_handshake_timeout}

```gdscript
void set_handshake_timeout(timeout: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inbound_buffer_size {#set_inbound_buffer_size}

```gdscript
void set_inbound_buffer_size(buffer_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_queued_packets {#set_max_queued_packets}

```gdscript
void set_max_queued_packets(max_queued_packets: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outbound_buffer_size {#set_outbound_buffer_size}

```gdscript
void set_outbound_buffer_size(buffer_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_supported_protocols {#set_supported_protocols}

```gdscript
void set_supported_protocols(protocols: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### supported_protocols {#supported_protocols}

```gdscript
supported_protocols: PackedStringArray
```

- void set_supported_protocols(value: PackedStringArray)
- PackedStringArray get_supported_protocols()

The supported WebSocket sub-protocols. See WebSocketPeer.supported_protocols for more details.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedStringArray for more details.

### handshake_headers {#handshake_headers}

```gdscript
handshake_headers: PackedStringArray
```

- void set_handshake_headers(value: PackedStringArray)
- PackedStringArray get_handshake_headers()

The extra headers to use during handshake. See WebSocketPeer.handshake_headers for more details.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedStringArray for more details.

### inbound_buffer_size {#inbound_buffer_size}

```gdscript
inbound_buffer_size: int
```

- void set_inbound_buffer_size(value: int)
- int get_inbound_buffer_size()

The inbound buffer size for connected peers. See WebSocketPeer.inbound_buffer_size for more details.

### outbound_buffer_size {#outbound_buffer_size}

```gdscript
outbound_buffer_size: int
```

- void set_outbound_buffer_size(value: int)
- int get_outbound_buffer_size()

The outbound buffer size for connected peers. See WebSocketPeer.outbound_buffer_size for more details.

### handshake_timeout {#handshake_timeout}

```gdscript
handshake_timeout: float
```

- void set_handshake_timeout(value: float)
- float get_handshake_timeout()

The maximum time each peer can stay in a connecting state before being dropped.

### max_queued_packets {#max_queued_packets}

```gdscript
max_queued_packets: int
```

- void set_max_queued_packets(value: int)
- int get_max_queued_packets()

The maximum number of queued packets for connected peers. See WebSocketPeer.max_queued_packets for more details.
