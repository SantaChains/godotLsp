# 动画消费路径选型

**同一张精灵表，引擎有四条消费路径。选错路径不会报错，但会让时序控制、复用与性能付出额外成本。** 本页给出每条路径的完整可复制用法与选型边界。

## 路径一：AnimatedSprite2D + SpriteFrames {#animatedsprite2d}

最简单的角色/特效动画方案。`SpriteFrames` 是独立资源，面板内 Add frames from Sprite Sheet 按横竖网格数切帧，定义动画帧序列、FPS、是否循环。

```gdscript
extends CharacterBody2D

@onready var anim: AnimatedSprite2D = $AnimatedSprite2D

func _ready() -> void:
    anim.play("idle")   # 切换并播放；只赋 anim.animation = "run" 不会自动播放

func _physics_process(delta: float) -> void:
    var dir := Input.get_vector("left", "right", "up", "down")
    velocity = dir * speed
    move_and_slide()
    update_animation(dir)

func update_animation(dir: Vector2) -> void:
    if dir == Vector2.ZERO:
        # 停在第 0 帧：pause() 保留进度位，stop() 重置回第 0 帧并清空 custom_speed
        anim.stop()
        return
    # 水平优先：先判 x，再判 y
    if absf(dir.x) > absf(dir.y):
        anim.animation = "left_right"
        anim.flip_h = dir.x < 0   # 只做一套水平帧，反向翻转
    else:
        anim.animation = "up" if dir.y < 0 else "down"
    anim.play()
```

关键 API 语义（4.x 类参考核验）：

| API | 语义 |
|---|---|
| `anim.play(name: StringName = &"", custom_speed: float = 1.0, from_end: bool = false)` | 切换动画并播放；`custom_speed` 临时倍速，`from_end=true` 从末帧倒放 |
| `anim.pause()` | 暂停，保留 `frame` 与 `frame_progress`，无参 `play()` 从断点续播 |
| `anim.stop()` | 停止并把位置重置到第 0 帧，custom_speed 重置为 1.0 |
| `anim.animation: StringName` | 当前动画名，可赋值换动画，但单独赋值不改变播放/暂停状态，要播放仍需 `play()` |
| `anim.speed_scale: float` | 全局倍速，1.0 正常、0.5 半速、2.0 倍速，必须 >0 |
| `anim.is_playing() -> bool` | 是否正在播放 |
| `anim.frame: int` / `frame_progress: float` | 当前帧索引 / 当前帧内进度（0–1） |
| `anim.flip_h` / `flip_v: bool` | 水平/垂直翻转，不复制帧 |

> 常见误区：只写 `anim.animation = "run"` 而不调 `play()`，动画停在首帧；idle 想停住时用 `pause()` 会从断点续，用 `stop()` 才回到待机第 0 帧。

动态取动画名（随机播放、读资源）：

```gdscript
var names: Array[String] = anim.sprite_frames.get_animation_names()
anim.animation = names.pick_random()
anim.play()
```

## 路径二：Sprite2D 的 hframes / vframes / frame {#hframes}

最轻量，不建任何额外资源。`Sprite2D` 把纹理均分为 `hframes × vframes` 网格，`frame` 是从左上起按行优先的索引（0 起）。

```gdscript
@onready var sprite: Sprite2D = $Sprite2D

func _ready() -> void:
    sprite.texture = preload("res://assets/items.png")
    sprite.hframes = 4          # 4 列
    sprite.vframes = 4          # 4 行，共 16 格
    sprite.frame = 3            # 取第 4 格（行优先索引）
```

适用：静态图标集合、由代码按索引轮换的简单帧、不需要循环时序的场合。**它不提供帧率与循环**，逐帧推进要自己在 `_process` 里按 delta 累加索引，因此角色动画不选这条路。

## 路径三：Sprite2D + AnimationPlayer {#animationplayer}

需要变时序、演出类动画（位移+缩放+帧切换混合、缓动、事件回调）时，由 `AnimationPlayer` 对属性做关键帧。两种取帧方式：

- 直接关键帧 `frame`（配合 hframes/vframes）。
- 开启 `region_enabled = true`，关键帧 `region_rect`（Rect2 区域）。

```gdscript
@onready var ap: AnimationPlayer = $AnimationPlayer

func interact() -> void:
    ap.play("chest_open")   # 动画轨道内含 frame / region_rect / position 关键帧
```

> AnimationTree 驱动 region 插值时，矩形会被线性插值导致撕裂。把该轨道 Callback Mode 设为 Discrete（Dominant），让关键帧直接跳变而非插值。

## 路径四：AtlasTexture 独立资源 {#atlastexture}

`AtlasTexture` 是 `Texture2D` 的子资源，三属性：`atlas`（源大图）、`region`（Rect2 裁切区域）、`margin`（区域外扩），另有 `filter_clip` 防邻格渗色。可交给任何接受 Texture2D 的节点（Sprite2D、TextureRect、Button 图标、SpriteFrames 帧）。

```gdscript
# 代码构造（多数情况在编辑器里建 .tres 更直接）
var atlas := AtlasTexture.new()
atlas.atlas = preload("res://assets/atlas.png")
atlas.region = Rect2(32, 0, 32, 32)   # x, y, w, h
$Sprite2D.texture = atlas
```

**关键权衡**：`region_rect` 内嵌在节点场景文件里；`AtlasTexture` 是独立 `.tres`。50 个敌人变体引用同一图集时，改图集布局后 `.tres` 重新生成即全部生效，而 50 个场景内嵌的 region_rect 需逐一手改。规范：**跨场景复用的帧一律用 AtlasTexture `.tres`，region_rect 只用于一次性精灵**。

## TileSet 图集

瓦片走 TileSet 编辑器的 atlas 页签，自动网格化，支持碰撞层、导航层、地形集（autotiling），多个图集可合并为单一大图集。详见类参考 [TileSet](../builtin/class_tileset.md) 与 [TileMapLayer](../builtin/class_tilemaplayer.md)。

## 选型对照

| 维度 | AnimatedSprite2D | Sprite2D 索引 | Sprite2D+AnimationPlayer | AtlasTexture |
|---|---|---|---|---|
| 循环帧时序 | 内置 FPS/循环 | 无，自推 | 轨道控制 | 无 |
| 变时序/缓动/事件 | 弱 | 无 | 强 | 无 |
| 跨场景复用 | SpriteFrames 可共享 | 每节点重复参数 | 每节点重复 | 独立 .tres，改一处全变 |
| 资源开销 | SpriteFrames 资源 | 零额外资源 | AnimationPlayer 轨道 | AtlasTexture 资源 |
| 适用 | 角色/特效循环动画 | 静态图标/索引轮换 | 演出动画 | 复用帧/图集切片 |

## 延伸阅读

- 官方：[AnimatedSprite2D](../builtin/class_animatedsprite2d.md)、[Sprite2D](../builtin/class_sprite2d.md)、[AtlasTexture](../builtin/class_atlastexture.md)、[AnimationPlayer](../builtin/class_animationplayer.md)
- 本站：[精灵表规范](00-sprite-sheet.md)、[delta 与时间](../paradigm/05-delta.md)
