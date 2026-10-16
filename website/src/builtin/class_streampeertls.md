# StreamPeerTLS

**继承**：[StreamPeer](class_streampeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StreamPeerTLS**

> A stream peer that handles TLS connections. This object can be used to connect to a TLS server or accept a single TLS client connection.

<details><summary>官方英文描述（中文精译进行中）</summary>

A stream peer that handles TLS connections. This object can be used to connect to a TLS server or accept a single TLS client connection.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### accept_stream {#accept_stream}

```gdscript
Error accept_stream(stream: StreamPeer, server_options: TLSOptions)
```

Accepts a peer connection as a server using the given server_options. See TLSOptions.server().

### connect_to_stream {#connect_to_stream}

```gdscript
Error connect_to_stream(stream: StreamPeer, common_name: String, client_options: TLSOptions = null)
```

Connects to a peer using an underlying StreamPeer stream and verifying the remote certificate is correctly signed for the given common_name. You can pass the optional client_options parameter to customize the trusted certification authorities, or disable the common name verification. See TLSOptions.client() and TLSOptions.client_unsafe().

### disconnect_from_stream {#disconnect_from_stream}

```gdscript
void disconnect_from_stream()
```

Disconnects from host.

### get_status {#get_status}

```gdscript
StreamPeerTLS.Status get_status() const
```

Returns the status of the connection.

### get_stream {#get_stream}

```gdscript
StreamPeer get_stream() const
```

Returns the underlying StreamPeer connection, used in accept_stream() or connect_to_stream().

### poll {#poll}

```gdscript
void poll()
```

Poll the connection to check for incoming bytes. Call this right before StreamPeer.get_available_bytes() for it to work properly.

## 枚举

### Status {#enum-Status}

```gdscript
enum Status { STATUS_DISCONNECTED = 0, STATUS_HANDSHAKING = 1, STATUS_CONNECTED = 2, STATUS_ERROR = 3, STATUS_ERROR_HOSTNAME_MISMATCH = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STATUS_DISCONNECTED {#STATUS_DISCONNECTED}

```gdscript
STATUS_DISCONNECTED = 0
```

A status representing a StreamPeerTLS that is disconnected.

#### STATUS_HANDSHAKING {#STATUS_HANDSHAKING}

```gdscript
STATUS_HANDSHAKING = 1
```

A status representing a StreamPeerTLS during handshaking.

#### STATUS_CONNECTED {#STATUS_CONNECTED}

```gdscript
STATUS_CONNECTED = 2
```

A status representing a StreamPeerTLS that is connected to a host.

#### STATUS_ERROR {#STATUS_ERROR}

```gdscript
STATUS_ERROR = 3
```

A status representing a StreamPeerTLS in error state.

#### STATUS_ERROR_HOSTNAME_MISMATCH {#STATUS_ERROR_HOSTNAME_MISMATCH}

```gdscript
STATUS_ERROR_HOSTNAME_MISMATCH = 4
```

An error status that shows a mismatch in the TLS certificate domain presented by the host and the domain requested for validation.

---
