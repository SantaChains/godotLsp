# 文档词条与 LSP 对齐路线

本篇说明文档站词条与自研 LSP 符号模型如何共用同一事实源，逐项列出当前已识别、误报来源与修复顺序，目标是提高正确性与覆盖度、消除"未声明"类误报。

## 1. 单一事实源

文档与 LSP 都以 `crates/gdscript-engine/testdata/extension_api.json`（引擎 API dump）为唯一签名事实源：类、方法、属性、常量、枚举、全局工具函数、引擎单例。

- 文档侧：`tools/gen_docs.py` 读取该 json 生成 `website/src/builtin/` 类页与 `crates/godotlsp/src/hoverdata.json`；中文只通过 `tools/translations.py` 的覆盖表叠加，不改写英文签名与成员名。
- LSP 侧：`gdscript-engine` 把同一 json 解析为 `EngineApi`，语义层 `gdscript-syntax/semantic.rs` 与服务层 `godotlsp/main.rs` 只查询该模型。

因此词条名必须与 dump 完全一致：类名、方法名、属性名、常量名、枚举值名。中文覆盖键为 `class:X`、`member:X.member`、`fn:name`、`anno:name`、`member:@GlobalScope.name`，新增词条不得另造别名，否则 hover 与补全落空。

## 2. 当前已识别的名称层级

语义层 `analyze_with` 按以下顺序判定一个裸标识符是否已知（`semantic.rs` 的 `is_known`）：

1. 局部作用域（函数块、for 变量、形参）。
2. 文件级成员：`var`、`const`、`func`、`signal`、`enum`、内部 `class`。
3. 语言保留名：硬编码 `BUILTIN_RESERVED`（print/len/abs/min/max/clamp/round/floor/ceil/sign/pow/sqrt/posmod/snapped/lerp/lerp_angle/inverse_lerp/remap/move_toward/smoothstep/is_inf/is_nan/range_lerp，以及 PI/TAU/INF/NAN/SQRT12/SQRT3、true/false/null/self/super）加 `GDSCRIPT_BUILTINS`（@GDScript 语言内建：preload/load/range/char/ord/assert/is_instance_of/type_exists/get_stack/print_stack/print_debug/dict_to_inst/inst_to_dict/Color8/convert，见 G1）。
4. 引擎类 `class_by_name`、内建类型 `builtin_by_name`。
5. 全局工具函数 `utility_function`（114 个 @GlobalScope 函数）、引擎单例 `singleton_type`（Input/OS/Time/Engine 等）、全局常量 `global_constants`、全局枚举名 `global_enums`、全局枚举值 `global_enum_value`（KEY_*/ERR_*/MOUSE_BUTTON_* 裸值，见 G2）。
6. 项目级：`class_name_type`（脚本声明的全局类型）、`autoload_type`（project.godot 的 [autoload]，已解析到 class_name/引擎类/合成 script 键，见 G4）、`preload_type`（见 G3）。

成员访问校验：`find_method`、`find_property`、`find_const`、`find_enum` 均已沿 `inherits` 继承链向上查找，并对内建类型（Vector2/Color 等）做成员兜底，因此继承成员不会因"子类上没有"而误报。

## 3. 误报与覆盖缺口（按修复优先级）

### G1 @GDScript 语言内建未全量进保留名集（最高优先，误报主因）

`BUILTIN_RESERVED`（`gdscript-syntax/src/semantic.rs`）只硬编码约 20 个。`@GDScript` 语言内建（非引擎 @GlobalScope 工具函数）目前只存在于 hoverdata 的 `f` 表，语义层查不到，导致以下写法被误报"未声明的标识符"。

权威名单取自 `godot-docs/gdd_1590_@GDScript.md` 的 Methods 表，共 16 个：`Color8`、`assert`、`char`、`convert`、`dict_to_inst`、`get_stack`、`inst_to_dict`、`is_instance_of`、`len`、`load`、`ord`、`preload`、`print_debug`、`print_stack`、`range`、`type_exists`。其中 `len` 已在保留集，其余 15 个需补入。`str`/`typeof`/`type_convert` 已在 extension_api 的 utility_functions 内，由 `engine.utility_function` 识别，不重复列入。

