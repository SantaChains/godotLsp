# GDExtension 是什么

**GDExtension 是 Godot 4 的原生扩展系统：你用编译型语言写动态库（.dll/.so/.dylib/.wasm），Godot 启动时经 `.gdextension` 清单加载，扩展类像内置类一样被 GDScript 调用，全程不重编译引擎。**

## 工作链路

```
你的原生代码（C++/Rust/Zig…）
   │ 编译为平台动态库
   ▼
.gdextension 清单（声明入口符号、依赖库、兼容版本）
   │ Godot 启动读取
   ▼
注册进 ClassDB，GDScript 直接 new 你的类、调你的方法
```

`.gdextension` 是纯文本清单：记录动态库路径（按平台分键）、初始化/反初始化入口符号、`compatibility_minimum`（最低兼容引擎版本）。它是加载凭证，不是代码本体。

## 支持语言

| 语言 | 绑定生态 | 定位 |
|---|---|---|
| C++ | 官方 `godot-cpp` | 一等公民，文档最全 |
| Rust | 社区 godot-rust（bindgen） | 内存安全，适合本仓库这类工具/热路径 |
| Zig | 社区 godot-zig 绑定 | 显式分配、comptime，构建轻量 |
| C# | **不是 GDExtension**，是独立 .NET 版引擎 | 走 Mono/CLR，另一条分发线 |

## GDExtension vs Custom Module

| | GDExtension | Custom Module |
|---|---|---|
| 编译 | 只编译你的库 | 重编译整个引擎 |
| 分发 | 动态库随项目走 | 必须分发改版引擎二进制 |
| 性能 | 接近原生（跨语言调用有 ABI 成本，见[边界](04-boundaries.md)） | 原生，无跨层 |
| 版本兼容 | `compatibility_minimum` 约束 ABI | 随引擎源码变，升级要重做 |
| 适用 | 项目级高性能模块、接第三方库 | 改动引擎本身的渲染/物理内核 |

> GDExtension 不修改 Godot 内部逻辑：你通过 C ABI 暴露的是自己的新类，不能替换渲染管线、物理求解器或场景树实现；要改内核那是 Custom Module。

## 什么时候值得上

| 场景 | 方案 |
|---|---|
| UI、流程、普通游戏逻辑 | GDScript，别上原生 |
| 单帧跑几十万次的算法/模拟 | GDExtension 批量处理 |
| 复用现成 C/C++/Rust 库 | GDExtension 包装 |
| SIMD、多线程密集计算 | GDExtension |
| 中小规模业务、团队无原生经验 | 留 GDScript 或 C# |

## 不适合

- 用原生写 UI：GDScript 与控件树咬合最好，C++ 无收益。
- 改引擎内部行为：GDExtension 做不到。
- 一份脚本发全平台却不愿按平台编译二进制：GDExtension 必须为每个目标平台出动态库。

## 延伸阅读

- 本站：[C++ 环境搭建](02-setup.md)、[暴露类与方法](03-binding.md)、[性能边界与模式](04-boundaries.md)
- 官方：[What is GDExtension?](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html)
