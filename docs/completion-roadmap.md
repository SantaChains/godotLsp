# 补全增强路线图 — 引擎数据面 × 官方 snippet(2026-10-16 立项)

> **状态:P0/P2/P3 已实施;C7.3/C6.4/C6.6/C6.1/C6.2/C7.1/C7.2/N2/C8(一期)已落地(2026-10-17);
> 2026-10-17 资料盘点轮完成,剩余工作按阻塞簇重排为 P4-P11(见 §四)。**
> 本文件是 C6-C9 四项工作流的排期依据;设计论证与源码锚点见
> `lsp-design.md` §6.9(证据深挖),能力面索引见 `lsp-engine-capabilities-roadmap.md` P3.6。
>
> **定位不变**:零引擎依赖 + 内嵌模型。本路线图只做"数据面补齐、工程面接入、snippet 对齐",
> 不动并发架构、不外挂引擎、不引运行时依赖。
>
> **验收基建已就位**:`tools/completion_oracle.py`(引擎 oracle harness)+
> `tools/completion_oracle_baseline.json`(基线 **36/140**)。每期实施后重跑,
> `--update-baseline` 前必须零回归;分桶通过率即量化验收。

## 一、结论(一屏)

| 期 | 项 | 层 | 内容 | 风险 | 状态 |
|---|---|---|---|---|---|
| P0 | **V0** | 工具 | 引擎补全 oracle harness(140 可判定夹具 = 146 .gd − 6 .notest;量化基线) | 无(纯新增工具) | ✅ 已实施(2026-10-17,首跑 20/140) |
| P1 | **C6** + **C7.3** | gdscript-engine + godotlsp | 模型字段解析+三类消费+内建构造/运算符面(C6.5);$/% 回归止血 | 低(附加式 serde) | 🔶 C6.1/C6.2/C6.4/C6.6 已落地;C6.3/C6.5 待 |
| P1 | **C7.3** | godotlsp | $/% 节点路径上下文止血 | 低 | ✅ 已落地(2026-10-17,单测+E2E) |
| P2 | **C7.1/C7.2** | godotlsp | class_name/autoload 全局面;res:// 路径面 + N2 字符串抑制 | 低-中 | ✅ 已落地(2026-10-17,oracle 24→**33/140**,+9 全为 C6.1 信号面战绩) |
| P3 | **C8** | godotlsp + gdscript-project | 参数选项面(信号名/输入动作/注解参数)+ oracle harness 两处修复 | 低(表漂移可检测) | ✅ 一期已实施(2026-10-17,oracle 33→**36/140**,argument_options 0/11→3/11) |
| P4 | **C6.6b** | godotlsp | 枚举面入口扩展(match 测试位 + `global_enums` 值面 + 全局枚举类型名入全局面 + 单点枚举名解析 + 上下文门控) | 低(数据已在,唯一种全局面增量) | ✅ 已实施(2026-10-17,oracle 36→**58/140**,+22:builtin_enum 5/5 新增、global_enum 3/6→6/6、enum_values_in_match 3/7→7/7、全局枚举类型名入全局面 +22 项、单点枚举名三态解析 + 字典键/类型标注门控) |
| P5 | **C10** | godotlsp | 脚本成员面(簇 A:self/裸标识符/脚本接收者三层成员 + override 签名 + extends 位类名) | 中(全局面合并,设计先行) | ✅ 已实施(2026-10-17,58→75/140,+17) |
| P6 | **C5** | gdscript-syntax | 语义层类型推断(locals_at + 返回类型推断) | 中(内核附加式) | 待 |
| P7 | **C6.3** | gdscript-engine + godotlsp | 虚方法模型化(`is_virtual` 1437 + override 面引擎签名) | 低(附加式 serde) | 待 |
| P8 | **C8 二期** | godotlsp | tween_property 属性面 + InputMap 单例面 + `&`/`^` 设置 plumbing | 低-中 | 待 |
| P9 | **C6.5** | engine + tools | builtin 构造/运算符面(XML×JSON 双源:模型 + 文档 + 构造补全) | 低(无 oracle,单测+文档验收) | 待(可插空) |
| P10 | **N6** | gdscript-project + godotlsp | 场景树(tscn 解析 → 节点名面 + 动画 + 连接) | 高(新解析器) | 待 |
| P11 | **C9** | 插件 + 工具 | snippet 16→~40 对齐 + fetch 审计;templates 来源重定 | 无(纯增量) | 待(可插空) |

**取舍主线**:先建能量化的验收基建(V0),再做零行为风险的模型层(C6),回归修复(C7.3)随 P1 止血;
**2026-10-17 资料盘点后按阻塞簇重排(P4-P11):数据就绪度(入口扩展 C6.6b)> 最大证据簇(C10 脚本成员面)> 共同前置(C5 类型推断)> 模型化(C6.3)> 剩余面(C8b/C6.5)> 最大工程(N6);工具面(C9)不锁主线可插空**;
每项独立可回退、独立可组合(遵循项目一贯的开关纪律)。

## 二、证据基线(全部本地化,可复验)

| 源 | pinned | 本地位置 | 入库? | 用途 |
|---|---|---|---|---|
| Godot 引擎全量仓库 | `5b4e0cb0`(2026-06-17) | `.refs/godot/`(gitignore) | 否 | `modules/gdscript/` 全部源码锚点;**`doc/classes/` 810 XML** |
| 引擎自带补全测试 | `9a05d57` | `corpus/godot-engine-tests/completion/` | 否(`fetch_corpus.py` 幂等) | **oracle**:146 `.gd`(➡ 光标)+ 143 `.cfg`(include/exclude)+ 2 `.tscn`,14 个分类 |
| godot-vscode-plugin | `145a0f0`(2026-07-01,36 条) | `corpus/snippets/vscode/`(规划) | 否(`fetch_snippets.py` 规划) | 官方 snippet 基准(MIT) |
| gdscript-zed-snippets | `eacdbf0`(2025-11-01,34 条) | `corpus/snippets/zed/`(规划) | 否(同上) | Zed 向 snippet 基准(MIT),与我方多 IDE 定位相关 |
| extension_api.json | Godot 4.7.0 stable dump | `crates/gdscript-engine/testdata/` | 是(`include_str!` 生产数据) | 引擎模型唯一事实源 |
| 文档站/hover 生成面 | 随语料 | `tools/gen_docs.py` → `website/src/builtin/class_*.md` + `crates/godotlsp/src/hoverdata.json` | 是(生成物) | 类描述/方法/属性/枚举/常量 hover 与文档站;**Constructor/Operator 区块未解析(见 §3.5)** |

**2026-10-17 资料盘点结论(全部本地可验,纠正两处历史假设)**:

| 资料 | 假设 | 实测 | 影响 |
|---|---|---|---|
| `.refs/godot` 内 script_templates | "9 个模板在仓" | **不存在**——全仓唯一 `.gd` 是 `tests/scripts/utils.notest.gd`;`editor/` 源码无 `script_templates` 引用(Godot 4 模板走编辑器安装目录/用户目录,不在源码仓) | C9.3 来源重定(见 §四 P11),不得再称"入库官方 9 模板" |
| 引擎内置 snippet 表 | "可复用内部表" | **不存在**——GDScript 编辑器无 snippet 库(snippet 功能是用户自定义 EditorSettings;`editor/` 仅 `editor_help.cpp` 文档命中) | C9 基准锁定 vscode + zed 两家上游,已是定论 |
| `doc/classes/int.xml` 构造/运算符 | (未查) | **存在且完整**:`<constructors>` 5 个 + `<operators>` 64 个,均带 `param name/type` + `return` + `description` | **C6.5 签名+描述源落地**:extension_api 只有 `constructors[].index`/`operators[]{name,right_type,return_type}`(无描述),两者互补(见 §3.5) |
| `godot-docs/gdd_1559_int.md` | (未查) | Constructors/Operators 区块在(与 XML 同源英文) | C6.5b 备选源,与 XML 二选一即够 |
| `godot-docs/gdd_0310` warning 表 | 路线图原计划取此 | **无警告名表**(0 个可匹配 token) | C8 已改取引擎 `gdscript_warning.h`(46 名,行为事实源),偏离已记录 |
| `corpus/snippets/` | C9.2 规划 | **不存在**(fetch 脚本未建) | C9.2 = 全新基建,零启动成本 |

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

**2026-10-17 数据源补充盘点**(C6.5 实施前定论):
- `extension_api.json` builtin `constructors` **只有 `index`**(无参数签名)、`operators` 只有
  `name/right_type/return_type`(无描述)——单靠 JSON 不够。
- `.refs/godot/doc/classes/int.xml` 实测:`<constructors>` 5 个 + `<operators>` 64 个,
  **均带 `param name/type` + `return` + `description`**(810 XML 全类同构,已抽样验证)。
- **定论:C6.5 = XML(签名+描述)× JSON(结构/索引)双源互补**;英文语料,中文翻译走 P2-16 i18n 线。

### 3.6 oracle 普查差距地图(2026-10-17 四轮:20/140 → 24/140 → 33/140 → **36/140**)

分桶通过率(当期 36/140):filter 2/2、script_enum 1/1、builtin_enum 5/5、common 8/20、
global_enum 3/6、index 1/9、enum_values_in_dictionary 1/13、types 2/19、get_node 8/33、
**argument_options 3/11**(C8 一期:connect/dont_add_string_name/prepended 三夹具)、
no_parenthesis 2/8、enum_values_in_match 0/7、assignment_options 0/4、
extends 0/2。未命中成因:absent 279 / paren-display 50 / string-literal 7。

**按阻塞簇归并(2026-10-17 资料盘点轮重排依据,夹具计数为 verbose 失败明细实测)**:

| 簇 | 共同阻塞 | 夹具(约) | 明细 |
|---|---|---|---|
| **A 脚本成员面** | 裸标识符/self/脚本接收者位不出"自声明 + 脚本父类链 + 引擎链"三层成员;`func ` 位不出可重载签名 | **~30** | common: `identifiers_in_*` ×3 + `self.gd` + `invalid_super_call_*` ×2 + `override_function_*` ×6 + `infer_return_type_without_value`;extends: `extends_self`/`extends_inherited_inner_class`(兄弟内部类);types: local ×6 + member ×5(typehint/inferred 的 `add_child(…)`/`owner`/`child_entered_tree` + `property_of_a`);get_node: `member_typehint*` ×3;assignment: `enum_identifier` ×2(部分) |
| **B 枚举面入口** | C6.6 已有枚举值数据,缺入口:match 测试位、`global_enums` 接收者、单点枚举名解析、上下文门控 | **~16(实际 +22)** | `enum_values_in_match` ×7(`match AutoTranslateMode.➡` → 值,3/7→7/7);`global_enum` ×3(`Key.➡` → `KEY_A/KEY_B` 值面 + `Key`/`Error`/`KeyLocation` 类型名,3/6→6/6);`builtin_enum` ×5(`Class.Enum.` 两段式,新增);单点枚举名三态解析 + 字典键/类型标注门控 |
| **C 语义层类型推断** | semantic 无 locals 查询:局部变量(注解/字面量推断)/返回类型未知 → 不出成员 | **~9** | `index` ×6(`array/dictionary/untyped` 的 `outer`/`inner` 元素成员);`types/local/no_type` ×2 + `inferred`(`begins_with(…)`);`common/infer_return_type_without_value` |
| **D callable 上下文** | Callable 形参位出函数名不带括号(需形参类型识别) | **6** | `no_parenthesis_when_callable_is_expected` ×6(`hello_world`) |
| **E 场景树/节点名** | tscn 未解析:节点路径不出场景子节点名;动画面不出动画名 | **~25** | `get_node` 余 25(成员面 3 个归簇 A 后剩 ~22 节点名面);`argument_options/play_*` ×2(缺 `"bounce"` 动画名) |
| **F 观察项** | — | 2 | `index/*dictionary_keys` ×2(N7 明确不做) |

**簇间依赖**:C 是 A/D/C8二期的共同前置(locals 查询);但 A 的主体(自声明+引擎链成员)**不依赖 C**——
类型提示已知的接收者(脚本类/引擎类)即可跑通,字面量推断留给 C。B 完全不依赖 A/C(数据已在)。

历史发现编号(保留索引,归并关系如下):
| # | 发现 | oracle 证据 | 处置 |
|---|---|---|---|
| N1 | ~~**self 成员面缺失**~~ | common `identifiers_in_*` ×3、`self.gd`、`invalid_super_call_*` ×2 | ✅ 已落地(P5,2026-10-17):三层成员合并 + `receiver_word` self./super. 短路 + 局部作用域面;oracle self/identifiers/super 六夹具全过 |
| N2 | **字符串上下文不抑制** ~~:字符串字面量内仍出全局面~~ | `no_completion_in_string`(曾为 vacuous pass) | ✅ 已落地(2026-10-17,C7.2 同批):`string_literal_at` 前向状态机,字符串内返回空(路径面例外);vacuous pass 转为真实通过 |
| N3 | ~~**脚本类成员面缺失**~~ | common `invalid_super_call_*` ×2、`override_function_*` ×4 | ✅ 已落地(P5,2026-10-17):`script_base_key` 支持 Ident/字符串路径 extends;override 签名引擎虚方法部分归 P7(`_get`/`_static_init`) |
| N4 | **枚举限定展示**:类型化 LHS 后引擎出 `Control.SIZE_EXPAND`(限定名),我方不出类枚举值 | assignment_options ×4 | ✅ 已随 P4 簇 B 落地:`enum_member_values_at` 枚举值面(含 `Class.Enum.` 限定名形式) + 上下文门控;assignment_options 的 `new(…)`/限定名展示属 C8 二期范围,本簇只覆盖枚举值入口 |
| N5 | **callable-expected 上下文不出括号** | no_parenthesis ×8(当前 0/8,C6.4 后仍挂) | **簇 D**:需形参类型识别(C6.4 后仅剩识别层,工作量低于原估) |
| N6 | **节点路径类型推断 + 场景树节点名** | get_node 0/33 | **簇 E**(成员面部分归簇 A):语义层 + C7.3+(project.scenes → NodeDecl),最大工程,末期 |
| N7 | **字典键/字典内枚举值** | index 1/9、enum_values_in_dictionary 1/13 | 明确不做 #3 延续,仅观察 |
| N8 | **字符串字面量参数面**(NodePath/StringName 建议) | argument_options/string_literals ×8 | ✅ C8 一期:输入动作面(默认设置侧,3/8 过——`dont_add_*` 两口 + prepended 一口);tween_property 属性面(前置簇 C)与 AnimationPlayer 动画面(前置簇 E)缓行;`add_*` 两口为设置 plumbing |