设计与落点：
- 语义 crate 在分析期不读文件，故采用静态常量数组 `GDSCRIPT_BUILTINS`，名单与 gdd Methods 表一一对应，注释标注来源，升级引擎版本时对照该表增补。
- `is_known` 在 `BUILTIN_RESERVED` 命中判断后追加 `GDSCRIPT_BUILTINS` 命中判断；只解决"不报未声明"，返回类型推断（preload/load → Resource、range → Array 等）留待 G3。
- 弃用族（`Color8`、`convert`、`dict_to_inst`、`inst_to_dict`）同样判为已知，不报错；弃用警告属后续增强，不在 G1 引入。
- 验收：样本中裸写 `preload(...)`、`load(...)`、`range(...)`、`char(...)`、`ord(...)`、`assert(...)`、`is_instance_of(...)`、`get_stack()`、`print_stack()`、`print_debug(...)`、`type_exists(...)`、`dict_to_inst(...)`、`inst_to_dict(...)`、`Color8(...)`、`convert(...)` 均不产生"未声明的标识符"。

### G2 全局枚举值裸用未识别（误报主因）

`is_known` 只按枚举名查 `global_enums`，不查单个枚举值。代码里裸写 `KEY_ENTER`、`KEY_SPACE`、`ERR_OK`、`MOUSE_BUTTON_LEFT`、`KEY_LEFT` 时被判未声明。`EngineApi.global_enum_value(name)`（`gdscript-engine/src/lib.rs`，返回枚举名与数值）已实现，但未接入 `is_known`。

设计与落点：
- 在 `is_known` 的引擎模型分支追加 `self.engine.global_enum_value(name).is_some()`，与 utility_function/singleton_type/global_constants 并列。
- 只做"已知"判定，不做数值回写；补全列表把全局枚举值纳入、hover 显示所属枚举与数值属后续增强。
- 验收：裸写 `KEY_ENTER`、`OK`、`FAILED`、`MOUSE_BUTTON_LEFT`、`KEY_SPACE`、`ERR_FILE_NOT_FOUND` 不产生"未声明的标识符"，且合法自定义标识符不受影响。注意 Error 枚举前两项为 `OK`/`FAILED`（无前缀），其余为 `ERR_` 前缀，不存在 `ERR_OK`。

### G3 无 class_name 的 preload/load 常量绑定类型（已完成）

问题：`const X = preload("res://x.gd")` 在被加载脚本没有 `class_name` 时，`preload_type` 返回 None，X 不报已知也无类型，`X.new()`、`X.SOME_CONST` 失去校验。

已落地设计：
- `ProjectContext.preload_type`（gdscript-project/lib.rs）：命中脚本时，有 `class_name` 返回全局类型名；无则合成稳定键 `script:<相对路径>`（如 `script:level.gd`），不再返回 None；路径未命中返回 None。
- 语义层 `bind_load_call`（semantic.rs）：识别 `const X = preload(...)` / `var X = preload(...)` / `load(...)`；preload 用 preload_type 结果，未命中回退 `Resource`；load 恒绑定 `Resource`。绑定类型写入文件级成员表（`bind_member`），X 因此可解析、`X.new()` 特判为该类型。
- 成员门控 `checkable_type`：仅引擎类/内建类型或已注册 `class_name` 才做成员/方法存在性校验；合成脚本键、autoload 等成员模型未挂载的类型静默返回 Unknown，避免新增"没有成员"误报。class_name 用户类的未知成员仍按旧用例报错。
- 边界：合成键当前只解决"X 不报未声明、类型链起步"，脚本自身成员（变量/常量/方法）的跨文件校验属后续（与 G4 同批挂载脚本符号模型）；`load(...)` 路径为运行时动态，静态期统一 Resource。
- 回归测试：`g3_preload_const_binds_without_class_name`（合成键 + X.new() 无报错）、`g3_load_const_binds_resource`（未命中 load 不报未声明），旧 `still_flags_unknown_cross_file_member` 保持通过。

### G4 autoload 解析与脚本成员模型挂载（已完成，第一阶段）

