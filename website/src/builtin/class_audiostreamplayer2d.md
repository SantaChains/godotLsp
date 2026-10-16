# AudioStreamPlayer2D

**继承**：[Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **AudioStreamPlayer2D**

> Plays audio that is attenuated with distance to the listener.

<details><summary>官方英文描述</summary>

Plays audio that is attenuated with distance to the listener.

By default, audio is heard from the screen center. This can be changed by adding an AudioListener2D node to the scene and enabling it by calling AudioListener2D.make_current() on it.

See also AudioStreamPlayer to play a sound non-positionally.

Note: Hiding an AudioStreamPlayer2D node does not disable its audio output. To temporarily disable an AudioStreamPlayer2D's audio output, set volume_db to a very low value like -100 (which isn't audible to human hearing).

</details>

## 方法

### get_area_mask {#get_area_mask}

```gdscript
int get_area_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_attenuation {#get_attenuation}

```gdscript
float get_attenuation() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bus {#get_bus}

```gdscript
StringName get_bus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_distance {#get_max_distance}

```gdscript
float get_max_distance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_polyphony {#get_max_polyphony}

```gdscript
int get_max_polyphony() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_panning_strength {#get_panning_strength}

```gdscript
float get_panning_strength() const
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

Returns the position in the AudioStream.

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

Returns the AudioStreamPlayback object associated with this AudioStreamPlayer2D.

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

Returns whether the AudioStreamPlayer can return the AudioStreamPlayback object or not.

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

Queues the audio to play on the next physics frame, from the given position from_position, in seconds.

### seek {#seek}

```gdscript
void seek(to_position: float)
```

Sets the position from which audio will be played, in seconds.

### set_area_mask {#set_area_mask}

```gdscript
void set_area_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_attenuation {#set_attenuation}

```gdscript
void set_attenuation(curve: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

### set_max_distance {#set_max_distance}

```gdscript
void set_max_distance(pixels: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_polyphony {#set_max_polyphony}

```gdscript
void set_max_polyphony(max_polyphony: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_panning_strength {#set_panning_strength}

```gdscript
void set_panning_strength(panning_strength: float)
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

Stops the audio.

## 属性

### stream {#stream}

```gdscript
stream: AudioStream
```

- void set_stream(value: AudioStream)
- AudioStream get_stream()

The AudioStream object to be played.

### volume_db {#volume_db}

```gdscript
volume_db: float
```

- void set_volume_db(value: float)
- float get_volume_db()

Base volume before attenuation, in decibels.

### volume_linear {#volume_linear}

```gdscript
volume_linear: float
```

- void set_volume_linear(value: float)
- float get_volume_linear()

Base volume before attenuation, as a linear value.

Note: This member modifies volume_db for convenience. The returned value is equivalent to the result of @GlobalScope.db_to_linear() on volume_db. Setting this member is equivalent to setting volume_db to the result of @GlobalScope.linear_to_db() on a value.

### pitch_scale {#pitch_scale}

```gdscript
pitch_scale: float
```

- void set_pitch_scale(value: float)
- float get_pitch_scale()

The pitch and the tempo of the audio, as a multiplier of the audio sample's sample rate.

### playing {#playing}

```gdscript
playing: bool
```

- void set_playing(value: bool)
- bool is_playing()

If true, audio is playing or is queued to be played (see play()).

### autoplay {#autoplay}

```gdscript
autoplay: bool
```

- void set_autoplay(value: bool)
- bool is_autoplay_enabled()

If true, audio plays when added to scene tree.

### stream_paused {#stream_paused}

```gdscript
stream_paused: bool
```

- void set_stream_paused(value: bool)
- bool get_stream_paused()

If true, the playback is paused. You can resume it by setting stream_paused to false.

### max_distance {#max_distance}

```gdscript
max_distance: float
```

- void set_max_distance(value: float)
- float get_max_distance()

Maximum distance from which audio is still hearable.

### attenuation {#attenuation}

```gdscript
attenuation: float
```

- void set_attenuation(value: float)
- float get_attenuation()

The volume is attenuated over distance with this as an exponent.

### max_polyphony {#max_polyphony}

```gdscript
max_polyphony: int
```

- void set_max_polyphony(value: int)
- int get_max_polyphony()

The maximum number of sounds this node can play at the same time. Playing additional sounds after this value is reached will cut off the oldest sounds.

### panning_strength {#panning_strength}

```gdscript
panning_strength: float
```

- void set_panning_strength(value: float)
- float get_panning_strength()

Scales the panning strength for this node by multiplying the base ProjectSettings.audio/general/2d_panning_strength with this factor. Higher values will pan audio from left to right more dramatically than lower values.

### bus {#bus}

```gdscript
bus: StringName
```

- void set_bus(value: StringName)
- StringName get_bus()

Bus on which this audio is playing.

Note: When setting this property, keep in mind that no validation is performed to see if the given name matches an existing bus. This is because audio bus layouts might be loaded after this property is set. If this given name can't be resolved at runtime, it will fall back to "Master".

### area_mask {#area_mask}

```gdscript
area_mask: int
```

- void set_area_mask(value: int)
- int get_area_mask()

Determines which Area2D layers affect the sound for reverb and audio bus effects. Areas can be used to redirect AudioStreams so that they play in a certain audio bus. An example of how you might use this is making a "water" area so that sounds played in the water are redirected through an audio bus to make them sound like they are being played underwater.

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

Emitted when the audio stops playing.
