# await 与协程

**`await` 把「等一件事发生再继续」写成顺序代码**。GDScript 没有 async 关键字：**函数体内出现 `await`，该函数自动成为协程**——调用它立即返回，到 `await` 处挂起，事件到达后从断点恢复。

## 基本形态 {#kw-await}

```gdscript
func _ready() -> void:
    await get_node("Enemy").died       # 等信号：函数在此挂起
    print("enemy down")                # 信号发出后恢复执行

    await get_tree().create_timer(1.5).timeout   # 等时长：1.5 秒
    spawn_next_wave()
```

await 的右侧是**任何可以「完成」的东西**：

| 目标 | 挂起到 |
|---|---|
| `signal` | 信号首次发出（返回值是信号参数；无参返回 `null`） |
| 协程函数调用 | 该协程执行完毕（返回其 return 值） |
| `SceneTreeTimer.timeout` | 计时结束（延时最常用法） |
| `Tween.finished` | 补间播放完 |
| `ResourceLoader.load_threaded_request` 完成信号 | 资源异步加载 |

## 与 3.x yield 的断代 {#kw-yield}

3.x：`yield(函数调用, "completed")`——返回 GDScriptFunctionState，手动管理恢复；信号要包一层 timer。4.x 一律 `await`，`yield` 仅作迁移期保留关键字。**3.x 心智模型（函数状态对象可传递）在 4.x 不再适用**。

## 常用模式

**帧末等待**（规避物理回调中改树的限制）：

```gdscript
await get_tree().process_frame      # 等一帧（物理帧用 physics_frame）
```

**超时保护**（先到者胜——赛跑两路 await，Godot 无 select，用状态标记）：

```gdscript
var done := false
func fetch() -> void:
    await slow_operation()
    done = true

func with_timeout(seconds: float) -> void:
    fetch()                                   # 不 await：并发跑
    await get_tree().create_timer(seconds).timeout
    if not done:
        push_warning("fetch 超时")
```

**顺序动画链**：

```gdscript
func play_intro() -> void:
    $Door.open()
    await $Door.animation_finished
    $Light.dim()
    await get_tree().create_timer(0.4).timeout
    $Music.fade_in()
```

## 常见误用

1. **在 `_ready` 里 await 后访问已销毁的节点**：等待期间节点可能被 `queue_free`，恢复后 `self` 已死——长等待开头先存 `is_inside_tree()` 判断，或用弱引用。
2. **忘 await 调用协程**：协程函数不 await 也合法（fire-and-forget），返回的「执行状态」被丢弃；误以为拿到返回值，实际是 `null`。
3. **await 死锁**：await 一个**永远不会发出的信号**（比如节点已销毁、信号拼错名）——协程永久挂起，无报错。
4. **回调里 await 后修改树**：恢复点可能在任意帧，改树前加 `is_inside_tree()` 防御。

> 值得注意的是：`await` 不是多线程。协程仍跑在主线程，只是**分帧让出**——重计算不阻塞 UI 要用 `Thread` / `WorkerThreadPool`，那是[性能篇](../perf/03-bulk.md)的事。

## 延伸阅读

- 官方英文文档：[GDScript reference · Awaiting signals or coroutines](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 类参考锚点：[SceneTreeTimer](../builtin/class_scenetreetimer.md)、[Tween · finished](../builtin/class_tween.md)
- 本站：[autoload 单例](13-autoload.md)（下一步）、[信号系统](10-signals.md)