原问题：`autoload_type` 返回 project.godot 原始值（`*res://x.gd` 或 `*ClassName`），未解析为脚本/引擎类型，`GameManager.xxx` 成员访问无法校验。

已落地设计：
- autoload 解析（gdscript-project/lib.rs）：`res://x.gd` 经 `resolve_preload` 取脚本，复用 `script_key`（class_name 或合成 `script:<path>`）；裸类名命中注册 class_name 或引擎类时原样返回；其余返回 None。删除了与 trait 方法同名的 inherent `autoload_type`（原先遮蔽 trait 实现，导致解析失效）。
- 脚本成员模型：trait 新增 `script_type_exists(key)` 与 `script_member_kind(key,name)->ScriptMember{Method,Property,Const}`；按类型键（class_name / `script:<path>`）定位脚本，扫描顶层 `func/var/const` 登记自身成员。
- 语义层命中：方法/成员访问先查引擎 `find_method/find_property`，再查脚本自身成员；命中即返回，不再误报。
- 报错边界（strict_type）：仅引擎已知类型与已注册 class_name 对缺失成员/方法报错；autoload 与合成脚本键缺失时静默 Unknown。理由：脚本 `extends Node/Node2D` 的引擎基类成员链尚未遍历，此时硬报"没有成员"会误报，先保证零误报。
- 回归测试：`g4_autoload_resolves_and_knows_own_members`（Bus.energy 自身属性、Bus.emit_pulse 自身方法命中，未知成员静默）；旧 `still_flags_unknown_cross_file_member`（class_name 未知成员报错）保持通过。

未完成（并入后续）：
- 无注解 var 的字面量/调用结果类型推断、跨脚本 `extends 另一个用户脚本` 的链式解析；autoload 指向引擎类时的成员校验已由引擎模型承担。

### G4.6 脚本成员声明类型回写（已完成）

问题：脚本 var/const/func 命中后一律返回 Unknown，类型链断，下游无法继续成员补全/校验。

已落地设计：
- trait 新增 `script_member_type(key,name)->Option<String>`：var: T / const: T 取注解根类名，func -> T 取返回类型根类名；无注解返回 None，不臆造。
- 公共 `type_root` 提取 Ident/Generic 根（extends_root 复用它）；preload/路径/无注解返回 None。
- 语义层方法调用命中自身 Method、属性命中自身 Property/Const 时，有声明类型返回 Named(T)，无注解回退 Unknown。
- 回归测试：`g46_script_member_declared_types_write_back`（hp/MAX=int、label=String、loose 无注解 None、nope None；main.gd 零报错）。

诚实边界：只回写显式注解，不做字面量/返回值推断；`extends 另一个用户脚本` 的跨脚本链未做。

### G4.5 脚本 extends 基类引擎成员链（已完成）

问题：脚本 `extends Node2D/Node` 后，访问基类成员（`p.position`、`Bus.name`）在脚本键/class_name 上查不到，旧逻辑要么误报要么静默。

已落地设计：
- trait 新增 `script_extends_base(key)`：从 `extends Ident` / `extends Base[T]` 提取根类名；`extends preload(...)`/路径返回 None。
- 语义层 `script_base(cls)`：方法/属性访问在自身与本类引擎成员之后，再沿基类名查 `find_method/find_property`（引擎侧已沿 inherits 链向上），命中即返回，不再误报。
- 报错边界不变：仍仅引擎已知类型与 class_name 对缺失成员报错；autoload/合成键缺失静默。
- 回归测试：`g45_extends_base_members_resolve`（Player extends Node2D 读 p.position、Bus extends Node 读 Bus.name 均无报错，自身 hp/energy 同时命中）。

附带修复（阻塞构建的既有 bin 错误）：
- initialize_params.workspace_folders 被闭包 move 后又借用，改为 clone 后再取首个 folder，消除 E0382。
- lsp-types 0.97 的 `SemanticTokens.data` 为 `Vec<SemanticToken>`（序列化器再压成 u32 增量），由 u32 数组改为结构化 token。

### G5 局部枚举值、信号、内部类（枚举部分已完成，签名/内部类待做）

