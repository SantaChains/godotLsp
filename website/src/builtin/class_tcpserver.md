# TCPServer

**继承**：[SocketServer](class_socketserver.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **TCPServer**

> A TCP server. Listens to connections on a port and returns a StreamPeerTCP when it gets an incoming connection.

<details><summary>官方英文描述（中文精译进行中）</summary>

A TCP server. Listens to connections on a port and returns a StreamPeerTCP when it gets an incoming connection.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### get_local_port {#get_local_port}

```gdscript
int get_local_port() const
```

Returns the local port this server is listening to.

### listen {#listen}

```gdscript
Error listen(port: int, bind_address: String = "*")
```

Listen on the port binding to bind_address.

If bind_address is set as "*" (default), the server will listen on all available addresses (both IPv4 and IPv6).

If bind_address is set as "0.0.0.0" (for IPv4) or "::" (for IPv6), the server will listen on all available addresses matching that IP type.

If bind_address is set to any valid address (e.g. "192.168.1.101", "::1", etc.), the server will only listen on the interface with that address (or fail if no interface with the given address exists).

### take_connection {#take_connection}

```gdscript
StreamPeerTCP take_connection()
```

If a connection is available, returns a StreamPeerTCP with the connection.
