# 场景树与节点引用

> 🚧 **骨架阶段占位页**。本页正文按里程碑填充（语法篇 W3、范式/内部篇 W4、类参考由生成管线产出）。
> 规划见仓库 `docs/gdscript-docs-site-plan.md`。

## 本章计划小节

- $/% 与 get_node
- owner 与场景归属
- 实例化 preload/load/instantiate/add_child（add_child 才激活生命周期）
- @onready 时机
- _ready 缓存引用最佳实践
- 生命周期回调全表（_init/_enter_tree/_ready/_process/_physics_process/_input/_unhandled_input/_exit_tree/_notification 顺序图）
