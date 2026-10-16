# GodotInstance

**继承**：[Object](class_object.md) → **GodotInstance**

> GodotInstance represents a running Godot instance that is controlled from an outside codebase, without a perpetual main loop. It is created by the C API libgodot_create_godot_instance. Only one may be created per process.

<details><summary>官方英文描述（中文精译进行中）</summary>

GodotInstance represents a running Godot instance that is controlled from an outside codebase, without a perpetual main loop. It is created by the C API libgodot_create_godot_instance. Only one may be created per process.

</details>

## 方法

### focus_in {#focus_in}

```gdscript
void focus_in()
```

Notifies the instance that it is now in focus.

### focus_out {#focus_out}

```gdscript
void focus_out()
```

Notifies the instance that it is now not in focus.

### is_started {#is_started}

```gdscript
bool is_started()
```

Returns true if this instance has been fully started.

### iteration {#iteration}

```gdscript
bool iteration()
```

Runs a single iteration of the main loop. Returns true if the engine is attempting to quit.

### pause {#pause}

```gdscript
void pause()
```

Notifies the instance that it is going to be paused.

### resume {#resume}

```gdscript
void resume()
```

Notifies the instance that it is being resumed.

### start {#start}

```gdscript
bool start()
```

Finishes this instance's startup sequence. Returns true on success.
