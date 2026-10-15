# rejected-designs — 明确不做 / 排除 / 降级的设计决策汇总

> **定位**:本文是主路线图(`gdscript-features-roadmap.md`)与相关文档中**所有「不采纳/排除/降级」决策的聚合视图**——按类别重组,补充完整理由与**复活条件**。
> 原始决策的上下文仍在主路线图对应条目处(本文不替换、只汇总);新决策先落到主路线图,再同步到这里。

## 一、永久不做(除非前提被推翻)

| 决策 | 理由 | 复活条件 |
|---|---|---|
| 离线授权/注册码/机器码体系(类 fnMap) | 本项目开源 MIT;绑定机器码与用户利益冲突 | 项目转商业模型(不在构想内) |
| 第三方付费图库(Streamline/Flaticon 等) | 授权链复杂;图标纪律已定:VS Code codicon 优先 → Godot 官方图标(MIT,509 节点图标)→ iconify 通用兜底 | 免费来源无法覆盖的特化图标需求 |
| GoJS / yFiles(商业图布局引擎) | 商业授权 + vendored 打包冲突(yFiles 闭源) | 无——其层级布局质量作为 ELK 调参标杆参照 |
| 云厂商架构图工具(AWS Application Composer 等) | 面向基础设施拓扑,与本地代码库资源图谱是不同问题域 | 做云端部署拓扑视图时(不在构想内) |
| 蓝图的 `.gd → 蓝图` 逆向工程(GDBlue) | 自由文本结构恢复图结构不可靠;蓝图 JSON 是 SSOT、生成 .gd 是派生物(单向) | 出现大量「手写 GDScript 想转蓝图」的真实需求(已有 Expression/CustomNode 逃生舱承接手写) |
| 手改蓝图生成的 .gd 生成物 | 再生成即覆盖,双 SSOT 冲突;单向 + 头部标注是唯一自洽形态 | 同上,逃生舱节点已覆盖手写需求 |
| 特定工作流硬编码(类 vscode-git-flow 的 master/develop/feature 固化 UI) | 项目组织习惯一变即废;资源图谱/大纲保持数据驱动(边类型来自文件事实) | 无——原则性排除 |
| 内嵌/连接 Godot 引擎运行时做语言服务 | 与「完全脱离 Godot」的核心差异化矛盾;官方插件已有引擎内 LSP | 无——v2 引擎模型走 `--dump-extension-api` 静态数据路线(from-scratch-design §7) |

## 二、技术排除(有更优替代,记录以防重蹈)

| 决策 | 理由 | 替代 |
|---|---|---|
| dagre 作为图布局引擎 | 对「子节点→父节点」复合边有 6 年未修致命 bug(issue #238);1000 节点 3384ms vs elk 619ms(5.5×);skew 216px vs 110px | **ELK layered**(`hierarchyHandling: INCLUDE_CHILDREN` + `edgeRouting: ORTHOGONAL`) |
| `create_id_for_path` 确定性 uid 派生复刻 | 需移植 Godot 私有 hash64 + RandomPCG;无硬需求驱动 | CSPRNG 随机 uid(engine-analyzer.md U2) |
| stderr 打印服务器信息日志 | vscode-languageclient 把 stderr 一律按 [error] 展示 | `window/logMessage` 通知(log_notify) |
| 主线程双句柄 salsa 写入 | salsa 0.27 `cancel_others` 单线程双句柄死锁(源码注释明言) | 「写者等待在飞读者(有界)」+ revision 比对(P2 已实现) |
| 通知路径(didChange 等)panic 捕获 | 半状态比崩溃重启更危险 | 崩溃即重启;线程模型+快照化后(R2+)重评 |

## 三、降级接受(TreeDataProvider 形态的能力边界)

| 决策 | 代价 | 升级条件 |
|---|---|---|
| 大纲树不迁 webview(Outline Map 式) | 做不到视口映射高亮/滚动动画;follow 只有光标→树定位 | follow 使用率证明需求,或与 GDBlue/资源图谱共用 webview 基建时一并立项 |
| 编辑器折叠层级包装命令(vscode-git-flow 类) | — | 无——vscode 原生 `editor.foldLevel1-5` 已覆盖,原则:不包装平台已有能力 |
| documentHighlight 旧内核病态(起点锚行首) | 插件 middleware + 服务器 clamp_word_range 双层矫正 | R2 重写内核根治(正确性回归 P1.5 条目 15 验收) |
| fnMap 的 pin/tag 置顶体系暂缓 | 临时标记/长期关注位置能力暂缺 | 大纲增强包(21)落地后按用户反馈评估 |

## 四、条件触发(远期立项,见主路线图 X 段与各条「远期」注记)

- X1 AI 资源管线(规格已备,等消费方);
- X2 确定性路径派生 uid(见上表);
- gd 函数级控制流 flowchart(CodeVisualizer 路线:Tree-sitter WASM → IR → Mermaid);
- 架构规则 lint(依 dependency-cruiser 思想:依赖规则即代码);
- 手绘草图 → 蓝图节点(hall/draw 的笔输入 + 手写识别 provider 模式参照);
- 大图 Canvas/WebGL 渲染层(React Flow 碰壁的升级路径:触发线 = 全工程大图 >2000 节点)。

## 五、缓行(方向认可,时机未到)

- **工作区符号池**(Outline Map 式文件级聚合):与项目树/P2-19 B 方案服务器图索引重叠,避免双索引,进图期统一;
- **GDBlue 运行时解释器**(蓝图 JSON → GDScript 微型解释器执行):性能差 + 生成物路线已闭环;仅当出现「热重载图逻辑不落盘」需求时重评;
- **3D 城市隐喻架构图**(CodeCharta 式):CI 管线级工程,收益与成本不匹配;其「指标多视觉通道映射」思想已被 codemap 式指标层吸收。
