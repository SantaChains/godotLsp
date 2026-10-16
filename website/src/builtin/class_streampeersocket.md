# StreamPeerSocket

**继承**：[StreamPeer](class_streampeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StreamPeerSocket**

> StreamPeerSocket is an abstract base class that defines common behavior for socket-based streams.

<details><summary>官方英文描述</summary>

StreamPeerSocket is an abstract base class that defines common behavior for socket-based streams.

</details>

## 方法

### disconnect_from_host {#disconnect_from_host}

```gdscript
void disconnect_from_host()
```

Disconnects from host.

### get_status {#get_status}

```gdscript
StreamPeerSocket.Status get_status() const
```

Returns the status of the connection.

### poll {#poll}

```gdscript
Error poll()
```

Polls the socket, updating its state. See get_status().

## 枚举

### Status {#enum-Status}

```gdscript
enum Status { STATUS_NONE = 0, STATUS_CONNECTING = 1, STATUS_CONNECTED = 2, STATUS_ERROR = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STATUS_NONE {#STATUS_NONE}

```gdscript
STATUS_NONE = 0
```

The initial status of the StreamPeerSocket. This is also the status after disconnecting.

#### STATUS_CONNECTING {#STATUS_CONNECTING}

```gdscript
STATUS_CONNECTING = 1
```

A status representing a StreamPeerSocket that is connecting to a host.

#### STATUS_CONNECTED {#STATUS_CONNECTED}

```gdscript
STATUS_CONNECTED = 2
```

A status representing a StreamPeerSocket that is connected to a host.

#### STATUS_ERROR {#STATUS_ERROR}

```gdscript
STATUS_ERROR = 3
```

A status representing a StreamPeerSocket in error state.

---
