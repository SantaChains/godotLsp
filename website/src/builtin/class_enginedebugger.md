# EngineDebugger

**继承**：[Object](class_object.md) → **EngineDebugger**

> EngineDebugger handles the communication between the editor and the running game. It is active in the running game. Messages can be sent/received through it. It also manages the profilers.

<details><summary>官方英文描述（中文精译进行中）</summary>

EngineDebugger handles the communication between the editor and the running game. It is active in the running game. Messages can be sent/received through it. It also manages the profilers.

</details>

## 方法

### clear_breakpoints {#clear_breakpoints}

```gdscript
void clear_breakpoints()
```

Clears all breakpoints.

### debug {#debug}

```gdscript
void debug(can_continue: bool = true, is_error_breakpoint: bool = false)
```

Starts a debug break in script execution, optionally specifying whether the program can continue based on can_continue and whether the break was due to a breakpoint.

### get_depth {#get_depth}

```gdscript
int get_depth() const
```

Experimental: This method may be changed or removed in future versions.

Returns the current debug depth.

### get_lines_left {#get_lines_left}

```gdscript
int get_lines_left() const
```

Experimental: This method may be changed or removed in future versions.

Returns the number of lines that remain.

### has_capture {#has_capture}

```gdscript
bool has_capture(name: StringName)
```

Returns true if a capture with the given name is present otherwise false.

### has_profiler {#has_profiler}

```gdscript
bool has_profiler(name: StringName)
```

Returns true if a profiler with the given name is present otherwise false.

### insert_breakpoint {#insert_breakpoint}

```gdscript
void insert_breakpoint(line: int, source: StringName)
```

Inserts a new breakpoint with the given source and line.

### is_active {#is_active}

```gdscript
bool is_active()
```

Returns true if the debugger is active otherwise false.

### is_breakpoint {#is_breakpoint}

```gdscript
bool is_breakpoint(line: int, source: StringName) const
```

Returns true if the given source and line represent an existing breakpoint.

### is_profiling {#is_profiling}

```gdscript
bool is_profiling(name: StringName)
```

Returns true if a profiler with the given name is present and active otherwise false.

### is_skipping_breakpoints {#is_skipping_breakpoints}

```gdscript
bool is_skipping_breakpoints() const
```

Returns true if the debugger is skipping breakpoints otherwise false.

### line_poll {#line_poll}

```gdscript
void line_poll()
```

Forces a processing loop of debugger events. The purpose of this method is just processing events every now and then when the script might get too busy, so that bugs like infinite loops can be caught.

### profiler_add_frame_data {#profiler_add_frame_data}

```gdscript
void profiler_add_frame_data(name: StringName, data: Array)
```

Calls the add callable of the profiler with given name and data.

### profiler_enable {#profiler_enable}

```gdscript
void profiler_enable(name: StringName, enable: bool, arguments: Array = [])
```

Calls the toggle callable of the profiler with given name and arguments. Enables/Disables the same profiler depending on enable argument.

### register_message_capture {#register_message_capture}

```gdscript
void register_message_capture(name: StringName, callable: Callable)
```

Registers a message capture with given name. If name is "my_message" then messages starting with "my_message:" will be called with the given callable.

The callable must accept a message string and a data array as argument. The callable should return true if the message is recognized.

Note: The callable will receive the message with the prefix stripped, unlike EditorDebuggerPlugin._capture(). See the EditorDebuggerPlugin description for an example.

### register_profiler {#register_profiler}

```gdscript
void register_profiler(name: StringName, profiler: EngineProfiler)
```

Registers a profiler with the given name. See EngineProfiler for more information.

### remove_breakpoint {#remove_breakpoint}

```gdscript
void remove_breakpoint(line: int, source: StringName)
```

Removes a breakpoint with the given source and line.

### script_debug {#script_debug}

```gdscript
void script_debug(language: ScriptLanguage, can_continue: bool = true, is_error_breakpoint: bool = false)
```

Starts a debug break in script execution, optionally specifying whether the program can continue based on can_continue and whether the break was due to a breakpoint.

### send_message {#send_message}

```gdscript
void send_message(message: String, data: Array)
```

Sends a message with given message and data array.

### set_depth {#set_depth}

```gdscript
void set_depth(depth: int)
```

Experimental: This method may be changed or removed in future versions.

Sets the current debugging depth.

### set_lines_left {#set_lines_left}

```gdscript
void set_lines_left(lines: int)
```

Experimental: This method may be changed or removed in future versions.

Sets the current debugging lines that remain.

### unregister_message_capture {#unregister_message_capture}

```gdscript
void unregister_message_capture(name: StringName)
```

Unregisters the message capture with given name.

### unregister_profiler {#unregister_profiler}

```gdscript
void unregister_profiler(name: StringName)
```

Unregisters a profiler with given name.
