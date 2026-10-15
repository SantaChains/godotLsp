# Godot 工程文件编辑规则（面向 AI 助手）

> **适用场景**：AI（或脚本）要直接改 `project.godot` / `*.tscn` / `*.tres` / `*.gd`。
> **唯一目标**：不允许出现「改完程序加载不了」。真实事故与全貌见 §5。
> **引用基线**：引擎源码 `.refs/godot`（4.7-stable 浅克隆）+ 实测引擎 `4.7.2.stable.steam`、`4.8.dev7`。

---

## 0. 验收门：改完就跑，不看退出码

第一次用先导一次环境（本机 Godot 来自 Steam，PATH 上没有）：

```bash
export GODOT="D:/Steam/steamapps/common/Godot Engine/godot.windows.opt.tools.64-old.exe"   # 4.7.2
export GODOTLSP="$PWD/target/debug/godotlsp.exe"
```

四个门，按需组合（细节见 §10 / §11；也可改用 `--godot` / `--server` 逐个指定）：

```bash
uv run tools/godot_verify.py check <工程目录>                     # ① 工程加载：导入 + 实跑 N 帧,只回显错误行
uv run tools/godot_verify.py lsp   <工作区根> [--project 工程]     # ② 分析语义：诊断/符号/hover/补全
uv run tools/godot_probe.py <工程目录> tools/probes/*.gd          # ③ 行为：跑 GDScript 探针的断言
uv run tools/check_docs.py docs/*.md                             # ④ 文档：章节/表格/围栏/引用结构
```

- 全程无头（不弹窗）；`check` 内部先跑 `--import`（=工程生成），再 `--quit-after N` 实跑。
- **按错误行判账，不看退出码**：Godot 遇到致命错误也可能 `exit=0`（§5.2 实测）。
- 统一退出码：`0` 干净 / `1` 有失败 / `2` 环境或用法问题。
- **只跑 ① 不够**：不在加载路径上的脚本（死文件）引擎永远抓不到，必须配 ②（R10）；改了行为或物理则必须配 ③（R13）。

---

## 1. 先分清三类文件：谁生成、能不能手写

| 文件 | 谁写 | 能否手写 | 关键点 |
|---|---|---|---|
| `project.godot` | 编辑器保存（纯文本） | 可以 | ConfigFile/VariantParser 语法，见 §2 |
| `*.tscn` / `*.tres` | 编辑器保存 | 可以 | 结构约束见 §3，uid 见 §4 |
| `.godot/**` | **引擎生成** | **禁止** | `uid_cache.bin` 是二进制；`global_script_class_cache.cfg` 是扫描产物 |
| `*.import` / `*.uid` | **引擎生成** | **禁止** | 新增资源后由 `--import` 生成 |
| 备份/验证脚本 | 人 / AI | 可以 | 放点目录（`.backup/` 之类），引擎会跳过（§4 R8） |

---

## 2. `project.godot` 语法硬规则

**R1 字符串值必须加引号。** 这是本项目里真实发生过的致命事故：

```ini
window/handheld/orientation=locked     # ✗ 裸标识符 → 整个文件解析失败 → 工程完全加载不了
```

- 报错原文：`ERROR: Error parsing '…/project.godot' at line 29: Unexpected identifier 'locked' File might be corrupted.`
- **报错行号比真实行少 1**（真实在第 30 行，报 `at line 29`）——按报的行号去找会看错一行。

**R2 受 enum 提示的整型设置写整数，绝不编标签。**
`display/window/handheld/orientation` 的真身是 `Variant::INT` + `PROPERTY_HINT_ENUM, "Landscape,Portrait,Reverse Landscape,Reverse Portrait,Sensor Landscape,Sensor Portrait,Sensor"`
（`.refs/godot/core/config/project_settings.cpp:1766`），默认值 `SCREEN_LANDSCAPE`（`main/main.cpp:2831`）。
事件里的 `locked` **连合法标签都不是**。实测三种写法（`="Portrait"` / `=1` / 删行）都能加载，但规范只有两条：**写整数**，或**不确定就把整行删掉用默认值**。

