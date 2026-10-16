# 类系统

**一个 `.gd` 文件就是一个类**；文件级 `extends` 决定它挂在什么上，`class_name` 决定它在全局是否可见。GDScript 没有「裸函数脚本」——即使不写任何声明，脚本也默认继承 `RefCounted`。

## extends 与实例化 {#kw-extends}

```gdscript
# player.gd
extends CharacterBody2D        # 挂在 CharacterBody2D 节点上
```

```gdscript
# 纯数据/工具类：不挂节点
extends RefCounted             # 或 @abstract / class_name 二选一起步

const GRAVITY := 980.0
```

不写 `extends` 的默认基类是 **`RefCounted`**（自动引用计数、离开作用域自动释放）。**要挂到节点上就必须显式 `extends Node` 谱系**——这也是 [autoload](13-autoload.md) 注册失败的静默坑。

实例化有两条路：

```gdscript
var obj := MyClass.new()       # RefCounted 谱系：纯代码构造
var node := Node2D.new()       # Object/Node 谱系：new 后还需 add_child 才进场景树
```

## _init 与构造

```gdscript
class Item:
    var price: int
    func _init(p: int = 10) -> void:    # 构造函数，new 时调用
        price = p

var sword := Item.new(50)                # 参数传给 _init
```

`_init` 在**实例存在之前**的构造阶段运行（节点还没进树）；需要等树就绪的初始化用 `_ready`，见[场景树章 · 生命周期](11-scenetree.md#生命周期回调全表)。

## class_name：全局注册 {#kw-class_name}

```gdscript
class_name Player
extends CharacterBody2D
```

- 注册后 `Player` 成为**全局类型名**，任何脚本可直接 `is Player`、`Player.new()`。
- 编辑器「创建节点」对话框也会出现它（作为附加脚本的快捷方式）。
- `class_name` 与 `extends` 可同行：`class_name Player extends CharacterBody2D`。

> 值得注意的是：`class_name` **不产生命名空间**。注册名与内建类冲突会报错；全工程唯一。

## 内部类 {#kw-class}

```gdscript
# 顶层类（本文件即 Player）
class Inventory:                  # 内部类：常量偏移表之类的小型附属结构
    var slots: Array[String] = []
    func add(item: String) -> void:
        slots.append(item)

var inv := Inventory.new()        # 用类名直接构造
```

<a id="kw-preload"></a><a id="kw-self"></a>小类不跨文件复用就写内部类，省一次 `preload()`/`class_name` 注册；内部类方法里的 `self` 指内部类实例而非外层脚本。

内部类不进全局注册表，**仅在本脚本作用域可见**（外部访问要经 `Player.Inventory`）。

## 属性 setter / getter

```gdscript
# 注意：setter 里给同名变量赋值会无限递归，必须用底层字段 _hp
var _hp := 100:
    set(v):
        _hp = clampi(v, 0, max_hp)   # 写底层字段，不回进 setter
        if _hp == 0:
            die()
    get:
        return _hp                    # 对外仍以 hp 语义暴露，可另起公开名

# 简写：只有 setter 时
var seen := false:
    set(v):
        seen = v
        queue_redraw()
```

在 setter 里给**同一个变量**赋值会再次进入 setter 直到栈溢出——要拦截就用 `_hp` 这类 backing field 分离存储与对外属性，见[官方文档 · Properties](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)。

## static var：类级状态（4.1+）

```gdscript
class_name GameState

static var instance: GameState          # 全局唯一状态载体
static var play_count := 0
```

static 成员属于**类本身**，全工程共享。这是 4.x 实现单例的纯语言手段（与引擎级 [autoload](13-autoload.md) 互补）。

## 枚举 {#kw-enum}

```gdscript
enum State { IDLE, RUN, JUMP }          # 挂在类上：State.IDLE
enum { ATTACK, DEFEND }                 # 匿名：ATTACK 直接可用
```

## 继承的真实语义

```gdscript
# enemy.gd
class_name Enemy
extends CharacterBody2D
func take_damage(amount: int) -> void: ...
```

```gdscript
# boss.gd
extends Enemy                           # extends 类名（class_name 注册的）
func take_damage(amount: int) -> void:
    super(amount / 2)                   # 半伤并复用父逻辑
    rage += 1
```

`is` 沿继承链向上匹配（`boss is CharacterBody2D` 为真）；`super` 只向上找一层同名方法。**深继承链在游戏逻辑里通常是设计异味**，为什么与替代方案见[组合优于继承](../paradigm/09-composition.md)。

## 延伸阅读

- 官方英文文档：[GDScript reference · Classes](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 本站：[注解全表](09-annotations.md)（下一步）、[RefCounted](../builtin/class_refcounted.md)、[Object](../builtin/class_object.md)
