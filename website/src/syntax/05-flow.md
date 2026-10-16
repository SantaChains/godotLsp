# 流程控制

一篇讲完 GDScript 全部分支与循环构造。**没有 `switch`、没有 `do-while`、没有 `goto`**——分支靠 `if` 与 `match`，循环靠 `while` 与 `for`。

## if / elif / else {#kw-if}

```gdscript
if hp <= 0:
    die()
elif hp < 30:            # 可多个 elif
    retreat()
else:
    attack()
```

<a id="kw-elif"></a><a id="kw-else"></a>条件必须是 `bool`——**`if 1:` 之类非布尔条件是编译错误**（与 Python 不同）。`elif` 是"否则如果"链式分支；`else` 兜住所有未命中情况，两者都必须跟在 `if`（或 `elif`）之后。

## match：模式匹配 {#kw-match}

`match` 远强于 switch：支持常量、变量绑定、数组/字典解构与多重模式。

```gdscript
match event.type:
    InputEvent.KEY:            # 常量模式（枚举值）
        handle_key(event)
    1, 2, 3:                   # 多值模式
        pass
    var x when x > 100:        # 绑定 + 守卫（4.4+）
        print("big: ", x)
    [var a, var b]:            # 数组解构（恰好两个元素）
        print(a + b)
    {"name": var n}:           # 字典解构（键存在即匹配）
        print(n)
    _:                         # 通配（相当于 else）
        pass
```

<a id="kw-when"></a>`when` 是 match 分支守卫（4.4+）：`var x when x > 100` 先解构绑定再验条件，条件为假则继续向下匹配。

匹配语义：**从上到下首个命中即止**；`_` 分支可省略——全部未命中则整个 match 空过。

## while / for {#kw-while}

```gdscript
while lives > 0:
    play_round()
    if quit_requested:
        break                  # 跳出当前循环
    if round_invalid:
        continue               # 直接进入下一轮

for i in range(3):             # 0,1,2
    pass
for i in range(2, 10, 2):      # 2,4,6,8（start, end, step）
    pass
for item in inventory:         # 遍历 Array / String 字符 / Dictionary 的键
    print(item)
for k in dict:                 # 遍历键；值用 dict.values()，键用 dict.keys()
    pass
```

**`for` 的 in 右侧**：`Range`（`range()` 返回惰性区间）、Array、Dictionary（遍历键）、String（遍历字符）、int（4.2+：`for i in 5` 等价 `range(5)`）。

<a id="kw-for"></a><a id="kw-break"></a><a id="kw-continue"></a>`for` 是唯一推荐的确定次数遍历写法；`break` 立即跳出**最内层**循环，`continue` 跳过剩余体直接进入下一轮——两者都只作用于所在循环，不跨层。

没有 `do-while`；需要先执行后判断就 `while true:` + 内部 `break`。

## 三元与 assert

```gdscript
var sign_str := "正" if hp > 0 else "负"     # a if cond else b（右结合）
assert(hp >= 0, "hp 不应为负")               # 仅 debug 构建生效，release 被剥除
```

`assert` 的第二个参数是错误信息；**它不是输入校验工具**——release 构建里整个调用消失。运行期校验用 `push_error()` + 早返回，见[错误处理模型](14-errors.md)。

## 使用准则

- 分支 ≥ 3 且按同一变量取值 → 用 `match`；混合条件 → `if`。
- 循环体里改被遍历容器是未定义行为（跳元素/重复元素），需要改就先复制键列表。
- 嵌套超过两层考虑提取函数；`match` 里塞 50 行逻辑用函数替代。

## 延伸阅读

- 官方英文文档：[GDScript reference · Logic / Loops / Match](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)
- 本站：[运算符全表](06-operators.md)（下一步）、[await 与协程](12-await.md)（异步分支）
