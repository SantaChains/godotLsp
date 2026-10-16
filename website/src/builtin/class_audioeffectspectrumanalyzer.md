# AudioEffectSpectrumAnalyzer

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectSpectrumAnalyzer**

> Calculates a Fourier Transform of the audio signal. This effect does not alter the audio. Can be used for creating real-time audio visualizations, like a spectrogram.

<details><summary>官方英文描述（中文精译进行中）</summary>

Calculates a Fourier Transform of the audio signal. This effect does not alter the audio. Can be used for creating real-time audio visualizations, like a spectrogram.

This resource configures an AudioEffectSpectrumAnalyzerInstance, which performs the actual analysis at runtime. An instance should be obtained with AudioServer.get_bus_effect_instance() to make use of this effect.

</details>

## 方法

### get_buffer_length {#get_buffer_length}

```gdscript
float get_buffer_length() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fft_size {#get_fft_size}

```gdscript
AudioEffectSpectrumAnalyzer.FFTSize get_fft_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_buffer_length {#set_buffer_length}

```gdscript
void set_buffer_length(seconds: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fft_size {#set_fft_size}

```gdscript
void set_fft_size(size: AudioEffectSpectrumAnalyzer.FFTSize)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### buffer_length {#buffer_length}

```gdscript
buffer_length: float
```

- void set_buffer_length(value: float)
- float get_buffer_length()

The length of the buffer to keep, in seconds. Higher values keep data around for longer, but require more memory. Value can range from 0.1 to 4.

### fft_size {#fft_size}

```gdscript
fft_size: int
```

- void set_fft_size(value: FFTSize)
- FFTSize get_fft_size()

The size of the Fast Fourier transform [https://en.wikipedia.org/wiki/Fast_Fourier_transform] buffer. Higher values smooth out the spectrum analysis over time, but have greater latency. The effects of this higher latency are especially noticeable with sudden amplitude changes.

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