**R3 不要发明键名。** 未注册的键会被静默保留但不生效——比报错更危险，因为你会以为它生效了。
拿不准就先在 `.refs/godot` 里 `grep` 一次（键名通常在 `main/main.cpp` 的 `GLOBAL_DEF*` 或 `core/config/project_settings.cpp:1766` 一带的 hint 表里）。

---

## 3. `*.tscn` 结构约束（全部实测）

**R6-1 第一个 `[node]` 必须是根节点（不带 `parent`）。** 反例——把子节点写在父节点之前：

```ini
[node name="Child" type="Node2D" parent="Parent"]   # ← 第一个节点却带 parent
[node name="Parent" type="Node2D" parent="."]
```

实测结果：`ERROR: Invalid scene: root node Child cannot specify a parent node`（`scene/resources/packed_scene.cpp:221`）→ `ERROR: Failed loading scene: res://main.tscn`（`main/main.cpp:4767`）。**场景整体加载失败**。

**R6-2 `parent="..."` 是 NodePath**（相对场景根，`.` 为根，层级用 `/`；`scene/resources/resource_format_text.cpp:206`）。

**R6-3 `ExtResource("id")` 必须与 `[ext_resource … id="…"]` 一一对应**；`SubResource` 的 id 在文件内唯一。

**R6-4 `%Name` 生效的前提是节点带 `unique_name_in_owner = true`**（`scene/main/node.cpp:4040`），否则运行期 `%Name` 找不到。

**R6-5 `load_steps=N` 只影响加载进度提示**，写错不致命——不要依赖它做校验。

---

## 4. 生成物与 uid

**R3′ `.godot/` 不要手写。** 缺失或损坏时跑 `godot --headless --path <工程> --import` 让引擎重新生成。注意：有 `.godot/` **不能**救一个语法错误的 `project.godot`（案例工程有 1.7MB 的 `.godot/`，照样加载不了）。

**R4 uid 一律不要手写。**
- `ext_resource` 的 `uid` 是**可选字段**，缺了就走 `path`（`scene/resources/resource_format_text.cpp:491`）。实测：手写 `.tscn` 完全不带 uid，导入/运行/编辑器打开**零告警**。
- 无效 uid 只报 `WARN_PRINT … invalid UID … using text path instead`（`:501`、`:504`）——不致命，但会污染 `uid_cache.bin`。
- 同一 id 重复注册会被拒（`core/io/resource_uid.cpp:164`，`ResourceUID::add_id` 里 `ERR_FAIL_COND(unique_ids.has(p_id))`）。
- 结论：**省略 uid，让引擎回填**。不要编 `uid://main123456` 这种值。
- **例外（程序化生成，2026-10-16）**：AI 管线若需程序化产出合法 uid（随机 63 位 + base-34 编码 + `.uid` 侧写注册），规格见 `engine-analyzer.md` UID 节——与「手编 uid 值」是两回事：手编是瞎造值，程序化是按引擎算法造值并走官方侧写注册通道。

**R5 `class_name` 跨文件引用依赖"工程生成"缓存。**
新拷贝或清掉 `.godot/` 后直接运行会得到：

```
SCRIPT ERROR: Parse Error: Could not find type "X" in the current scope.
SCRIPT ERROR: Parse Error: Identifier "X" not declared in the current scope.
ERROR: Failed to load script "res://…gd" with error "Parse error".   (modules/gdscript/gdscript_resource_format.cpp:46)
```

两种正解：先 `--import` 一次（`check` 已内含）；或**跨文件引用改用 `preload("res://x.gd")`**，零前置即可运行（实测通过）。对照实验：仅含一个 `class_name` 的极小工程同样有此行为——**这是 Godot 的固有要求，不是工程缺陷**。

**R7 autoload 与全局标识符（源码锚点核验 2026-10-16，LSP 实证）。**

写法：`[autoload]` 段 `Name="*res://scripts/x.gd"`。核心隐形规则是 **`*` 前缀 = 全局标志**：

