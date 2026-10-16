# AudioStreamPlayback

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamPlayback**

> Can play, loop, pause a scroll through audio. See AudioStream and AudioStreamOggVorbis for usage.

<details><summary>官方英文描述</summary>

Can play, loop, pause a scroll through audio. See AudioStream and AudioStreamOggVorbis for usage.

</details>

## 方法

### _get_loop_count {#_get_loop_count}

```gdscript
int _get_loop_count() virtual const
```

Overridable method. Should return how many times this audio stream has looped. Most built-in playbacks always return 0.

### _get_parameter {#_get_parameter}

```gdscript
Variant _get_parameter(name: StringName) virtual const
```

Return the current value of a playback parameter by name (see AudioStream._get_parameter_list()).

### _get_playback_position {#_get_playback_position}

```gdscript
float _get_playback_position() virtual const
```

Overridable method. Should return the current progress along the audio stream, in seconds.

### _is_playing {#_is_playing}

```gdscript
bool _is_playing() virtual const
```

Overridable method. Should return true if this playback is active and playing its audio stream.

### _mix {#_mix}

```gdscript
int _mix(buffer: AudioFrame*, rate_scale: float, frames: int) virtual
```

Override this method to customize how the audio stream is mixed. This method is called even if the playback is not active.

Note: It is not useful to override this method in GDScript or C#. Only GDExtension can take advantage of it.

### _seek {#_seek}

```gdscript
void _seek(position: float) virtual
```

Override this method to customize what happens when seeking this audio stream at the given position, such as by calling AudioStreamPlayer.seek().

### _set_parameter {#_set_parameter}

```gdscript
void _set_parameter(name: StringName, value: Variant) virtual
```

Set the current value of a playback parameter by name (see AudioStream._get_parameter_list()).

### _start {#_start}

```gdscript
void _start(from_pos: float) virtual
```

Override this method to customize what happens when the playback starts at the given position, such as by calling AudioStreamPlayer.play().

### _stop {#_stop}

```gdscript
void _stop() virtual
```

Override this method to customize what happens when the playback is stopped, such as by calling AudioStreamPlayer.stop().

### _tag_used_streams {#_tag_used_streams}

```gdscript
void _tag_used_streams() virtual
```

Overridable method. Called whenever the audio stream is mixed if the playback is active and AudioServer.set_enable_tagging_used_audio_streams() has been set to true. Editor plugins may use this method to "tag" the current position along the audio stream and display it in a preview.

### get_loop_count {#get_loop_count}

```gdscript
int get_loop_count() const
```

Returns the number of times the stream has looped.

### get_playback_position {#get_playback_position}

```gdscript
float get_playback_position() const
```

Returns the current position in the stream, in seconds.

### get_sample_playback {#get_sample_playback}

```gdscript
AudioSamplePlayback get_sample_playback() const
```

Experimental: This method may be changed or removed in future versions.

Returns the AudioSamplePlayback associated with this AudioStreamPlayback for playing back the audio sample of this stream.

### is_playing {#is_playing}

```gdscript
bool is_playing() const
```

Returns true if the stream is playing.

### mix_audio {#mix_audio}

```gdscript
PackedVector2Array mix_audio(rate_scale: float, frames: int)
```

Mixes up to frames of audio from the stream from the current position, at a rate of rate_scale, advancing the stream.

Returns a PackedVector2Array where each element holds the left and right channel volume levels of each frame.

Note: Can return fewer frames than requested, make sure to use the size of the return value.

### seek {#seek}

```gdscript
void seek(time: float = 0.0)
```

Seeks the stream at the given time, in seconds.

### set_sample_playback {#set_sample_playback}

```gdscript
void set_sample_playback(playback_sample: AudioSamplePlayback)
```

Experimental: This method may be changed or removed in future versions.

Associates AudioSamplePlayback to this AudioStreamPlayback for playing back the audio sample of this stream.

### start {#start}

```gdscript
void start(from_pos: float = 0.0)
```

Starts the stream from the given from_pos, in seconds.

### stop {#stop}

```gdscript
void stop()
```

Stops the stream.
