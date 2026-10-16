# Jai 与 Jonathan Blow 理念

**为游戏而生的语言。无隐藏控制流，秒级编译。**

## 背景

Jonathan Blow（Braid / The Witness 作者）2014 年立项 Jai。目标：解决 C++ 的编译慢和隐藏控制流问题。

## 核心设计

| 特性 | 含义 |
|---|---|
| 无异常 | 错误用返回值，不用 try/catch |
| 无 RAII | 析构函数显式调用，不隐式触发 |
| 无隐式构造析构 | 一切分配和释放显式 |
| `#run` | 编译时执行代码（类似 Zig comptime） |
| context allocator | 分配器作为参数传递，不隐式 malloc |

## 编译速度

> 值得注意的是：**80k 行代码编译 < 1 秒**。C++ 编译同样规模要几十秒到几分钟。快编译 = 快迭代 = 生产力。

## 对 Zig / Odin 的影响

Jai 是这一代系统语言的思想源头之一：显式控制流、编译时执行、自定义分配器。Zig 的 `comptime` 和 `std.heap.page_allocator` 都能看到 Jai 的影子。

## 与 godotlsp 的共鸣

godotlsp 用 Rust 写，追求快编译快迭代。Rust 编译比 Jai 慢，但比 C++ 快。核心原则一致：**工具链速度影响开发速度**。

## 延伸阅读

- Jonathan Blow 演讲：[The Art of Code](https://www.youtube.com/watch?v=W3s_iC2lJ6M)
- 本站：[DOD](01-dod.md)
