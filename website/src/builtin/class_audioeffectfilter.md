# AudioEffectFilter

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectFilter**

> A "filter" controls the gain of frequencies, using cutoff_hz as a frequency threshold. Filters can help to give room for each sound, and create interesting effects.

<details><summary>官方英文描述（中文精译进行中）</summary>

A "filter" controls the gain of frequencies, using cutoff_hz as a frequency threshold. Filters can help to give room for each sound, and create interesting effects.

There are different types of filter that inherit this class:

Shelf filters: AudioEffectLowShelfFilter and AudioEffectHighShelfFilter

Band-pass and notch filters: AudioEffectBandPassFilter, AudioEffectBandLimitFilter, and AudioEffectNotchFilter

Low/high-pass filters: AudioEffectLowPassFilter and AudioEffectHighPassFilter

</details>

## 方法

### get_cutoff {#get_cutoff}

```gdscript
float get_cutoff() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_db {#get_db}

```gdscript
AudioEffectFilter.FilterDB get_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gain {#get_gain}

```gdscript
float get_gain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_resonance {#get_resonance}

```gdscript
float get_resonance() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_cutoff {#set_cutoff}

```gdscript
void set_cutoff(freq: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_db {#set_db}

```gdscript
void set_db(amount: AudioEffectFilter.FilterDB)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gain {#set_gain}

```gdscript
void set_gain(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_resonance {#set_resonance}

```gdscript
void set_resonance(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### cutoff_hz {#cutoff_hz}

```gdscript
cutoff_hz: float
```

- void set_cutoff(value: float)
- float get_cutoff()

Frequency threshold for the filter, in Hz. Value can range from 1 to 20500.

### resonance {#resonance}

```gdscript
resonance: float
```

- void set_resonance(value: float)
- float get_resonance()

Gain at or directly next to the cutoff_hz frequency threshold. Value can range from 0 to 1.

Its exact behavior depends on the selected filter type:

- For shelf filters, it accentuates or masks the order by increasing frequencies right next to the cutoff_hz frequency and decreasing frequencies on the opposite side.
- For the band-pass and notch filters, it widens or narrows the filter at the cutoff_hz frequency threshold.
- For low/high-pass filters, it increases or decreases frequencies at the cutoff_hz frequency threshold.

### gain {#gain}

```gdscript
gain: float
```

- void set_gain(value: float)
- float get_gain()

Gain of the frequencies affected by the filter. This property is only available for AudioEffectLowShelfFilter and AudioEffectHighShelfFilter. Value can range from 0 to 4.

### db {#db}

```gdscript
db: int
```

- void set_db(value: FilterDB)
- FilterDB get_db()

Steepness of the cutoff curve in dB per octave (twice the frequency above cutoff_hz, or half the frequency below cutoff_hz), also known as the "order" of the filter. Higher orders have a more aggressive cutoff.

## 枚举

### FilterDB {#enum-FilterDB}

```gdscript
enum FilterDB { FILTER_6DB = 0, FILTER_12DB = 1, FILTER_18DB = 2, FILTER_24DB = 3 }
```

FilterDB FILTER_6DB = 0

Cutting off at 6 dB per octave. One octave is twice the frequency above cutoff_hz, or half the frequency below cutoff_hz.

FilterDB FILTER_12DB = 1

Cutting off at 12 dB per octave. One octave is twice the frequency above cutoff_hz, or half the frequency below cutoff_hz.

FilterDB FILTER_18DB = 2

Cutting off at 18 dB per octave. One octave is twice the frequency above cutoff_hz, or half the frequency below cutoff_hz.

FilterDB FILTER_24DB = 3

Cutting off at 24 dB per octave. One octave is twice the frequency above cutoff_hz, or half the frequency below cutoff_hz.

#### FILTER_6DB {#FILTER_6DB}

```gdscript
FILTER_6DB = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILTER_12DB {#FILTER_12DB}

```gdscript
FILTER_12DB = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILTER_18DB {#FILTER_18DB}

```gdscript
FILTER_18DB = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FILTER_24DB {#FILTER_24DB}

```gdscript
FILTER_24DB = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