**vacuous pass 清单(补面时必须回归验证)**:`no_constructor_*`(6)、
`builtin_enum/global_enum/script_enum` 的 metatype 三件套。`no_completion_in_string`
已于 2026-10-17 随 N2 落地转为真实通过。oracle 的 exclude 语义会在对应面补齐后自动
翻转这些桶——这正是 harness 的价值。

## 四、分期路线图

### 排期总表(2026-10-17 资料盘点轮重排)

| 期 | 项 | 层 | 内容 | oracle 预期 | 前置 | 状态 |
|---|---|---|---|---|---|---|
| P4 | **C6.6b** | godotlsp | 枚举面入口扩展:match 测试位 + `global_enums` 值面 + 全局枚举类型名入全局面 + 单点枚举名三态解析 + 上下文门控 | **+22**(实际:builtin_enum +5、global_enum +3、enum_values_in_match +4、全局枚举名 +22 入全局面、单点解析 + 门控消除 2 回归) | 无(数据已在) | ✅ 已实施(2026-10-17,oracle 36→58/140) |
| P5 | **C10** | godotlsp + gdscript-project | 脚本成员面(簇 A):self/super/裸标识符三层成员 + `func ` 位 override 签名(脚本父类部分) + extends 位类名 + 局部作用域面;`Project::script_chain_members`/`script_engine_base_by_path` 列表能力 | **+17 实际**(58→75;common 8→16、extends 0→2、index 1→7) | ✅ 已实施(2026-10-17);引擎虚方法签名归 P7 | 完成 |
| P6 | **C5** | gdscript-syntax | 语义层类型推断:locals_at(注解 + 字面量)+ 返回类型推断 | **+9~12**(index 6 + types 3 + infer_return) | P5 数据面(部分) | 待 |
| P7 | **C6.3** | gdscript-engine + godotlsp | 虚方法模型化:`is_virtual` 解析 + override 面引擎虚方法完整签名 | override 剩余夹具 | P5 override 面 | 待 |
| P8 | **C8 二期** | godotlsp | tween_property 属性面 + InputMap 单例面 + `&`/`^` 设置 plumbing | `add_*` ×2(+动画 ×2 挂 P10) | P6 | 待 |
| P9 | **C6.5** | engine + tools | builtin 构造/运算符面(XML×JSON 双源):模型解析 + 文档面 + 构造补全 | 无 oracle(单测+文档验收) | 无(可插空) | 待 |
| P10 | **N6** | gdscript-project + godotlsp | 场景树:tscn 解析 → NodeDecl → 节点名面 + 场景信号 + 动画面 | **+24**(get_node 22 + play_* 2) | 大工程,独立 | 待 |
| P11 | **C9** | 插件 + 工具 | snippet 16→~40 对齐 + fetch 审计;templates 来源重定 | 无(工具面) | 无(可插空) | 待 |

**重排原则(实践最佳序的判据)**:
1. **数据就绪度优先**:B 的数据(C6.6 枚举值 + `global_enums`)已在模型里,只缺入口 → 最先行;
   A 的数据(脚本 AST + 引擎链 + `global_enums`)也全在,但动全局面合并(最大回归面)→ 设计后行。
2. **共同阻塞优先**:C(locals 查询)是 A 二期/C8b/N5 的共同前置,排 P6;A 主体不依赖 C,先 A。
3. **风险梯度**:B(入口扩展,零全局面污染)< C(内核附加式)< A(全局面合并,需 sort 桶设计 +
   vacuous pass 联动)< E(全新 tscn 解析器,最大工程故末期)。
4. **独立可插空**:C9(工具面)与 C6.5(文档面,无 oracle 覆盖)不锁主线,排后但可随时插空;
   全局面回归每期用 `filter` 桶(2/2)+ E2E 探针守。
5. **每期独立 commit,oracle 零回归才 `--update-baseline`**(既有纪律不变)。

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

### P3 — C8 参数选项面 ✅ 一期已实施(2026-10-17)

落点:`crates/godotlsp/src/argopts.rs`(新模块,前向括号帧扫描 + 面表)+ `main.rs`
`completion()` 分支插入(节点路径面之后、字符串抑制之前)+ `gdscript-project`
`[input]` 解析(`input_actions()`)+ `tests/fixtures/projface/project.godot` 补
`[input] test_input_action`。

**语义(全部有引擎锚点,模块头注)**:
- **独占面**:CALL_ARGUMENTS/ANNOTATION_ARGUMENTS 上下文只出选项;调用面零项时
  回落常规标识面(`r_forced = size > 0`,`.refs/godot/modules/gdscript/gdscript_editor.cpp:3432`),注解参数面
  forced 独占(:3474,零项也拦)。
- **默认设置侧**:`use_single_quotes`/`add_string_name_literals`/`add_node_path_literals`
  引擎默认均 false(`.refs/godot/editor/settings/editor_settings.cpp:868-872`)→ 一律双引号、不发 `&`/`^` 前缀。
  oracle `add_*`(设置开)与 `dont_*`(设置关)成对出现,本期对齐默认侧;设置
  plumbing 不入本期。
- **既有字面量整串替换**:光标在闭合字符串内 → textEdit 替换整串(保留 `&`/`^`
  前缀,镜像 `_calculate_string_insertion` final_code),insertText 为裸引号串。

**一期落地三面**:
1. **信号名面**(`connect`/`is_connected`/`disconnect`/`emit_signal`/`has_signal`
   首参):接收者 = self(裸调用,extends 链引擎信号 + 本文件 extends 脚本链自声明)
   或已解析类型(引擎类链 / 工程脚本类);oracle `connect.gd` 转 pass。
