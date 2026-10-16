# GDExtension 是什么

**GDExtension 让你用 C++ 写共享库，运行时加载进 Godot，不用编译引擎本身。**

## 核心概念

GDExtension 是 Godot 4 的原生扩展系统。你写 C++ 代码，编译成 `.dll`/`.so`/`.dylib`，Godot 启动时通过 `.gdextension` 配置文件加载。扩展类像内置类一样被 GDScript 使用。

## GDExtension vs 自定义模块

| | GDExtension | Custom Module |
|---|---|---|
| 编译 | 只编译你的库 | 重新编译整个引擎 |
| 分发 | 随项目走 | 需要分发引擎二进制 |
| 性能 | 接近原生 | 原生 |
| API 稳定性 | 跨版本兼容（`compatibility_minimum`） | 随引擎源码变 |
| 适用 | 项目级高性能模块 | 引擎级改动 |

> 值得注意的是：**GDExtension 不触碰 Godot 内部逻辑**。你写的是独立的 C++ 类，通过 Godot 的 C ABI 与引擎交互。不会修改渲染管线、物理内核或场景树。

## 什么时候需要

| 场景 | 方案 |
|---|---|
| 游戏逻辑 | GDScript |
| 性能热点（算法、模拟） | C# → GDExtension |
| 万级实例批量计算 | GDExtension + Server API |
| 第三方 C/C++ 库集成 | GDExtension |
| 需要 SIMD/多线程密集计算 | GDExtension |

## 不适合的场景

- UI 逻辑、简单游戏流程——GDScript 够快
- 需要修改引擎内部渲染/物理行为——那是 Custom Module，不是 GDExtension
- 跨平台分发复杂的——GDExtension 要为每个平台编译二进制

## 延伸阅读

- 本站：[C++ 环境搭建](02-setup.md)、[暴露类与方法](03-binding.md)
- 官方：[What is GDExtension?](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html)
