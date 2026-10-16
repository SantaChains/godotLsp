# GDScript 中文文档站规划（godotlsp LSP 数据源 + GitHub Pages）

> 状态：规划稿（待确认后执行）。日期：2026-10-16。

## 〇、目标与约束

1. **脱离官方文档站**：LSP hover 中的跳转链接（`globals.rs::url` 模块，现指向
   `docs.godotengine.org/zh-CN/stable`）改为指向**自建 GitHub Pages 文档站**。
2. **一一对应**：每个内置类 / 方法 / 常量 / 枚举 / 注解 / 关键字 / 操作符
   在文档站上有**稳定锚点**，内核 `doc_link()` 生成的 URL 必须命中。
3. **全中文**，涵盖 GDScript 全部语法 + 全部内置函数/方法/类。
4. 纯静态、GitHub Actions 自动部署 mdBook，零运维。

## 一、现状结论

| 项 | 现状 | 处置 |
|---|---|---|
| `website/` | **Rust 语言圣经（rust-course）的 mdBook 副本，16MB**，与本项目无关 | 整体清空，仅保留 mdBook 技术栈选型；`theme/`、`ci/`、`deploy.sh`、`genpdf.sh` 一并删除 |
| `globals.rs::url` | Sphinx 锚点规则（`_`→`-`、`%40` 转义），BASE 指向官方站 | 重写为自建站 BASE + 自定义锚点规则 |
| `godot-docs/` | 1593 篇英文 md 语料（上游 MIT/CC-BY），已有 `build.rs` 漂移表提取基建 | 内容主数据源 |
| 内嵌引擎模型 | `gdscript-api::bundled()`（engine_api.bin） | 签名/成员表权威来源（生成脚本读同一数据） |

## 二、架构

站内结构定为**四篇 + 类参考**，层进关系：语法（怎么写）→ 范式（Godot 怎么想）→ 引擎内部（机器怎么处理）→ 类参考（逐条速查）。

```
src/
  intro/                   # 总览、版本说明
  syntax/                  # 语法篇（手写，~16 篇）
  paradigm/                # 范式篇（手写，Godot 三大支柱，~8 篇）
  internals/               # 引擎内部篇（取材 engine-analyzer.md，~7 篇）
  builtin/                 # 类参考篇（脚本生成，~900 类）
  appendix/                # 高频坑速查、3.x→4.x 迁移表、学习路线
```

**内容管线（一一对应的机制保障）**

1. `tools/gen_docs.py` 从 `gdscript-api` 的 API 数据（extension_api.json 同源）
   枚举每个类的 methods / constants / enums / signals / properties，
   生成 `builtin/class_<name>.md`，**每个成员一个显式锚点标题**：
   ```markdown
   ### get_node {#get_node}
   ### OFFSET_MAX {#OFFSET_MAX}
   ### enum Key {#enum-Key}
   ```
   mdBook 标题属性 `{#id}` 显式指定锚点，**不依赖 slug 推断**，锚点由代码锁死。
2. 成员正文：优先复用 `gdscript_docs.rs` 已有文档表 + godot-docs 英文语料翻译；
   缺失者 AI 补写后经语料交叉核验。
3. **内核侧同步**：`globals.rs::url` 改为
   - `class(name)` → `builtin/class_<lower>.html`
   - 成员锚点 → `#<原名>`（下划线保留，不做 `_`→`-`，比 Sphinx 规则简单）
   - `@GlobalScope` / `@GDScript` 特殊页同规则
   - 关键字/操作符 → `syntax/ops.html#<token>`、`syntax/keywords.html#<word>`
4. **防漂移测试（关键）**：`cargo test` 新增 URL 一致性测试——
   枚举内核全部可生成的 doc_link → 映射回 `website/src/builtin/*.md` 文件与
   `{#锚点}` 标题 → 缺一即 fail。文档站与 LSP 永不脱钩。

**URL 契约（定死，后续不兼容改动需双端同步）**

| 目标 | URL |
|---|---|
| 类 | `/builtin/class_<name小写>.html` |
| 方法/常量/信号/属性 | 同上页 + `#<成员原名>` |
| 枚举 | 同上页 + `#enum-<Enum名>` |
| 枚举成员 | 同上页 + `#<成员原名>` |
| 关键字 | `/syntax/keywords.html#<word>` |
| 操作符 | `/syntax/operators.html#<符号>` |
| 注解 | `/syntax/annotations.html#@<name>` |

