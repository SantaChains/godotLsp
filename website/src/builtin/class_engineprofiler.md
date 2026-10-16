# EngineProfiler

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EngineProfiler**

> This class can be used to implement custom profilers that are able to interact with the engine and editor debugger.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class can be used to implement custom profilers that are able to interact with the engine and editor debugger.

See EngineDebugger and EditorDebuggerPlugin for more information.

</details>

## 方法

### _add_frame {#_add_frame}

```gdscript
void _add_frame(data: Array) virtual
```

Called when data is added to profiler using EngineDebugger.profiler_add_frame_data().

### _tick {#_tick}

```gdscript
void _tick(frame_time: float, process_time: float, physics_time: float, physics_frame_time: float) virtual
```

Called once every engine iteration when the profiler is active with information about the current frame. All time values are in seconds. Lower values represent faster processing times and are therefore considered better.

### _toggle {#_toggle}

```gdscript
void _toggle(enable: bool, options: Array) virtual
```

Called when the profiler is enabled/disabled, along with a set of options.