官方规则（gdscript_basics Enums 节，已核验）：
- 匿名枚举 `enum { A, B }`：条目即文件级 int 常量，可裸写 A。
- 命名枚举 `enum State { A, B }`：State 是 const Dictionary，条目只允许 `State.A` 访问，裸写 A 不是全局常量，仍应报未声明。
- 命名枚举可走 Dictionary 方法 `keys()/values()/has()/get()`。

已落地（消除误报、不放松合法裸值、不误放非法裸值）：
- collect_member 区分两种枚举：匿名条目写入 members 为 int；命名枚举名保留在 members，条目名单独存 enum_members。
- `State.A` 在属性分支命中 enum_entry 返回 int；`State.keys()` 等字典方法对非严格类型静默放过。
- 自定义 signal 已注册为 Ty::Signal，`sig.emit(...)`/`sig.connect(...)` 当前静默通过，不报未声明。
- 回归测试：g5_named_enum_and_signal_ok（State.IDLE/RUNNING、emit/connect、Inner.new() 静默）、g5_anonymous_enum_bare_constant_ok（裸 TILE_*）、g5_bare_named_enum_key_still_unknown（裸 IDLE 仍报 1 条）。

待做（G5 剩余，不影响零误报，属增强校验）：
- signal 声明签名与 emit/connect 实参个数/类型校验。
- 内部 class 建立独立作用域表，解析 Inner 的 var/func 成员（当前 Inner.new() 与成员访问对非严格类型静默）。
- 命名枚举 Dictionary 方法的显式白名单（当前静默，不报错）。

### G6 静态类型链与场景节点引用

- `var p: Player = preload(...)` 这类带注解的变量已能识别注解类型；但注解指向未注册 class_name 的脚本时回退 Unknown。
- `$Path`、`get_node("Path")` 静态期无法知道场景树结构，当前判 Unknown 且不报错（正确，避免误报）；后续可结合 tscn 索引做节点路径补全与存在性提示，属增强项，不作为硬诊断。

## 4. 文档词条建设（已完成项）

### 4.1 @GlobalScope 全局工具函数（#43，已完成）

- 规模：extension_api.json 的 `utility_functions` 共 114 个，全部中文覆盖，键 `member:@GlobalScope.<name>`，分组为三角/双曲、取整取模、比较判定、插值缓动角度、夹取极值环绕、随机数、调试输出、序列化反射对象。
- 修掉的真实 bug：`gen_global_scope_page` 原先只取 gdd 英文描述、不走 tr 覆盖；改为 `tr("member:@GlobalScope.<name>")` 优先，页面与 hoverdata 同源生效（hover 的 m 表本就走 member 覆盖）。
- 约束：只译描述，签名（参数/返回类型）来自 json 不改；`str/typeof/type_convert` 同时是 utility_function，由引擎模型识别。

### 4.2 属性 / 常量 / 枚举词条（#44，结构全量，中文分层）

全量规模：4224 个属性、161 个类常量、764 个枚举值，无法逐条手译，采用结构全量、中文分层叠加：

1. 结构层（已完成）：`gen_docs.py` 从 json 渲染每个类的属性（类型名、默认值、可读写标注）、常量（名、值）、枚举（名、取值表），全部带锚点供跳转；英文官方描述取自 gdd，无描述处显示"描述待补"，不臆造。
2. 渲染缺口修复（已完成）：属性、信号、枚举值、类常量四处原先只取英文、不应用 tr，现全部改为中文覆盖优先（`member:Class.member`）。
3. 中文覆盖层（增量）：先核心值类型（Vector2/Vector2i/Vector3/Vector3i/Vector4/Color/Rect2/Rect2i/AABB/Transform2D/Transform3D/Basis/Quaternion/Plane/Projection 的标量成员与 ZERO/ONE/INF/方向/IDENTITY/FLIP/PLANE_* 常量），再高频引擎类（Node/Node2D/Control/Resource/SceneTree/Input/Timer/Tween 等），长尾保留英文官方描述；140 个 CSS 色名常量自明保留英文。
4. 一致性约束：中文只改描述，不改类型、默认值、成员名；`--check` 保证生成区不被手改漂移。

### 4.3 信号词条（#42，488/503，15 条源缺占位）

