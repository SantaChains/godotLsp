# 补全增强路线图 — 引擎数据面 × 官方 snippet(2026-10-16 立项)

> **状态:P0 已实施;C7.3/C6.4/C6.6/C6.1/C6.2/C7.1/C7.2/N2 已落地(2026-10-17);其余规划中。**
> 本文件是 C6-C9 四项工作流的排期依据;设计论证与源码锚点见
> `lsp-design.md` §6.9(证据深挖),能力面索引见 `lsp-engine-capabilities-roadmap.md` P3.6。
>
> **定位不变**:零引擎依赖 + 内嵌模型。本路线图只做"数据面补齐、工程面接入、snippet 对齐",
> 不动并发架构、不外挂引擎、不引运行时依赖。
>
> **验收基建已就位**:`tools/completion_oracle.py`(引擎 oracle harness)+
> `tools/completion_oracle_baseline.json`(基线 **33/140**)。每期实施后重跑,
> `--update-baseline` 前必须零回归;分桶通过率即量化验收。

## 一、结论(一屏)

| 期 | 项 | 层 | 内容 | 风险 | 状态 |
|---|---|---|---|---|---|
| P0 | **V0** | 工具 | 引擎补全 oracle harness(140 可判定夹具 = 146 .gd − 6 .notest;量化基线) | 无(纯新增工具) | ✅ 已实施(2026-10-17,首跑 20/140) |
| P1 | **C6** + **C7.3** | gdscript-engine + godotlsp | 模型字段解析+三类消费+内建构造/运算符面(C6.5);$/% 回归止血 | 低(附加式 serde) | 🔶 C6.1/C6.2/C6.4/C6.6 已落地;C6.3/C6.5 待 |
| P1 | **C7.3** | godotlsp | $/% 节点路径上下文止血 | 低 | ✅ 已落地(2026-10-17,单测+E2E) |
| P2 | **C7.1/C7.2** | godotlsp | class_name/autoload 全局面;res:// 路径面 + N2 字符串抑制 | 低-中 | ✅ 已落地(2026-10-17,oracle 24→**33/140**,+9 全为 C6.1 信号面战绩) |
| P3 | **C8** | godotlsp | 注解参数面(小手搬表 + 模型全局枚举) | 低(表漂移可检测) | 待 |
| P4 | **C9** | 插件 + 工具 | snippet 16→~40 对齐;官方模板入库备料 | 无(纯增量) | 待 |

**取舍主线**:先建能量化的验收基建(V0),再做零行为风险的模型层(C6),回归修复(C7.3)随 P1 止血;
每项独立可回退、独立可组合(遵循项目一贯的开关纪律)。

## 二、证据基线(全部本地化,可复验)

| 源 | pinned | 本地位置 | 入库? | 用途 |
|---|---|---|---|---|
| Godot 引擎全量仓库 | `5b4e0cb0`(2026-06-17) | `.refs/godot/`(gitignore) | 否 | `modules/gdscript/` 全部源码锚点;`doc/classes/` 810 XML;9 个 script_templates |
| 引擎自带补全测试 | `9a05d57` | `corpus/godot-engine-tests/completion/` | 否(`fetch_corpus.py` 幂等) | **oracle**:146 `.gd`(➡ 光标)+ 143 `.cfg`(include/exclude)+ 2 `.tscn`,14 个分类 |
| godot-vscode-plugin | `145a0f0`(2026-07-01,36 条) | `corpus/snippets/vscode/`(规划) | 否(`fetch_snippets.py` 规划) | 官方 snippet 基准(MIT) |
| gdscript-zed-snippets | `eacdbf0`(2025-11-01,34 条) | `corpus/snippets/zed/`(规划) | 否(同上) | Zed 向 snippet 基准(MIT),与我方多 IDE 定位相关 |
| extension_api.json | Godot 4.7.0 stable dump | `crates/gdscript-engine/testdata/` | 是(`include_str!` 生产数据) | 引擎模型唯一事实源 |
| 文档站/hover 生成面 | 随语料 | `tools/gen_docs.py` → `website/src/builtin/class_*.md` + `crates/godotlsp/src/hoverdata.json` | 是(生成物) | 类描述/方法/属性/枚举/常量 hover 与文档站;**Constructor/Operator 区块未解析(见 §3.5)** |

