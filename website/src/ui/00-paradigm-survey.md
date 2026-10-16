# UI 范式全景

**Godot 官方不规定 UI 架构，只给积木（Control/Container/Theme/CanvasLayer/信号）。MVC/MVP/MVVM 是三种经典分法，Godot 社区落地为信号解耦、被动状态、封装控件、屏幕管理器等变体。按规模选，不要一上来上最重的。**

## 引擎立场

> 值得注意的是：Godot 文档只讲控件、容器、锚点、主题、信号怎么用，从不要求你用哪个 MVC 变体。架构是你在积木之上自己选的组织方式。官方反复强调的只有一条：用信号解耦，不要让对象互相硬引用。

## 经典三模式对比

| 维度 | MVC | MVP | MVVM |
|---|---|---|---|
| 中介角色 | Controller | Presenter | ViewModel |
| View 姿态 | 较主动，可直接读 Model | 被动，只转发输入、显示给的数据 | 绑定对象，自动响应状态 |
| 同步方式 | Controller 显式通知 | Presenter 手动双向同步 | 状态绑定自动推送 |
| 可测试性 | 中 | 高（View 可 mock） | 高 |
| 样板代码 | 少 | 多（View/Presenter 委托） | 中（需绑定机制） |
| 主要缺点 | View 与 Model 仍可能耦合 | Presenter 易膨胀 | 自动绑定调试难 |

### MVC（Model-View-Controller）

```
用户输入 → Controller → 改 Model
Model 变化 → 通知 View → View 读 Model 重绘
```

Controller 是独立中介，接收输入后决定改哪个 Model，再让 View 刷新。View 保留一定主动性（自己读 Model 渲染）。

### MVP（Model-View-Presenter）

```
View 转发输入给 Presenter → Presenter 调 Model → Presenter 写回 View
```

View 完全被动，不含业务判断，所有协调在 Presenter。换一套 View（比如测试替身）Presenter 不变。代价是 View 每个动作都要委托，Presenter 容易写成大杂烩。

### MVVM（Model-View-ViewModel）

```
View 绑定 ViewModel 的可观察状态 → 状态一变，View 自动刷新
```

ViewModel 暴露状态，View 通过绑定自动更新，没有手动同步代码。Web 的 React/Vue、WPF 是典型。GDScript 没有内置数据绑定，要自己写一层响应式对象（见[响应式封装实现](02-reactive-wrapper.md)）才能近似。

## Godot 社区落地变体

### 1. 信号观察者（地基，官方推荐）

游戏对象/Model 发信号，UI 订阅后刷新。这是上面所有模式的共同底座，Godot 的 signal 就是观察者模式实现。

```gdscript
# Model 发，UI 订阅，互不持有引用
signal health_changed(current: int, max: int)

# HUD 只连接，不认识 Player 内部
player.health_changed.connect(_update_bar)
```

### 2. 被动模型 + 信号广播（MVVM-lite）

数据独立成状态，控件经信号读写，无独立中介。即本站[状态与控件分离](01-state-separation.md)和[响应式封装](02-reactive-wrapper.md)讲的做法，菜单/雇佣列表这类多联动界面最常用。

### 3. 自封装自定义控件

官方 [Custom GUI Controls](https://docs.godotengine.org/en/stable/tutorials/ui/custom_gui_controls.html) 的思路：一个控件场景自包含，内部子节点不外泄，对外只暴露少量方法和自定义信号。

```gdscript
# 血量条对外只给一个方法，内部 Label/TextureProgressBar 藏起来
class_name HealthBar extends Control
signal died
func set_hp(current: int, max: int) -> void:
    $Bar.value = float(current) / max * 100
```

复用的是"一个完整控件"，别人不碰你内部节点。这与游戏对象的组合思想同源。

### 4. 面板场景 + 屏幕管理器

每个界面（主菜单/设置/暂停）是独立场景，一个 UIManager 负责切换、显示隐藏、焦点栈。避免每个面板自己决定谁该显示。

```gdscript
# UIManager（autoload 或顶层 CanvasLayer 节点）
var panels := {}
func show_panel(name: String) -> void:
    for p in panels.values():
        p.visible = false
    panels[name].visible = true
```

### 5. CanvasLayer 分层

HUD、菜单、弹窗放在不同 `CanvasLayer`，按 `layer` 数值决定绘制先后，脱离相机变换固定在屏幕。暂停菜单用 `PROCESS_MODE_ALWAYS`，游戏暂停时仍可响应。

| 层 | 内容 | layer 值（建议） |
|---|---|---|
| 世界 | 2D/3D  gameplay | 0 |
| HUD | 血量/计分 | 1 |
| 菜单 | 主界面 | 10 |
| 弹窗 | 确认框/提示 | 20 |

### 6. 事件总线（信号总线 autoload）

一个全局 autoload 集中放跨场景信号，新实例化的节点不用靠脆弱的 NodePath 找人。

```gdscript
# EventBus.gd（autoload）
signal coin_collected(amount: int)
```

> 值得注意的是：总线只放"跨系统广播事件"，不要把它变成存数据的全局垃圾桶。状态仍归状态层，总线只传事件。

### 7. Theme：样式与结构分离

`Theme` 资源集中管字体、颜色、StyleBox，类比网页 CSS。换皮肤只改 Theme，不动控件层级和逻辑。

## 按规模选型

| 规模 | 推荐 | 不上什么 |
|---|---|---|
| 单按钮/单血条 | 脚本直接赋值 | 任何模式 |
| 3-5 个联动控件 | 信号 + 被动状态（变体 2） | 独立 Controller/Presenter |
| 多面板菜单导航 | 屏幕管理器 + 面板场景（变体 4） | 响应式绑定层 |
| 复杂看板/编辑器/数据表单 | MVVM-lite 响应式 + 封装控件 | 巨型单脚本 |
| 跨系统全局通知 | 事件总线（变体 6） | 满树找节点 |

## 反模式

| 反模式 | 后果 |
|---|---|
| 一个 Godot 脚本管全部控件和数据 | 改一处崩三处 |
| 控件直接 `$OtherPanel/Label` 互找 | 删节点即崩，无法复用 |
| Model 里写显示/排版逻辑 | 数据无法独立测试与复用 |
| 事件总线存全部数据 | 全局耦合，依赖关系看不清 |
| 为单按钮上 MVVM | 样板超过业务，过度设计 |

## 延伸阅读

- 本站：[状态与控件分离](01-state-separation.md)、[响应式封装实现](02-reactive-wrapper.md)、[信号深入](../paradigm/04-signals.md)
- 官方：[Custom GUI controls](https://docs.godotengine.org/en/stable/tutorials/ui/custom_gui_controls.html)、[Containers](https://docs.godotengine.org/en/stable/tutorials/ui/gui_containers.html)、[Themes](https://docs.godotengine.org/en/stable/tutorials/ui/themes/index.html)
