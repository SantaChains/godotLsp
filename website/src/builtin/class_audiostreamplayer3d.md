# AudioStreamPlayer3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **AudioStreamPlayer3D**

> Plays audio with positional sound effects, based on the relative position of the audio listener. Positional effects include distance attenuation, directionality, and the Doppler effect. For greater realism, a low-pass filter is applied to distant sounds. This can be disabled by setting attenuation_filter_cutoff_hz to 20500.

<details><summary>官方英文描述（中文精译进行中）</summary>

Plays audio with positional sound effects, based on the relative position of the audio listener. Positional effects include distance attenuation, directionality, and the Doppler effect. For greater realism, a low-pass filter is applied to distant sounds. This can be disabled by setting attenuation_filter_cutoff_hz to 20500.

By default, audio is heard from the camera position. This can be changed by adding an AudioListener3D node to the scene and enabling it by calling AudioListener3D.make_current() on it.

See also AudioStreamPlayer to play a sound non-positionally.

Note: Hiding an AudioStreamPlayer3D node does not disable its audio output. To temporarily disable an AudioStreamPlayer3D's audio output, set volume_db to a very low value like -100 (which isn't audible to human hearing).

</details>

## 方法

### get_area_mask {#get_area_mask}

```gdscript
int get_area_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_attenuation_filter_cutoff_hz {#get_attenuation_filter_cutoff_hz}

```gdscript
float get_attenuation_filter_cutoff_hz() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_attenuation_filter_db {#get_attenuation_filter_db}

```gdscript
float get_attenuation_filter_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_attenuation_model {#get_attenuation_model}

```gdscript
AudioStreamPlayer3D.AttenuationModel get_attenuation_model() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bus {#get_bus}

```gdscript
StringName get_bus() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_doppler_tracking {#get_doppler_tracking}

```gdscript
AudioStreamPlayer3D.DopplerTracking get_doppler_tracking() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_angle {#get_emission_angle}

```gdscript
float get_emission_angle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_emission_angle_filter_attenuation_db {#get_emission_angle_filter_attenuation_db}

```gdscript
float get_emission_angle_filter_attenuation_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_max_db {#get_max_db}

```gdscript
float get_max_db() const
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

Returns the AudioStreamPlayback object associated with this AudioStreamPlayer3D.

### get_unit_size {#get_unit_size}

```gdscript
float get_unit_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

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

### is_emission_angle_enabled {#is_emission_angle_enabled}

```gdscript
bool is_emission_angle_enabled() const
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

### set_attenuation_filter_cutoff_hz {#set_attenuation_filter_cutoff_hz}

```gdscript
void set_attenuation_filter_cutoff_hz(degrees: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_attenuation_filter_db {#set_attenuation_filter_db}

```gdscript
void set_attenuation_filter_db(db: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_attenuation_model {#set_attenuation_model}

```gdscript
void set_attenuation_model(model: AudioStreamPlayer3D.AttenuationModel)
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

### set_doppler_tracking {#set_doppler_tracking}

```gdscript
void set_doppler_tracking(mode: AudioStreamPlayer3D.DopplerTracking)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_angle {#set_emission_angle}

```gdscript
void set_emission_angle(degrees: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_angle_enabled {#set_emission_angle_enabled}

```gdscript
void set_emission_angle_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_emission_angle_filter_attenuation_db {#set_emission_angle_filter_attenuation_db}

```gdscript
void set_emission_angle_filter_attenuation_db(db: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_db {#set_max_db}

```gdscript
void set_max_db(max_db: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_max_distance {#set_max_distance}

```gdscript
void set_max_distance(meters: float)
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

### set_unit_size {#set_unit_size}

```gdscript
void set_unit_size(unit_size: float)
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

The AudioStream resource to be played.

### attenuation_model {#attenuation_model}

```gdscript
attenuation_model: int
```

- void set_attenuation_model(value: AttenuationModel)
- AttenuationModel get_attenuation_model()

Decides if audio should get quieter with distance linearly, quadratically, logarithmically, or not be affected by distance, effectively disabling attenuation.

### volume_db {#volume_db}

```gdscript
volume_db: float
```

- void set_volume_db(value: float)
- float get_volume_db()

The base sound level before attenuation, in decibels.

### volume_linear {#volume_linear}

```gdscript
volume_linear: float
```

- void set_volume_linear(value: float)
- float get_volume_linear()

The base sound level before attenuation, as a linear value.

Note: This member modifies volume_db for convenience. The returned value is equivalent to the result of @GlobalScope.db_to_linear() on volume_db. Setting this member is equivalent to setting volume_db to the result of @GlobalScope.linear_to_db() on a value.

### unit_size {#unit_size}

```gdscript
unit_size: float
```

- void set_unit_size(value: float)
- float get_unit_size()

The factor for the attenuation effect. Higher values make the sound audible over a larger distance.

### max_db {#max_db}

```gdscript
max_db: float
```

- void set_max_db(value: float)
- float get_max_db()

Sets the absolute maximum of the sound level, in decibels.

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

If true, audio plays when the AudioStreamPlayer3D node is added to scene tree.

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

The distance past which the sound can no longer be heard at all. Only has an effect if set to a value greater than 0.0. max_distance works in tandem with unit_size. However, unlike unit_size whose behavior depends on the attenuation_model, max_distance always works in a linear fashion. This can be used to prevent the AudioStreamPlayer3D from requiring audio mixing when the listener is far away, which saves CPU resources.

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

Scales the panning strength for this node by multiplying the base ProjectSettings.audio/general/3d_panning_strength by this factor. If the product is 0.0 then stereo panning is disabled and the volume is the same for all channels. If the product is 1.0 then one of the channels will be muted when the sound is located exactly to the left (or right) of the listener.

Two speaker stereo arrangements implement the WebAudio standard for StereoPannerNode Panning [https://webaudio.github.io/web-audio-api/#stereopanner-algorithm] where the volume is cosine of half the azimuth angle to the ear.

For other speaker arrangements such as the 5.1 and 7.1 the SPCAP (Speaker-Placement Correction Amplitude) algorithm is implemented.

### bus {#bus}

```gdscript
bus: StringName
```

- void set_bus(value: StringName)
- StringName get_bus()

The bus on which this audio is playing.

Note: When setting this property, keep in mind that no validation is performed to see if the given name matches an existing bus. This is because audio bus layouts might be loaded after this property is set. If this given name can't be resolved at runtime, it will fall back to "Master".

### area_mask {#area_mask}

```gdscript
area_mask: int
```

- void set_area_mask(value: int)
- int get_area_mask()

Determines which Area3D layers affect the sound for reverb and audio bus effects. Areas can be used to redirect AudioStreams so that they play in a certain audio bus. An example of how you might use this is making a "water" area so that sounds played in the water are redirected through an audio bus to make them sound like they are being played underwater.

### playback_type {#playback_type}

```gdscript
playback_type: int
```

- void set_playback_type(value: PlaybackType)
- PlaybackType get_playback_type()

Experimental: This property may be changed or removed in future versions.

The playback type of the stream player. If set other than to the default value, it will force that playback type.

### emission_angle_enabled {#emission_angle_enabled}

```gdscript
emission_angle_enabled: bool
```

- void set_emission_angle_enabled(value: bool)
- bool is_emission_angle_enabled()

If true, the audio should be attenuated according to the direction of the sound.

### emission_angle_degrees {#emission_angle_degrees}

```gdscript
emission_angle_degrees: float
```

- void set_emission_angle(value: float)
- float get_emission_angle()

The angle in which the audio reaches a listener unattenuated.

### emission_angle_filter_attenuation_db {#emission_angle_filter_attenuation_db}

```gdscript
emission_angle_filter_attenuation_db: float
```

- void set_emission_angle_filter_attenuation_db(value: float)
- float get_emission_angle_filter_attenuation_db()

Attenuation factor used if listener is outside of emission_angle_degrees and emission_angle_enabled is set, in decibels.

### attenuation_filter_cutoff_hz {#attenuation_filter_cutoff_hz}

```gdscript
attenuation_filter_cutoff_hz: float
```

- void set_attenuation_filter_cutoff_hz(value: float)
- float get_attenuation_filter_cutoff_hz()

The cutoff frequency of the attenuation low-pass filter, in Hz. A sound above this frequency is attenuated more than a sound below this frequency. To disable this effect, set this to 20500 as this frequency is above the human hearing limit.

### attenuation_filter_db {#attenuation_filter_db}

```gdscript
attenuation_filter_db: float
```

- void set_attenuation_filter_db(value: float)
- float get_attenuation_filter_db()

Amount how much the filter affects the loudness, in decibels.

### doppler_tracking {#doppler_tracking}

```gdscript
doppler_tracking: int
```

- void set_doppler_tracking(value: DopplerTracking)
- DopplerTracking get_doppler_tracking()

Decides in which step the Doppler effect should be calculated.

Note: If doppler_tracking is not DOPPLER_TRACKING_DISABLED but the current Camera3D/AudioListener3D has doppler tracking disabled, the Doppler effect will be heard but will not take the movement of the current listener into account. If accurate Doppler effect is desired, doppler tracking should be enabled on both the AudioStreamPlayer3D and the current Camera3D/AudioListener3D.

## 信号

### finished {#finished}

```gdscript
signal finished()
```

Emitted when the audio stops playing.

## 枚举

### AttenuationModel {#enum-AttenuationModel}

```gdscript
enum AttenuationModel { ATTENUATION_INVERSE_DISTANCE = 0, ATTENUATION_INVERSE_SQUARE_DISTANCE = 1, ATTENUATION_LOGARITHMIC = 2, ATTENUATION_DISABLED = 3 }
```

AttenuationModel ATTENUATION_INVERSE_DISTANCE = 0

Attenuation of loudness according to linear distance.

AttenuationModel ATTENUATION_INVERSE_SQUARE_DISTANCE = 1

Attenuation of loudness according to squared distance.

AttenuationModel ATTENUATION_LOGARITHMIC = 2

Attenuation of loudness according to logarithmic distance.

AttenuationModel ATTENUATION_DISABLED = 3

No attenuation of loudness according to distance. The sound will still be heard positionally, unlike an AudioStreamPlayer. ATTENUATION_DISABLED can be combined with a max_distance value greater than 0.0 to achieve linear attenuation clamped to a sphere of a defined size.

#### ATTENUATION_INVERSE_DISTANCE {#ATTENUATION_INVERSE_DISTANCE}

```gdscript
ATTENUATION_INVERSE_DISTANCE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ATTENUATION_INVERSE_SQUARE_DISTANCE {#ATTENUATION_INVERSE_SQUARE_DISTANCE}

```gdscript
ATTENUATION_INVERSE_SQUARE_DISTANCE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ATTENUATION_LOGARITHMIC {#ATTENUATION_LOGARITHMIC}

```gdscript
ATTENUATION_LOGARITHMIC = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### ATTENUATION_DISABLED {#ATTENUATION_DISABLED}

```gdscript
ATTENUATION_DISABLED = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DopplerTracking {#enum-DopplerTracking}

```gdscript
enum DopplerTracking { DOPPLER_TRACKING_DISABLED = 0, DOPPLER_TRACKING_IDLE_STEP = 1, DOPPLER_TRACKING_PHYSICS_STEP = 2 }
```

DopplerTracking DOPPLER_TRACKING_DISABLED = 0

Disables doppler tracking.

DopplerTracking DOPPLER_TRACKING_IDLE_STEP = 1

Executes doppler tracking during process frames (see Node.NOTIFICATION_INTERNAL_PROCESS).

DopplerTracking DOPPLER_TRACKING_PHYSICS_STEP = 2

Executes doppler tracking during physics frames (see Node.NOTIFICATION_INTERNAL_PHYSICS_PROCESS).

#### DOPPLER_TRACKING_DISABLED {#DOPPLER_TRACKING_DISABLED}

```gdscript
DOPPLER_TRACKING_DISABLED = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOPPLER_TRACKING_IDLE_STEP {#DOPPLER_TRACKING_IDLE_STEP}

```gdscript
DOPPLER_TRACKING_IDLE_STEP = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### DOPPLER_TRACKING_PHYSICS_STEP {#DOPPLER_TRACKING_PHYSICS_STEP}

```gdscript
DOPPLER_TRACKING_PHYSICS_STEP = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
