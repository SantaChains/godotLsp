# WebSocketPeer

**继承**：[PacketPeer](class_packetpeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **WebSocketPeer**

> This class represents WebSocket connection, and can be used as a WebSocket client (RFC 6455 [https://datatracker.ietf.org/doc/html/rfc6455]-compliant) or as a remote peer of a WebSocket server.

<details><summary>官方英文描述</summary>

This class represents WebSocket connection, and can be used as a WebSocket client (RFC 6455 [https://datatracker.ietf.org/doc/html/rfc6455]-compliant) or as a remote peer of a WebSocket server.

You can send WebSocket binary frames using PacketPeer.put_packet(), and WebSocket text frames using send() (prefer text frames when interacting with text-based API). You can check the frame type of the last packet via was_string_packet().

To start a WebSocket client, first call connect_to_url(), then regularly call poll() (e.g. during Node process). You can query the socket state via get_ready_state(), get the number of pending packets using PacketPeer.get_available_packet_count(), and retrieve them via PacketPeer.get_packet().

```
extends Node
var socket = WebSocketPeer.new()
func _ready():
    socket.connect_to_url("wss://example.com")
func _process(delta):
    socket.poll()
    var state = socket.get_ready_state()
    if state == WebSocketPeer.STATE_OPEN:
        while socket.get_available_packet_count():
            print("Packet: ", socket.get_packet())
    elif state == WebSocketPeer.STATE_CLOSING:
        # Keep polling to achieve proper close.
        pass
    elif state == WebSocketPeer.STATE_CLOSED:
        var code = socket.get_close_code()
        var reason = socket.get_close_reason()
        print("WebSocket closed with code: %d, reason %s. Clean: %s" % [code, reason, code != -1])
        set_process(false) # Stop processing.
```

To use the peer as part of a WebSocket server refer to accept_stream() and the online tutorial.

</details>

## 方法

### accept_stream {#accept_stream}

```gdscript
Error accept_stream(stream: StreamPeer)
```

Accepts a peer connection performing the HTTP handshake as a WebSocket server. The stream must be a valid TCP stream retrieved via TCPServer.take_connection(), or a TLS stream accepted via StreamPeerTLS.accept_stream().

Note: Not supported in Web exports due to browsers' restrictions.

### close {#close}

```gdscript
void close(code: int = 1000, reason: String = "")
```

Closes this WebSocket connection.

code is the status code for the closure (see RFC 6455 section 7.4 [https://datatracker.ietf.org/doc/html/rfc6455#section-7.4.1] for a list of valid status codes). If code is negative, the connection will be closed immediately without notifying the remote peer.

reason is the human-readable reason for closing the connection. It can be any UTF-8 string that's smaller than 123 bytes.

Note: To achieve a clean closure, you will need to keep polling until STATE_CLOSED is reached.

Note: The Web export might not support all status codes. Please refer to browser-specific documentation for more details.

### connect_to_url {#connect_to_url}

```gdscript
Error connect_to_url(url: String, tls_client_options: TLSOptions = null)
```

Connects to the given URL. TLS certificates will be verified against the hostname when connecting using the wss:// protocol. You can pass the optional tls_client_options parameter to customize the trusted certification authorities, or disable the common name verification. See TLSOptions.client() and TLSOptions.client_unsafe().

Note: This method is non-blocking, and will return @GlobalScope.OK before the connection is established as long as the provided parameters are valid and the peer is not in an invalid state (e.g. already connected). Regularly call poll() (e.g. during Node process) and check the result of get_ready_state() to know whether the connection succeeds or fails.

Note: To avoid mixed content warnings or errors in Web, you may have to use a url that starts with wss:// (secure) instead of ws://. When doing so, make sure to use the fully qualified domain name that matches the one defined in the server's TLS certificate. Do not connect directly via the IP address for wss:// connections, as it won't match with the TLS certificate.

### get_close_code {#get_close_code}

```gdscript
int get_close_code() const
```

Returns the received WebSocket close frame status code, or -1 when the connection was not cleanly closed. Only call this method when get_ready_state() returns STATE_CLOSED.

### get_close_reason {#get_close_reason}

```gdscript
String get_close_reason() const
```

Returns the received WebSocket close frame status reason string. Only call this method when get_ready_state() returns STATE_CLOSED.

### get_connected_host {#get_connected_host}

```gdscript
String get_connected_host() const
```

Returns the IP address of the connected peer.

Note: Not available in the Web export.

### get_connected_port {#get_connected_port}

```gdscript
int get_connected_port() const
```

Returns the remote port of the connected peer.

Note: Not available in the Web export.

### get_current_outbound_buffered_amount {#get_current_outbound_buffered_amount}

```gdscript
int get_current_outbound_buffered_amount() const
```

Returns the current amount of data in the outbound websocket buffer. Note: Web exports use WebSocket.bufferedAmount, while other platforms use an internal buffer.

### get_handshake_headers {#get_handshake_headers}

```gdscript
PackedStringArray get_handshake_headers() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_heartbeat_interval {#get_heartbeat_interval}

```gdscript
float get_heartbeat_interval() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inbound_buffer_size {#get_inbound_buffer_size}

```gdscript
int get_inbound_buffer_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_queued_packets {#get_max_queued_packets}

```gdscript
int get_max_queued_packets() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outbound_buffer_size {#get_outbound_buffer_size}

```gdscript
int get_outbound_buffer_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ready_state {#get_ready_state}

```gdscript
WebSocketPeer.State get_ready_state() const
```

Returns the ready state of the connection.

### get_requested_url {#get_requested_url}

```gdscript
String get_requested_url() const
```

Returns the URL requested by this peer. The URL is derived from the url passed to connect_to_url() or from the HTTP headers when acting as server (i.e. when using accept_stream()).

### get_selected_protocol {#get_selected_protocol}

```gdscript
String get_selected_protocol() const
```

Returns the selected WebSocket sub-protocol for this connection or an empty string if the sub-protocol has not been selected yet.

### get_supported_protocols {#get_supported_protocols}

```gdscript
PackedStringArray get_supported_protocols() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### poll {#poll}

```gdscript
void poll()
```

Updates the connection state and receive incoming packets. Call this function regularly to keep it in a clean state.

### send {#send}

```gdscript
Error send(message: PackedByteArray, write_mode: WebSocketPeer.WriteMode = 1)
```

Sends the given message using the desired write_mode. When sending a String, prefer using send_text().

### send_text {#send_text}

```gdscript
Error send_text(message: String)
```

Sends the given message using WebSocket text mode. Prefer this method over PacketPeer.put_packet() when interacting with third-party text-based API (e.g. when using JSON formatted messages).

### set_handshake_headers {#set_handshake_headers}

```gdscript
void set_handshake_headers(protocols: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_heartbeat_interval {#set_heartbeat_interval}

```gdscript
void set_heartbeat_interval(interval: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inbound_buffer_size {#set_inbound_buffer_size}

```gdscript
void set_inbound_buffer_size(buffer_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_queued_packets {#set_max_queued_packets}

```gdscript
void set_max_queued_packets(buffer_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_no_delay {#set_no_delay}

```gdscript
void set_no_delay(enabled: bool)
```

Disable Nagle's algorithm on the underlying TCP socket (default). See StreamPeerTCP.set_no_delay() for more information.

Note: Not available in the Web export.

### set_outbound_buffer_size {#set_outbound_buffer_size}

```gdscript
void set_outbound_buffer_size(buffer_size: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_supported_protocols {#set_supported_protocols}

```gdscript
void set_supported_protocols(protocols: PackedStringArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### was_string_packet {#was_string_packet}

```gdscript
bool was_string_packet() const
```

Returns true if the last received packet was sent as a text payload. See WriteMode.

## 属性

### supported_protocols {#supported_protocols}

```gdscript
supported_protocols: PackedStringArray
```

- void set_supported_protocols(value: PackedStringArray)
- PackedStringArray get_supported_protocols()

The WebSocket sub-protocols allowed during the WebSocket handshake.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedStringArray for more details.

### handshake_headers {#handshake_headers}

```gdscript
handshake_headers: PackedStringArray
```

- void set_handshake_headers(value: PackedStringArray)
- PackedStringArray get_handshake_headers()

The extra HTTP headers to be sent during the WebSocket handshake.

Note: Not supported in Web exports due to browsers' restrictions.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedStringArray for more details.

### inbound_buffer_size {#inbound_buffer_size}

```gdscript
inbound_buffer_size: int
```

- void set_inbound_buffer_size(value: int)
- int get_inbound_buffer_size()

The size of the input buffer in bytes (roughly the maximum amount of memory that will be allocated for the inbound packets).

### outbound_buffer_size {#outbound_buffer_size}

```gdscript
outbound_buffer_size: int
```

- void set_outbound_buffer_size(value: int)
- int get_outbound_buffer_size()

The size of the input buffer in bytes (roughly the maximum amount of memory that will be allocated for the outbound packets).

### max_queued_packets {#max_queued_packets}

```gdscript
max_queued_packets: int
```

- void set_max_queued_packets(value: int)
- int get_max_queued_packets()

The maximum amount of packets that will be allowed in the queues (both inbound and outbound).

### heartbeat_interval {#heartbeat_interval}

```gdscript
heartbeat_interval: int
```

- void set_heartbeat_interval(value: float)
- float get_heartbeat_interval()

The interval (in seconds) at which the peer will automatically send WebSocket "ping" control frames. When set to 0, no "ping" control frames will be sent.

Note: Has no effect in Web exports due to browser restrictions.

## 枚举

### WriteMode {#enum-WriteMode}

```gdscript
enum WriteMode { WRITE_MODE_TEXT = 0, WRITE_MODE_BINARY = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### WRITE_MODE_TEXT {#WRITE_MODE_TEXT}

```gdscript
WRITE_MODE_TEXT = 0
```

Specifies that WebSockets messages should be transferred as text payload (only valid UTF-8 is allowed).

#### WRITE_MODE_BINARY {#WRITE_MODE_BINARY}

```gdscript
WRITE_MODE_BINARY = 1
```

Specifies that WebSockets messages should be transferred as binary payload (any byte combination is allowed).

---

### State {#enum-State}

```gdscript
enum State { STATE_CONNECTING = 0, STATE_OPEN = 1, STATE_CLOSING = 2, STATE_CLOSED = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STATE_CONNECTING {#STATE_CONNECTING}

```gdscript
STATE_CONNECTING = 0
```

Socket has been created. The connection is not yet open.

#### STATE_OPEN {#STATE_OPEN}

```gdscript
STATE_OPEN = 1
```

The connection is open and ready to communicate.

#### STATE_CLOSING {#STATE_CLOSING}

```gdscript
STATE_CLOSING = 2
```

The connection is in the process of closing. This means a close request has been sent to the remote peer but confirmation has not been received.

#### STATE_CLOSED {#STATE_CLOSED}

```gdscript
STATE_CLOSED = 3
```

The connection is closed or couldn't be opened.

---
