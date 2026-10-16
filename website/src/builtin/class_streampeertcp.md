# StreamPeerTCP

**继承**：[StreamPeerSocket](class_streampeersocket.md) → [StreamPeer](class_streampeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StreamPeerTCP**

> A stream peer that handles TCP connections. This object can be used to connect to TCP servers, or also is returned by a TCP server.

<details><summary>官方英文描述（中文精译进行中）</summary>

A stream peer that handles TCP connections. This object can be used to connect to TCP servers, or also is returned by a TCP server.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### bind {#bind}

```gdscript
Error bind(port: int, host: String = "*")
```

Opens the TCP socket, and binds it to the specified local address.

This method is generally not needed, and only used to force the subsequent call to connect_to_host() to use the specified host and port as source address. This can be desired in some NAT punchthrough techniques, or when forcing the source network interface.

### connect_to_host {#connect_to_host}

```gdscript
Error connect_to_host(host: String, port: int)
```

Connects to the specified host:port pair. A hostname will be resolved if valid. Returns @GlobalScope.OK on success.

### get_connected_host {#get_connected_host}

```gdscript
String get_connected_host() const
```

Returns the IP of this peer.

### get_connected_port {#get_connected_port}

```gdscript
int get_connected_port() const
```

Returns the port of this peer.

### get_local_port {#get_local_port}

```gdscript
int get_local_port() const
```

Returns the local port to which this peer is bound.

### set_no_delay {#set_no_delay}

```gdscript
void set_no_delay(enabled: bool)
```

If enabled is true, packets will be sent immediately. If enabled is false (the default), packet transfers will be delayed and combined using Nagle's algorithm [https://en.wikipedia.org/wiki/Nagle%27s_algorithm].

Note: It's recommended to leave this disabled for applications that send large packets or need to transfer a lot of data, as enabling this can decrease the total available bandwidth.
