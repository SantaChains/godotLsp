# 场景树与节点引用

**一切围绕「当前脚本挂在哪个节点上」展开**：取子节点、实例化场景、把握生命周期时序。这是 GDScript 与场景模型咬合最紧的一章。

## `$` 与 `%`：节点路径语法

```gdscript
$Sprite2D              # 直接子节点（NodePath 字面量的语法糖）
$UI/HealthBar          # 多级路径（/ 分隔）
$"UI/HP Bar"           # 名字含空格用引号
get_node("UI/HealthBar")   # 等价的函数形式

%HealthBar             # 唯一名（unique name）：跨层级直达
%Player/AnimationPlayer # 唯一节点的子节点
```

- `$` 等价 `get_node()`，返回 `Node`；类型化写法：`$UI/HealthBar as ProgressBar`。
- **`%` 唯一名**：在节点上下文菜单勾选「Access as Unique Name」后，**同场景内任何脚本**都可用 `%Name` 直达——层级重构不断链，**优先于相对路径使用**。
- 路径语法细节（`..`、`/root/` 绝对路径）与引擎解析规则见[引擎内部篇 · .tscn 解析](../internals/02-tscn.md)。

> 值得注意的是：**不要在 `_process` 里调 `$` / `get_node`**——每次都是一次树查找。缓存到成员变量（配合 `@onready`）是铁律。

## @onready 的时机

```gdscript
@onready var bar: ProgressBar = %HealthBar
```

初始化被推迟到 `_ready` 调用时执行。**为什么必须推迟**：子节点要等「本节点进树且整棵子树构建完成」才保证可用。时序图：

```
实例化 instantiate()        # 对象创建，_init 运行
  └─ add_child()            # 挂进树：_enter_tree 自上而下
       └─ 最深的子节点先触发 _ready   # 自下而上
            └─ 父节点 _ready 最后    # 此刻 @onready 变量已就位
```

## 实例化：preload / load / instantiate / add_child

```gdscript
const BulletScene := preload("res://scenes/bullet.tscn")  # 编译期加载

func spawn(pos: Vector2) -> void:
    var bullet := BulletScene.instantiate()   # 只是创建节点对象（未进树）
    bullet.global_position = pos
    add_child(bullet)                         # 挂进树：此刻生命周期才启动
```

| 调用 | 时机 | 适用 |
|---|---|---|
| `preload(path)` | **编译期**，路径必须常量 | 固定依赖（场景、贴图、脚本） |
| `load(path)` | 运行时，路径可以是变量 | 动态资源（存档里读出的路径） |
| `instantiate()` | 场景 → 节点 | PackedScene 专用 |
| `add_child(node)` | 节点 → 树 | **挂树后才跑 `_ready`/`_process`** |

三条铁律：

1. `instantiate()` 后忘 `add_child()`，节点不运行、不渲染（悬空对象）。
2. `preload` 循环依赖会编译报错，相互引用改成运行时 `load` 或信号解耦。
3. 销毁用 `queue_free()`（帧末安全释放）；`free()` 是立即释放，**别在自己正被调用的回调里 free 自己**。

## owner 与场景归属

`owner` 指向「打包进场景文件」的根：编辑器保存场景时，只有 `owner` 指向场景根的节点会被写进 `.tscn`。运行时 `add_child` 的节点 `owner` 为空——它在场景文件里**不存在**，重进场景即消失。`%` 唯一名查找同样依赖 owner 语义，边界情况见[引擎内部篇 · 静默失败](../internals/07-fatal.md)。

## 生命周期回调全表

固定名字、引擎自动调用（**不是你手动调用**）：

| 回调 | 触发时机 | 典型用途 |
|---|---|---|
| `_init()` | 实例构造（未进树） | 纯数据初始化 |
| `_enter_tree()` | 本节点进入树（自上而下） | 需要在子节点之前做的事 |
| `_ready()` | 本节点**及全部子节点**就绪（自下而上） | 初始化主战场：取节点、连信号 |
| `_process(delta)` | 每帧，帧率不固定 | UI、输入驱动的移动、动画（乘 `delta`） |
| `_physics_process(delta)` | 固定物理帧率（默认 60Hz） | **物理、碰撞、`move_and_slide` 必须写这里** |
| `_input(event)` | 每个输入事件 | 需要拦下所有输入的场合 |
| `_unhandled_input(event)` | 未被 UI 消费的输入 | ** gameplay 输入首选** |
| `_notification(what)` | 各类引擎通知 | 细粒度事件（`NOTIFICATION_PAUSED` 等） |
| `_exit_tree()` | 离开树 | 清理反向依赖 |
| `_draw()` | CanvasItem 重绘 | 自定义绘制（配 `@tool` 预览） |

顺序记忆：`_init → _enter_tree（父→子）→ _ready（子→父）→ 每帧 _process/_physics_process → _exit_tree（子→父）`。`delta` 与帧率无关化的原理见[范式篇 · delta 与时间](../paradigm/05-delta.md)。

## 延伸阅读

- 官方英文文档：[Nodes and scene instances](https://docs.godotengine.org/en/stable/tutorials/scripting/scene_tree.html)、[Node 类](https://docs.godotengine.org/en/stable/classes/class_node.html)
- 类参考锚点：[Node](../builtin/class_node.md)、[PackedScene · instantiate](../builtin/class_packedscene.md)
- 本站：[await 与协程](12-await.md)（下一步）、[场景 Scene](../paradigm/03-scene.md)
