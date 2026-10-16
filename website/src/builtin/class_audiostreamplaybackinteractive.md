# AudioStreamPlaybackInteractive

**继承**：[AudioStreamPlayback](class_audiostreamplayback.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioStreamPlaybackInteractive**

> Playback component of AudioStreamInteractive. Contains functions to change the currently played clip.

<details><summary>官方英文描述（中文精译进行中）</summary>

Playback component of AudioStreamInteractive. Contains functions to change the currently played clip.

</details>

## 方法

### get_current_clip_index {#get_current_clip_index}

```gdscript
int get_current_clip_index() const
```

Return the index of the currently playing clip. You can use this to get the name of the currently playing clip with AudioStreamInteractive.get_clip_name().

Example: Get the currently playing clip name from inside an AudioStreamPlayer node.

```
var playing_clip_name = stream.get_clip_name(get_stream_playback().get_current_clip_index())
```

### switch_to_clip {#switch_to_clip}

```gdscript
void switch_to_clip(clip_index: int)
```

Switch to a clip (by index).

### switch_to_clip_by_name {#switch_to_clip_by_name}

```gdscript
void switch_to_clip_by_name(clip_name: StringName)
```

Switch to a clip (by name).
