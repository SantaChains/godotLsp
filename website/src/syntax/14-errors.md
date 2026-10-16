# 错误处理模型

**GDScript 没有异常（exception）**。没有 try/catch，没有调用栈展开。错误分三种通道，各自语义完全不同——用对通道是 GDScript 风格的入门课。

## 三通道总览

| 通道 | 机制 | 会不会停 | 用途 |
|---|---|---|---|
| `Error` 枚举返回 | 函数返回 `int`（`OK=0` 或负错误码） | 不会，调用方负责检查 | API 层可预期失败 |
| `push_error` / `push_warning` | 写入调试器输出（红色/黄色） | 不会 | 不可恢复问题的记录 |
| `assert` | 条件失败时中断（**仅 debug**） | debug 中断，release 不存在 | 开发期不变量 |
| 致命错误 | 引擎内部检查（如数组越界） | **脚本崩溃**（输出栈后停函数） | —— |

## Error 枚举：可预期失败的正道

两类失败面，判法不同：

```gdscript
# ① 返回对象的工厂函数：判 null（错误码要另取）
var file := FileAccess.open("res://save.json", FileAccess.READ)
if file == null:
    push_warning("存档缺失：%s" % error_string(FileAccess.get_open_error()))
    return

# ② 声明返回 Error 的 API：判 != OK
var connect_err := btn.pressed.connect(_on_press)
if connect_err != OK:
    push_error("连接失败：%s" % error_string(connect_err))
```

使用准则：**`!= OK` 检查只用于文档声明返回 `Error` 的 API**（`connect`、`bind`、`play` 等）；返回对象的工厂函数判 `null`。`error_string(err)` 把码翻成可读文本。`Error` 全枚举见 [@GlobalScope 类页](../builtin/class_globalscope.md)。

## push_error / push_warning

```gdscript
push_error("无法解析关卡数据：%s" % path)     # 红
push_warning("旧版存档格式，已自动迁移")       # 黄
```

输出进调试器 + 远程日志，**不中断执行**。规则：错误信息必须**可行动**——带上路径、参数名、期望值；`push_error("failed")` 这种等于没写。

## assert：开发期不变量

```gdscript
assert(slot_count > 0, "slot_count 必须为正，检查场景配置")
```

- **release 构建里整个调用被剥除**——里面的表达式都不会执行（有副作用的 assert 是 bug）。
- 失败时 debug 构建中断在断言处。
- 定位：校验「不可能发生的事」；校验玩家输入/外部数据用 `push_error` + 早返回。

## 防御式写法惯例

```gdscript
func load_profile(path: String) -> Dictionary:
    if not FileAccess.file_exists(path):          # 1. 前置校验，早返回
        return {}
    var file := FileAccess.open(path, FileAccess.READ)
    if file == null:                              # 2. 逐层判空
        return {}
    var data: Variant = JSON.parse_string(file.get_as_text())
    if data is Dictionary:                        # 3. 类型窄化
        return data
    return {}
```

要点：**早返回优于深嵌套**；**判空在解引用之前**（`node.free()` 前先 `is_instance_valid(node)`）；**引擎崩 ≠ 事务回滚**——崩溃前已执行的副作用不会撤销，关键操作要设计幂等。

## 与引擎崩溃的关系

类型不匹配、null 解引用、数组越界在 GDScript 里是**脚本级致命错误**：打印堆栈、中断当前函数，游戏继续跑（不整个崩溃）。这不同于 C++ 段错误。运行期错误清单与引擎如何裁决它们，见[引擎内部篇 · 脚本致命面](../internals/07-fatal.md)。

## 延伸阅读

- 官方英文文档：[GDScript reference · Assert](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)、[@GlobalScope · Error 枚举](https://docs.godotengine.org/en/stable/classes/class_%40globalscope.html)
- 类参考锚点：[@GlobalScope · 全局枚举 Error](../builtin/class_globalscope.md)
- 本站：[输入系统](15-input.md)（下一步）、[性能篇 · 优化方法论](../perf/01-principles.md)（错误与日志的性能代价）
