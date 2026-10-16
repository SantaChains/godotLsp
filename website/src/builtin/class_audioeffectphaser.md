# AudioEffectPhaser

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectPhaser**

> A "phaser" effect creates a copy of the original audio that phase-rotates differently across the entire frequency spectrum, with the use of a series of all-pass filter stages (6 in this effect). This copy modulates with a low-frequency oscillator and combines with the original audio, resulting in peaks and troughs that sweep across the spectrum.

<details><summary>官方英文描述</summary>

A "phaser" effect creates a copy of the original audio that phase-rotates differently across the entire frequency spectrum, with the use of a series of all-pass filter stages (6 in this effect). This copy modulates with a low-frequency oscillator and combines with the original audio, resulting in peaks and troughs that sweep across the spectrum.

This effect can be used to create a "glassy" or "bubbly" sound.

</details>

## 方法

### get_depth {#get_depth}

```gdscript
float get_depth() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_feedback {#get_feedback}

```gdscript
float get_feedback() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_range_max_hz {#get_range_max_hz}

```gdscript
float get_range_max_hz() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_range_min_hz {#get_range_min_hz}

```gdscript
float get_range_min_hz() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_rate_hz {#get_rate_hz}

```gdscript
float get_rate_hz() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_depth {#set_depth}

```gdscript
void set_depth(depth: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_feedback {#set_feedback}

```gdscript
void set_feedback(fbk: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_range_max_hz {#set_range_max_hz}

```gdscript
void set_range_max_hz(hz: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_range_min_hz {#set_range_min_hz}

```gdscript
void set_range_min_hz(hz: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rate_hz {#set_rate_hz}

```gdscript
void set_rate_hz(hz: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### range_min_hz {#range_min_hz}

```gdscript
range_min_hz: float
```

- void set_range_min_hz(value: float)
- float get_range_min_hz()

Determines the minimum frequency affected by the low-frequency oscillator modulations, in Hz. Value can range from 10 to 10000.

### range_max_hz {#range_max_hz}

```gdscript
range_max_hz: float
```

- void set_range_max_hz(value: float)
- float get_range_max_hz()

Determines the maximum frequency affected by the low-frequency oscillator modulations, in Hz. Value can range from 10 to 10000.

### rate_hz {#rate_hz}

```gdscript
rate_hz: float
```

- void set_rate_hz(value: float)
- float get_rate_hz()

Adjusts the rate in Hz at which the effect sweeps up and down across the frequency range. Value can range from 0.01 to 20.

### feedback {#feedback}

```gdscript
feedback: float
```

- void set_feedback(value: float)
- float get_feedback()

The volume ratio of the filtered audio that is fed back to the all-pass filters. The higher the value, the sharper and louder the peak filters created by the effect. Value can range from 0.1 to 0.9.

### depth {#depth}

```gdscript
depth: float
```

- void set_depth(value: float)
- float get_depth()

Intensity of the effect. Value can range from 0.1 to 4.0.
