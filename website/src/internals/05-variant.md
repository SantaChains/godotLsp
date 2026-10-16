# Variant 序列化

编辑器 Inspector 里填一个 `Transform2D`，存盘后 `.tscn` 里是一串 `Transform2D(1, 0, 0, 1, 100, 50)`；填一个带换行的文本，文件里换行**原样保留**。属性值能这样写进文本文件，靠的是 **Variant 文本序列化**：每种 Variant 类型都有确定的构造串语法，字符串有一套转义规则。本章给出双向规则——为什么你在编辑器里填的值和文件里长的不一样，以及手写文件时怎样保证解析端读得回去。

## 字符串转义

tscn/tres 的解析端（`core/variant/variant_parser.cpp::get_token`）接受以下转义序列（`\` 后跟）：

| 序列 | 结果 | 序列 | 结果 |
|---|---|---|---|
| `\b` | 退格 | `\n` | 换行 |
| `\t` | 制表 | `\f` | 换页 |
| `\r` | 回车 | `\uXXXX` | 4 位十六进制码点 |
| `\"` / `\\` | 字面 `"` / `\` | `\UXXXXXX` | 6 位十六进制码点 |

三条关键语义：

- **未知转义宽容处理**：`\x` 解析为字面 `x`，不报错。
- **`\u` 代理对**：高位代理 + 低位代理自动合并成完整码点；不配对 → 🔴 解析错误。
- **未闭合字符串（EOF 前没等到 `"`）→ 🔴 解析错误，整个文件拒载**。字符串内的**原始换行合法**——解析是逐字符进行的，只有 `"` 能终止字符串，引擎自己保存时也这么写。

**写出端只需两条规则**（引擎 `c_escape_multiline` 同此精神）：

```text
s.replace("\\", "\\\\").replace("\"", "\\\"")
```

换行原样保留不转义，其余字符原样。解析端宽容 + 写出端保守，双向闭环成立。

## 常用类型的构造串

| 类型 | 写法示例 |
|---|---|
| int / float | `5` / `1.5`（float 可带 `e` 指数；`inf`/`nan` 有特殊记号） |
| bool | `true` / `false` |
| String | `"..."`（按上节转义） |
| StringName | `&"..."` |
| NodePath | `NodePath("Player/Hand")` |
| Vector2 / Vector3 | `Vector2(1, 2)` |
| Color | `Color(1, 0, 0, 1)` |
| Rect2 | `Rect2(x, y, w, h)` |
| Transform2D | `Transform2D(6 个数，列主序)` |
| Transform3D | `Transform3D(12 个数，Basis 9 + origin 3)` |
| Array | `[1, "a", true]` |
| Dictionary | `{"k": 1, "other": 2}` |
| Packed*Array | `PackedInt32Array(1, 2)` / `PackedFloat32Array(...)` / `PackedStringArray(...)` |
| 外部资源引用 | `ExtResource("1_abcd")` |
| 内嵌资源引用 | `SubResource("RectangleShape2D_1")` |

两个书写守则：

1. **float 保留小数点**：写 `1.0` 不写 `1`——整数写法与浮点字段有类型歧义，`rtos_fix` 输出规则总是带小数。
2. **Transform2D 是列主序**：`Transform2D(1, 0, 0, 1, 100, 50)` 前 4 个数是基（每列一个单位向量），后 2 个是原点。对着 Inspector 面板逐格抄容易抄错方向。

> 值得注意的是：节点属性里的 NodePath 值（`node_paths` 声明过的）在实例化期**延迟解析**——先按字符串存，等所有节点建好后再 `get_node` 转成真节点；目标不存在则置 null（⚪ 无报错）。这与脚本里的 `$` 路径一样，都是「树建好才能查」的约束。

## 延伸阅读

- 官方英文文档：[Variant](https://docs.godotengine.org/en/stable/classes/class_variant.html)、[Godot file formats](https://docs.godotengine.org/en/stable/contributing/development/file_formats.html)
- 本站：[词法基础](../syntax/02-lexical.md)（GDScript 层的字面量与字符串转义）、[类型系统](../syntax/04-types.md)（Variant 动态类型的使用层视角）
- 类参考：[String](../builtin/class_string.md)、[NodePath](../builtin/class_nodepath.md)
