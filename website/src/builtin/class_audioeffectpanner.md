# AudioEffectPanner

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectPanner**

> Determines how much of the audio signal is sent to the left and right channels. This helps with audio spatialization, giving sounds distinct places in a mix.

<details><summary>官方英文描述</summary>

Determines how much of the audio signal is sent to the left and right channels. This helps with audio spatialization, giving sounds distinct places in a mix.

AudioStreamPlayer2D and AudioStreamPlayer3D handle panning automatically, following where the source of the sound is on the screen.

</details>

## 方法

### get_pan {#get_pan}

```gdscript
float get_pan() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pan {#set_pan}

```gdscript
void set_pan(cpanume: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### pan {#pan}

```gdscript
pan: float
```

- void set_pan(value: float)
- float get_pan()

Pan position. Negative values pan the sound to the left, positive pan to the right. Value can range from -1 to 1.