## 三、语法篇大纲（手写部分）

原则：**跨语言通用概念合并快讲，Godot 特有独占篇幅深挖**。
（已吸收用户提供的 GDScript 核心资料结构；其中若干技术错误在成文时修正，见 §三末）

1. **语言总览**：GDScript 定位（依附节点的脚本语言，非通用语言）、与 Python 异同表、
   4.x 与 3.x 断代差异速查。
2. **词法基础**（1 篇合讲）：标识符、关键字全表、注释、缩进即语法、换行/续行、
   分号可选、字面量全表（int/float/`_` 分隔符/0x/0b/字符串/`&"name"`/`^"path"`）。
3. **变量与常量**：var/const、类型标注与推断（类型标注 3.x 已有，4.x 补全推断与
   typed Array——资料中"Godot4 新增"说法修正）、无 `int?` 可空语法（可空即不标注/Variant）、
   Variant 动态类型真相、作用域。
4. **类型系统**：
   - 基本类型：int/float/bool/String/StringName/NodePath/Callable/Signal
   - 数学类型合讲：Vector2/2i/3/3i/4/4i、Color、Rect2/2i、AABB、Basis、Quaternion、
     Transform2D/3D、Plane、Projection（一表通览 + 通用运算语义）
   - 容器：Array/typed `Array[int]`（方括号，资料中 `Array<int>` 修正）、
     Dictionary/typed Dictionary(4.4+)、Packed*Array 八件套
   - 类型转换：is / as / 显式构造 / int↔float 截断陷阱
5. **流程控制**（1 篇合讲）：if/elif/else、match（含绑定/数组/字典模式）、
   while/for、break/continue、三元 `a if c else b`、assert。
6. **运算符全表**：优先级完整表、`/` 整数截断、`%` 双义（取模 + 格式化字符串）、
   `**`、`is`、`in`、`??`、位运算、`==` 深比较语义。
7. **函数**：func、默认值、类型化返回、变参、static func、lambda（捕获语义）、
   Callable 一等公民、super 调用。
8. **类系统**：script 即 class、extends（不写则默认 RefCounted——资料中
   "第一行必须写"措辞修正）、class_name、内部类、`_init` 与构造、
   属性 setter/getter、static var(4.1+)、枚举、`new()`。
9. **注解全表** ⭐：@export 系全家（range/file/dir/multiline/node_path/color/…）、
   @onready、@tool、@rpc、@warning_ignore、@abstract、@static_unload 等，
   逐条语义 + 参数 + 示例（hover 高频跳转目标）。
10. **信号系统** ⭐：signal 声明、`signal.emit()`（4.x 首选；`emit_signal()` 为
    3.x 遗留，资料用法修正）、connect(4.x Callable)/bind、CONNECT_* flags、
    内置节点信号、信号 vs 回调选型。