2. **输入动作面**(InputEvent 系接收者 + 方法名含 `action` + 首参):项目
   `[input]` 动作名;oracle `dont_add_string_name_input_event` +
   `add_string_name_input_event_prepended` 转 pass(后者走 `&"…"` 整串替换)。
3. **注解参数面**:`@export_range` 参 3-5(slider 四串)、`@export_exp_easing` 参 0-1、
   `@rpc` 参 0-2(七选项)、`@warning_ignore*`(46 个警告名,**取自引擎
   gdscript_warning.h 枚举而非 docs 语料——本地 gdd_0310 页无警告名表,不可验证;
   引擎头文件是补全行为的事实源,pinned @5b4e0cb0**)、`@export_custom` 参 0/2
   (模型 `global_enums` 的 PropertyHint 45 / PropertyUsageFlags 32,零手搬)、
   `@export_node_path` 参 0(Node + 引擎继承树 + 工程 Node 系脚本类)。无 oracle
   覆盖(语料无注解实参位夹具),以单测锁定。

**oracle harness 两处修复**(C8.a,均为基建缺口非产品问题):
- `build_workspace` 此前写最小 project.godot(无 `[input]`),输入动作面永远缺数据;
  改为优先采用语料根 project.godot(`completion/` 与其父级都探)。
- 匹配器容许 include 超集语义不变;基线 33→36 且零回归后才 `--update-baseline`。

**二期(明确缓行,记入开放问题)**:
- `Tween.tween_property` 次参属性面:需实参 0 的局部变量类型推断(semantic
  `Analysis` 未暴露 locals 查询,`FileInfo::resolve_var` 只覆盖类级成员)。
- `AnimationPlayer.play` 等动画面:需 tscn AnimationLibrary 解析 + 接收者推断,归 N6 家族。
- `InputMap` 单例方法动作面(`InputMap.has_action(` 等):无 oracle 覆盖。
- `&`/`^` 前缀设置 plumbing(oracle `add_*` 夹具另一半):需 workspace/configuration 通道。

### P4 — C6.6b 枚举面入口扩展(godotlsp,数据就绪度最高 → 最先行) ✅ 已实施(2026-10-17)

**为什么最先**:三个面的数据都已在模型里(C6.6 `enum_member_values_at` 的枚举值解析 +
`EngineApi.global_enums`),缺的只是入口识别与展示约定,零全局面污染(除全局枚举类型名
+22 项,见下),是剩余工作中风险最低、oracle 回报最快的一项(+7~10 预期,实际 +22)。

**实际落地范围**(比原三项规划更广,因实现时一并覆盖单点枚举名解析与上下文门控):
1. **match 测试位枚举值**(`enum_values_in_match` ×7):`match X.➡` / `when X.➡` 的测试
   表达式位 → `after_dot` 分支枚举解析优先于成员面(引擎语义:`_code_completion` 先解析枚举
   常量再回落成员),`enum_values_in_match` 3/7→**7/7**。
2. **`global_enums` 接收者值面 + 两段式 `Class.Enum.`**(`global_enum` 6/6 + `builtin_enum`
   5/5 新增):`Key.➡` → `KEY_A`/`KEY_B`;`Vector2.Axis.X` / `AStarGrid2D.Heuristic.HEURISTIC_MAX`
   经类名查 `EngineClass.enums`/`Builtin.enums`;接收者名查 `engine.global_enums`(22 个)。
3. **全局枚举类型名入全局面**(`global_enum_identifier` 等):`Key`/`Error`/`KeyLocation` 等
   22 个枚举名进裸标识符全局面(kind ENUM_MEMBER,detail `@GlobalScope 枚举`);`extends <|>`
   位除外(引擎 extends 只出类名)。
4. **单点枚举名三态解析**(被 1/2 复用):脚本自声明枚举(顶层 `enum` + 类级 `class{enum}`)→
   宿主 extends 链类枚举(`extends Node` → `AutoTranslateMode.`)→ 引擎全局枚举;脚本枚举
   值未显式赋值时按声明序递增值(`const_int_value` 常量折叠 int 域)。
5. **枚举值面上下文门控**(`enum_value_completion_allowed`):字典键(`{ Enum. = }`,光标后
   首非空白为 `=`)与类型标注(`var x: Enum.`,独立词关键字 `var/const/signal/...`)抑制枚举
   值面,回落 `recv_type_at`/`class_members`(消除 `index_1_local_enum`/`lua_key_2` 两回归)。

- **取舍**:match 位只做测试表达式(不做 `match` 本体后的分支体);全局枚举值面不做
  `@GlobalScope.Key.` 全限定形式(引擎也不出);枚举值面门控严格对齐引擎"值/表达式才出枚举值"。
- **验收**:oracle **58/140**(基线 36,+22,回归 0);`builtin_enum` 5/5、`global_enum` 6/6、
  `enum_values_in_match` 7/7、`script_enum` 1/1 全绿;`filter` 2/2 不回退;workspace 48 单测
  全绿;smoke ALL OK;clippy 零新增。

### P5 — C10 脚本成员面(簇 A) ✅ 已实施(2026-10-17,58→75/140,+17 零回归)

**规格来源(代码实证,2026-10-17 复核)**:`common/self.gd` 一个夹具定义三层成员面契约;
`common/override_function_*` 定义 override 签名面;`extends/extends_inherited_inner_class.gd`
定义 extends 位类名面。全部 include 清单 = 规格。

**实际落地(与设计差异以本节为准)**:

- **gdscript-syntax**:`span_slice`(反转/越界钳制 + UTF-8 边界)、`expr_span`/`expr_to_text`、
  `FuncDecl::signature_text`(参数整段 `Param.span` 源切 + 空白折叠)/`signature_text_raw`
  (源码原样——override display 引擎行为,oracle 以 `call := func():\n\tvar x_of_a = 10`
  多行原文锁定)。**解析器修复**:`Param.span` 原取 `span_of(self.peek())`(指向下一 token),
  改为参数自身区间(无消费者依赖旧语义)。
- **gdscript-project**:`Script` 加 `src`;`ScriptMemberDecl{name,kind:DeclKind,is_static,
  signature}`(**自有 `DeclKind`,不与语义层 `ScriptMember` 耦合;区分 Method/Property/Const/
  Signal/Enum/Class,内部类不递归展开**)、`script_chain_members(key)`(近→远收集,近层同名
  先到先得,depth≤16 环防御)、`script_base_key`(放宽:**引擎基类名也返回**,Ident/字符串路径
  extends 双支持——`extends "res://…"` 此前断链是 N3 根因)、`script_base_key_by_path`、
  `script_engine_base_by_path`(链尾引擎基类 → 层③起点)。
