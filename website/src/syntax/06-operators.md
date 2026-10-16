# 运算符全表

优先级从高到低的完整表（**所有二元运算符左结合**，含 `**`；三元 `if/else` 右结合）：

| 优先级 | 运算符 | 说明 |
|---|---|---|
| 1 | `( )` | 分组 |
| 2 | `x[i]` `x.attr` `f()` `await x` | 下标 / 成员 / 调用 / 等待 |
| 3 | `x is T` `x is not T` | 类型检查 |
| 4 | `x ** y` | 幂。左结合：`2 ** 2 ** 3 == (2 ** 2) ** 3 == 64`，**与数学直觉相反** |
| 5 | `~x` | 按位取反 |
| 6 | `+x` `-x` | 正负号 |
| 7 | `*` `/` `%` | 乘 / 除 / 余，语义陷阱见下 |
| 8 | `+` `-` | 加 / 减（也用于字符串、数组拼接） |
| 9 | `<<` `>>` | 移位 |
| 10 | `&` | 位与 |
| 11 | `^` | 位异或 |
| 12 | `\|` | 位或 |
| 13 | `==` `!=` `<` `>` `<=` `>=` | 比较 |
| 14 | `in` `not in` | 包含检测 |
| 15 | `not x`（`!x` 别名） | 逻辑非 |
| 16 | `and`（`&&`） | 逻辑与 |
| 17 | `or`（`\|\|`） | 逻辑或 |
| 18 | `a if c else b` | 三元（右结合） |
| 19 | `x as T` | 类型转换 |
| 20 | `=` `+=` `-=` `*=` `/=` `**=` `%=` `&=` `\|=` `^=` `<<=` `>>=` | 赋值（**不能出现在表达式中间**） |

> 值得注意的是：`&&`/`||`/`!` 是合法别名，但[官方风格指南](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_styleguide.html)只用 `and`/`or`/`not`。

## 四个语义陷阱

**1. `/` 对 int 截断**（与 Python/JS 相反，同 C++）：

```gdscript
print(5 / 2)        # 输出 2（int / int）
print(5 / 2.0)      # 输出 2.5
print(5.0 / 2)      # 输出 2.5
```

**2. `%` 只对 int**；float 用 `fmod()`；负数取余跟随被除数（截断语义），数学取模用 `posmod()` / `fposmod()`：

```gdscript
print(-7 % 3)        # 输出 -1（不是 2！）
print(posmod(-7, 3)) # 输出 2
print(fmod(-7.5, 2)) # 输出 -1.5
```

**3. `%` 是双义运算符**：整数语境取模，字符串语境格式化（`"HP: %d" % 100`），见[字符串章](16-strings.md)。

**4. `==` 跨类型比较不总安全**：`1 == 1.0` 为真，但跨不兼容类型可能运行时报错。浮点比较**永远不要用 `==`**：

```gdscript
print(0.1 + 0.2 == 0.3)          # 输出 false
print(is_equal_approx(0.1 + 0.2, 0.3))   # 输出 true
print(is_zero_approx(x))         # 判断 |x| 接近 0
print(is_same(a, b))             # 类型严格的相等（含引用语义）
```

## `in` 的多面性

```gdscript
"ell" in "hello"           # 字符串包含 → true
3 in [1, 2, 3]             # 数组包含
"hp" in player_stats       # 字典键检测（不是值检测！）
body in get_tree().root    # 节点是否在该子树下
```

`in` 用于 `for` 是迭代语法的一部分，见[流程控制](05-flow.md#while--for)。

## 使用准则

- 混合 int/float 的表达式**先转 float 再算**，别依赖截断「顺便」发生。
- 位运算优先级低于乘除——`a & 3 == 1` 实际是 `a & (3 == 1)`，**加括号**：`(a & 3) == 1`。
- 浮点相等判断一律 `is_equal_approx` / `is_zero_approx`。

## 延伸阅读

- 官方英文文档：[GDScript reference · Operators](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 本站：[函数](07-functions.md)（下一步）、[随机与概率](../math/05-random.md)（浮点陷阱的延伸场景）
