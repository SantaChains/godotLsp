# 绕过场景树直调 RenderingServer

**场景系统是建立在 Server API 之上的便利层。每个 Sprite2D 节点都有场景树开销。万级精灵时绕过它。**

## 原理：节点和 Server 的关系

Godot 渲染分两层：

```
场景层（你写的代码）
  Sprite2D 节点
    ├── position 属性
    ├── modulate 属性
    ├── 进入/离开场景树回调
    ├── _process 调度
    └── 每帧把数据同步给 Server
         ↓
Server 层（引擎内部）
  RenderingServer
    canvas_item_create()  → 创建一个绘制单元（RID）
    canvas_item_set_transform()  → 设置变换
    canvas_item_add_texture_rect()  → 提交一次绘制
```

每个 Sprite2D 节点在 Server 里对应一个 canvas_item RID。节点层多出来的是：场景树调度、生命周期回调、属性同步、信号连接。

> 值得注意的是：10000 个 Sprite2D 节点 = 10000 次节点遍历 + 10000 次属性同步。直调 Server 只需要 10000 次 `canvas_item_set_transform`，没有中间层。

## 什么时候绕过

| 精灵数量 | 推荐方案 |
|---|---|
| < 50 | Sprite2D 节点，简单直接 |
| 50-500 | 对象池复用 Sprite2D |
| 500-5000 | MultiMesh（一次 draw call 画 N 个实例） |
| > 5000 | RenderingServer 直调 + 自管理 canvas_item |

## 完整示例：像素精灵动画

```gdscript
extends Node2D

# 一个自管理的精灵，不挂 Sprite2D
var ci: RID                    # canvas item 句柄
var texture: RID               # 纹理句柄
var frame := 0
var frame_timer := 0.0
const FRAME_TIME := 0.1       # 每帧 0.1 秒
const FRAME_W := 16
const SHEET_COLS := 4          # 精灵表一行 4 帧

func _ready():
    # 1. 创建 canvas item（不进场景树）
    ci = RenderingServer.canvas_item_create()
    # 2. 挂到当前节点的 canvas 下（继承坐标）
    RenderingServer.canvas_item_set_parent(ci, get_canvas_item())
    # 3. 加载纹理
    var tex: Texture2D = load("res://player_sheet.png")
    texture = tex.get_rid()

func _process(delta):
    # 动画计时
    frame_timer += delta
    if frame_timer >= FRAME_TIME:
        frame_timer -= FRAME_TIME
        frame = (frame + 1) % SHEET_COLS

    # 设置位置（世界坐标）
    RenderingServer.canvas_item_set_transform(ci,
        Transform2D(0, Vector2(100, 200)))

    # 绘制当前帧
    var src_x := frame * FRAME_W
    RenderingServer.canvas_item_clear(ci)  # 清掉上一帧的绘制
    RenderingServer.canvas_item_add_texture_rect_region(
        ci,
        Rect2(0, 0, FRAME_W, FRAME_W),        # 目标位置
        texture,
        Rect2(src_x, 0, FRAME_W, FRAME_W)     # 精灵表区域
    )

func _exit_tree():
    # 必须手动释放，否则内存泄漏
    RenderingServer.free_rid(ci)
```

## 核心 API 速查

| API | 作用 |
|---|---|
| `canvas_item_create()` | 创建一个绘制单元，返回 RID |
| `canvas_item_set_parent(ci, parent_ci)` | 挂到某个 canvas 下（决定坐标系） |
| `canvas_item_set_transform(ci, xform)` | 设置位置/旋转/缩放 |
| `canvas_item_clear(ci)` | 清空上一帧的绘制命令 |
| `canvas_item_add_texture_rect(ci, rect, texture)` | 画整块纹理 |
| `canvas_item_add_texture_rect_region(ci, dst, texture, src)` | 画纹理的某块区域（动画切片） |
| `free_rid(ci)` | 释放 |

## 与 MultiMesh 的区别

| | RenderingServer 直调 | MultiMesh |
|---|---|---|
| 绘制调用 | 每帧每个 item 一次提交 | 一次提交画 N 个实例 |
| 灵活性 | 每个 item 可不同纹理/不同动画 | 同一纹理同一网格 |
| 万级实例 | 适合（自管理数组） | 更适合（GPU 批量） |
| 代码复杂度 | 高 | 中 |

## 代价

| 代价 | 具体表现 |
|---|---|
| 无 _process 调度 | 自己写循环驱动动画 |
| 无编辑器可视 | 场景编辑器里看不到，运行时才画 |
| 无信号 | 碰撞检测要自己做或用 PhysicsServer |
| 手动释放 | `free_rid()` 忘调 = 内存泄漏 |
| 坐标继承 | `set_parent` 搞错层级就位置错乱 |

## 自定义序列化（视频资料中的模式）

不走 Godot 编辑器，纯代码驱动数据：

```gdscript
# serializer.gd
extends RefCounted

static func serialize(obj: Object) -> Dictionary:
    var dict := {}
    for prop in obj.get_property_list():
        if prop.usage & PROPERTY_USAGE_STORAGE:
            dict[prop.name] = obj.get(prop.name)
    return dict

static func deserialize(data: Dictionary, obj: Object):
    for key in data:
        obj.set(key, data[key])
```

把 Dictionary 存成 .tres（ResourceSaver.save）或 JSON（FileAccess），运行时加载重建。

## 延伸阅读

- 本站：[Godot Server 架构即 DOD](../thought/04-servers-dod.md)、[批量与结构优化](03-bulk.md)
- 官方：[Using servers](https://docs.godotengine.org/en/stable/tutorials/performance/using_servers.html)
