# AudioEffectNotchFilter

**继承**：[AudioEffectFilter](class_audioeffectfilter.md) → [AudioEffect](class_audioeffect.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectNotchFilter**

> A "notch" filter attenuates frequencies at AudioEffectFilter.cutoff_hz and allows frequencies outside the frequency threshold to pass unchanged. It is a narrower and stronger version of AudioEffectBandLimitFilter, and is the opposite of AudioEffectBandPassFilter.

<details><summary>官方英文描述（中文精译进行中）</summary>

A "notch" filter attenuates frequencies at AudioEffectFilter.cutoff_hz and allows frequencies outside the frequency threshold to pass unchanged. It is a narrower and stronger version of AudioEffectBandLimitFilter, and is the opposite of AudioEffectBandPassFilter.

This filter can be used to give more room for other sounds to play at that frequency. Because of how much it attenuates frequencies, it can also be used to completely remove undesired frequencies.

</details>
