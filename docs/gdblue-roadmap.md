# GDBlue 路线图 — Godot 可视化蓝图(独立文档)

> **定位**:本文是蓝图系统的**专属路线图**,从 `gdscript-features-roadmap.md` P3-23 迁出并扩写。
> 主路线图只留摘要指针;蓝图相关的所有设计、源码依据、分期与验收集中于此。
> 源码依据:godotengine/godot-visual-script(2026-10-16 clone 逐文件核验,工作副本 `$TEMP/vs-study`)。
> 画布体系参照:hall/draw(同日核验,取舍见 §5)。

## 0. 构想

把 Godot 3.x 的 Visual Script(4.x 已移除)以现代化蓝图形态复活在 VS Code——类 ComfyUI / Unity Visual Scripting 的节点图编程,原生状态图(状态机),面向数据编程(DOP)。
**蓝图是「人机共享 IR」**:AI 读写结构化 JSON,人操作可视化编辑器,产出经类型校验的 GDScript——这是「AI 更好理解和干活」的架构级答案。

## 1. 源码依据(godot-visual-script 核验结论)

- **38 个节点类**(全清单见 §3 分类学):流控 7 / 函数属性 4 / 数据 21 / 场景引擎 5 / 表达式 1;
- **双通道图 IR**:每节点两类端口——**sequence ports**(执行流,输出可多个且带文本标签,如 Condition 的 true/false)+ **data ports**(`PropertyInfo` 类型化输入/输出);资源持有 `sequence_connections` 与 `data_connections` 两组边集(visual_script.h:229-230);函数 = 节点集(id→node)+ 边集;
- **协程**:YieldSignal 节点 = `await signal`;**逃生舱**:Expression 节点 = 内联单行 GDScript;CustomNode = 用户脚本节点;
- **执行**:VisualScriptInstance 逐节点解释(sequence_outputs 索引跳转)。

## 2. 原版失败教训 → 现代化三招

Godot 4 移除 VS 的原因:动态类型端口、编辑器内嵌引擎的维护成本、与 GDScript 2.0 重叠。对应三招:

1. **端口静态类型化**:类型来自字面量推断/变量声明/函数签名(经 gdscript-engine 查询),连接传播检查,错型编辑器红线——原版全 Variant 是被移除的核心原因之一;
2. **编辑器在 VS Code**:引擎无关,复用 P2-19 E1 webview 基建(React Flow + ELK layered 布局,蓝图编辑器本质 = 可编辑的资源图谱);
3. **JSON 序列化**:`.gdblue.json`——AI 可直接读写的结构化 IR(原版 .vs 资源格式对工具链/AI 不友好)。

## 3. 节点分类学(38 类 → 6 组,V1 实现清单)

| 组 | 节点(VS 原名) | → GDScript 生成 |
|---|---|---|
| 流控 | Sequence / Condition / While / Iterator / Switch / TypeCast / Return + **新增 Match** | 语句序列 / if-else(端口 true-false)/ while / for / switch / is-as / return / match |
| 函数属性 | FunctionCall / PropertyGet / PropertySet / EmitSignal | 调用表达式 / 读 / 写 / emit |
| 数据 | VariableGet(+Set)/ LocalVar(+Set)/ Constant(4 类合一)/ Preload / IndexGet(+Set)/ Deconstruct / Constructor | 标识符/临时变量/字面量/构造/下标/解构 |
| 场景引擎 | Self / SceneNode(%unique + NodePath)/ SceneTree / EngineSingleton / ResourcePath / InputAction | 对应表达式 |
| 表达式 | Expression(单行 GDScript 逃生舱)+ **新增 CustomNode**(.gd 实现 `process(inputs)->outputs` 契约的用户节点) | 内联 |
| 协程 | YieldSignal(现代化为 await 语义) | await 语句 |

**核心洞察**:VS 节点本就是 GDScript 语句的图形化——蓝图→GDScript 是 **1:1 结构映射,不是编译器工程**。

## 4. 分期

### V1 — BIR 规范 + 只读渲染
- `.gdblue.json` schema:`functions[]{name, nodeId} + nodes{id, kind, params, ports{in[], out[]}(类型化)} + sequenceEdges[]{from, outPort, to} + dataEdges[]{from, outPort, to, inPort}`;
- React Flow **只读**视图;.gdblue 文件类型注册(图标/语法/语言);与资源图谱(P2-19 E1-G)互通:蓝图文件是图节点,蓝图内 preload 引用是边;
- ELK 自动布局(配置锚点:`hierarchyHandling: INCLUDE_CHILDREN`、`edgeRouting: ORTHOGONAL`);
- **CustomDocument 自定义编辑器**(§9.2):CustomEditorProvider + CustomDocument + onDidChangeCustomDocument(撤销/重做),双击即图、右键回文本。

