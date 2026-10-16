# 字符串格式化与惯用法

**String 不可变**——一切「修改」都返回新串。本页收格式化三套语法、高频方法与 GDScript 惯用法。

## `%` 格式化（Printf 系）

```gdscript
print("HP: %d/%d" % [hp, max_hp])          # 输出 HP: 80/100
print("%s 击杀了 %s" % [attacker, victim])  # %s 任意类型自动转字符串
print("0x%02X" % 255)                       # 输出 0xFF（宽度/进制标志同 printf）
print("%5.2f" % 3.14159)                    # 输出 " 3.14"（宽 5 精度 2）
print("%s=%d" % {"name": 7})                # 字典键占位
```

常用占位符：`%d` 整数、`%f` 浮点、`%s` 字符串（自动 str()）、`%x`/`%X` 十六进制、`%%` 字面百分号。`%` 后数组给多个参数、字典给键名占位——这是 `%` 运算符的字符串义（整数义是取模，见[运算符全表](06-operators.md#四个语义陷阱)）。

## String.format()

```gdscript
"{0} 号玩家获得 {1} 分".format([7, 90])          # 位置占位
"{name}: {score}".format({"name": "Alice", "score": 90})   # 键占位
# 注意：format() 只替换 {索引}/{键}，不支持 Python 式 {:.2f} 精度说明符；
# 小数精度走上面的 % 运算符："%.2f" % 3.14159
```

比 `%` 更可读（占位符有名字），嵌串模板首选；性能与 `%` 同量级。

## 拼接与转换

```gdscript
var full := "res://" + folder + "/" + file      # + 拼接（少量）
var path := "/".join([res_root, folder, file])  # join（多数组元素）
var n := int("42")                              # "42" → 42
var f := float("3.5")                           # 字符串 → 数值
var s := str(3.5)                               # 任意 → 字符串
print("x=%d" % int(f))                          # float→int 显式截断再格式化
```

**性能红线**：`_process` 里每帧拼字符串（尤其 `+=`）会产生大量临时串——调试文本放 `_draw` 或隔帧更新，见[性能篇 · GDScript 高性能实施](../perf/02-gdscript.md)。

## 高频方法精选

```gdscript
"Hello".to_lower()              # "hello"
"  hi  ".strip_edges()          # "hi"
"a,b,c".split(",")              # PackedStringArray ["a","b","c"]
"abc".substr(1, 1)              # "b"
"abc".replace("b", "x")         # "axc"
"abc".contains("bc")            # true（4.x 首选；find() 返回下标 -1 表示无）
"path/to/file.txt".get_file()   # "file.txt"
"path/to".get_basename()        # "path/to/file"（去扩展名）
"42".is_valid_int()             # 解析前校验
"%s".repeat(3)                  # "%s%s%s"
"abc".hash()                    # 整数哈希
```

全表见 [String 类页](../builtin/class_string.md)（与 LSP 补全一致）。

## GDScript 风格要点

- **命名**：`snake_case`（函数/变量），`PascalCase`（类/class_name），`CONST_CASE`（常量）——官方[风格指南](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_styleguide.html)的唯一大写场景是常量。
- **文档注释用 `##`**（见[词法章](02-lexical.md#注释)），编辑器悬停可读。
- 静态字符串比较走 StringName：`if anim_name == &"run"`——驻留指针比较，零分配。

## 延伸阅读

- 官方英文文档：[GDScript style guide](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_styleguide.html)、[String format strings](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_format_strings.html)
- 类参考锚点：[String](../builtin/class_string.md)、[StringName](../builtin/class_stringname.md)
- 本站：语法篇完——下一站 [范式篇 · 三大支柱总览](../paradigm/01-pillars.md)
