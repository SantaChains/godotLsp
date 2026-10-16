# 优化方法论

**测量先行。不要猜。三类慢有不同药方。**

## 三类慢

| 症状 | 原因 | 药方 |
|---|---|---|
| 每帧低帧率 | 每帧都有热点 | 修算法/加类型/减 draw call |
| 间歇卡顿 | GC/加载/shader 编译 | 预加载/对象池/异步 |
| 加载慢 | 资产过大/同步加载 | 压缩/异步流式 |

## 内置 Profiler

Godot 编辑器 → Debugger → Profiler 标签页。录制后看：
- 哪个函数占 CPU 时间最多
- 哪个节点的 _process 最慢
- 内存分配次数

## Debug Monitors

编辑器 → Debug → Show Debug Monitors。实时显示：
- FPS
- draw call 数
- 节点数
- 对象数
- 内存占用

## 手动计时

```gdscript
func _process(_delta):
    var t0 := Time.get_ticks_usec()
    do_expensive_thing()
    var t1 := Time.get_ticks_usec()
    var ms := (t1 - t0) / 1000.0
    if ms > 1.0:  # 超过 1ms 才打印
        print("耗时: ", ms, " ms")
```

## 假设检验

> 不要猜哪里慢。量出来。改一个假设，量一次。再改，再量。

## Knuth 原则

> "过早优化是万恶之源。"——但前提是你量过。**先让它对，再让它快，只量热点。**

## 外部工具

| 工具 | 用途 |
|---|---|
| Tracy | 全栈 profiling（开源） |
| Perfetto | Google 的 trace 工具 |
| Godot Profiler | 内置，脚本级 |

## 延伸阅读

- 本站：[GDScript 高性能实施](02-gdscript.md)、[批量与结构优化](03-bulk.md)
- 官方：[Profiling](https://docs.godotengine.org/en/stable/tutorials/performance/index.html)
