# 高频坑速查

**Godot 新手最常踩的五个坑。**

## 1. _process 中 $ 查找节点

```gdscript
# 慢：每帧字符串查找
func _process(delta):
    var sprite = $Sprite2D  # 每帧都走 NodePath 解析

# 快：缓存
@onready var sprite = $Sprite2D
func _process(delta):
    sprite.position.x += speed * delta
```

## 2. 漏乘 delta

```gdscript
# 错：帧率依赖
position.x += speed

# 对：帧率无关
position.x += speed * delta
```

## 3. 物理逻辑放错回调

```gdscript
# 错：移动放 _process
func _process(delta):
    velocity = direction * speed
    move_and_slide()

# 对：移动放 _physics_process
func _physics_process(delta):
    velocity = direction * speed
    move_and_slide()
```

## 4. instantiate 后忘 add_child

```gdscript
# 错：创建了但不进树，_ready 不触发
var b = BulletScene.instantiate()
b.position = ...

# 对：add_child 后才活
var b = BulletScene.instantiate()
add_child(b)
b.position = ...
```

## 5. 已 free 节点访问

> 值得注意的是：同树节点销毁时信号自动断连。野引用主要来自异步 lambda 捕获了已销毁节点，或跨树持有引用。用 `is_instance_valid()` 检查。

## 延伸阅读

- 本站：[脚本致命面](../internals/07-fatal.md)
