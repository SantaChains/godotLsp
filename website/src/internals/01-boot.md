# 工程启动链路

按下运行键之后、任何一行 `func _ready()` 执行之前，引擎要先完成三件事：**读配置、挂单例、开主场景**。本章沿真实加载顺序拆解这条链路，并标注每一步的失败后果——错误被分为三级：🔴 **拒载**（引擎直接报错退出）、🟡 **降级**（打印警告后继续，功能受损）、⚪ **静默**（无任何报错，结果悄悄不对）。全部结论以 Godot 4.7 源码为锚点核验。

## 第一步：project.godot

引擎启动的第一个动作是把工程根目录的 `project.godot` 读进 `ProjectSettings` 单例。这是一个普通 INI 风格文本文件，但解析失败的代价是**整个工程打不开**：

| 检查项 | 规则 | 失败后果 |
|---|---|---|
| `config_version` | 必须 ≤ 5（4.x 的版本号） | 大于 5 → 🔴 `ERR_FILE_CANT_OPEN`，项目管理器直接拒绝打开 |
| `config_version` 缺失 | 视为 0 | 兼容，正常加载 |
| 任何语法错误 | 引号不闭合、枚举值非法等 | 🔴 `Error parsing 'project.godot' at line N ... File might be corrupted` |
| `config/features` 含未知 feature | 如未来引擎版本号 | 🟡 弹「可能不受支持」警告，照常打开 |
| 未注册的键（键名拼写错） | 不属于任何已知设置项 | ⚪ **静默保留**——引擎不报错不生效，配置悄悄失效 |

最后一条是最隐蔽的：`project.godot` 对未知键宽容存放，拼写错误（如把 `run/main_scene` 写成 `run/mian_scene`）不会得到任何提示。这正是 LSP 对配置文件做键名校验的价值所在。

源码锚点 `core/config/project_settings.cpp::_load_settings_text`。`config_version` 的语义是**格式版本而非引擎版本**——4.x 全系都是 5，它存在的作用是让旧引擎遇到「更新的配置格式」时能明确拒绝，而不是猜。

```ini
; Engine configuration file.
config_version=5

[application]
config/name="Demo"
run/main_scene="res://scenes/main.tscn"
config/features=PackedStringArray("4.7", "Forward Plus")

[autoload]
GameState="*res://scripts/game_state.gd"
```

## 第二步：autoload 注册

`[autoload]` 节的每一项都是一个节点：键是节点名，值是脚本或场景路径。`*` 前缀表示**单例**——注册为全局常量，任何脚本里直接写名字就能访问；无 `*` 则只是挂到根下、不进全局命名空间。

加载过程分**两遍**（源码锚点 `main/main.cpp` 启动段），这个顺序决定了几个重要行为：

```text
第一遍：为所有 * 单例先注册"空占位"全局常量
第二遍：逐项实例化 → 设名 → 把真值写进全局常量
最后：  所有 autoload 节点统一 add_child 挂到根节点下
```

由此推出的三条规则：

1. **顺序 = 文件内出现顺序**。排在前面的 autoload 先可用；`GameState` 想在 `_init` 里访问 `SaveManager`，`SaveManager` 必须排在它前面。
2. **脚本必须继承 Node**。`get_instance_base_type()` 校验基类，不满足 → `ERR_CONTINUE` **跳过该单例**（🟡）——下游报出的 `Identifier not found` 是假错误，真因是单例没注册上。
3. **统一挂树**。所有 autoload 建好之后才一起 `add_child`（源码注释原话：*defer so references are all valid on _ready()*）——所以任何 autoload 的 `_ready()` 里都能引用其他 autoload。

> 值得注意的是：autoload 值既可以是脚本也可以是 `.tscn` 场景（引擎按资源类型分流）。用场景做 autoload 时，根节点上的脚本才是那个"单例"。

## 第三步：主场景

`application/run/main_scene` 指向的 `.tscn` 会被实例化为当前场景，挂到根节点下。这一步是启动链路的终点，也是下一环节（场景实例化，见[场景文件 .tscn 解析](02-tscn.md)）的起点。

| 状况 | 后果 |
|---|---|
| `main_scene` 未设置且启动参数无场景 | 🔴 `Can't run project: no main scene defined`，运行直接报错退出 |
| `main_scene` 指向的文件不存在或非法 | 🔴 按场景加载错误处理，无法进入主循环 |

> 需要注意的是：编辑器里 F5 第一次运行会引导你选定主场景，本质上就是在写 `run/main_scene` 这一行。程序化/CI 环境没有这层引导，漏写即拒启。

## 启动链路总览

```text
godot 进程启动
 └─ Main::setup
     └─ 读 project.godot          ← config_version 超版 / 语法错误 → 拒开
 └─ Main::start
     ├─ 构建 SceneTree
     ├─ 第一遍：单例空占位注册
     ├─ 第二遍：逐项实例化 autoload（脚本须继承 Node，否则跳过）
     ├─ 统一 add_child 挂根
     └─ 实例化主场景 → 进入主循环（_process/_physics_process 开始）
```

## 延伸阅读

- 官方英文文档：[Project Settings](https://docs.godotengine.org/en/stable/tutorials/scripting/project_settings.html)、[Singletons (Autoload)](https://docs.godotengine.org/en/stable/tutorials/scripting/singletons_autoload.html)
- 本站：[autoload 单例](../syntax/13-autoload.md)（语法与最佳实践）、[脚本致命面与静默失败](07-fatal.md)（本章各失败项的汇总清单）、[场景文件 .tscn 解析](02-tscn.md)（主场景实例化之后发生什么）
