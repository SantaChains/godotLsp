# 组合优于继承

**深继承的死因是组合爆炸和脆弱基类。Godot 的解法：把子节点当组件，父节点只做协调。**

## 深继承为什么死

假设你做一个敌人系统。用继承：

```
CharacterBody2D
└── Enemy                    # 血量、受伤
    ├── FlyingEnemy          # 飞行动力学
    │   └── BossFlying       # Boss 血量翻倍
    └── GroundEnemy          # 地面移动
        └── BossGround       # Boss 血量翻倍
```

现在需求来了：

- Boss 既要飞又要近战？→ `BossFlyingMeleeEnemy`，又一条继承链
- 加一个"会隐身"的敌人？→ 每个子类都要加 Stealth 字段和方法
- 基类 `Enemy` 改了 `take_damage()` 的签名 → 所有子类跟着改

这就是 **fragile base class（脆弱基类）**：基类的一个小改动，波及整个继承树。子类不知不觉依赖了基类的内部实现。

## Godot 的解法：子节点即组件

不用继承叠加功能，用节点组合：

```
Enemy (CharacterBody2D)     ← 根节点，只做协调
├── Health                 ← 血量逻辑（脚本）
├── Movement               ← 移动逻辑（脚本）
├── Attack                  ← 攻击逻辑（脚本）
└── Stealth                 ← 隐身（可选子节点，没有就不挂）
```

每个子节点是一个独立组件脚本，只管自己的事。根节点通过 `$Health`、`$Movement` 引用它们，调用接口。

## 完整代码示例

```gdscript
# health.gd —— 挂在 Health 子节点上
extends Node

signal died(cause: String)

var max_hp := 100
var hp := 100

func take_damage(amount: int):
    hp = max(0, hp - amount)
    if hp == 0:
        died.emit("dead")

func is_dead() -> bool:
    return hp <= 0
```

```gdscript
# movement.gd —— 挂在 Movement 子节点上
extends Node

var speed := 200.0

func move(direction: Vector2, delta: float):
    var body := get_parent() as CharacterBody2D
    body.velocity = direction * speed
    body.move_and_slide()
```

```gdscript
# enemy.gd —— 根节点，只做协调
extends CharacterBody2D

@onready var health := $Health as Node
@onready var movement := $Movement as Node

func _ready():
    health.died.connect(_on_died)

func _physics_process(delta):
    # 协调：读输入 → 调 Movement
    var dir := Input.get_vector("left", "right", "up", "down")
    movement.move(dir, delta)

func _on_died(_cause: String):
    queue_free()

# 外部调用：受击
func take_damage(amount: int):
    health.take_damage(amount)
```

## 关键原则

| 原则 | 说明 |
|---|---|
| 根节点不写逻辑 | 只做引用和协调，具体逻辑全在子节点 |
| 子节点互相不知道 | Health 不引用 Movement，Movement 不引用 Health |
| 根节点才知道全貌 | 只有 Enemy 知道 Health 和 Movement 的存在 |
| 加功能 = 加子节点 | 隐身 = 加 Stealth 子节点，不改现有代码 |

## 信号解耦 vs 直接调用

```gdscript
# 紧耦合：根节点直接调子节点
health.take_damage(10)

# 松耦合：信号广播，谁关心谁连
health.died.emit("shot")
# 不需要知道谁监听，HUD、音效、掉落物各自连接
```

直接调用要求知道对方接口。信号只要求知道事件名。血量归零后，HUD 要更新、音效要播放、掉落物要生成——根节点不需要一个个通知，emit 一个信号全响应。

## 继承什么时候还用

组合不是否定继承。**IS-A 关系用继承，HAS-A 关系用组合**：

| 关系 | 例子 | 用什么 |
|---|---|---|
| IS-A | "Boss 是一种 Enemy" | 继承 |
| HAS-A | "Enemy 有血量" | 组合（Health 子节点） |
| HAS-A | "Enemy 有移动" | 组合（Movement 子节点） |
| 可插拔 | "某些敌人会隐身" | 组合（可选 Stealth 子节点） |

## 与 ECS 的关系

Godot 节点树是**场景图 + 组件混合体**，不是纯 ECS。但思想一致：数据和行为拆开，靠组合拼装。ECS 把这个推到极致——Entity 只是 ID，Component 是纯数据，System 批量处理。Godot 的子节点脚本是"带行为的组件"，比纯 ECS 重，但比深继承灵活得多。

## 延伸阅读

- 本站：[节点](02-node.md)、[脚本即组件](08-script-as-component.md)
- 思想篇：[DOD](../thought/01-dod.md)、[ECS](../thought/02-ecs.md)
- Robert Nystrom《Game Programming Patterns》—— Component 模式
