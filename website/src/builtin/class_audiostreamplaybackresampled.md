# AudioStreamPlaybackResampled

**继承**：[AudioStreamPlayback](class_audiostreamplayback.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamPlaybackResampled**

> Playback class used to mix an AudioStream's audio samples to AudioServer.get_mix_rate() using cubic interpolation.

<details><summary>官方英文描述</summary>

Playback class used to mix an AudioStream's audio samples to AudioServer.get_mix_rate() using cubic interpolation.

</details>

## 方法

### _get_stream_sampling_rate {#_get_stream_sampling_rate}

```gdscript
float _get_stream_sampling_rate() virtual const
```

Returns an AudioStream's sample rate, in Hz. Used to perform resampling.

### _mix_resampled {#_mix_resampled}

```gdscript
int _mix_resampled(dst_buffer: AudioFrame*, frame_count: int) virtual
```

Called by begin_resample() to mix an AudioStream to AudioServer.get_mix_rate(). Uses _get_stream_sampling_rate() as the source sample rate. Returns the number of mixed frames.

### begin_resample {#begin_resample}

```gdscript
void begin_resample()
```

Called when an AudioStream is played. Clears the cubic interpolation history and starts mixing by calling _mix_resampled().