- 引擎侧 `core/config/project_settings.cpp:352` `path.begins_with("*")` → `is_singleton`（`core/config/project_settings.h:71`）并 `substr(1)` 剥掉前缀。**非 `*` 条目仍会实例化到 `/root/Name`，但名字不是全局标识符**——代码里引用它会得到 `Identifier "X" not declared in the current scope`。编辑器「项目设置 → 全局 → 自动加载」表格里的「全局变量」勾选框就是这个标志，勾了才有 `*`。
- 解析锚点 `modules/gdscript/gdscript_analyzer.cpp:4579`（`resolve_identifier`）：仅 `is_singleton` 分支把名字解析为常量，成员类型来自 autoload **脚本本身**的解析结果（脚本编译失败时降级为 `Node`，不追加报错）；`PackedScene` 型 autoload 走场景根脚本（`:4596`）。
- `class_name` 与 autoload 同名 → `Class "X" hides an autoload singleton.`（`modules/gdscript/gdscript_analyzer.cpp:404`）。不要撞名（§4 R7 原规则）。
- **解析期求值看不到 autoload 实例**：const 初始化、`--check-only --script`（`main.cpp` 的 check_only 分支不建 SceneTree、不注册 autoload）都会 `Identifier not found`——这不是工程坏了，见 §11 探针约定的运行时取法。
- **LSP 侧实证**（release 二进制 + `godot_verify.py lsp`，探针工程含 `GameConfig="*res://scripts/game_config.gd"`）：`*` 单例的标识符解析、成员访问（`GameConfig.max_score`）、方法调用、成员补全（303 项，自有成员置顶）全部零误报；对非 `*` autoload 引用报错——**与引擎语义一致，是正确行为不是误报**。已知缺口（宽容方向，不阻塞）：autoload 成员不存在时不报（引擎会报 Cannot find member）；autoload 标识符上的 hover 为 null。工程不在工作区根时 autoload 解析失效 → 见 R9。

**R8 点目录（`.` 开头）被引擎跳过**（`editor/file_system/editor_file_system.cpp:1913` 逐级判断 `path_bit.begins_with(".")`）。所以备份、验证脚本、临时产物放在工程内的点目录（如 `.backup/`）是安全的、不会被导入。

---

## 5. 案例：knowledge-lock（AI 改坏 → 加载不了）

### 5.1 症状 → 根因

```bash
uv run tools/godot_verify.py check <工程>
```
```
[check] import(工程生成): exit=0 error=6
        ERROR: Error parsing '…/project.godot' at line 29: Unexpected identifier 'locked' File might be corrupted.
           at: _load_settings_text (core/config/project_settings.cpp:978)          ← 根因就在第一条
        ERROR: Couldn't load file '…/project.godot', error code 43.
           at: _load_settings_text_or_binary (core/config/project_settings.cpp:1027)
        SCRIPT ERROR: Parse Error: Could not find type "PlayerPhysics" in the current scope.
           at: GDScript::reload (res://scripts/level/level_manager.gd:126)
        ERROR: Failed to instantiate an autoload, script 'res://scripts/level/level_manager.gd' does not inherit from 'Node'.
           at: start (main/main.cpp:4539)
        SCRIPT ERROR: Invalid call. Nonexistent function 'load_level' in base 'Nil'.
           at: _ready (res://scenes/main.tscn::GDScript_main:4)
[check] 结论: 有 9 条 ERROR
```

**根因只有一个字符级问题**：`project.godot` 第 30 行 `window/handheld/orientation=locked` 少了引号（且 `locked` 是编出来的值）。

### 5.2 为什么"看起来还行"——两个假信号

1. **`exit=0`**：Godot 自己的退出码是 0。任何"跑了没报错/退出码为 0 就算通过"的判断都会放过它。
2. **错误清单很长**：9 条 ERROR 里 7 条是连锁（§5.3），容易被当成"后端问题"去逐个修。

### 5.3 一条引号的连锁反应：下游报错 ≠ 根因

`project.godot` 解析失败 → `ProjectSettings` 近乎为空 → 于是：

| 下游症状 | 真实原因 |
|---|---|
| `Could not find type "PlayerPhysics"` | 工程级 class 注册随 `ProjectSettings` 一起没了 |
| `Failed to instantiate an autoload … does not inherit from 'Node'` | 该 autoload 脚本因依赖解析失败而编译不过 |
| `Nonexistent function 'load_level' in base 'Nil'` | autoload 没实例化成功 → `LevelManager` 是 `Nil` |

