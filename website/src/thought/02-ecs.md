# ECS 架构

**Entity = ID，Component = 纯数据，System = 批量逻辑。组合优于继承的架构化表达。**

## 三支柱

| 概念 | 含义 |
|---|---|
| Entity | 唯一 ID，没有数据也没有行为 |
| Component | 纯数据结构体（position/health/velocity） |
| System | 遍历所有有特定 Component 的 Entity，批量处理 |

## 为什么快

System 遍历 `[Position, Velocity]` 的 Entity 时，数据在内存里连续排列（SoA）。一次 cache line 加载多个实体，分支预测友好。

## 什么时候需要 ECS

> 值得注意的是：**ECS 只在规模上万时才比节点树快**。几十个敌人用 Godot 节点树完全够。ECS 的成本是架构复杂度，收益是万级实例的吞吐量。

## Godot 为什么不是 ECS

Godot 是 **scene graph + 组件混合体**。节点有类型有行为有生命周期，不是纯 ID。但 Godot 的 Server API（RenderingServer/PhysicsServer）底层就是 DOD 风格。

| 框架 | 架构 |
|---|---|
| Unity DOTS | 纯 ECS |
| Bevy | 纯 ECS（Rust） |
| Unreal Mass | ECS 风格 |
| Godot | Scene graph + Server |

## 延伸阅读

- 本站：[DOD](01-dod.md)、[Godot Server 架构](04-servers-dod.md)
