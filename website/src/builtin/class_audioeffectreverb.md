# AudioEffectReverb

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectReverb**

> A "reverb" effect plays the input audio back continuously, decaying over a period of time. It simulates sounds in different kinds of spaces, ranging from small rooms, to big caverns.

<details><summary>官方英文描述（中文精译进行中）</summary>

A "reverb" effect plays the input audio back continuously, decaying over a period of time. It simulates sounds in different kinds of spaces, ranging from small rooms, to big caverns.

See also AudioEffectDelay for a non-blurry type of echo.

</details>

## 方法

### get_damping {#get_damping}

```gdscript
float get_damping() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_dry {#get_dry}

```gdscript
float get_dry() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_hpf {#get_hpf}

```gdscript
float get_hpf() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_predelay_feedback {#get_predelay_feedback}

```gdscript
float get_predelay_feedback() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_predelay_msec {#get_predelay_msec}

```gdscript
float get_predelay_msec() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_room_size {#get_room_size}

```gdscript
float get_room_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_spread {#get_spread}

```gdscript
float get_spread() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_wet {#get_wet}

```gdscript
float get_wet() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_damping {#set_damping}

```gdscript
void set_damping(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_dry {#set_dry}

```gdscript
void set_dry(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_hpf {#set_hpf}

```gdscript
void set_hpf(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_predelay_feedback {#set_predelay_feedback}

```gdscript
void set_predelay_feedback(feedback: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_predelay_msec {#set_predelay_msec}

```gdscript
void set_predelay_msec(msec: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_room_size {#set_room_size}

```gdscript
void set_room_size(size: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_spread {#set_spread}

```gdscript
void set_spread(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_wet {#set_wet}

```gdscript
void set_wet(amount: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### predelay_msec {#predelay_msec}

```gdscript
predelay_msec: float
```

- void set_predelay_msec(value: float)
- float get_predelay_msec()

Time between the original audio and the early reflections of the reverb signal, in milliseconds. Value can range from 20 to 500.

### predelay_feedback {#predelay_feedback}

```gdscript
predelay_feedback: float
```

- void set_predelay_feedback(value: float)
- float get_predelay_feedback()

Gain of early reflection copies. At higher values, early reflection copies are louder and ring out for longer. Value can range from 0 to 1.

### room_size {#room_size}

```gdscript
room_size: float
```

- void set_room_size(value: float)
- float get_room_size()

Dimensions of simulated room. Bigger means more echoes. Value can range from 0 to 1.

### damping {#damping}

```gdscript
damping: float
```

- void set_damping(value: float)
- float get_damping()

Defines how reflective the imaginary room's walls are. The more reflective, the more high frequency content the reverb has. Value can range from 0 to 1.

### spread {#spread}

```gdscript
spread: float
```

- void set_spread(value: float)
- float get_spread()

Widens or narrows the stereo image of the reverb tail. At 1, it fully widens. Value can range from 0 to 1.

### hipass {#hipass}

```gdscript
hipass: float
```

- void set_hpf(value: float)
- float get_hpf()

High-pass filter allows frequencies higher than a certain cutoff threshold and attenuates frequencies lower than the cutoff threshold. Value can range from 0 to 1.

### dry {#dry}

```gdscript
dry: float
```

- void set_dry(value: float)
- float get_dry()

The volume ratio of the original audio. At 0, only the modified audio is outputted. Value can range from 0 to 1.

### wet {#wet}

```gdscript
wet: float
```

- void set_wet(value: float)
- float get_wet()

The volume ratio of the modified audio. At 0, only the original audio is outputted. Value can range from 0 to 1.