- **godotlsp**:`script_self_members`(三层合并:①live AST 本脚本自声明 → ②脚本父链 → ③
  引擎链 `class_members`;seen 先到先得即优先级;`include_live=false` 即 `super.` 位)、
  `receiver_word`(self./super. 识别——`recv_type_at` 不识别关键字,设计原案的 recv_type_at
  扩展改为 after_dot 分支前置短路)、`override_signatures`(func/static func 位;**不依赖
  snippet 协商**;`insert_text = display = 签名原样`;光标在内部类内走该类 extends——
  `override_function_abstract` 顶层 `class B extends A` 形态)、`extends_inner_class_items`
  (兄弟内部类 + 父链内部类,排除光标所在类自身)、`local_scope_names`(光标所在函数形参 +
  body 内 var/const 声明,含嵌套块)。
- **span 放宽**:`enclosing_class_name`(body 最后一个 stmt 到文件尾)——不完整行尾
  `func `/`extends ` 待补时 at 超过解析器 span 终点(override_function_abstract 因此
  差 1 字节漏判,统一修复 override/extends/局部三处)。
- **scoring**:新增 `SELF = 0x08` 桶(裸标识符面的脚本成员;self./super. 位由 after_dot
  收尾统一 MEMBER;override 面用 SNIPPET 桶)。
- **oracle harness 布局修复(基建)**:语料镜像从平铺 ws 根改为 `ws/completion/` 子目录、
  夹具 uri 对齐——夹具源内 `res://completion/…` 引用(extends/scene=)从此真实可解析;
  此前脚本链与场景路径全部断链(修后 index 桶 1→7/9 连带改善)。
- **oracle 运行纪律**:`--server target-bench/debug/godotlsp.exe` 必带——默认找
  `target/debug`(被外部 LSP 进程锁定的旧二进制),曾致"0 改善"假象。

**规格细节(设计阶段实证,保留备查)**:

**三层成员模型**(单一事实源:gdscript-project 已具备按名查基础设施,缺"列表"变体——见下):

| 层 | 数据 | display 约定 |
|---|---|---|
| 本脚本自声明 | `file.body` 顶层 `Var/Const/Func/Signal`(主类成员;内部 `Class`/`Enum` 另行作类型名暴露) | 函数 `name(…)`(C6.4);信号裸名;属性/常量裸名 |
| 脚本父类链 | `extends "res://…"`/`class_name` → 沿 `Project::class_names` 链走 `Script.ast.body`,命中引擎基类名止步 | 同左(`func_of_a()`/`property_of_a`/`signal_of_a`) |
| 引擎链 | 最终引擎基类 → `engine.ancestor_chain`(复用 C6.1/6.2 `class_members`) | 同左(`add_child(…)`/`owner`/`child_entered_tree`) |

**入口族**:
1. `self.➡`:`recv_type_at` 识别 `self` → 当前脚本三层合并(当前实现 `resolve_var` 不识别 `self`,故现在 `self.` 空——根因已定位);
2. `super.➡`:识别 `super` → 父脚本类型(从 `file.extends` 解析)→ 自 layer2 起三层合并(`invalid_super_call_1/2`);
3. 裸标识符位(函数体/调用实参):局部(参数/`var`/`const`)+ 三层类成员 + 既有全局面(`identifiers_in_*`);
4. `func ➡` / `static func ➡` 位(override 面):可重载方法**签名**(脚本父类方法取 AST,P7 补引擎虚方法);
5. `class X extends ➡` 位:兄弟内部类(`extends_self` exclude 自身)+ class_name 脚本类 + 引擎类。

**实施设计(代码实证,2026-10-17 据 gdscript-project/src/lib.rs + semantic.rs 复核)**:

- **复用已有**:`Project::walk_member_kind/walk_member_type`(按名查,已处理脚本链+引擎止步)、
  `script_extends_base`、`class_names`、`script_by_key`、`ScriptMember{Method,Property,Const,Signal}`。
  `main.rs` 已直接持有 `state.project`(消费 `proj.scripts`+`proj.autoloads()`)。
- **新增 `Project` 公开方法(列表变体)**:
  ```rust
  pub struct ScriptMemberDecl {
      pub name: String,
      pub kind: ScriptMember,            // Method/Property/Const/Signal
      pub is_static: bool,
      pub signature: Option<String>,     // Method: "name(p: T, q := def) -> R"(AST 渲染,无 body);其余 None
      pub origin: String,                // class_name / "script:<path>" / "engine:<Class>"
  }
  /// 沿脚本 extends 链收集完整成员列表(depth ≤16 环防御);命中引擎基类名即止(引擎层由 class_members 补)。
  pub fn script_chain_members(&self, key: &str) -> Vec<ScriptMemberDecl>;
  ```
  实现 = 在 `walk_member_kind` 的遍历骨架上改为"收集每层全部成员"而非"按名命中即返",并回填
  `signature`(由 `FuncDecl.params`+`ret_ty` 经 AST 渲染;默认值为 `expr_to_text` 源切 + 空白折叠,
  处理 `func_of_a_callable` 的 `call := func(): var x_of_a = 10` 跨行默认值——**最棘手的子点**)。
- **`main.rs` 新增 `script_self_members(state, uri, src, &file) -> Vec<CompletionItem>`**:
  layer1 用 LIVE `file`(避免与索引时差);layer2+3 调 `proj.script_chain_members(key)`
  (key = `file.class_name` 或 `script:<path>`),其返回的引擎基类名再接 `class_members(engine, base)`。
- **`recv_type_at` 扩展**:`self`/`super` 关键字短路 → 分别解析为"当前脚本三层类型键"/"父脚本类型键",
  复用 `script_chain_members` 取引擎基类名喂 `class_members`。
- **override 签名面**:`func ` 位(prefix 匹配)→ 自 `script_chain_members`(非 static 桶)+ 引擎虚方法
  (P7 回填)生成 `name(params):` / `name(params) -> R:` 标签(源码形态,非 C6.4 的 `name(…)` 缩写——
  引擎 override 完成行为);`static func ` 只取 `is_static` 项,排除 `_static_init`(引擎 :3738 特判)。

**设计要点(实施前入 lsp-design.md)**:
- **sort 桶**:新增 `SELF`/`OVERRIDE` 桶(`scoring.rs`),序 = LOCAL < SELF/OVERRIDE < MEMBER < GLOBAL;
  与频率表(fuzzy)正交,不破坏 C6.4 查表剥后缀逻辑。
- **合并规则**:同名优先级 = 局部变量 > 本脚本自声明 > 脚本父类链(近到远)> 引擎链 > 全局面;
  每层内去重;全局面与类成员同名时类成员胜出(引擎行为)。
- **vacuous pass 联动**:`no_constructor_*`(6)+ metatype 三件套必须同批回归(§3.6 清单)。
- **回归守位**:`filter` 桶 2/2 + E2E 全局面计数 + `types/member` 桶不回退。

**开放决策点(待拍板,实施前)**:
1. `script_chain_members` 落点:新增为 `Project` 公开方法(推荐,godotlsp 已持 `proj` 引用)
   vs 加到 `ProjectContext` trait(语义更解耦,但 godotlsp 当前未走 trait 消费脚本列表)。
