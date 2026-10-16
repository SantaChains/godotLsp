# AudioEffectCapture

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectCapture**

> Copies all audio frames, also known as "samples" or "audio samples", from the attached audio bus into its internal ring buffer. This effect does not alter the audio. Can be used for storing real-time audio data for playback, and for creating real-time audio visualizations, like an oscilloscope.

<details><summary>官方英文描述</summary>

Copies all audio frames, also known as "samples" or "audio samples", from the attached audio bus into its internal ring buffer. This effect does not alter the audio. Can be used for storing real-time audio data for playback, and for creating real-time audio visualizations, like an oscilloscope.

Application code should consume these audio frames from this ring buffer using get_buffer() and process it as needed, for example to capture data from an AudioStreamMicrophone, implement application-defined effects, or to transmit audio over the network. When capturing audio data from a microphone, the format of the samples will be stereo 32-bit floating-point PCM.

Unlike AudioEffectRecord, this effect only returns the raw audio samples instead of encoding them into an AudioStream.

</details>

## 方法

### can_get_buffer {#can_get_buffer}

```gdscript
bool can_get_buffer(frames: int) const
```

Returns true if at least frames samples are available to read in the internal ring buffer.

### clear_buffer {#clear_buffer}

```gdscript
void clear_buffer()
```

Clears the internal ring buffer.

Note: Calling this during a capture can cause the loss of samples which causes popping in the playback.

### get_buffer {#get_buffer}

```gdscript
PackedVector2Array get_buffer(frames: int)
```

Gets the next frames samples from the internal ring buffer.

Returns a PackedVector2Array containing exactly frames samples if available, or an empty PackedVector2Array if insufficient data was available.

The samples are signed floating-point PCM between -1 and 1. You will have to scale them if you want to use them as 8 or 16-bit integer samples. (v = 0x7fff * samples[0].x)

### get_buffer_length {#get_buffer_length}

```gdscript
float get_buffer_length()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_buffer_length_frames {#get_buffer_length_frames}

```gdscript
int get_buffer_length_frames() const
```

Returns the total size of the internal ring buffer in number of samples.

### get_discarded_frames {#get_discarded_frames}

```gdscript
int get_discarded_frames() const
```

Returns the number of samples discarded from the audio bus due to full buffer.

### get_frames_available {#get_frames_available}

```gdscript
int get_frames_available() const
```

Returns the number of samples available to read using get_buffer().

### get_pushed_frames {#get_pushed_frames}

```gdscript
int get_pushed_frames() const
```

Returns the number of samples inserted from the audio bus.

### set_buffer_length {#set_buffer_length}

```gdscript
void set_buffer_length(buffer_length_seconds: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### buffer_length {#buffer_length}

```gdscript
buffer_length: float
```

- void set_buffer_length(value: float)
- float get_buffer_length()

Length of the internal ring buffer, in seconds. Higher values keep data around for longer, but require more memory. Value can range from 0.01 to 10.

Note: Setting the buffer length will have no effect if already initialized.
