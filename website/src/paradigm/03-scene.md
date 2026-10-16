# 场景 Scene

**场景 = 保存为 .tscn 的节点树 = 预制体。这是 Godot 复用的核心机制。**

## .tscn 文件长什么样

```ini
[gd_scene load_steps=2 format=3]

[ext_resource type="Script" path="res://player.gd" id="1_abc"]

[node name="Player" type="CharacterBody2D"]
script = ExtResource("1_abc")

[node name="Sprite2D" type="Sprite2D" parent="."]
[node name="CollisionShape2D" type="CollisionShape2D" parent="."]
```

引擎加载这个文件 = 读入 → 实例化每个 `[node]` → 挂脚本 → 设属性。结果是一棵完整的节点树。

## PackedScene：内存中的场景

`.tscn` 文件加载到内存后是一个 `PackedScene` 对象。它不是节点，是**场景模板**。

```gdscript
# 加载场景文件 = 得到 PackedScene
var BulletScene: PackedScene = preload("res://bullet.tscn")

# instantiate() = 用模板造一棵新节点树
var bullet = BulletScene.instantiate()

# 此时 bullet 是一个 CharacterBody2D 节点，但还不在场景树里
# 必须 add_child 才活
bullet.position = $Gun/Muzzle.position
add_child(bullet)
```

> 值得注意的是：`instantiate()` 不进场景树。`_ready()` 不触发，`_process()` 不跑。必须 `add_child()` 之后才活。

## 嵌套场景

场景里可以放另一个场景实例。这是组合的基础：

```
Main.tscn (Node2D)
├── HUD.tscn            ← 实例化的 HUD 场景
├── Player.tscn         ← 实例化的玩家场景
└── TileMapLayer         ← 直接放的节点
```

在编辑器里：从文件系统拖一个 .tscn 到场景树，就是实例化。双击进去可以编辑子节点（需要开启"Editable Children"）。

## 实例与原场景的关系

改原场景文件，所有实例跟着变。这是 Godot 的优势：改子弹的贴图，游戏里所有子弹的子弹都变。

| 操作 | 效果 |
|---|---|
| 改原场景 | 所有实例同步 |
| 在实例上改属性 | 只影响这一个实例（覆盖默认值） |
| 开启 Editable Children | 可以在实例里改子节点属性 |

## 复用纪律

| 做成独立场景 | 理由 |
|---|---|
| 子弹 | 频繁 instantiate()，参数一致 |
| 敌人 | 多种敌人共享骨架，只改属性 |
| UI 面板 | 弹窗/菜单复用 |
| 技能特效 | 动画+音效打包 |

## 延伸阅读

- 本站：[节点](02-node.md)、[资源](07-resource.md)
- 官方：[Instancing](https://docs.godotengine.org/en/stable/getting_started/step_by_step/instancing.html)
