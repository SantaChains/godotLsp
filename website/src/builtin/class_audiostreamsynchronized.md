# AudioStreamSynchronized

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamSynchronized**

> This is a stream that can be fitted with sub-streams, which will be played in-sync. The streams begin at exactly the same time when play is pressed, and will end when the last of them ends. If one of the sub-streams loops, then playback will continue.

<details><summary>官方英文描述</summary>

This is a stream that can be fitted with sub-streams, which will be played in-sync. The streams begin at exactly the same time when play is pressed, and will end when the last of them ends. If one of the sub-streams loops, then playback will continue.

</details>

## 方法

### get_stream_count {#get_stream_count}

```gdscript
int get_stream_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sync_stream {#get_sync_stream}

```gdscript
AudioStream get_sync_stream(stream_index: int) const
```

Get one of the synchronized streams, by index.

### get_sync_stream_volume {#get_sync_stream_volume}

```gdscript
float get_sync_stream_volume(stream_index: int) const
```

Get the volume of one of the synchronized streams, by index.

### set_stream_count {#set_stream_count}

```gdscript
void set_stream_count(stream_count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sync_stream {#set_sync_stream}

```gdscript
void set_sync_stream(stream_index: int, audio_stream: AudioStream)
```

Set one of the synchronized streams, by index.

### set_sync_stream_volume {#set_sync_stream_volume}

```gdscript
void set_sync_stream_volume(stream_index: int, volume_db: float)
```

Set the volume of one of the synchronized streams, by index.

## 属性

### stream_count {#stream_count}

```gdscript
stream_count: int
```

- void set_stream_count(value: int)
- int get_stream_count()

Set the total amount of streams that will be played back synchronized.

## 常量

### MAX_STREAMS {#MAX_STREAMS}

```gdscript
const MAX_STREAMS = 32
```

Maximum amount of streams that can be synchronized.
