# AudioStreamWAV

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamWAV**

> AudioStreamWAV stores sound samples loaded from WAV files. To play the stored sound, use an AudioStreamPlayer (for non-positional audio) or AudioStreamPlayer2D/AudioStreamPlayer3D (for positional audio). The sound can be looped.

<details><summary>官方英文描述（中文精译进行中）</summary>

AudioStreamWAV stores sound samples loaded from WAV files. To play the stored sound, use an AudioStreamPlayer (for non-positional audio) or AudioStreamPlayer2D/AudioStreamPlayer3D (for positional audio). The sound can be looped.

This class can also be used to store dynamically-generated PCM audio data. See also AudioStreamGenerator for procedural audio generation.

</details>

## 方法

### get_data {#get_data}

```gdscript
PackedByteArray get_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_format {#get_format}

```gdscript
AudioStreamWAV.Format get_format() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_loop_begin {#get_loop_begin}

```gdscript
int get_loop_begin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_loop_end {#get_loop_end}

```gdscript
int get_loop_end() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_loop_mode {#get_loop_mode}

```gdscript
AudioStreamWAV.LoopMode get_loop_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mix_rate {#get_mix_rate}

```gdscript
int get_mix_rate() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_tags {#get_tags}

```gdscript
Dictionary get_tags() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_stereo {#is_stereo}

```gdscript
bool is_stereo() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### load_from_buffer {#load_from_buffer}

```gdscript
static AudioStreamWAV load_from_buffer(stream_data: PackedByteArray, options: Dictionary = {})
```

Creates a new AudioStreamWAV instance from the given buffer. The buffer must contain WAV data.

The keys and values of options match the properties of ResourceImporterWAV. The usage of options is identical to load_from_file().

### load_from_file {#load_from_file}

```gdscript
static AudioStreamWAV load_from_file(path: String, options: Dictionary = {})
```

Creates a new AudioStreamWAV instance from the given file path. The file must be in WAV format.

The keys and values of options match the properties of ResourceImporterWAV.

Example: Load the first file dropped as a WAV and play it:

```gdscript
@onready var audio_player = $AudioStreamPlayer
func _ready():
    get_window().files_dropped.connect(_on_files_dropped)
func _on_files_dropped(files):
    if files[0].get_extension() == "wav":
        audio_player.stream = AudioStreamWAV.load_from_file(files[0], {
                "force/max_rate": true,
                "force/max_rate_hz": 11025
            })
        audio_player.play()
```

### save_to_wav {#save_to_wav}

```gdscript
Error save_to_wav(path: String)
```

Saves the AudioStreamWAV as a WAV file to path. Samples with IMA ADPCM or Quite OK Audio formats can't be saved.

Note: A .wav extension is automatically appended to path if it is missing.

### set_data {#set_data}

```gdscript
void set_data(data: PackedByteArray)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_format {#set_format}

```gdscript
void set_format(format: AudioStreamWAV.Format)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop_begin {#set_loop_begin}

```gdscript
void set_loop_begin(loop_begin: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop_end {#set_loop_end}

```gdscript
void set_loop_end(loop_end: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_loop_mode {#set_loop_mode}

```gdscript
void set_loop_mode(loop_mode: AudioStreamWAV.LoopMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mix_rate {#set_mix_rate}

```gdscript
void set_mix_rate(mix_rate: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stereo {#set_stereo}

```gdscript
void set_stereo(stereo: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_tags {#set_tags}

```gdscript
void set_tags(tags: Dictionary)
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

Note: If format is set to FORMAT_8_BITS, this property expects signed 8-bit PCM data. To convert from unsigned 8-bit PCM, subtract 128 from each byte.

Note: If format is set to FORMAT_QOA, this property expects data from a full QOA file.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedByteArray for more details.

### format {#format}

```gdscript
format: int
```

- void set_format(value: Format)
- Format get_format()

Audio format.

### loop_mode {#loop_mode}

```gdscript
loop_mode: int
```

- void set_loop_mode(value: LoopMode)
- LoopMode get_loop_mode()

The loop mode.

### loop_begin {#loop_begin}

```gdscript
loop_begin: int
```

- void set_loop_begin(value: int)
- int get_loop_begin()

The loop start point (in number of samples, relative to the beginning of the stream).

### loop_end {#loop_end}

```gdscript
loop_end: int
```

- void set_loop_end(value: int)
- int get_loop_end()

The loop end point (in number of samples, relative to the beginning of the stream).

### mix_rate {#mix_rate}

```gdscript
mix_rate: int
```

- void set_mix_rate(value: int)
- int get_mix_rate()

The sample rate for mixing this audio. Higher values require more storage space, but result in better quality.

In games, common sample rates in use are 11025, 16000, 22050, 32000, 44100, and 48000.

According to the Nyquist-Shannon sampling theorem [https://en.wikipedia.org/wiki/Nyquist%E2%80%93Shannon_sampling_theorem], there is no quality difference to human hearing when going past 40,000 Hz (since most humans can only hear up to ~20,000 Hz, often less). If you are using lower-pitched sounds such as voices, lower sample rates such as 32000 or 22050 may be usable with no loss in quality.

### stereo {#stereo}

```gdscript
stereo: bool
```

- void set_stereo(value: bool)
- bool is_stereo()

If true, audio is stereo.

### tags {#tags}

```gdscript
tags: Dictionary
```

- void set_tags(value: Dictionary)
- Dictionary get_tags()

Contains user-defined tags if found in the WAV data.

Commonly used tags include title, artist, album, tracknumber, and date (date does not have a standard date format).

Note: No tag is guaranteed to be present in every file, so make sure to account for the keys not always existing.

Note: Only WAV files using a LIST chunk with an identifier of INFO to encode the tags are currently supported.

## 枚举

### Format {#enum-Format}

```gdscript
enum Format { FORMAT_8_BITS = 0, FORMAT_16_BITS = 1, FORMAT_IMA_ADPCM = 2, FORMAT_QOA = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FORMAT_8_BITS {#FORMAT_8_BITS}

```gdscript
FORMAT_8_BITS = 0
```

8-bit PCM audio codec.

#### FORMAT_16_BITS {#FORMAT_16_BITS}

```gdscript
FORMAT_16_BITS = 1
```

16-bit PCM audio codec.

#### FORMAT_IMA_ADPCM {#FORMAT_IMA_ADPCM}

```gdscript
FORMAT_IMA_ADPCM = 2
```

Audio is lossily compressed as IMA ADPCM.

#### FORMAT_QOA {#FORMAT_QOA}

```gdscript
FORMAT_QOA = 3
```

Audio is lossily compressed as Quite OK Audio [https://qoaformat.org/].

---

### LoopMode {#enum-LoopMode}

```gdscript
enum LoopMode { LOOP_DISABLED = 0, LOOP_FORWARD = 1, LOOP_PINGPONG = 2, LOOP_BACKWARD = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### LOOP_DISABLED {#LOOP_DISABLED}

```gdscript
LOOP_DISABLED = 0
```

Audio does not loop.

#### LOOP_FORWARD {#LOOP_FORWARD}

```gdscript
LOOP_FORWARD = 1
```

Audio loops the data between loop_begin and loop_end, playing forward only.

#### LOOP_PINGPONG {#LOOP_PINGPONG}

```gdscript
LOOP_PINGPONG = 2
```

Audio loops the data between loop_begin and loop_end, playing back and forth.

#### LOOP_BACKWARD {#LOOP_BACKWARD}

```gdscript
LOOP_BACKWARD = 3
```

Audio loops the data between loop_begin and loop_end, playing backward only.

---
