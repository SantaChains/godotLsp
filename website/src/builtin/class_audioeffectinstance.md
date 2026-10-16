# AudioEffectInstance

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **AudioEffectInstance**

> An audio effect instance manipulates the audio it receives for a given effect. This instance is automatically created by an AudioEffect when it is added to a bus, and should usually not be created directly. If necessary, it can be fetched at run-time with AudioServer.get_bus_effect_instance().

<details><summary>官方英文描述（中文精译进行中）</summary>

An audio effect instance manipulates the audio it receives for a given effect. This instance is automatically created by an AudioEffect when it is added to a bus, and should usually not be created directly. If necessary, it can be fetched at run-time with AudioServer.get_bus_effect_instance().

</details>

## 方法

### _process {#_process}

```gdscript
void _process(src_buffer: const void*, r_dst_buffer: AudioFrame*, frame_count: int) virtual
```

Called by the AudioServer to process this effect. When _process_silence() is not overridden or it returns false, this method is called only when the bus is active.

Note: It is not useful to override this method in GDScript or C#. Only GDExtension can take advantage of it.

### _process_silence {#_process_silence}

```gdscript
bool _process_silence() virtual const
```

Override this method to customize the processing behavior of this effect instance.

Should return true to force the AudioServer to always call _process(), even if the bus has been muted or cannot otherwise be heard.
