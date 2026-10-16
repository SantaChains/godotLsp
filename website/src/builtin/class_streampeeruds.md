# StreamPeerUDS

**继承**：[StreamPeerSocket](class_streampeersocket.md) → [StreamPeer](class_streampeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StreamPeerUDS**

> A stream peer that handles UNIX Domain Socket (UDS) connections. This object can be used to connect to UDS servers, or also is returned by a UDS server. Unix Domain Sockets provide inter-process communication on the same machine using the filesystem namespace.

<details><summary>官方英文描述</summary>

A stream peer that handles UNIX Domain Socket (UDS) connections. This object can be used to connect to UDS servers, or also is returned by a UDS server. Unix Domain Sockets provide inter-process communication on the same machine using the filesystem namespace.

Note: UNIX Domain Sockets are only available on UNIX-like systems (Linux, macOS, etc.) and are not supported on Windows.

</details>

## 方法

### bind {#bind}

```gdscript
Error bind(path: String)
```

Opens the UDS socket, and binds it to the specified socket path.

This method is generally not needed, and only used to force the subsequent call to connect_to_host() to use the specified path as the source address.

### connect_to_host {#connect_to_host}

```gdscript
Error connect_to_host(path: String)
```

Connects to the specified UNIX Domain Socket path. Returns @GlobalScope.OK on success.

### get_connected_path {#get_connected_path}

```gdscript
String get_connected_path() const
```

Returns the socket path of this peer.
