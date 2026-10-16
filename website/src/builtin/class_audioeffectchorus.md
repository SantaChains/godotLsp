# AudioEffectChorus

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectChorus**

> A "chorus" effect creates multiple copies of the original audio (called "voices") with variations in pitch, and layers on top of the original, giving the impression that the sound comes from multiple sources. This creates spectral and spatial movement.

<details><summary>官方英文描述（中文精译进行中）</summary>

A "chorus" effect creates multiple copies of the original audio (called "voices") with variations in pitch, and layers on top of the original, giving the impression that the sound comes from multiple sources. This creates spectral and spatial movement.

Each voice is played a short period of time after the original audio, controlled by delay. An internal low-frequency oscillator (LFO) controls their pitch, and depth controls the LFO's maximum amount.

In the real world, this kind of effect is found in pianos, choirs, and instrument ensembles.

This effect can also be used to widen mono audio and make digital sounds have a more natural or analog quality.

</details>

## 方法

### get_dry {#get_dry}

```gdscript
float get_dry() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_voice_count {#get_voice_count}

```gdscript
int get_voice_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_voice_cutoff_hz {#get_voice_cutoff_hz}

```gdscript
float get_voice_cutoff_hz(voice_idx: int) const
```

Returns the frequency threshold of a given voice_idx's low-pass filter in Hz. Frequencies above this value are removed from the voice.

### get_voice_delay_ms {#get_voice_delay_ms}

```gdscript
float get_voice_delay_ms(voice_idx: int) const
```

Returns the delay of a given voice_idx in milliseconds, compared to the original audio.

### get_voice_depth_ms {#get_voice_depth_ms}

```gdscript
float get_voice_depth_ms(voice_idx: int) const
```

Returns the depth of a given voice_idx's low-frequency oscillator in milliseconds.

### get_voice_level_db {#get_voice_level_db}

```gdscript
float get_voice_level_db(voice_idx: int) const
```

Returns the gain of a given voice_idx in dB.

### get_voice_pan {#get_voice_pan}

```gdscript
float get_voice_pan(voice_idx: int) const
```

Returns the pan position of a given voice_idx. Negative values mean the left channel, positive mean the right.

### get_voice_rate_hz {#get_voice_rate_hz}

```gdscript
float get_voice_rate_hz(voice_idx: int) const
```

Returns the rate of a given voice_idx's low-frequency oscillator in Hz.

### get_wet {#get_wet}

```gdscript
float get_wet() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dry {#set_dry}

```gdscript
void set_dry(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_voice_count {#set_voice_count}

```gdscript
void set_voice_count(voices: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_voice_cutoff_hz {#set_voice_cutoff_hz}

```gdscript
void set_voice_cutoff_hz(voice_idx: int, cutoff_hz: float)
```

Sets the frequency threshold of a given voice_idx's low-pass filter in Hz. Frequencies above cutoff_hz are removed from voice_idx. Value can range from 1 to 20500.

### set_voice_delay_ms {#set_voice_delay_ms}

```gdscript
void set_voice_delay_ms(voice_idx: int, delay_ms: float)
```

Sets the delay of a given voice_idx in milliseconds, compared to the original audio. Value can range from 0 to 50.

### set_voice_depth_ms {#set_voice_depth_ms}

```gdscript
void set_voice_depth_ms(voice_idx: int, depth_ms: float)
```

Sets the depth of a given voice_idx's low-frequency oscillator in milliseconds. Value can range from 0 to 20.

### set_voice_level_db {#set_voice_level_db}

```gdscript
void set_voice_level_db(voice_idx: int, level_db: float)
```

Sets the gain of a given voice_idx in dB. Value can range from -60 to 24.

### set_voice_pan {#set_voice_pan}

```gdscript
void set_voice_pan(voice_idx: int, pan: float)
```

Sets the pan position of a given voice_idx. Negative values pan the sound to the left, positive pan to the right. Value can range from -1 to 1.

### set_voice_rate_hz {#set_voice_rate_hz}

```gdscript
void set_voice_rate_hz(voice_idx: int, rate_hz: float)
```

Sets the rate of a given voice_idx's low-frequency oscillator in Hz. Value can range from 0.1 to 20.

### set_wet {#set_wet}

```gdscript
void set_wet(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### voice_count {#voice_count}

```gdscript
voice_count: int
```

- void set_voice_count(value: int)
- int get_voice_count()

The number of voices in the effect. Value can range from 1 to 4.

### dry {#dry}

```gdscript
dry: float
```

- void set_dry(value: float)
- float get_dry()

The volume ratio of the original audio. Value can range from 0 to 1.

### wet {#wet}

```gdscript
wet: float
```

- void set_wet(value: float)
- float get_wet()

The volume ratio of all voices. Value can range from 0 to 1.
