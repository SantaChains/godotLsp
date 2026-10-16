# AudioStreamOggVorbis

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamOggVorbis**

> The AudioStreamOggVorbis class is a specialized AudioStream for handling Ogg Vorbis file formats. It offers functionality for loading and playing back Ogg Vorbis files, as well as managing looping and other playback properties. More info can be found in ResourceImporterOggVorbis.

<details><summary>官方英文描述</summary>

The AudioStreamOggVorbis class is a specialized AudioStream for handling Ogg Vorbis file formats. It offers functionality for loading and playing back Ogg Vorbis files, as well as managing looping and other playback properties. More info can be found in ResourceImporterOggVorbis.

This class is part of the audio stream system, which also supports WAV files through the AudioStreamWAV class, and MP3 files through the AudioStreamMP3 class.

</details>

## 方法

### get_bar_beats {#get_bar_beats}

```gdscript
int get_bar_beats() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_beat_count {#get_beat_count}

```gdscript
int get_beat_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_bpm {#get_bpm}

```gdscript
float get_bpm() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_loop_offset {#get_loop_offset}

```gdscript
float get_loop_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_packet_sequence {#get_packet_sequence}

```gdscript
OggPacketSequence get_packet_sequence() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tags {#get_tags}

```gdscript
Dictionary get_tags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_loop {#has_loop}

```gdscript
bool has_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### load_from_buffer {#load_from_buffer}

```gdscript
static AudioStreamOggVorbis load_from_buffer(stream_data: PackedByteArray)
```

Creates a new AudioStreamOggVorbis instance from the given buffer. The buffer must contain Ogg Vorbis data.

### load_from_file {#load_from_file}

```gdscript
static AudioStreamOggVorbis load_from_file(path: String)
```

Creates a new AudioStreamOggVorbis instance from the given file path. The file must be in Ogg Vorbis format.

### set_bar_beats {#set_bar_beats}

```gdscript
void set_bar_beats(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_beat_count {#set_beat_count}

```gdscript
void set_beat_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bpm {#set_bpm}

```gdscript
void set_bpm(bpm: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop {#set_loop}

```gdscript
void set_loop(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop_offset {#set_loop_offset}

```gdscript
void set_loop_offset(seconds: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_packet_sequence {#set_packet_sequence}

```gdscript
void set_packet_sequence(packet_sequence: OggPacketSequence)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tags {#set_tags}

```gdscript
void set_tags(tags: Dictionary)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### packet_sequence {#packet_sequence}

```gdscript
packet_sequence: Object
```

- void set_packet_sequence(value: OggPacketSequence)
- OggPacketSequence get_packet_sequence()

Contains the raw Ogg data for this stream.

### bpm {#bpm}

```gdscript
bpm: float
```

- void set_bpm(value: float)
- float get_bpm()

The tempo of the audio track, measured in beats per minute.

### beat_count {#beat_count}

```gdscript
beat_count: int
```

- void set_beat_count(value: int)
- int get_beat_count()

The length of the audio track, in beats. The actual duration of the audio file might be longer than what is indicated by this property. It defines the end of the audio for looping, AudioStreamPlaylist, and AudioStreamInteractive.

### bar_beats {#bar_beats}

```gdscript
bar_beats: int
```

- void set_bar_beats(value: int)
- int get_bar_beats()

The number of beats within a single bar in the audio track.

### tags {#tags}

```gdscript
tags: Dictionary
```

- void set_tags(value: Dictionary)
- Dictionary get_tags()

Contains user-defined tags if found in the Ogg Vorbis data.

Commonly used tags include title, artist, album, tracknumber, and date (date does not have a standard date format).

Note: No tag is guaranteed to be present in every file, so make sure to account for the keys not always existing.

### loop {#loop}

```gdscript
loop: bool
```

- void set_loop(value: bool)
- bool has_loop()

If true, the stream will play again from the specified loop_offset once it reaches the end of the audio track, or once it reaches the end of the last beat according to the amount specified in beat_count. Useful for ambient sounds and background music.

### loop_offset {#loop_offset}

```gdscript
loop_offset: float
```

- void set_loop_offset(value: float)
- float get_loop_offset()

Time in seconds at which the stream starts after being looped.
