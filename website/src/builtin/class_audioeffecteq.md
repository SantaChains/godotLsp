# AudioEffectEQ

**继承**：[AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectEQ**

> An "equalizer" gives you control over the gain of frequencies in the entire spectrum, by allowing their adjustment through bands. A band is a point in the frequency spectrum, and each band means a division of the spectrum that can be adjusted.

<details><summary>官方英文描述（中文精译进行中）</summary>

An "equalizer" gives you control over the gain of frequencies in the entire spectrum, by allowing their adjustment through bands. A band is a point in the frequency spectrum, and each band means a division of the spectrum that can be adjusted.

Use equalizers to compensate for existing deficiencies in the audio, make room for other elements, or remove undesirable frequencies. AudioEffectEQs are useful on the Master bus to balance the entire mix or give it more character. They are also useful when a game is run on a mobile device, to adjust the mix to that kind of speakers (it can be disabled when headphones are plugged in).

</details>

## 方法

### get_band_count {#get_band_count}

```gdscript
int get_band_count() const
```

Returns the number of bands of the equalizer.

### get_band_gain_db {#get_band_gain_db}

```gdscript
float get_band_gain_db(band_idx: int) const
```

Returns the band's gain at the specified index, in dB.

### set_band_gain_db {#set_band_gain_db}

```gdscript
void set_band_gain_db(band_idx: int, volume_db: float)
```

Sets band's gain at the specified index, in dB.
