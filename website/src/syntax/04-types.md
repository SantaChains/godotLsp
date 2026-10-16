# 类型系统

GDScript 的类型分两层：**内建值类型**（`int`/`float`/`bool`/`String`/数学向量族，按值传递或写时复制）与**对象类型**（`Object` 谱系 + `Array`/`Dictionary` 容器，按引用传递）。全表与语义如下。

## 基本类型

| 类型 | 字面量示例 | 语义要点 |
|---|---|---|
| `int` | `42` | 64 位有符号 |
| `float` | `3.14` | 64 位双精度 |
| `bool` | `true` | 与 int 不隐式互转（`1` 不是 `true`） |
| `String` | `"hi"` | 不可变；操作返回新串 |
| `StringName` | `&"name"` | 不可变、驻留（interned）；比较是**指针级**，做字典键/高频比较首选 |
| `NodePath` | `^"A/B"` | 场景树路径；`$"A/B"` 的底层类型 |
| `Callable` | `self.method` | 可调用体一等公民，见[函数章](07-functions.md#callable-与-lambda) |
| `Signal` | `health_changed` | 信号对象的引用，见[信号章](10-signals.md) |
| `nil/null` | `null` | 空引用；对象类型变量缺省值 |

## 数学类型（一表通览）

| 族 | 类型 | 通用运算 |
|---|---|---|
| 向量 | `Vector2` `Vector2i` `Vector3` `Vector3i` `Vector4` `Vector4i` | 加减、数乘、`length()`、`dot()`、`cross()`（3D）、`normalized()` |
| 矩形 | `Rect2` `Rect2i` | `intersects()` `encloses()` `grow()` |
| 包围盒 | `AABB` | 3D 盒，同 Rect2 风格 |
| 旋转 | `Quaternion` | 免万向节锁，`slerp()` 插值，见[数学篇](../math/03-transform.md) |
| 变换 | `Transform2D` `Transform3D` `Basis` `Plane` `Projection` | 局部↔全局坐标的载体，见[范式篇 · 2D/3D 空间](../paradigm/06-space.md) |
| 颜色 | `Color` | `Color(1, 0, 0)` 或 `Color("#ff0000")` |

`i` 后缀 = 整数分量版（网格、像素坐标用）；无 `i` = 浮点（物理、渲染用）。**二者不隐式互转**，`Vector2(1,1) + Vector2i(1,1)` 直接报错，显式 `Vector2(v2i)` 转换。

## 容器

| 容器 | 写法 | 特性 |
|---|---|---|
| `Array` | `[1, "a", true]` | 动态数组，元素任意 Variant |
| 类型化 Array | `Array[int]`、`Array[Node2D]` | 4.x：编译期检查元素类型，遍历无装箱 |
| `Dictionary` | `{"hp": 100, 1: "x"}` | 任意键值；插入序遍历 |
| 类型化 Dictionary | `Dictionary[String, int]` | 4.4+ |
| Packed 数组 | `PackedInt32Array` 等 8 种 | 连续内存、无装箱；大数据量（网格顶点、像素）专用 |

```gdscript
var scores: Array[int] = [90, 85]
var by_name: Dictionary[String, int] = {"alice": 90}   # 4.4+
var verts := PackedVector3Array()   # 几何数据用 Packed，别用 Array[Vector3]
```

> 需要注意的是：**方括号**是 `Array[int]`，泛型尖括号写法 `Array<int>` 不存在。Packed 与 typed Array 的取舍：读写密集的定长数值用 Packed，一般逻辑数据用 typed Array。

## 类型转换与检查

```gdscript
if node is CharacterBody2D:        # 类型检查（含继承链）
    var body := node as CharacterBody2D   # 安全转换，失败得 null
    var n := int(3.7)              # 显式构造转换：n == 3（截断，非四舍五入）
    var f := float(3)              # f == 3.0
    var s := str(42)               # "42"
```

三条硬规则：

1. **`int` ↔ `float` 的坑在除法**：`5 / 2 == 2`；要小数写 `5 / 2.0`。
2. **`is` 只认类与内建类型名**，不能对 Variant 里的字符串用 `is "xxx"`。
3. `as` 失败返回 `null` 而不是报错——**空引用往往从这里来**，转换后立刻判空。

## 延伸阅读

- 官方英文文档：[GDScript reference · Built-in types](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html)、[Static typing](https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/static_typing.html)
- 本站：[流程控制](05-flow.md)（下一步）、[向量实战](../math/01-vectors.md)、类成员锚点查 [类参考](../builtin/index.md)