- 503 个信号由 `tools/gen_signals.py` 从 extension_api.json（签名事实源）+ 本地 4.7 XML（官方描述）生成 10 个分类平铺页，锚点校验 503/0 坏链；运行期/Editor/Server/XR 共 488 条已中文化。
- 已核验补译：OpenXRInterface.cpu_level_changed/gpu_level_changed（指定子域 CPU/GPU 性能等级改变）、OpenXRAnchorTracker.uuid_changed（锚点持久化 UUID 改变）。
- 剩余 15 条为 4.7 新增深层 OpenXR 扩展（OpenXRFutureResult.completed、OpenXRInterface.session_synchronized/user_presence_changed、OpenXRPlaneTracker.mesh_changed、OpenXRRenderModel/Extension/Manager 的 render_model_added/removed/top_level_path_changed、OpenXRSpatialEntityExtension.spatial_discovery_recommended、OpenXRSpatialEntityTracker.next_changed/spatial_tracking_state_changed、OpenXRBindingModifierEditor.binding_modifier_removed、VisualShaderNodeInput.input_type_changed）：本地 4.7 XML 为 0、官方最新页取不到可核验英文，保留"待补"占位并链类页，不编造；边界写在 signals/00-guide.md，待官方补文档后按同一 CHINESE 表补译。

## 5. 文档词条到 LSP 查询入口的对应

| 文档词条 | LSP 查询入口 |
| --- | --- |
| 类页 brief | `class_by_name` / `builtin_by_name` |
| 方法（含继承） | `find_method`（沿 inherits 链） |
| 属性/成员变量 | `find_property`（沿链 + 内建成员） |
| 类常量 | `find_const`（沿链 + 内建常量） |
| 类枚举 | `find_enum`（沿链） |
| @GlobalScope 工具函数 | `utility_function` |
| 全局枚举值（KEY_/ERR_ 等） | `global_enum_value`（已接入 is_known，见 G2） |
| 引擎单例（Input/OS/Time） | `singleton_type` |
| 用户 class_name | `ProjectContext.class_name_type` |
| autoload | `ProjectContext.autoload_type`（已解析到 class_name/引擎类/script 键，见 G4）；自身成员 `script_member_kind` |
| preload/load 引用 | `ProjectContext.preload_type`（class_name 或合成 `script:<path>`，见 G3） |

## 6. 落地顺序

- [x] G1：@GDScript 内建并入已知集，消除语言内建误报（已完成：`GDSCRIPT_BUILTINS` 静态表接入 `is_known`，补回归测试 `g1_recognizes_gdscript_builtins`）。
- [x] G2：`global_enum_value` 接入 `is_known`，消除 KEY_/ERR_ 裸值误报（已完成，补 `g2_recognizes_global_enum_values` 与边界 `g2_unknown_identifier_still_reported`）。
- [x] G3：无 class_name 的 preload/load 常量绑定类型（已完成：合成 `script:<path>` 键、load/未命中回退 Resource、checkable_type 门控，补两条回归测试）。
- [x] G4（第一阶段）：autoload 解析到 class_name/引擎类/合成 script 键，挂载脚本自身 method/property/const；class_name 未知成员仍报错，autoload/合成键缺失静默。
- [x] G4.5：脚本 `extends` 基类引擎成员链接入（自身 → 本类 → 基类 inherits 链），基类成员不再误报。
- [x] G4.6：脚本 var/const 注解类型、func 返回类型回写；无注解不臆造。
- [ ] G4.7：无注解字面量/调用结果类型推断；extends 指向另一个用户脚本的链式解析。
- [x] G5（枚举部分）：匿名枚举裸值进常量作用域，命名枚举 State.KEY 识别，裸命名键仍报未声明；signal emit/connect 静默通过。
- [ ] G5 剩余：signal 签名/实参校验、内部类独立作用域、命名枚举字典方法白名单。
- [ ] G6：静态类型链、$节点路径增强。
- [ ] #44：按核心值类型到高频引擎类的顺序叠加属性/常量/枚举中文，长尾保持英文官方描述。

每步完成后用多文件工程样本回归：先确认误报归零，再谈成员级漏检；新增诊断宁可 Unknown 放过，也不得对无法确定的标识符报"未声明"。
