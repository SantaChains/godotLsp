# 信号导读

**本目录是信号的「发现层」：按域平铺列出引擎全部声明信号，每条给原型、含义、连接用法，全文长描述链回 builtin 类页。信号怎么声明、发射、连接、传参、断连，见[语法篇 · 信号系统](../syntax/10-signals.md)与[范式篇 · 信号深入](../paradigm/04-signals.md)，本目录不重复讲机制。**

## 怎么读每条信号

每条三列：

| 列 | 内容 |
|---|---|
| 原型 | `signal 名字(参数: 类型)`，来自 extension_api.json，与 LSP 补全/hover 同源，是唯一事实源 |
| 含义 | 触发时机；已中文化为主，少数本地无描述的深层扩展信号标注「待补」并链类页 |
| 使用方式 | 连接写法与回调签名，`obj` 指代接收该信号的节点实例 |

## 三个必须先懂的语义

### 1. 信号在声明类上，接收方可以是子类

信号只在**声明它的类**上定义一次，子类经继承直接拥有。最典型：`pressed` 声明在 `BaseButton`，`Button`、`CheckBox`、`CheckButton`、`OptionButton`、`MenuButton` 全都继承它，自己的类页里不重复定义。索引把链接指向**声明类**（BaseButton），你在任何按钮上都能连。

```gdscript
# pressed 声明在 BaseButton，Button 实例直接可用
$StartButton.pressed.connect(_on_start)
```

判断信号归谁：沿继承链向上找第一个声明它的类。`ready` 在 Node，`mouse_entered` 在 Control，`body_entered` 在 Area2D/Area3D。

### 2. 同名信号在兄弟类各自声明，别混用

`body_entered` 同时声明在 Area2D、Area3D、RigidBody2D、RigidBody3D；`finished` 跨音频播放器、粒子、视频、Tween 七处。名字相同但**参数类型与触发主体不同**，连接时以接收节点的实际类型为准，不要凭名字套回调。

### 3. 编辑器信号只在编辑器内触发

08-editor 各 `Editor*` 类的信号只在编辑器插件环境触发，导出的游戏运行时不会发射，不要写进玩法逻辑。10-servers 是 RenderingServer 等低层回调，普通玩法一般不直接连。

## 连接速查

```gdscript
# 基本连接
$Button.pressed.connect(_on_click)

# 带参信号，回调参数对齐原型
$Area.body_entered.connect(_on_body_entered)
func _on_body_entered(body: Node2D) -> void:
    pass

# 预绑额外参数（追加在信号参数之后）
$Button.pressed.connect(_on_click.bind(1))

# 一次性 / 延迟到帧末
$Timer.timeout.connect(_alarm, CONNECT_ONE_SHOT)
$Area.body_entered.connect(_on_hit, CONNECT_DEFERRED)

# await 单个信号（无需手动 connect）
await $Button.pressed
```

## 索引导航

见[分类索引](index.md)：基类生命周期、输入控件、物理区域、动画补间、音视频粒子计时、场景导航多人网络、资源瓦片、编辑器、XR、Server 低层。

## 待补边界（不臆造）

503 条信号中 488 条已中文化。剩余 15 条为 4.7 新增的深层 OpenXR 扩展节点（OpenXRFutureResult.completed、OpenXRInterface.session_synchronized/user_presence_changed、OpenXRPlaneTracker.mesh_changed、OpenXRRenderModel/Extension/Manager 的 render_model_added/removed/top_level_path_changed、OpenXRSpatialEntityExtension.spatial_discovery_recommended、OpenXRSpatialEntityTracker.next_changed/spatial_tracking_state_changed）以及 OpenXRBindingModifierEditor.binding_modifier_removed、VisualShaderNodeInput.input_type_changed。本地 4.7 XML 无这些类，官方最新页暂未取到可核验英文描述，故保留「待补」占位并链回类页，不编造含义；待官方文档补齐后按同一 CHINESE 覆盖表补译。

## 延伸阅读

- 本站：[信号系统](../syntax/10-signals.md)、[信号深入](../paradigm/04-signals.md)、[信号连接的存储](../internals/04-connections.md)、[await 与协程](../syntax/12-await.md)
- 官方：[Using signals](https://docs.godotengine.org/en/stable/getting_started/step_by_step/signals.html)
