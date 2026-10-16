# 批量与结构优化

**万 Node 各跑 _process 是调度灾难。单管理器遍历 typed array 才是解法。**

## 问题

10000 个节点每个都有 `_process()`，引擎要调度 10000 次回调，每次有 GDScript 解释器开销。

## 解法：管理器模式

```gdscript
# 一个管理器节点，遍历 typed array，只触发一次回调
var enemies: Array[CharacterBody2D] = []

func _physics_process(delta):
    for e in enemies:
        e.velocity.x += gravity * delta
        e.move_and_slide()
```

一个回调遍历 10000 个元素，比 10000 个独立回调快一个数量级。

## 对象池

子弹、粒子反复 instantiate/queue_free 会反复分配。池化复用：

```gdscript
var pool: Array[CharacterBody2D] = []
var BulletScene: PackedScene = preload("res://bullet.tscn")

func spawn(pos: Vector2):
    var b: CharacterBody2D
    if pool.is_empty():
        b = BulletScene.instantiate()
        add_child(b)
    else:
        b = pool.pop_back()
        b.visible = true
        b.process_mode = Node.PROCESS_MODE_INHERIT
    b.global_position = pos
    # 不再 free，回收时塞回池
```

```gdscript
# 回收而非销毁
func despawn(b: CharacterBody2D):
    b.visible = false
    pool.append(b)
```

## MultiMesh

一次 draw call 画上千个实例。适合纯视觉重复（草、树、子弹），GPU 批量处理，CPU 只传一次变换数组。

```gdscript
var mm := MultiMesh.new()
mm.transform_format = MultiMesh.TRANSFORM_2D
mm.instance_count = 1000
for i in 1000:
    mm.set_instance_transform_2d(i, Transform2D(0, Vector2(x, y)))
```

## Server API

绕过场景树，直接调 `RenderingServer`。适合万级实例且不需要节点行为的场景。

## 优化模式表

| 模式 | Godot 版 |
|---|---|
| 对象池 | preload 场景 + 复用回收，不 free |
| 脏标识 | 只更新变化的节点 |
| 数据局部性 | PackedVector2Array 连续内存 |
| 事件队列 | 信号批量处理 |

## 延伸阅读

- 本站：[DOD](../thought/01-dod.md)、[Server 架构](../thought/04-servers-dod.md)
- 官方：[MultiMesh](https://docs.godotengine.org/en/stable/tutorials/performance/procedural_geometry/multimesh.html)
