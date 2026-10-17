# Vector3i

> 三维整数向量。x、y、z 分量为整数。

<details><summary>官方英文描述</summary>

A 3-element structure that can be used to represent 3D grid coordinates or any other triplet of integers.

It uses integer coordinates and is therefore preferable to Vector3 when exact precision is required. Note that the values are limited to 32 bits, and unlike Vector3 this cannot be configured with an engine build option. Use int or PackedInt64Array if 64-bit values are needed.

Note: In a boolean context, a Vector3i will evaluate to false if it's equal to Vector3i(0, 0, 0). Otherwise, a Vector3i will always evaluate to true.

</details>

## 构造函数

```gdscript
Vector3i()
```

Constructs a default-initialized Vector3i with all components set to 0.

```gdscript
Vector3i(from: Vector3i)
```

Constructs a Vector3i as a copy of the given Vector3i.

```gdscript
Vector3i(from: Vector3)
```

Constructs a new Vector3i from the given Vector3 by truncating components' fractional parts (rounding towards zero). For a different behavior consider passing the result of Vector3.ceil(), Vector3.floor() or Vector3.round() to this constructor instead.

```gdscript
Vector3i(x: int, y: int, z: int)
```

Returns a Vector3i with the given components.

## 方法

### abs {#abs}

```gdscript
Vector3i abs() const
```

Returns a new vector with all components in absolute values (i.e. positive).

### clamp {#clamp}

```gdscript
Vector3i clamp(min: Vector3i, max: Vector3i) const
```

Returns a new vector with all components clamped between the components of min and max, by running @GlobalScope.clamp() on each component.

### clampi {#clampi}

```gdscript
Vector3i clampi(min: int, max: int) const
```

Returns a new vector with all components clamped between min and max, by running @GlobalScope.clamp() on each component.

### distance_squared_to {#distance_squared_to}

```gdscript
int distance_squared_to(to: Vector3i) const
```

Returns the squared Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