11. **场景树与节点引用** ⭐：$/%、get_node、owner 与场景归属、
    实例化（preload/load/instantiate/**add_child 才激活生命周期**——资料的强调正确，
    收录）、@onready 时机、`_ready` 缓存引用最佳实践、生命周期回调全表
    （_init/_enter_tree/_ready/_process/_physics_process/_input/_unhandled_input/
    _exit_tree/_notification 触发顺序图）。
12. **await 与协程** ⭐：await signal / await coroutine、与 3.x yield 对照、
    超时模式（race）、常见误用。
13. **autoload 单例** ⭐：注册机制、`*` 全局标志、访问规则（呼应内核 R7 规则）。
14. **错误处理模型** ⭐：GDScript 无异常——Error 枚举返回约定、push_error/warning、
    assert 仅 debug、防御式写法惯例。
15. **输入系统**：Input 单例、action 映射（项目设置预定义，不硬编码按键）、
    Input.get_axis/is_action_pressed/_input/_unhandled_input 分层。
16. **字符串格式化与惯用法**：`%` 格式、String.format、GDScript 风格指南要点。

**附录（用户资料中的「高频坑」直接改写为独立速查页）**：`_process` 中 `$` 查找、
漏乘 delta、物理逻辑放错回调、instantiate 后忘 add_child、已 free 节点访问
（4.x 中同树节点销毁会自动断连，野引用主要来自异步 lambda/跨树引用——措辞精确化）。

## 三之二、范式篇（语法讲完后的"不一样"——Godot 三大支柱）

定位：语法会了 ≠ 会用 Godot。此篇讲**编程模型与设计哲学**，
对标用户提供的"Godot 核心重点"资料并深化：

1. **三大支柱总览**：节点（组织对象）+ 场景（打包复用）+ 信号（解耦通信）
   ——一句话模型：*场景打包复用对象；节点构成对象树承载逻辑；信号让节点互通信而不互持有*。
   与 Unity(GameObject+Prefab+Event)/UE 对照表（差异最大处=独特范式）。
2. **节点 Node**：一切都是节点（继承树）、单父多子的树形约束（继承坐标/激活态）、
   内置属性与功能、**反巨型单脚本：拆分功能到子节点**的组合式设计。
3. **场景 Scene**：保存到文件（.tscn）的节点树 = 预制体、实例化/嵌套、
   角色/子弹/UI 做成独立场景的复用纪律。
4. **信号深入**：事件订阅解耦本质、A 不知 B、编辑器连接 vs 代码连接、
   连接生命周期与销毁语义（精确版：同树自动断连的边界）。
5. **delta 与时间**：帧间隔、帧率无关移动、_process vs _physics_process 的分工依据。
6. **2D/3D 空间**：Node2D/Node3D 的 position/rotation/scale、CanvasItem 层级、
   坐标系（局部/全局/屏幕）。
7. **资源 Resource**：与节点分离的数据容器、多节点共享同一份、
   内置资源（贴图/材质/音频/动画）与自定义 Resource 子类。
8. **脚本即组件**：脚本附加在节点上，是节点的组件而非独立对象——
   修正"GDScript 是 OOP 语言"的直觉偏差。

## 三之三、引擎内部篇（逆向视角：Godot 程序如何被引擎处理）

素材直接取自本项目逆向规格 `docs/engine-analyzer.md`（U/N/S/V/P 五章，源码锚点齐全），
改写为面向使用者的「引擎怎么处理我写的东西」：

1. **工程启动链路**：project.godot → autoload 注册（`*` 全局标志）→ 主场景实例化
   （P1 章：config_version>5 拒载等致命面）。
2. **场景文件 .tscn 解析**：`[node]` 标签字段表、实例化线性顺序与 parent 前向引用限制、
   owner 语义与 `%Name` 唯一名查找（N1-N3 章）。
3. **UID 系统**：uid:// 编码算法（U1）、CSPRNG 生成（U2）、uid 无效时 WARN 回退 path
   ——解释 .uid 文件、编辑器如何维持稳定引用、为什么 git 要提交 uid。
4. **信号连接的存储**：[connection] 四字段（from/to/signal/method 缺一拒载，S1）、
   连接是声明式数据而非代码、端点丢失 → 静默丢弃（S2/⚪ 级）。
5. **Variant 序列化**：tscn 值的写入格式（V1 字符串转义规则 + V2 常用构造串）——
   为什么你在编辑器里填的值和文件里长的不一样。
6. **资产管线**：.import 文件与导入产物、res:// 虚拟文件系统、preload/load 的时机差异、
   资产间引用关系图（脚本↔场景↔资源↔uid 全联通）。
7. **脚本致命面**：autoload 不继承 Node 被静默跳过、owner 为空的 %Name 永远查不到、
   parent 消失节点被静默删除（P2 章 + ⚪ 静默失败合集）——
   此篇同时是「为什么 LSP 要做这些检查」的说明书，与 godotlsp 诊断能力互相印证。

## 三之四、思想篇（DOD / ECS / Jai —— 组合大于继承之后的"为什么"）

定位：讲清现代游戏范式的思想源头。范式篇讲「怎么用 Godot」，本篇讲「为什么业界往
这个方向走」。**概念内容，不入 LSP 锚点契约**（无对应 API，hover 不指向这里）。

1. **数据导向设计（DOD）** ⭐：硬件真相——cache line（L1 命中 ~4 周期 vs 内存
   ~200-300 周期）、SoA vs AoS 布局、分支预测与批量处理；核心句「先按访问方式
   布局数据，再写代码」；与 OOP 的取舍边界（不是替代，是热路径武器）。
2. **ECS 架构** ⭐：Entity(ID)/Component(纯数据)/System(批量逻辑) 三支柱；
   组合优于继承的架构化表达——深继承的真正死因（组合爆炸 + fragile base class，
   非"代码丑"）；「ECS 只在规模上更快」的精确边界；Unity DOTS / Bevy / Unreal Mass
   对照表；**Godot 为什么不是 ECS**（节点树是 scene graph + 组件混合体）。
3. **Jai 与 Jonathan Blow 理念** ⭐：为游戏而生的语言（Braid/The Witness 作者）——
   无隐藏控制流（无异常/无 RAII/无隐式构造析构/无隐式分配）、`#run` 编译时执行、
   context allocator、百万行秒级编译（80k 行 <1s 实测）、"给好程序员的工具，不防
   傻瓜"；对 Zig/Odin 的血统影响；与本项目 godotlsp 的共鸣（快编译=快迭代=生产力）。
4. **Godot Server 架构即 DOD**：RenderingServer/PhysicsServer/AudioServer——场景系统
   是可选层，一切高阶 API 的底层都是 RID 句柄 + 批量指令；何时不走节点树
   （万级同屏实例）；Bullet Shower 官方示例。

## 三之五、性能篇（Godot 优化手段 + 高性能代码实施）

定位：官方 performance 文档体系（general_optimization / cpu / gpu / using_servers /
high-end 诸篇）+ 社区实证的 GDScript 热点知识的体系化。**同不入 LSP 锚点契约**。

1. **优化方法论**：三类慢（每帧持续低帧率/间歇卡顿/加载慢）→ 测量先行
   （内置 Profiler、Monitors、`Time.get_ticks_usec()` 手动计时、外部 Tracy/Perfetto/
   RenderDoc）；假设检验 + 二分定位；Knuth 原则——先怀疑算法再怀疑代码；
   「烟雾与镜子」（能用骗的不用算的）。
2. **GDScript 高性能实施** ⭐：四类热点杀手——①无类型变量/数组（typed GDScript
   是真实加速：跳过运行时类型解析，`Array[T]` 防 Variant 装箱）②热循环里
   `get_node`/`get_nodes_in_group`（树遍历，须 @onready 缓存）③_process 内字符串
   操作（每帧两次分配）④重复 connect（不去重，槽位翻倍）；升级阶梯：
   修算法 → 加类型 → 移植单热点到 C# → GDExtension（顺序不可颠倒）。
3. **批量与结构优化**：万 Node 各跑 `_process` 的调度开销 → 单管理器遍历
   typed array；MultiMesh；Server API 绕过场景树；Nystrom 优化模式 Godot 版——
   对象池（子弹/特效）、脏标识（world transform 缓存）、数据局部性、事件队列。
4. **物理与渲染预算**：physics tick 降档 + 物理插值；碰撞形状 primitive 优先；
   睡眠与远距 body；渲染三后端选型（Forward+/Mobile/Compatibility）；
   GI 分档（lightmap 静态世界 / VoxelGI 动态室内 / SDFGI 开放世界）；
   AA 取舍（TAA 真实向 / MSAA 2x-4x 风格化 / FXAA 兜底）；Environment 开销
   静默叠加警告。

## 三之六、算法篇（游戏算法与设计）

定位：游戏专用算法的「原理讲透 + Godot 落地 + 延伸阅读」（延伸资源：Nystrom
《Game Programming Patterns》免费在线、Red Blob Games 交互可视化、Gaffer on Games）。

1. **游戏循环与时间**：fixed timestep 与可变 timestep、Gaffer「Fix Your Timestep」
   （accumulator + 插值渲染）、与 `_process`/`_physics_process` 的精确映射、
   delta 与时间缩放。
2. **AI 与寻路** ⭐：A*（F=G+H、启发函数表：曼哈顿/欧几里得/对角线、admissible
   准则）、Grid vs NavMesh、flow field（群体寻路）、状态机/下推自动机/行为树、
   steering behaviors 与 boids 三定律。
3. **空间数据结构**：hash grid/quadtree/BVH——O(n²) 碰撞对查询 → 邻域查询；
   Godot 物理内部即此结构；何时自己造（boids 上百实例）。
4. **噪声与程序化生成**：value noise → Perlin/Simplex → fBm 分形叠加；
   Roguelike 地牢（房间放置 + 迷宫回溯连接）；高度图着色地形；
   深入指向 Red Blob Games（多边形地图/蓝噪声/Voronoi）。

## 三之七、数学篇（游戏数学 + 图形学速览）

定位：quant67「游戏中的数学」路线的 Godot 版——解释引擎 Mathf/Vector API 背后的
数学，而非 API 罗列。语法篇 04-types 只做类型速览，数学原理全部归此篇。

1. **向量实战**：点积/叉积的几何意义与应用（朝向判断、视野锥、法线、左右手系）、
   距离族（欧氏/曼哈顿/切比雪夫）。
2. **插值家族**：lerp/InverseLerp/Remap/Clamp/SmoothStep、缓动函数、
   贝塞尔曲线（相机路径/轨迹）。
3. **变换与四元数**：矩阵/欧拉角/四元数三表示对比、万向节死锁、slerp、
   Transform2D/3D 与 Godot 变换语义（local/global）。
4. **几何检测**：Raycast 原理、AABB、圆-圆/矩形碰撞、SAT 分离轴、
   到 Godot Physics2D/3D API 的映射。
5. **随机与概率**：RNG 种子与可复现、RPG 伤害概率分布、加权抽取。
6. **图形学速览** ⭐：渲染管线五阶段（剔除→阴影→不透明→透明排序→后处理）、
   Blinn-Phong 光照模型（点积直觉）、法线贴图/阴影映射/后处理概念层、
   Godot shader 入口（衔接 P2.6 GDShader 支持）与 gdshader 内建速查页互链。

## 四、类参考篇（生成部分，分期）

规模现实：Godot 4.x ~900 类，逐类中文精译是数月级工作。分期：

- **P0（hover 高频集，全量精译）**：`@GDScript`（全部内建函数）、`@GlobalScope`
  （全部工具函数/枚举/常量）、约 50 个核心类——全部 Variant 数学/容器类型、
  Object/RefCounted/Node/Node2D/Node3D/Control 系主干、常用 Resource。
  与内核 `gdscript_docs.rs` 文档表互为校验。
- **P1（全类骨架）**：900 类全部生成——签名表 + 一句话简介 + 成员锚点齐全
  （跳转链接 100% 可用），正文后续填充。
- **P2（长尾精译）**：按使用频率逐批补全中文详解。

骨架先行保证「链接全部有效」，精译保证「质量」——两件事解耦。

## 五、部署

- `.github/workflows/pages.yml`：checkout → 装 mdbook（rust-lang 版本锁定）→
  `mdbook build website` → `actions/deploy-pages` 上传 `website/book/`。
- Pages 挂 project pages：`https://santachains.github.io/godotlsp/`
  （BASE 以此为准，实施前确认仓库 owner）。
- 版权页：godot-docs 内容 CC BY 3.0，站内署名 + 标注修改 + 版本锁定说明（4.x stable）。

## 六、执行顺序（确认后开工）

1. W1 清空 `website/` 重建骨架：book.toml + SUMMARY + intro（✅ 2026-10-16）。
2. W2 `tools/gen_docs.py` + P0 类生成 + 内核 `url` 模块改造 + 防漂移测试（核心闭环）。
3. W3 语法篇 16 篇（AI 起草，godot-docs 语料逐篇核验）。
4. W4 范式篇 8 篇 + 引擎内部篇 7 篇（engine-analyzer.md 改写）。
5. W5 P1 全类骨架 + Pages 工作流上线。
6. 迭代：P2 精译、附录（高频坑/迁移表/学习路线）、插件入口链接。
7. **P3 思想篇 4 章 + 性能篇 4 章**（范式思考的纵深；素材：官方 performance
   文档、Nystrom、Jai 公开资料——三之四/三之五）。
8. **P4 算法篇 4 章 + 数学篇 6 章**（素材：Red Blob Games、quant67 游戏数学、
   Gaffer on Games——三之六/三之七）。
9. W3 起执行 §五之二 写作规范；paradigm 补 09-composition（组合优于继承深挖，
   与思想篇 01-02 互链）。

### 取舍声明（明确不做）

- **不做完整图形学教材**：GPU 架构/光栅推导/BRDF 数学不展开，图形学速览止于
  「看懂 Godot 渲染选项在买什么」的深度，深水区走延伸阅读。
- **不复刻 Nystrom/Red Blob 全书**：模式与算法只收 Godot 语境下高频者
  （约 15%），其余链接延伸；避免与原书重复劳动。
- **ECS 不教自研**：Godot 无官方 ECS，教「何时用组合/何时上 Server/MultiMesh」
  而非在 GDScript 里手写 ECS 框架（性能和语言都不支持，反模式）。
- **Jai 只讲理念不讲语法**：语言未公开发布，理念层（无隐藏控制流/DOD/编译时）
  用于校准思维，代码示例一概不用 Jai。
- **思想/性能/算法/数学四篇全部不入 LSP 锚点契约**：hover/doc_link 只指向
  syntax/builtin，概念篇纯阅读向，防漂移测试范围不变。

## 五之二、写作风格规范（参照系：rust-course，refs/rust-course 实证抽样）

主题：浅色 = mdbook 默认 **light** 主题（classic blue，rust-course 线上即此档，book.toml
`default-theme = "light"` 已锁定）；深色 = 「影」黑泽明意象（kurosawa.css 覆盖 html.navy）。

抽样 basic/base-type/numbers.md、basic/compound-type/struct.md、SUMMARY.md 提炼：

### 借鉴（进入 W3/W4 成文规范）

1. **表格先行**：类型/字面量/默认值准则一律给表，表后接"使用准则"一句话
   （如"整型默认 i32，首选它"）。我们的运算符全表/注解全表/类型表直接沿用此模式。
2. **段内粗体锚定概念**：每个新概念首次出现即 `**粗体**`，后跟定义句。
3. **短代码 + 中文注释 + 输出内联**：示例 ≤ 15 行，关键行带 `// 注释`，
   println 输出以 `// 输出` 行内标注。拒绝长篇完整工程代码。
4. **blockquote 作补充说明 + 交叉引用**：不用 admonish 插件，`>` 引用块开头
   固定措辞「值得注意的是：」「需要注意的是：」，内部链接指向本站其他章节锚点
   （LSP hover 链接契约同此）。
5. **跨语言类比**：「如果你从 X 语言过来，会注意到……」句式，X 取
   Python/Lua/TypeScript/C#（GDScript 受众主池）；Unity/UE 对照表在范式篇。
6. **每篇章节首 intro 页**：rust-course 每个分篇都有 intro.md 概览——我们的
   syntax/paradigm/internals 各加一篇 intro（导读 + 前置知识 + 与其他章关系）。
7. **章末「延伸阅读」**：每章末尾给官方英文文档对应页 + 本站相关章链接，
   双源核验闭环（我们写每章时本就对照 godot-docs 语料，链接是副产物）。

### 不照搬

- **幽默口语化开场**：rust-course 的段子钩子适合教程连载；我们是参考文档 +
  LSP 跳转目标，开场直接给结论（呼应先结论后展开原则），段子一律不要。
- **playground / 在线运行**：GDScript 无 rust-playground 等价物，不设。
- **侧栏折叠（fold）**：mdbook 0.5 已删除该功能（book.js 无 fold 代码路径，
  源码级核验），不追求。
- **quiz 外链练习站**：无对应资源，不做。

### 成文硬约束（W3/W4 起执行）

- 每章 h1 唯一；h2 分节、h3 子节、#### 细则（对齐 rust-course 层级纪律）。
- 代码块语言标注 `gdscript`（插件/CI 侧后续可做语法高亮 lint）。
- 所有站内跳转用相对路径 + 显式锚点（`./09-annotations.md#@export`），
  与 LSP doc_link 契约一致——防漂移测试覆盖的对象在 W2 定稿。
