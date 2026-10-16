# EditorDebuggerSession

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorDebuggerSession**

> This class cannot be directly instantiated and must be retrieved via an EditorDebuggerPlugin.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class cannot be directly instantiated and must be retrieved via an EditorDebuggerPlugin.

You can add tabs to the session UI via add_session_tab(), send messages via send_message(), and toggle EngineProfilers via toggle_profiler().

</details>

## 方法

### add_session_tab {#add_session_tab}

```gdscript
void add_session_tab(control: Control)
```

Adds the given control to the debug session UI in the debugger bottom panel. The control's node name will be used as the tab title.

### is_active {#is_active}

```gdscript
bool is_active()
```

Returns true if the debug session is currently attached to a remote instance.

### is_breaked {#is_breaked}

```gdscript
bool is_breaked()
```

Returns true if the attached remote instance is currently in the debug loop.

### is_debuggable {#is_debuggable}

```gdscript
bool is_debuggable()
```

Returns true if the attached remote instance can be debugged.

### remove_session_tab {#remove_session_tab}

```gdscript
void remove_session_tab(control: Control)
```

Removes the given control from the debug session UI in the debugger bottom panel.

### send_message {#send_message}

```gdscript
void send_message(message: String, data: Array = [])
```

Sends the given message to the attached remote instance, optionally passing additionally data. See EngineDebugger for how to retrieve those messages.

### set_breakpoint {#set_breakpoint}

```gdscript
void set_breakpoint(path: String, line: int, enabled: bool)
```

Enables or disables a specific breakpoint based on enabled, updating the Editor Breakpoint Panel accordingly.

### toggle_profiler {#toggle_profiler}

```gdscript
void toggle_profiler(profiler: String, enable: bool, data: Array = [])
```

Toggle the given profiler on the attached remote instance, optionally passing additionally data. See EngineProfiler for more details.

## 信号

### started {#started}

```gdscript
signal started()
```

Emitted when a remote instance is attached to this session (i.e. the session becomes active).

### stopped {#stopped}

```gdscript
signal stopped()
```

Emitted when a remote instance is detached from this session (i.e. the session becomes inactive).

### breaked {#breaked}

```gdscript
signal breaked(can_debug: bool)
```

Emitted when the attached remote instance enters a break state. If can_debug is true, the remote instance will enter the debug loop.

### continued {#continued}

```gdscript
signal continued()
```

Emitted when the attached remote instance exits a break state.
