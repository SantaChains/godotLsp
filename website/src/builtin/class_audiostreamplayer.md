# AudioStreamPlayer

**继承**：[Node](class_node.md) → [Object](class_object.md) → **AudioStreamPlayer**

> The AudioStreamPlayer node plays an audio stream non-positionally. It is ideal for user interfaces, menus, or background music.

<details><summary>官方英文描述（中文精译进行中）</summary>

The AudioStreamPlayer node plays an audio stream non-positionally. It is ideal for user interfaces, menus, or background music.

To use this node, stream needs to be set to a valid AudioStream resource. Playing more than one sound at the same time is also supported, see max_polyphony.

If you need to play audio at a specific position, use AudioStreamPlayer2D or AudioStreamPlayer3D instead.

</details>

## 方法

### get_bus {#get_bus}

```gdscript
StringName get_bus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_polyphony {#get_max_polyphony}

```gdscript
int get_max_polyphony() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mix_target {#get_mix_target}

```gdscript
AudioStreamPlayer.MixTarget get_mix_target() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pitch_scale {#get_pitch_scale}

```gdscript
float get_pitch_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_playback_position {#get_playback_position}

```gdscript
float get_playback_position()
```

Returns the position in the AudioStream of the latest sound, in seconds. Returns 0.0 if no sounds are playing.

Note: The position is not always accurate, as the AudioServer does not mix audio every processed frame. To get more accurate results, add AudioServer.get_time_since_last_mix() to the returned position.

Note: This method always returns 0.0 if the stream is an AudioStreamInteractive, since it can have multiple clips playing at once.

### get_playback_type {#get_playback_type}

```gdscript
AudioServer.PlaybackType get_playback_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream {#get_stream}

```gdscript
AudioStream get_stream() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream_paused {#get_stream_paused}

```gdscript
bool get_stream_paused() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream_playback {#get_stream_playback}

```gdscript
AudioStreamPlayback get_stream_playback()
```

Returns the latest AudioStreamPlayback of this node, usually the most recently created by play(). If no sounds are playing, this method fails and returns an empty playback.

### get_volume_db {#get_volume_db}

```gdscript
float get_volume_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_volume_linear {#get_volume_linear}

```gdscript
float get_volume_linear() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_stream_playback {#has_stream_playback}

```gdscript
bool has_stream_playback()
```

Returns true if any sound is active, even if stream_paused is set to true. See also playing and get_stream_playback().

### is_autoplay_enabled {#is_autoplay_enabled}

```gdscript
bool is_autoplay_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_playing {#is_playing}

```gdscript
bool is_playing() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### play {#play}

```gdscript
void play(from_position: float = 0.0)
```

Plays a sound from the beginning, or the given from_position in seconds.

### seek {#seek}

```gdscript
void seek(to_position: float)
```

Restarts all sounds to be played from the given to_position, in seconds. Does nothing if no sounds are playing.

### set_autoplay {#set_autoplay}

```gdscript
void set_autoplay(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bus {#set_bus}

```gdscript
void set_bus(bus: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_polyphony {#set_max_polyphony}

```gdscript
void set_max_polyphony(max_polyphony: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mix_target {#set_mix_target}

```gdscript
void set_mix_target(mix_target: AudioStreamPlayer.MixTarget)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pitch_scale {#set_pitch_scale}

```gdscript
void set_pitch_scale(pitch_scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_playback_type {#set_playback_type}

```gdscript
void set_playback_type(playback_type: AudioServer.PlaybackType)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_playing {#set_playing}

```gdscript
void set_playing(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stream {#set_stream}

```gdscript
void set_stream(stream: AudioStream)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stream_paused {#set_stream_paused}

```gdscript
void set_stream_paused(pause: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_volume_db {#set_volume_db}

```gdscript
void set_volume_db(volume_db: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_volume_linear {#set_volume_linear}

```gdscript
void set_volume_linear(volume_linear: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stop {#stop}

```gdscript
void stop()
```

Stops all sounds from this node.

## 属性

### stream {#stream}

```gdscript
stream: AudioStream
```

- void set_stream(value: AudioStream)
- AudioStream get_stream()

The AudioStream resource to be played. Setting this property stops all currently playing sounds. If left empty, the AudioStreamPlayer does not work.

### volume_db {#volume_db}

```gdscript
volume_db: float
```

- void set_volume_db(value: float)
- float get_volume_db()

Volume of sound, in decibels. This is an offset of the stream's volume.

Note: To convert between decibel and linear energy (like most volume sliders do), use volume_linear, or @GlobalScope.db_to_linear() and @GlobalScope.linear_to_db().

### volume_linear {#volume_linear}

```gdscript
volume_linear: float
```

- void set_volume_linear(value: float)
- float get_volume_linear()

Volume of sound, as a linear value.

Note: This member modifies volume_db for convenience. The returned value is equivalent to the result of @GlobalScope.db_to_linear() on volume_db. Setting this member is equivalent to setting volume_db to the result of @GlobalScope.linear_to_db() on a value.

### pitch_scale {#pitch_scale}

```gdscript
pitch_scale: float
```

- void set_pitch_scale(value: float)
- float get_pitch_scale()

The audio's pitch and tempo, as a multiplier of the stream's sample rate. A value of 2.0 doubles the audio's pitch, while a value of 0.5 halves the pitch.

### playing {#playing}

```gdscript
playing: bool
```

- void set_playing(value: bool)
- bool is_playing()

If true, this node is playing sounds. Setting this property has the same effect as play() and stop().

### autoplay {#autoplay}

```gdscript
autoplay: bool
```

- void set_autoplay(value: bool)
- bool is_autoplay_enabled()

If true, this node calls play() when entering the tree.

### stream_paused {#stream_paused}

```gdscript
stream_paused: bool
```

- void set_stream_paused(value: bool)
- bool get_stream_paused()

If true, the sounds are paused. Setting stream_paused to false resumes all sounds.

Note: This property is automatically changed when exiting or entering the tree, or this node is paused (see Node.process_mode).

### mix_target {#mix_target}

```gdscript
mix_target: int
```

- void set_mix_target(value: MixTarget)
- MixTarget get_mix_target()

The mix target channels. Has no effect when two speakers or less are detected (see SpeakerMode).

### max_polyphony {#max_polyphony}

```gdscript
max_polyphony: int
```

- void set_max_polyphony(value: int)
- int get_max_polyphony()

The maximum number of sounds this node can play at the same time. Calling play() after this value is reached will cut off the oldest sounds.

### bus {#bus}

```gdscript
bus: StringName
```

- void set_bus(value: StringName)
- StringName get_bus()

The target bus name. All sounds from this node will be playing on this bus.

Note: At runtime, if no bus with the given name exists, all sounds will fall back on "Master". See also AudioServer.get_bus_name().

### playback_type {#playback_type}

```gdscript
playback_type: int
```

- void set_playback_type(value: PlaybackType)
- PlaybackType get_playback_type()

Experimental: This property may be changed or removed in future versions.

The playback type of the stream player. If set other than to the default value, it will force that playback type.

## 信号

### finished {#finished}

```gdscript
signal finished()
```

Emitted when a sound finishes playing without interruptions. This signal is not emitted when calling stop(), or when exiting the tree while sounds are playing.

## 枚举

### MixTarget {#enum-MixTarget}

```gdscript
enum MixTarget { MIX_TARGET_STEREO = 0, MIX_TARGET_SURROUND = 1, MIX_TARGET_CENTER = 2 }
```

MixTarget MIX_TARGET_STEREO = 0

The audio will be played only on the first channel. This is the default.

MixTarget MIX_TARGET_SURROUND = 1

The audio will be played on all surround channels.

MixTarget MIX_TARGET_CENTER = 2

The audio will be played on the second channel, which is usually the center.

#### MIX_TARGET_STEREO {#MIX_TARGET_STEREO}

```gdscript
MIX_TARGET_STEREO = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MIX_TARGET_SURROUND {#MIX_TARGET_SURROUND}

```gdscript
MIX_TARGET_SURROUND = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MIX_TARGET_CENTER {#MIX_TARGET_CENTER}

```gdscript
MIX_TARGET_CENTER = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
