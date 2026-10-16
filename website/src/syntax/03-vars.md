# 变量与常量

**`var` 声明变量，`const` 声明常量；类型标注可选，但推荐写全**——4.x 中标注是编译期检查与运行时加速的开关。

## var：成员变量 {#kw-var}

```gdscript
var hp := 100                 # := 推断类型（推断为 int）
var name: String = "hero"     # 显式标注
var inventory: Array[String] = []   # 类型化容器（4.x）
var speed = 300.0             # 无标注：动态类型（Variant），能跑但慢且不检查
```

使用准则：**成员变量一律带类型标注**；局部小变量可用 `:=` 推断；只有接不住类型的动态数据（如 JSON 解析结果）才裸写 `var`。

**声明即初始化**：成员变量初始化在**实例构造时**执行，先后顺序为书写顺序：

```gdscript
var a := 1
var b := a + 1        # 合法：a 先初始化
```

## const：编译期常量 {#kw-const}

```gdscript
const MAX_HP := 100
const NAMES := ["sword", "shield"]    # 容器常量：内容也被冻结（只读）
const GRAVITY := 9.8 * 2.0            # 初始化必须是常量表达式
```

常量在编译期求值，可用于注解参数、case 表达式等要求常量上下文的位置。**给常量标注类型允许但冗余**——推断是精确的。

## enum：具名整数常量组

```gdscript
enum State { IDLE, RUN, JUMP }              # IDLE=0, RUN=1, JUMP=2
enum Error2 { OK = 0, FAILED = -1 }         # 可显式赋值（含负数）
var state := State.IDLE
print(State.keys())      # 输出 [IDLE, RUN, JUMP]
```

enum 本质是 `Dictionary`（名字→值）+ 一组常量。匿名 enum（`enum { A, B }`）把成员直接挂到脚本作用域。详见[类系统 · 枚举](08-classes.md#枚举)。

## 类型推断规则（速查）

| 表达式 | 推断为 |
|---|---|
| `1` | `int` |
| `1.0` | `float` |
| `"s"` | `String` |
| `[1, 2]` | `Array`（**非** `Array[int]`；需要元素类型就显式标注） |
| `{"a": 1}` | `Dictionary` |
| `Node.new()` | 该类实例 |

> 值得注意的是：**GDScript 没有 `int?` 这类可空类型语法**。「可空」就等于不标注（Variant）或标注类类型——对象变量本来就可以是 `null`。`int`/`float`/`bool` 这类值类型标了注就不可能是 `null`，赋 `null` 会运行时报错。

## 作用域

- **成员**：整个脚本可见；跨脚本经实例访问 `player.hp`。
- **局部**：函数 / 块内；**没有块级作用域的越界遮蔽问题**——`if` 里声明的 `var` 在 `if` 结束后不可见，同一函数内先声明后使用。
- **遮蔽（shadowing）**：局部变量与成员同名时，编辑器警告 `SHADOWED_VARIABLE`，不要压。

## 动态类型真相（Variant）

不标注的变量是 **Variant**：运行期携带类型标签的通用容器。代价与收益：

```gdscript
var x = 5          # Variant
var y: int = 5     # 定型
# x 在每次运算时都要查类型标签、可能隐式转换；
# y 在编译期就确定指令，且类型错误在赋值瞬间即报。
```

静态与动态可以混用——**热点数据定型、边界数据留动态**，是 [GDScript 高性能实施](../perf/02-gdscript.md)的基本功。

## 延伸阅读

- 官方英文文档：[Static typing in GDScript](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/static_typing.html)
- 本站：[类型系统](04-types.md)（下一步）、[类系统](08-classes.md)（含静态成员与枚举）
