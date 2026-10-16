# 3.x → 4.x 迁移表

**Godot 4 语法大改。以下是最常见的迁移对照。**

## 关键字与注解变更

| 3.x | 4.x | 说明 |
|---|---|---|
| `yield()` | `await` | 协程关键字 |
| `export var x` | `@export var x` | 注解式导出 |
| `onready var x` | `@onready var x` | 注解式就绪 |
| `tool` | `@tool` | 编辑器运行脚本 |
| `master`/`slave` | `@rpc` | 网络同步注解 |
| `funcref()` | `Callable(self, "method")` | 函数引用 |
| `export_node_path` | `@export_node_path` | 节点路径导出 |

## 信号连接

```gdscript
# 3.x：字符串 + self
button.connect("pressed", self, "_on_pressed")

# 4.x：Callable，方法直接传
button.pressed.connect(_on_pressed)
```

## 实例化

```gdscript
# 3.x
var scene = preload("res://player.tscn").instance()

# 4.x
var scene = preload("res://player.tscn").instantiate()
```

## 移动 API（KinematicBody → CharacterBody）

```gdscript
# 3.x：KinematicBody2D，手动传速度
move_and_slide(velocity, Vector2.UP)

# 4.x：CharacterBody2D，速度存成员，重力在项目设置
velocity = direction * speed
move_and_slide()
# move_and_slide() 返回修正后的速度
```

## 输入

```gdscript
# 3.x
if Input.is_action_pressed("ui_left"):
    velocity.x -= speed

# 4.x：get_vector 直接出单位向量
var dir := Input.get_vector("left", "right", "up", "down")
```

## 类型系统

```gdscript
# 3.x：无类型数组
var items = []

# 4.x：泛型数组
var items: Array[int] = []

# 4.x：Packed 数组零拷贝
var positions := PackedVector2Array()
```

## Area2D 信号

```gdscript
# 3.x
body_entered.connect(self, "_on_body_entered")

# 4.x
body_entered.connect(_on_body_entered)
```

## 材质与资源

| 3.x | 4.x |
|---|---|
| SpatialMaterial | StandardMaterial3D |
| CanvasItemTexture | 子资源直接拖 |
| ColorRamp | Gradient |

## 延伸阅读

- 官方：[Migration to 4.0](https://docs.godotengine.org/en/stable/tutorials/migrating/index.html)