oracle 分类(与工作流的对应):`argument_options/`(C8)、`get_node/`($/% 上下文 + 成员链;
信号面 C6.2 后 8/33,余为节点名面 N6)、
`no_parenthesis_when_callable_is_expected/`(C6.4 insertText)、`filter/`(排噪/排序)、
`types/`(C6.3 虚方法/类型面)、`assignment_options/`(C8 枚举属性)、`index/`、`extends/`、
`builtin_enum/`、`global_enum/`、`script_enum/`、`common/` 等。

## 三、差距量化(为什么要做)

### 3.1 模型层:已在吞咽却丢弃的字段(RawApi 未解析,4.7.0 testdata 实测)

| 字段 | 规模 | 现状 | 消费计划 |
|---|---:|---|---|
| `classes[].signals` | 503 条 | 丢弃 | C6.2 成员面补信号(kind Event) |
| `methods[].is_virtual` | 1437 个 | 丢弃 | C6.3 替代手维护 VIRTUALS 12 条(与 `ClassDB::get_virtual_methods` 同源) |
| builtin `operators`/`constructors` | int: 64/5;38 类全有 | 丢弃 | **C6.5 升级为工作面**(见 §3.5) |
| builtin `is_keyed`/`indexing_return_type` | — | 丢弃 | C6.5d 只解析,消费备未来 |
| `api_type`/`is_refcounted`/`is_bitfield` | — | 丢弃 | C6.5 只解析(版本展示/过滤备未来) |
| `header.version_full_name` | "Godot Engine v4.7.stable.official" | 丢弃 | C6.5 可立即进日志/关于展示(小甜点) |

### 3.2 snippet 面:16 vs union 52(前缀差集实测)

我方 `editors/vscode/snippets/gdscript.json` 16 条(自造短前缀 asrt/conn/exn/exr/exv/getn/
inp/inst/lam/match/onr/phyp/prc/rdy/sig/timer);上游两家 union 52 个前缀。缺口按价值分桶:

| 桶 | 缺口前缀 | 取舍 |
|---|---|---|
| 回调标准名(引擎用户肌肉记忆) | ready/process/physics/input/init/draw/guii/uinput/unhandled | **收**(我方只有 rdy/prc/phyp/inp 自造名) |
| 声明类 | func/void/funcret/stfunc/stvar/var/export/exportvar/onready/onvar/signal/class/cn/enum | **收** |
| 控制流 | for/if/ifelse/while/pass/is/in | **收**(match/lam 已有) |
| 惯用 | print/pr/prwarn/prerr/clamp/lerp/await/sc/tween/wait/waitsec/waitframe/loadres/preloadres | **收** |
| 低性价比 | gdscript/gs/gst(vargetset)、element in array | **不收** |

### 3.3 工程面缺口

class_name / autoload 全局面补全缺失(引擎有;`gdscript-project` **已索引** `class_names`/`autoloads`/`scenes`/`uid_paths`,只缺消费);`preload("res://` / `load("res://` 字符串内无路径补全;$/% 节点路径上下文在 R4 重写(22882f7)中回归丢失(fa7d645 曾实现,探针实测 1662 项污染 → 纯节点列表)。

### 3.4 注解参数面缺口

`@export_range` 第 4 参 slider 串、`@export_exp_easing`、`@rpc` 三组选项、`@warning_ignore*` 全名、
`@export_custom` 两参、`@export_node_path`——引擎在注解名后第一参数位出候选(源码锚点
`.refs/godot/modules/gdscript/gdscript_editor.cpp:1028` `_find_annotation_arguments`),我方空白。

### 3.5 内建类型参考面缺口(2026-10-16 晚审计:以 `int` 为样本)

用户贴出的官方 `int` 类参考(描述 + 5 构造函数 + 64 运算符,各带描述与示例)逐层核对结果:

