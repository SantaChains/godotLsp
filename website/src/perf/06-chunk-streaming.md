# 大地图分块流式加载

**TileMap 的 quadrant 只解决"不画屏幕外的瓦片"，不解决"内存里不存屏幕外的瓦片"。超大地图必须自己做 chunk 流式加载。**

## 原理：为什么需要自己做

Godot TileMap 内部有一个 `quadrant_size` 属性。它的工作方式是：

1. 把整个 TileMap 的瓦片数据按 quadrant（默认 16×16 格）分成块
2. 每帧渲染时，只提交视野内的 quadrant 到 GPU
3. **但所有瓦片数据仍然在内存里**

这意味着：
- 12K×12K 地图 = 144M 格，即使大部分不在屏幕上，内存里全存着
- 官方 issue 报告这种规模会掉帧，因为 quadrant 数量本身拖累渲染调度

> 结论：quadrant 是渲染优化，不是内存优化。超大地图必须自己把世界切成 chunk 文件，按玩家位置动态加载/卸载。

## 设计参数

| 参数 | 值 | 理由 |
|---|---|---|
| 瓦片尺寸 | 16×16 px | 像素游戏标准 |
| 游戏分辨率 | 640×480 | 一屏 = 40×30 格 |
| 单个 chunk | 40×30 格（恰好一屏） | 加载一个 chunk 就是加载一屏 |
| 加载窗口 | 5×5（25 个 chunk） | 上下左右各多缓冲 2 个 chunk，走到边界不会闪 |
| 卸载距离 | 超出窗口边缘 1 个 chunk | 延迟卸载，回头不用重新加载 |

## 项目结构

```
res://
├── chunks/
│   ├── chunk_0_0.tscn      # chunk 坐标 (0,0)，内含一个 TileMapLayer
│   ├── chunk_0_1.tscn
│   └── ...
├── maps/
│   └── map_manager.gd      # 全局管理器
└── world/
    └── player.tscn
```

每个 chunk 是一个 `.tscn` 场景，内含一个配好瓦片的 `TileMapLayer`。运行时用 `PackedScene.instantiate()` 造实例、按坐标摆位、`add_child` 入树，离开窗口就 `queue_free`。TileMapLayer 的格子数据内部是 `PackedByteArray`（`set_tile_map_data_from_array`/`get_tile_map_data_as_array`），不提供独立的 `TileMapData` 资源直接挂载，因此走场景实例化最稳。

## 完整实现

```gdscript
# map_manager.gd —— 挂在一个全局 Node2D 上（可设为 autoload）
extends Node2D

const CHUNK_W := 40          # 每 chunk 宽度（格）
const CHUNK_H := 30          # 每 chunk 高度（格）
const TILE_SIZE := 16        # 瓦片像素
const VIEW_RADIUS := 2       # 5x5 窗口

var player: Node2D
var loaded_chunks: Dictionary = {}  # Vector2i -> Node2D

func set_player(p: Node2D) -> void:
    player = p

func _physics_process(_delta: float) -> void:
    if player == null:
        return
    # 世界坐标 → chunk 坐标
    var cx := int(floor(player.position.x / (CHUNK_W * TILE_SIZE)))
    var cy := int(floor(player.position.y / (CHUNK_H * TILE_SIZE)))
    var center := Vector2i(cx, cy)

    # 加载视野内缺失的 chunk
    for dx in range(-VIEW_RADIUS, VIEW_RADIUS + 1):
        for dy in range(-VIEW_RADIUS, VIEW_RADIUS + 1):
            var coord := center + Vector2i(dx, dy)
            if not loaded_chunks.has(coord):
                load_chunk(coord)

    # 卸载超出窗口的（留一圈缓冲，回头不重复加载）
    for coord in loaded_chunks.keys():
        if abs(coord.x - center.x) > VIEW_RADIUS \
        or abs(coord.y - center.y) > VIEW_RADIUS:
            unload_chunk(coord)

func load_chunk(coord: Vector2i) -> void:
    var path := "res://chunks/chunk_%d_%d.tscn" % [coord.x, coord.y]
    if not ResourceLoader.exists(path):
        return  # 该 chunk 不存在（未探索区域）
    var scene := load(path) as PackedScene
    var chunk := scene.instantiate()
    chunk.position = Vector2(
        coord.x * CHUNK_W * TILE_SIZE,
        coord.y * CHUNK_H * TILE_SIZE
    )
    add_child(chunk)
    loaded_chunks[coord] = chunk

func unload_chunk(coord: Vector2i) -> void:
    if loaded_chunks.has(coord):
        loaded_chunks[coord].queue_free()
        loaded_chunks.erase(coord)
```

> 加载节流：不要每物理帧都尝试实例化，先用坐标差判断玩家是否跨过 chunk 边界，只在跨格时刷新窗口；大 chunk 的场景加载可放异步（`ResourceLoader.load_threaded_request`）避免单帧卡顿。

## Area2D 触发器方案（备选）

如果不想每帧轮询，可以用 Area2D 检测玩家进入 chunk 范围：

```gdscript
# chunk 场景根节点
extends Area2D

func _ready():
    collision_layer = 2       # 我是"地图区域"
    collision_mask = 1         # 检测"玩家层"
    body_entered.connect(_on_body_entered)

func _on_body_entered(body: Node2D):
    if body.is_in_group("player"):
        # 通知 MapManager 加载相邻 chunk
        get_node("/root/MapManager").on_player_near(self)
```

玩家 collision_layer 必须包含 layer 1（自己），Area2D 的 collision_mask 必须包含 layer 1（检测谁）。

## 关键参数解释

| 参数 | 调大效果 | 调小效果 |
|---|---|---|
| VIEW_RADIUS | 缓冲多，不闪，但内存高 | 内存低，边界可能闪 |
| CHUNK_W/H | 加载次数少，但单 chunk 大 | 加载频繁，但单 chunk 快 |
| TILE_SIZE | 大瓦片少格子 | 小瓦片精细 |

## 延伸阅读

- 本站：[空间数据结构](../algo/03-spatial.md)、[资产管线](../internals/06-assets.md)
- 官方：[TileMap](https://docs.godotengine.org/en/stable/classes/class_tilemap.html)
