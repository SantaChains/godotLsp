# VideoStreamPlayback

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VideoStreamPlayback**

> This class is intended to be overridden by video decoder extensions with custom implementations of VideoStream.

<details><summary>官方英文描述</summary>

This class is intended to be overridden by video decoder extensions with custom implementations of VideoStream.

</details>

## 方法

### _get_channels {#_get_channels}

```gdscript
int _get_channels() virtual const
```

Returns the number of audio channels.

### _get_length {#_get_length}

```gdscript
float _get_length() virtual const
```

Returns the video duration in seconds, if known, or 0 if unknown.

### _get_mix_rate {#_get_mix_rate}

```gdscript
int _get_mix_rate() virtual const
```

Returns the audio sample rate used for mixing.

### _get_playback_position {#_get_playback_position}

```gdscript
float _get_playback_position() virtual const
```

Return the current playback timestamp. Called in response to the VideoStreamPlayer.stream_position getter.

### _get_texture {#_get_texture}

```gdscript
Texture2D _get_texture() virtual const
```

Allocates a Texture2D in which decoded video frames will be drawn.

### _is_paused {#_is_paused}

```gdscript
bool _is_paused() virtual const
```

Returns the paused status, as set by _set_paused().

### _is_playing {#_is_playing}

```gdscript
bool _is_playing() virtual const
```

Returns the playback state, as determined by calls to _play() and _stop().

### _play {#_play}

```gdscript
void _play() virtual
```

Called in response to VideoStreamPlayer.autoplay or VideoStreamPlayer.play(). Note that manual playback may also invoke _stop() multiple times before this method is called. _is_playing() should return true once playing.

### _seek {#_seek}

```gdscript
void _seek(time: float) virtual
```

Seeks to time seconds. Called in response to the VideoStreamPlayer.stream_position setter.

### _set_audio_track {#_set_audio_track}

```gdscript
void _set_audio_track(idx: int) virtual
```

Select the audio track idx. Called when playback starts, and in response to the VideoStreamPlayer.audio_track setter.

### _set_paused {#_set_paused}

```gdscript
void _set_paused(paused: bool) virtual
```

Set the paused status of video playback. _is_paused() must return paused. Called in response to the VideoStreamPlayer.paused setter.

### _stop {#_stop}

```gdscript
void _stop() virtual
```

Stops playback. May be called multiple times before _play(), or in response to VideoStreamPlayer.stop(). _is_playing() should return false once stopped.

### _update {#_update}

```gdscript
void _update(delta: float) virtual
```

Ticks video playback for delta seconds. Called every frame as long as both _is_paused() and _is_playing() return true.

### mix_audio {#mix_audio}

```gdscript
int mix_audio(num_frames: int, buffer: PackedFloat32Array = PackedFloat32Array(), offset: int = 0)
```

Render num_frames audio frames (of _get_channels() floats each) from buffer, starting from index offset in the array. Returns the number of audio frames rendered, or -1 on error.