**教训：从第一条 ERROR 往下读，别从最后一条往回猜。**

### 5.4 修掉致命项之后，露出的才是真实代码缺陷

把 `orientation` 那行删掉（或写整数）后重跑：致命层消失，**露出 3 类真实缺陷**（同样是 AI 写的）：

| 缺陷 | 证据 | 修法（本次已落地，实测可加载） |
|---|---|---|
| 重定义原生成员 | `scripts/player/player_physics.gd:18` — `Member "floor_snap_length" redefined (original in native class 'CharacterBody2D')`；“修”法的痕迹很典型：第 40 行写成了自赋值 `floor_snap_length = floor_snap_length` | 删掉非法的 `@export` 声明，在 `_ready()` 里做真正赋值 `floor_snap_length = 8.0`（原生属性不能重声明，但仍可在代码里设值） |
| 把类名当函数调用 | `scripts/ui/rule_indicator.gd:95` — `Function "Polygon2D()" not found in base self` | `Polygon2D.new()` |
| 在 `_ready()` 里同步换场景 | 运行期 `Parent node is busy adding/removing children, remove_child() can't be called at this time`（`scene/main/node.cpp:1750`） | 见 §6：在换场景出口统一 `call_deferred` |
| （引擎门抓不到）死文件里的错 API | `project_config.gd` — `viewport_set_default_canvas_item_texture_filter()` 少传 viewport `RID` | 见 R10：必须同时跑 `lsp` 子命令才能发现 |

注意：修前修后**错误总数都是 9**——所以「错误数」根本不是指标，**要看第一条是什么**。

### 5.5 反面教材：那份已有分析文档为什么危险

同目录下的 `LOADING_ISSUES_ANALYSIS.md`（203 行）全文**没有出现** `orientation` / `locked` / `解析` / 引号 等任何字样——它没找到致命项，却给出「把所有关卡场景坐标批量减半」「删除所有手写 UID」这类**大规模且不必要**的改动方向。

**错误的诊断会把 AI 推向"改得越多越像在修"的方向；先定位致命项再动手。**

---

## 6. 报错定位配方

| 现象 | 判断 |
|---|---|
| 输出第一条是 `_load_settings_text` / `ConfigFile parse error` | 就是根因；报错行号 **−1** 才是真实行 |
| 输出里出现**别的工程**的路径 | `--path` 指错了工程根：Godot 会静默打开项目管理器，并去解析"最近工程"列表里的其它工程，噪音就从那来 |
| `SCRIPT ERROR: Parse Error: … at: GDScript::reload (res://x.gd:126)` | 直接跳 `x.gd:126` |
| `Could not find type` / `not declared` | 先怀疑 `.godot/` 缓存缺失（R5），而不是先改类型注解 |
| 运行期 `busy adding/removing children` | `SceneTree::change_scene_to_node` 会**同步**执行 `root->remove_child(current_scene)`（`scene/main/scene_tree.cpp`），而新场景反而是延到 idle 才 add——所以在 `_ready()` 里换场景**必炸**。修法：在换场景出口统一延帧 `get_tree().change_scene_to_file.call_deferred(path)`；只改调用点也能过，但任何新的 `_ready` 入口会重新踩坑。引擎回溯直接给出调用点：`[0] load_level (…:81)` ← `[1] _ready (res://scenes/main.tscn::GDScript_main:4)` |

---

## 7. 分析侧（LSP）附加约束


**R9 语言服务器的 cwd = 插件的工作区根**（`editors/vscode/src/extension.ts:75`；Run Project 用同一根发 `--path .`，`:185`、`:205`）。工程必须在工作区根下且 `project.godot` 可见，否则 autoload 解析失败。实测同一份代码：

| 工作区根 | `game.gd` 诊断 | `GameConfig.` 补全 |
|---|---|---|
| 工程根 | 0 | **315** 项 |
| 仓库根（工程在子目录） | **8 条 error** | **68** 项 |

工程位于子目录时，用 `*.code-workspace` 把**工程排在第一**（`workspaceFolders[0]` 有语义）。
自检：`uv run tools/godot_verify.py lsp <工作区根> [--project 工程]`。

