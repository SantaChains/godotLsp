# AudioEffectLimiter

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectLimiter**

> A "limiter" is an audio effect designed to stop audio signals from exceeding a specified volume threshold level, and usually works by decreasing the volume or soft-clipping the audio. Adding one in the Master bus is always recommended to prevent clipping when the volume goes above 0 dB.

<details><summary>官方英文描述（中文精译进行中）</summary>

A "limiter" is an audio effect designed to stop audio signals from exceeding a specified volume threshold level, and usually works by decreasing the volume or soft-clipping the audio. Adding one in the Master bus is always recommended to prevent clipping when the volume goes above 0 dB.

Soft clipping starts to decrease the peaks a little below the volume threshold level and progressively increases its effect as the input volume increases such that the threshold level is never exceeded.

If hard clipping is desired, consider AudioEffectDistortion.MODE_CLIP.

</details>

## 方法

### get_ceiling_db {#get_ceiling_db}

```gdscript
float get_ceiling_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_soft_clip_db {#get_soft_clip_db}

```gdscript
float get_soft_clip_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_soft_clip_ratio {#get_soft_clip_ratio}

```gdscript
float get_soft_clip_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_threshold_db {#get_threshold_db}

```gdscript
float get_threshold_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ceiling_db {#set_ceiling_db}

```gdscript
void set_ceiling_db(ceiling: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_soft_clip_db {#set_soft_clip_db}

```gdscript
void set_soft_clip_db(soft_clip: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_soft_clip_ratio {#set_soft_clip_ratio}

```gdscript
void set_soft_clip_ratio(soft_clip: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_threshold_db {#set_threshold_db}

```gdscript
void set_threshold_db(threshold: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### ceiling_db {#ceiling_db}

```gdscript
ceiling_db: float
```

- void set_ceiling_db(value: float)
- float get_ceiling_db()

The waveform's maximum allowed value, in dB. Value can range from -20 to -0.1.

### threshold_db {#threshold_db}

```gdscript
threshold_db: float
```

- void set_threshold_db(value: float)
- float get_threshold_db()

The volume threshold level from which the limiter begins to be active, in dB. Value can range from -30 to 0.

### soft_clip_db {#soft_clip_db}

```gdscript
soft_clip_db: float
```

- void set_soft_clip_db(value: float)
- float get_soft_clip_db()

Modifies the volume of the limited waves, in dB. Value can range from 0 to 6.

### soft_clip_ratio {#soft_clip_ratio}

```gdscript
soft_clip_ratio: float
```

- void set_soft_clip_ratio(value: float)
- float get_soft_clip_ratio()

This property has no effect on the audio. Use AudioEffectHardLimiter instead, as this Limiter effect is deprecated.
