# AudioEffectRecord

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectRecord**

> Allows the user to record the sound from an audio bus into an AudioStreamWAV. When used on the Master audio bus, this includes all audio output by Godot.

<details><summary>官方英文描述</summary>

Allows the user to record the sound from an audio bus into an AudioStreamWAV. When used on the Master audio bus, this includes all audio output by Godot.

Unlike AudioEffectCapture, this effect encodes the recording with the given format (8-bit, 16-bit, or compressed) instead of giving access to the raw audio samples.

Can be used (with an AudioStreamMicrophone) to record from a microphone.

Note: ProjectSettings.audio/driver/enable_input must be true for audio input to work. See also that setting's description for caveats related to permissions and operating system privacy settings.

</details>

## 方法

### get_format {#get_format}

```gdscript
AudioStreamWAV.Format get_format() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_recording {#get_recording}

```gdscript
AudioStreamWAV get_recording() const
```

Returns the recorded sample.

### is_recording_active {#is_recording_active}

```gdscript
bool is_recording_active() const
```

Returns whether the recording is active or not.

### set_format {#set_format}

```gdscript
void set_format(format: AudioStreamWAV.Format)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_recording_active {#set_recording_active}

```gdscript
void set_recording_active(record: bool)
```

If true, the sound will be recorded. Note that restarting the recording will remove the previously recorded sample.

## 属性

### format {#format}

```gdscript
format: int
```

- void set_format(value: Format)
- Format get_format()

Specifies the format in which the sample will be recorded.