**R10 引擎门有盲区：不在加载路径上的脚本，引擎永远抓不到它的错。**
未被 `[autoload]` 注册、也未被任何场景引用的“孤岛”脚本不会被执行，所以 `check` 报 `error=0` 不代表它是对的。
实测（knowledge-lock）：根目录两个脚本名叫 `autoload.gd` / `project_config.gd`，但**根本没注册为 autoload**；
其中 `RenderingServer.viewport_set_default_canvas_item_texture_filter(FILTER_NEAREST)` 少传了 viewport 的 `RID`（4.7 需要 `(RID, filter)`）：

```
[lsp] DIAG project_config.gd: total=2 errors=2
        - [sev1] Too few arguments for "viewport_set_default_canvas_item_texture_filter()" call. Expected at least 2 but received 1.
```

**结论：`check` 与 `lsp` 两个子命令都要跑，别只用 `check`。**

---

## 8. 反模式清单（见一次改一次）

1. 往 `project.godot` / `.tscn` 写**不带引号的字符串**。
2. 编设置名、编 enum 标签、编 uid 值（`locked`、`uid://main123456`）。
3. 手写或手改 `.godot/**`、`*.import`、`*.uid`。
4. 把 `.tscn` 的子节点写在父节点之前。
5. 在 `.gd` 里重定义原生成员；把类名当函数调用（`Polygon2D()`）。
6. `remove_child()` 不延帧调用。
7. 改完不跑 §0 的验收门就宣称完成。
8. 拿错误**条数**当进度指标（§5.4：修前修后都是 9 条）。
9. 只跑 ① 工程加载门就宣布通过（死文件里的错 API 抓不到，R10）。
10. 改完行为不跑探针，只看"能不能加载"（R13）。
11. 用脚本对 `.md` 做逐行增删后不做结构校验（§11：围栏失衡、表格列数、编号断档都是这么来的）。
12. 探针或门改完不跑**反向用例**（§11：本次两个 bug 会让门永远报失败）。
13. 引用引擎行号不注明来源构建（R11）。
14. 改动前不自建备份（R12）。

---

## 9. 读引擎源码定位事实

**基线**：`.refs/godot` 是 **4.7-stable(=4.7.0) 浅克隆**；本机实测引擎是 **4.7.2.stable.steam**（Steam 目录里另有 4.8.dev7）。

**R11 引用引擎行号必须注明来源构建。** 同一函数在不同补丁版之间行号会漂移，而 `.refs` 只代表其中一个版本。踩过：报错原文里的行号在 4.7.0 克隆里落在**完全不同的函数**上（§5.1 那个引用块是 4.7.2 的构建输出，不要拿去 `sed -n Np` 对号）。查证一律**用符号名/字符串 grep**，别用行号：

```bash
cd .refs/godot
grep -rn "键名" core/config/project_settings.cpp main/main.cpp    # project.godot 键:类型/hint/默认值
grep -n  "方法名" doc/classes/X.xml                               # API 签名与是否存在(以引擎版本为准)
grep -n  'has("uid")' scene/resources/resource_format_text.cpp   # 文本场景(.tscn/.tres/project.godot)解析规则
grep -rn "报错里的原句" .                                       # 反查是谁打印的(带文件:行 + 函数名)
```

| 要查什么 | 去哪 |
|---|---|
| 某 project.godot 键的**类型 / 合法标签 / 默认值** | `core/config/project_settings.cpp`（`GLOBAL_DEF` 的 `PROPERTY_HINT_ENUM` 串）+ 消费它的 `GLOBAL_DEF_BASIC`（如 `main/main.cpp`） |
| 某方法**是否存在、参数签名** | `doc/classes/类名.xml`（别凭记忆写 API：`viewport_set_default_canvas_item_texture_filter()` 少一个 `RID` 就是错） |
| `.tscn`/`.tres` **文本格式规则** | `scene/resources/resource_format_text.cpp:491`（`ext_resource` 的 uid 可选）、`scene/resources/resource_format_text.cpp:501`（无效 uid 只 WARN）、`scene/resources/resource_format_text.cpp:276`（`groups` 字段）、`scene/resources/resource_format_text.cpp:206`（`parent` 是 NodePath） |
| **uid 语义** | 上面 + `core/io/resource_uid.cpp`（`add_id` 拒绝重复 id） |
| **场景/节点运行期行为** | `scene/main/node.cpp:1750`（`remove_child` 的 `blocked` 判定）、`scene/main/scene_tree.cpp:1722`（换场景会**同步** `root->remove_child(current_scene)`） |
| **物理体属性约束** | `scene/2d/physics/character_body_2d.cpp:651`（`up_direction` 拒绝 `Vector2.ZERO`） |
| **报错宏输出格式**（对照报错原文判真伪） | `core/error/error_macros.h:454`（`Condition "…" is true. Returning: …` 来自 `ERR_FAIL_COND_V_MSG`） |

