#!/usr/bin/env python3
"""P0 高频集中文翻译补丁。

覆盖 hoverdata.json 中的高频条目：
  - 16 个 GDScript 编译器内建函数
  - 36 个注解
  - 50 个核心类 brief
  - 高频成员（position/rotation/_process/_ready 等）

运行：python tools/translate_p0.py
"""
import json, pathlib

ROOT = pathlib.Path(__file__).resolve().parent.parent
HD = ROOT / "crates/godotlsp/src/hoverdata.json"

# === 16 个 GDScript 内建函数 ===
BUILTIN_FN = {
    "len": ("获取集合的元素数量，包括 Array、Dictionary、String、Packed*Array。",
            "len(container: Variant) -> int"),
    "preload": ("在编译时加载资源并返回其引用。路径必须是字符串常量或 \"^\" 前缀的字符串字面量。",
                "preload(path: String) -> Variant"),
    "load": ("在运行时从 res:// 路径异步加载资源。返回加载的资源，失败返回 null。",
             "load(path: String) -> Resource"),
    "range": ("生成整数序列。range(stop)、range(start, stop)、range(start, stop, step)。",
              "range(...) -> Array[int]"),
    "print": ("将参数打印到调试控制台。自动用空格连接，末尾换行。",
              "print(...args) -> void"),
    "printerr": ("将参数打印到调试控制台（错误流，红色）。",
                 "printerr(...args) -> void"),
    "printraw": ("将参数打印到调试控制台，不加空格和换行。",
                 "printraw(...args) -> void"),
    "prints": ("将参数打印到调试控制台，用空格连接，末尾换行。",
               "prints(...args) -> void"),
    "printt": ("将参数打印到调试控制台，用制表符连接。",
               "printt(...args) -> void"),
    "printdebug": ("打印调试信息，包含脚本位置和行号。",
                   "printdebug(...args) -> void"),
    "instance_from_id": ("根据脚本 ID 实例化脚本。",
                         "instance_from_id(id: int) -> Variant"),
    "str": ("将参数转换为字符串表示。",
            "str(...args) -> String"),
    "var2str": ("将 Variant 序列化为可读字符串，可被 str2var 还原。",
                "var2var(value: Variant) -> String"),
    "str2var": ("将 var2str 生成的字符串反序列化为 Variant。",
                "str2var(string: String) -> Variant"),
    "type_convert": ("在兼容类型之间转换值。",
                     "type_convert(what: Variant, type: int) -> Variant"),
    "dict2inst": ("将 Dictionary 转换为 Object 实例。",
                  "dict2inst(dictionary: Dictionary) -> Object"),
    "inst2dict": ("将 Object 实例序列化为 Dictionary。",
                  "inst2dict(instance: Object) -> Dictionary"),
    "Vector2": ("二维向量。x、y 分量，支持向量运算、长度、距离、插值。", None),
    "Vector3": ("三维向量。x、y、z 分量，支持向量运算、长度、距离、插值。", None),
    "Color": ("颜色。r、g、b、a 分量（0.0-1.0），支持从 HTML 色值构造。", None),
}

