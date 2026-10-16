# AudioStreamPolyphonic

**继承**：[AudioStream](class_audiostream.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamPolyphonic**

> AudioStream that lets the user play custom streams at any time from code, simultaneously using a single player.

<details><summary>官方英文描述（中文精译进行中）</summary>

AudioStream that lets the user play custom streams at any time from code, simultaneously using a single player.

Playback control is done via the AudioStreamPlaybackPolyphonic instance set inside the player, which can be obtained via AudioStreamPlayer.get_stream_playback(), AudioStreamPlayer2D.get_stream_playback() or AudioStreamPlayer3D.get_stream_playback() methods. Obtaining the playback instance is only valid after the stream property is set as an AudioStreamPolyphonic in those players.

</details>

## 方法

### get_polyphony {#get_polyphony}

```gdscript
int get_polyphony() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_polyphony {#set_polyphony}

```gdscript
void set_polyphony(voices: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### polyphony {#polyphony}

```gdscript
polyphony: int
```

- void set_polyphony(value: int)
- int get_polyphony()

Maximum amount of simultaneous streams that can be played.
