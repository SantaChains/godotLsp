# 场景 Scene

**场景 = 保存到文件的节点树 = 预制体。这是 Godot 复用的核心机制。**

## .tscn 文件

一个场景就是一个 `.tscn` 文本文件，描述一棵节点树：

```
[node name="Player" type="CharacterBody2D"]
script = ExtResource("player_script.gd")

[node name="Sprite2D" type="Sprite2D" parent="."]
texture = ExtResource("player.png")
```

可以在编辑器里可视化搭建，也可以手写。

## 实例化

```gdscript
var BulletScene := preload("res://bullet.tscn")

func shoot():
    var bullet := BulletScene.instantiate()
    bullet.position = $Gun/Muzzle.position
    add_child(bullet)  # 必须 add_child 才进入场景树
```

> 值得注意的是：`instantiate()` 只是在内存中创建节点树，**不进入场景树**。必须 `add_child()` 之后，`_ready()`、`_process()` 才会触发。

## 嵌套场景

场景可以嵌套：玩家场景里放一个 HUD 场景，敌人场景里放一个血条场景。嵌套是组合的基础。

## 复用纪律

| 做成独立场景 | 理由 |
|---|---|
| 子弹 | 频繁实例化，参数一致 |
| 敌人 | 多种敌人共享骨架 |
| UI 面板 | 弹窗/菜单复用 |
| 技能特效 | 动画+音效打包 |

## 延伸阅读

- 本站：[节点](02-node.md)、[资源](07-resource.md)
- 官方：[Instancing](https://docs.godotengine.org/en/stable/getting_started/step_by_step/instancing.html)
