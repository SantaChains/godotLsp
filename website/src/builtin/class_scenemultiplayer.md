# SceneMultiplayer

**继承**：[MultiplayerAPI](class_multiplayerapi.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SceneMultiplayer**

> This class is the default implementation of MultiplayerAPI, used to provide multiplayer functionalities in Godot Engine.

<details><summary>官方英文描述</summary>

This class is the default implementation of MultiplayerAPI, used to provide multiplayer functionalities in Godot Engine.

This implementation supports RPCs via Node.rpc() and Node.rpc_id() and requires MultiplayerAPI.rpc() to be passed a Node (it will fail for other object types).

This implementation additionally provide SceneTree replication via the MultiplayerSpawner and MultiplayerSynchronizer nodes, and the SceneReplicationConfig resource.

Note: The high-level multiplayer API protocol is an implementation detail and isn't meant to be used by non-Godot servers. It may change without notice.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### clear {#clear}

```gdscript
void clear()
```

Clears the current SceneMultiplayer network state (you shouldn't call this unless you know what you are doing).

### complete_auth {#complete_auth}

```gdscript
Error complete_auth(id: int)
```

Mark the authentication step as completed for the remote peer identified by id. The MultiplayerAPI.peer_connected signal will be emitted for this peer once the remote side also completes the authentication. No further authentication messages are expected to be received from this peer.

If a peer disconnects before completing authentication, either due to a network issue, the auth_timeout expiring, or manually calling disconnect_peer(), the peer_authentication_failed signal will be emitted instead of MultiplayerAPI.peer_disconnected.

### disconnect_peer {#disconnect_peer}

```gdscript
void disconnect_peer(id: int)
```

Disconnects the peer identified by id, removing it from the list of connected peers, and closing the underlying connection with it.

### get_auth_callback {#get_auth_callback}

```gdscript
Callable get_auth_callback() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_auth_timeout {#get_auth_timeout}

```gdscript
float get_auth_timeout() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_authenticating_peers {#get_authenticating_peers}

```gdscript
PackedInt32Array get_authenticating_peers()
```

Returns the IDs of the peers currently trying to authenticate with this MultiplayerAPI.

### get_max_delta_packet_size {#get_max_delta_packet_size}

```gdscript
int get_max_delta_packet_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_sync_packet_size {#get_max_sync_packet_size}

```gdscript
int get_max_sync_packet_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_root_path {#get_root_path}

```gdscript
NodePath get_root_path() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_object_decoding_allowed {#is_object_decoding_allowed}

```gdscript
bool is_object_decoding_allowed() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_refusing_new_connections {#is_refusing_new_connections}

```gdscript
bool is_refusing_new_connections() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_server_relay_enabled {#is_server_relay_enabled}

```gdscript
bool is_server_relay_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### send_auth {#send_auth}

```gdscript
Error send_auth(id: int, data: PackedByteArray)
```

Sends the specified data to the remote peer identified by id as part of an authentication message. This can be used to authenticate peers, and control when MultiplayerAPI.peer_connected is emitted (and the remote peer accepted as one of the connected peers).

### send_bytes {#send_bytes}

```gdscript
Error send_bytes(bytes: PackedByteArray, id: int = 0, mode: MultiplayerPeer.TransferMode = 2, channel: int = 0)
```

Sends the given raw bytes to a specific peer identified by id (see MultiplayerPeer.set_target_peer()). Default ID is 0, i.e. broadcast to all peers.

### set_allow_object_decoding {#set_allow_object_decoding}

```gdscript
void set_allow_object_decoding(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auth_callback {#set_auth_callback}

```gdscript
void set_auth_callback(callback: Callable)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_auth_timeout {#set_auth_timeout}

```gdscript
void set_auth_timeout(timeout: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_delta_packet_size {#set_max_delta_packet_size}

```gdscript
void set_max_delta_packet_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_sync_packet_size {#set_max_sync_packet_size}

```gdscript
void set_max_sync_packet_size(size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_refuse_new_connections {#set_refuse_new_connections}

```gdscript
void set_refuse_new_connections(refuse: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_root_path {#set_root_path}

```gdscript
void set_root_path(path: NodePath)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_server_relay_enabled {#set_server_relay_enabled}

```gdscript
void set_server_relay_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### root_path {#root_path}

```gdscript
root_path: NodePath
```

- void set_root_path(value: NodePath)
- NodePath get_root_path()

The root path to use for RPCs and replication. Instead of an absolute path, a relative path will be used to find the node upon which the RPC should be executed.

This effectively allows to have different branches of the scene tree to be managed by different MultiplayerAPI, allowing for example to run both client and server in the same scene.

### auth_callback {#auth_callback}

```gdscript
auth_callback: Callable
```

- void set_auth_callback(value: Callable)
- Callable get_auth_callback()

The callback to execute when receiving authentication data sent via send_auth(). If the Callable is empty (default), peers will be automatically accepted as soon as they connect.

### auth_timeout {#auth_timeout}

```gdscript
auth_timeout: float
```

- void set_auth_timeout(value: float)
- float get_auth_timeout()

If set to a value greater than 0.0, the maximum duration in seconds peers can stay in the authenticating state, after which the authentication will automatically fail. See the peer_authenticating and peer_authentication_failed signals.

### allow_object_decoding {#allow_object_decoding}

```gdscript
allow_object_decoding: bool
```

- void set_allow_object_decoding(value: bool)
- bool is_object_decoding_allowed()

If true, the MultiplayerAPI will allow encoding and decoding of object during RPCs.

Warning: Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threat such as remote code execution.

### refuse_new_connections {#refuse_new_connections}

```gdscript
refuse_new_connections: bool
```

- void set_refuse_new_connections(value: bool)
- bool is_refusing_new_connections()

If true, the MultiplayerAPI's MultiplayerAPI.multiplayer_peer refuses new incoming connections.

### server_relay {#server_relay}

```gdscript
server_relay: bool
```

- void set_server_relay_enabled(value: bool)
- bool is_server_relay_enabled()

Enable or disable the server feature that notifies clients of other peers' connection/disconnection, and relays messages between them. When this option is false, clients won't be automatically notified of other peers and won't be able to send them packets through the server.

Note: Changing this option while other peers are connected may lead to unexpected behaviors.

Note: Support for this feature may depend on the current MultiplayerPeer configuration. See MultiplayerPeer.is_server_relay_supported().

### max_sync_packet_size {#max_sync_packet_size}

```gdscript
max_sync_packet_size: int
```

- void set_max_sync_packet_size(value: int)
- int get_max_sync_packet_size()

Maximum size of each synchronization packet. Higher values increase the chance of receiving full updates in a single frame, but also the chance of packet loss. See MultiplayerSynchronizer.

### max_delta_packet_size {#max_delta_packet_size}

```gdscript
max_delta_packet_size: int
```

- void set_max_delta_packet_size(value: int)
- int get_max_delta_packet_size()

Maximum size of each delta packet. Higher values increase the chance of receiving full updates in a single frame, but also the chance of causing networking congestion (higher latency, disconnections). See MultiplayerSynchronizer.

## 信号

### peer_authenticating {#peer_authenticating}

```gdscript
signal peer_authenticating(id: int)
```

Emitted when this MultiplayerAPI's MultiplayerAPI.multiplayer_peer connects to a new peer and a valid auth_callback is set. In this case, the MultiplayerAPI.peer_connected will not be emitted until complete_auth() is called with given peer id. While in this state, the peer will not be included in the list returned by MultiplayerAPI.get_peers() (but in the one returned by get_authenticating_peers()), and only authentication data will be sent or received. See send_auth() for sending authentication data.

### peer_authentication_failed {#peer_authentication_failed}

```gdscript
signal peer_authentication_failed(id: int)
```

Emitted when this MultiplayerAPI's MultiplayerAPI.multiplayer_peer disconnects from a peer for which authentication had not yet completed. See peer_authenticating.

### peer_packet {#peer_packet}

```gdscript
signal peer_packet(id: int, packet: PackedByteArray)
```

Emitted when this MultiplayerAPI's MultiplayerAPI.multiplayer_peer receives a packet with custom data (see send_bytes()). ID is the peer ID of the peer that sent the packet.
