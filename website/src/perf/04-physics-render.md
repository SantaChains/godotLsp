# 物理与渲染预算

**物理降档、碰撞 primitive 优先、渲染后端选型、GI 分档。**

## 物理预算

| 优化 | 做法 | 效果 |
|---|---|---|
| 降物理频率 | `Engine.physics_ticks_per_second = 30` | CPU 减半 |
| 碰撞形状 | 用矩形/圆形 primitive，不用 concave | 检测快一个数量级 |
| 睡眠 | 静止刚体自动休眠 | 不参与碰撞检测 |
| 远距 body | 远离相机的物理体禁用 | 省 CPU |

```gdscript
# 静态物体用 StaticBody2D，不要用 RigidBody2D
# StaticBody 不参与物理模拟，只提供碰撞
```

## 渲染后端选型

| 后端 | 适用 | 特点 |
|---|---|---|
| Forward+ | 桌面端 | 功能最全，支持 SDFGI、大量灯光 |
| Mobile | 移动端 | 简化光照，减 draw call |
| Compatibility | 旧设备/WebGL | 无 compute shader，最低画质 |

项目设置 → Rendering → Renderer → Rendering Method。

## GI 分档

| 方案 | 运行时开销 | 质量 | 适用 |
|---|---|---|---|
| 无 GI | 最低 | 扁平 | 像素游戏 |
| LightmapGI | 零（预计算） | 静态 | 固定场景 |
| VoxelGI | 中 | 动态物体 | 室内 |
| SDFGI | 高 | 实时全局 | 开放世界 |

## AA 取舍

| 方案 | 开销 | 效果 |
|---|---|---|
| FXAA | 最低 | 模糊，无边缘锯齿 |
| MSAA 2x/4x | 中 | 边缘硬锯齿消除 |
| TAA | 中高 | 适合动画，有拖影 |

## Environment 开销

> 值得注意的是：**Environment 里每个效果都加渲染时间**。辉光（Bloom）、DOF、雾、SSAO、光晕——关掉一半可能帧率翻倍。先全开，再逐个关看哪个最贵。

## 延伸阅读

- 本站：[图形学速览](../math/06-graphics.md)
- 官方：[Performance](https://docs.godotengine.org/en/stable/tutorials/performance/index.html)