| 维度 | godot-docs 语料 | gen_docs/hoverdata 管线 | RawApi 模型 |
|---|---|---|---|
| 类描述 | ✅ `gdd_1559_int.md` Description | ✅ 已解析(29 个内建类全,c 表 1076 条) | — |
| 方法/属性/枚举/常量 | ✅ Method/Property/Enumerations 区块 | ✅ 已解析(`m` 表:String 116 / Array 51 / Vector2 54) | ✅ 已解析 |
| **构造函数**(签名+描述) | ✅ Constructors + Constructor Descriptions 区块 | ❌ **未解析**(904 个页面零"构造函数"节) | ⚠️ dump 有(38 类),RawApi 丢弃 |
| **运算符**(签名+描述) | ✅ Operators + Operator Descriptions 区块(64 条) | ❌ **未解析** | ⚠️ dump 有(int 64 个),RawApi 丢弃 |
| 通用运算符语义 | GDScript reference | ✅ `OPERATOR_DOCS` 20 条中文一行(langdocs.rs) | — |
| 中文翻译 | ❌ 英文语料 | ❌ | P2-16 i18n 线 |

**结论:数据在两层源里都完整,但两个消费管线(文档生成面、LSP 模型面)都只接了方法/属性/枚举/常量;
构造函数与运算符两条区块全线未接。** 这不是"备未来"问题,是明确的工作面缺口 → C6.5 升级(见 §四)。

### 3.6 oracle 普查差距地图(2026-10-17 两轮:20/140 → 24/140 → **33/140**)

分桶通过率(当期 33/140):filter 2/2、script_enum 1/1、builtin_enum 5/5、common 8/20、
global_enum 3/6、index 1/9、enum_values_in_dictionary 1/13、types 2/19、get_node 8/33、
argument_options 0/11、no_parenthesis 2/8、enum_values_in_match 0/7、assignment_options 0/4、
extends 0/2。未命中成因:absent 281 / paren-display 55 / string-literal 9。

按"阻塞面"归纳(而非按桶),新增/调整如下工作项:

| # | 发现 | oracle 证据 | 处置 |
|---|---|---|---|
| N1 | **self 成员面缺失**:裸标识符上下文不出当前类成员(`owner`/`add_child(…)/property_of_a`) | common `identifiers_in_*` ×3、argument_options/connect | **新项 C10(设计后实施)**:全局面合并宿主链成员 + 新 sort 桶;N2 已落地不再构成前置,与 N4 同批即可 |
| N2 | **字符串上下文不抑制** ~~:字符串字面量内仍出全局面~~ | `no_completion_in_string`(曾为 vacuous pass) | ✅ 已落地(2026-10-17,C7.2 同批):`string_literal_at` 前向状态机,字符串内返回空(路径面例外);vacuous pass 转为真实通过 |
| N3 | **脚本类成员面缺失**:`extends "res://x.gd"` 的成员不出(`func_of_a()`) | common `invalid_super_call_*` ×2、`override_function_*` ×4 | C7.1 的语义延伸(工程面解析 extends 路径 → 成员面);override 签名展示另属 C6.3 |
| N4 | **枚举限定展示**:类型化 LHS 后引擎出 `Control.SIZE_EXPAND`(限定名),我方不出类枚举值 | assignment_options ×4 | 依赖 N1(裸上下文类枚举值)+ 展示约定,随 C10 |
| N5 | **callable-expected 上下文不出括号** | no_parenthesis ×8(当前 0/8,C6.4 后仍挂) | 需参数类型分析(语义层),缓行;桶内 `lambda_body` 已因 C6.4 通过 |
| N6 | **节点路径类型推断 + 场景树节点名** | get_node 0/33 | 语义层 + C7.3+(project.scenes → NodeDecl),最大桶,分期 |
| N7 | **字典键/字典内枚举值** | index 1/9、enum_values_in_dictionary 1/13 | 明确不做 #3 延续,仅观察 |
| N8 | **字符串字面量参数面**(NodePath/StringName 建议) | argument_options/string_literals ×8 | C8 家族;N2 已落地,字符串上下文识别不再阻塞 |

**vacuous pass 清单(补面时必须回归验证)**:`no_constructor_*`(6)、
`builtin_enum/global_enum/script_enum` 的 metatype 三件套。`no_completion_in_string`
已于 2026-10-17 随 N2 落地转为真实通过。oracle 的 exclude 语义会在对应面补齐后自动
翻转这些桶——这正是 harness 的价值。

