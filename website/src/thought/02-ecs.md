# ECS 架构

**ECS 是组合优于继承在数据层的彻底表达：实体只是编号，数据拆成可插拔的组件，行为写成遍历组件的系统。它同时带来解耦与批量吞吐，但复杂度只在规模上来后才值回票价。**

## 三支柱

| 概念 | 是什么 | 不是什么 |
|---|---|---|
| Entity | 一个纯 ID 编号 | 不挂数据、不带方法、没有父节点 |
| Component | 纯数据记录（position、health、velocity） | 不含行为，不互相调用 |
| System | 一类规则：遍历"同时拥有某些组件"的实体并批量处理 | 不持有实体状态，无个体记忆 |

新增一种行为 = 加一个组件 + 一个系统，而不是在继承链里插入新子类。这正是[组合优于继承](../paradigm/09-composition.md)走到极端的样子：能力自由拼装，避免继承树爆炸。

## 为什么快（以及为什么不只是因为快）

System 扫描 `[Position, Velocity]` 实体时，组件数据按 archetype 或稀疏集组织，偏向连续存放（SoA，见[DOD](01-dod.md)），一次 cache line 带走多个实体，分支稳定、预取友好。

但 ECS 的第一层收益其实是**解耦**：碰撞、移动、受伤、渲染各自独立，加新规则不改旧代码；性能是第二层红利。只拿"快"理解 ECS 会在小规模项目里误判。

主流实现分两派，思想一致、存储不同：

- **Archetype 式**（如 Unity DOTS、Bevy）：把组件组合相同的实体归到一类连续存储，扫描极快，组件组合频繁变化时有成本。
- **Sparse-set 式**（如 Flecs）：用集合索引，增删组件便宜，遍历依然高效。

## 什么时候才需要 ECS

> 纠正一个常见绝对化说法：不是"必须上万才快"。准确表述是——**几十到几百个实体，Godot 节点树 + 单个管理器遍历数组就够，ECS 的架构税收不回；当成百上千同质实体、规则交叉组合（移动/碰撞/受伤/ AI/拾取同时作用）时，ECS 的解耦与批量才明显回本。**

判断信号：你发现自己在继承链里反复加"会飞又会游泳还能拾取"的子类，或要对几百个实体跑同一套规则——这时候 ECS 才轮到。

## Godot 里的位置

Godot 本体是 scene graph + 组件混合，**没有内置 ECS**；底层 Server（RenderingServer/PhysicsServer）是 DOD 风格但不对游戏逻辑暴露完整 ECS。两条现实路径：

1. 小规模：自写轻量管理器（ID + 组件字典 + 遍历函数），思想到位即可，不引依赖。
2. 大规模：接成熟 ECS 库（如 C 库 Flecs）通过 GDExtension 接入热路径，逻辑层不必用脚本硬造数据库。

最小思想演示（非生产实现）：

```gdscript
# entity 只是整数 ID；组件用字典按类型分桶存储
var next_id := 0
var positions: Dictionary = {}   # id -> Vector2
var velocities: Dictionary = {} # id -> Vector2

func spawn() -> int:
    var id = next_id
    next_id += 1
    return id

func system_move(delta: float) -> void:
    for id in positions.keys():
        if velocities.has(id):      # 只处理同时有位置和速度的实体
            positions[id] += velocities[id] * delta
```

真实 ECS 的增删组件、查询过滤、事件调度远比这个完备，演示只说明"实体=ID、组件=数据、系统=遍历"三者的拆分。

## 框架对照

| 框架 | 架构 |
|---|---|
| Unity DOTS | 纯 archetype ECS |
| Bevy（Rust） | 纯 ECS |
| Unreal Mass | ECS 风格子系统 |
| Godot | Scene graph + Server 层 DOD，逻辑层无内置 ECS |

## 延伸阅读

- 本站：[DOD](01-dod.md)、[组合优于继承](../paradigm/09-composition.md)、[Godot Server 架构](04-servers-dod.md)
