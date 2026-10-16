# UDSServer

**继承**：[SocketServer](class_socketserver.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **UDSServer**

> A Unix Domain Socket (UDS) server. Listens to connections on a socket path and returns a StreamPeerUDS when it gets an incoming connection. Unix Domain Sockets provide inter-process communication on the same machine using the filesystem namespace.

<details><summary>官方英文描述</summary>

A Unix Domain Socket (UDS) server. Listens to connections on a socket path and returns a StreamPeerUDS when it gets an incoming connection. Unix Domain Sockets provide inter-process communication on the same machine using the filesystem namespace.

Note: Unix Domain Sockets are only available on Unix-like systems (Linux, macOS, etc.) and are not supported on Windows.

</details>

## 方法

### listen {#listen}

```gdscript
Error listen(path: String)
```

Listens on the socket at path. The socket file will be created at the specified path.

Note: The socket file must not already exist at the specified path. You may need to remove any existing socket file before calling this method.

### take_connection {#take_connection}

```gdscript
StreamPeerUDS take_connection()
```

If a connection is available, returns a StreamPeerUDS with the connection.
