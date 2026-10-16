# AudioEffectCompressor

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectCompressor**

> A "compressor" decreases the volume of sounds when it exceeds a certain volume threshold level.

<details><summary>官方英文描述</summary>

A "compressor" decreases the volume of sounds when it exceeds a certain volume threshold level.

A compressor can have many uses in a mix:

- To compress the whole volume in the Master bus (although an AudioEffectHardLimiter is probably better).
- To ensure balance of voice audio clips.
- To sidechain, using another bus as a trigger. This decreases the volume of the bus it is attached to, by using the volume from another audio bus for threshold detection. This technique is common in video game mixing to decrease the volume of music and SFX while voices are being heard. This effect is also known as "ducking".
- To accentuate transients by using a long attack, letting sounds exceed the volume threshold level for a short period before compressing them. This can be used to make SFX more punchy.

</details>

## 方法

### get_attack_us {#get_attack_us}

```gdscript
float get_attack_us() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gain {#get_gain}

```gdscript
float get_gain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_mix {#get_mix}

```gdscript
float get_mix() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_ratio {#get_ratio}

```gdscript
float get_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_release_ms {#get_release_ms}

```gdscript
float get_release_ms() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_sidechain {#get_sidechain}

```gdscript
StringName get_sidechain() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_threshold {#get_threshold}

```gdscript
float get_threshold() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_attack_us {#set_attack_us}

```gdscript
void set_attack_us(attack_us: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_gain {#set_gain}

```gdscript
void set_gain(gain: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_mix {#set_mix}

```gdscript
void set_mix(mix: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_ratio {#set_ratio}

```gdscript
void set_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_release_ms {#set_release_ms}

```gdscript
void set_release_ms(release_ms: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sidechain {#set_sidechain}

```gdscript
void set_sidechain(sidechain: StringName)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_threshold {#set_threshold}

```gdscript
void set_threshold(threshold: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### threshold {#threshold}

```gdscript
threshold: float
```

- void set_threshold(value: float)
- float get_threshold()

The volume level above which compression is applied to the audio, in dB. Value can range from -60 to 0.

### ratio {#ratio}

```gdscript
ratio: float
```

- void set_ratio(value: float)
- float get_ratio()

Amount of compression applied to the audio once it passes the volume threshold level. The higher the ratio, the stronger the compression applied to audio signals that pass the volume threshold level. Value can range from 1 to 48.

### gain {#gain}

```gdscript
gain: float
```

- void set_gain(value: float)
- float get_gain()

Gain of the audio signal, in dB. Value can range from -20 to 20.

### attack_us {#attack_us}

```gdscript
attack_us: float
```

- void set_attack_us(value: float)
- float get_attack_us()

Compressor's reaction time when the audio exceeds the volume threshold level, in microseconds. Value can range from 20 to 2000.

### release_ms {#release_ms}

```gdscript
release_ms: float
```

- void set_release_ms(value: float)
- float get_release_ms()

Compressor's delay time to stop decreasing the volume after the it falls below the volume threshold level, in milliseconds. Value can range from 20 to 2000.

### mix {#mix}

```gdscript
mix: float
```

- void set_mix(value: float)
- float get_mix()

Balance between the original audio and the compressed audio. Value can range from 0 (totally dry) to 1 (totally wet).

### sidechain {#sidechain}

```gdscript
sidechain: StringName
```

- void set_sidechain(value: StringName)
- StringName get_sidechain()

Audio bus to use for the volume threshold detection.
