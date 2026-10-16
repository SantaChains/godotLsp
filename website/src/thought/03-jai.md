# Jai 与 Jonathan Blow 理念

**Jai 是 Jonathan Blow（Braid、The Witness 作者）2014 年立项的游戏向系统语言，至今处于 closed beta、未公开发布，不能当生产工具用；它的价值是把一代系统语言的设计主张讲清楚，供 Zig、Odin、Rust 对照。**

## 现状先说清

截至 2026 年，Jai 仍是封闭测试，没有公开稳定版，没有官方承诺的发布日期，Blow 自己的游戏《The Sinking Ship》内部引擎用 Jai 编写。**学习它的语法没有现实工程收益，读它的设计主张才有。** 想立刻上手同类思想，选 Odin（语法最接近、已发布）或 Zig（comptime 路线）。

## 核心设计主张

| 主张 | 含义 | 反对的 C++ 包袱 |
|---|---|---|
| 错误显式返回 | 错误用返回值/状态传递，不靠异常栈展开 | 隐藏的控制流与运行时成本 |
| 内存与分配显式 | 分配器作为参数传入（context allocator），不隐式 malloc/隐式析构 | RAII 背后自动触发的隐藏释放 |
| 编译期执行 `#run` | 构建、代码生成、数据烘焙就是普通 Jai 代码，无需独立宏语言/外部构建系统 | 宏与构建脚本两套语言 |
| 数据导向默认 | 语言层面鼓励按批量访问布局内存 | 对象抽象掩盖内存排布 |
| 极快编译目标 | Blow 的目标是百万行级秒级编译；公开演示中约 8 万行代码库桌面端编译低于 1 秒（演示值，不是通用保证） | C++ 单编译单元几十秒到分钟级 |

> 编译速度的真正论点不是"省几秒等待"：快编译让"改一行立刻跑"的 tight loop 成立，调试和实验成本下降，架构才能快速试错。工具链速度直接折算成开发速度。

## 对同期系统语言的映射

| Jai 主张 | 今天可用的对应 |
|---|---|
| 编译期执行/元编程 | Zig `comptime`、Zig 自定义 build.zig；Rust proc macro/const |
| 显式分配器 | Zig 传入 `Allocator`；Rust 的 `Global`/自定义 `GlobalAllocator`/Arena crate |
| 显式错误传递 | Zig 错误联合类型 `!T`；Rust `Result<T, E>` |
| 数据导向布局 | 各语言手写 SoA + 连续容器，Rust `Vec`/Packed 思路同理 |

## 对 Godot/GDExtension 的实际共鸣

本书热路径走 GDExtension（见[性能边界](../ext/04-boundaries.md)）：脚本层保持薄，把批量、分配敏感、每帧调用的部分下沉到原生库。Jai 主张的"显式、无隐藏、按数据布局"正是写这层原生代码时该守的纪律，而不是去等 Jai 本身。

## 延伸阅读

- 资料汇总：[inductive.no/jai](https://inductive.no/jai/)（closed beta 状态以官方与社区汇总为准）
- 本站：[DOD](01-dod.md)、[性能边界与模式](../ext/04-boundaries.md)
