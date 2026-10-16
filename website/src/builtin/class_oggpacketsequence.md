# OggPacketSequence

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OggPacketSequence**

> A sequence of Ogg packets.

<details><summary>官方英文描述</summary>

A sequence of Ogg packets.

</details>

## 方法

### get_length {#get_length}

```gdscript
float get_length() const
```

The length of this stream, in seconds.

### get_packet_data {#get_packet_data}

```gdscript
typedarray::Array get_packet_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_packet_granule_positions {#get_packet_granule_positions}

```gdscript
PackedInt64Array get_packet_granule_positions() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sampling_rate {#get_sampling_rate}

```gdscript
float get_sampling_rate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_packet_data {#set_packet_data}

```gdscript
void set_packet_data(packet_data: typedarray::Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_packet_granule_positions {#set_packet_granule_positions}

```gdscript
void set_packet_granule_positions(granule_positions: PackedInt64Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sampling_rate {#set_sampling_rate}

```gdscript
void set_sampling_rate(sampling_rate: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### packet_data {#packet_data}

```gdscript
packet_data: typedarray::PackedByteArray
```

- void set_packet_data(value: Array[Array])
- Array[Array] get_packet_data()

Contains the raw packets that make up this OggPacketSequence.

### granule_positions {#granule_positions}

```gdscript
granule_positions: PackedInt64Array
```

- void set_packet_granule_positions(value: PackedInt64Array)
- PackedInt64Array get_packet_granule_positions()

Contains the granule positions for each page in this packet sequence.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedInt64Array for more details.

### sampling_rate {#sampling_rate}

```gdscript
sampling_rate: float
```

- void set_sampling_rate(value: float)
- float get_sampling_rate()

Holds sample rate information about this sequence. Must be set by another class that actually understands the codec.
