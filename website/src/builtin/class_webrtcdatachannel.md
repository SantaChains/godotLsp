# WebRTCDataChannel

**继承**：[PacketPeer](class_packetpeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **WebRTCDataChannel**

## 方法

### close {#close}

```gdscript
void close()
```

Closes this data channel, notifying the other peer.

### get_buffered_amount {#get_buffered_amount}

```gdscript
int get_buffered_amount() const
```

Returns the number of bytes currently queued to be sent over this channel.

### get_id {#get_id}

```gdscript
int get_id() const
```

Returns the ID assigned to this channel during creation (or auto-assigned during negotiation).

If the channel is not negotiated out-of-band the ID will only be available after the connection is established (will return 65535 until then).

### get_label {#get_label}

```gdscript
String get_label() const
```

Returns the label assigned to this channel during creation.

### get_max_packet_life_time {#get_max_packet_life_time}

```gdscript
int get_max_packet_life_time() const
```

Returns the maxPacketLifeTime value assigned to this channel during creation.

Will be 65535 if not specified.

### get_max_retransmits {#get_max_retransmits}

```gdscript
int get_max_retransmits() const
```

Returns the maxRetransmits value assigned to this channel during creation.

Will be 65535 if not specified.

### get_protocol {#get_protocol}

```gdscript
String get_protocol() const
```

Returns the sub-protocol assigned to this channel during creation. An empty string if not specified.

### get_ready_state {#get_ready_state}

```gdscript
WebRTCDataChannel.ChannelState get_ready_state() const
```

Returns the current state of this channel.

### get_write_mode {#get_write_mode}

```gdscript
WebRTCDataChannel.WriteMode get_write_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_negotiated {#is_negotiated}

```gdscript
bool is_negotiated() const
```

Returns true if this channel was created with out-of-band configuration.

### is_ordered {#is_ordered}

```gdscript
bool is_ordered() const
```

Returns true if this channel was created with ordering enabled (default).

### poll {#poll}

```gdscript
Error poll()
```

Reserved, but not used for now.

### set_write_mode {#set_write_mode}

```gdscript
void set_write_mode(write_mode: WebRTCDataChannel.WriteMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### was_string_packet {#was_string_packet}

```gdscript
bool was_string_packet() const
```

Returns true if the last received packet was transferred as text. See write_mode.

## 属性

### write_mode {#write_mode}

```gdscript
write_mode: int
```

- void set_write_mode(value: WriteMode)
- WriteMode get_write_mode()

The transfer mode to use when sending outgoing packet. Either text or binary.

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

Tells the channel to send data over this channel as text. An external peer (non-Godot) would receive this as a string.

#### WRITE_MODE_BINARY {#WRITE_MODE_BINARY}

```gdscript
WRITE_MODE_BINARY = 1
```

Tells the channel to send data over this channel as binary. An external peer (non-Godot) would receive this as array buffer or blob.

---

### ChannelState {#enum-ChannelState}

```gdscript
enum ChannelState { STATE_CONNECTING = 0, STATE_OPEN = 1, STATE_CLOSING = 2, STATE_CLOSED = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### STATE_CONNECTING {#STATE_CONNECTING}

```gdscript
STATE_CONNECTING = 0
```

The channel was created, but it's still trying to connect.

#### STATE_OPEN {#STATE_OPEN}

```gdscript
STATE_OPEN = 1
```

The channel is currently open, and data can flow over it.

#### STATE_CLOSING {#STATE_CLOSING}

```gdscript
STATE_CLOSING = 2
```

The channel is being closed, no new messages will be accepted, but those already in queue will be flushed.

#### STATE_CLOSED {#STATE_CLOSED}

```gdscript
STATE_CLOSED = 3
```

The channel was closed, or connection failed.

---
