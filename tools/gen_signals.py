#!/usr/bin/env python3
"""信号分类索引生成器（发现层，非第二套类参考）。

数据源：
  - crates/gdscript-engine/testdata/extension_api.json   信号原型（签名/参数/继承），唯一事实源
  - .refs/godot/doc/classes/<class>.xml                  官方英文描述（4.7.0），清洗为首句
  - 本文件 CHINESE 覆盖表：高频信号中文「含义 + 使用方式」

产出（website/src/signals/）：
  - index.md                 分层目录（生成，勿手改）
  - <bucket>.md              每域一页，页内平铺；每条 = 原型 + 含义 + 使用方式

约束：
  - 全文签名与长描述仍在 builtin 类页，本索引只做发现，不复制长文。
  - 链接指向声明类类页锚点（继承信号的声明在基类，如 pressed 在 BaseButton）。
  - 未列入 BUCKETS 的声明类落入 99-unclassified 并打印告警，保证「全部收录」不丢信号。

用法：python tools/gen_signals.py [--check]
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
API_JSON = ROOT / "crates/gdscript-engine/testdata/extension_api.json"
XML_DIR = ROOT / ".refs/godot/doc/classes"
OUT_DIR = ROOT / "website/src/signals"

BEGIN = "<!-- SIGNALS:BEGIN（生成器区间，勿手改） -->"
END = "<!-- SIGNALS:END -->"


# ───────────────────────── 分类（声明类名 → 域） ─────────────────────────
# 未列出的声明类自动进 99-unclassified，运行后按告警补入。
BUCKETS: list[tuple[str, str, str, set[str]]] = [
    (
        "01-core", "基类与生命周期",
        "Object/Node/CanvasItem/Control/Viewport/Window/Resource/MainLoop 提供的通用信号，几乎所有节点都能收到。",
        {
            "Object", "Node", "CanvasItem", "Control", "Viewport", "Window",
            "Resource", "MainLoop", "CanvasLayer", "Node3D", "ThemeDB",
        },
    ),
    (
        "02-ui", "输入与控件",
        "按钮、文本、列表、树、分页、滑块、对话框、选色、富文本、节点图控件等用户界面信号。",
        {
            "BaseButton", "ButtonGroup", "LineEdit", "TextEdit", "CodeEdit",
            "ItemList", "Tree", "TabBar", "TabContainer", "Range", "Slider",
            "ScrollBar", "FileDialog", "Popup", "PopupMenu", "AcceptDialog",
            "ColorPicker", "ColorPickerButton", "RichTextLabel", "SplitContainer",
            "Container", "ScrollContainer", "FoldableContainer", "OptionButton",
            "MenuButton", "TouchScreenButton", "StatusIndicator", "VirtualJoystick",
            "NinePatchRect", "GraphEdit", "GraphElement", "GraphNode", "GraphFrame",
            "Input", "InputMap",
        },
    ),
    (
        "03-physics", "物理与区域检测",
        "Area 触发器、刚体进出、碰撞对象输入、屏幕可见通知。",
        {
            "Area2D", "Area3D", "RigidBody2D", "RigidBody3D",
            "CollisionObject2D", "CollisionObject3D",
            "VisibleOnScreenNotifier2D", "VisibleOnScreenNotifier3D",
        },
    ),
    (
        "04-animation", "动画与补间",
        "AnimationMixer/Player/Library、帧动画精灵、Tween/Tweener、骨骼相关。",
        {
            "AnimationMixer", "AnimationPlayer", "AnimationLibrary",
            "AnimationNode", "AnimationNodeBlendTree", "AnimationNodeBlendSpace2D",
            "AnimationNodeStateMachinePlayback", "AnimationNodeStateMachineTransition",
            "AnimatedSprite2D", "AnimatedSprite3D", "Tween", "Tweener",
            "Skeleton3D", "SkeletonModifier3D", "Skeleton2D", "BoneMap",
            "AnimationTree",
        },
    ),
    (
        "05-media", "音频 / 视频 / 粒子 / 计时",
        "播放器完成、音频总线、视频结束、粒子发射完毕、定时触发、喂给相机帧。",
        {
            "AudioStreamPlayer", "AudioStreamPlayer2D", "AudioStreamPlayer3D",
            "AudioServer", "AudioStream", "VideoStreamPlayer",
            "CPUParticles2D", "CPUParticles3D", "GPUParticles2D", "GPUParticles3D",
            "Timer", "SceneTreeTimer", "ParticleProcessMaterial", "CameraFeed",
        },
    ),
    (
        "06-scene-net", "场景树 / 导航 / 多人 / 网络",
        "SceneTree 帧与节点变动、寻路代理与区域、多人同步与连接、HTTP/WebRTC、系统与设置。",
        {
            "SceneTree", "NavigationAgent2D", "NavigationAgent3D",
            "NavigationRegion2D", "NavigationRegion3D",
            "NavigationServer2D", "NavigationServer3D",
            "MultiplayerAPI", "SceneMultiplayer", "MultiplayerPeer",
            "MultiplayerSpawner", "MultiplayerSynchronizer",
            "HTTPRequest", "WebRTCPeerConnection", "DisplayServer",
            "ProjectSettings", "UndoRedo", "GDExtensionManager",
            "JavaScriptBridge",
        },
    ),
    (
        "07-resource-tile", "资源 / 材质 / 瓦片 / 贴图",
        "曲线、路径、TileMap 系列变更、精灵与实例贴图替换、配置资源更新。",
        {
            "Curve", "Path3D", "TileMap", "TileMapLayer", "TileData", "GridMap",
            "Sprite2D", "Sprite3D", "MeshInstance2D", "MultiMeshInstance2D",
            "SkeletonProfile", "FoldableGroup",
        },
    ),
    (
        "08-editor", "编辑器扩展",
        "仅在编辑器内运行的 Editor* 面板、脚本编辑器、文件系统、调试会话信号，游戏运行期不触发。",
        {
            "EditorProperty", "EditorInspector", "EditorFileSystem", "EditorPlugin",
            "EditorDebuggerSession", "EditorResourcePicker", "EditorDock",
            "EditorUndoRedoManager", "EditorResourcePreview", "EditorSelection",
            "EditorSettings", "ScriptCreateDialog", "FileSystemDock",
            "ScriptEditor", "ScriptEditorBase", "EditorSpinSlider",
            "OpenXRBindingModifierEditor", "VisualShaderNodeInput",
        },
    ),
    (
        "09-xr", "XR / OpenXR",
        "XR 会话、姿态追踪、控制器输入、渲染模型与空间实体发现。",
        {
            "XRServer", "XRInterface", "XRNode3D", "XRController3D",
            "XRPositionalTracker", "OpenXRInterface", "WebXRInterface",
            "OpenXRRenderModelExtension", "OpenXRRenderModelManager",
            "OpenXRSpatialEntityTracker", "OpenXRAnchorTracker",
            "OpenXRPlaneTracker", "OpenXRFutureResult",
            "OpenXRSpatialEntityExtension", "OpenXRRenderModel",
        },
    ),
    (
        "10-servers", "Server 低层",
        "渲染/文本/相机服务器的低层回调，普通游戏逻辑一般不直接连接。",
        {"RenderingServer", "TextServerManager", "CameraServer"},
    ),
]


# ───────────────────────── 中文覆盖表："Class.signal" → (含义, 使用方式) ─────────────────────────
CHINESE: dict[str, tuple[str, str]] = {
    # Object
    "Object.script_changed": ("脚本资源被替换时触发。", "替换脚本后重新初始化成员时连接：`obj.script_changed.connect(reinit)`。"),
    "Object.property_list_changed": ("对象的属性列表改变（新增/移除导出属性）时触发。", "自定义类动态增删属性后刷新 inspector 时用。"),
    # Node 生命周期
    "Node.ready": ("节点及其全部子节点进入场景树且初始化完成时触发。", "与覆写 `_ready()` 二选一，不要同时用：`node.ready.connect(_on_ready)`。"),
    "Node.tree_entered": ("节点进入场景树时触发（早于 ready，子节点未必就绪）。", "需要立即知道挂树、但不依赖子节点时连接。"),
    "Node.tree_exiting": ("节点即将退出场景树时触发（销毁流程开始）。", "在此清理外部连接、释放跨树引用。"),
    "Node.tree_exited": ("节点已退出场景树后触发。", "兜底清理；主动 disconnect 外部信号放这里。"),
    "Node.child_entered_tree": ("某个子节点进入场景树时触发（递归含孙节点）。", "容器需要感知后代加入时连接，配合 child_exiting_tree。"),
    "Node.child_exiting_tree": ("某个子节点即将退出场景树时触发。", "子节点离场前做收尾。"),
    "Node.renamed": ("节点被重命名时触发。", "靠名字维护映射的系统在改名后更新表。"),
    "Node.child_order_changed": ("兄弟节点排列顺序改变（reorder）时触发。", "自定义排序容器重排后刷新。"),
    # CanvasItem
    "CanvasItem.draw": ("请求重绘 _draw() 内容时触发。", "覆写 `_draw()` 的自定义绘制节点可用它做额外钩子。"),
    "CanvasItem.visibility_changed": ("可见性（visible）改变时触发。", "节点隐藏/显示后同步附加状态。"),
    "CanvasItem.hidden": ("节点变为隐藏时触发。", "仅隐藏事件，对应 visibility_changed 的隐藏分支。"),
    "CanvasItem.item_rect_changed": ("本项的包围矩形改变时触发。", "依赖尺寸布局的自定义绘制用。"),
    # Control
    "Control.mouse_entered": ("鼠标指针进入控件区域时触发。", "做悬停高亮：`$Btn.mouse_entered.connect(hover_on)`。"),
    "Control.mouse_exited": ("鼠标指针离开控件区域时触发。", "配合 mouse_entered 关闭悬停态。"),
    "Control.focus_entered": ("控件获得键盘焦点时触发。", "输入框聚焦高亮外框。"),
    "Control.focus_exited": ("控件失去键盘焦点时触发。", "失焦校验输入内容。"),
    "Control.gui_input": ("收到一条路由到本控件的输入事件时触发。", "自定义控件拦截鼠标/触摸事件，事件类型判分支。"),
    "Control.resized": ("控件尺寸改变时触发。", "尺寸自适应布局后重算内部元素。"),
    "Control.minimum_size_changed": ("最小尺寸建议改变时触发。", "容器重新排版。"),
    "Control.maximum_size_changed": ("最大尺寸建议改变时触发。", "容器重新排版。"),
    "Control.size_flags_changed": ("伸缩标志改变时触发。", "容器刷新。"),
    "Control.theme_changed": ("主题或主题属性改变时触发。", "自定义皮肤后刷新派生样式。"),
    # Window
    "Window.close_requested": ("用户请求关闭窗口（点关闭按钮/Alt+F4）时触发。", "拦截关闭做确认弹窗：连接后可 hide() 阻止真正关闭。"),
    "Window.files_dropped": ("外部文件拖入窗口时触发，给出路径数组。", "拖放导入：`win.files_dropped.connect(_on_drop)`。"),
    "Window.about_to_popup": ("弹窗即将弹出前触发（PopupMenu/MenuButton 同源）。", "弹出前刷新菜单条目。"),
    "Window.dpi_changed": ("窗口 DPI/缩放比例改变时触发。", "高分屏适配重排。"),
    "Window.title_changed": ("窗口标题改变时触发。", "同步外部标题栏显示。"),
    "Window.visibility_changed": ("窗口可见性改变时触发。", "弹窗开关后清理临时状态。"),
    # Resource
    "Resource.changed": ("资源内容被修改、需要刷新引用方时触发。", "`emit_changed()` 的对应信号，材质/曲线编辑后通知使用方。"),
    "Resource.setup_local_to_scene_requested": ("请求把资源配置应用到本地场景实例时触发。", "自定义资源本地化流程钩子。"),
    # BaseButton
    "BaseButton.pressed": ("按钮被按下并确认点击时触发（按压或释放取决于 action_mode）。", "最常用的点击：`$Btn.pressed.connect(_on_click)`。"),
    "BaseButton.button_down": ("按键开始按下（按住）的瞬间触发。", "需要区分按下/抬起时用，常配合 button_up。"),
    "BaseButton.button_up": ("按键松开的瞬间触发。", "长按逻辑、松开结算。"),
    "BaseButton.toggled": ("toggle_mode 开启时，在按下/弹起状态间切换后触发，携带新状态。", "复选/单选：`chb.toggled.connect(_on_toggle)`，回调收 bool。"),
    # LineEdit
    "LineEdit.text_changed": ("文本内容每次改变时触发。", "实时搜索/校验：`$Line.text_changed.connect(_on_edit)`。"),
    "LineEdit.text_submitted": ("用户按回车提交时触发。", "登录框回车提交。"),
    "LineEdit.text_change_rejected": ("输入被 reject 回调拒绝时触发。", "非法字符输入反馈。"),
    "LineEdit.editing_toggled": ("进入/退出编辑状态（获焦/失焦）时触发。", "行编辑焦点切换钩子。"),
    # Range / Slider
    "Range.value_changed": ("数值（value）改变时触发，携带新值。", "音量条/进度：`$Slider.value_changed.connect(_on_value)`。"),
    "Range.changed": ("范围或步长等参数改变时触发（非当前值）。", "重新标定滑块。"),
    "Slider.drag_started": ("开始拖动滑块时触发。", "拖动中暂停实时同步。"),
    "Slider.drag_ended": ("结束拖动滑块时触发。", "拖动结束后统一提交。"),
    # List / Tree
    "ItemList.item_selected": ("选中列表项时触发，携带索引。", "选中查看详情：`$List.item_selected.connect(_show)`。"),
    "ItemList.item_activated": ("双击或回车激活列表项时触发，携带索引。", "确认打开该项。"),
    "ItemList.multi_selected": ("多选集合变化时触发。", "多选批量操作刷新。"),
    "ItemList.empty_clicked": ("点击列表空白处时触发。", "取消选中。"),
    "Tree.item_selected": ("选中树节点时触发。", "文件树选中展开详情。"),
    "Tree.item_activated": ("双击/回车激活树节点时触发。", "打开节点。"),
    "Tree.item_collapsed": ("折叠树节点时触发。", "折叠记忆。"),
    "Tree.button_clicked": ("节点上的按钮列被点击时触发。", "行内按钮交互。"),
    "TabBar.tab_selected": ("选中页签时触发，携带索引。", "切换页面。"),
    "TabBar.tab_changed": ("当前页签改变时触发。", "页签切换通用钩子。"),
    "TabBar.tab_close_pressed": ("点页签关闭按钮时触发，携带索引。", "关闭标签页。"),
    "TabContainer.tab_changed": ("容器当前页改变时触发，携带索引。", "多页卡片切换。"),
    # Dialogs / popup
    "AcceptDialog.confirmed": ("用户点确认按钮时触发。", "确认框主操作。"),
    "AcceptDialog.canceled": ("用户取消/关闭时触发。", "取消分支。"),
    "AcceptDialog.custom_action": ("点自定义按钮时触发，携带动作名。", "多按钮对话框。"),
    "PopupMenu.id_pressed": ("点菜单项时触发，携带 item id。", "菜单分发动作：`$Menu.id_pressed.connect(_dispatch)`。"),
    "FileDialog.file_selected": ("选定单个文件时触发，携带路径。", "打开文件。"),
    "FileDialog.files_selected": ("多选文件时触发，携带路径数组。", "批量导入。"),
    "FileDialog.dir_selected": ("选定目录时触发。", "选文件夹。"),
    "ColorPicker.color_changed": ("选色值改变时触发。", "实时改色。"),
    "ColorPickerButton.color_changed": ("按钮内嵌选色改色时触发。", "取色器按钮对外通知。"),
    "RichTextLabel.meta_clicked": ("点击链接元数据时触发，携带节点名。", "正文内链接跳转。"),
    "RichTextLabel.finished": ("富文本播放（打字机/动画）结束时触发。", "对白打完再推进剧情。"),
    # Physics Area2D/3D
    "Area2D.body_entered": ("物理体进入本区域时触发，携带对方节点。", "触发器：`$Zone.body_entered.connect(_on_enter)`，需 monitoring=true。"),
    "Area2D.body_exited": ("物理体离开本区域时触发。", "离开结算。"),
    "Area2D.area_entered": ("另一 Area 进入本区域时触发。", "区域重叠检测。"),
    "Area2D.area_exited": ("另一 Area 离开时触发。", "重叠解除。"),
    "Area2D.body_shape_entered": ("对方某个形状进入本区域某形状时触发，带 RID 与形状索引。", "需要定位具体碰撞形状时用。"),
    "Area2D.body_shape_exited": ("对方形状离开时触发。", "配合上者。"),
    "Area2D.area_shape_entered": ("对方 Area 的形状进入时触发。", "形状级区域检测。"),
    "Area2D.area_shape_exited": ("对方 Area 形状离开时触发。", "配合上者。"),
    "Area3D.body_entered": ("3D 物理体进入本区域时触发，携带 Node3D。", "3D 触发器，需 monitoring=true。"),
    "Area3D.body_exited": ("3D 物理体离开时触发。", "离开结算。"),
    "Area3D.area_entered": ("另一 3D Area 进入时触发。", "3D 区域重叠。"),
    "Area3D.area_exited": ("另一 3D Area 离开时触发。", "重叠解除。"),
    "Area3D.body_shape_entered": ("3D 对方形状进入时触发，带 RID 与形状索引。", "形状级 3D 检测。"),
    "Area3D.body_shape_exited": ("3D 对方形状离开时触发。", "配合上者。"),
    "Area3D.area_shape_entered": ("对方 3D Area 形状进入时触发。", "形状级区域检测。"),
    "Area3D.area_shape_exited": ("对方 3D Area 形状离开时触发。", "配合上者。"),
    "RigidBody2D.body_entered": ("另一个刚体进入本刚体的区域通知范围时触发。", "刚体间碰撞通知（非 Area）。"),
    "RigidBody2D.body_exited": ("另一刚体离开时触发。", "配合上者。"),
    "RigidBody2D.sleeping_state_changed": ("刚体进入/退出休眠时触发。", "休眠唤醒钩子。"),
    "RigidBody3D.body_entered": ("另一 3D 刚体进入通知范围时触发。", "3D 刚体通知。"),
    "RigidBody3D.body_exited": ("另一 3D 刚体离开时触发。", "配合上者。"),
    "RigidBody3D.sleeping_state_changed": ("3D 刚体休眠状态改变时触发。", "休眠唤醒钩子。"),
    "CollisionObject2D.input_event": ("指针/输入在本碰撞体上按下时触发，携带事件与形状索引。", "2D 可点物体拾取。"),
    "CollisionObject2D.mouse_entered": ("鼠标悬停到碰撞体时触发。", "2D hover。"),
    "CollisionObject2D.mouse_exited": ("鼠标离开碰撞体时触发。", "配合上者。"),
    "CollisionObject2D.mouse_shape_entered": ("鼠标进入某形状时触发，带形状索引。", "分形状 hover。"),
    "CollisionObject2D.mouse_shape_exited": ("鼠标离开某形状时触发。", "配合上者。"),
    "CollisionObject3D.input_event": ("指针在 3D 碰撞体上按下时触发。", "3D 点击拾取。"),
    "CollisionObject3D.mouse_entered": ("鼠标悬停 3D 碰撞体时触发。", "3D hover。"),
    "CollisionObject3D.mouse_exited": ("鼠标离开 3D 碰撞体时触发。", "配合上者。"),
    "VisibleOnScreenNotifier2D.screen_entered": ("节点进入可视屏幕时触发。", "离屏裁剪、只渲染可见物。"),
    "VisibleOnScreenNotifier2D.screen_exited": ("节点离开可视屏幕时触发。", "回收离屏对象。"),
    "VisibleOnScreenNotifier3D.screen_entered": ("3D 节点进入相机视锥时触发。", "3D 可见剔除。"),
    "VisibleOnScreenNotifier3D.screen_exited": ("3D 节点离开视锥时触发。", "配合上者。"),
    # Animation
    "AnimationMixer.animation_finished": ("当前播放动画播放完毕时触发。", "播完切状态：`$Mixer.animation_finished.connect(_done)`。"),
    "AnimationMixer.animation_started": ("开始播放某动画时触发，携带动画名。", "动画切换钩子。"),
    "AnimationMixer.animation_list_changed": ("动画列表增删时触发。", "运行时动态加动画后刷新。"),
    "AnimationPlayer.current_animation_changed": ("当前动画名改变时触发。", "追踪 AnimationPlayer 当前片段。"),
    "AnimationPlayer.animation_changed": ("动画库内容改变时触发。", "编辑期/运行时库更新。"),
    "AnimationLibrary.animation_added": ("新增动画片段时触发，携带名。", "动态建库刷新。"),
    "AnimationLibrary.animation_removed": ("移除动画片段时触发。", "配合上者。"),
    "AnimationLibrary.animation_renamed": ("动画改名时触发。", "更新引用。"),
    "AnimationLibrary.animation_changed": ("库内容整体改变时触发。", "刷新。"),
    "AnimatedSprite2D.animation_finished": ("当前帧动画播放完一轮时触发。", "动作结束切待机。"),
    "AnimatedSprite2D.animation_changed": ("切换到另一个动画时触发。", "状态切换钩子。"),
    "AnimatedSprite2D.animation_looped": ("循环动画播完一轮时触发。", "循环计数。"),
    "AnimatedSprite2D.frame_changed": ("当前帧号改变时触发。", "逐帧触发事件（脚步声）。"),
    "AnimatedSprite3D.animation_finished": ("3D 精灵帧动画播完时触发。", "同 2D。"),
    "AnimatedSprite3D.animation_changed": ("3D 切换动画时触发。", "状态钩子。"),
    "AnimatedSprite3D.animation_looped": ("3D 循环一轮时触发。", "循环计数。"),
    "AnimatedSprite3D.frame_changed": ("3D 帧号改变时触发。", "逐帧事件。"),
    "Tween.finished": ("补间全部完成（含循环结束）时触发。", "序列播完执行下一步。"),
    "Tween.step_finished": ("单个 step 补间完成时触发。", "分步编排。"),
    "Tween.loop_finished": ("一轮循环结束时触发。", "循环计数。"),
    "Tweener.finished": ("单个补间项完成时触发。", "项级完成钩子。"),
    # Media / timer
    "AudioStreamPlayer.finished": ("音频完整播放完毕（未被打断）时触发。", "播完切下一首。"),
    "AudioStreamPlayer2D.finished": ("2D 位置音频播完时触发。", "音效结束回收。"),
    "AudioStreamPlayer3D.finished": ("3D 位置音频播完时触发。", "音效结束回收。"),
    "VideoStreamPlayer.finished": ("视频播放结束时触发。", "过场视频完毕进游戏。"),
    "CPUParticles2D.finished": ("CPU 2D 粒子发射周期结束时触发。", "一次性特效完成销毁。"),
    "CPUParticles3D.finished": ("CPU 3D 粒子发射结束时触发。", "一次性特效完成。"),
    "GPUParticles2D.finished": ("GPU 2D 粒子发射结束时触发。", "一次性特效完成。"),
    "GPUParticles3D.finished": ("GPU 3D 粒子发射结束时触发。", "一次性特效完成。"),
    "Timer.timeout": ("计时到达设定时间时触发。", "延时/循环：`$Timer.timeout.connect(_alarm)`。"),
    "SceneTreeTimer.timeout": ("SceneTree.create_timer 返回的一次性计时器到时触发。", "await 之外的一次性延时回调。"),
    # Scene tree / nav / net
    "SceneTree.process_frame": ("每渲染帧处理前触发。", "需要每帧钩子但不想写 _process 时用。"),
    "SceneTree.physics_frame": ("每物理步处理前触发。", "全局物理帧钩子。"),
    "SceneTree.node_added": ("节点加入场景树时触发。", "全局登记新节点。"),
    "SceneTree.node_removed": ("节点离开场景树时触发。", "全局清理。"),
    "SceneTree.scene_changed": ("当前主场景切换时触发。", "切场景后初始化。"),
    "NavigationAgent2D.target_reached": ("2D 代理到达目标点时触发。", "到达后停止/换目标。"),
    "NavigationAgent2D.path_changed": ("2D 路径重算完成时触发。", "拿到新路径。"),
    "NavigationAgent2D.navigation_finished": ("2D 导航结束时触发。", "导航生命周期收尾。"),
    "NavigationAgent3D.target_reached": ("3D 代理到达目标时触发。", "到达停止。"),
    "NavigationAgent3D.path_changed": ("3D 路径重算完成时触发。", "新路径。"),
    "NavigationAgent3D.navigation_finished": ("3D 导航结束时触发。", "收尾。"),
    "NavigationRegion2D.bake_finished": ("2D 导航多边形烘焙完成时触发。", "烘焙后启用寻路。"),
    "NavigationRegion3D.bake_finished": ("3D 导航网格烘焙完成时触发。", "烘焙后启用寻路。"),
    "MultiplayerAPI.peer_connected": ("有新对端连接时触发，携带远端 id。", "加入广播。"),
    "MultiplayerAPI.peer_disconnected": ("对端断开时触发，携带远端 id。", "离场清理。"),
    "MultiplayerAPI.connected_to_server": ("作为客户端连上服务器时触发。", "连接成功进游戏。"),
    "MultiplayerAPI.connection_failed": ("连接失败时触发。", "失败提示重试。"),
    "MultiplayerAPI.server_disconnected": ("与服务器断开时触发。", "断线回主菜单。"),
    "MultiplayerSpawner.spawned": ("自动生成一个网络实体时触发，携带节点。", "生成后初始化。"),
    "MultiplayerSpawner.despawned": ("自动回收网络实体时触发。", "离场清理。"),
    "MultiplayerSynchronizer.synchronized": ("一次状态同步完成时触发。", "同步后刷新本地表现。"),
    "HTTPRequest.request_completed": ("HTTP 请求完成（成功或失败）时触发，携带结果码与数据。", "发请求后在此收响应。"),
    # 01 补
    "Node.editor_description_changed": ("编辑器内节点描述字段改变时触发（仅编辑器）。", "编辑器插件用，运行期不触发。"),
    "Node.editor_state_changed": ("与编辑器相关的节点属性改变时触发（仅编辑器）。", "编辑器插件用。"),
    "Node.replacing_by": ("节点正被另一节点替换时触发，见 replace_by()。", "替换流程钩子。"),
    "Viewport.gui_focus_changed": ("控件抢到键盘焦点时触发。", "全局追踪焦点切换。"),
    "Viewport.size_changed": ("视口尺寸改变（窗口缩放等）时触发。", "视口自适应布局。"),
    "Window.focus_entered": ("窗口获得焦点时触发。", "窗口激活钩子。"),
    "Window.focus_exited": ("窗口失去焦点时触发。", "失焦暂停背景动画。"),
    "Window.go_back_requested": ("收到返回请求（如安卓返回键）时触发。", "移动端返回导航。"),
    "Window.mouse_entered": ("鼠标进入窗口未被遮挡的可见区域时触发。", "窗口悬停。"),
    "Window.mouse_exited": ("鼠标离开窗口可见区域时触发。", "配合上者。"),
    "Window.nonclient_window_input": ("自定义标题栏区域收到鼠标事件且普通输入被阻断时触发。", "自制窗口边框交互。"),
    "Window.output_max_linear_value_changed": ("输出最大线性值改变时触发。", "HDR 输出范围同步。"),
    "Window.theme_changed": ("主题变更通知发送到窗口时触发。", "窗口皮肤刷新。"),
    "Window.titlebar_changed": ("标题栏装饰改变时触发。", "标题栏重排。"),
    "Window.window_input": ("窗口聚焦时收到任意输入时触发，携带事件。", "窗口级输入拦截。"),
    "MainLoop.on_request_permissions_result": ("用户响应权限请求后触发，携带权限名与是否授予。", "Web/移动权限回调。"),
    "CanvasLayer.visibility_changed": ("图层可见性改变时触发。", "图层显隐同步。"),
    "Node3D.visibility_changed": ("3D 节点可见性改变时触发。", "显隐钩子。"),
    "ThemeDB.fallback_changed": ("某个回退主题值改变时触发。", "全局主题回退刷新。"),
    # 02 补
    "ButtonGroup.pressed": ("组内某个按钮被按下时触发，携带该按钮。", "单选组统一切换处理。"),
    "TextEdit.caret_changed": ("光标位置改变时触发。", "光标位置追踪。"),
    "TextEdit.gutter_added": ("新增一个槽位（行号/断点栏）时触发。", "编辑器 UI 刷新。"),
    "TextEdit.gutter_clicked": ("点击槽位时触发，携带行号与槽位号。", "点槽位加断点。"),
    "TextEdit.gutter_removed": ("移除一个槽位时触发。", "配合上者。"),
    "TextEdit.lines_edited_from": ("文本改变瞬间触发，携带起止行号。", "大范围改动范围。"),
    "TextEdit.text_changed": ("文本内容改变时触发。", "实时编辑回调。"),
    "TextEdit.text_set": ("调用 clear() 或整体设置文本时触发。", "重置后刷新。"),
    "CodeEdit.breakpoint_toggled": ("某行加/删断点时触发，携带行号。", "调试断点栏刷新。"),
    "CodeEdit.code_completion_requested": ("用户请求代码补全时触发。", "自定义补全钩子。"),
    "CodeEdit.symbol_hovered": ("悬停在符号上时触发，携带符号与坐标。", "悬停提示。"),
    "CodeEdit.symbol_lookup": ("点击有效符号时触发。", "跳转定义。"),
    "CodeEdit.symbol_validate": ("悬停校验符号时触发。", "符号合法性检查。"),
    "ItemList.item_clicked": ("任意鼠标键点击列表项时触发，携带索引/位置/键。", "点击项操作。"),
    "Tree.cell_selected": ("选中单元格时触发。", "单元格选择。"),
    "Tree.check_propagated_to_item": ("调用 propagate_check() 传播勾选状态时触发。", "父子勾选联动。"),
    "Tree.column_title_clicked": ("点击列标题（左/右键）时触发，携带列号。", "列排序。"),
    "Tree.custom_item_clicked": ("自定义模式单元格被点击时触发。", "自定义列交互。"),
    "Tree.custom_popup_edited": ("点击自定义单元格进入编辑时触发。", "自定义编辑弹窗。"),
    "Tree.empty_clicked": ("点击树空白处时触发，携带位置与鼠标键。", "取消选中。"),
    "Tree.item_edited": ("项被编辑时触发。", "改名提交。"),
    "Tree.item_icon_double_clicked": ("双击项图标时触发。", "图标双击动作。"),
    "Tree.item_mouse_selected": ("用鼠标选中项时触发，携带位置与键。", "鼠标选择。"),
    "Tree.multi_selected": ("多选模式下勾选/取消项时触发，携带项/列/状态。", "多选集合同步。"),
    "Tree.nothing_selected": ("左键点击未选中任何项时触发。", "清空选择态。"),
    "TabBar.active_tab_rearranged": ("拖拽重排活动页签时触发，携带目标位置。", "保存页签顺序。"),
    "TabBar.tab_button_pressed": ("按下页签按钮图标时触发，携带页签号。", "页签按钮动作。"),
    "TabBar.tab_clicked": ("点击页签（含当前页）时触发，携带页签号。", "点击钩子。"),
    "TabBar.tab_hovered": ("鼠标悬停页签时触发，携带页签号。", "悬停预览。"),
    "TabBar.tab_rmb_clicked": ("右键点击页签时触发，携带页签号。", "页签右键菜单。"),
    "TabContainer.active_tab_rearranged": ("拖拽重排容器活动页时触发。", "保存顺序。"),
    "TabContainer.pre_popup_pressed": ("点容器的溢出弹出按钮时触发。", "溢出页菜单。"),
    "TabContainer.tab_button_pressed": ("点击页签按钮图标时触发。", "按钮动作。"),
    "TabContainer.tab_clicked": ("点击页签（含当前）时触发，携带页签号。", "点击钩子。"),
    "TabContainer.tab_hovered": ("悬停页签时触发。", "悬停预览。"),
    "TabContainer.tab_selected": ("点击/方向键/脚本选中页签时触发，携带页签号。", "切换页面。"),
    "ScrollBar.scrolling": ("滚动条正在被滚动时触发。", "滚动过程同步。"),
    "FileDialog.filename_filter_changed": ("文件名过滤器改变时触发，携带过滤串。", "过滤项切换。"),
    "Popup.popup_hide": ("弹窗隐藏时触发。", "关闭后清理临时态。"),
    "PopupMenu.id_focused": ("用上下方向键导航到某 id 项时触发。", "键盘导航高亮。"),
    "PopupMenu.index_pressed": ("按索引压下菜单项时触发。", "按索引分发。"),
    "PopupMenu.menu_changed": ("增删改菜单项时触发。", "动态菜单刷新。"),
    "ColorPicker.preset_added": ("新增一个预设色时触发，携带颜色。", "预设列表刷新。"),
    "ColorPicker.preset_removed": ("移除预设色时触发，携带颜色。", "配合上者。"),
    "ColorPickerButton.picker_created": ("首次按下按钮创建内嵌选色器时触发。", "初始化取色器。"),
    "ColorPickerButton.popup_closed": ("选色弹窗关闭时触发。", "关闭提交。"),
    "RichTextLabel.meta_hover_ended": ("鼠标离开元数据标签时触发，携带 meta。", "取消链接高亮。"),
    "RichTextLabel.meta_hover_started": ("鼠标进入元数据标签时触发，携带 meta。", "链接悬停高亮。"),
    "SplitContainer.drag_ended": ("结束拖拽分隔条时触发。", "停止拖动提交。"),
    "SplitContainer.drag_started": ("开始拖拽分隔条时触发。", "拖动中暂停布局同步。"),
    "SplitContainer.dragged": ("拖动任意分隔条时触发，携带偏移。", "实时调整比例。"),
    "Container.pre_sort_children": ("即将排序子节点前触发。", "排序前置钩子。"),
    "Container.sort_children": ("需要排序子节点时触发。", "自定义排序。"),
    "ScrollContainer.scroll_ended": ("触摸拖拽滚动停止时触发。", "滚动结束吸附。"),
    "ScrollContainer.scroll_started": ("触摸开始拖拽滚动时触发。", "滚动开始。"),
    "FoldableContainer.folding_changed": ("折叠/展开时触发，携带状态。", "折叠态同步。"),
    "OptionButton.item_focused": ("方向键导航到某项时触发，携带索引。", "键盘高亮。"),
    "OptionButton.item_selected": ("用户改变当前选项时触发，携带索引。", "下拉选择取值。"),
    "MenuButton.about_to_popup": ("按钮的菜单即将弹出时触发。", "弹出前刷新条目。"),
    "TouchScreenButton.pressed": ("触控按钮按下（按下沿）时触发。", "触屏按下。"),
    "TouchScreenButton.released": ("触控按钮松开（抬起沿）时触发。", "触屏抬起。"),
    "StatusIndicator.pressed": ("状态指示器被按下时触发，携带鼠标键与位置。", "状态图标点击。"),
    "VirtualJoystick.flick_canceled": ("摇杆头部离开死区后又回到死区时触发。", "取消甩动。"),
    "VirtualJoystick.flicked": ("头部移出死区后松开摇杆时触发，携带方向向量。", "甩动冲刺。"),
    "VirtualJoystick.pressed": ("按下摇杆时触发。", "开始操控。"),
    "VirtualJoystick.released": ("松开摇杆时触发，携带输入向量。", "停止操控归零。"),
    "VirtualJoystick.tapped": ("未移动头部即松开（点按）时触发。", "点按动作。"),
    "NinePatchRect.texture_changed": ("九宫格纹理改变时触发。", "换肤刷新。"),
    "GraphEdit.begin_node_move": ("开始移动图节点时触发。", "移动前置。"),
    "GraphEdit.connection_drag_ended": ("连线拖拽结束时触发。", "落点校验。"),
    "GraphEdit.connection_drag_started": ("开始从端口拖线时触发，携带源节点/端口/是否输出。", "连线起点。"),
    "GraphEdit.connection_from_empty": ("从输入端口拖线到空白处松开时触发。", "空白处建连接。"),
    "GraphEdit.connection_request": ("尝试在两节点端口间建连时触发，携带两端。", "建连校验/拒绝。"),
    "GraphEdit.connection_to_empty": ("从输出端口拖线到空白处松开时触发。", "空白落点。"),
    "GraphEdit.copy_nodes_request": ("捕获复制动作（默认 Ctrl+C）时触发。", "自定义复制。"),
    "GraphEdit.cut_nodes_request": ("捕获剪切动作（默认 Ctrl+X）时触发。", "自定义剪切。"),
    "GraphEdit.delete_nodes_request": ("捕获删除动作（默认 Delete）时触发，携带节点名数组。", "删除选中。"),
    "GraphEdit.disconnection_request": ("尝试移除两端连接时触发，携带两端。", "断连校验。"),
    "GraphEdit.duplicate_nodes_request": ("捕获复制节点动作（默认 Ctrl+D）时触发。", "重复节点。"),
    "GraphEdit.end_node_move": ("结束移动图节点时触发。", "落位保存。"),
    "GraphEdit.frame_rect_changed": ("图框缩放为新矩形时触发，携带图框与矩形。", "框尺寸同步。"),
    "GraphEdit.graph_elements_linked_to_frame_request": ("把元素拖到此前未归属的图框时触发。", "归入图框。"),
    "GraphEdit.node_deselected": ("取消选中某图节点时触发，携带节点。", "选择态同步。"),
    "GraphEdit.node_selected": ("选中某图节点时触发，携带节点。", "选择高亮。"),
    "GraphEdit.paste_nodes_request": ("捕获粘贴动作（默认 Ctrl+V）时触发。", "自定义粘贴。"),
    "GraphEdit.popup_request": ("请求弹出菜单时触发，携带位置。", "右键菜单定位。"),
    "GraphEdit.scroll_offset_changed": ("用户改变滚动偏移时触发，携带偏移。", "视口同步。"),
    "GraphElement.delete_request": ("请求删除该图元素时触发。", "删除确认。"),
    "GraphElement.dragged": ("拖动图元素时触发，携带起止位置。", "拖动实时位置。"),
    "GraphElement.node_deselected": ("该元素被取消选中时触发。", "选择同步。"),
    "GraphElement.node_selected": ("该元素被选中时触发。", "选择高亮。"),
    "GraphElement.position_offset_changed": ("元素被移动时触发。", "位置保存。"),
    "GraphElement.raise_request": ("请求把元素置顶显示时触发。", "调整层级。"),
    "GraphElement.resize_end": ("拖缩放柄松开时触发，携带新尺寸。", "尺寸提交。"),
    "GraphElement.resize_request": ("请求缩放元素时触发，携带新尺寸。", "缩放中。"),
    "GraphNode.slot_sizes_changed": ("任意槽位尺寸可能改变时触发。", "槽位重排。"),
    "GraphNode.slot_updated": ("某槽位更新时触发，携带槽位号。", "槽位刷新。"),
    "GraphFrame.autoshrink_changed": ("自动收缩开关或边距改变时触发。", "图框自适应。"),
    "Input.joy_connection_changed": ("手柄设备连接/断开时触发，携带设备号与状态。", "手柄热插拔。"),
    "InputMap.project_settings_loaded": ("项目设置的输入映射加载完成时触发。", "映射就绪初始化。"),
    # 03 补
    "RigidBody2D.body_shape_entered": ("本刚体某形状与另一物理体/TileMap 形状碰撞开始时触发，带 RID 与形状索引。", "形状级碰撞开始。"),
    "RigidBody2D.body_shape_exited": ("上述形状碰撞结束时触发。", "形状级碰撞结束。"),
    "RigidBody3D.body_shape_entered": ("本 3D 刚体某形状与另一物理体/GridMap 形状碰撞开始时触发。", "3D 形状级碰撞开始。"),
    "RigidBody3D.body_shape_exited": ("3D 形状碰撞结束时触发。", "配合上者。"),
    # 04 补
    "AnimationMixer.animation_libraries_updated": ("动画库发生变更时通知。", "动态加库后刷新。"),
    "AnimationMixer.caches_cleared": ("缓存被自动或手动 clear_caches() 清空时通知。", "清缓存后重建。"),
    "AnimationMixer.mixer_applied": ("混合结果已应用到目标对象时通知。", "应用后钩子。"),
    "AnimationMixer.mixer_updated": ("属性处理流程更新时通知。", "更新钩子。"),
    "AnimationNode.animation_node_removed": ("内部树中某动画节点被移除时触发，携带对象 id 与节点名。", "节点树变更。"),
    "AnimationNode.animation_node_renamed": ("内部树中节点改名时触发，携带 id/旧名/新名。", "更新引用。"),
    "AnimationNode.node_updated": ("动画资源改变或连线变化时触发，携带对象 id。", "刷新节点。"),
    "AnimationNode.tree_changed": ("内部树任一节点变化时触发。", "树结构刷新。"),
    "AnimationNodeBlendTree.node_changed": ("输入端口信息改变时触发，携带节点名。", "端口重连。"),
    "AnimationNodeBlendSpace2D.triangles_updated": ("混合空间三角形增删或顶点移动时触发。", "重建三角插值。"),
    "AnimationNodeStateMachinePlayback.state_finished": ("某状态播放结束时触发，携带状态名。", "状态完成转移。"),
    "AnimationNodeStateMachinePlayback.state_started": ("某状态开始播放时触发，携带状态名。", "进入状态钩子。"),
    "AnimationNodeStateMachineTransition.advance_condition_changed": ("advance_condition 改变时触发。", "转移条件刷新。"),
    "AnimatedSprite2D.sprite_frames_changed": ("sprite_frames 资源替换时触发。", "换图集后重置。"),
    "AnimatedSprite3D.sprite_frames_changed": ("3D 精灵图集替换时触发。", "换图集重置。"),
    "Skeleton3D.bone_enabled_changed": ("用 set_bone_enabled() 切换某骨骼启用时触发，携带骨号。", "骨骼开关同步。"),
    "Skeleton3D.bone_list_changed": ("骨骼列表变化（增骨/改父/解绑/清空）时触发。", "重建骨骼表。"),
    "Skeleton3D.pose_updated": ("姿态更新时触发。", "姿态后处理。"),
    "Skeleton3D.rest_updated": ("静止姿势更新时触发。", "重置姿态。"),
    "Skeleton3D.show_rest_only_changed": ("show_rest_only 值改变时触发。", "视图切换。"),
    "Skeleton3D.skeleton_updated": ("最终姿态计算完成并将应用到蒙皮前触发。", "蒙皮前钩子。"),
    "SkeletonModifier3D.modification_processed": ("修饰处理完成时通知。", "修饰后钩子。"),
    "Skeleton2D.bone_setup_changed": ("挂接的 Bone2D 配置改变时触发。", "2D 骨骼重建。"),
    "BoneMap.bone_map_updated": ("BoneMap 键值改变时触发。", "骨骼映射刷新。"),
    "BoneMap.profile_updated": ("配置文件值或其引用改变时触发。", "配置刷新。"),
    "AnimationTree.animation_player_changed": ("绑定的动画播放器改变时触发。", "重绑后重置。"),
    # 05 补
    "AudioServer.bus_layout_changed": ("音频总线新增/删除/移动时触发。", "总线布局重建。"),
    "AudioServer.bus_renamed": ("指定索引总线改名时触发，携带索引/旧名/新名。", "总线名同步。"),
    "AudioStream.parameter_list_changed": ("参数列表改变时通知。", "动态参数刷新。"),
    "ParticleProcessMaterial.emission_shape_changed": ("发射形状被改动时触发。", "重建发射范围。"),
    "CameraFeed.format_changed": ("采集格式改变时触发。", "重设帧尺寸。"),
    "CameraFeed.frame_changed": ("新一帧可用时触发。", "取帧回调。"),
    # 06 补
    "SceneTree.node_configuration_warning_changed": ("调用 update_configuration_warnings() 时触发，携带节点。", "警告刷新。"),
    "SceneTree.node_renamed": ("节点改名时触发，携带节点。", "全局名映射更新。"),
    "SceneTree.tree_changed": ("树结构变化（移动/改名等）时触发。", "结构变更刷新。"),
    "SceneTree.tree_process_mode_changed": ("任一节点 process_mode 改变时触发。", "处理模式同步。"),
    "NavigationAgent2D.link_reached": ("代理到达导航链接时触发，携带详情字典。", "过链钩子。"),
    "NavigationAgent2D.velocity_computed": ("避障速度计算完成时触发，携带安全速度。", "取避障速度。"),
    "NavigationAgent2D.waypoint_reached": ("到达路径途经点时触发，携带详情。", "逐点动作。"),
    "NavigationAgent3D.link_reached": ("3D 代理到达导航链接时触发。", "过链钩子。"),
    "NavigationAgent3D.velocity_computed": ("3D 避障速度计算完成时触发，携带安全速度。", "取速度。"),
    "NavigationAgent3D.waypoint_reached": ("3D 到达途经点时触发。", "逐点动作。"),
    "NavigationRegion2D.navigation_polygon_changed": ("导航多边形被替换或内部提交变更时触发。", "重建区域。"),
    "NavigationRegion3D.navigation_mesh_changed": ("导航网格变更时通知。", "重建区域。"),
    "NavigationServer2D.avoidance_debug_changed": ("避障调试设置改变时触发。", "调试刷新。"),
    "NavigationServer2D.map_changed": ("导航图更新（区域移动/修改）时触发，携带 RID。", "图刷新。"),
    "NavigationServer2D.navigation_debug_changed": ("导航调试设置改变时触发。", "调试刷新。"),
    "NavigationServer3D.avoidance_debug_changed": ("3D 避障调试设置改变时触发。", "调试刷新。"),
    "NavigationServer3D.map_changed": ("3D 导航图更新时触发，携带 RID。", "图刷新。"),
    "NavigationServer3D.navigation_debug_changed": ("3D 导航调试设置改变时触发。", "调试刷新。"),
    "MultiplayerPeer.peer_connected": ("远端对端连接时触发，携带 id。", "加入广播。"),
    "MultiplayerPeer.peer_disconnected": ("远端对端断开时触发，携带 id。", "离场清理。"),
    "DisplayServer.orientation_changed": ("设备屏幕方向改变时触发，携带方向值。", "横竖屏重排。"),
    "ProjectSettings.settings_changed": ("任一设置改变时触发（每帧至多一次）。", "设置生效刷新。"),
    "UndoRedo.version_changed": ("调用 undo() 或 redo() 后触发。", "撤销栈版本同步。"),
    "GDExtensionManager.extension_loaded": ("编辑器加载完新扩展后触发，携带扩展。", "加载后初始化。"),
    "GDExtensionManager.extension_unloading": ("编辑器开始卸载扩展前触发，携带扩展。", "卸载前清理。"),
    "GDExtensionManager.extensions_reloaded": ("重载一个或多个扩展完成后触发。", "重载后刷新。"),
    "JavaScriptBridge.pwa_update_available": ("检测到 PWA 新版本待激活时触发。", "提示刷新页面。"),
    # 07 补
    "Curve.domain_changed": ("定义域 max/min_domain 改变时触发。", "横轴范围刷新。"),
    "Curve.range_changed": ("值域 max/min_value 改变时触发。", "纵轴范围刷新。"),
    "Path3D.curve_changed": ("曲线改变时触发。", "路径重建。"),
    "Path3D.debug_color_changed": ("自定义调试颜色改变时触发。", "调试显示刷新。"),
    "TileMap.changed": ("TileMap 的 TileSet 改变时触发。", "换图集重建。"),
    "TileMapLayer.changed": ("图层属性改变时触发。", "图层刷新。"),
    "TileData.changed": ("任意瓦片属性改变时触发。", "瓦片数据同步。"),
    "Sprite2D.frame_changed": ("当前帧改变时触发。", "逐帧事件。"),
    "Sprite2D.texture_changed": ("纹理替换时触发。", "换图重置。"),
    "Sprite3D.frame_changed": ("3D 精灵当前帧改变时触发。", "逐帧事件。"),
    "Sprite3D.texture_changed": ("3D 精灵纹理替换时触发。", "换图重置。"),
    "MeshInstance2D.texture_changed": ("2D 网格实例纹理改变时触发。", "换图。"),
    "MultiMeshInstance2D.texture_changed": ("多网格实例纹理改变时触发。", "换图。"),
    "SkeletonProfile.profile_updated": ("配置文件值改变时触发。", "配置刷新。"),
    "FoldableGroup.expanded": ("组内某个容器被展开时触发，携带容器。", "组状态同步。"),
    # 联网核验（本地无 XML）
    "SceneMultiplayer.peer_authenticating": ("新对端连入且设置了有效 auth_callback 时触发；完成 complete_auth(id) 前不发 peer_connected，该对端只在 get_authenticating_peers() 中，仅收发认证数据。", "在此发认证数据并判定后 complete_auth(id)。"),
    "SceneMultiplayer.peer_authentication_failed": ("对端在认证完成前断开（auth_timeout 到期或手动 disconnect_peer）时触发，替代 peer_disconnected，携带 id。", "认证失败清理该对端。"),
    "SceneMultiplayer.peer_packet": ("收到 send_bytes 发来的自定义数据包时触发，携带发送者 id 与字节。", "低等级自定义报文收发。"),
    "MultiplayerSynchronizer.delta_synchronized": ("本同步器收到新的增量（delta）同步状态、属性已更新后触发。", "增量同步落地钩子。"),
    "MultiplayerSynchronizer.visibility_changed": ("某对端可见性经 update_visibility() 更新时触发，携带 for_peer。", "可见性广播同步。"),
    "WebRTCPeerConnection.data_channel_received": ("收到带内（negotiated:false，默认）新建的数据通道时触发，携带通道对象；必须持有引用，否则自动关闭。", "保存引用后收发数据。"),
    "WebRTCPeerConnection.ice_candidate_created": ("生成新 ICE 候选时触发，三参数经信令服务器转发给对端。", "把候选转发给远端。"),
    "WebRTCPeerConnection.session_description_created": ("create_offer/answer 后会话描述（type+sdp）就绪时触发，经信令转发对端。", "发送 SDP 给远端。"),
    "GridMap.cell_size_changed": ("单元格尺寸设置改变时触发，携带新尺寸。", "尺寸变更重建显示。"),
    "GridMap.changed": ("网格单元被设置或移除、内容修改时触发。", "网格变更刷新。"),
    # ===== 08 编辑器扩展 =====
    "EditorDebuggerSession.breaked": ("附加的远端实例进入断点状态时触发，携带是否可调试。", "断点进入钩子。"),
    "EditorDebuggerSession.continued": ("远端实例退出断点、继续运行时触发。", "恢复执行钩子。"),
    "EditorDebuggerSession.started": ("远端实例附加到本调试会话时触发。", "会话建立。"),
    "EditorDebuggerSession.stopped": ("远端实例从本会话断开时触发。", "会话结束清理。"),
    "EditorDock.closed": ("用上下文菜单关闭按钮关闭停靠面板时、移出父节点前触发。", "关闭前收尾。"),
    "EditorDock.opened": ("经 Editor > Editor Docks 菜单打开面板时、显示前触发。", "打开初始化。"),
    "EditorFileSystem.filesystem_changed": ("文件系统发生变化时触发。", "扫描后重建资源表。"),
    "EditorFileSystem.resources_reimported": ("资源完成重新导入时触发，携带资源路径数组。", "导入完成刷新。"),
    "EditorFileSystem.resources_reimporting": ("资源即将重新导入前触发，携带路径数组。", "导入前清理。"),
    "EditorFileSystem.resources_reload": ("扫描时至少一个资源被重载时触发，携带路径数组。", "热重载刷新。"),
    "EditorFileSystem.script_classes_updated": ("全局脚本类列表更新时触发。", "class_name 表重建。"),
    "EditorFileSystem.sources_changed": ("任一导入文件的源改变时触发，携带是否存在。", "源变更重导。"),
    "EditorInspector.edited_object_changed": ("检视器正在编辑的对象切换时触发。", "换对象重置面板。"),
    "EditorInspector.object_id_selected": ("点对象的 Edit 按钮时触发，携带对象 id。", "打开子检视。"),
    "EditorInspector.property_deleted": ("属性从检视器移除时触发，携带属性名。", "动态属性清理。"),
    "EditorInspector.property_edited": ("属性在检视器被编辑时触发，携带属性名。", "值变更回调。"),
    "EditorInspector.property_keyed": ("属性被打成动画关键帧时触发，携带属性/值/是否前进。", "关键帧钩子。"),
    "EditorInspector.property_selected": ("选中某属性时触发，携带属性名。", "属性聚焦。"),
    "EditorInspector.property_toggled": ("布尔属性被切换时触发，携带属性名与状态。", "开关同步。"),
    "EditorInspector.resource_selected": ("检视器选中某资源时触发，携带资源与路径。", "资源选择。"),
    "EditorInspector.restart_requested": ("编辑了需重启才生效的属性时触发。", "提示重启。"),
    "EditorPlugin.main_screen_changed": ("用户切换工作区（2D/3D/脚本/游戏/资产商店）时触发，携带屏幕名。", "切换视图钩子。"),
    "EditorPlugin.project_settings_changed": ("任一项目设置改变时触发。", "设置生效刷新。"),
    "EditorPlugin.resource_saved": ("给定资源落盘保存时触发，携带资源。", "保存后处理。"),
    "EditorPlugin.scene_changed": ("编辑器切换当前场景时触发，携带场景根节点。", "换场景初始化。"),
    "EditorPlugin.scene_closed": ("用户关闭场景时触发，携带文件路径。", "关闭清理。"),
    "EditorPlugin.scene_saved": ("场景落盘保存时触发，携带文件路径。", "保存后钩子。"),
    "EditorProperty.multiple_properties_changed": ("希望同时修改多个属性时自行发射，携带属性名数组与值数组。", "批量改值。"),
    "EditorProperty.object_id_selected": ("子检视器使用，选中对象时触发。", "子检视跳转。"),
    "EditorProperty.property_can_revert_changed": ("属性是否可回退（有无非默认值、是否显示回退图标）改变时触发。", "回退图标刷新。"),
    "EditorProperty.property_changed": ("属性变更信号；不要手动发射，用 emit_changed()。", "值变更，走 emit_changed。"),
    "EditorProperty.property_checked": ("属性被勾选时触发，携带属性名与状态。", "勾选同步。"),
    "EditorProperty.property_deleted": ("属性被删除时触发，携带属性名。", "删除清理。"),
    "EditorProperty.property_favorited": ("希望把属性标为收藏（置顶显示）时自行发射。", "收藏置顶。"),
    "EditorProperty.property_keyed": ("希望把当前值打成关键帧时自行发射（先确认可关键帧）。", "关键帧。"),
    "EditorProperty.property_keyed_with_value": ("以单值给属性打关键帧时自行发射，携带属性与值。", "带值关键帧。"),
    "EditorProperty.property_overridden": ("请求对当前项目做设置覆盖时触发。", "覆盖态同步。"),
    "EditorProperty.property_pinned": ("标记/取消标记属性值强制保存（即使等于默认）时自行发射。", "钉住保存。"),
    "EditorProperty.resource_selected": ("要编辑子资源时携带资源发射。", "打开子资源。"),
    "EditorProperty.selected": ("属性项被选中时触发，携带路径与可聚焦索引。", "选中聚焦。"),
    "EditorResourcePicker.resource_changed": ("被编辑资源的值改变时触发，携带资源。", "资源换值。"),
    "EditorResourcePicker.resource_selected": ("资源值设定且用户点击编辑时触发，携带资源与是否检视。", "编辑资源。"),
    "EditorResourcePreview.preview_invalidated": ("预览失效（内容改变）时触发，携带路径。", "重建缩略图。"),
    "EditorSelection.selection_changed": ("当前选择集变化时触发。", "选择同步。"),
    "EditorSettings.settings_changed": ("任一编辑器设置改变后触发。", "编辑器偏好刷新。"),
    "EditorSpinSlider.grabbed": ("抓住数值旋钮/滑条时触发。", "拖动开始。"),
    "EditorSpinSlider.ungrabbed": ("松开旋钮/滑条时触发。", "拖动结束提交。"),
    "EditorSpinSlider.updown_pressed": ("点按上下步进按钮时触发。", "步进回调。"),
    "EditorSpinSlider.value_focus_entered": ("数值输入框获得焦点时触发。", "聚焦。"),
    "EditorSpinSlider.value_focus_exited": ("数值输入框失去焦点时触发。", "失焦提交。"),
    "EditorUndoRedoManager.history_changed": ("任一历史列表变化（提交动作或清空）时触发。", "撤销栈刷新。"),
    "EditorUndoRedoManager.version_changed": ("undo/redo 导致任一历史版本变化时触发。", "版本同步。"),
    "FileSystemDock.display_mode_changed": ("切换文件显示模式或分栏模式时触发。", "视图切换。"),
    "FileSystemDock.file_removed": ("给定文件被移除时触发，携带路径。", "文件删除同步。"),
    "FileSystemDock.files_moved": ("文件从旧路径移到新路径时触发。", "移动同步。"),
    "FileSystemDock.folder_color_changed": ("文件夹颜色改变时触发。", "颜色刷新。"),
    "FileSystemDock.folder_moved": ("文件夹从旧路径移到新路径时触发。", "移动同步。"),
    "FileSystemDock.folder_removed": ("给定文件夹被移除时触发，携带路径。", "删除同步。"),
    "FileSystemDock.inherit": ("新建一个继承指定路径场景的场景时触发。", "继承新建钩子。"),
    "FileSystemDock.instantiate": ("在编辑器内实例化给定场景时触发，携带路径数组。", "实例化钩子。"),
    "FileSystemDock.resource_removed": ("外部资源文件被删除时触发，携带资源。", "资源清理。"),
    "FileSystemDock.selection_changed": ("文件面板选择变化时触发。", "选择同步。"),
    "ScriptCreateDialog.script_created": ("用户点 OK 创建脚本时触发，携带脚本。", "新脚本后处理。"),
    "ScriptEditor.editor_script_changed": ("用户切换活动脚本时触发，携带脚本。", "切换脚本。"),
    "ScriptEditor.script_close": ("编辑器即将关闭活动脚本时触发，携带脚本。", "关闭保存检查。"),
    "ScriptEditorBase.edited_script_changed": ("脚本校验完成后触发。", "校验后刷新。"),
    "ScriptEditorBase.go_to_help": ("用户请求打开特定文档页时触发，携带主题。", "跳帮助。"),
    "ScriptEditorBase.go_to_method": ("请求查看脚本某方法时触发，携带脚本与方法名。", "定位方法。"),
    "ScriptEditorBase.name_changed": ("校验后或被编辑资源改变时触发。", "改名刷新。"),
    "ScriptEditorBase.replace_in_files_requested": ("用户请求在文件系统内查找替换时触发，携带文本。", "全局替换。"),
    "ScriptEditorBase.request_help": ("用户请求上下文帮助时触发，携带主题。", "上下文帮助。"),
    "ScriptEditorBase.request_open_script_at_line": ("请求查看脚本指定行时触发，携带脚本与行号。", "打开定位。"),
    "ScriptEditorBase.request_save_history": ("上下文跳转且条目在同一脚本时触发。", "保存历史栈。"),
    "ScriptEditorBase.request_save_previous_state": ("切换脚本或在同脚本内移动光标跨 10 列以上时触发，携带状态字典。", "保存前态。"),
    "ScriptEditorBase.search_in_files_requested": ("用户请求在文件系统内搜索文本时触发，携带文本。", "全局搜索。"),
    # ===== 09 XR（有描述） =====
    "XRController3D.button_pressed": ("本控制器按钮按下时触发，携带动作名。", "按映射动作响应。"),
    "XRController3D.button_released": ("本控制器按钮松开时触发，携带动作名。", "松开响应。"),
    "XRController3D.input_float_changed": ("扳机类浮点输入改变时触发，携带动作名与值。", "扳机力度。"),
    "XRController3D.input_vector2_changed": ("摇杆/触控板移动时触发，携带动作名与向量。", "摇杆位移。"),
    "XRController3D.profile_changed": ("本控制器交互配置改变时触发，携带角色。", "配置切换。"),
    "XRInterface.play_area_changed": ("游玩区域改变时触发，携带模式。", "安全区重设。"),
    "XRNode3D.tracking_changed": ("追踪器开始/停止收到位姿更新时触发，携带状态。", "丢追踪提示。"),
    "XRPositionalTracker.button_pressed": ("本追踪器按钮按下时触发，携带动作名。", "按下响应。"),
    "XRPositionalTracker.button_released": ("本追踪器按钮松开时触发，携带动作名。", "松开响应。"),
    "XRPositionalTracker.input_float_changed": ("扳机类浮点输入改变时触发，携带动作名与值。", "力度读取。"),
    "XRPositionalTracker.input_vector2_changed": ("摇杆/触控板移动时触发，携带动作名与向量。", "位移读取。"),
    "XRPositionalTracker.pose_changed": ("所追踪位姿状态变化时触发，携带位姿。", "位姿更新。"),
    "XRPositionalTracker.pose_lost_tracking": ("所追踪位姿停止收到更新时触发，携带位姿。", "丢失兜底。"),
    "XRPositionalTracker.profile_changed": ("追踪器配置改变时触发，携带角色。", "配置同步。"),
    "XRServer.interface_added": ("新增 XR 接口时触发，携带接口名。", "接口注册。"),
    "XRServer.interface_removed": ("移除 XR 接口时触发，携带接口名。", "接口注销。"),
    "XRServer.reference_frame_changed": ("参考坐标系变换改变时触发。", "坐标系重建。"),
    "XRServer.tracker_added": ("新增追踪器时触发，携带名字与类型。", "追踪器注册。"),
    "XRServer.tracker_removed": ("移除追踪器时触发，携带名字与类型。", "追踪器注销。"),
    "XRServer.tracker_updated": ("已有追踪器更新时触发，携带名字与类型。", "刷新。"),
    "XRServer.world_origin_changed": ("世界原点变换改变时触发。", "原点重建。"),
    # ===== 10 Server 低层 =====
    "CameraServer.camera_feed_added": ("新增相机采集源时触发，携带 id。", "采集源注册。"),
    "CameraServer.camera_feed_removed": ("移除相机采集源时触发，携带 id。", "注销。"),
    "CameraServer.camera_feeds_updated": ("相机采集源列表更新时触发。", "刷新列表。"),
    "RenderingServer.frame_post_draw": ("帧末、RenderingServer 更新完所有视口后触发。", "帧末钩子（勿重绘）。"),
    "RenderingServer.frame_pre_draw": ("帧初、RenderingServer 更新视口前触发。", "帧初钩子。"),
    "TextServerManager.interface_added": ("新增文字排版接口时触发，携带接口名。", "接口注册。"),
    "TextServerManager.interface_removed": ("移除文字排版接口时触发，携带接口名。", "接口注销。"),
    # OpenXR / WebXR 联网核验（本地无 XML）
    "OpenXRInterface.session_begun": ("通知 OpenXR 会话已开始。", "会话启动初始化。"),
    "OpenXRInterface.session_focussed": ("通知会话现已获得焦点（可接收 XR 输入）。", "聚焦后开启交互。"),
    "OpenXRInterface.session_visible": ("通知会话可见（输出已送头显但不收输入，如系统菜单/摘头显）。", "可见态暂停输入。"),
    "OpenXRInterface.instance_exiting": ("通知 OpenXR 实例正在退出。", "实例退出清理。"),
    "OpenXRInterface.refresh_rate_changed": ("通知头显刷新率改变（仅 runtime 支持刷新率扩展时发射），携带新刷新率。", "刷新率切换。"),
    "OpenXRInterface.cpu_level_changed": ("通知指定子域内设备 CPU 性能等级改变，携带子域/旧等级/新等级。", "按档位降级画质。"),
    "OpenXRInterface.gpu_level_changed": ("通知指定子域内 GPU 性能等级改变，携带子域/旧等级/新等级。", "按档位降负载。"),
    "OpenXRAnchorTracker.uuid_changed": ("本空间锚点的持久化 UUID 改变时通知。", "持久化锚点更新。"),
    "OpenXRInterface.session_stopping": ("通知会话正在停止。", "停止前收尾。"),
    "OpenXRInterface.session_loss_pending": ("通知会话正处于丢失过程中。", "丢会话兜底。"),
    "OpenXRInterface.pose_recentered": ("用户重新居中时触发（runtime 把相机移回原点）。", "按居中重置位置。"),
    "WebXRInterface.session_started": ("WebXR 会话开始时触发。", "会话启动。"),
    "WebXRInterface.session_ended": ("WebXR 会话结束时触发。", "会话清理。"),
    "WebXRInterface.session_failed": ("会话失败时触发，携带错误消息。", "失败提示。"),
    "WebXRInterface.session_supported": ("查询某会话模式是否支持时触发，携带模式与是否支持。", "能力探测。"),
    "WebXRInterface.selectstart": ("某输入源开始主选择动作（点按/扳机按下沿）时触发，携带输入源 id。", "选择开始。"),
    "WebXRInterface.select": ("某输入源完成主选择动作时触发，携带 id。", "点按确认。"),
    "WebXRInterface.selectend": ("主选择动作结束时触发，携带 id。", "选择结束。"),
    "WebXRInterface.squeezestart": ("某输入源开始主挤压（握取）动作时触发，携带 id。", "握取开始。"),
    "WebXRInterface.squeeze": ("完成主挤压动作时触发，携带 id。", "握取确认。"),
    "WebXRInterface.squeezeend": ("主挤压动作结束时触发，携带 id。", "握取结束。"),
    "WebXRInterface.visibility_state_changed": ("可见性状态改变时触发。", "可见态切换。"),
    "WebXRInterface.display_refresh_rate_changed": ("显示刷新率改变时触发。", "刷新率同步。"),
    "WebXRInterface.reference_space_reset": ("参考空间重置时触发。", "空间重建。"),
}


# ───────────────────────── XML 描述清洗 ─────────────────────────
def clean_xml(t: str) -> str:
    t = re.sub(r"\[param\s+([^\]]+)\]", r"\1", t)
    t = re.sub(r"\[member\s+([^\]]+)\]", r"\1", t)
    t = re.sub(r"\[method\s+([^\]]+)\]", r"\1()", t)
    t = re.sub(r"\[signal\s+([^\]]+)\]", r"\1", t)
    t = re.sub(r"\[constant\s+([^\]]+)\]", r"\1", t)
    t = re.sub(r"\[enum\s+([^\]]+)\]", r"\1", t)
    t = re.sub(r"\[code\](.*?)\[/code\]", r"`\1`", t, flags=re.S)
    t = re.sub(r"\[/?[a-z]+\]", "", t)
    t = re.sub(r"\[url[^\]]*\](.*?)\[/url\]", r"\1", t, flags=re.S)
    t = re.sub(r"\[([A-Z][\w0-9]*)\]", r"\1", t)
    t = re.sub(r"\s+", " ", t).strip()
    m = re.split(r"(?<=[.!?])\s+", t, maxsplit=1)
    return m[0]


def load_xml_descriptions() -> dict[str, dict[str, str]]:
    out: dict[str, dict[str, str]] = {}
    for f in XML_DIR.glob("*.xml"):
        t = f.read_text(encoding="utf-8", errors="replace")
        m = re.search(r'<class name="([^"]+)"', t)
        if not m:
            continue
        name = m.group(1)
        sigs: dict[str, str] = {}
        for sm in re.finditer(r'<signal name="([^"]+)"[^>]*>(.*?)</signal>', t, re.S):
            d = re.search(r"<description>(.*?)</description>", sm.group(2), re.S)
            sigs[sm.group(1)] = clean_xml(d.group(1)) if d else ""
        out[name] = sigs
    return out


def slug(class_name: str) -> str:
    return "class_" + class_name.lower().replace("@", "")


def render_signal(sig: dict) -> str:
    args = ", ".join(
        f"{a['name']}: {a['type'].replace('enum::', '')}"
        + (f" = {a['default_value']}" if "default_value" in a else "")
        for a in sig.get("arguments", [])
    )
    return f"signal {sig['name']}({args})"


def generic_usage(sig: dict) -> str:
    params = ", ".join(
        f"{a['name']}: {a['type'].replace('enum::', '')}" for a in sig.get("arguments", [])
    )
    cb_name = "_on_" + (sig["name"][3:] if sig["name"].startswith("on_") else sig["name"])
    cb = f"func {cb_name}({params}):" if params else f"func {cb_name}():"
    return f"连接：`obj.{sig['name']}.connect({cb_name})`；回调 `{cb}`"


def build_rows(cls_name: str, cls: dict, xml_sigs: dict[str, str]) -> list[str]:
    rows = []
    for s in sorted(cls.get("signals") or [], key=lambda x: x["name"]):
        key = f"{cls_name}.{s['name']}"
        sig_text = render_signal(s)
        if key in CHINESE:
            meaning, usage = CHINESE[key]
        else:
            meaning = xml_sigs.get(s["name"], "") or "（描述待补，参见类页）"
            usage = generic_usage(s)
        link = f"[类页锚点](../builtin/{slug(cls_name)}.md#{s['name']})"
        rows.append(
            f"| `{sig_text}` | {meaning} | {usage} {link} |"
        )
    return rows


def main() -> int:
    check = "--check" in sys.argv
    api = json.loads(API_JSON.read_text(encoding="utf-8"))
    xml = load_xml_descriptions()
    classes = {c["name"]: c for c in api["classes"]}

    # 声明类 → bucket
    owner: dict[str, str] = {}
    pages: dict[str, tuple[str, str, list[str]]] = {}
    for fname, title, intro, members in BUCKETS:
        pages[fname] = (title, intro, [])
        for m in members:
            owner[m] = fname

    unclassified: dict[str, list[str]] = {}
    total = 0
    for name, cls in classes.items():
        sigs = cls.get("signals") or []
        if not sigs:
            continue
        total += len(sigs)
        bucket = owner.get(name, "99-unclassified")
        pages.setdefault(bucket, ("未分类", "尚未归类的声明类，运行生成器后按告警补入 BUCKETS。", []))
        rows = build_rows(name, cls, xml.get(name, {}))
        pages[bucket][2].append((name, cls.get("inherits") or "", rows))

    # 99-unclassified 显式建页
    pages.setdefault("99-unclassified", ("未分类", "尚未归类的声明类。", []))

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    index_lines = [BEGIN, "# 信号分类索引", "",
                   "> 本页与各分类页由 `tools/gen_signals.py` 生成，勿手改。原型签名以 extension_api.json 为唯一事实源；"
                   "完整长描述见声明类类页锚点。继承信号（如 Button 的 pressed 继承自 BaseButton）链接到声明基类。", ""]

    written: list[Path] = []
    for fname, (title, intro, groups) in pages.items():
        if fname == "99-unclassified":
            # 收集所有未归类
            groups = []
            for name, cls in classes.items():
                if (cls.get("signals") or []) and name not in owner:
                    groups.append((name, cls.get("inherits") or "", build_rows(name, cls, xml.get(name, {}))))
        page = OUT_DIR / f"{fname}.md"
        if not groups:
            # 无未归类时删除遗留空页，避免 --check 漂移与无效导航
            if page.exists():
                page.unlink()
            continue
        L = [BEGIN, f"# {title}", "", f"> {intro}", ""]
        sig_count = 0
        for name, inherits, rows in sorted(groups, key=lambda g: g[0]):
            if not rows:
                continue
            sig_count += len(rows)
            inh = f"（继承 {inherits}）" if inherits else ""
            L.append(f"## {name}{inh}")
            L.append("")
            L.append("| 原型 | 含义 | 使用方式 |")
            L.append("|---|---|---|")
            L.extend(rows)
            L.append("")
        L.append(END)
        page = OUT_DIR / f"{fname}.md"
        content = "\n".join(L) + "\n"
        if check:
            if not page.exists() or page.read_text(encoding="utf-8") != content:
                print(f"DRIFT: {page}")
                return 1
        else:
            page.write_text(content, encoding="utf-8", newline="\n")
        written.append(page)
        index_lines.append(f"- [{title}]({fname}.md)（{sig_count} 条）")
        index_lines.append("")

    index_lines.append(END)
    index_content = "\n".join(index_lines) + "\n"
    idx = OUT_DIR / "index.md"
    if check:
        if not idx.exists() or idx.read_text(encoding="utf-8") != index_content:
            print("DRIFT: signals/index.md")
            return 1
    else:
        idx.write_text(index_content, encoding="utf-8", newline="\n")

    # 告警：未归类声明类
    unc = [n for n, c in classes.items() if (c.get("signals") or []) and n not in owner]
    for n in sorted(unc):
        print(f"[unclassified] {n}: {[s['name'] for s in classes[n]['signals']]}")
    print(f"{'check' if check else 'generate'}: {len(written)} pages, {total} signal defs, unclassified {len(unc)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
