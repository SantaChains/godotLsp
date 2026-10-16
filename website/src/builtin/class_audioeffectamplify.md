# AudioEffectAmplify

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectAmplify**

> Increases or decreases the volume being routed through the audio bus.

<details><summary>官方英文描述（中文精译进行中）</summary>

Increases or decreases the volume being routed through the audio bus.

</details>

## 方法

### get_volume_db {#get_volume_db}

```gdscript
float get_volume_db() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_volume_linear {#get_volume_linear}

```gdscript
float get_volume_linear() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_volume_db {#set_volume_db}

```gdscript
void set_volume_db(volume: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_volume_linear {#set_volume_linear}

```gdscript
void set_volume_linear(volume: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### volume_db {#volume_db}

```gdscript
volume_db: float
```

- void set_volume_db(value: float)
- float get_volume_db()

Amount of amplification in dB. Positive values make the sound louder, negative values make it quieter. Value can range from -80 to 24.

### volume_linear {#volume_linear}

```gdscript
volume_linear: float
```

- void set_volume_linear(value: float)
- float get_volume_linear()

Amount of amplification as a linear value.

Note: This member modifies volume_db for convenience. The returned value is equivalent to the result of @GlobalScope.db_to_linear() on volume_db. Setting this member is equivalent to setting volume_db to the result of @GlobalScope.linear_to_db() on a value.
