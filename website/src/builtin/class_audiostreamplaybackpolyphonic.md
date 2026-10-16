# AudioStreamPlaybackPolyphonic

**继承**：[AudioStreamPlayback](class_audiostreamplayback.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamPlaybackPolyphonic**

> Playback instance for AudioStreamPolyphonic. After setting the stream property of AudioStreamPlayer, AudioStreamPlayer2D, or AudioStreamPlayer3D, the playback instance can be obtained by calling AudioStreamPlayer.get_stream_playback(), AudioStreamPlayer2D.get_stream_playback() or AudioStreamPlayer3D.get_stream_playback() methods.

<details><summary>官方英文描述</summary>

Playback instance for AudioStreamPolyphonic. After setting the stream property of AudioStreamPlayer, AudioStreamPlayer2D, or AudioStreamPlayer3D, the playback instance can be obtained by calling AudioStreamPlayer.get_stream_playback(), AudioStreamPlayer2D.get_stream_playback() or AudioStreamPlayer3D.get_stream_playback() methods.

</details>

## 方法

### is_stream_playing {#is_stream_playing}

```gdscript
bool is_stream_playing(stream: int) const
```

Returns true if the stream associated with the given integer ID is still playing. Check play_stream() for information on when this ID becomes invalid.

### play_stream {#play_stream}

```gdscript
int play_stream(stream: AudioStream, from_offset: float = 0, volume_db: float = 0, pitch_scale: float = 1.0, playback_type: AudioServer.PlaybackType = 0, bus: StringName = &"Master")
```

Play an AudioStream at a given offset, volume, pitch scale, playback type, and bus. Playback starts immediately.

The return value is a unique integer ID that is associated to this playback stream and which can be used to control it.

This ID becomes invalid when the stream ends (if it does not loop), when the AudioStreamPlaybackPolyphonic is stopped, or when stop_stream() is called.

This function returns INVALID_ID if the amount of streams currently playing equals AudioStreamPolyphonic.polyphony. If you need a higher amount of maximum polyphony, raise this value.

### set_stream_pitch_scale {#set_stream_pitch_scale}

```gdscript
void set_stream_pitch_scale(stream: int, pitch_scale: float)
```

Change the stream pitch scale. The stream argument is an integer ID returned by play_stream().

### set_stream_volume {#set_stream_volume}

```gdscript
void set_stream_volume(stream: int, volume_db: float)
```

Change the stream volume (in db). The stream argument is an integer ID returned by play_stream().

### stop_stream {#stop_stream}

```gdscript
void stop_stream(stream: int)
```

Stop a stream. The stream argument is an integer ID returned by play_stream(), which becomes invalid after calling this function.

## 常量

### INVALID_ID {#INVALID_ID}

```gdscript
const INVALID_ID = -1
```

Returned by play_stream() in case it could not allocate a stream for playback.
