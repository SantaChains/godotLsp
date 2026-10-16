# AudioServer

**继承**：[Object](class_object.md) → **AudioServer**

> AudioServer is a low-level server interface for audio access. It is in charge of creating sample data (playable audio) as well as its playback via a voice interface.

<details><summary>官方英文描述（中文精译进行中）</summary>

AudioServer is a low-level server interface for audio access. It is in charge of creating sample data (playable audio) as well as its playback via a voice interface.

</details>

## 方法

### add_bus {#add_bus}

```gdscript
void add_bus(at_position: int = -1)
```

Adds a bus at at_position.

### add_bus_effect {#add_bus_effect}

```gdscript
void add_bus_effect(bus_idx: int, effect: AudioEffect, at_position: int = -1)
```

Adds an AudioEffect effect to the bus bus_idx at at_position.

### generate_bus_layout {#generate_bus_layout}

```gdscript
AudioBusLayout generate_bus_layout() const
```

Generates an AudioBusLayout using the available buses and effects.

### get_bus_channels {#get_bus_channels}

```gdscript
int get_bus_channels(bus_idx: int) const
```

Returns the number of channels of the bus at index bus_idx.

### get_bus_count {#get_bus_count}

```gdscript
int get_bus_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bus_effect {#get_bus_effect}

```gdscript
AudioEffect get_bus_effect(bus_idx: int, effect_idx: int)
```

Returns the AudioEffect at position effect_idx in bus bus_idx.

### get_bus_effect_count {#get_bus_effect_count}

```gdscript
int get_bus_effect_count(bus_idx: int)
```

Returns the number of effects on the bus at bus_idx.

### get_bus_effect_instance {#get_bus_effect_instance}

```gdscript
AudioEffectInstance get_bus_effect_instance(bus_idx: int, effect_idx: int, channel: int = 0)
```

Returns the AudioEffectInstance assigned to the given bus and effect indices (and optionally channel).

### get_bus_index {#get_bus_index}

```gdscript
int get_bus_index(bus_name: StringName) const
```

Returns the index of the bus with the name bus_name. Returns -1 if no bus with the specified name exist.

### get_bus_name {#get_bus_name}

```gdscript
String get_bus_name(bus_idx: int) const
```

Returns the name of the bus with the index bus_idx.

### get_bus_peak_volume_left_db {#get_bus_peak_volume_left_db}

```gdscript
float get_bus_peak_volume_left_db(bus_idx: int, channel: int) const
```

Returns the peak volume of the left speaker at bus index bus_idx and channel index channel.

### get_bus_peak_volume_right_db {#get_bus_peak_volume_right_db}

```gdscript
float get_bus_peak_volume_right_db(bus_idx: int, channel: int) const
```

Returns the peak volume of the right speaker at bus index bus_idx and channel index channel.

### get_bus_send {#get_bus_send}

```gdscript
StringName get_bus_send(bus_idx: int) const
```

Returns the name of the bus that the bus at index bus_idx sends to.

### get_bus_volume_db {#get_bus_volume_db}

```gdscript
float get_bus_volume_db(bus_idx: int) const
```

Returns the volume of the bus at index bus_idx in dB.

### get_bus_volume_linear {#get_bus_volume_linear}

```gdscript
float get_bus_volume_linear(bus_idx: int) const
```

Returns the volume of the bus at index bus_idx as a linear value.

Note: The returned value is equivalent to the result of @GlobalScope.db_to_linear() on the result of get_bus_volume_db().

### get_driver_name {#get_driver_name}

```gdscript
String get_driver_name() const
```

Returns the name of the current audio driver. The default usually depends on the operating system, but may be overridden via the --audio-driver command line argument. --headless also automatically sets the audio driver to Dummy. See also ProjectSettings.audio/driver/driver.

### get_input_buffer_length_frames {#get_input_buffer_length_frames}

```gdscript
int get_input_buffer_length_frames()
```

Experimental: This method may be changed or removed in future versions.

Returns the absolute size of the microphone input buffer. This is set to a multiple of the audio latency and can be used to estimate the minimum rate at which the frames need to be fetched.

### get_input_device {#get_input_device}

```gdscript
String get_input_device()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_input_device_list {#get_input_device_list}

```gdscript
PackedStringArray get_input_device_list()
```

Returns the names of all audio input devices detected on the system.

Note: ProjectSettings.audio/driver/enable_input must be true for audio input to work. See also that setting's description for caveats related to permissions and operating system privacy settings.

### get_input_frames {#get_input_frames}

```gdscript
PackedVector2Array get_input_frames(frames: int)
```

Experimental: This method may be changed or removed in future versions.

Returns a PackedVector2Array containing exactly frames audio samples from the internal microphone buffer if available, otherwise returns an empty PackedVector2Array.

The buffer is filled at the rate of get_input_mix_rate() frames per second when set_input_device_active() has successfully been set to true.

The samples are signed floating-point PCM values between -1 and 1.

### get_input_frames_available {#get_input_frames_available}

```gdscript
int get_input_frames_available()
```

Experimental: This method may be changed or removed in future versions.

Returns the number of frames available to read using get_input_frames().

### get_input_mix_rate {#get_input_mix_rate}

```gdscript
float get_input_mix_rate() const
```

Returns the sample rate at the input of the AudioServer.

### get_mix_rate {#get_mix_rate}

```gdscript
float get_mix_rate() const
```

Returns the sample rate at the output of the AudioServer.

### get_output_device {#get_output_device}

```gdscript
String get_output_device()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_output_device_list {#get_output_device_list}

```gdscript
PackedStringArray get_output_device_list()
```

Returns the names of all audio output devices detected on the system.

### get_output_latency {#get_output_latency}

```gdscript
float get_output_latency() const
```

Returns the audio driver's effective output latency. This is based on ProjectSettings.audio/driver/output_latency, but the exact returned value will differ depending on the operating system and audio driver.

Note: This can be expensive; it is not recommended to call get_output_latency() every frame.

### get_playback_speed_scale {#get_playback_speed_scale}

```gdscript
float get_playback_speed_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_speaker_mode {#get_speaker_mode}

```gdscript
AudioServer.SpeakerMode get_speaker_mode() const
```

Returns the speaker configuration.

### get_time_since_last_mix {#get_time_since_last_mix}

```gdscript
float get_time_since_last_mix() const
```

Returns the relative time since the last mix occurred, in seconds.

### get_time_to_next_mix {#get_time_to_next_mix}

```gdscript
float get_time_to_next_mix() const
```

Returns the relative time until the next mix occurs, in seconds.

### is_bus_bypassing_effects {#is_bus_bypassing_effects}

```gdscript
bool is_bus_bypassing_effects(bus_idx: int) const
```

If true, the bus at index bus_idx is bypassing effects.

### is_bus_effect_enabled {#is_bus_effect_enabled}

```gdscript
bool is_bus_effect_enabled(bus_idx: int, effect_idx: int) const
```

If true, the effect at index effect_idx on the bus at index bus_idx is enabled.

### is_bus_mute {#is_bus_mute}

```gdscript
bool is_bus_mute(bus_idx: int) const
```

If true, the bus at index bus_idx is muted.

### is_bus_solo {#is_bus_solo}

```gdscript
bool is_bus_solo(bus_idx: int) const
```

If true, the bus at index bus_idx is in solo mode.

### is_stream_registered_as_sample {#is_stream_registered_as_sample}

```gdscript
bool is_stream_registered_as_sample(stream: AudioStream)
```

Experimental: This method may be changed or removed in future versions.

If true, the stream is registered as a sample. The engine will not have to register it before playing the sample.

If false, the stream will have to be registered before playing it. To prevent lag spikes, register the stream as sample with register_stream_as_sample().

### lock {#lock}

```gdscript
void lock()
```

Locks the audio driver's main loop.

Note: Remember to unlock it afterwards.

### move_bus {#move_bus}

```gdscript
void move_bus(index: int, to_index: int)
```

Moves the bus from index index to index to_index.

### register_stream_as_sample {#register_stream_as_sample}

```gdscript
void register_stream_as_sample(stream: AudioStream)
```

Experimental: This method may be changed or removed in future versions.

Forces the registration of a stream as a sample.

Note: Lag spikes may occur when calling this method, especially on single-threaded builds. It is suggested to call this method while loading assets, where the lag spike could be masked, instead of registering the sample right before it needs to be played.

### remove_bus {#remove_bus}

```gdscript
void remove_bus(index: int)
```

Removes the bus at index index.

### remove_bus_effect {#remove_bus_effect}

```gdscript
void remove_bus_effect(bus_idx: int, effect_idx: int)
```

Removes the effect at index effect_idx from the bus at index bus_idx.

### set_bus_bypass_effects {#set_bus_bypass_effects}

```gdscript
void set_bus_bypass_effects(bus_idx: int, enable: bool)
```

If true, the bus at index bus_idx is bypassing effects.

### set_bus_count {#set_bus_count}

```gdscript
void set_bus_count(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bus_effect_enabled {#set_bus_effect_enabled}

```gdscript
void set_bus_effect_enabled(bus_idx: int, effect_idx: int, enabled: bool)
```

If true, the effect at index effect_idx on the bus at index bus_idx is enabled.

### set_bus_layout {#set_bus_layout}

```gdscript
void set_bus_layout(bus_layout: AudioBusLayout)
```

Overwrites the currently used AudioBusLayout.

### set_bus_mute {#set_bus_mute}

```gdscript
void set_bus_mute(bus_idx: int, enable: bool)
```

If true, the bus at index bus_idx is muted.

### set_bus_name {#set_bus_name}

```gdscript
void set_bus_name(bus_idx: int, name: String)
```

Sets the name of the bus at index bus_idx to name.

### set_bus_send {#set_bus_send}

```gdscript
void set_bus_send(bus_idx: int, send: StringName)
```

Connects the output of the bus at bus_idx to the bus named send.

### set_bus_solo {#set_bus_solo}

```gdscript
void set_bus_solo(bus_idx: int, enable: bool)
```

If true, the bus at index bus_idx is in solo mode.

### set_bus_volume_db {#set_bus_volume_db}

```gdscript
void set_bus_volume_db(bus_idx: int, volume_db: float)
```

Sets the volume in decibels of the bus at index bus_idx to volume_db.

### set_bus_volume_linear {#set_bus_volume_linear}

```gdscript
void set_bus_volume_linear(bus_idx: int, volume_linear: float)
```

Sets the volume as a linear value of the bus at index bus_idx to volume_linear.

Note: Using this method is equivalent to calling set_bus_volume_db() with the result of @GlobalScope.linear_to_db() on a value.

### set_enable_tagging_used_audio_streams {#set_enable_tagging_used_audio_streams}

```gdscript
void set_enable_tagging_used_audio_streams(enable: bool)
```

If set to true, all instances of AudioStreamPlayback will call AudioStreamPlayback._tag_used_streams() every mix step.

Note: This is enabled by default in the editor, as it is used by editor plugins for the audio stream previews.

### set_input_device {#set_input_device}

```gdscript
void set_input_device(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_input_device_active {#set_input_device_active}

```gdscript
Error set_input_device_active(active: bool)
```

Experimental: This method may be changed or removed in future versions.

If active is true, starts the microphone input stream specified by input_device or returns an error if it failed.

If active is false, stops the input stream if it is running.

### set_output_device {#set_output_device}

```gdscript
void set_output_device(name: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_playback_speed_scale {#set_playback_speed_scale}

```gdscript
void set_playback_speed_scale(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### swap_bus_effects {#swap_bus_effects}

```gdscript
void swap_bus_effects(bus_idx: int, effect_idx: int, by_effect_idx: int)
```

Swaps the position of two effects in bus bus_idx.

### unlock {#unlock}

```gdscript
void unlock()
```

Unlocks the audio driver's main loop. (After locking it, you should always unlock it.)

## 属性

### bus_count {#bus_count}

```gdscript
bus_count: int
```

- void set_bus_count(value: int)
- int get_bus_count()

Number of available audio buses.

### output_device {#output_device}

```gdscript
output_device: String
```

- void set_output_device(value: String)
- String get_output_device()

Name of the current device for audio output (see get_output_device_list()). On systems with multiple audio outputs (such as analog, USB and HDMI audio), this can be used to select the audio output device. The value "Default" will play audio on the system-wide default audio output. If an invalid device name is set, the value will be reverted back to "Default".

### input_device {#input_device}

```gdscript
input_device: String
```

- void set_input_device(value: String)
- String get_input_device()

Name of the current device for audio input (see get_input_device_list()). On systems with multiple audio inputs (such as analog, USB and HDMI audio), this can be used to select the audio input device. The value "Default" will record audio on the system-wide default audio input. If an invalid device name is set, the value will be reverted back to "Default".

Note: ProjectSettings.audio/driver/enable_input must be true for audio input to work. See also that setting's description for caveats related to permissions and operating system privacy settings.

### playback_speed_scale {#playback_speed_scale}

```gdscript
playback_speed_scale: float
```

- void set_playback_speed_scale(value: float)
- float get_playback_speed_scale()

Scales the rate at which audio is played (i.e. setting it to 0.5 will make the audio be played at half its speed). See also Engine.time_scale to affect the general simulation speed, which is independent from playback_speed_scale.

## 信号

### bus_layout_changed {#bus_layout_changed}

```gdscript
signal bus_layout_changed()
```

Emitted when an audio bus is added, deleted, or moved.

### bus_renamed {#bus_renamed}

```gdscript
signal bus_renamed(bus_index: int, old_name: StringName, new_name: StringName)
```

Emitted when the audio bus at bus_index is renamed from old_name to new_name.

## 枚举

### SpeakerMode {#enum-SpeakerMode}

```gdscript
enum SpeakerMode { SPEAKER_MODE_STEREO = 0, SPEAKER_SURROUND_31 = 1, SPEAKER_SURROUND_51 = 2, SPEAKER_SURROUND_71 = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SPEAKER_MODE_STEREO {#SPEAKER_MODE_STEREO}

```gdscript
SPEAKER_MODE_STEREO = 0
```

Two or fewer speakers were detected.

#### SPEAKER_SURROUND_31 {#SPEAKER_SURROUND_31}

```gdscript
SPEAKER_SURROUND_31 = 1
```

A 3.1 channel surround setup was detected.

#### SPEAKER_SURROUND_51 {#SPEAKER_SURROUND_51}

```gdscript
SPEAKER_SURROUND_51 = 2
```

A 5.1 channel surround setup was detected.

#### SPEAKER_SURROUND_71 {#SPEAKER_SURROUND_71}

```gdscript
SPEAKER_SURROUND_71 = 3
```

A 7.1 channel surround setup was detected.

---

### PlaybackType {#enum-PlaybackType}

```gdscript
enum PlaybackType { PLAYBACK_TYPE_DEFAULT = 0, PLAYBACK_TYPE_STREAM = 1, PLAYBACK_TYPE_SAMPLE = 2, PLAYBACK_TYPE_MAX = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PLAYBACK_TYPE_DEFAULT {#PLAYBACK_TYPE_DEFAULT}

```gdscript
PLAYBACK_TYPE_DEFAULT = 0
```

Experimental: This constant may be changed or removed in future versions.

The playback will be considered of the type declared at ProjectSettings.audio/general/default_playback_type.

#### PLAYBACK_TYPE_STREAM {#PLAYBACK_TYPE_STREAM}

```gdscript
PLAYBACK_TYPE_STREAM = 1
```

Experimental: This constant may be changed or removed in future versions.

Force the playback to be considered as a stream.

#### PLAYBACK_TYPE_SAMPLE {#PLAYBACK_TYPE_SAMPLE}

```gdscript
PLAYBACK_TYPE_SAMPLE = 2
```

Experimental: This constant may be changed or removed in future versions.

Force the playback to be considered as a sample. This can provide lower latency and more stable playback (with less risk of audio crackling), at the cost of having less flexibility.

Note: Only currently supported on the web platform.

Note: AudioEffects are not supported when playback is considered as a sample.

#### PLAYBACK_TYPE_MAX {#PLAYBACK_TYPE_MAX}

```gdscript
PLAYBACK_TYPE_MAX = 3
```

Experimental: This constant may be changed or removed in future versions.

Represents the size of the PlaybackType enum.

---