两个常用判据：

- **报错行号可能是"上一行"**：VariantParser 在 `project.godot` 第 30 行的错误给的是 `at line 29`（§5.1 实测）。看到 parse error，**上下各看一行**。
- **引擎里搜不到的字符串就不是引擎输出**：把粘贴来的 `ERR0R`（数字 0）全树 grep 无命中 → 那是复制时走形，不是 Godot 打的。"看着像引擎的报错"先反查发射点。

---

## 10. 工程化：改动前的准备与闭环

**R12 改动前必须自建备份。** 本仓库的 Godot 工程（含本次案例工程）**都没有 git 兜底**（只有残留的 `.gitignore`），改坏了没有 `git checkout`。用 R8（点目录引擎跳过）在工程内建备份，并按轮次分目录：

```bash
cd <工程>
mkdir -p .backup/round2/scripts/player
cp scripts/player/player_physics.gd .backup/round2/scripts/player/   # 本轮动手前的版本
cp <待删文件> .backup/                                              # 删除类改动必须留副本
```

报告里要写清"撤本轮 = 还原哪几个文件 + 删哪几个新文件"，不能只说"有备份"。

**R13 验收分三层，行为类改动必须配探针。** 顺序固定：先跑门拿基线 → 一次只改一类 → 立刻跑门 → 行为类再跑探针 → 记证据。

| 层 | 命令 | 覆盖 |
|---|---|---|
| 静态 | `uv run python -m py_compile tools/*.py` + `uvx ruff check` + `uvx pyright`；GDScript 用 `--check-only --script`（**仅对不引用 autoload 的脚本有效**） | 语法、类型、未使用项 |
| 工程加载 | `uv run tools/godot_verify.py check <工程>` | `project.godot` 解析、全工程脚本编译、实跑 N 帧 |
| 分析语义 | `uv run tools/godot_verify.py lsp <工作区根>` | 诊断/符号/hover/补全；能抓死文件里的错 API（R10） |
| 行为 | `uv run tools/godot_probe.py <工程> tools/probes/*.gd` | 规则、物理、关卡流程的真实行为 |

**文档改完也要跑** `tools/check_docs.py`：章节号连续、表格列数一致、围栏成对、`§N`/`Rn` 引用可解析、**正文里的引擎源码引用有效**（这是文档唯一的自动化正确性检查）。引用只在写成连续的 `path:line` 时才进自动校验，写成“路径 … `:NNN`”的简写只有人能核。

**本仓库现有的两个 Godot 靶子**：

| 路径 | 性质 | 备注 |
|---|---|---|
| `godot-test/` | 可运行集成夹具（贪吃蛇） | 47 断言探针在 `godot-test/.backup/verify_probe.gd` |
| `D:/LocalSend/Files/pro/knowledgeGame/knowledge-lock` | 外部实战案例（曾被 AI 改到加载不了） | 规则回归探针在 `tools/probes/kl_*.gd` |

**新语义要显式写下来并标注可否决。** 案例：实现 `COLOR_LOCK` 时取的是"同色 = 钥匙（同色门可穿、异色门阻挡）"，反过来的读法也说得通 —— 这种二选一要么问，要么**写清取值 + 一句"要改是一行"**，不能默认藏起来。

---

## 11. 程序化：工具、探针约定与本机踩过的坑

### 工具

