# AudioEffectPitchShift

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectPitchShift**

> Allows modulation of pitch without modifying speed. All frequencies can be raised or lowered with minimal effect on transients.

<details><summary>官方英文描述（中文精译进行中）</summary>

Allows modulation of pitch without modifying speed. All frequencies can be raised or lowered with minimal effect on transients.

</details>

## 方法

### get_fft_size {#get_fft_size}

```gdscript
AudioEffectPitchShift.FFTSize get_fft_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_oversampling {#get_oversampling}

```gdscript
int get_oversampling() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pitch_scale {#get_pitch_scale}

```gdscript
float get_pitch_scale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fft_size {#set_fft_size}

```gdscript
void set_fft_size(size: AudioEffectPitchShift.FFTSize)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_oversampling {#set_oversampling}

```gdscript
void set_oversampling(amount: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pitch_scale {#set_pitch_scale}

```gdscript
void set_pitch_scale(rate: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### pitch_scale {#pitch_scale}

```gdscript
pitch_scale: float
```

- void set_pitch_scale(value: float)
- float get_pitch_scale()

The pitch scale to use. 1.0 is the default pitch and plays sounds unaffected. pitch_scale can range from 0 (infinitely low pitch, inaudible) to 16 (16 times higher than the initial pitch).

### oversampling {#oversampling}

```gdscript
oversampling: float
```

- void set_oversampling(value: int)
- int get_oversampling()

The oversampling factor to use. Higher values result in better quality, but are more demanding on the CPU and may cause audio cracking if the CPU can't keep up.

### fft_size {#fft_size}

```gdscript
fft_size: int
```

- void set_fft_size(value: FFTSize)
- FFTSize get_fft_size()

The size of the Fast Fourier transform [https://en.wikipedia.org/wiki/Fast_Fourier_transform] buffer. Higher values smooth out the effect over time, but have greater latency. The effects of this higher latency are especially noticeable on audio signals that have sudden amplitude changes.

## 枚举

### FFTSize {#enum-FFTSize}

```gdscript
enum FFTSize { FFT_SIZE_256 = 0, FFT_SIZE_512 = 1, FFT_SIZE_1024 = 2, FFT_SIZE_2048 = 3, FFT_SIZE_4096 = 4, FFT_SIZE_MAX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FFT_SIZE_256 {#FFT_SIZE_256}

```gdscript
FFT_SIZE_256 = 0
```

Use a buffer of 256 samples for the Fast Fourier transform. Lowest latency, but least stable over time.

#### FFT_SIZE_512 {#FFT_SIZE_512}

```gdscript
FFT_SIZE_512 = 1
```

Use a buffer of 512 samples for the Fast Fourier transform. Low latency, but less stable over time.

#### FFT_SIZE_1024 {#FFT_SIZE_1024}

```gdscript
FFT_SIZE_1024 = 2
```

Use a buffer of 1024 samples for the Fast Fourier transform. This is a compromise between latency and stability over time.

#### FFT_SIZE_2048 {#FFT_SIZE_2048}

```gdscript
FFT_SIZE_2048 = 3
```

Use a buffer of 2048 samples for the Fast Fourier transform. High latency, but stable over time.

#### FFT_SIZE_4096 {#FFT_SIZE_4096}

```gdscript
FFT_SIZE_4096 = 4
```

Use a buffer of 4096 samples for the Fast Fourier transform. Highest latency, but most stable over time.

#### FFT_SIZE_MAX {#FFT_SIZE_MAX}

```gdscript
FFT_SIZE_MAX = 5
```

Represents the size of the FFTSize enum.

---
