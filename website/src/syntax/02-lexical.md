# 词法基础

一篇讲完 GDScript 的「字符层」规则：标识符、关键字、注释、缩进、续行与字面量。这些是编译器的第一道关，错了连解析都过不去。

## 标识符与关键字

**标识符**：字母 / `_` 开头，后接字母、数字、`_`；区分大小写；允许 Unicode 字母，但实践上坚持 ASCII。

**关键字是保留字**，不能做标识符。全表（依据 GDScript tokenizer）：

| 组 | 关键字 |
|---|---|
| 控制流 | `if` `elif` `else` `for` `while` `match` `when`（match 守卫，4.4+） `break` `continue` `pass` `return` |
| 声明 | `var` `const` `func` `signal` `enum` `class` `class_name` `extends` `static` `void` |
| 类型与转换 | `is` `as` `in` `self` `super` `breakpoint` `preload` `await` `yield`（仅存留兼容） `assert` |

内建类型名（`int`、`Vector2`……）与字面常量（`PI` `TAU` `INF` `NAN` `true` `false` `null`）同样保留。

> 值得注意的是：`PI`/`INF` 等在 3.x 里是 `@GDScript` 的常量成员，4.x 仍是保留字形式直接可用；自定义常量不要与之重名。

## 注释

```gdscript
# 行注释：到行尾
# TODO: 这里的 TODO 默认会在编辑器中高亮为黄色

## 文档注释：紧贴声明的 ## 会被收集进脚本文档
## 并显示在编辑器悬停提示里（配合 BBCode）：
func damage(amount: int) -> void:
    pass
```

**代码折叠区**（`#` 与 `region` 之间不能有空格）：

```gdscript
#region 状态机
# ...大段逻辑...
#endregion
```

## 缩进即语法

代码块用**缩进 + 制表符或空格**划分，同一文件必须统一（Godot 编辑器默认 Tab）。冒号是块的开端：

```gdscript
func f(x: int) -> void:
    if x > 0:            # 冒号开始块
        print(x)
    print("done")        # 缩进回退 = 块结束
```

> 需要注意的是：混合 Tab 与空格会直接报解析错误。团队协作建议在编辑器设置里锁定一种。

## 换行与续行

语句默认一行一条，**分号可选**（风格指南建议不用）。长表达式用反斜杠或括号内换行：

```gdscript
var total := a \
    + b \
    + c

var items := [          # 括号内可自由换行，无需反斜杠
    "sword", "shield",
]
```

## 字面量全表

| 类别 | 写法 | 备注 |
|---|---|---|
| int | `42` `-7` `1_000_000` | `_` 分隔符仅增可读性 |
| int 其他进制 | `0x1F` `0b1010` | 十六进制 / 二进制 |
| float | `3.14` `2e6` `58.1e-10` | 双精度（64 位） |
| bool | `true` `false` | |
| null | `null` | 「无引用」的空值 |
| String | `"hi"` `'hi'` `"a\nb"` | 双/单引号等价，转义序列同 C |
| String 原始 | `r"C:\path"` | `r` 前缀，转义不生效 |
| StringName | `&"move_left"` | 比较与字典键更快，见[类型系统](04-types.md) |
| NodePath | `^"Player/Sprite"` | 场景树路径专用，见[场景树章](11-scenetree.md) |
| 数组 | `[1, 2, 3]` | 见[类型系统](04-types.md) |
| 字典 | `{"hp": 100}` | 4.x 起支持 Lua 风格 `{hp = 100}` |

## 使用准则

- 字符串字面量优先双引号（官方风格指南，避免与 `'` 撇号文本冲突）。
- 节点/输入动作标识频繁比较时用 `&"..."` StringName。
- 大整数一律写 `_` 分隔符：`1_000_000` 比 `1000000` 不易数错位。

## 延伸阅读

- 官方英文文档：[GDScript reference · Identifiers/Keywords/Literals](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 本站：[变量与常量](03-vars.md)（下一步）、[运算符全表](06-operators.md)
