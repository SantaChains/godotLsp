# 插值家族

**lerp 是最基础的插值：a + (b-a) * t。Godot 的 Tween 内部全靠它。**

## 基础函数

```gdscript
# lerp：线性插值
# t=0 → a，t=1 → b，t=0.5 → 中间
var x = lerp(100, 200, 0.5)  # 150

# inverse_lerp：反求 t
# 已知 v 在 a-b 之间，求 t
var t = inverse_lerp(100, 200, 150)  # 0.5

# remap：区间映射
# 把 [0,100] 的值映射到 [0,1]
var normalized = remap(50, 0, 100, 0, 1)  # 0.5

# clamp：截断
var v = clamp(150, 0, 100)  # 100
```

### remap 原理与边界

`remap(value, in_min, in_max, out_min, out_max)` 本质是 `lerp(out_min, out_max, inverse_lerp(in_min, in_max, value))`：先求 value 在输入区间的占比 t，再用 t 在输出区间取点。结果恒为 float（混用 int/float 无碍，需要整数用 `int(remap(...))` 截断）。

**越界不外推但也不截断**：value 超出输入区间时仍按比例继续算，不会夹回边界。`remap(5, 0, 4, 0, 100) = 125`（125% 处）；value 小于最小值同理得到小于 out_min 的值。要把结果限制在输出区间内，外层套 clamp：

```gdscript
# 电量 5→0，光源能量 1.25→0，且不外溢
var energy := clampf(remap(battery, 0.0, max_battery, 0.0, 1.25), 0.0, 1.25)
$PointLight2D.energy = energy
```

**除零边界**：remap 内部先做 `(value - in_min) / (in_max - in_min)`，当 `in_max == in_min` 时除数为 0，结果是 inf/nan，不可参与后续运算。对可能退化的档位（如最大速度为 0、满量程为 0）先判空或给兜底，不要直接 remap 后参与运算。

典型用例：手电筒随电池降能量、经验条按"当前经验/升级所需"映射到 ProgressBar 值域（免去手改最大值）、脚步声音高随移动速度在 0.8–1.3 间映射（速度 x 取 `absf` 避免负值落出区间）。

## Color 插值

```gdscript
# 从红渐变到绿
var c = Color.lerp(Color.RED, Color.GREEN, 0.5)  # 黄色
```

## Tween：自动插值

```gdscript
# 创建 Tween
var tween := create_tween()

# 1 秒内把 position.x 从当前值移到 500
tween.tween_property($Sprite2D, "position:x", 500, 1.0) \
    .set_trans(Tween.TRANS_SINE) \
    .set_ease(Tween.EASE_IN_OUT)

# 链式：先移到 (500, 0)，再旋转 360 度
tween.tween_property($Sprite2D, "position", Vector2(500, 0), 1.0)
tween.tween_property($Sprite2D, "rotation", TAU, 1.0)
```

### 串行与并行

Tweener 默认**串行**（上一段结束才开始下一段）。`set_parallel(true)` 之后追加的段同时跑，`chain()` 回到串行：

```gdscript
var tween := create_tween().set_parallel(true)
tween.tween_property(self, "position", target, 0.3)   # 与下一段同时
tween.tween_property(self, "modulate", Color.WHITE, 0.3)
tween.chain().tween_callback(emit_shoot)               # 两段都结束后回调
```

### 重入：先杀旧 Tween

同一属性被两个 Tween 同时驱动时，后创建的覆盖前者，表现为抽搐/跳变。反复触发的动画（换枪、闪避、连点）把 Tween 存为成员，重入先 `kill()`：

```gdscript
var _gun_tween: Tween

var Guns: Array[Node] = []
var current_gun := 0

func set_gun(value: int) -> void:
    value = posmod(value, Guns.size())   # 取余保证索引合法，防越界回绕
    if value == current_gun:
        return
    var old := current_gun
    current_gun = value
    if not is_inside_tree() or Guns.is_empty():
        return
    if _gun_tween and _gun_tween.is_valid():
        _gun_tween.kill()                # 杀掉旧 Tween，防止两段冲突
    _gun_tween = create_tween()
    _gun_tween.set_parallel(false)       # 串行：先沉旧枪，再切显隐，再抬新枪
    _gun_tween.tween_property(Guns[old], "position", base_pos + Vector3(0, -1.1, 0), 0.2)
    _gun_tween.tween_callback(func():
        Guns[old].hide()
        Guns[current_gun].show()
    )
```

规则补充：

- **Tween 不可复用**，每次动画新建一个；`Node.create_tween()` 自动 bind_node，节点释放时 Tween 自动终止，`SceneTree.create_tween()` 不绑定需自行管理。
- `tween_callback(callable)` 在序列指定点插入一次方法调用，是切显隐、发信号、`queue_free` 的正确位置，不要在属性段中间手写分支。
- `posmod` 是 floor 取余，结果恒非负，用于环形索引回绕（递增递减都安全）；普通 `%` 在被除数为负时符号跟随被除数，环形计数统一用 posmod 避免边界负值。

## 缓动曲线

| TRANS 常量 | 效果 |
|---|---|
| TRANS_LINEAR | 匀速 |
| TRANS_SINE | 平滑正弦 |
| TRANS_QUAD | 二次方 |
| TRANS_CUBIC | 三次方 |
| TRANS_ELASTIC | 弹性回弹 |
| TRANS_BACK | 轻微过冲 |
| TRANS_BOUNCE | 弹跳 |

| EASE 常量 | 效果 |
|---|---|
| EASE_IN | 慢启动，加速 |
| EASE_OUT | 快启动，减速 |
| EASE_IN_OUT | 两端慢，中间快 |

## 贝塞尔曲线

```gdscript
# Curve2D：2D 路径
var curve := Curve2D.new()
curve.add_point(Vector2(0, 0))
curve.add_point(Vector2(100, 100))
curve.add_point(Vector2(200, 0))

# 取曲线上 t 位置的点
var pos := curve.sample_baked(0.5)  # 中间点
```

相机路径、技能轨迹用这个。

## 延伸阅读

- 本站：[变换与四元数](03-transform.md)
- 官方：[Tween](https://docs.godotengine.org/en/stable/classes/class_tween.html)
