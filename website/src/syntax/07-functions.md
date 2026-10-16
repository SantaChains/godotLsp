# 函数

**`func` 声明函数；参数与返回值都可标注类型；函数是第一公民——可以被赋值、传递、存储在变量里**（以 `Callable` 形态）。

## 基本形态 {#kw-func}

```gdscript
func add(a: int, b: int) -> int:
    return a + b

func greet(name: String, greeting: String = "Hello") -> void:   # 默认值
    print("%s, %s!" % [greeting, name])

func sum(...nums) -> int:          # 变参（4.2+），nums 是收集剩余实参的无类型 Array
    var t := 0
    for n in nums:
        t += n
    return t
```

> GDScript 的变参收集 `...nums` 是无类型 Array，不能写成 `...nums: int`；需要元素类型就在函数内逐个 `as` 收窄。<a id="kw-void"></a>`return` 从函数返回值，无返回值函数可裸 `return`；`void` 是「无返回值」的类型标注。使用准则：**返回类型一律标注**（不返回值标 `-> void`）——这是免费的编译期检查，也是编辑器补全质量的基石。

<a id="kw-pass"></a>`pass` 是空语句占位：语法上需要语句但不想执行任何代码时使用（空函数体、未写完的分支），与 Python 同名语句语义一致。

## static func：不依赖实例 {#kw-static}

```gdscript
class Math2:
    static func clampi(v: int, lo: int, hi: int) -> int:
        return min(max(v, lo), hi)

Math2.clampi(5, 0, 3)     # 通过类名直接调用，无需实例
```

static 函数内**不能访问实例成员**（没有 `self` 的实例语义），只能访问 static 成员。

## Callable 与 lambda

**`Callable` 是「可调用体」的类型**：方法引用、lambda、绑定了上下文的函数，都是它。

```gdscript
var c: Callable = take_damage       # 方法引用
c.call(30)                          # 调用

var double := func(x: int) -> int:  # lambda（内联匿名函数）
    return x * 2
print(double.call(21))              # 输出 42

var later := double.bind(2)         # bind 预绑参数：调用时补齐
```

lambda **捕获外部局部变量按值拷贝**（捕获瞬间的值），捕获成员变量按引用。这是「回调拿到旧值」类 bug 的根源，见[信号章 · bind](10-signals.md#bind-与参数预绑)。

## super：调用父类实现 {#kw-super}

```gdscript
func _ready() -> void:
    super()                        # 调父类同名方法（此处置 _ready）
    setup_minimap()                # 再做自己的事
```

**覆写引擎生命周期回调时忘记 `super()`** 是经典 bug（父类的 `_ready` 有真实逻辑时）。注：`Node._ready()` 本身空实现，可省；但自定义基类的逻辑不会自动执行。

## 函数即成员的其他形态

```gdscript
func damage(amount: int) -> void:
    hp = maxi(hp - amount, 0)
    if hp <= 0:
        die()
```

函数体省略 `{}`，缩进即体。**无显式 return 的函数返回 `null`**，所以「有返回值标注却在某分支漏 return」会被检查为编译期警告。

## 与协程

函数体中含 `await` 即成为**协程函数**：调用立即返回、在 await 点挂起。这是 GDScript 异步的全部基础，专章见[await 与协程](12-await.md)。

## 延伸阅读

- 官方英文文档：[GDScript reference · Functions](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 本站：[类系统](08-classes.md)（下一步）、[Callable](../builtin/class_callable.md)（类参考锚点）
