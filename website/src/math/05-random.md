# 随机与概率

**`RandomNumberGenerator` 比全局 `randf()` 更可控。种子可复现。**

## 全局 vs 实例

```gdscript
# 全局（简单但不可控）
randomize()  # 用时间做种子
var n = randi() % 10

# 实例（推荐，可复现）
var rng = RandomNumberGenerator.new()
rng.seed = 12345  # 固定种子，每次运行结果相同
var n = rng.randi_range(0, 9)
```

## 种子保存与复现

```gdscript
# 从头复现整局：对局开始时固定 seed
rng.seed = save_game.rng_seed

# 中途存档要复现"接下来的随机流"：存完整内部状态，而不只是 seed
save_game.rng_state = rng.state
# 读档：rng.state = save_game.rng_state
```

> 值得注意的是：固定 seed 只能从起点复现；存档时点之后已经消耗掉的随机序列，必须存 `rng.state` 才能接着复现。肉鸽关卡、掉落、抽卡从一个种子派生，回放、bug 复现、对战公平性才成立。全局 `randf()/randi()` 不受实例种子控制，统一用 `RandomNumberGenerator` 实例。

## 常用分布

| 需求 | 方法 |
|---|---|
| 均匀整数 | `rng.randi_range(min, max)` |
| 均匀浮点 | `rng.randf_range(min, max)` |
| 正态分布 | `rng.randfn(mean, stddev)` |
| 概率判定 | `rng.randf() < 0.3`（30% 概率） |

## 加权抽取

```gdscript
# 按权重抽物品
var items := [
    {"name": "剑", "weight": 5},
    {"name": "盾", "weight": 3},
    {"name": "药水", "weight": 2},
]

func pick_weighted(rng: RandomNumberGenerator) -> String:
    var total := 0
    for item in items:
        total += item.weight
    var roll := rng.randi_range(0, total - 1)
    var acc := 0
    for item in items:
        acc += item.weight
        if roll < acc:
            return item["name"]
    return items[-1]["name"]
```

## 延伸阅读

- 本站：[噪声与程序化生成](../algo/04-procgen.md)
- 官方：[RandomNumberGenerator](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html)
