# MultiplayerPeer

**继承**：[PacketPeer](class_packetpeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MultiplayerPeer**

> Manages the connection with one or more remote peers acting as server or client and assigning unique IDs to each of them. See also MultiplayerAPI.

<details><summary>官方英文描述</summary>

Manages the connection with one or more remote peers acting as server or client and assigning unique IDs to each of them. See also MultiplayerAPI.

Note: The MultiplayerAPI protocol is an implementation detail and isn't meant to be used by non-Godot servers. It may change without notice.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### close {#close}

```gdscript
void close()
```

Immediately close the multiplayer peer returning to the state CONNECTION_DISCONNECTED. Connected peers will be dropped without emitting peer_disconnected.

### disconnect_peer {#disconnect_peer}

```gdscript
void disconnect_peer(peer: int, force: bool = false)
```

Disconnects the given peer from this host. If force is true the peer_disconnected signal will not be emitted for this peer.

### generate_unique_id {#generate_unique_id}

```gdscript
int generate_unique_id() const
```

Returns a randomly generated integer that can be used as a network unique ID.

### get_connection_status {#get_connection_status}

```gdscript
MultiplayerPeer.ConnectionStatus get_connection_status() const
```

Returns the current state of the connection.

### get_packet_channel {#get_packet_channel}

```gdscript
int get_packet_channel() const
```

Returns the channel over which the next available packet was received. See PacketPeer.get_available_packet_count().

### get_packet_mode {#get_packet_mode}

```gdscript
MultiplayerPeer.TransferMode get_packet_mode() const
```

Returns the transfer mode the remote peer used to send the next available packet. See PacketPeer.get_available_packet_count().

### get_packet_peer {#get_packet_peer}

```gdscript
int get_packet_peer() const
```

Returns the ID of the MultiplayerPeer who sent the next available packet. See PacketPeer.get_available_packet_count().

### get_transfer_channel {#get_transfer_channel}

```gdscript
int get_transfer_channel() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transfer_mode {#get_transfer_mode}

```gdscript
MultiplayerPeer.TransferMode get_transfer_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_unique_id {#get_unique_id}

```gdscript
int get_unique_id() const
```

Returns the ID of this MultiplayerPeer.

### is_refusing_new_connections {#is_refusing_new_connections}

```gdscript
bool is_refusing_new_connections() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_server_relay_supported {#is_server_relay_supported}

```gdscript
bool is_server_relay_supported() const
```

Returns true if the server can act as a relay in the current configuration. That is, if the higher level MultiplayerAPI should notify connected clients of other peers, and implement a relay protocol to allow communication between them.

### poll {#poll}

```gdscript
void poll()
```

Waits up to 1 second to receive a new network event.

### set_refuse_new_connections {#set_refuse_new_connections}

```gdscript
void set_refuse_new_connections(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_target_peer {#set_target_peer}

```gdscript
void set_target_peer(id: int)
```

Sets the peer to which packets will be sent.

The id can be one of: TARGET_PEER_BROADCAST to send to all connected peers, TARGET_PEER_SERVER to send to the peer acting as server, a valid peer ID to send to that specific peer, a negative peer ID to send to all peers except that one. By default, the target peer is TARGET_PEER_BROADCAST.

### set_transfer_channel {#set_transfer_channel}

```gdscript
void set_transfer_channel(channel: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transfer_mode {#set_transfer_mode}

```gdscript
void set_transfer_mode(mode: MultiplayerPeer.TransferMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### refuse_new_connections {#refuse_new_connections}

```gdscript
refuse_new_connections: bool
```

- void set_refuse_new_connections(value: bool)
- bool is_refusing_new_connections()

If true, this MultiplayerPeer refuses new connections.

### transfer_mode {#transfer_mode}

```gdscript
transfer_mode: int
```

- void set_transfer_mode(value: TransferMode)
- TransferMode get_transfer_mode()

The manner in which to send packets to the target peer. See the set_target_peer() method.

### transfer_channel {#transfer_channel}

```gdscript
transfer_channel: int
```

- void set_transfer_channel(value: int)
- int get_transfer_channel()

The channel to use to send packets. Many network APIs such as ENet and WebRTC allow the creation of multiple independent channels which behaves, in a way, like separate connections. This means that reliable data will only block delivery of other packets on that channel, and ordering will only be in respect to the channel the packet is being sent on. Using different channels to send different and independent state updates is a common way to optimize network usage and decrease latency in fast-paced games.

Note: The default channel (0) actually works as 3 separate channels (one for each TransferMode) so that TRANSFER_MODE_RELIABLE and TRANSFER_MODE_UNRELIABLE_ORDERED does not interact with each other by default. Refer to the specific network API documentation (e.g. ENet or WebRTC) to learn how to set up channels correctly.

## 信号

### peer_connected {#peer_connected}

```gdscript
signal peer_connected(id: int)
```

Emitted when a remote peer connects.

### peer_disconnected {#peer_disconnected}

```gdscript
signal peer_disconnected(id: int)
```

Emitted when a remote peer has disconnected.

## 枚举

### ConnectionStatus {#enum-ConnectionStatus}

```gdscript
enum ConnectionStatus { CONNECTION_DISCONNECTED = 0, CONNECTION_CONNECTING = 1, CONNECTION_CONNECTED = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CONNECTION_DISCONNECTED {#CONNECTION_DISCONNECTED}

```gdscript
CONNECTION_DISCONNECTED = 0
```

The MultiplayerPeer is disconnected.

#### CONNECTION_CONNECTING {#CONNECTION_CONNECTING}

```gdscript
CONNECTION_CONNECTING = 1
```

The MultiplayerPeer is currently connecting to a server.

#### CONNECTION_CONNECTED {#CONNECTION_CONNECTED}

```gdscript
CONNECTION_CONNECTED = 2
```

This MultiplayerPeer is connected.

---

### TransferMode {#enum-TransferMode}

```gdscript
enum TransferMode { TRANSFER_MODE_UNRELIABLE = 0, TRANSFER_MODE_UNRELIABLE_ORDERED = 1, TRANSFER_MODE_RELIABLE = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSFER_MODE_UNRELIABLE {#TRANSFER_MODE_UNRELIABLE}

```gdscript
TRANSFER_MODE_UNRELIABLE = 0
```

Packets are not acknowledged, no resend attempts are made for lost packets. Packets may arrive in any order. Potentially faster than TRANSFER_MODE_UNRELIABLE_ORDERED. Use for non-critical data, and always consider whether the order matters.

#### TRANSFER_MODE_UNRELIABLE_ORDERED {#TRANSFER_MODE_UNRELIABLE_ORDERED}

```gdscript
TRANSFER_MODE_UNRELIABLE_ORDERED = 1
```

Packets are not acknowledged, no resend attempts are made for lost packets. Packets are received in the order they were sent in. Potentially faster than TRANSFER_MODE_RELIABLE. Use for non-critical data or data that would be outdated if received late due to resend attempt(s) anyway, for example movement and positional data.

#### TRANSFER_MODE_RELIABLE {#TRANSFER_MODE_RELIABLE}

```gdscript
TRANSFER_MODE_RELIABLE = 2
```

Packets must be received and resend attempts should be made until the packets are acknowledged. Packets must be received in the order they were sent in. Most reliable transfer mode, but potentially the slowest due to the overhead. Use for critical data that must be transmitted and arrive in order, for example an ability being triggered or a chat message. Consider carefully if the information really is critical, and use sparingly.

---

## 常量

### TARGET_PEER_BROADCAST {#TARGET_PEER_BROADCAST}

```gdscript
const TARGET_PEER_BROADCAST = 0
```

Packets are sent to all connected peers.

TARGET_PEER_SERVER = 1 

Packets are sent to the remote peer acting as server.

### TARGET_PEER_SERVER {#TARGET_PEER_SERVER}

```gdscript
const TARGET_PEER_SERVER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
