# 数据导向设计（DOD）

**DOD 不是一套类库，是一个提问顺序：先问"这些数据会被谁、以什么顺序、读哪些字段"，再决定数据怎么放，最后才写处理逻辑。OOP 的默认顺序是先定义对象封装数据，DOD 反过来。**

## 为什么 cache 决定成败

现代 CPU 不从主存直接取数，而是逐级缓存。典型 x86 量级（随架构浮动，只用于建立直觉）：

| 层级 | 典型延迟 |
|---|---|
| L1 cache | ~4 周期 |
| L2 cache | ~12 周期 |
| L3 cache | ~40 周期 |
| 主存 | ~200–300 周期 |

CPU 一次加载 64 字节（一条 cache line）进缓存，并借助硬件预取器顺着连续地址提前搬数据。于是两种遍历天差地别：

- **数据连续**：一次加载带走后续多个元素，预取器持续命中，循环几乎全在 L1 跑。
- **数据分散**：每个元素藏在不同堆对象里，指针跳到哪，cache miss 到哪，预取失效，循环耗时被主存延迟主导。

OOP 的 `enemies[i].position` 背后通常是对象指针链：先取指针，再解引用到对象，对象里 position 字段和别的字段混排，遍历"所有敌人的位置"时把用不到的血量、状态、引用也一并载入，浪费带宽。

## AoS 与 SoA

- **AoS（Array of Structures）**：一个对象装齐 position/health/speed，对象数组排列。适合"一次操作一个实体的全部字段"。
- **SoA（Structure of Arrays）**：position、health、speed 各自存成连续数组。适合"对所有实体只做一件事"，比如每帧只积分位移。

```gdscript
# AoS 风格：每个敌人是一个对象，字段混排
var enemies: Array[Enemy] = [e1, e2, e3]
func _physics_process(delta: float) -> void:
    for e in enemies:
        e.position += e.velocity * delta

# SoA 风格的思想演示：同类字段并排，管理器统一遍历
var positions := PackedVector2Array()
var velocities := PackedVector2Array()
func _physics_process(delta: float) -> void:
    for i in positions.size():
        positions[i] += velocities[i] * delta
```

> 边界要说清：GDScript 是解释/转换层，`PackedVector2Array` 比普通 Array 紧凑，但它不带来原生语言那种严格的连续内存保证。**真正榨干 cache 的 SoA 连续分配发生在 GDExtension（Rust/C++/Zig）热路径**；在 GDScript 层，DOD 的落地形态是"减少节点数量、一个管理器批量遍历、共享纹理合批"，而不是指望脚本数组替你打穿内存墙。

## DOD 的三个提问

动手布局前依次问：

1. **谁读这块数据？** 每帧读、隔帧读、还是只在事件时读？热数据和冷数据必须分开，别让冷字段占着热循环的 cache line。
2. **一次读哪些字段？** 位移循环只需要 position/velocity，就不要把 health、对话状态拉进同一张遍历表。
3. **遍历顺序是什么？** 按实体 ID 随机访问，还是线性扫描全场？前者用查表，后者用连续数组。

## 在 Godot 里的实践形态

| 规模 | OOP/节点树做法 | DOD/管理器做法 |
|---|---|---|
| 几十个实体 | 每实体一个节点 + `_process`，直观 | 没必要上 DOD |
| 几百个同类实体 | 对象池 + 单一管理器遍历数组 | 开始有收益 |
| 上千上万实例 | 每实体一个节点会拖垮场景树与 `_process` 调度 | MultiMesh 批绘 + 管理器/Server 层 |

配套原则：共享同一纹理与材质才能合批（见[批量与结构优化](../perf/03-bulk.md)）；把每帧都跑的逻辑集中到少数管理器，而不是散布到上百个节点各自的 `_process`。

## 先测量，再优化

DOD 是武器不是教条。先开 Debugger > Monitors / Performance 定位真正的热循环，确认瓶颈在遍历而不是绘制、脚本解释或物理，再改布局。为 50 个敌人上 ECS/SoA，付出的复杂度远超收益。

## 核心句

**数据为访问方式服务，而不是为"概念上属于谁"服务。DOD 不替代 OOP，它是热路径上的专用视角。**

## 延伸阅读

- 本站：[ECS 架构](02-ecs.md)、[Godot Server 架构即 DOD](04-servers-dod.md)、[批量与结构优化](../perf/03-bulk.md)
- 官方：[Optimization using servers](https://docs.godotengine.org/en/stable/tutorials/performance/using_servers.html)
