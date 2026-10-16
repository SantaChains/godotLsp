# 信号连接的存储

编辑器 Signals 面板里拖出的一条连接，落盘后是 `.tscn` 里的一行 `[connection]` 声明。**连接是数据，不是代码**——引擎加载场景时读这些声明，在实例化完成后统一执行 `connect()`。本章拆解声明格式、实例化期的执行规则，以及它最容易踩的两个坑。

## `[connection]` 四字段

```ini
[connection signal="pressed" from="UI/StartButton" to="." method="_on_start_pressed"]
```

| 字段 | 必填 | 语义 |
|---|---|---|
| `signal` | **缺失 → 🔴 `ERR_FILE_CORRUPT`** | 发射的信号名 |
| `method` | **同上** | 目标方法名（接收者脚本内） |
| `from` | **同上** | 发射者 NodePath，相对场景根（存盘时 `simplified()` 去冗余段） |
| `to` | **同上** | 接收者 NodePath，同上 |
| `flags` | 否 | 默认 `CONNECT_PERSIST`；一般不写此字段 |
| `binds`/`unbinds` | 否 | 预绑参数个数；手工编辑基本用不到 |

> 需要注意的是：节点级字段缺失（如 `type` 不存在）走 WARN + 降级占位，而 connection 四字段缺失是**硬拒绝、整个场景拒载**。手改 tscn 删半行连接，代价是整张场景打不开。

## 实例化期执行规则

源码锚点 `packed_scene.cpp` 场景实例化段。连接的执行发生在**所有节点实例化完成之后**：

1. `from`/`to` 各自按[场景解析](02-tscn.md)的路径规则解析为节点。
2. **任一端节点找不到 → `continue`，这条连接被静默丢弃**（⚪ 信号永不触发且无任何报错）。
3. 端点存在但 `signal` 名在发射者类型上不存在、或 `method` 不在接收者脚本里 → `connect()` 返回运行时错误并打印（🟡 日志里可见 `Signal ... not found` / `Method expected`），其余连接不受影响。
4. 连接执行时脚本已全部挂好——所以 tscn 里 `[connection]` 可以引用文件中**后声明**的节点（与 `parent` 的前向引用禁令恰好相反）。

## 与代码连接的关系

```gdscript
# 编辑器连接落盘为 [connection]，运行时由引擎替你执行这句：
start_button.pressed.connect(_on_start_pressed)
```

两者语义完全等价，规则也一样：**同一对 (from, signal, to, method) 重复连接不会去重**——编辑器连一次、代码里再连一次，回调触发两次（⚪ 无报错）。同一对连接请只选一处。

书写与校验要点：

- `signal`/`method` 必须**逐字符匹配**源码里的 `signal X` 声明与 `func X` 定义——GDScript 大小写敏感，改名脚本后 tscn 不会自动跟着改（端点还在，`method` 名对不上 → 🟡 运行时报错；节点删了 → ⚪ 静默丢弃）。
- `from`/`to` 写相对根的最短路径，或 `%Name` 唯一名（运行时经 `get_node` 解析，合法）。
- 静态验证这类错误正是 LSP 的主场：信号存在性、方法存在性、路径有效性都能在编辑期查出，见[脚本致命面](07-fatal.md)。

## 延伸阅读

- 官方英文文档：[Using signals](https://docs.godotengine.org/en/stable/getting_started/step_by_step/signals.html)、[PackedScene](https://docs.godotengine.org/en/stable/classes/class_packedscene.html)
- 本站：[信号系统](../syntax/10-signals.md)（使用层：connect/emit/bind）、[场景文件 .tscn 解析](02-tscn.md)（节点实例化顺序）
- 类参考：[Node](../builtin/class_node.md)（connect 与连接标志定义处）
