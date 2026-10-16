# 节点 Node

**一切都是节点。节点构成树，树承载一切逻辑。**

## 继承树

```
Object
  └── Node
       ├── Node2D          # 2D 世界
       │    ├── Sprite2D
       │    ├── CharacterBody2D
       │    └── Camera2D
       ├── Node3D          # 3D 世界
       │    ├── MeshInstance3D
       │    └── Camera3D
       ├── Control          # UI
       │    ├── Button
       │    └── Label
       └── AnimationPlayer  # 工具节点
```

## 树形约束

- **单父多子**：每个节点只有一个父节点，但可以有任意多子节点。
- **坐标继承**：子节点的 position 是相对于父节点的。移动父节点，所有子节点跟着移动。
- **激活态继承**：父节点不可见，子节点全部不可见。
- **顺序即层级**：在编辑器里上下拖动节点改变绘制顺序（2D）或渲染顺序（3D）。

## 组合式设计

> 需要注意的是：**不要写一个 500 行的 Player.gd**。把功能拆到子节点：

```
Player (CharacterBody2D)
├── Sprite2D          # 显示
├── CollisionShape2D  # 碰撞
├── Camera2D         # 跟随
├── AnimationPlayer   # 动画
└── AudioStreamPlayer # 音效
```

每个子节点管自己的事，根节点只负责协调。这就是 Godot 的"组合大于继承"。

## 生命周期回调

| 回调 | 触发时机 |
|---|---|
| `_init()` | 对象被构造时（还不在树中） |
| `_enter_tree()` | 节点进入场景树 |
| `_ready()` | 节点及其所有子节点都进入树后 |
| `_process(delta)` | 每渲染帧 |
| `_physics_process(delta)` | 每物理帧（默认 60Hz） |
| `_exit_tree()` | 节点离开场景树 |
| `_notification(what)` | 通用通知，可监听任意生命周期事件 |

执行顺序（一棵新子树 add_child 后）：

```
父 _enter_tree → 子 _enter_tree（自顶向下）
→ 子 _ready（自底向上，最深的先 ready）
→ 父 _ready
```

> 值得注意的是：`_ready()` 只在节点第一次进入树时触发一次。`_enter_tree()` 每次进出树都触发。引用子节点用 `@onready` 或 `_ready()`，不要在 `_init()` 里找子节点（此时还没进树）。

```gdscript
func _ready():
    # 子节点都已就绪，可以安全引用
    $Sprite2D.modulate = Color.GREEN
```

## 延伸阅读

- 本站：[场景](03-scene.md)、[delta 与时间](05-delta.md)
- 官方：[Nodes](https://docs.godotengine.org/en/stable/classes/class_node.html)
