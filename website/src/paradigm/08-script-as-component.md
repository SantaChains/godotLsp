# 脚本即组件

**GDScript 脚本附加在节点上，是节点的行为定义，不是独立对象。**

## 脚本 IS-A 节点

```gdscript
# 这个脚本就是一个 CharacterBody2D，不是"挂在 CharacterBody2D 上的东西"
extends CharacterBody2D

var speed := 200.0

func _physics_process(delta):
    velocity = Input.get_vector("left", "right", "up", "down") * speed
    move_and_slide()
```

在编辑器里把这个脚本拖到一个 CharacterBody2D 节点上，那个节点就变成了"会移动的玩家"。脚本没有独立存在，它就是节点本身。

> 类型必须对上：`extends CharacterBody2D` 的脚本只能挂在 CharacterBody2D（或其子类）节点上，挂到普通 Node 上会报类型不匹配。脚本定义的是"在这个节点类型上追加行为"，不是脱离节点的独立组件。

## extends 链

```gdscript
# enemy.gd
extends CharacterBody2D

var hp := 100

func take_damage(amount: int):
    hp -= amount
```

```gdscript
# flying_enemy.gd —— 继承 enemy.gd
extends "enemy.gd"

var flight_height := 50.0

func _physics_process(delta):
    # 调用父类方法
    super._physics_process(delta)
    # 自己加飞行逻辑
    position.y += sin(Time.get_ticks_msec() / 1000.0) * 0.5
```

## class_name：全局注册

```gdscript
# 这个脚本声明了 class_name，其他文件不用 preload 就能用
class_name Player extends CharacterBody2D

func take_damage(amount: int):
    pass
```

```gdscript
# 其他文件直接用 Player 类型
var player := Player.new()
```

`class_name` 把脚本注册到全局类表，所有文件都能直接引用。

## 与 Unity 的区别

| | Unity | Godot |
|---|---|---|
| 对象 | GameObject（空容器） | Node（有类型） |
| 脚本 | Component（挂上去，独立存在） | 就是节点本身 |
| 组合 | 给空物体挂多个组件 | 把子节点作为功能拆分 |
| 调用父类 | base.Method() | super.Method() |

## 什么时候用继承，什么时候用组合

```gdscript
# IS-A：Boss 是一种 Enemy → 用继承
extends "enemy.gd"

# HAS-A：Enemy 有血量 → 用子节点/Resource
# Health 子节点管血量，Enemy 根节点协调
```

## 延伸阅读

- 本站：[节点](02-node.md)、[组合优于继承](09-composition.md)
- 官方：[Scripting](https://docs.godotengine.org/en/stable/tutorials/scripting/index.html)
