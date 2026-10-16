# autoload 单例

**autoload 把一个节点/脚本提升为全工程单例**：项目设置里注册一个名字，引擎启动时自动实例化并挂到根下，**任何脚本直接用这个名字访问**。

## 注册机制

编辑器：Project Settings → Globals → Autoload，登记 `Path` 与名字（如 `GameState` → `res://scripts/game_state.gd`）。落盘到 `project.godot`：

```ini
[autoload]

GameState="*res://scripts/game_state.gd"
```

**名字前的 `*` 是全局标志**：带 `*` 的名字注册进全局脚本类命名空间，代码里可直接写 `GameState.add_coins(5)`；**不带 `*` 只实例化挂树，名字不是全局标识符**——引用它会报 `Identifier not declared`，这是正确行为而非编辑器 bug（引擎源码 `project_settings.cpp` 对 `*` 前缀的判定）。

## 访问形态

```gdscript
# game_state.gd
extends Node                       # ⚠️ 必须继承 Node 谱系

signal coins_changed(total: int)
var coins := 0

func add_coins(n: int) -> void:
    coins += n
    coins_changed.emit(coins)
```

```gdscript
# 任意脚本
GameState.add_coins(10)            # 直接用注册名（带 * 时）
get_node("/root/GameState")        # 等价的树路径形式（有无 * 都行）
```

三种承载选择：

| 注册目标 | 形态 | 适用 |
|---|---|---|
| 脚本（`.gd`） | 引擎自动包一个 Node | 纯状态/服务：存档、音频管理 |
| 场景（`.tscn`） | 整棵树常驻 | 带子节点的管理器：UI 层、世界容器 |
| 显式关闭 `*` | 仅挂树 | 需要 get_node 查找但不污染全局命名 |

## 典型用途与替代方案

```gdscript
# 典型 autoload 清单
GameState      # 全局状态：分数、进度
AudioManager   # 音效池：play_sfx("hit")
EventBus       # 跨场景信号中转（防 UI 反向依赖玩法节点）
SaveManager    # 存档读写
```

**不要 autoload 一切**：每个 autoload 都是常驻内存 + 全局可变状态。玩家、敌人这类**多实例**对象永远不该进 autoload——那是场景实例化的事（[场景树章](11-scenetree.md)）。4.1+ 的 `static var`（[类系统](08-classes.md)）可以承载轻量单例状态，不需要进树的纯数据优先考虑它。

## 加载时序

autoload 在**主场景之前**加载（`_init → _enter_tree → _ready` 都早于主场景），所以：

1. autoload 的 `_ready` 里**不能访问主场景节点**（还没建）。
2. 主场景的 `_ready` 里**可以放心用**一切 autoload。
3. autoload 相互依赖按注册表顺序初始化——把被依赖者放上面。

> 值得注意的是：注册的脚本**必须继承 `Node` 或其子类**，否则引擎启动时 `ERR_CONTINUE` **静默跳过**该 autoload——不报错、不上线，只剩一个查不到的单例。这是[引擎内部篇 · 静默失败](../internals/07-fatal.md)合集的高频成员，也是 LSP 静态检查的覆盖点之一。

## 延伸阅读

- 官方英文文档：[Singletons (Autoload)](https://docs.godotengine.org/en/stable/tutorials/scripting/singletons_autoload.html)
- 类参考锚点：[ProjectSettings](../builtin/class_projectsettings.md)
- 本站：[错误处理模型](14-errors.md)（下一步）、[引擎内部篇 · 工程启动链路](../internals/01-boot.md)