# === 36 个注解 ===
ANNOTATIONS = {
    "export": "将属性暴露到编辑器 Inspector 面板，可在场景文件中保存。配合类型提示使用。",
    "export_category": "在 Inspector 中创建新的属性分类标题，用于组织导出属性。",
    "export_group": "在 Inspector 中创建属性分组，组内属性缩进显示。",
    "export_subgroup": "在 Inspector 中创建子分组，嵌套在父组内。",
    "export_range": "限制导出属性的取值范围，Inspector 中显示滑块。用法：@export_range(0, 100, 1)",
    "export_color_no_alpha": "导出 Color 属性但隐藏 alpha 通道，只允许编辑 RGB。",
    "export_node_path": "导出 NodePath 属性，Inspector 中显示节点选择器。",
    "export_file": "导出 String 属性为文件路径选择器。",
    "export_dir": "导出 String 属性为目录路径选择器。",
    "export_global_file": "导出 String 属性为全局文件路径选择器。",
    "export_global_dir": "导出 String 属性为全局目录路径选择器。",
    "export_multiline": "导出 String 属性，Inspector 中显示多行文本编辑框。",
    "export_placeholder": "为导出的输入框设置占位提示文字。",
    "export_anim": "导出 Animation 属性，Inspector 中显示动画选择器。",
    "export_anim_list": "导出 Animation 列表属性。",
    "export_anim_track": "导出 Animation 轨道属性。",
    "export_custom": "自定义导出提示，直接传递 hint_string 和 usage_flags。",
    "onready": "在 _ready() 回调执行前初始化变量。常用于节点引用（如 @onready var player = $Player）。",
    "tool": "脚本在编辑器中运行。用于编辑器插件和自定义节点预览。",
    "remote": "标记变量或函数为网络同步（服务器→客户端）。已弃用，改用 @rpc。",
    "onserver": "标记变量或函数仅在服务器上运行。",
    "onclient": "标记变量或函数仅在客户端上运行。",
    "onmaster": "标记变量或函数仅在网络主服务器上运行。",
    "onpuppet": "标记变量或函数仅在客户端（非主服务器）上运行。",
    "unordered": "标记函数参数为无序调用。",
    "rpc": "标记函数为远程过程调用。用法：@rpc(authority = 1)，authority: 1=服务器, 2=客户端, 3=任意。",
    "rpc_unreliable": "标记函数为不可靠 RPC（可能丢失，速度更快）。",
    "rpc_any": "标记函数为任意端可调用的 RPC。",
    "rpc_unreliable_any": "标记函数为任意端可调用的不可靠 RPC。",
    "abstract": "标记类或方法为抽象。抽象方法必须由子类实现。",
    "warning_ignore": "抑制指定的编译警告。用法：@warning_ignore("unused_parameter")",
    "warning_ignore_start": "开始抑制指定范围的编译警告。",
    "warning_ignore_end": "结束抑制编译警告范围。",
    "static_unload": "标记脚本为静态卸载，帮助 Godot 释放脚本缓存。",
    "deprecated": "标记变量或函数为已弃用。编译器会对使用处产生警告。",
    "no_unsafe_access": "禁止在脚本中使用不安全的成员访问。",
    "deprecated_method": "标记方法为已弃用。",
}

