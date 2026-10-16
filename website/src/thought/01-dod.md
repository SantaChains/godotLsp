# 数据导向设计（DOD）

**先按访问方式布局数据，再写代码。硬件真相：cache miss 是性能杀手。**

## 硬件真相

| 存储 | 延迟 |
|---|---|
| L1 cache | ~4 周期 |
| L2 cache | ~12 周期 |
| L3 cache | ~40 周期 |
| 内存 | ~200-300 周期 |

CPU 一次加载 64 字节 cache line。如果数据连续排列，一次加载用多次。如果数据分散，每次都 cache miss。

## SoA vs AoS

```gdscript
# AoS（Array of Structures）：每个敌人一个对象
# 内存不连续，遍历所有敌人的 position 要跳着找
var enemies = [enemy1, enemy2, enemy3]

# SoA（Structure of Arrays）：position 存一起
# 遍历 position 时连续命中 cache
var positions = PackedVector2Array()
var speeds = PackedFloat32Array()
```

> 值得注意的是：Godot 的节点树是 AoS。万级同屏实例遍历 `_process` 很慢。解法是用一个管理器节点遍历 `PackedVector2Array`，绕过节点树。

## 核心句

**先按访问方式布局数据，再写代码。**

OOP 把数据和行为绑在一起，DOD 把数据拆开批量处理。DOD 不是替代 OOP，是热路径上的武器。

## 延伸阅读

- 本站：[ECS 架构](02-ecs.md)、[Godot Server 架构即 DOD](04-servers-dod.md)
- 官方：[Optimization using servers](https://docs.godotengine.org/en/stable/tutorials/performance/using_servers.html)
