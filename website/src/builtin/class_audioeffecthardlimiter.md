# AudioEffectHardLimiter

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectHardLimiter**

> A "limiter" disallows audio signals from exceeding a given volume threshold level in dB. Hard limiters predict volume peaks, and will smoothly apply gain reduction when a peak crosses the ceiling threshold level to prevent clipping. It preserves the waveform and prevents it from crossing the ceiling threshold level. Adding one in the Master bus is recommended as a safety measure to prevent sudden volume peaks from occurring, and to prevent distortion caused by clipping, when the volume exceeds 0 dB.

<details><summary>官方英文描述</summary>

A "limiter" disallows audio signals from exceeding a given volume threshold level in dB. Hard limiters predict volume peaks, and will smoothly apply gain reduction when a peak crosses the ceiling threshold level to prevent clipping. It preserves the waveform and prevents it from crossing the ceiling threshold level. Adding one in the Master bus is recommended as a safety measure to prevent sudden volume peaks from occurring, and to prevent distortion caused by clipping, when the volume exceeds 0 dB.

If clipping is desired, consider AudioEffectDistortion.MODE_CLIP.

</details>

## 方法

### get_ceiling_db {#get_ceiling_db}

```gdscript
float get_ceiling_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pre_gain_db {#get_pre_gain_db}

```gdscript
float get_pre_gain_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_release {#get_release}

```gdscript
float get_release() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ceiling_db {#set_ceiling_db}

```gdscript
void set_ceiling_db(ceiling: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pre_gain_db {#set_pre_gain_db}

```gdscript
void set_pre_gain_db(pre_gain: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_release {#set_release}

```gdscript
void set_release(release: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### pre_gain_db {#pre_gain_db}

```gdscript
pre_gain_db: float
```

- void set_pre_gain_db(value: float)
- float get_pre_gain_db()

Gain before limiting, in dB. Value can range from -24 to 24.

### ceiling_db {#ceiling_db}

```gdscript
ceiling_db: float
```

- void set_ceiling_db(value: float)
- float get_ceiling_db()

The waveform's maximum allowed value, in dB. This value can range from -24 to 0.

The default value of -0.3 prevents potential inter-sample peaks (ISP) from crossing over 0 dB, which can cause slight distortion on some older hardware.

### release {#release}

```gdscript
release: float
```

- void set_release(value: float)
- float get_release()

Time it takes in seconds for the gain reduction to fully release. Value can range from 0.01 to 3.
