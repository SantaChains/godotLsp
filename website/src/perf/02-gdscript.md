# GDScript 高性能实施

**四类热点杀手。升级阶梯：修算法 → 加类型 → C# → GDExtension。**

## 四类热点

### 1. 无类型变量

```gdscript
# 慢：Variant 装箱，每次访问走动态分发
var items = []
for i in 1000:
    items.append(i)

# 快：泛型数组，免装箱
var items: Array[int] = []
for i in 1000:
    items.append(i)
```

```gdscript
# 慢
var x = 0
func add():
    x += 1

# 快
var x: int = 0
func add():
    x += 1
```

### 2. 热循环 get_node

```gdscript
# 慢：每帧字符串查找
func _process(_delta):
    $Sprite2D.position.x += speed * delta

# 快：onready 缓存
@onready var sprite := $Sprite2D
func _process(delta):
    sprite.position.x += speed * delta
```

### 3. 字符串操作

```gdscript
# 慢：每帧拼接新字符串
func _process(_delta):
    label.text = "HP: " + str(hp) + "/" + str(max_hp)

# 快：少用字符串，或用 StringName
```

### 4. 重复 connect

```gdscript
# 慢：_ready 每帧都连（如果误写在 _process 里）
signal.pressed.connect(_on_pressed)  # 重复连接 = 触发多次

# 快：先检查再连
if not signal.pressed.is_connected(_on_pressed):
    signal.pressed.connect(_on_pressed)
```

## 升级阶梯

> 顺序不可颠倒：

1. **修算法**：O(n²) → O(n log n)，效果最大
2. **加类型**：typed array、类型标注，免 Variant 装箱
3. **换 C#**：热点函数用 C# 写，编译执行
4. **GDExtension**：C++ 原生，最后手段

## 延伸阅读

- 本站：[优化方法论](01-principles.md)、[批量与结构优化](03-bulk.md)、[节点查找的真相](05-node-lookup.md)
- 官方：[GDScript performance](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_best_practices.html)