## 四、分期路线图

### P0 — V0:oracle harness ✅ 已实施(2026-10-17)

`tools/completion_oracle.py` 对 `corpus/godot-engine-tests/completion/` 全量夹具跑
LSP `textDocument/completion`,按 `.cfg` 的 `include`(必须出现)/`exclude`(不得出现)判定,
输出**分桶通过率**(每目录 pass/fail 计数 + 失败明细 + 未命中成因分类)。

- **用法**:`python tools/completion_oracle.py [--filter S] [--verbose] [--update-baseline]`;
  默认 server=`target/debug/godotlsp.exe`(回退 release);退出码 1 = 相对基线回归。
- **判定语义**(镜像 `test_completion.h`):➡ 光标定位(UTF-16 列);`.cfg` ConfigFile 子集解析;
  `display`↔label、`insert_text`↔`insertText`(LSP camelCase;**容许剥 `$1)`**——引擎适配器在
  snippetSupport 时把 `name(` 改写为 `name($1)`,oracle 期望值是适配前的 option 值);
  `location` 不可验证忽略;`kind`/`call_hint`/`forced` 语料未用。
- **工作区**:夹具树(.gd/.tscn)镜像进临时工作区 + 最小 project.godot,单服务器顺序
  didOpen→completion→didClose(项目扫描在 initialize handler 内同步完成,FIFO 保证文档先于请求)。
- **首跑普查(2026-10-17,C6.4 前)**:20/140(14.3%);当期(24/140,17.1%)。
- **自确证**:harness 自身抓到并修复一个匹配 bug(读 snake_case `insert_text` 恒 None
  → 带 insert_text 期望的 include 假性未命中)——oracle 语义与引擎同构,值得持续信任。
- **为什么先做**:首跑即得基线普查(预期大量 fail——这正是 C6-C9 的量化目标面);之后每期重跑,
  对应桶 fail→pass 即为验收。**没有它,C6-C9 的"对齐引擎"不可证明。**
- **诚实校准(通过集含虚过)**:`no_constructor_*`(6)、`no_completion_in_string`、
  metatype 三件套等通过项是** vacuous pass**——我方因整个面缺失而未触发的排除规则,非正确实现。
  后续补 self 成员面/字符串上下文抑制时必须连带验证这些桶(见 §3.6)。
- **风险**:引擎 oracle 语义与 LSP 有差(display 带引号、超集通过);匹配器容许 include 超集
  (引擎测试同语义),exclude 严格。

### P1 — C6 模型层(gdscript-engine,附加式 serde,零行为风险)

落点:`crates/gdscript-engine/src/lib.rs`(RawApi 221-350 / `into_model` 352+)。

- **C6.1 解析** ✅(引擎层,并发会话落地):`EngineClass.signals: Vec<EngineSignal>` +
  `find_signal`/`find_enum_value` 已在 `gdscript-engine`;`api_type`/`is_refcounted`/
  `is_virtual`/builtin `operators`/`constructors` 仍未解析(见 C6.3/C6.5)。
- **C6.2 信号入成员面** ✅ 已落地(2026-10-17):`class_members` 沿祖先链收
  `EngineClass.signals` → `CompletionItemKind::Event`,detail 为 `render_signal` 签名
  (`name(a: T, b)`,无返回类型——信号是通知)。**oracle +9 全部来自本项**:
  `$AnimationPlayer.` 经引擎类名回退解析成成员链后,`child_entered_tree`(Node 信号)/
  `animation_changed`(AnimationPlayer 信号)补齐,get_node 桶 0/33→8/33、
  types/member 桶 +1;单测 `completion_member_signals`(CanvasItem.draw)+ E2E(444 项成员面)。
  `argument_options/connect.cfg`(connect 首参信号名串)仍待 C8 参数面。
