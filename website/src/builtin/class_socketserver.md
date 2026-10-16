# SocketServer

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **SocketServer**

> A socket server.

<details><summary>官方英文描述</summary>

A socket server.

</details>

## 方法

### is_connection_available {#is_connection_available}

```gdscript
bool is_connection_available() const
```

Returns true if a connection is available for taking.

### is_listening {#is_listening}

```gdscript
bool is_listening() const
```

Returns true if the server is currently listening for connections.

### stop {#stop}

```gdscript
void stop()
```

Stops listening.

### take_socket_connection {#take_socket_connection}

```gdscript
StreamPeerSocket take_socket_connection()
```

If a connection is available, returns a StreamPeerSocket with the connection.