2. override 引擎虚方法签名:本 P5 先做脚本父类部分 + 基础设施;引擎虚方法(`_get` 等,需
   `is_virtual` 1437)严格归 **P7** 回填(当前 `RawApi` 其余字段未解析,见 MEMORY)——还是本 P5 用
   既有 `snippets::virtual_items` 近似覆盖(会偏离 `name(params) -> R:` 精确形态)?**建议前者**。
3. 内部类(`InnerA`)/命名枚举(`EnumOfA`)暴露口径:oracle 暂无强约束,先保守(命名枚举名 +
   内部类名作类型名暴露,与 `walk_member_kind` 现有口径一致);`extends_inherited_inner_class` 的
   `InnerClassA` 由 extends 位面单独覆盖(见入口 5),不进成员面。
4. 成员面是否出虚方法(`_init`/`_ready` 等):引擎成员面不出(只在 override 面出),`class_members`
   已按引擎 `methods` 表全出——需确认是否要按 `_` 前缀在成员面过滤(`_init` 等用户极少在 `.` 后调)。
   建议:成员面全出(与引擎 `methods` 表一致),不额外过滤。

- **验收(实际)**:oracle **58→75/140(+17,零回归)**——common 8→16/20(self/identifiers ×3/
  invalid_super ×2/override no_underscore·underscore·abstract)、extends 0→2/2、index 1→7/9
  (布局修复连带);单测锁三层合并/godotlsp 5 项 + gdscript-project 3 项;smoke ALL OK;
  零回归后基线已更新。**剩余归界**:`override_function_full_name`(_get,引擎虚方法)与
  `override_function_static` 的 `_static_init` → **P7 C6.3**;`infer_return_type_without_value`
  → **P6 C5**;get_node 22 → **P10 N6**;types 17 → P6/P5 二期(preload 常量类型 + 内部类
  作用域类型面)。

### P6 — C5 语义层类型推断(gdscript-syntax,附加式)✅ 已实施(2026-10-17)

**解锁面**:`index` 桶元素成员(`outer`/`inner`)、`types/local/no_type`(`begins_with(…)`)、
`infer_return_type_without_value`,并是 C8 二期 tween_property 与 N5(callable 形参识别)的前置。

- **C5.1 `Analysis::local_types(offset)`(或等价查询)**:最内层 func 的形参 + `var` 声明
  类型表:注解直取;无注解做**字面量推断**(int/float/String/bool/`Vector2(…)`/`Array[…]`/
  `Dictionary`/构造调用);当前 `FileInfo::resolve_var` 只覆盖类级成员,本查询补函数级。
- **C5.2 返回类型推断**:`func_return` 扩展——无返回注解时收集 `return <expr>` 字面量类型
  (与 C5.1 同源推断器);`infer_return_type_without_value` 即此面。
- **C5.3 消费**:`recv_type_at` 增加局部变量名解析(成员面);N5 分支:实参位形参类型含
  `Callable` → 该标识位出函数名**不带括号**(no_parenthesis 语义)。

- **取舍**:只做单层字面量推断(不做跨语句数据流、不做集合元素类型下推——`index` 桶的
  `array_value` 需元素类型,取 `Array[int]` 注解或字面量首元素);推断失败静默降级(不出成员,
  与引擎"类型未知"行为一致)。

**实施落点(实际)**:
- **`gdscript-syntax/src/infer.rs`(新模块,静默推断器)**:`InferTy` 三态
  (`Class(String)`/`ScriptPath(String)`/`Unknown`)+ `infer_expr_ty`(字面量/构造调用/utility
  返回/`A.new()` Attribute 分支/preload·load)+ `type_annotation_ty` + `var_decl_types` +
  `func_return_inferred`(递归收集嵌套 return,全一致才给类型)+ `preload_consts` +
  `ret_from_signature`;与 semantic.rs 诊断分析解耦,失败一律 Unknown 静默降级;11 单测。
- **D1 子类收窄**:`decl_infer_ty` + `is_subtype`(root 闭包统一 ScriptPath/Class 根解析)——
  初始值类型 is-a 注解类型时用初始值(`var test: Node = A.new()` 出 A 成员);脚本类型根 =
  `script_engine_base_by_path`,引擎类型沿 ancestor_chain。
- **D2 类枚举名**:764 个 native 类枚举(如 Object.ConnectFlags,不在 global_enums)在类型
  标注位(`: word$` / `-> word$`,冒号/箭头到词首必须**同行**——防 `func f():` 块冒号跨行误判)
  专属追加;数据源 `tools/gen_internal_props.py` 同批引擎语料。
- **D3 类型成员面**:`var x: R.➡` 经 `type_member_items` 逐段下钻(Src::{Local, ScriptPath,
  Engine}),出内部类/枚举/const + 引擎链枚举/常量,尾部追加全局枚举+764 类枚举名;path 无
  类型源返回 None 回落原链(保 P4 枚举值面门控,py_value 回归防线)。
- **D4 N5 Callable 面**:`argopts::callable_arg_items`——实参位形参类型 callable → 裸方法名
  (insert 无括号);非 Callable 位(本文件方法)出精确 `name()` 形态;**置于 argument_options
  之后兜底** + `site.literal` 有值先退(字符串字面量/动作面不被抢)。3 单测。
- **`RecvType` 统一消费形态**:`Engine(String)`/`Script { res_path, inner }`;
  `resolve_name_type` 解析链:局部作用域(span<at)→ 类级+顶层 → preload 常量递归 →
  内部类名 → 全局 class_name → 引擎类。
- **internal 属性排除**:引擎 PROPERTY_USAGE_INTERNAL 成员(146 处,含 Translation.messages)
  从源码 ADD_PROPERTY 提取(`tools/gen_internal_props.py` → `internal_props.json`,45 类/80
  属性)入库 `gdscript-engine/testdata/`,`class_members` 按名过滤。

- **验收(实际)**:oracle **75→99/140(+24,零回归)**——types/local 8、types/member 5、
  types/hints 8、infer_return 1、no_parenthesis 3/6(其中 2 个 variant 形态差异)+ C8 一期
  argument_options 0/11→3/11;workspace 126+ 测试全绿(infer.rs 11 + argopts N5 3 新测试);
  clippy 0 warning;smoke ALL OK;零回归后基线已更新(99/140)。**剩余归界**:
  `no_parenthesis_when_callable_is_expected` 的 `builtin_complete_braces`/`builtin_incomplete_braces`/
  `native` 三夹具依赖「未闭合调用参数列表把后续 `func` 声明吞进实参」的**解析器层修复**
  (根因在 gdscript-syntax 词法器括号计数抑制缩进标记,AST 层恢复无效已回滚)——独立债记 §3.6;
  `index/const_dictionary_keys`、`local_dictionary_keys` 维持不做(roadmap 既定);
  get_node 22 夹具归 **P10 N6**。

