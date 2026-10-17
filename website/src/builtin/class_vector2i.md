# Vector2i

> 二维整数向量。x、y 分量为整数，用于网格坐标。

<details><summary>官方英文描述</summary>

A 2-element structure that can be used to represent 2D grid coordinates or any other pair of integers.

It uses integer coordinates and is therefore preferable to Vector2 when exact precision is required. Note that the values are limited to 32 bits, and unlike Vector2 this cannot be configured with an engine build option. Use int or PackedInt64Array if 64-bit values are needed.

Note: In a boolean context, a Vector2i will evaluate to false if it's equal to Vector2i(0, 0). Otherwise, a Vector2i will always evaluate to true.

</details>

## 构造函数

```gdscript
Vector2i()
```

Constructs a default-initialized Vector2i with all components set to 0.

```gdscript
Vector2i(from: Vector2i)
```

Constructs a Vector2i as a copy of the given Vector2i.

```gdscript
Vector2i(from: Vector2)
```

Constructs a new Vector2i from the given Vector2 by truncating components' fractional parts (rounding towards zero). For a different behavior consider passing the result of Vector2.ceil(), Vector2.floor() or Vector2.round() to this constructor instead.

```gdscript
Vector2i(x: int, y: int)
```

Constructs a new Vector2i from the given x and y.

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

## 运算符

```gdscript
operator !=(right: Vector2i)
```

Returns true if the vectors are not equal.

```gdscript
operator %(right: Vector2i)
```

Gets the remainder of each component of the Vector2i with the components of the given Vector2i. This operation uses truncated division, which is often not desired as it does not work well with negative numbers. Consider using @GlobalScope.posmod() instead if you want to handle negative numbers.

```gdscript
print(Vector2i(10, -20) % Vector2i(7, 8)) # Prints (3, -4)
```

```gdscript
operator %(right: int)
```

Gets the remainder of each component of the Vector2i with the given int. This operation uses truncated division, which is often not desired as it does not work well with negative numbers. Consider using @GlobalScope.posmod() instead if you want to handle negative numbers.

```gdscript
print(Vector2i(10, -20) % 7) # Prints (3, -6)
```

```gdscript
operator *(right: Vector2i)
```

Multiplies each component of the Vector2i by the components of the given Vector2i.

```gdscript
print(Vector2i(10, 20) * Vector2i(3, 4)) # Prints (30, 80)
```

```gdscript
operator *(right: float)
```

Multiplies each component of the Vector2i by the given float. Returns a Vector2.

```gdscript
print(Vector2i(10, 15) * 0.9) # Prints (9.0, 13.5)
```

```gdscript
operator *(right: int)
```

Multiplies each component of the Vector2i by the given int.

```gdscript
operator +(right: Vector2i)
```

Adds each component of the Vector2i by the components of the given Vector2i.

```gdscript
print(Vector2i(10, 20) + Vector2i(3, 4)) # Prints (13, 24)
```

```gdscript
operator -(right: Vector2i)
```

Subtracts each component of the Vector2i by the components of the given Vector2i.

```gdscript
print(Vector2i(10, 20) - Vector2i(3, 4)) # Prints (7, 16)
```

```gdscript
operator /(right: Vector2i)
```

Divides each component of the Vector2i by the components of the given Vector2i.

```gdscript
print(Vector2i(10, 20) / Vector2i(2, 5)) # Prints (5, 4)
```

```gdscript
operator /(right: float)
```

Divides each component of the Vector2i by the given float. Returns a Vector2.

```gdscript
print(Vector2i(1, 2) / 2.5) # Prints (0.4, 0.8)
```

```gdscript
operator /(right: int)
```

Divides each component of the Vector2i by the given int.

```gdscript
operator <(right: Vector2i)
```

Compares two Vector2i vectors by first checking if the X value of the left vector is less than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors. This operator is useful for sorting vectors.

```gdscript
operator <=(right: Vector2i)
```

Compares two Vector2i vectors by first checking if the X value of the left vector is less than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors. This operator is useful for sorting vectors.

```gdscript
operator ==(right: Vector2i)
```

Returns true if the vectors are equal.

```gdscript
operator >(right: Vector2i)
```

Compares two Vector2i vectors by first checking if the X value of the left vector is greater than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors. This operator is useful for sorting vectors.

```gdscript
operator >=(right: Vector2i)
```

Compares two Vector2i vectors by first checking if the X value of the left vector is greater than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors. This operator is useful for sorting vectors.

```gdscript
operator [](index: int)
```

Access vector components using their index. v[0] is equivalent to v.x, and v[1] is equivalent to v.y.

```gdscript
operator unary+()
```

Returns the same value as if the + was not there. Unary + does nothing, but sometimes it can make your code more readable.

```gdscript
operator unary-()
```

Returns the negative value of the Vector2i. This is the same as writing Vector2i(-v.x, -v.y). This operation flips the direction of the vector while keeping the same magnitude.

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
