# AudioEffectStereoEnhance

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectStereoEnhance**

> Adjusts gain of the left and right channels, and makes mono sounds stereo through phase shifting.

<details><summary>官方英文描述</summary>

Adjusts gain of the left and right channels, and makes mono sounds stereo through phase shifting.

</details>

## 方法

### get_pan_pullout {#get_pan_pullout}

```gdscript
float get_pan_pullout() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_surround {#get_surround}

```gdscript
float get_surround() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_time_pullout {#get_time_pullout}

```gdscript
float get_time_pullout() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pan_pullout {#set_pan_pullout}

```gdscript
void set_pan_pullout(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_surround {#set_surround}

```gdscript
void set_surround(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_time_pullout {#set_time_pullout}

```gdscript
void set_time_pullout(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### pan_pullout {#pan_pullout}

```gdscript
pan_pullout: float
```

- void set_pan_pullout(value: float)
- float get_pan_pullout()

Gain of the side channels, if they exist. A value of 0 will downmix stereo to mono. Value can range from 0 to 4.

### time_pullout_ms {#time_pullout_ms}

```gdscript
time_pullout_ms: float
```

- void set_time_pullout(value: float)
- float get_time_pullout()

Widens the stereo image through phase shifting in conjunction with surround. Just delays the right channel if surround is 0. Value is in milliseconds, and can range from 0 to 50.

### surround {#surround}

```gdscript
surround: float
```

- void set_surround(value: float)
- float get_surround()

Widens the stereo image through phase shifting in conjunction with time_pullout_ms. Just pans sound to the left channel if time_pullout_ms is 0. Value can range from 0 to 1.
