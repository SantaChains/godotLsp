# 场景文件 .tscn 解析

`.tscn` 是**文本场景格式**：节点树、引用的脚本与资源、信号连接，全部以声明式标签存盘。引擎加载一个场景 = 解析这些标签 → 按声明顺序线性实例化。本章拆解文件里的每个标签、实例化的顺序约束、以及 `owner` 与 `%Name` 唯一名的底层语义——三者决定了「什么样的 tscn 是合法的」「什么样的 tscn 会静默出错」。

## 文件结构：五种标签

```ini
[gd_scene load_steps=3 format=3 uid="uid://cecaux1sm7mo0"]

[ext_resource type="Script" path="res://player.gd" id="1_abcd"]

[sub_resource type="RectangleShape2D" id="RectangleShape2D_1"]
size = Vector2(16, 16)

[node name="Player" type="CharacterBody2D"]
script = ExtResource("1_abcd")

[node name="Sprite" type="Sprite2D" parent="."]

[connection signal="hit" from="." to="Sprite" method="_on_hit"]
```

| 标签 | 作用 | 缺失/非法的致命面 |
|---|---|---|
| `[gd_scene]` / `[gd_resource]` | 文件头，声明格式版本、资源总数、场景自身 uid | 首个标签不是它 → 🔴 `Unrecognized file type`；`format` 大于引擎支持 → 🔴 `Saved with newer format version`（缺 format 字段则按最新版处理）；`load_steps` 与实际不符 → 🟡 **仍能正确加载**，但进度条等依赖该值的 UI 显示失真 |
| `[ext_resource]` | 引用外部文件（脚本/贴图/场景） | 缺 `id`/`type`/`path` 任一 → 🔴 `ERR_FILE_CORRUPT`；目标文件不存在 → 🔴 拒载（编辑器默认）；运行时关掉 abort 开关则 🟡 继续但依赖缺失 |
| `[sub_resource]` | 文件内嵌资源（形状/材质等） | 缺 `type` 或 `id` → 🔴 `ERR_FILE_CORRUPT` |
| `[node]` | 节点声明 | 见下文字段表 |
| `[connection]` | 信号连接声明 | 见[信号连接的存储](04-connections.md) |

`load_steps` 的官方定义：**内外部资源总数 + 1**（文件自身）；文件没有资源时省略。它是给加载进度显示用的提示值，写错不影响正确性——但编辑器保存时总是写对，手写文件建议同样对齐。

另外，`.tscn`/`.tres` 与 `project.godot`、`.import` 共用同一套解析内核（`VariantParser` 的 tag/assign 状态机）——所以[Variant 序列化](05-variant.md)的转义与构造串规则在所有这些文件里通用。

## `[node]` 字段表

| 字段 | 必填 | 语义 | 缺失/非法后果 |
|---|---|---|---|
| `name` | 是 | 节点名 | 缺失按空名注册，同名兄弟会被编辑器改名（🟡） |
| `parent` | 首节点**禁用**，其余必填 | NodePath，**相对场景根** | 首节点带 parent → 🔴 `Invalid scene: root node X cannot specify a parent node`；其余节点缺失 → 🔴 `Invalid scene: node X does not specify its parent node` |
| `type` | 与 `instance` 二选一 | C++ 类名（经 ClassDB 查找） | 类型不存在 → 🟡 WARN 后按父节点类型降级建占位：Control/Node2D/Node3D 分支，兜底 Node |
| `instance` | 与 `type` 二选一 | ExtResource id（指向 PackedScene） | 实例化失败 → 🔴 `Scene instance is missing.`（运行时；编辑器有 MissingNode 占位兜底） |
| `owner` | 否 | NodePath | **缺省时非根节点自动 owner = 根**，几乎不需要手写 |
| `index` | 否 | 兄弟排序下标 | 超界忽略 |
| `groups` | 否 | 字符串数组 | 无致命面 |

## 实例化顺序：两条铁律

引擎按**声明顺序**逐个 `[node]` 线性实例化，由此产生两条硬约束：

1. **第一个 `[node]` 必须是无 `parent` 的根节点**；根之后每个节点必须有 `parent`。
2. **`parent` 只能指向在它之前声明的节点**。实例化是线性的，声明在后的节点尚不存在，前向引用直接解析失败。这与 `[connection]` 正好相反——连接是在所有节点建好**之后**统一执行的，可以引用后声明的节点。

`parent` 路径的解析规则（源码对 parent 值做 `prepend_period()`，锚点 `resource_format_text.cpp`）：

- `parent="."` = 根的直接子节点；`parent="Player/Hand"` = 根下 `Player` 的子节点。**根节点名不出现在任何 parent 路径里**。
- 路径段按节点**名字**匹配（非类型、非 uid）；`..` 上溯合法；**`%Name` 唯一名在 parent 中无效**——场景文件内部路径是索引化存储，不走运行时查名。
- 实例化时 parent 失效（中间节点被删）：DEBUG 构建 WARN + 挂根；release 构建 → 该节点进 `stray_instances` 被 **memdelete 静默删除**（⚪ 节点直接消失，无报错）。

## owner 与 `%Name` 唯一名

`$`、`get_node`、信号 `from`/`to` 的共同底座是 `Node::get_node_or_null`（锚点 `node.cpp`）：

- 相对路径从**当前节点**开始；`/` 开头的绝对路径从场景树根开始，且**不在树内的节点用绝对路径直接返回 null**。
- `.` = 自身，`..` = 父节点（根的 `..` → null）；每段在子节点列表里按名字线性比对，失配即整条路径 null（这也是热循环反复 `$` 查找慢的原因，见[节点查找的真相](../perf/05-node-lookup.md)）。
- `%Name` 唯一名：名字带 `%` 前缀时，先查当前节点的 `owned_unique_nodes` 表，查不到再查**当前节点 owner** 的表。

`%Name` 的注册规则决定了它的两个经典陷阱：

```gdscript
# 节点必须满足 unique_name_in_owner = true（编辑器里右键 -> Access as Unique Name）
# 注册发生在「设置 owner 的那一刻」：
#   - owner 为空的孤儿节点，%Name 永远查不到（注册前置校验 owner 非空）
#   - 同一场景两个节点同 % 名：先注册者赢，后者被取消 unique 标记并 WARN
#     → %Name 解析到旧节点，静默错绑（🟡）
```

> 需要注意的是：`%` 只能出现在路径**段首**（紧跟 `$` 或 `/` 之后），如 `$Path/%Name`、`$%Name`、`%Name`（当前节点作用域）。路径中段的 `%` 是取模运算——`A%B` 不是节点路径。

`$` 路径在**脚本编译期不做存在性检查**：`$Foo` 指向不存在的节点，写脚本时不报错，运行时才炸 `Node not found`。这就是 LSP 在编辑期解析场景树做静态检查的价值所在（见[脚本致命面](07-fatal.md)）。

## 延伸阅读

- 官方英文文档：[Node](https://docs.godotengine.org/en/stable/classes/class_node.html)、[SceneTree](https://docs.godotengine.org/en/stable/classes/class_scenetree.html)
- 本站：[场景树与节点引用](../syntax/11-scenetree.md)（`$`/`%`/owner 的使用层）、[信号连接的存储](04-connections.md)、[工程启动链路](01-boot.md)
- 类参考：[Node](../builtin/class_node.md)、[PackedScene](../builtin/class_packedscene.md)