### V2 — 编辑器
- 端口拖拽连线/断开、默认值编辑、**静态类型检查红线**(连接传播)、节点库搜索(复用主路线 21b 搜索语法)、双通道边视觉区分(执行流实线箭头 / 数据流虚线);
- **cable 自动落端口**(§9.2 Blackprint 实证:拖线到节点自动吸附类型匹配端口)+ **webview↔host 增量同步**(diff 消息,不全图重传);
- ⚠ 连线 UX 是全项目最大交互深水区——先只读后编辑,鼠标优先,键盘/手写远期。

### V3 — transpile → GDScript(核心闭环)
- 生成策略按 §3 映射表直译:data 边 → 临时变量(`var _n3 = ...`,命名含节点 id 保证可读可追溯);函数图 → 一个 GDScript 函数;
- **蓝图 JSON 是 SSOT,生成 .gd 带头标注为派生物**;取舍:单向生成、放弃手改生成物(手写需求由 Expression/CustomNode 逃生舱承接;逆向 .gd→蓝图明确不做,见 rejected-designs.md);
- 生成物进 gdscript-engine parse/诊断验证闭环(editing-rules R10 双验证)。

### V4 — 状态图 + ECS/DOP 模板
- **State Graph(原版没有,对标 Unity Visual Scripting)**:State 节点 + Transition 边(guard 条件端口)+ Entry/Exit;生成**组合式状态机**:状态枚举 + `_enter_state`/`_exit_state` 钩子 + match 驱动——**组合大于继承:状态是数据不是子类**;
- ECS/DOP 映射:组件 = Resource(.tres 纯数据)/ System = 函数或蓝图 / Entity = 场景节点;模板包脚手架;
- 生成代码性能模式:Typed Array/PackedArray、避免每帧分配。

## 5. 画布体系研究(hall/draw,2026-10-16 clone 核验)

`hall/draw`("Draw an SVG image with a mouse or pen",~2800 行,零图形库依赖)的可借鉴与取舍:

- **借鉴思想**:①**轻量 SVG 自绘完全可行**——纯 path 生成函数(rect2path/ellipse2path)+ path-int.js 路径相交检测(hit-test),证明 webview 内不引重型图库也能做画布 → 作为 GDBlue V2 的**降级备选路线**记录(若 React Flow 打包体积/可控性出问题,自绘 SVG 层是务实退路);②**手写笔输入**(mouse or pen)与手写识别 provider 模式(MyScript iink-js / Mathpix,secrets 管理)→ 远期「手绘草图 → 蓝图节点」的参照;③行内锚定工作流(SVG 内联 markdown 行 → webview 编辑 → 写回)——不采纳(蓝图 SSOT 是独立 JSON 文件,非文档内嵌),但其「编辑器状态 ↔ 文档写回」的往返纪律可借鉴;
- **取舍结论**:React Flow 主路线不变(draw 无节点图系统,是自由绘制白板);引其「零依赖自绘可行 + 笔输入远期」两个思想,不引代码。

## 6. AI 协同(「更好的理解和干活」的架构答案)

- 蓝图 JSON = 人机共享 IR:**AI 生成/修改蓝图必须过端口类型校验 = 生成代码的类型安全网**(自由文本 GDScript 没有这层);
- gdblue → gd 生成器输出进 gdscript-engine 验证闭环;LSP 对蓝图文件提供 hover/补全(节点库/端口类型);
- AI 工作流:LLM 写 .gdblue.json → 插件校验/渲染 → transpile → 验证 → 进工程;人工在编辑器微调,双向同一 IR。

## 7. 风险与控制

- 范围爆炸:V2 连线 UX 深水区(先只读后编辑,分期硬门);
- 生成代码可读性:节点 id 命名 + 分区注释 + 头部 SSOT 标注;
- 蓝图间引用(preload 别的蓝图):纳入资源图谱防循环检测;
- 依赖边界:V2+ 依赖 P2-19 E1 webview 基建(React Flow/ELK vendored);V3 依赖 gdscript-engine(R2 后)。

## 8. 验收

- corpus 状态机夹具(敌人 AI patrol/attack/flee):蓝图 → 生成 GDScript → 引擎运行行为等价;
- 端口错型连接全部静态拦截(类型检查夹具);
- 生成代码过 gdscript-engine 零诊断;AI 端到端(LLM 写蓝图 → 引擎可运行)演练一次。

