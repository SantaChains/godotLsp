# MultiplayerAPI

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **MultiplayerAPI**

> Base class for high-level multiplayer API implementations. See also MultiplayerPeer.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base class for high-level multiplayer API implementations. See also MultiplayerPeer.

By default, SceneTree has a reference to an implementation of this class and uses it to provide multiplayer capabilities (i.e. RPCs) across the whole scene.

It is possible to override the MultiplayerAPI instance used by specific tree branches by calling the SceneTree.set_multiplayer() method, effectively allowing to run both client and server in the same scene.

It is also possible to extend or replace the default implementation via scripting or native extensions. See MultiplayerAPIExtension for details about extensions, SceneMultiplayer for the details about the default implementation.

</details>

## 方法

### create_default_interface {#create_default_interface}

```gdscript
static MultiplayerAPI create_default_interface()
```

Returns a new instance of the default MultiplayerAPI.

### get_default_interface {#get_default_interface}

```gdscript
static StringName get_default_interface()
```

Returns the default MultiplayerAPI implementation class name. This is usually "SceneMultiplayer" when SceneMultiplayer is available. See set_default_interface().

### get_multiplayer_peer {#get_multiplayer_peer}

```gdscript
MultiplayerPeer get_multiplayer_peer()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_peers {#get_peers}

```gdscript
PackedInt32Array get_peers()
```

Returns the peer IDs of all connected peers of this MultiplayerAPI's multiplayer_peer.

### get_remote_sender_id {#get_remote_sender_id}

```gdscript
int get_remote_sender_id()
```

Returns the sender's peer ID for the RPC currently being executed.

Note: This method returns 0 when called outside of an RPC. As such, the original peer ID may be lost when code execution is delayed (such as with GDScript's await keyword).

### get_unique_id {#get_unique_id}

```gdscript
int get_unique_id()
```

Returns the unique peer ID of this MultiplayerAPI's multiplayer_peer.

### has_multiplayer_peer {#has_multiplayer_peer}

```gdscript
bool has_multiplayer_peer()
```

Returns true if there is a multiplayer_peer set.

### is_server {#is_server}

```gdscript
bool is_server()
```

Returns true if this MultiplayerAPI's multiplayer_peer is valid and in server mode (listening for connections).

### object_configuration_add {#object_configuration_add}

```gdscript
Error object_configuration_add(object: Object, configuration: Variant)
```

Notifies the MultiplayerAPI of a new configuration for the given object. This method is used internally by SceneTree to configure the root path for this MultiplayerAPI (passing null and a valid NodePath as configuration). This method can be further used by MultiplayerAPI implementations to provide additional features, refer to specific implementation (e.g. SceneMultiplayer) for details on how they use it.

Note: This method is mostly relevant when extending or overriding the MultiplayerAPI behavior via MultiplayerAPIExtension.

### object_configuration_remove {#object_configuration_remove}

```gdscript
Error object_configuration_remove(object: Object, configuration: Variant)
```

Notifies the MultiplayerAPI to remove a configuration for the given object. This method is used internally by SceneTree to configure the root path for this MultiplayerAPI (passing null and an empty NodePath as configuration). This method can be further used by MultiplayerAPI implementations to provide additional features, refer to specific implementation (e.g. SceneMultiplayer) for details on how they use it.

Note: This method is mostly relevant when extending or overriding the MultiplayerAPI behavior via MultiplayerAPIExtension.

### poll {#poll}

```gdscript
Error poll()
```

Method used for polling the MultiplayerAPI. You only need to worry about this if you set SceneTree.multiplayer_poll to false. By default, SceneTree will poll its MultiplayerAPI(s) for you.

Note: This method results in RPCs being called, so they will be executed in the same context of this function (e.g. _process, physics, Thread).

### rpc {#rpc}

```gdscript
Error rpc(peer: int, object: Object, method: StringName, arguments: Array = [])
```

Sends an RPC to the target peer. The given method will be called on the remote object with the provided arguments. The RPC may also be called locally depending on the implementation and RPC configuration. See Node.rpc() and Node.rpc_config().

Note: Prefer using Node.rpc(), Node.rpc_id(), or my_method.rpc(peer, arg1, arg2, ...) (in GDScript), since they are faster. This method is mostly useful in conjunction with MultiplayerAPIExtension when extending or replacing the multiplayer capabilities.

### set_default_interface {#set_default_interface}

```gdscript
static void set_default_interface(interface_name: StringName)
```

Sets the default MultiplayerAPI implementation class. This method can be used by modules and extensions to configure which implementation will be used by SceneTree when the engine starts.

### set_multiplayer_peer {#set_multiplayer_peer}

```gdscript
void set_multiplayer_peer(peer: MultiplayerPeer)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### multiplayer_peer {#multiplayer_peer}

