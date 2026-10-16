<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->
# 基类与生命周期

> Object/Node/CanvasItem/Control/Viewport/Window/Resource/MainLoop 提供的通用信号，几乎所有节点都能收到。

## CanvasItem（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal draw()` | 请求重绘 _draw() 内容时触发。 | 覆写 `_draw()` 的自定义绘制节点可用它做额外钩子。 [类页锚点](../builtin/class_canvasitem.md#draw) |
| `signal hidden()` | 节点变为隐藏时触发。 | 仅隐藏事件，对应 visibility_changed 的隐藏分支。 [类页锚点](../builtin/class_canvasitem.md#hidden) |
| `signal item_rect_changed()` | 本项的包围矩形改变时触发。 | 依赖尺寸布局的自定义绘制用。 [类页锚点](../builtin/class_canvasitem.md#item_rect_changed) |
| `signal visibility_changed()` | 可见性（visible）改变时触发。 | 节点隐藏/显示后同步附加状态。 [类页锚点](../builtin/class_canvasitem.md#visibility_changed) |

## CanvasLayer（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal visibility_changed()` | 图层可见性改变时触发。 | 图层显隐同步。 [类页锚点](../builtin/class_canvaslayer.md#visibility_changed) |

## Control（继承 CanvasItem）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal focus_entered()` | 控件获得键盘焦点时触发。 | 输入框聚焦高亮外框。 [类页锚点](../builtin/class_control.md#focus_entered) |
| `signal focus_exited()` | 控件失去键盘焦点时触发。 | 失焦校验输入内容。 [类页锚点](../builtin/class_control.md#focus_exited) |
| `signal gui_input(event: InputEvent)` | 收到一条路由到本控件的输入事件时触发。 | 自定义控件拦截鼠标/触摸事件，事件类型判分支。 [类页锚点](../builtin/class_control.md#gui_input) |
| `signal maximum_size_changed()` | 最大尺寸建议改变时触发。 | 容器重新排版。 [类页锚点](../builtin/class_control.md#maximum_size_changed) |
| `signal minimum_size_changed()` | 最小尺寸建议改变时触发。 | 容器重新排版。 [类页锚点](../builtin/class_control.md#minimum_size_changed) |
| `signal mouse_entered()` | 鼠标指针进入控件区域时触发。 | 做悬停高亮：`$Btn.mouse_entered.connect(hover_on)`。 [类页锚点](../builtin/class_control.md#mouse_entered) |
| `signal mouse_exited()` | 鼠标指针离开控件区域时触发。 | 配合 mouse_entered 关闭悬停态。 [类页锚点](../builtin/class_control.md#mouse_exited) |
| `signal resized()` | 控件尺寸改变时触发。 | 尺寸自适应布局后重算内部元素。 [类页锚点](../builtin/class_control.md#resized) |
| `signal size_flags_changed()` | 伸缩标志改变时触发。 | 容器刷新。 [类页锚点](../builtin/class_control.md#size_flags_changed) |
| `signal theme_changed()` | 主题或主题属性改变时触发。 | 自定义皮肤后刷新派生样式。 [类页锚点](../builtin/class_control.md#theme_changed) |

## MainLoop（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal on_request_permissions_result(permission: String, granted: bool)` | 用户响应权限请求后触发，携带权限名与是否授予。 | Web/移动权限回调。 [类页锚点](../builtin/class_mainloop.md#on_request_permissions_result) |

## Node（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal child_entered_tree(node: Node)` | 某个子节点进入场景树时触发（递归含孙节点）。 | 容器需要感知后代加入时连接，配合 child_exiting_tree。 [类页锚点](../builtin/class_node.md#child_entered_tree) |
| `signal child_exiting_tree(node: Node)` | 某个子节点即将退出场景树时触发。 | 子节点离场前做收尾。 [类页锚点](../builtin/class_node.md#child_exiting_tree) |
| `signal child_order_changed()` | 兄弟节点排列顺序改变（reorder）时触发。 | 自定义排序容器重排后刷新。 [类页锚点](../builtin/class_node.md#child_order_changed) |
| `signal editor_description_changed(node: Node)` | 编辑器内节点描述字段改变时触发（仅编辑器）。 | 编辑器插件用，运行期不触发。 [类页锚点](../builtin/class_node.md#editor_description_changed) |
| `signal editor_state_changed()` | 与编辑器相关的节点属性改变时触发（仅编辑器）。 | 编辑器插件用。 [类页锚点](../builtin/class_node.md#editor_state_changed) |
| `signal ready()` | 节点及其全部子节点进入场景树且初始化完成时触发。 | 与覆写 `_ready()` 二选一，不要同时用：`node.ready.connect(_on_ready)`。 [类页锚点](../builtin/class_node.md#ready) |
| `signal renamed()` | 节点被重命名时触发。 | 靠名字维护映射的系统在改名后更新表。 [类页锚点](../builtin/class_node.md#renamed) |
| `signal replacing_by(node: Node)` | 节点正被另一节点替换时触发，见 replace_by()。 | 替换流程钩子。 [类页锚点](../builtin/class_node.md#replacing_by) |
| `signal tree_entered()` | 节点进入场景树时触发（早于 ready，子节点未必就绪）。 | 需要立即知道挂树、但不依赖子节点时连接。 [类页锚点](../builtin/class_node.md#tree_entered) |
| `signal tree_exited()` | 节点已退出场景树后触发。 | 兜底清理；主动 disconnect 外部信号放这里。 [类页锚点](../builtin/class_node.md#tree_exited) |
| `signal tree_exiting()` | 节点即将退出场景树时触发（销毁流程开始）。 | 在此清理外部连接、释放跨树引用。 [类页锚点](../builtin/class_node.md#tree_exiting) |

## Node3D（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal visibility_changed()` | 3D 节点可见性改变时触发。 | 显隐钩子。 [类页锚点](../builtin/class_node3d.md#visibility_changed) |

## Object

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal property_list_changed()` | 对象的属性列表改变（新增/移除导出属性）时触发。 | 自定义类动态增删属性后刷新 inspector 时用。 [类页锚点](../builtin/class_object.md#property_list_changed) |
| `signal script_changed()` | 脚本资源被替换时触发。 | 替换脚本后重新初始化成员时连接：`obj.script_changed.connect(reinit)`。 [类页锚点](../builtin/class_object.md#script_changed) |

## Resource（继承 RefCounted）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal changed()` | 资源内容被修改、需要刷新引用方时触发。 | `emit_changed()` 的对应信号，材质/曲线编辑后通知使用方。 [类页锚点](../builtin/class_resource.md#changed) |
| `signal setup_local_to_scene_requested()` | 请求把资源配置应用到本地场景实例时触发。 | 自定义资源本地化流程钩子。 [类页锚点](../builtin/class_resource.md#setup_local_to_scene_requested) |

## ThemeDB（继承 Object）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal fallback_changed()` | 某个回退主题值改变时触发。 | 全局主题回退刷新。 [类页锚点](../builtin/class_themedb.md#fallback_changed) |

## Viewport（继承 Node）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal gui_focus_changed(node: Control)` | 控件抢到键盘焦点时触发。 | 全局追踪焦点切换。 [类页锚点](../builtin/class_viewport.md#gui_focus_changed) |
| `signal size_changed()` | 视口尺寸改变（窗口缩放等）时触发。 | 视口自适应布局。 [类页锚点](../builtin/class_viewport.md#size_changed) |

## Window（继承 Viewport）

| 原型 | 含义 | 使用方式 |
|---|---|---|
| `signal about_to_popup()` | 弹窗即将弹出前触发（PopupMenu/MenuButton 同源）。 | 弹出前刷新菜单条目。 [类页锚点](../builtin/class_window.md#about_to_popup) |
| `signal close_requested()` | 用户请求关闭窗口（点关闭按钮/Alt+F4）时触发。 | 拦截关闭做确认弹窗：连接后可 hide() 阻止真正关闭。 [类页锚点](../builtin/class_window.md#close_requested) |
| `signal dpi_changed()` | 窗口 DPI/缩放比例改变时触发。 | 高分屏适配重排。 [类页锚点](../builtin/class_window.md#dpi_changed) |
| `signal files_dropped(files: PackedStringArray)` | 外部文件拖入窗口时触发，给出路径数组。 | 拖放导入：`win.files_dropped.connect(_on_drop)`。 [类页锚点](../builtin/class_window.md#files_dropped) |
| `signal focus_entered()` | 窗口获得焦点时触发。 | 窗口激活钩子。 [类页锚点](../builtin/class_window.md#focus_entered) |
| `signal focus_exited()` | 窗口失去焦点时触发。 | 失焦暂停背景动画。 [类页锚点](../builtin/class_window.md#focus_exited) |
| `signal go_back_requested()` | 收到返回请求（如安卓返回键）时触发。 | 移动端返回导航。 [类页锚点](../builtin/class_window.md#go_back_requested) |
| `signal mouse_entered()` | 鼠标进入窗口未被遮挡的可见区域时触发。 | 窗口悬停。 [类页锚点](../builtin/class_window.md#mouse_entered) |
| `signal mouse_exited()` | 鼠标离开窗口可见区域时触发。 | 配合上者。 [类页锚点](../builtin/class_window.md#mouse_exited) |
| `signal nonclient_window_input(event: InputEvent)` | 自定义标题栏区域收到鼠标事件且普通输入被阻断时触发。 | 自制窗口边框交互。 [类页锚点](../builtin/class_window.md#nonclient_window_input) |
| `signal output_max_linear_value_changed(output_max_linear_value: float)` | 输出最大线性值改变时触发。 | HDR 输出范围同步。 [类页锚点](../builtin/class_window.md#output_max_linear_value_changed) |
| `signal theme_changed()` | 主题变更通知发送到窗口时触发。 | 窗口皮肤刷新。 [类页锚点](../builtin/class_window.md#theme_changed) |
| `signal title_changed()` | 窗口标题改变时触发。 | 同步外部标题栏显示。 [类页锚点](../builtin/class_window.md#title_changed) |
| `signal titlebar_changed()` | 标题栏装饰改变时触发。 | 标题栏重排。 [类页锚点](../builtin/class_window.md#titlebar_changed) |
| `signal visibility_changed()` | 窗口可见性改变时触发。 | 弹窗开关后清理临时状态。 [类页锚点](../builtin/class_window.md#visibility_changed) |
| `signal window_input(event: InputEvent)` | 窗口聚焦时收到任意输入时触发，携带事件。 | 窗口级输入拦截。 [类页锚点](../builtin/class_window.md#window_input) |

<!-- SIGNALS:END -->
