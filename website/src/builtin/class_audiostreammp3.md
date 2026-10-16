# AudioStreamMP3

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamMP3**

> MP3 audio stream driver. See data if you want to load an MP3 file at run-time. More info can be found in ResourceImporterMP3.

<details><summary>官方英文描述（中文精译进行中）</summary>

MP3 audio stream driver. See data if you want to load an MP3 file at run-time. More info can be found in ResourceImporterMP3.

Note: This class can optionally support legacy MP1 and MP2 formats, provided that the engine is compiled with the minimp3_extra_formats=yes SCons option. These extra formats are not enabled by default.

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

### get_data {#get_data}

```gdscript
PackedByteArray get_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_loop_offset {#get_loop_offset}

```gdscript
float get_loop_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_loop {#has_loop}

```gdscript
bool has_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### load_from_buffer {#load_from_buffer}

```gdscript
static AudioStreamMP3 load_from_buffer(stream_data: PackedByteArray)
```

Creates a new AudioStreamMP3 instance from the given buffer. The buffer must contain MP3 data.

### load_from_file {#load_from_file}

```gdscript
static AudioStreamMP3 load_from_file(path: String)
```

Creates a new AudioStreamMP3 instance from the given file path. The file must be in MP3 format.

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

### set_data {#set_data}

```gdscript
void set_data(data: PackedByteArray)
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

## 属性

### data {#data}

```gdscript
data: PackedByteArray
```

- void set_data(value: PackedByteArray)
- PackedByteArray get_data()

Contains the audio data in bytes.

You can load a file without having to import it beforehand using the code snippet below. Keep in mind that this snippet loads the whole file into memory and may not be ideal for huge files (hundreds of megabytes or more).

```
func load_mp3(path):
    var file = FileAccess.open(path, FileAccess.READ)
    var sound = AudioStreamMP3.new()
    sound.data = file.get_buffer(file.get_length())
    return sound
```

```
public AudioStreamMP3 LoadMP3(string path)
{
    using var file = FileAccess.Open(path, FileAccess.ModeFlags.Read);
    var sound = new AudioStreamMP3();
    sound.Data = file.GetBuffer(file.GetLength());
    return sound;
}
```

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

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
