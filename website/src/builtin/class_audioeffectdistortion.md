# AudioEffectDistortion

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectDistortion**

> A "distortion" effect modifies the waveform via a nonlinear mathematical function (see available ones in Mode), based on the amplitude of the waveform's samples.

<details><summary>官方英文描述（中文精译进行中）</summary>

A "distortion" effect modifies the waveform via a nonlinear mathematical function (see available ones in Mode), based on the amplitude of the waveform's samples.

Note: In a nonlinear function, an input sample at x amplitude value, will either have its amplitude increased or decreased to a y value, based on the function value at x, which is why even at the same drive, the output sound will vary depending on the input's volume. To change the volume while maintaining the output waveform, use post_gain.

In this effect, each type is a different nonlinear function. The different types available are: clip, atan, lofi (bitcrush), overdrive, and waveshape. Every distortion type available here is symmetric: negative amplitude values are affected the same way as positive ones.

Although distortion will always change frequency content, usually by introducing high harmonics, different distortion types offer a range of sound qualities; from "soft" and "warm", to "crunchy" and "abrasive".

For games, it can help simulate sound coming from some saturated device or speaker very efficiently. It can also help the audio stand out in a mix, by introducing higher frequencies and increasing the volume.

Note: Although usually imperceptible, an enabled distortion effect still changes the sound even when drive is set to 0. This is not a bug. If this behavior is undesirable, consider disabling the effect using AudioServer.set_bus_effect_enabled().

</details>

## 方法

### get_drive {#get_drive}

```gdscript
float get_drive() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_keep_hf_hz {#get_keep_hf_hz}

```gdscript
float get_keep_hf_hz() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mode {#get_mode}

```gdscript
AudioEffectDistortion.Mode get_mode() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_post_gain {#get_post_gain}

```gdscript
float get_post_gain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pre_gain {#get_pre_gain}

```gdscript
float get_pre_gain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_drive {#set_drive}

```gdscript
void set_drive(drive: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_keep_hf_hz {#set_keep_hf_hz}

```gdscript
void set_keep_hf_hz(keep_hf_hz: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mode {#set_mode}

```gdscript
void set_mode(mode: AudioEffectDistortion.Mode)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_post_gain {#set_post_gain}

```gdscript
void set_post_gain(post_gain: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pre_gain {#set_pre_gain}

```gdscript
void set_pre_gain(pre_gain: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### mode {#mode}

```gdscript
mode: int
```

- void set_mode(value: Mode)
- Mode get_mode()

Distortion type. Changes the nonlinear function used to distort the waveform. See Mode.

### pre_gain {#pre_gain}

```gdscript
pre_gain: float
```

- void set_pre_gain(value: float)
- float get_pre_gain()

Gain before the effect, in dB. Value can range from -60 to 60.

### keep_hf_hz {#keep_hf_hz}

```gdscript
keep_hf_hz: float
```

- void set_keep_hf_hz(value: float)
- float get_keep_hf_hz()

High-pass filter, in Hz. Frequencies higher than this value will not be affected by the distortion. Value can range from 1 to 20000.

### drive {#drive}

```gdscript
drive: float
```

- void set_drive(value: float)
- float get_drive()

Distortion intensity. Controls how much of the input audio is affected by the distortion curve by moving from a linear function to a nonlinear one. Value can range from 0 to 1.

### post_gain {#post_gain}

```gdscript
post_gain: float
```

- void set_post_gain(value: float)
- float get_post_gain()

Gain after the effect, in dB. Value can range from -80 to 24.

## 枚举

### Mode {#enum-Mode}

```gdscript
enum Mode { MODE_CLIP = 0, MODE_ATAN = 1, MODE_LOFI = 2, MODE_OVERDRIVE = 3, MODE_WAVESHAPE = 4 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MODE_CLIP {#MODE_CLIP}

```gdscript
MODE_CLIP = 0
```

Flattens the waveform at 0 dB in a sharp manner. drive increases amplitude of samples exponentially. This mode functions as a hard clipper if drive is set to 0, and is the only mode that clips audio signals at 0 dB.

#### MODE_ATAN {#MODE_ATAN}

```gdscript
MODE_ATAN = 1
```

Flattens the waveform in a smooth manner, following an arctangent curve. The audio decreases in volume, before flattening peaks to PI * 4.0 (linear value), if it was normalized beforehand.

#### MODE_LOFI {#MODE_LOFI}

```gdscript
MODE_LOFI = 2
```

Decreases audio bit depth to achieve a low-resolution audio signal, going from 16-bit to 2-bit. Can be used to emulate the sound of early digital audio devices.

#### MODE_OVERDRIVE {#MODE_OVERDRIVE}

```gdscript
MODE_OVERDRIVE = 3
```

Emulates the warm distortion produced by a field effect transistor, which is commonly used in solid-state musical instrument amplifiers. drive has no effect in this mode.

#### MODE_WAVESHAPE {#MODE_WAVESHAPE}

```gdscript
MODE_WAVESHAPE = 4
```

Flattens the waveform in a smooth manner, until it reaches a sharp peak at drive = 1, following a generic absolute sigmoid function.

---
