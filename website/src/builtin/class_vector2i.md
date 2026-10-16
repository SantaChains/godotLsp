# Vector2i

> 二维整数向量。x、y 分量为整数，用于网格坐标。

<details><summary>官方英文描述</summary>

A 2-element structure that can be used to represent 2D grid coordinates or any other pair of integers.

It uses integer coordinates and is therefore preferable to Vector2 when exact precision is required. Note that the values are limited to 32 bits, and unlike Vector2 this cannot be configured with an engine build option. Use int or PackedInt64Array if 64-bit values are needed.

Note: In a boolean context, a Vector2i will evaluate to false if it's equal to Vector2i(0, 0). Otherwise, a Vector2i will always evaluate to true.

</details>

## 方法

### abs {#abs}

```gdscript
Vector2i abs() const
```

Returns a new vector with all components in absolute values (i.e. positive).

### aspect {#aspect}

```gdscript
float aspect() const
```

Returns the aspect ratio of this vector, the ratio of x to y.

### clamp {#clamp}

```gdscript
Vector2i clamp(min: Vector2i, max: Vector2i) const
```

Returns a new vector with all components clamped between the components of min and max, by running @GlobalScope.clamp() on each component.

### clampi {#clampi}

```gdscript
Vector2i clampi(min: int, max: int) const
```

Returns a new vector with all components clamped between min and max, by running @GlobalScope.clamp() on each component.

### distance_squared_to {#distance_squared_to}

```gdscript
int distance_squared_to(to: Vector2i) const
```

Returns the squared Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

This method runs faster than distance_to(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### distance_to {#distance_to}

```gdscript
float distance_to(to: Vector2i) const
```

Returns the Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

### length {#length}

```gdscript
float length() const
```

Returns the length (magnitude) of this vector.

### length_squared {#length_squared}

```gdscript
int length_squared() const
```

Returns the squared length (squared magnitude) of this vector.

This method runs faster than length(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### max {#max}

```gdscript
Vector2i max(with: Vector2i) const
```

Returns the component-wise maximum of this and with, equivalent to Vector2i(maxi(x, with.x), maxi(y, with.y)).

### max_axis_index {#max_axis_index}

```gdscript
int max_axis_index() const
```

Returns the axis of the vector's highest value. See AXIS_* constants. If all components are equal, this method returns AXIS_X.

### maxi {#maxi}

```gdscript
Vector2i maxi(with: int) const
```

Returns the component-wise maximum of this and with, equivalent to Vector2i(maxi(x, with), maxi(y, with)).

### min {#min}

```gdscript
Vector2i min(with: Vector2i) const
```

Returns the component-wise minimum of this and with, equivalent to Vector2i(mini(x, with.x), mini(y, with.y)).

### min_axis_index {#min_axis_index}

```gdscript
int min_axis_index() const
```

Returns the axis of the vector's lowest value. See AXIS_* constants. If all components are equal, this method returns AXIS_Y.

### mini {#mini}

```gdscript
Vector2i mini(with: int) const
```

Returns the component-wise minimum of this and with, equivalent to Vector2i(mini(x, with), mini(y, with)).

### sign {#sign}

```gdscript
Vector2i sign() const
```

Returns a new vector with each component set to 1 if it's positive, -1 if it's negative, and 0 if it's zero. The result is identical to calling @GlobalScope.sign() on each component.

### snapped {#snapped}

```gdscript
Vector2i snapped(step: Vector2i) const
```

Returns a new vector with each component snapped to the closest multiple of the corresponding component in step.

### snappedi {#snappedi}

```gdscript
Vector2i snappedi(step: int) const
```

Returns a new vector with each component snapped to the closest multiple of step.

## 属性

### x {#x}

```gdscript
x: int
```

X 分量（整数）。

### y {#y}

```gdscript
y: int
```

Y 分量（整数）。

## 枚举

### Axis {#enum-Axis}

```gdscript
enum Axis { AXIS_X = 0, AXIS_Y = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### AXIS_X {#AXIS_X}

```gdscript
AXIS_X = 0
```

Enumerated value for the X axis. Returned by max_axis_index() and min_axis_index().

#### AXIS_Y {#AXIS_Y}

```gdscript
AXIS_Y = 1
```

Enumerated value for the Y axis. Returned by max_axis_index() and min_axis_index().

---

## 常量

### ZERO {#ZERO}

```gdscript
const ZERO = Vector2i(0, 0)
```

整数零向量 (0, 0)。

### ONE {#ONE}

```gdscript
const ONE = Vector2i(1, 1)
```

整数单位向量 (1, 1)。

### MIN {#MIN}

```gdscript
const MIN = Vector2i(-2147483648, -2147483648)
```

各分量取整数最小的向量。

### MAX {#MAX}

```gdscript
const MAX = Vector2i(2147483647, 2147483647)
```

各分量取整数最大的向量。

### LEFT {#LEFT}

```gdscript
const LEFT = Vector2i(-1, 0)
```

整数左向向量 (-1, 0)。

### RIGHT {#RIGHT}

```gdscript
const RIGHT = Vector2i(1, 0)
```

整数右向向量 (1, 0)。

### UP {#UP}

```gdscript
const UP = Vector2i(0, -1)
```

整数上向向量 (0, -1)。

### DOWN {#DOWN}

```gdscript
const DOWN = Vector2i(0, 1)
```

整数下向向量 (0, 1)。
