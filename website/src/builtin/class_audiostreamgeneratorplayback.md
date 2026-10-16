# AudioStreamGeneratorPlayback

**继承**：[AudioStreamPlaybackResampled](class_audiostreamplaybackresampled.md) → [AudioStreamPlayback](class_audiostreamplayback.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamGeneratorPlayback**

> This class is meant to be used with AudioStreamGenerator to play back the generated audio in real-time.

<details><summary>官方英文描述</summary>

This class is meant to be used with AudioStreamGenerator to play back the generated audio in real-time.

</details>

## 方法

### can_push_buffer {#can_push_buffer}

```gdscript
bool can_push_buffer(amount: int) const
```

Returns true if a buffer of the size amount can be pushed to the audio sample data buffer without overflowing it, false otherwise.

### clear_buffer {#clear_buffer}

```gdscript
void clear_buffer()
```

Clears the audio sample data buffer.

### get_frames_available {#get_frames_available}

```gdscript
int get_frames_available() const
```

Returns the number of frames that can be pushed to the audio sample data buffer without overflowing it. If the result is 0, the buffer is full.

### get_skips {#get_skips}

```gdscript
int get_skips() const
```

Returns the number of times the playback skipped due to a buffer underrun in the audio sample data. This value is reset at the start of the playback.

### push_buffer {#push_buffer}

```gdscript
bool push_buffer(frames: PackedVector2Array)
```

Pushes several audio data frames to the buffer. This is usually more efficient than push_frame() in C# and compiled languages via GDExtension, but push_buffer() may be less efficient in GDScript.

### push_frame {#push_frame}

```gdscript
bool push_frame(frame: Vector2)
```

Pushes a single audio data frame to the buffer. This is usually less efficient than push_buffer() in C# and compiled languages via GDExtension, but push_frame() may be more efficient in GDScript.