This method runs faster than distance_to(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### distance_to {#distance_to}

```gdscript
float distance_to(to: Vector3i) const
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
Vector3i max(with: Vector3i) const
```

Returns the component-wise maximum of this and with, equivalent to Vector3i(maxi(x, with.x), maxi(y, with.y), maxi(z, with.z)).

### max_axis_index {#max_axis_index}

```gdscript
int max_axis_index() const
```

Returns the axis of the vector's highest value. See AXIS_* constants. If all components are equal, this method returns AXIS_X.

### maxi {#maxi}

```gdscript
Vector3i maxi(with: int) const
```

Returns the component-wise maximum of this and with, equivalent to Vector3i(maxi(x, with), maxi(y, with), maxi(z, with)).

### min {#min}

```gdscript
Vector3i min(with: Vector3i) const
```

Returns the component-wise minimum of this and with, equivalent to Vector3i(mini(x, with.x), mini(y, with.y), mini(z, with.z)).

### min_axis_index {#min_axis_index}

```gdscript
int min_axis_index() const
```

Returns the axis of the vector's lowest value. See AXIS_* constants. If all components are equal, this method returns AXIS_Z.

### mini {#mini}

```gdscript
Vector3i mini(with: int) const
```

Returns the component-wise minimum of this and with, equivalent to Vector3i(mini(x, with), mini(y, with), mini(z, with)).

### sign {#sign}

```gdscript
Vector3i sign() const
```

Returns a new vector with each component set to 1 if it's positive, -1 if it's negative, and 0 if it's zero. The result is identical to calling @GlobalScope.sign() on each component.

### snapped {#snapped}

```gdscript
Vector3i snapped(step: Vector3i) const
```

Returns a new vector with each component snapped to the closest multiple of the corresponding component in step.

### snappedi {#snappedi}

```gdscript
Vector3i snappedi(step: int) const
```

Returns a new vector with each component snapped to the closest multiple of step.

## 运算符

```gdscript
operator !=(right: Vector3i)
```

Returns true if the vectors are not equal.

```gdscript
operator %(right: Vector3i)
```

Gets the remainder of each component of the Vector3i with the components of the given Vector3i. This operation uses truncated division, which is often not desired as it does not work well with negative numbers. Consider using @GlobalScope.posmod() instead if you want to handle negative numbers.

```gdscript
print(Vector3i(10, -20, 30) % Vector3i(7, 8, 9)) # Prints (3, -4, 3)
```

```gdscript
operator %(right: int)
```

Gets the remainder of each component of the Vector3i with the given int. This operation uses truncated division, which is often not desired as it does not work well with negative numbers. Consider using @GlobalScope.posmod() instead if you want to handle negative numbers.

```gdscript
print(Vector3i(10, -20, 30) % 7) # Prints (3, -6, 2)
```

```gdscript
operator *(right: Vector3i)
```

Multiplies each component of the Vector3i by the components of the given Vector3i.

```gdscript
print(Vector3i(10, 20, 30) * Vector3i(3, 4, 5)) # Prints (30, 80, 150)
```

```gdscript
operator *(right: float)
```

Multiplies each component of the Vector3i by the given float. Returns a Vector3.

```gdscript
print(Vector3i(10, 15, 20) * 0.9) # Prints (9.0, 13.5, 18.0)
```

```gdscript
operator *(right: int)
```

Multiplies each component of the Vector3i by the given int.

```gdscript
operator +(right: Vector3i)
```

Adds each component of the Vector3i by the components of the given Vector3i.

```gdscript
print(Vector3i(10, 20, 30) + Vector3i(3, 4, 5)) # Prints (13, 24, 35)
```

```gdscript
operator -(right: Vector3i)
```

Subtracts each component of the Vector3i by the components of the given Vector3i.

```gdscript
print(Vector3i(10, 20, 30) - Vector3i(3, 4, 5)) # Prints (7, 16, 25)
```

```gdscript
operator /(right: Vector3i)
```

Divides each component of the Vector3i by the components of the given Vector3i.

```gdscript
print(Vector3i(10, 20, 30) / Vector3i(2, 5, 3)) # Prints (5, 4, 10)
```

```gdscript
operator /(right: float)
```

Divides each component of the Vector3i by the given float. Returns a Vector3.

```gdscript
print(Vector3i(1, 2, 3) / 2.5) # Prints (0.4, 0.8, 1.2)
```

```gdscript
operator /(right: int)
```

Divides each component of the Vector3i by the given int.

```gdscript
operator <(right: Vector3i)
```

Compares two Vector3i vectors by first checking if the X value of the left vector is less than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, and then with the Z values. This operator is useful for sorting vectors.

```gdscript
operator <=(right: Vector3i)
```

Compares two Vector3i vectors by first checking if the X value of the left vector is less than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, and then with the Z values. This operator is useful for sorting vectors.

```gdscript
operator ==(right: Vector3i)
```

Returns true if the vectors are equal.

```gdscript
operator >(right: Vector3i)
```

Compares two Vector3i vectors by first checking if the X value of the left vector is greater than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, and then with the Z values. This operator is useful for sorting vectors.

```gdscript
operator >=(right: Vector3i)
```

Compares two Vector3i vectors by first checking if the X value of the left vector is greater than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, and then with the Z values. This operator is useful for sorting vectors.

```gdscript
operator [](index: int)
```

Access vector components using their index. v[0] is equivalent to v.x, v[1] is equivalent to v.y, and v[2] is equivalent to v.z.

```gdscript
operator unary+()
```

Returns the same value as if the + was not there. Unary + does nothing, but sometimes it can make your code more readable.

```gdscript
operator unary-()
```

Returns the negative value of the Vector3i. This is the same as writing Vector3i(-v.x, -v.y, -v.z). This operation flips the direction of the vector while keeping the same magnitude.

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

### z {#z}

```gdscript
z: int
```

Z 分量（整数）。

## 枚举

### Axis {#enum-Axis}

```gdscript
enum Axis { AXIS_X = 0, AXIS_Y = 1, AXIS_Z = 2 }
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

#### AXIS_Z {#AXIS_Z}

```gdscript
AXIS_Z = 2
```

Enumerated value for the Z axis. Returned by max_axis_index() and min_axis_index().

---

## 常量

### ZERO {#ZERO}

```gdscript
const ZERO = Vector3i(0, 0, 0)
```

整数零向量。

### ONE {#ONE}

```gdscript
const ONE = Vector3i(1, 1, 1)
```

整数单位向量。

### MIN {#MIN}

```gdscript
const MIN = Vector3i(-2147483648, -2147483648, -2147483648)
```

各分量取整数最小。

### MAX {#MAX}

```gdscript
const MAX = Vector3i(2147483647, 2147483647, 2147483647)
```

各分量取整数最大。

### LEFT {#LEFT}

```gdscript
const LEFT = Vector3i(-1, 0, 0)
```

整数左向 (-1, 0, 0)。

### RIGHT {#RIGHT}

```gdscript
const RIGHT = Vector3i(1, 0, 0)
```

整数右向 (1, 0, 0)。

### UP {#UP}

```gdscript
const UP = Vector3i(0, 1, 0)
```

整数上向 (0, 1, 0)。

### DOWN {#DOWN}

```gdscript
const DOWN = Vector3i(0, -1, 0)
```

整数下向 (0, -1, 0)。

### FORWARD {#FORWARD}

```gdscript
const FORWARD = Vector3i(0, 0, -1)
```

整数前向 (0, 0, -1)。

### BACK {#BACK}

```gdscript
const BACK = Vector3i(0, 0, 1)
```

整数后向 (0, 0, 1)。
