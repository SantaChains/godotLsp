# 语言总览

**GDScript 是 Godot 的原生脚本语言：高阶、动态类型、依附于节点系统**。语法接近 Python（缩进分块），但它是**为游戏迭代速度专门设计的独立语言**——有自己的字节码虚拟机、独立的类型系统，以及与场景树深度耦合的运行时。

## 定位：它不是什么

| 常见误解 | 事实 |
|---|---|
| 「Godot 里的 Python」 | 缩进相似而已；无 `__init__`、无装饰器、无 `import`、`/` 对整数做截断 |
| 通用脚本语言 | 依附节点：脚本必须挂在 `Object` 子类实例上才有意义 |
| 性能语言 | 解释执行比 C# / GDExtension 慢一到两个数量级；热点路径的升级阶梯见[性能篇](../perf/02-gdscript.md) |
| 动态语言只能动态写 | 4.x 的类型标注是**一等公民**：编译期检查 + 运行时加速，强烈推荐 |

> 值得注意的是：Godot 不强制单一语言。同一工程里 GDScript（原型与玩法）、C#（重逻辑）、GDExtension/C++（引擎级扩展）可以共存，全部实现同一套引擎 API。

## 与 Python 速查对照

| 特性 | Python | GDScript 4.x |
|---|---|---|
| 代码块 | 缩进 | 缩进（冒号可省略） |
| 变量声明 | 赋值即声明 | `var` / `const` |
| 整数除法 `5 / 2` | `2.5`（真除法） | `2`（**截断**） |
| 取模负数 | 结果跟随除数（`-7 % 3 == 2`） | 结果跟随被除数（`-7 % 3 == -1`），数学取模用 `posmod()` |
| 主入口 | `if __name__ == ...` | 无；生命周期回调 `_ready()` / `_process()` 由引擎调用 |
| 继承 | `class A(B):` | 文件级 `extends B`，见[类系统](08-classes.md) |
| 包 / 模块 | `import` | 无命名空间导入；全局类注册用 `class_name` |
| 异常 | try/except | **无异常**；错误以 `Error` 枚举返回，见[错误处理模型](14-errors.md) |
| 协程 | async/await | `await`（无 async 关键字），见[await 与协程](12-await.md) |
| 三元 | `a if c else b` | 同左（且为右结合） |
| 字符串格式化 | f-string | `%` 运算符 / `String.format()`，见[字符串章](16-strings.md) |

## 最小完整脚本

```gdscript
extends Node2D

signal health_changed(new_hp: int)   # 信号声明

@export var max_hp: int = 100        # 编辑器面板可调
var hp: int = max_hp

func _ready() -> void:               # 节点进入场景树时调用一次
    take_damage(30)

func take_damage(amount: int) -> void:
    hp = maxi(hp - amount, 0)        # maxi 是全局工具函数（非内建方法）
    health_changed.emit(hp)          # 4.x 首选 emit() 写法
    if hp == 0:
        queue_free()                 # 从场景树移除并释放自身
```

逐行涉及的语言构造在后续章节展开：[词法](02-lexical.md) → [变量](03-vars.md) → [函数](07-functions.md) → [注解](09-annotations.md) → [信号](10-signals.md) → [场景树引用](11-scenetree.md)。

## 4.x 与 3.x 的断代差异

3.x 项目无法直接改语法过关，核心断点：

| 主题 | 3.x | 4.x |
|---|---|---|
| 协程 | `yield` | `await` |
| 导出变量 | `export var` | `@export`（注解全家） |
| 延迟赋值 | `onready var` | `@onready` |
| 工具模式 | `tool` | `@tool` |
| 网络 RPC | `remote` / `master` / `puppet` | `@rpc("any_peer", ...)` |
| 信号连接 | `connect("sig", obj, "method")` 字符串签名 | `sig.connect(callable)`，Callable 一等公民 |
| lambda | 无 | `func(x): return x * 2` 内联函数 |
| 静态成员 | 仅 static func | `static func` + `static var`（4.1+） |
| 属性 | `setget` | `setter` / `getter` 后缀语法 |
| 实例化场景 | `instance()` | `instantiate()` |

完整逐项对照见[附录 · 3.x → 4.x 迁移表](../appendix/migration.md)。

## 版本基线

本站以 **Godot 4.x（4.3+ 特性单独标注）** 为基线；类参考页由与 LSP 内核同源的引擎数据生成（4.7 stable），见[版本与适配说明](../intro/versions.md)。

## 延伸阅读

- 官方英文文档：[GDScript reference](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 本站：[词法基础](02-lexical.md)（下一步）、[范式篇 · 三大支柱](../paradigm/01-pillars.md)
