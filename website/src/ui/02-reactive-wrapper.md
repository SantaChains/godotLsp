# 响应式封装实现

**用一个继承 Resource 的 Reactive 基类发 changed 信号，owner 链向上传播；typed 子类包标量，数组单独封装。**

## 为什么不能只靠 setter

给普通变量写 setter 并在里面发信号，单个标量可行，但有两个坑：

1. **数组失效**：整个数组替换时 setter 会调用，但 `array.append(x)` 这种原地修改不经过 setter，信号永远不发。
2. **重复代码**：每个字段都手写一个 setter + 一个信号，字段一多就混乱。

解法：抽一个 Reactive 基类，子类只管存值和发信号。

## Reactive 基类与 owner 传播

Resource 自带一个内置 `changed` 信号（自定义资源属性不会自动发，需自己在 setter 里发），直接复用它，不必另起名。

```gdscript
# reactive.gd
class_name Reactive extends Resource

# 复用 Resource 内置的 changed 信号，不重复声明

var _owner: Reactive  # 底层存储，避免在 setter 里自递归

# owner 是父级 Reactive；子级变化时沿链向上冒泡
var owner: Reactive:
    get: return _owner
    set(value: Reactive) -> void:
        if _owner:
            changed.disconnect(_owner._propagate)  # 换父级前断开旧连接
        _owner = value
        if _owner:
            # 子级 changed → 调用父级的传播函数 → 父级也发 changed
            changed.connect(_owner._propagate)

func _propagate() -> void:
    emit_changed()  # 触发内置 changed 信号
```

传播原理：名字字段发 changed → 它的 owner（Hero）收到后调 `_propagate()` → Hero 自己的 changed 也发出去。于是"改了英雄的名字"等价于"英雄本身变了"，监听 Hero 的上层不用关心具体是哪个字段变了。

## 标量子类

```gdscript
# reactive_string.gd
class_name ReactiveString extends Reactive

var _value: String

func _init(initial: String = "", p_owner: Reactive = null) -> void:
    _value = initial
    owner = p_owner  # 初始化时直接挂到父级

var value: String:
    get: return _value
    set(v: String) -> void:
        if _value == v:
            return
        _value = v
        emit_changed()  # 值真的变了才发，避免重复刷新
```

```gdscript
# reactive_int.gd（结构相同，类型换成 int）
class_name ReactiveInt extends Reactive

var _value: int

func _init(initial: int = 0, p_owner: Reactive = null) -> void:
    _value = initial
    owner = p_owner

var value: int:
    get: return _value
    set(v: int) -> void:
        if _value == v:
            return
        _value = v
        emit_changed()
```

> 值得注意的是：GDScript 没有 C++ 模板/泛型类，无法写一个 `Reactive[T]` 包住所有类型，所以按类型各写一个子类，这是语言限制下的常规变通。

## 响应式数组

```gdscript
# reactive_array.gd
class_name ReactiveArray extends Reactive

var _items: Array = []

func _init(p_owner: Reactive = null) -> void:
    owner = p_owner

# 封装所有会改内容的操作，每个操作完成后发信号
func add(item: Variant) -> void:
    _items.append(item)
    emit_changed()

func remove(item: Variant) -> void:
    _items.erase(item)
    emit_changed()

func clear() -> void:
    _items.clear()
    emit_changed()

func get_items() -> Array:
    return _items

func size() -> int:
    return _items.size()
```

## 领域对象 Hero

```gdscript
# hero.gd
class_name Hero extends Reactive

var name: ReactiveString
var strength: ReactiveInt
var wisdom: ReactiveInt

func _init(p_name: String) -> void:
    # 每个字段的 owner 都设为 Hero，变化自动冒泡到 Hero
    name = ReactiveString.new(p_name, self)
    strength = ReactiveInt.new(5, self)
    wisdom = ReactiveInt.new(5, self)
```

## 状态（autoload 单例）

```gdscript
# GameState.gd，注册为 autoload
var available: ReactiveArray
var recruited: ReactiveArray
var selected: Hero = null

func _ready() -> void:
    available = ReactiveArray.new()
    recruited = ReactiveArray.new()
    available.add(Hero.new("剑士"))
    available.add(Hero.new("法师"))
    available.add(Hero.new("游侠"))
```

## 菜单视图接线

```gdscript
# 菜单根 Control 脚本
@onready var name_edit: LineEdit = $Center/NameEdit
@onready var hire_btn: Button = $Center/HireBtn
@onready var fire_btn: Button = $Center/FireBtn
@onready var left_list: VBoxContainer = $Left/Scroll/List
@onready var right_list: VBoxContainer = $Right/Scroll/List

func _ready() -> void:
    # 控件 → 状态：改名
    name_edit.text_changed.connect(func(t: String) -> void:
        if GameState.selected:
            GameState.selected.name.value = t)

    # 按钮 → 状态：雇佣 / 移除，只改数组，不碰另一个列表
    hire_btn.pressed.connect(_on_hire)
    fire_btn.pressed.connect(_on_fire)

    # 状态 → 控件：两个数组一变就整体重建列表（简单做法）
    GameState.available.changed.connect(_rebuild_left)
    GameState.recruited.changed.connect(_rebuild_right)

    # 选中项一变就刷新详情面板
    _rebuild_left()
    _rebuild_right()

func _on_hire() -> void:
    var h := GameState.selected
    if h and GameState.available.get_items().has(h):
        GameState.available.remove(h)
        GameState.recruited.add(h)

func _on_fire() -> void:
    var h := GameState.selected
    if h and GameState.recruited.get_items().has(h):
        GameState.recruited.remove(h)
        GameState.available.add(h)

func _rebuild_left() -> void:
    # 清空容器子节点，按 available 数组重建
    for c in left_list.get_children():
        c.queue_free()
    for h in GameState.available.get_items():
        var row := Button.new()
        row.text = h.name.value
        row.pressed.connect(func() -> void:
            GameState.selected = h
            _show_detail(h))
        left_list.add_child(row)

func _show_detail(h: Hero) -> void:
    name_edit.text = h.name.value
    # 力量/智慧标签同理，全部从状态读
```

## 关键纪律

| 规则 | 原因 |
|---|---|
| 控件之间不直接引用 | 删一个控件不牵连其他 |
| 所有读写经过状态 | 单一数据源，不会出现两处显示不一致 |
| 变化只靠信号通知 | 发送方不认识接收方 |
| 数组用封装方法改 | 原地修改也能触发刷新 |

> 值得注意的是：整体重建列表是最简单的做法；数据量大时可改为只增删变化的那一行，但代价是要自己追踪 diff。菜单级规模重建足够。

## 延伸阅读

- 本站：[UI 范式全景](00-paradigm-survey.md)、[状态与控件分离](01-state-separation.md)、[信号深入](../paradigm/04-signals.md)、[资源 Resource](../paradigm/07-resource.md)
- 官方：[Containers](https://docs.godotengine.org/en/stable/tutorials/ui/gui_containers.html)、[LineEdit](https://docs.godotengine.org/en/stable/classes/class_lineedit.html)
