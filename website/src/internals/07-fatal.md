# 脚本致命面与静默失败

前六章零散出现的失败项，本章汇总成一张完整地图。核心认知：**引擎的报错密度是刻意设计的**——只有「继续跑会损坏数据」的错误才拒载，大量逻辑错误被降级甚至静默。写错的东西不一定会报错，这正是需要静态检查工具的原因。

## 三级失败标记

| 级别 | 含义 | 表现 |
|---|---|---|
| 🔴 拒载 | 引擎返回错误码，场景或工程加载失败 | 弹窗 / stderr 报错，直接不能用 |
| 🟡 降级 | WARN + 占位替换或回退 | 能跑，但功能受损，日志里有线索 |
| ⚪ 静默 | 无任何报错 | 结果悄悄不对，最难排查 |

## 拒载清单（🔴）

| 触发点 | 规则 | 出处 |
|---|---|---|
| project.godot | `config_version` 大于引擎版本、任何语法错误 | [工程启动链路](01-boot.md) |
| 主场景 | 未设置 `main_scene` 且无启动参数 | 同上 |
| tscn 文件头 | 首标签非 `gd_scene`/`gd_resource`、`format` 超引擎版本 | [场景解析](02-tscn.md) |
| tscn 结构 | 根节点带 `parent`；非根节点缺 `parent`；`parent` 前向引用 | 同上 |
| 资源引用 | `[ext_resource]` 缺 `id`/`type`/`path`；`[sub_resource]` 缺 `type`/`id` | 同上 |
| 信号连接 | `[connection]` 缺 `signal`/`method`/`from`/`to` 任一字段 | [信号连接](04-connections.md) |
| 字符串值 | 引号未闭合（EOF 前无 `"`） | [Variant 序列化](05-variant.md) |

## 降级清单（🟡）

| 触发点 | 行为 | 出处 |
|---|---|---|
| `[node] type` 不存在 | WARN + 占位降级：按父类型取 Control/Node2D/Node3D，兜底 Node | 场景解析 |
| autoload 脚本不继承 Node / 路径无效 | 整个单例被跳过，全局常量缺失，下游报 `Identifier not found` 假错误 | 工程启动 |
| `config/features` 含未知项 | 项目管理器警告，不拒开 | 工程启动 |
| `load_steps` 与实际资源数不符 | 仍能正确加载，进度条等 UI 失真 | 场景解析 |
| `ext_resource` uid 无效 | WARN，回退按 `path` 加载 | UID 系统 |
| `%Name` 重名 | 先注册者赢，后者被取消 unique 标记并 WARN | 场景解析 |
| `signal`/`method` 名对不上 | `connect()` 运行时返回错误并打印，连接失败但其余继续 | 信号连接 |
| 同名兄弟节点 | 编辑器改名修复；运行时并存 | 场景解析 |
| `load()` 路径不存在 | 返回 null，不报错 | 资产管线 |

## 静默清单（⚪）

| 触发点 | 行为 | 出处 |
|---|---|---|
| `project.godot` 键名拼写错误 | 未知键静默保留，不报错不生效 | 工程启动 |
| `parent` 路径失效（release 构建） | 节点进 `stray_instances` 被 `memdelete` 删除——节点直接消失 | 场景解析 |
| `[connection]` 端点节点丢失 | `continue` 跳过，连接静默丢弃，信号永不触发 | 信号连接 |
| 同一对连接重复建立 | 回调触发两次，无去重无报错 | 信号连接 |
| 节点属性 NodePath 延迟解析失败 | 置 null，无报错 | Variant 序列化 |
| 同名子节点并存 | 按名查找永远命中第一个，另一个成了幽灵节点 | 场景解析 |
| 孤儿节点的 `%Name` | 注册前置校验 owner 非空，永远查不到 | 场景解析 |

## 脚本运行期特有面

- **`class_name` 依赖导入缓存**：全局类注册在 `--import` 扫描时建立，新写 `class_name` 后引擎的类缓存不刷新，其他脚本报 `Identifier not found`——先跑一次导入再验证。
- **`@onready` 的求值时机**：在 `_ready` 前一刻执行，`$` 路径**此时**才解析。tscn 层级改了、脚本没跟上，编辑期一切正常，运行时 `Node not found`。
- **`extends` 失效**：指向不存在的脚本或不继承预期基类 → 脚本解析失败，节点脚本空挂，节点本身照常生成（🟡）。

## 为什么 LSP 要做这些检查

上表的共同模式：**引擎把大量错误推迟到运行时，甚至吞掉**。LSP 的静态诊断本质上是把这张表提前到编辑期执行——这是 godotlsp 诊断能力的说明书：

- 节点树解析：静态验证 `$`/`%` 路径指向的节点存在、`%Name` 全场景唯一——把「运行时 `Node not found`」变成编辑期波浪线；
- GDScript 解析：`extends`、`signal`、`method` 存在性检查——覆盖 `[connection]` 静默丢弃与名字错配的前半程；
- 资源闭环：`res://` 引用目标存在性校验——`preload` 编译期报错的等效检查扩展到全部引用点。

引擎语义决定检查规则的优先级：🔴 项是硬错误诊断，🟡/⚪ 项是 warning 级提示。本站各章的源码锚点同时是 godotlsp 实现这些检查的规格依据。

## 延伸阅读

- 官方英文文档：[Error handling](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)、[GDScript exports](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_exports.html)
- 本站：[错误处理模型](../syntax/14-errors.md)（脚本层 Error 约定与防御式写法）、[工程启动链路](01-boot.md)、[场景文件 .tscn 解析](02-tscn.md)
