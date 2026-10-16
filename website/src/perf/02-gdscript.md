# GDScript 高性能实施

> 🚧 **骨架阶段占位页**。本页正文按里程碑填充（P3/P4 期，见 `docs/gdscript-docs-site-plan.md` §六）。

## 本章计划小节

- 四类热点杀手：无类型变量（typed GDScript 真实加速，Array[T] 防装箱）/热循环 get_node（@onready 缓存）/_process 字符串操作/重复 connect 不去重
- 升级阶梯：修算法→加类型→C#→GDExtension（顺序不可颠倒）
- profile first 纪律