- **C6.3 虚方法模型化**(`crates/godotlsp/src/snippets.rs` VIRTUALS 12 条 → `chain_virtuals`):
  签名取模型参数名+类型;9 个非 ClassDB 回调留手表(`_init`/`_notification`/`_get`/`_set`/
  `_to_string`/`_property_can_revert`/`_property_get_revert`/`_get_property_list`/`_validate_property`)。
  **取舍**:Node 系虚方法数十个全出是引擎行为(override 面预期),sortText 分桶不受影响;
  不引入"频率截断"(那需要 C5 locals_at,属未来线)。
- **C6.4 函数 insertText** ✅ 已落地(2026-10-17):成员面函数项 + 全局面 utility/gdscript 函数
  统一 `label = name + "(…)"`;`insertText` 在 snippetSupport 协商时为 `name($1)`(Snippet),
  否则退化为裸名(引擎 `trim_suffix("(")` 同款——客户端放不进光标就不给括号)。
  频率表查表剥 `(…)` 后缀(scored_fuzzy),热度细分不失效。**callable-expected 上下文
  (如 `connect(` 内)不出 `()` 未做**——需语义层参数类型分析,见 §3.6 新发现。
  oracle 增量:paren-display 缺口 75→50,+3 夹具(lambda_body/assign_local_variant 等)。
- **C6.5 builtin 构造/运算符面**(§3.5 审计后由"只解析备未来"升级为工作面):
  - **C6.5a 模型解析**:`RawBuiltin` +`constructors`/`operators`(附加式 serde,38/38 类有数据);
    查询 API:`builtin_constructors(name)`、`builtin_operators(name)`。
  - **C6.5b 文档生成面**:`tools/gen_docs.py` 增补 `Constructors`/`Operators`/
    `Constructor Descriptions`/`Operator Descriptions` 四区块解析(现有解析器已处理同构的
    Method/Property Descriptions,扩展成本低)→ website 内建页补"构造函数/运算符"节 +
    hoverdata 增构造/运算符条目(与 `--check` 防漂移机制同程)。
  - **C6.5c 消费面**:① `Vector2(` / `int(` 等内建类型调用 → 构造重载补全(参数签名,
    引擎 `BUILT_IN_TYPE` 上下文同款);② 类型名 hover 追加构造重载列表;③ 运算符 hover 维持
    通用 `OPERATOR_DOCS`(**per-type 运算符 hover 需二元表达式类型推断,属语义层,列入开放问题**)。
  - **C6.5d 仍只解析备未来**:`is_keyed`/`indexing_return_type`/`api_type`/`is_bitfield`;
    `version_full_name` 可立即进启动日志(小甜点,可选)。
- **验收**:C6.5a 单测锁 38 类构造/运算符计数;C6.5b `gen_docs.py --check` 通过且
  `class_int.md` 含 5 构造 + 64 运算符节;C6.5c 单测(`Vector2(` 构造面)+ smoke;bench 不回退。
- **验收**:C6.2/6.3/6.4 各带单测 + oracle 对应桶 + bench_load 不回退(基线 completion p50 8.6ms)。

### P1 — C7.3 回归止血 ✅ 已落地(2026-10-17)

- fa7d645 方案镜像恢复:服务器侧 `$`/`%` 上下文后向字节扫描 + `%` 取模操作数前缀启发式
  (`in_node_path_context`);smoke 补 `$Panel/` 断言(单测 `node_path_context_detection` +
  `completion_node_path_no_global_pollution` + E2E 探针:`$Panel/` 1808→0 项、`%Panel` 0 项、
  取模上下文不误伤);`gdscript-features-roadmap.md` 第 7 项 ⚠️ → ✅(节点名面待 C7.3+)。
- **附带的 C6.6 小面**(同批落地):`Class.Enum.<|>` 枚举接收者 → 出该枚举值面
  (`enum_member_values_at`,引擎类+内建类;oracle builtin_enum 桶 4/5→5/5)。
- 依赖:无。风险:低。这是已知回归,不是新设计。

### P2 — C7 工程面接入 ✅ 已落地(2026-10-17)

落点:`crates/godotlsp/src/main.rs` `fn completion`;新增 `string_literal_at` /
`is_load_call_arg` / `preload_path_items` 三函数 + `gdscript-project::Project::autoloads()`
访问器(附加式)。测试 fixture:`crates/godotlsp/tests/fixtures/projface/`(project.godot
带 `[autoload]` + class_name 脚本 + tscn)。

