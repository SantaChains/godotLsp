# AudioEffectSpectrumAnalyzerInstance

**继承**：[AudioEffectInstance](class_audioeffectinstance.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectSpectrumAnalyzerInstance**

> The runtime part of an AudioEffectSpectrumAnalyzer, which can be used to query the magnitude of a frequency range on its host bus.

<details><summary>官方英文描述（中文精译进行中）</summary>

The runtime part of an AudioEffectSpectrumAnalyzer, which can be used to query the magnitude of a frequency range on its host bus.

An instance of this class can be obtained with AudioServer.get_bus_effect_instance().

</details>

## 方法

### get_magnitude_for_frequency_range {#get_magnitude_for_frequency_range}

```gdscript
Vector2 get_magnitude_for_frequency_range(from_hz: float, to_hz: float, mode: AudioEffectSpectrumAnalyzerInstance.MagnitudeMode = 1) const
```

Returns the magnitude of the frequencies from from_hz to to_hz in linear energy as a Vector2. The x component of the return value represents the left stereo channel, and y represents the right channel.

mode determines how the frequency range will be processed.

## 枚举

### MagnitudeMode {#enum-MagnitudeMode}

```gdscript
enum MagnitudeMode { MAGNITUDE_AVERAGE = 0, MAGNITUDE_MAX = 1 }
```

MagnitudeMode MAGNITUDE_AVERAGE = 0

Use the average value across the frequency range as magnitude.

MagnitudeMode MAGNITUDE_MAX = 1

Use the maximum value of the frequency range as magnitude.

#### MAGNITUDE_AVERAGE {#MAGNITUDE_AVERAGE}

```gdscript
MAGNITUDE_AVERAGE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MAGNITUDE_MAX {#MAGNITUDE_MAX}

```gdscript
MAGNITUDE_MAX = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
