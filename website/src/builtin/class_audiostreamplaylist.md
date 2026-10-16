# AudioStreamPlaylist

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamPlaylist**

> An audio stream that can play back sub-streams in sequence. Streams can be added to the Playlist with set_list_stream(), and shuffled with shuffle.

<details><summary>官方英文描述（中文精译进行中）</summary>

An audio stream that can play back sub-streams in sequence. Streams can be added to the Playlist with set_list_stream(), and shuffled with shuffle.

</details>

## 方法

### get_bpm {#get_bpm}

```gdscript
float get_bpm() const
```

Returns the BPM of the playlist, which can vary depending on the clip being played.

### get_fade_time {#get_fade_time}

```gdscript
float get_fade_time() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_list_stream {#get_list_stream}

```gdscript
AudioStream get_list_stream(stream_index: int) const
```

Returns the stream at playback position index.

### get_shuffle {#get_shuffle}

```gdscript
bool get_shuffle() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_stream_count {#get_stream_count}

```gdscript
int get_stream_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### has_loop {#has_loop}

```gdscript
bool has_loop() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fade_time {#set_fade_time}

```gdscript
void set_fade_time(dec: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_list_stream {#set_list_stream}

```gdscript
void set_list_stream(stream_index: int, audio_stream: AudioStream)
```

Sets the stream at playback position index.

### set_loop {#set_loop}

```gdscript
void set_loop(loop: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shuffle {#set_shuffle}

```gdscript
void set_shuffle(shuffle: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_stream_count {#set_stream_count}

```gdscript
void set_stream_count(stream_count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### shuffle {#shuffle}

```gdscript
shuffle: bool
```

- void set_shuffle(value: bool)
- bool get_shuffle()

If true, the playlist will shuffle each time playback starts and each time it loops.

### loop {#loop}

```gdscript
loop: bool
```

- void set_loop(value: bool)
- bool has_loop()

If true, the playlist will loop, otherwise the playlist will end when the last stream is finished.

### fade_time {#fade_time}

```gdscript
fade_time: float
```

- void set_fade_time(value: float)
- float get_fade_time()

Fade time used when a stream ends, when going to the next one. Streams are expected to have an extra bit of audio after the end to help with fading.

### stream_count {#stream_count}

```gdscript
stream_count: int
```

- void set_stream_count(value: int)
- int get_stream_count()

Amount of streams in the playlist.

### stream_0 {#stream_0}

```gdscript
stream_0: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_1 {#stream_1}

```gdscript
stream_1: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_2 {#stream_2}

```gdscript
stream_2: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_3 {#stream_3}

```gdscript
stream_3: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_4 {#stream_4}

```gdscript
stream_4: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_5 {#stream_5}

```gdscript
stream_5: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_6 {#stream_6}

```gdscript
stream_6: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_7 {#stream_7}

```gdscript
stream_7: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_8 {#stream_8}

```gdscript
stream_8: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_9 {#stream_9}

```gdscript
stream_9: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_10 {#stream_10}

```gdscript
stream_10: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_11 {#stream_11}

```gdscript
stream_11: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_12 {#stream_12}

```gdscript
stream_12: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_13 {#stream_13}

```gdscript
stream_13: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_14 {#stream_14}

```gdscript
stream_14: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_15 {#stream_15}

```gdscript
stream_15: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_16 {#stream_16}

```gdscript
stream_16: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_17 {#stream_17}

```gdscript
stream_17: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_18 {#stream_18}

```gdscript
stream_18: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_19 {#stream_19}

```gdscript
stream_19: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_20 {#stream_20}

```gdscript
stream_20: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_21 {#stream_21}

```gdscript
stream_21: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_22 {#stream_22}

```gdscript
stream_22: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_23 {#stream_23}

```gdscript
stream_23: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_24 {#stream_24}

```gdscript
stream_24: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_25 {#stream_25}

```gdscript
stream_25: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_26 {#stream_26}

```gdscript
stream_26: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_27 {#stream_27}

```gdscript
stream_27: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_28 {#stream_28}

```gdscript
stream_28: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_29 {#stream_29}

```gdscript
stream_29: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_30 {#stream_30}

```gdscript
stream_30: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_31 {#stream_31}

```gdscript
stream_31: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_32 {#stream_32}

```gdscript
stream_32: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_33 {#stream_33}

```gdscript
stream_33: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_34 {#stream_34}

```gdscript
stream_34: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_35 {#stream_35}

```gdscript
stream_35: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_36 {#stream_36}

```gdscript
stream_36: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_37 {#stream_37}

```gdscript
stream_37: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_38 {#stream_38}

```gdscript
stream_38: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_39 {#stream_39}

```gdscript
stream_39: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_40 {#stream_40}

```gdscript
stream_40: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_41 {#stream_41}

```gdscript
stream_41: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_42 {#stream_42}

```gdscript
stream_42: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_43 {#stream_43}

```gdscript
stream_43: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_44 {#stream_44}

```gdscript
stream_44: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_45 {#stream_45}

```gdscript
stream_45: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_46 {#stream_46}

```gdscript
stream_46: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_47 {#stream_47}

```gdscript
stream_47: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_48 {#stream_48}

```gdscript
stream_48: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_49 {#stream_49}

```gdscript
stream_49: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_50 {#stream_50}

```gdscript
stream_50: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_51 {#stream_51}

```gdscript
stream_51: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_52 {#stream_52}

```gdscript
stream_52: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_53 {#stream_53}

```gdscript
stream_53: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_54 {#stream_54}

```gdscript
stream_54: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_55 {#stream_55}

```gdscript
stream_55: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_56 {#stream_56}

```gdscript
stream_56: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_57 {#stream_57}

```gdscript
stream_57: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_58 {#stream_58}

```gdscript
stream_58: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_59 {#stream_59}

```gdscript
stream_59: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_60 {#stream_60}

```gdscript
stream_60: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_61 {#stream_61}

```gdscript
stream_61: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_62 {#stream_62}

```gdscript
stream_62: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stream_63 {#stream_63}

```gdscript
stream_63: AudioStream
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 常量

### MAX_STREAMS {#MAX_STREAMS}

```gdscript
const MAX_STREAMS = 64
```

Maximum amount of streams supported in the playlist.
