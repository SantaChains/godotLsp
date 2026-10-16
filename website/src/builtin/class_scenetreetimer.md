# SceneTreeTimer

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **SceneTreeTimer**

> A one-shot timer managed by the scene tree, which emits timeout on completion. See also SceneTree.create_timer().

<details><summary>官方英文描述</summary>

A one-shot timer managed by the scene tree, which emits timeout on completion. See also SceneTree.create_timer().

As opposed to Timer, it does not require the instantiation of a node. Commonly used to create a one-shot delay timer as in the following example:

```
func some_function():
    print("Timer started.")
    await get_tree().create_timer(1.0).timeout
    print("Timer ended.")
```

```
public async Task SomeFunction()
{
    GD.Print("Timer started.");
    await ToSignal(GetTree().CreateTimer(1.0f), SceneTreeTimer.SignalName.Timeout);
    GD.Print("Timer ended.");
}
```

The timer will be dereferenced after its time elapses. To preserve the timer, you can keep a reference to it. See RefCounted.

Note: The timer is processed after all of the nodes in the current frame, i.e. node's Node._process() method would be called before the timer (or Node._physics_process() if process_in_physics in SceneTree.create_timer() has been set to true).

</details>

## 方法

### get_time_left {#get_time_left}

```gdscript
float get_time_left() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_time_left {#set_time_left}

```gdscript
void set_time_left(time: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### time_left {#time_left}

```gdscript
time_left: float
```

- void set_time_left(value: float)
- float get_time_left()

The time remaining (in seconds).

## 信号

### timeout {#timeout}

```gdscript
signal timeout()
```

Emitted when the timer reaches 0.