### P7 — C6.3 虚方法模型化(gdscript-engine + godotlsp)✅ 已实施(2026-10-17)

- **解析**:`RawMethod` 附加 `is_virtual`(JSON `methods[].is_virtual` 实测在,1437 个)+
  `EngineMethod.is_virtual`;`EngineApi::chain_virtuals(chain)` 替代 `snippets.rs` 手维护
  VIRTUALS 12 条(保留作对照测试基准);9 个非 ClassDB 回调(_init/_notification/_get/_set/
  _to_string/_property_can_revert/_property_get_revert/_get_property_list/_validate_property)
  留手表——引擎侧同样是特判(源码锚点 :3735 注释 "Not truly a virtual method")。
- **override 面补全**:P5 的 `func ` 位签名面,引擎虚方法部分换模型源(参数名/类型/返回值
  ——extension_api `arguments` 有 name/type,默认值无;脚本父类方法默认值取 AST,已由 P5 覆盖);
  `_static_init` 排除规则精读引擎 :4117-4119(override 面虚拟方法表的第二参数语义)后落单测。
- **验收**:oracle override 剩余夹具(`override_function_full_name` 的 `_get(property: StringName)
  -> Variant:` 等)+ 单测锁 is_virtual 计数(1437)+ VIRTUALS→模型切换零 diff(对照测试)。

**实施落点(实际,2026-10-17)**:
- **`Function.is_virtual`**(engine):RawFunction serde 透传,`into_fn` 接线;1437 计数单测锚定。
- **`EngineApi::chain_virtuals(class)`**:沿 ancestor_chain 收 is_virtual,**同名先到先得**
  (镜像引擎 `options.has(mi.name)` 名字去重;Node._process 胜 MainLoop._process,单测锁)。
- **`OBJECT_CORE_VIRTUALS` 12 条**(engine 常量表,决策点拍板):object.cpp:1919-1979
  `ClassDB::add_virtual_method` 手工注册、**不在 extension_api** 的编辑器侧虚方法
  (_init/_to_string/_notification/_set/_get/_get_property_list/_validate_property/
  _property_can_revert/_property_get_revert/_iter_init/_iter_next/_iter_get)——roadmap 原
  记 9 个漏了 `_iter_*` 3 个;`Object.free` p_virtual=false 不入表;单测锁 12 条 + api 无名断言。
- **`push_engine_virtuals`**(godotlsp):非 static 位 = 宿主链 chain_virtuals + core 手表,
  签名渲染镜像 `_get_visual_datatype`(`name(arg: Type) -> Ret:`,void 恒带 `-> void:`);
  static 位不走 ClassDB,只出 `_static_init() -> void:`(引擎 :3735-3739 特造 MethodInfo);
  宿主 = `script_engine_base_by_path` 链尾 / 内部类 extends 引擎类 / 默认 RefCounted。
- **关键修复:光标所在函数排除**——引擎 `has_function(mi.name) && member != function_node`
  的第二子句:declared 收集必须排除 span 覆盖 at 的 func(`func _get➡` 时 _get 不算已声明),
  否则 full_name 夹具的 `_get` 永远被 skip(absent,oracle 100 卡 1 的根因)。
- **C2 `snippets::virtual_items` 12 条保留**(决策点拍板):snippet 模板面与 override 全量
  签名面语义不同,不换模型源。

- **验收(实际)**:oracle **99→101/140(+2,零回归)**——override 桶 3/5→**5/5**
  (`override_function_static` 出 `_static_init() -> void:`、`override_function_full_name`
  出 `_get(property: StringName) -> Variant:`);workspace 测试全绿(engine +3:is_virtual
  计数/chain_virtuals 先到先得/core 表;godotlsp +3:引擎面签名/static 位/declared 跳过);
  clippy 0;smoke ALL OK;基线已写 101。**剩余归界**:VIRTUALS→模型对照测试未做(两边
  语义不同,保留 C2 手表即可);`_static_init` 与脚本链 static 方法并存(引擎 static 位
  不出脚本链 static——夹具 subset 判定无感,差异记录不修)。

### P8 — C8 二期参数选项面剩余(godotlsp)

- **tween_property 属性面**:`Tween.tween_property(` 实参 1 → 实参 0 的局部变量类型属性表
  (引擎类属性 + 脚本类成员);前置 = P6 的 local_types(原判"semantic 未暴露 locals 查询"的成本
  在 P6 化解)。
- **InputMap 单例面**:`InputMap.has_action(`/`event_is_action(` 等首参 → `[input]` 动作表
  (与 C8 一期输入动作面同数据,入口扩到单例方法名识别;无 oracle 覆盖,单测锁定)。
- **`&`/`^` 前缀设置 plumbing**:经 initializeOptions/`workspace/configuration` 读三项
  编辑器设置(`use_single_quotes`/`add_string_name_literals`/`add_node_path_literals`,
  引擎默认全 false)→ oracle `add_*` 两口;默认侧不变(回归守护)。
- **仍挂 P10**:AnimationPlayer 动画面(需 tscn 动画库)。
- **验收**:`argument_options` 桶 add_* 两口 + 单测;设置缺省时行为与一期一致(零回归)。

### P9 — C6.5 builtin 构造/运算符面(引擎数据面 × 文档生成面)

**数据源定论(2026-10-17 盘点)**:`extension_api.json` 的 builtin `constructors` 只有
`index`、`operators` 只有 `name/right_type/return_type`——**均无签名/描述**;
`.refs/godot/doc/classes/*.xml`(810 个,已抽样 int.xml 验证)带完整
`param name/type` + `return` + `description`。→ **双源互补:XML 签名+描述,JSON 结构**。

- **C6.5a 模型解析**:`tools/gen_builtin_sig.py`(新,仿 gen_docs 管线)从 pinned XML 生成
  `crates/gdscript-engine/testdata/builtin_signatures.json`(**生成物入库**,与 hoverdata.json
  同律;XML 本体仍不入库)→ `RawBuiltin` 解析 + `builtin_constructors(name)`/
  `builtin_operators(name)` 查询 API。
- **C6.5b 文档生成面**:`tools/gen_docs.py` 增 `Constructors`/`Operators`/
  `Constructor Descriptions`/`Operator Descriptions` 四区块解析(现有 Method/Property
  Descriptions 解析器同构扩展,成本低)→ website 内建页补两节 + hoverdata 增条目
  (与 `--check` 防漂移同程)。
- **C6.5c 消费面**:① `Vector2(`/`int(` 等内建类型调用 → 构造重载补全(参数签名,
  引擎 `BUILT_IN_TYPE` 上下文同款);② 类型名 hover 追加构造重载列表;③ per-type 运算符
  hover **仍挂语义层**(二元表达式两侧类型推断,开放问题不变)。