| 工具 | 干什么 | 退出码 |
|---|---|---|
| `tools/godot_verify.py check <工程>` | 无头导入 + 实跑，只回显 ERROR/WARNING（带 `at:` 与 GDScript 回溯）；**按错误行判账** | 0/1/2 |
| `tools/godot_verify.py lsp <根> [--project]` | stdio LSP 会话：诊断数、documentSymbol、关键 hover、`autoload.` 补全项数 | 0/1/2 |
| `tools/godot_probe.py <工程> <探针.gd>…` | **在临时副本**注入并运行 GDScript 探针，汇总断言与引擎 ERROR；副本缺工程生成产物时先 `--import` | 0/1/2 |
| `tools/check_docs.py <md>…` | Markdown 结构门（围栏/编号/表格/引用/源码引用） | 0/1/2 |

### GDScript 探针约定（`tools/probes/*.gd`）

**R14 不要用 idle 帧数等待物理结果。** `_process` 与物理帧并不同步（本机曾观测到一次 `_process` 对应约 2 个物理帧），"等 120 帧"可能只过了 0.6 秒。改成**条件等待**：

```gdscript
if _player.is_on_floor() or _frame > 900:   # 等到真的落地再往下断言
```

踩过：翻转重力后按固定帧数取状态，玩家其实还在上升 → 误判为"修复无效"。

- `extends SceneTree` + `_process` 里 `match _phase` 的状态机；每条断言 `print("PASS | …")` / `print("FAIL | …")`。
- **必须**在结尾 `print("RESULT failures=", n)`：运行器缺这行即判"探针未跑完"，按失败处理（防"跑到一半崩了也算过"）。
- **不能在解析期引用 autoload**：探针在 autoload 常量注册**之前**被加载，直接写 `RuleManager` 会 `Compile Error: Identifier not found`。只能在运行时取：

  ```gdscript
  var rules := root.get_node_or_null("/root/RuleManager")
  ```

- 探针只写进**副本**的 `res://` 根；运行器负责复制与清理，源工程零改动（可用 `find <工程> -type f -exec md5sum {} + | sort | md5sum` 前后比对自证）。
- **反向用例是必跑项**：故意 FAIL 的探针 / 不打印 RESULT 的探针 / 语法错的探针 —— 三条都要，否则不知道门会不会失败。

### 本机脚本化的坑（都真实踩过）

| 现象 | 事实 | 对策 |
|---|---|---|
| 写文件工具把 `/tmp/x` 落到 `D:/tmp/x`，bash 的 `/tmp` 却是 `%TEMP%` | 两者不同盘 | 跨工具传路径一律用**绝对 Windows 路径** |
| MSYS `which uv` 给出 `/d/Soft/.../uv` | Windows 侧程序不认 POSIX 形式 | 传 `uv.exe` 的 Windows 路径 |
| `--check-only --script res://x.gd` 报 `Identifier not found: RuleManager` | `main.cpp` 的 `check_only` 分支**直接 return**，不建 SceneTree、不注册 autoload | 该模式只用于**不引用 autoload** 的脚本；其余脚本的编译检查交给工程加载门 |
| 管道后的 `$?` 不是被管道命令的退出码 | 拿到的是最后一个命令的 | 用 `${PIPESTATUS[0]}` |
| `print("a", b)` 是直接拼接（无空格） | 生成给解析器认的标记行时要注意 | 标记行用固定前缀（如 `RESULT failures=`） |
| 报错里混进**别的工程**的路径 | `--path` 指错时 Godot 会静默开项目管理器并解析"最近工程"列表 | 先确认 `--path` 是工程根（§6） |

### 门本身也要验

本次两个 bug 都属于"门自己算错账"，只有跑反向用例才会暴露：

1. `RESULT_RE` 的 `^` 漏了 `re.MULTILINE` → 干净的跑也判"未跑完"（三个探针全 FAIL）。
2. ERROR/WARNING **没按块分类** → 跟在 `WARNING` 后面的 `at:`/回溯行被计成独立 ERROR（显示 `ERROR=4`，实际 0）。

写"判账"逻辑时的三条：级别按**块**归属（头行定级、续行跟随）；正则带 `re.MULTILINE`；改完先跑一遍反向，再相信它会失败。
