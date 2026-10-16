# 噪声与程序化生成

**Perlin/Simplex 噪声是地形和动画基础。fBm 多层叠加产生自然感。**

## 噪声进化

| 噪声 | 特点 |
|---|---|
| Value Noise | 格子插值，有方块感 |
| Perlin | 梯度插值，平滑 |
| Simplex | 更快，各向同性 |
| fBm | 多层噪声叠加，自然 |

## FastNoiseLite 基础

```gdscript
var noise := FastNoiseLite.new()
noise.noise_type = FastNoiseLite.TYPE_SIMPLEX_SMOOTH
noise.frequency = 0.1          # 越大细节越碎
noise.seed = 12345             # 固定种子可复现
var height := noise.get_noise_2d(x, y)  # -1 到 1
```

## fBm 分形叠加

```gdscript
# 叠加多个频率的噪声，权重递减，产生山峦层次
func fbm(x: float, y: float, octaves: int) -> float:
    var total := 0.0
    var amplitude := 1.0
    var frequency := 1.0
    var max_val := 0.0
    for i in octaves:
        total += noise.get_noise_2d(x * frequency, y * frequency) * amplitude
        max_val += amplitude
        amplitude *= 0.5
        frequency *= 2.0
    return total / max_val
```

## 地形着色

```gdscript
# 高度图阈值着色
if h < -0.2:
    color = Color(0.2, 0.4, 0.8)  # 水
elif h < 0.0:
    color = Color(0.8, 0.8, 0.4)  # 沙滩
elif h < 0.3:
    color = Color(0.2, 0.6, 0.2)  # 草
else:
    color = Color(0.5, 0.5, 0.5)  # 岩石
```

## Roguelike 地牢生成

核心两步：随机放不重叠矩形房间 → 走廊连接。

```gdscript
func generate_rooms(grid: Array, count: int, rng: RandomNumberGenerator) -> Array:
    var rooms := []
    for i in count:
        var w := rng.randi_range(5, 12)
        var h := rng.randi_range(5, 12)
        var pos := Vector2i(rng.randi_range(0, 40), rng.randi_range(0, 24))
        var room := Rect2i(pos, w, h)
        # 不重叠才保留（简化版，略去碰撞检测）
        rooms.append(room)
    # 相邻房间用走廊连接（水平+垂直两段）
    for i in range(1, rooms.size()):
        connect_corridor(rooms[i-1], rooms[i])
    return rooms
```

## 延伸阅读

- 本站：[随机与概率](../math/05-random.md)
- 官方：[FastNoiseLite](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html)
- Red Blob Games：[Procedural Generation](https://www.redblobgames.com/)