- **C6.5d 仍只解析备未来**:`is_keyed`/`indexing_return_type`/`api_type`/`is_bitfield`;
  `version_full_name` 可进启动日志(小甜点,可选)。
- **取舍**:无 oracle 覆盖(语料无内建构造补全夹具)→ 单测锁 38 类构造/运算符计数 +
  `gen_docs.py --check` + smoke;英文数据,中文翻译走 P2-16。
- **验收**:`class_int.md` 含 5 构造 + 64 运算符节;`Vector2(` 构造面单测 + E2E;零回归。

### P10 — N6 场景树(gdscript-project + godotlsp,最大工程,末期)

- **C10.1 tscn 解析**(gdscript-project 新增,附加式):`[node path="…"]` 树 →
  `NodeDecl`(name/type/instance 占位符);`[ext_resource type="Animation"]` +
  AnimationPlayer 的 `animations` 属性 → 动画名表;`[connection]` → 场景信号连接表。
- **C10.2 节点名面**:填充 C7.3 之后一直悬空的 `$`/`%` 分支(当前策略 = 空列表优于噪声;
  场景树就绪后出场景子节点名)+ `get_node("➡")` 字符串路径面。
- **C10.3 动画面**:`AnimationPlayer.play("➡")` → 动画名(接回 P8 未竟项,
  `argument_options/play_*` ×2)。
- **取舍**:先节点树(主价值),后动画/连接;instance 占位符只记录不展开(需递归加载);
  uid 解析(`.uid` → 脚本路径)作为独立小步;场景缓存失效按 mtime(与既有索引同律)。
- **验收**:oracle get_node 余 22 + play_* ×2;单测锁 tscn 解析;E2E `$` 路径面;零回归。

### P11 — C9 snippet 对齐(工具/插件面,独立于服务器主线,可插空)

- **C9.1 插件面 16 → ~40**:按 §3.2 桶补齐(收全部标注桶);保留现有自造前缀(肌肉记忆),
  新增标准前缀双发;来源与许可证注记进 `corpus/snippets/README.md` 与 fetch 脚本。
- **C9.2 `tools/fetch_snippets.py`**:pinned 双仓库 → `corpus/snippets/`(不入库,幂等,
  仿 `fetch_corpus.py`);`--check` 模式输出"union vs ours"缺口清单——**把"对齐"变成可重跑审计**,
  上游更新时一条命令重对齐。**2026-10-17 盘点:该目录尚不存在,C9.2 是零启动成本的全新基建。**
- **C9.3 官方模板:来源重定(2026-10-17 盘点纠正)**:原计划"9 个 script_templates 入库"的
  前提**不成立**——`.refs/godot` 全量克隆内无任何模板 `.gd`(全仓唯一 `.gd` 是测试夹具;
  `editor/` 源码无 `script_templates` 引用;Godot 4 模板走编辑器安装目录/用户目录)。
  新方案:fetch 时先做**来源考证**(编辑器分发包/上游组织),能 pinned 复验才入库(判据不变:
  用户 clone 后能否直接跑);**不可 pinned 复验则明确不收**,在本文件记录来源供用户自取。
- **C9.4 服务器 C2 面小补**:`snippets.rs` COMPOSITES 补 await signal / waitsec(与 C9.1 去重后);
  keyword_snippet 不动。
- **验收**:`--check` 零缺口(除明确不收项);smoke snippet 面不回退;npm test 不受影响。

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
6. **引擎内置 snippet 表/script_templates 复用**:2026-10-17 实测两者在引擎源码仓
   **均不存在**(GDScript 编辑器无 snippet 库;模板走编辑器安装目录/用户目录)——C9 基准
   锁定 vscode + zed 上游两家;templates 改为"来源考证后定,不可 pinned 复验则明确不收"。

## 七、风险总账

| 风险 | 影响面 | 缓解 |
|---|---|---|
| C6 附加式 serde 解析失败 | 模型加载即崩(测试先炸) | `#[serde(default)]` 全套;单测锁 503/1437 计数 |
| 信号/虚方法沿链收集拖慢补全 | completion p50(基线 8.6ms) | O(链长×成员)可忽略;每期 bench_load 监控 |
| C8 手搬表随引擎版本漂移 | 注解参数过时 | 源码锚点注释 + oracle `argument_options` 桶检测;警告名改取引擎 `gdscript_warning.h`(补全行为事实源),与弃用项剔除规则同锚 |
| C7.2 字符串面客户端不触发 | 特性不可见 | 主流客户端默认行为;README 说明;监控后补开关 |
| **P5 全局面合并污染**(类成员/内部类/全局枚举名进裸标识符面) | 全局面 1800+ 项噪声、filter 排噪回退 | 新 sort 桶(SELF/OVERRIDE)与频率表正交;同名优先级规则(局部>自声明>父类链>引擎链>全局面);`filter` 桶 2/2 + E2E 计数 + vacuous pass 清单三重验收;设计先行入 lsp-design |
| **P6 语义推断误判**(字面量推断错误类型 → 出错误成员) | 成员面假阳性 | 保守策略:只做单层高置信推断,失败静默降级;单测锁各型;与引擎"类型未知不出成员"行为对齐 |
| oracle 语义差异误判 | 验收噪音 | 匹配器规范化 + 分桶趋势判断,不追求 100% |

## 八、开放问题(未来钩子,不在本期)

- ~~**C8 二期(参数选项面剩余)**~~ → 已排入 P8(tween_property 前置在 P6 化解;动画面挂 P10;
  InputMap 单例面与 `&`/`^` plumbing 同 P8)。
- ~~**C10 self 成员面**~~ → 已排入 P5(簇 A,含 N3/N4 归并;设计入 lsp-design.md 后排,
  纪律不变:先设计后实施)。
- **per-type 运算符 hover**(如 `int * Color` 的运算符文档):需语义层对二元表达式两侧做类型推断,
  依赖类型系统完备度;当前通用 `OPERATOR_DOCS` 20 条已覆盖语义科普,per-type 留待语义层增强后
  (与 P9 C6.5c-③ 衔接;C6.5a 的 `builtin_operators` 查询 API 是其数据前置)。
- **autoload 成员解析**:内核缺口(autoload hover 为 null、成员不存在不报假阴性)——需内核改,
  独立立项(R2 后)。
- **类型感知评分**:前置 P6 的 `local_types`(C5),之后 completion 排序可吃局部变量
  (与 P5 sort 桶设计联动)。
- **中文语料入 hover/completion**:godot-docs zh-CN(P2-16 i18n 线),与本路线图正交;
  C6.5b 生成的英文构造/运算符节是 i18n 的天然素材。
- **`filter/` 桶的 organized export 排噪**:`@export_category` 等注解串不进普通标识面——
  随 P4 全局枚举类型名入全局面时同批验证(oracle 已覆盖)。