# === 核心类 brief（P0 高频类）===
CLASS_BRIEF = {
    "Object": "所有 Godot 对象的基类。提供引用计数、信号连接、成员元数据、脚本附加等核心功能。",
    "RefCounted": "引用计数对象的基类。自动在无引用时释放，无需手动 free()。",
    "Node": "场景树中的基本单元。所有场景节点都继承自 Node。提供生命周期回调、组管理、节点查找。",
    "Node2D": "2D 游戏对象的基类。包含 position、rotation、scale 变换，CanvasItem 层级中的节点。",
    "Node3D": "3D 游戏对象的基类。包含 position、rotation、scale 变换，3D 空间中的节点。",
    "Control": "UI 控件的基类。提供布局、鼠标过滤、主题样式、锚点等 UI 功能。",
    "CanvasItem": "所有 2D 绘制节点的基类。管理绘制顺序、材质、调制颜色、可见性。",
    "Resource": "数据容器资源。与节点分离，可被多节点共享，保存为 .tres/.res 文件。",
    "PackedScene": "打包的场景资源。实例化后产生一棵节点树，是 Godot 复用的核心机制。",
    "SceneTree": "管理场景树的单例。提供节点遍历、组查询、当前场景、退出游戏等功能。",
    "Engine": "引擎全局单例。提供版本号、帧率、时间缩放、单例注册等信息。",
    "Input": "输入系统单例。处理键盘、鼠标、摇杆、动作映射（action）。",
    "InputEvent": "输入事件的基类。包含键盘、鼠标、触摸、手柄等事件类型。",
    "Vector2": "二维向量。x、y 分量，支持加减、点积、叉积、长度、距离、插值。",
    "Vector3": "三维向量。x、y、z 分量，支持向量运算、法线、交叉积、距离。",
    "Vector2i": "二维整数向量。x、y 分量为整数，用于网格坐标。",
    "Vector3i": "三维整数向量。x、y、z 分量为整数。",
    "Color": "颜色。r、g、b、a 分量（0.0-1.0），支持 from_html、to_html。",
    "Rect2": "二维矩形。由 position 和 size 定义，支持交集、包含检测。",
    "Rect2i": "二维整数矩形。",
    "AABB": "三维轴对齐包围盒。由 position 和 size 定义，用于碰撞检测。",
    "Quaternion": "四元数。用于旋转，避免万向节死锁。",
    "Basis": "3x3 旋转矩阵。表示三维旋转和缩放。",
    "Transform2D": "2D 变换。由旋转矩阵和平移组成。",
    "Transform3D": "3D 变换。由 Basis 旋转矩阵和 origin 平移组成。",
    "Plane": "三维平面。由法线和距离原点距离定义。",
    "Projection": "4x4 投影矩阵。用于相机投影。",
    "String": "UTF-8 字符串。支持查找、替换、分割、格式化、正则匹配。",
    "StringName": "优化的字符串类型。比较和哈希极快，适合标识符和枚举值。",
    "NodePath": "节点路径。用于场景树中的节点引用。",
    "Callable": "可调用对象。封装函数和对象绑定，用于信号连接和回调。",
    "Signal": "信号声明类型。用于自定义事件，connect() 连接到 Callable。",
    "Array": "动态数组。支持 typed Array[int]、Packed*Array 优化内存。",
    "Dictionary": "键值对容器。键和值都是 Variant。",
    "PackedByteArray": "紧凑字节数组。内存连续，用于二进制数据。",
    "PackedInt32Array": "紧凑 32 位整数数组。",
    "PackedFloat32Array": "紧凑 32 位浮点数组。",
    "PackedStringArray": "紧凑字符串数组。",
    "PackedVector2Array": "紧凑 Vector2 数组。",
    "PackedVector3Array": "紧凑 Vector3 数组。",
    "PackedColorArray": "紧凑 Color 数组。",
    "Math": "数学函数集合。sin/cos/tan/sqrt/lerp/clamp 等。",
    "Time": "时间单例。获取系统时间、日期、计时器。",
    "OS": "操作系统单例。命令行参数、环境变量、屏幕尺寸、退出。",
    "FileAccess": "文件读写。支持 res:// 和用户目录。",
    "DirAccess": "目录遍历和创建。",
    "HTTPRequest": "HTTP 请求节点。异步发送网络请求。",
    "Timer": "计时器节点。等待指定时间后发射 timeout 信号。",
    "Tween": "补间动画器。属性插值动画，支持并行、序列、延迟。",
    "AnimationPlayer": "动画播放节点。管理和播放骨骼、属性动画。",
    "AudioStreamPlayer": "音频播放节点。播放背景音乐和音效。",
    "RigidBody2D": "2D 刚体物理节点。受重力和力影响。",
    "CharacterBody2D": "2D 角色控制器。手动控制移动，适合平台游戏。",
    "Area2D": "2D 区域检测节点。检测物体进入/离开区域。",
    "Camera2D": "2D 相机节点。控制视口显示范围。",
    "Camera3D": "3D 相机节点。控制视口投影。",
    "MeshInstance3D": "3D 网格渲染节点。显示 3D 模型。",
    "CollisionShape2D": "2D 碰撞形状。为物理节点提供碰撞体。",
    "CollisionShape3D": "3D 碰撞形状。",
    "Label": "文本标签控件。显示单行或多行文本。",
    "Button": "按钮控件。支持点击、长按、自定义样式。",
    "LineEdit": "单行文本输入框。",
    "TextureRect": "纹理显示控件。显示图片。",
    "@GlobalScope": "全局作用域。包含所有内置常量、枚举、工具函数。",
    "@GDScript": "GDScript 语言内置函数和注解集合。",
}

