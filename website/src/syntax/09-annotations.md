# 注解全表

**注解（annotation）是 `@` 开头的声明修饰符**：告诉编辑器或编译器「下面这个声明要特殊对待」——暴露到面板、延迟赋值、进入工具模式等。本页是 hover 高频跳转目标，逐条给语义 + 参数 + 示例。

书写规则：注解作用于**其后的第一条非注解声明**，可同行堆叠（`@onready @export var b`），参数为常量表达式。

## @export 系：暴露到编辑器面板

`@export` 把成员变量送进 Inspector，策划/美术**不用改代码就能调参**。这是 GDScript 工作流的标志性特性。

```gdscript
@export var speed := 300.0            # 按推断类型自动选择控件
@export var item_name := "sword"      # 文本框
@export var icon: Texture2D           # 资源拖放槽
@export var target: NodePath          # 节点路径选择器
@export var team := Team.RED          # 枚举下拉
```

带参数的变体（全家福）：

| 注解 | 用途 | 示例 |
|---|---|---|
| `@export_range(lo, hi, step)` | 数值滑杆 | `@export_range(0, 1, 0.01) var grip := 0.5` |
| 同上 + `"or_greater"` / `"or_less"` | 允许超出上限/下限 | `@export_range(1, 100, 1, "or_greater") var hp := 50` |
| `@export_exp_easing` | 指数缓动曲线 | 音量、透明度类参数 |
| `@export_file("*.json")` | 文件选择器（可过滤扩展名） | `@export_file("*.tscn") var next_level` |
| `@export_global_file` | 同上但为 `res://` 全局绝对路径 | 插件配置类 |
| `@export_dir` / `@export_global_dir` | 目录选择器 | |
| `@export_multiline` | 多行文本 | `@export_multiline var description := ""` |
| `@export_color_no_alpha` | 颜色选择（禁透明度） | |
| `@export_node_path("Node2D", "Node3D")` | 限类型的节点路径 | |
| `@export_flags_2d_physics` 等 | 层位掩码多选框 | `@export_flags_2d_physics var mask := 1` |
| `@export_group("Name")` | 面板分组标题 | 后续属性归入该组 |
| `@export_subgroup("Name")` | 子分组 | |
| `@export_category("Name")` | 分类标题 | |

```gdscript
@export_range(0.0, 0.5 * MAX_SPEED)          # 参数可以是常量表达式
var initial_speed := 0.25 * MAX_SPEED
```

> 需要注意的是：**`@export` 与 `@onready` 叠加时，`@onready` 的初始化会在导出值就位后运行并覆盖它**——这是默认按错误处理的警告（`ONREADY_WITH_EXPORT`），别叠加，也别关闭警告。

## @onready：延迟到 _ready 赋值

```gdscript
@onready var bar: ProgressBar = $UI/HealthBar
# 等价于在 _ready() 里写 bar = $UI/HealthBar
```

场景子节点只有在进入场景树后才能可靠取到；`@onready` 把初始化推迟到 `_ready` 时刻，一行顶七行。机制细节与最佳实践见[场景树章](11-scenetree.md#onready-的时机)。

## @tool：编辑器内也执行

```gdscript
@tool
extends Node2D        # 该脚本在编辑器里也会跑 _ready/_process

func _process(delta: float) -> void:
    queue_redraw()    # 编辑器中实时预览自定义绘制
```

用途：自定义绘制预览、编辑器插件（配合 `EditorPlugin`）。**风险与纪律**：编辑器里跑的代码会真实修改场景状态——用 `Engine.is_editor_hint()` 分流编辑期/运行期行为，构造函数里别做重活。

## @rpc：多人同步标记

```gdscript
@rpc("any_peer", "call_local", "reliable")
func sync_position(pos: Vector2) -> void:
    global_position = pos
# 调用方：sync_position.rpc(p)     （经 MultiplayerAPI 分发）
```

模式参数（可组合）：`"authority"`（默认，仅权威端）/`"any_peer"`；`"call_remote"`（默认）/`"call_local"`；`"unreliable"`/`"unreliable_ordered"`/`"reliable"`（默认）。

## @warning_ignore：静默指定警告

```gdscript
@warning_ignore("unused_parameter")
func debug_hook(_delta: float) -> void:
    pass
```

参数为警告代码字符串（编辑器 Script 面板能看到全码）。**原则：能修就修，只在确知误报处使用**。

## @abstract：抽象类（4.5+）

```gdscript
@abstract
class_name Shape
extends Resource

@abstract func area() -> float      # 抽象方法：子类必须实现
```

标记类与成员为抽象：**禁止直接实例化抽象类**，未实现抽象方法的子类报错。纯语言层的接口约束，比「约定俗成的基类」多一层编译期保障。

## @static_unload

```gdscript
@static_unload
class_name Cache
static var pool: Array = []
```

脚本卸载时**清空其静态成员**。默认不清理（static 状态跨场景存活）；对随场景重建的缓存类加上它防内存驻留。

## @icon

```gdscript
@icon("res://interface/icons/item.png")
class_name Item
```

给脚本类指定编辑器图标。路径必须以 `res://` 开头。

## 全表速查

| 注解 | 作用对象 | 一句话 |
|---|---|---|
| `@export` 全家 | 成员变量 | 进 Inspector |
| `@onready` | 成员变量 | 初始化推迟到 _ready |
| `@tool` | 脚本 | 编辑器内也执行 |
| `@rpc(...)` | 函数 | 多人游戏远程调用 |
| `@warning_ignore(...)` | 声明 | 静默指定警告 |
| `@abstract` | 类 / 函数 | 抽象约束（4.5+） |
| `@static_unload` | 脚本 | 卸载时清静态成员 |
| `@icon(...)` | 脚本 | 自定义编辑器图标 |

## 延伸阅读

- 官方英文文档：[GDScript exports](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_exports.html)、[GDScript reference · Annotations](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 类参考锚点：[@GDScript · 注解成员](../builtin/class_at_gdscript.md)
- 本站：[信号系统](10-signals.md)（下一步）、[场景树与节点引用](11-scenetree.md)