## 9. 外部参照二轮:ComfyUI/Blackprint 类资料取舍(2026-10-16)

> 输入:一份「VS Code 节点编辑器可行性分析」(ComfyUI/Unity DOTS/三层架构/Canvas 渲染)。逐点对照本路线图与 rejected-designs 的取舍如下——**核心结论:三层架构的前两层与本设计同构,第三层(运行时)与本路线冲突,维持否决;渲染建议与已核验选型冲突,维持 React Flow。**

### 9.1 维持否决的两组建议(理由强化)

- **「运行时层:Godot 内解析 JSON 执行数据流图 + 拓扑排序增量重算」——否决,维持 V3 transpile 路线**。理由:①这正是 rejected-designs 已缓行的「蓝图运行时解释器」,而 godot-visual-script 的 VisualScriptInstance 逐节点解释本就是原版性能包袱与 4.x 移除诱因(§1-§2)——资料等于建议重建一个被引擎弃掉的解释器,再加一层拓扑排序增量优化器,复杂度远超我们的 1:1 直译;②生成 .gd 走引擎原生工具链:断点/分析/导出/静态检查全部免费继承,解释器路线全部要自建;③资料自己的「JSON 驱动、可热重载」在 transpile 路线下同样成立(变更 → 重 transpile → gdscript-engine 验证,V3 管线)。
- **「务必 Canvas 2D/WebGL 渲染(比 DOM 快得多)」——否决,维持 React Flow + ELK**。依据主路线图 P2-19 已核验的选型证据(双源 benchmark、ELK 复合边、确定性布局);决定性事实:**蓝图 = 函数级画布**(原版 38 节点类、per-function 组织,典型 <100 节点),远在 DOM/SVG 舒适区;React Flow 自带视口虚拟化(仅渲染视口内节点),资料担心的卡顿场景不存在;Canvas 自绘只在 >2000 节点全工程大图触发(升级线已记),蓝图编辑器不在此域。ComfyUI 用 canvas 系渲染不构成证据——它的画布是任意规模工作流,形态不同。

### 9.2 吸收清单(三项进分期,两项进参照系)

- **CustomDocument 自定义编辑器(V1/V2 落地要点)**:`CustomEditorProvider` + `CustomDocument`(打开/保存/backup)+ `onDidChangeCustomDocument`(撤销/重做)——VS Code 自定义编辑器正道,.gdblue.json 双击即图、右键「Open as Text」随时回文本(Blackprint .bpi 同款交互;我们的 SSOT 本就是 JSON 文本,零成本满足);
- **cable 自动落端口(V2 连线 UX 减压)**:Blackprint 实证——拖线到节点时自动吸附到类型匹配的合适端口(而非必须精确命中端口),配静态类型检查(现代化三招之一)是天然组合,直接降低「连线深水区」的交互压力;
- **webview↔host 增量同步(V2 实现要点)**:编辑消息只发 diff(变更节点/边),不做全图重传——与主路线 JSON Flow 100ms 节流同构,防通信抖动;
- **Blackprint 进参照系**(MIT,212★,v0.10 活跃):①**skeleton schema**(节点库的机器可读注册表:skeleton.json 描述节点树/端口类型,编辑器据此渲染与校验)——与我们的 BIR node kinds + 「LSP 对蓝图文件 hover/补全」(§6)同构,数据形状可参照;②**code-generation 是其官方路线**(JS 进行中、Python/C# 实验)——独立佐证「可视化编程的终局是生成宿主语言代码」,与我们 transpile 决策同向;③hot reload / mini preview / cable branching 等交互清单作 V2 校对清单。Flyde 同为「扩展 + 运行时分离」先例,参照价值同上不单独展开。
- **存疑与谬误标注**:①资料所引「TypeScript Genge」查无此物(Godot 生态无可核验的该扩展),按疑似幻觉处理不采纳;②「Unity DOTS 证明了可视化编辑可行」是论证瑕疵——Behavior Designer/State Designer 是第三方插件且与 DOTS 无关;状态图可视化参照我们 V4 已对标的 Unity Visual Scripting 即可。

### 9.3 与现有设计重合(无增益,记录避免重复劳动)

「每个函数一张独立画布」「控制流节点(Branch/ForEach/混合数据流+控制流)」「组合优于继承」「JSON 中间表示可版本化/序列化/分享」——分别对应本路线图 §1(函数=节点集+边集)、§3 流控组(38 类含 Condition/While/Iterator)、§4-V4(组合式状态机)、§2-招3(.gdblue.json SSOT),已全覆盖;资料的增量价值集中在 9.2 三项交互细节。
