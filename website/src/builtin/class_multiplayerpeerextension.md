# MultiplayerPeerExtension

**继承**：[MultiplayerPeer](class_multiplayerpeer.md) → [PacketPeer](class_packetpeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MultiplayerPeerExtension**

> This class is designed to be inherited from a GDExtension plugin to implement custom networking layers for the multiplayer API (such as WebRTC). All the methods below must be implemented to have a working custom multiplayer implementation. See also MultiplayerAPI.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class is designed to be inherited from a GDExtension plugin to implement custom networking layers for the multiplayer API (such as WebRTC). All the methods below must be implemented to have a working custom multiplayer implementation. See also MultiplayerAPI.

</details>

## 方法

### _close {#_close}

```gdscript
void _close() virtual
```

Called when the multiplayer peer should be immediately closed (see MultiplayerPeer.close()).

### _disconnect_peer {#_disconnect_peer}

```gdscript
void _disconnect_peer(peer: int, force: bool) virtual
```

Called when the connected peer should be forcibly disconnected (see MultiplayerPeer.disconnect_peer()).

### _get_available_packet_count {#_get_available_packet_count}

```gdscript
int _get_available_packet_count() virtual const
```

Called when the available packet count is internally requested by the MultiplayerAPI.

### _get_connection_status {#_get_connection_status}

```gdscript
MultiplayerPeer.ConnectionStatus _get_connection_status() virtual const
```

Called when the connection status is requested on the MultiplayerPeer (see MultiplayerPeer.get_connection_status()).

### _get_max_packet_size {#_get_max_packet_size}

```gdscript
int _get_max_packet_size() virtual const
```

Called when the maximum allowed packet size (in bytes) is requested by the MultiplayerAPI.

### _get_packet {#_get_packet}

```gdscript
Error _get_packet(r_buffer: const uint8_t **, r_buffer_size: int32_t*) virtual
```

Called when a packet needs to be received by the MultiplayerAPI, with r_buffer_size being the size of the binary r_buffer in bytes.

### _get_packet_channel {#_get_packet_channel}

```gdscript
int _get_packet_channel() virtual const
```

Called to get the channel over which the next available packet was received. See MultiplayerPeer.get_packet_channel().

### _get_packet_mode {#_get_packet_mode}

```gdscript
MultiplayerPeer.TransferMode _get_packet_mode() virtual const
```

Called to get the transfer mode the remote peer used to send the next available packet. See MultiplayerPeer.get_packet_mode().

### _get_packet_peer {#_get_packet_peer}

```gdscript
int _get_packet_peer() virtual const
```

Called when the ID of the MultiplayerPeer who sent the most recent packet is requested (see MultiplayerPeer.get_packet_peer()).

### _get_packet_script {#_get_packet_script}

```gdscript
PackedByteArray _get_packet_script() virtual
```

Called when a packet needs to be received by the MultiplayerAPI, if _get_packet() isn't implemented. Use this when extending this class via GDScript.

### _get_transfer_channel {#_get_transfer_channel}

```gdscript
int _get_transfer_channel() virtual const
```

Called when the transfer channel to use is read on this MultiplayerPeer (see MultiplayerPeer.transfer_channel).

### _get_transfer_mode {#_get_transfer_mode}

```gdscript
MultiplayerPeer.TransferMode _get_transfer_mode() virtual const
```

Called when the transfer mode to use is read on this MultiplayerPeer (see MultiplayerPeer.transfer_mode).

### _get_unique_id {#_get_unique_id}

```gdscript
int _get_unique_id() virtual const
```

Called when the unique ID of this MultiplayerPeer is requested (see MultiplayerPeer.get_unique_id()). The value must be between 1 and 2147483647.

### _is_refusing_new_connections {#_is_refusing_new_connections}

```gdscript
bool _is_refusing_new_connections() virtual const
```

Called when the "refuse new connections" status is requested on this MultiplayerPeer (see MultiplayerPeer.refuse_new_connections).

### _is_server {#_is_server}

```gdscript
bool _is_server() virtual const
```

Called when the "is server" status is requested on the MultiplayerAPI. See MultiplayerAPI.is_server().

### _is_server_relay_supported {#_is_server_relay_supported}

```gdscript
bool _is_server_relay_supported() virtual const
```

Called to check if the server can act as a relay in the current configuration. See MultiplayerPeer.is_server_relay_supported().

### _poll {#_poll}

```gdscript
void _poll() virtual
```

Called when the MultiplayerAPI is polled. See MultiplayerAPI.poll().

### _put_packet {#_put_packet}

```gdscript
Error _put_packet(buffer: const uint8_t*, buffer_size: int) virtual
```

Called when a packet needs to be sent by the MultiplayerAPI, with buffer_size being the size of the binary buffer in bytes.

### _put_packet_script {#_put_packet_script}

```gdscript
Error _put_packet_script(buffer: PackedByteArray) virtual
```

Called when a packet needs to be sent by the MultiplayerAPI, if _put_packet() isn't implemented. Use this when extending this class via GDScript.

### _set_refuse_new_connections {#_set_refuse_new_connections}

```gdscript
void _set_refuse_new_connections(enable: bool) virtual
```

Called when the "refuse new connections" status is set on this MultiplayerPeer (see MultiplayerPeer.refuse_new_connections).

### _set_target_peer {#_set_target_peer}

```gdscript
void _set_target_peer(peer: int) virtual
```

Called when the target peer to use is set for this MultiplayerPeer (see MultiplayerPeer.set_target_peer()).

### _set_transfer_channel {#_set_transfer_channel}

```gdscript
void _set_transfer_channel(channel: int) virtual
```

Called when the channel to use is set for this MultiplayerPeer (see MultiplayerPeer.transfer_channel).

### _set_transfer_mode {#_set_transfer_mode}

```gdscript
void _set_transfer_mode(mode: MultiplayerPeer.TransferMode) virtual
```

Called when the transfer mode is set on this MultiplayerPeer (see MultiplayerPeer.transfer_mode).
