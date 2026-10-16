# AudioStreamRandomizer

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamRandomizer**

> Picks a random AudioStream from the pool, depending on the playback mode, and applies random pitch shifting and volume shifting during playback.

<details><summary>官方英文描述</summary>

Picks a random AudioStream from the pool, depending on the playback mode, and applies random pitch shifting and volume shifting during playback.

</details>

## 方法

### add_stream {#add_stream}

```gdscript
void add_stream(index: int, stream: AudioStream, weight: float = 1.0)
```

Insert a stream at the specified index. If the index is less than zero, the insertion occurs at the end of the underlying pool.

### get_playback_mode {#get_playback_mode}

```gdscript
AudioStreamRandomizer.PlaybackMode get_playback_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_random_pitch {#get_random_pitch}

```gdscript
float get_random_pitch() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_random_pitch_semitones {#get_random_pitch_semitones}

```gdscript
float get_random_pitch_semitones() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_random_volume_offset_db {#get_random_volume_offset_db}

```gdscript
float get_random_volume_offset_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream {#get_stream}

```gdscript
AudioStream get_stream(index: int) const
```

Returns the stream at the specified index.

### get_stream_probability_weight {#get_stream_probability_weight}

```gdscript
float get_stream_probability_weight(index: int) const
```

Returns the probability weight associated with the stream at the given index.

### get_streams_count {#get_streams_count}

```gdscript
int get_streams_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### move_stream {#move_stream}

```gdscript
void move_stream(index_from: int, index_to: int)
```

Move a stream from one index to another.

### remove_stream {#remove_stream}

```gdscript
void remove_stream(index: int)
```

Remove the stream at the specified index.

### set_playback_mode {#set_playback_mode}

```gdscript
void set_playback_mode(mode: AudioStreamRandomizer.PlaybackMode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_random_pitch {#set_random_pitch}

```gdscript
void set_random_pitch(scale: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_random_pitch_semitones {#set_random_pitch_semitones}

```gdscript
void set_random_pitch_semitones(semitones: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_random_volume_offset_db {#set_random_volume_offset_db}

```gdscript
void set_random_volume_offset_db(db_offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stream {#set_stream}

```gdscript
void set_stream(index: int, stream: AudioStream)
```

Set the AudioStream at the specified index.

### set_stream_probability_weight {#set_stream_probability_weight}

```gdscript
void set_stream_probability_weight(index: int, weight: float)
```

Set the probability weight of the stream at the specified index. The higher this value, the more likely that the randomizer will choose this stream during random playback modes.

### set_streams_count {#set_streams_count}

```gdscript
void set_streams_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### playback_mode {#playback_mode}

```gdscript
playback_mode: int
```

- void set_playback_mode(value: PlaybackMode)
- PlaybackMode get_playback_mode()

Controls how this AudioStreamRandomizer picks which AudioStream to play next.

### random_pitch {#random_pitch}

```gdscript
random_pitch: float
```

- void set_random_pitch(value: float)
- float get_random_pitch()

The largest possible frequency multiplier of the random pitch variation. Pitch will be randomly chosen within a range of 1.0 / random_pitch and random_pitch. A value of 1.0 means no variation. A value of 2.0 means pitch will be randomized between double and half.

Note: Setting this property also sets random_pitch_semitones.

### random_pitch_semitones {#random_pitch_semitones}

```gdscript
random_pitch_semitones: float
```

- void set_random_pitch_semitones(value: float)
- float get_random_pitch_semitones()

The largest possible distance, in semitones, of the random pitch variation. A value of 0.0 means no variation.

Note: Setting this property also sets random_pitch.

### random_volume_offset_db {#random_volume_offset_db}

```gdscript
random_volume_offset_db: float
```

- void set_random_volume_offset_db(value: float)
- float get_random_volume_offset_db()

The intensity of random volume variation. Volume will be increased or decreased by a random value up to random_volume_offset_db. A value of 0.0 means no variation. A value of 3.0 means volume will be randomized between -3.0 dB and +3.0 dB.

### streams_count {#streams_count}

```gdscript
streams_count: int
```

- void set_streams_count(value: int)
- int get_streams_count()

The number of streams in the stream pool.

## 枚举

### PlaybackMode {#enum-PlaybackMode}

```gdscript
enum PlaybackMode { PLAYBACK_RANDOM_NO_REPEATS = 0, PLAYBACK_RANDOM = 1, PLAYBACK_SEQUENTIAL = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PLAYBACK_RANDOM_NO_REPEATS {#PLAYBACK_RANDOM_NO_REPEATS}

```gdscript
PLAYBACK_RANDOM_NO_REPEATS = 0
```

Pick a stream at random according to the probability weights chosen for each stream, but avoid playing the same stream twice in a row whenever possible. If only 1 sound is present in the pool, the same sound will always play, effectively allowing repeats to occur.

#### PLAYBACK_RANDOM {#PLAYBACK_RANDOM}

```gdscript
PLAYBACK_RANDOM = 1
```

Pick a stream at random according to the probability weights chosen for each stream. If only 1 sound is present in the pool, the same sound will always play.

#### PLAYBACK_SEQUENTIAL {#PLAYBACK_SEQUENTIAL}

```gdscript
PLAYBACK_SEQUENTIAL = 2
```

Play streams in the order they appear in the stream pool. If only 1 sound is present in the pool, the same sound will always play.

---
