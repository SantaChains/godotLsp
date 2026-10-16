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
# 对局开始时把种子存档，重放时用同一种子重现全部随机事件
rng.seed = save_game.rng_seed

# 结束时存当前种子（或存完整 rng 状态）
save_game.rng_seed = rng.seed
```

> 值得注意的是：固定种子 = 可复现。肉鸽的关卡、掉落、抽卡都应从一个种子派生，这样录像回放、bug 复现、对战公平性都成立。全局 `randf()` 不受实例种子控制，所以统一用 `RandomNumberGenerator` 实例。

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
var items = [
    {"name": "剑", "weight": 5},
    {"name": "盾", "weight": 3},
    {"name": "药水", "weight": 2},
]
var total = items.reduce(func(acc, i): return acc + i.weight, 0)
var roll = rng.randf() * total
var acc = 0.0
for item in items:
    acc += item.weight
    if roll < acc:
        return item.name
```

## 延伸阅读

- 本站：[噪声与程序化生成](../algo/04-procgen.md)
- 官方：[RandomNumberGenerator](https://docs.godotengine.org/en/stable/classes/class_randomnumbergenerator.html)
