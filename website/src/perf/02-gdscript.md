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
# 慢：每帧走一遍路径解析 + 子节点线性查找
@onready var sprite := $Sprite2D

func _physics_process(delta: float) -> void:
    sprite.position.x += speed * delta   # 缓存后直接访问成员变量
```

节点引用在 `_ready` 用 `@onready` 取一次存起来，不要在 `_process`/`_physics_process` 里反复写 `$Node` 或 `get_node()`，原理见[节点查找的真相](05-node-lookup.md)。

### 3. 每帧拼接字符串

```gdscript
# 慢：每帧新建字符串对象，哪怕数值没变
func _process(_delta: float) -> void:
    label.text = "HP: " + str(hp) + "/" + str(max_hp)

# 快：数值变化时才刷新，避免无谓分配
func _on_hp_changed(new_hp: int) -> void:
    label.text = "HP: %d/%d" % [new_hp, max_hp]
```

`StringName` 用于信号名、属性名这类**重复查找键**的去重，不是用来拼接显示文本，别用错场景。

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