# === 高频成员描述（类.成员）===
MEMBER_DESC = {
    # Node 生命周期
    "Node.position": "节点在父节点坐标系中的位置。",
    "Node.rotation": "节点绕 Z 轴的旋转角度（弧度）。",
    "Node.scale": "节点的缩放因子。(1,1) 为原始大小。",
    "Node.visible": "节点是否可见。",
    "Node.name": "节点名称。在父节点下必须唯一。",
    "Node.owner": "场景所有者。决定实例化时哪些节点被保存。",
    "Node.get_parent": "返回父节点。无父节点返回 null。",
    "Node.get_child": "按索引获取子节点。",
    "Node.get_children": "返回所有子节点数组。",
    "Node.add_child": "将节点添加为子节点。add_child 后节点进入场景树，生命周期回调开始触发。",
    "Node.remove_child": "从父节点移除节点。不销毁，只是脱离树。",
    "Node.queue_free": "在当前帧结束后安全销毁节点。推荐替代 free()。",
    "Node.find_child": "按名称递归查找子节点。",
    "Node.get_node": "按路径获取节点。路径如 $Player/Sprite2D 或 \"Player/Sprite2D\"。",
    "Node.has_node": "判断是否存在指定路径的节点。",
    "Node.is_inside_tree": "判断节点是否在场景树中。",
    "Node.get_tree": "返回场景树 SceneTree。",
    "Node.set_script": "动态附加脚本。",
    "Node.call_deferred": "延迟到帧末尾调用函数。避免在物理回调中修改树。",
    "Node.duplicate": "深拷贝节点。",
    "Node.print_tree": "打印节点树结构（调试用）。",
    # Node2D
    "Node2D.global_position": "全局位置（世界坐标系）。",
    "Node2D.global_rotation": "全局旋转角度。",
    "Node2D.global_scale": "全局缩放。",
    "Node2D.z_index": "2D 绘制层级。值越大越在前面。",
    "Node2D.z_as_relative": "z_index 是否相对于父节点。",
    "Node2D.y_sort_enabled": "是否启用按 Y 坐标排序绘制。",
    # Control
    "Control.size": "控件尺寸。",
    "Control.position": "控件位置（父容器坐标系）。",
    "Control.anchor_left": "左锚点（0.0-1.0，相对父控件宽度比例）。",
    "Control.anchor_right": "右锚点。",
    "Control.offset_left": "左偏移（像素，叠加在锚点上）。",
    "Control.mouse_filter": "鼠标事件过滤模式。STOP/IGNORE/PASS。",
    # CanvasItem
    "CanvasItem.modulate": "调制颜色。乘以此颜色影响整个节点及其子节点。",
    "CanvasItem.self_modulate": "自身调制颜色（不影响子节点）。",
    # Resource
    "Resource.resource_path": "资源在 res:// 中的路径。",
    # SceneTree
    "SceneTree.root": "根视口节点。",
    "SceneTree.current_scene": "当前运行场景的根节点。",
    "SceneTree.quit": "退出游戏。quit() 或 quit(exit_code)。",
    "SceneTree.create_timer": "创建一次性计时器。await create_timer(1.0).timeout 等待 1 秒。",
    # Input
    "Input.is_action_pressed": "判断动作是否被按下。动作在项目设置→输入映射中定义。",
    "Input.get_axis": "返回两个动作之间的差值（如左右移动）。",
    "Input.get_vector": "返回两个轴的向量。",
    "Input.is_key_pressed": "判断键盘按键是否按下。",
    # Signal
    "Signal.emit": "发射信号。4.x 推荐写法：my_signal.emit(args)。",
    "Signal.connect": "连接信号到 Callable。connect(callable, flags)。",
    "Signal.disconnect": "断开信号连接。",
    # Object
    "Object.free": "立即销毁对象。危险，推荐 queue_free()。",
    "Object.is_instance_valid": "判断 Object 是否已被销毁（悬空引用检测）。",
    "Object.set": "动态设置属性。set(property, value)。",
    "Object.get": "动态获取属性。",
    "Object.has_signal": "判断对象是否有指定信号。",
    # Timer
    "Timer.wait_time": "等待时间（秒）。",
    "Timer.start": "启动计时器。start(time) 或 start()。",
    "Timer.stop": "停止计时器。",
    "Timer.time_left": "剩余时间。",
    # Tween
    "Tween.tween_property": "补间属性。tween_property(node, \"position:x\", 100, 1.0)。",
    "Tween.tween_interval": "在动画序列中插入等待间隔。",
    "Tween.parallel": "并行执行补间动画。",
    # Vector2
    "Vector2.x": "X 分量。",
    "Vector2.y": "Y 分量。",
    "Vector2.length": "返回向量长度。",
    "Vector2.normalized": "返回单位向量（长度为 1）。",
    "Vector2.distance_to": "返回到目标点的距离。",
    "Vector2.lerp": "线性插值到目标向量。",
    # Color
    "Color.r": "红色分量（0.0-1.0）。",
    "Color.g": "绿色分量。",
    "Color.b": "蓝色分量。",
    "Color.a": "透明度。0.0 完全透明，1.0 不透明。",
    # String
    "String.length": "字符串字符数。",
    "String.format": "格式化字符串。\"Hello {name}\".format({\"name\": \"World\"})。",
    "String.find": "查找子串位置，找不到返回 -1。",
    "String.replace": "替换所有匹配子串。",
    "String.split": "按分隔符分割为字符串数组。",
    # Array
    "Array.append": "末尾追加元素。",
    "Array.size": "数组长度。",
    "Array.pop_back": "移除并返回末尾元素。",
    "Array.erase": "移除指定值的所有出现。",
    # Dictionary
    "Dictionary.has": "判断是否包含指定键。",
    "Dictionary.keys": "返回所有键的数组。",
    "Dictionary.values": "返回所有值的数组。",
    # GlobalScope constants
    "@GlobalScope.KEY_ENTER": "回车键的键码常量。",
    "@GlobalScope.KEY_SPACE": "空格键的键码常量。",
    "@GlobalScope.KEY_LEFT": "左方向键。",
    "@GlobalScope.KEY_RIGHT": "右方向键。",
    "@GlobalScope.KEY_UP": "上方向键。",
    "@GlobalScope.KEY_DOWN": "下方向键。",
    "@GlobalScope.ERR_OK": "操作成功，无错误。",
    "@GlobalScope.ERR_FAIL": "操作失败，无返回值。",
    "@GlobalScope.ERR_FILE_NOT_FOUND": "文件未找到错误。",
    "@GlobalScope.ERR_INVALID_PARAMETER": "无效参数错误。",
}


def main():
    d = json.loads(HD.read_text(encoding="utf-8"))

    # Patch builtin functions
    for name, (desc, sig) in BUILTIN_FN.items():
        if name in d["f"]:
            d["f"][name]["d"] = desc
            if sig:
                d["f"][name]["s"] = sig
        else:
            d["f"][name] = {"d": desc, "s": sig or name}

    # Patch annotations
    for name, desc in ANNOTATIONS.items():
        if name in d["a"]:
            d["a"][name] = desc

    # Patch class briefs
    for cls, brief in CLASS_BRIEF.items():
        if cls in d["c"]:
            d["c"][cls] = brief

    # Patch member descriptions
    for key, desc in MEMBER_DESC.items():
        if key in d["m"]:
            d["m"][key] = desc

    HD.write_text(json.dumps(d, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
    print(f"PATCHED: {len(BUILTIN_FN)} fns, {len(ANNOTATIONS)} annos, {len(CLASS_BRIEF)} classes, {len(MEMBER_DESC)} members")


if __name__ == "__main__":
    main()