- **C7.1 全局面 class_name + autoload** ✅:遍历 `Project.scripts` 的 `ast.class_name`
  (kind CLASS,detail `class_name (res://…)`)与 `Project.autoloads()`(kind VARIABLE,
  detail `类型 (autoload)`);与引擎类/内建/单例同名时让位引擎(同名注册在 Godot 中即非法)。
  实测全局面 1808→1810 项(+Player +GameManager)。
- **C7.2 res:// 路径面** ✅:`preload("res://` / `load("res://`(含 `ResourceLoader.load(`、
  空白变体)字符串实参 → 项目文件面;来源 = scripts + scenes 索引路径,label 完整
  `res://…` 路径,**textEdit 替换字符串内容区**(开引号后..光标)——不依赖客户端
  wordPattern(各编辑器对 `/`/`.` 成词约定不一)。E2E 实测 `res://pla` → player.gd。
- **N2 字符串抑制** ✅(C10 的一半,先落地):`string_literal_at` 前向状态机(单行/
  `"""` 多行/转义/`#` 注释截断/未闭合到行尾/`&`/`^` 前缀);字符串内默认返回空
  (引擎默认行为,`no_completion_in_string` 从 vacuous pass 转为真实通过),
  C7.2 路径面是唯一例外。**N1(self 成员面)仍待设计**——它是 C10 剩下的一半,
  落地时会翻转一批当前 vacuous pass,须与 N4 同批。
- **取舍记录**:字符串内补全依赖客户端在引号中触发的主流默认行为;路径面不做配置开关
  (前缀过滤已收敛);目录节点不补(只补文件);`.uid`/`.import`/`.godot` 不在索引内故不出现。
- **验收**:单测×4(成员信号/工程全局面/字符串上下文两面/状态机两函数)+ E2E 探针
  (真实二进制:路径面带 textEdit、全局面含工程名、普通字符串 0 项)+ oracle 零回归。

### P3 — C8 注解参数面(godotlsp)

现状:注解补全已能识别(特征 = label 无 `@` + kind PROPERTY + detail 空);本项扩展到
**注解名后第一参数位**。数据三路:

| 数据 | 来源 | 说明 |
|---|---|---|
| slider 串 / `@rpc` 三组 / easing | 手搬(源码锚点 `gdscript_editor.cpp:1028`) | 小表,注释注明锚点与 pinned commit |
| `@warning_ignore*` 全名 | `godot-docs/gdd_0310` 语料(GDScriptWarning 表) | 与现有注解文档管线同源 |
| `@export_custom` arg0/arg2 | 模型 `global_enums` 的 PropertyHint/PropertyUsageFlags(22 个全局枚举内,实测在) | 模型驱动,零手搬 |
| `@export_node_path` | Node 继承树 + 工程 class_name | 依赖 C7.1 的面 |

- **验收**:oracle `argument_options/` 全桶(connect/play_typed/play_untyped/play_inferred/
  string_literals)——引擎最全的注解参数 oracle,直接量化;`filter/` 桶(organized_export 排除
  "Test Category")顺带验证排噪。
- **取舍**:不做 `@export_tool_button` 图标表(编辑器主题数据,无头不可得)。

### P4 — C9 snippet 对齐与官方模板入库

- **C9.1 插件面 16 → ~40**:按 §3.2 桶补齐(收全部标注桶);保留现有自造前缀(肌肉记忆),
  新增标准前缀双发;来源与许可证注记进 `corpus/snippets/README.md` 与 fetch 脚本。
- **C9.2 `tools/fetch_snippets.py`**:pinned 双仓库 → `corpus/snippets/`(不入库,幂等,
  仿 `fetch_corpus.py`);`--check` 模式输出"union vs ours"缺口清单——**把"对齐"变成可重跑审计**,
  上游更新时一条命令重对齐。
- **C9.3 官方 script_templates 入库**:9 个(Node/CharacterBody2D/CharacterBody3D/EditorPlugin/
  EditorScenePostImport/EditorScript/Object/RichTextEffect/VisualShaderNodeCustom)→
  `editors/vscode/templates/`(MIT,pinned commit + fetch 更新脚本)。**入库判据:产品特性素材,
  clone 即可用**——为 features-roadmap 社区高频诉求「类/文件模板」备料。
