# 节点查找的真相

**$ 不是魔法快捷方式，是 get_node(NodePath) 的语法糖。每次调用都走一遍路径解析 + 子节点遍历。**

## 原理：NodePath 到底做了什么

你写 `$Player/Sprite2D`，Godot 内部执行的是：

1. 把字符串 `"Player/Sprite2D"` 解析成路径段列表：`["Player", "Sprite2D"]`
2. 从当前节点开始，在 `get_children()` 返回的数组里逐个比较名字，找到 "Player"
3. 进入 "Player"，再在它的 `get_children()` 里找 "Sprite2D"

关键：**第 2 步是线性遍历**。Godot 的 Node 内部用 `List<Node*>` 存储子节点，不是字典。子节点越多，找名字越慢。

```
当前节点的 children: [Enemy, GUI, Player, Camera, Music]
找 "Player" → 比较 Enemy(否) → GUI(否) → Player(命中) = 3 次比较
```

## 四种写法的性能阶梯

| 写法 | 每次调用做什么 | 相对开销 |
|---|---|---|
| `get_node("Player/Sprite2D")` | 解析路径字符串 + 两次线性遍历 | 100% |
| `$Player/Sprite2D` | 同上（语法糖） | 100% |
| `%PlayerSprite` | Godot 在场景树里建了唯一节点索引表，O(1) 查找 | ~20% |
| `@onready var s := $Player/Sprite2D` | 只在 _ready 时查一次，之后直接用成员变量 | ~0% |

## @onready 的原理

`@onready` 不是什么特殊机制。它等价于：

```gdscript
var sprite: Sprite2D

func _ready():
    sprite = $Player/Sprite2D
```

在 `_ready()` 里查一次节点，存到成员变量。之后所有访问都是直接指针解引用，零查找开销。

## % 唯一名节点的原理

在编辑器里右键节点 → "Access as Unique Name"，节点名前会出现 `%`。Godot 内部给每个标记过的节点维护一个 `unique_nodes` 字典。`%PlayerSprite` 查这个字典，O(1)。但仍然比直接成员变量慢，因为多了一次哈希查找。

## 反例：每帧 $ 查找

```gdscript
# 这个脚本挂在 Player 节点上
func _process(delta):
    # 每次都查 "Sprite2D"，5 个子节点 = 5 次比较
    $Sprite2D.position.x += speed * delta
    # 每次都查 "Sprite2D"，又是 5 次比较
    $Sprite2D.modulate = Color(1, 1, 1, sin(time) * 0.5 + 0.5)
    # 每次都查 "CollisionShape2D"
    $CollisionShape2D.position = $Sprite2D.position
```

一帧 3 次 $ 查找，60 FPS = 每秒 180 次子节点遍历。子节点 10 个就是 1800 次字符串比较。

## 正确写法

```gdscript
@onready var sprite := $Sprite2D as Sprite2D
@onready var collision := $CollisionShape2D as CollisionShape2D

func _process(delta):
    sprite.position.x += speed * delta
    sprite.modulate = Color(1, 1, 1, sin(Time.get_ticks_msec() / 1000.0) * 0.5 + 0.5)
    collision.position = sprite.position
```

_ready 时查一次，之后全是成员变量访问。

## 什么时候 $ 没问题

- 在 `_ready()`、`_enter_tree()` 等只调一次的回调里
- 在按钮信号回调里（用户点击频率低）
- 在编辑器扩展里

**只要不在 `_process`、`_physics_process` 里反复 $，就没问题。**

## 延伸阅读

- 本站：[GDScript 高性能实施](02-gdscript.md)
- 官方：[Godot interfaces - best practices](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_interfaces.html)