```gdscript
multiplayer_peer: MultiplayerPeer
```

- void set_multiplayer_peer(value: MultiplayerPeer)
- MultiplayerPeer get_multiplayer_peer()

The peer object to handle the RPC system (effectively enabling networking when set). Depending on the peer itself, the MultiplayerAPI will become a network server (check with is_server()) and will set root node's network mode to authority, or it will become a regular client peer. All child nodes are set to inherit the network mode by default. Handling of networking-related events (connection, disconnection, new clients) is done by connecting to MultiplayerAPI's signals.

## 信号

### peer_connected {#peer_connected}

```gdscript
signal peer_connected(id: int)
```

Emitted when this MultiplayerAPI's multiplayer_peer connects with a new peer. ID is the peer ID of the new peer. Clients get notified when other clients connect to the same server. Upon connecting to a server, a client also receives this signal for the server (with ID being 1).

### peer_disconnected {#peer_disconnected}

```gdscript
signal peer_disconnected(id: int)
```

Emitted when this MultiplayerAPI's multiplayer_peer disconnects from a peer. Clients get notified when other clients disconnect from the same server.

### connected_to_server {#connected_to_server}

```gdscript
signal connected_to_server()
```

Emitted when this MultiplayerAPI's multiplayer_peer successfully connected to a server. Only emitted on clients.

### connection_failed {#connection_failed}

```gdscript
signal connection_failed()
```

Emitted when this MultiplayerAPI's multiplayer_peer fails to establish a connection to a server. Only emitted on clients.

### server_disconnected {#server_disconnected}

```gdscript
signal server_disconnected()
```

Emitted when this MultiplayerAPI's multiplayer_peer disconnects from server. Only emitted on clients.

## 枚举

### RPCMode {#enum-RPCMode}

```gdscript
enum RPCMode { RPC_MODE_DISABLED = 0, RPC_MODE_ANY_PEER = 1, RPC_MODE_AUTHORITY = 2 }
```

RPCMode RPC_MODE_DISABLED = 0

Used with Node.rpc_config() to disable a method or property for all RPC calls, making it unavailable. Default for all methods.

RPCMode RPC_MODE_ANY_PEER = 1

Used with Node.rpc_config() to set a method to be callable remotely by any peer. Analogous to the @rpc("any_peer") annotation. Calls are accepted from all remote peers, no matter if they are node's authority or not.

RPCMode RPC_MODE_AUTHORITY = 2

Used with Node.rpc_config() to set a method to be callable remotely only by the current multiplayer authority (which is the server by default). Analogous to the @rpc("authority") annotation. See Node.set_multiplayer_authority().

#### RPC_MODE_DISABLED {#RPC_MODE_DISABLED}

```gdscript
RPC_MODE_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RPC_MODE_ANY_PEER {#RPC_MODE_ANY_PEER}

```gdscript
RPC_MODE_ANY_PEER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RPC_MODE_AUTHORITY {#RPC_MODE_AUTHORITY}

```gdscript
RPC_MODE_AUTHORITY = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