- **C9.4 服务器 C2 面小补**:`snippets.rs` COMPOSITES 补 await signal / waitsec(与 C9.1 去重后);
  keyword_snippet 不动。
- **验收**:`--check` 零缺口(除明确不收项);模板 9/9 在位;smoke snippet 面不回退;npm test 不受影响。

## 五、来源治理(克隆与入库判据)

- **未修改上游资产 → 不入库**:fetch 脚本 + pinned commit,仓库只记录来源
  (`.refs/godot`、`corpus/godot-engine-tests/`、`corpus/snippets/` 均走此律)。
- **产品特性素材 → 必须入库**:`editors/vscode/templates/`(官方模板)——
  用户 clone 后「类/文件模板」特性直接可用,分发链不断。
- **自建数据 → 必须入库**:`extension_api.json`(生产 `include_str!`)已在库;
  **`tools/completion_oracle_baseline.json` 同律**(验收基建,clone 即可跑回归;语料本身仍走 fetch)。
- 判据一句话:**用户 clone 之后能否直接跑起来?** 不能,就是分发链断了。

## 六、明确不做(带理由)

1. **外挂 `godot --headless --lsp-port`**:需用户自备引擎 + 全量 import(秒级)+ 编辑器缓存语义,
   违背零引擎依赖定位;只借鉴其数据契约(insertText 括号、kind 映射、scene cache 概念)。
2. **移植 `_guess_*` 千行类型推断**:现有 semantic + 模型覆盖务实面;类型未知降级不出成员
   与引擎行为一致。
3. **字典键补全**:需字面常量折叠,性价比低(oracle `index/const_dictionary_keys` 仅作观察)。
4. **`@export_tool_button` 图标表**:编辑器主题数据,无头不可得。
5. **全文克隆引擎入仓**:目标文件 raw 拉取 + pinned 引用;复用表随消费点入库并注源。

## 七、风险总账

| 风险 | 影响面 | 缓解 |
|---|---|---|
| C6 附加式 serde 解析失败 | 模型加载即崩(测试先炸) | `#[serde(default)]` 全套;单测锁 503/1437 计数 |
| 信号/虚方法沿链收集拖慢补全 | completion p50(基线 8.6ms) | O(链长×成员)可忽略;每期 bench_load 监控 |
| C8 手搬表随引擎版本漂移 | 注解参数过时 | 源码锚点注释 + oracle `argument_options` 桶检测 |
| C7.2 字符串面客户端不触发 | 特性不可见 | 主流客户端默认行为;README 说明;监控后补开关 |
| oracle 语义差异误判 | 验收噪音 | 匹配器规范化 + 分桶趋势判断,不追求 100% |

## 八、开放问题(未来钩子,不在本期)

- **C10 self 成员面**(§3.6 N1,字符串抑制 N2 已于 2026-10-17 随 C7.2 落地):oracle
  证据最充分的新面(common `identifiers_in_*` ×3 等),但涉及全局面合并语义 + 新 sort 桶
  + 与 vacuous pass 的联动,按项目纪律先设计后实施(设计入 lsp-design.md 后再排期);
  与 N4(枚举限定展示)同批。
- **per-type 运算符 hover**(如 `int * Color` 的运算符文档):需语义层对二元表达式两侧做类型推断,
  依赖类型系统完备度;当前通用 `OPERATOR_DOCS` 20 条已覆盖语义科普,per-type 留待语义层增强后
  (与 C6.5c-③ 衔接)。
- **autoload 成员解析**:内核缺口(autoload hover 为 null、成员不存在不报假阴性)——需内核改,
  独立立项(R2 后)。
- **类型感知评分**:前置 semantic 层 `locals_at`(P3.15/C5),之后 completion 排序可吃局部变量。
- **中文语料入 hover/completion**:godot-docs zh-CN(P2-16 i18n 线),与本路线图正交。
- **`filter/` 桶的 organized export 排噪**:`@export_category` 等注解串不进普通标识面——
  C8 实施时顺带对齐(oracle 已覆盖)。
