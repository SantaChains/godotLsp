# Vector4i

> A 4-element structure that can be used to represent 4D grid coordinates or any other quadruplet of integers.

<details><summary>官方英文描述</summary>

A 4-element structure that can be used to represent 4D grid coordinates or any other quadruplet of integers.

It uses integer coordinates and is therefore preferable to Vector4 when exact precision is required. Note that the values are limited to 32 bits, and unlike Vector4 this cannot be configured with an engine build option. Use int or PackedInt64Array if 64-bit values are needed.

Note: In a boolean context, a Vector4i will evaluate to false if it's equal to Vector4i(0, 0, 0, 0). Otherwise, a Vector4i will always evaluate to true.

</details>

## 构造函数

```gdscript
Vector4i()
```

Constructs a default-initialized Vector4i with all components set to 0.

```gdscript
Vector4i(from: Vector4i)
```

Constructs a Vector4i as a copy of the given Vector4i.

```gdscript
Vector4i(from: Vector4)
```

Constructs a new Vector4i from the given Vector4 by truncating components' fractional parts (rounding towards zero). For a different behavior consider passing the result of Vector4.ceil(), Vector4.floor() or Vector4.round() to this constructor instead.

```gdscript
Vector4i(x: int, y: int, z: int, w: int)
```

Returns a Vector4i with the given components.

## 方法

### abs {#abs}

```gdscript
Vector4i abs() const
```

Returns a new vector with all components in absolute values (i.e. positive).

### clamp {#clamp}

```gdscript
Vector4i clamp(min: Vector4i, max: Vector4i) const
```

Returns a new vector with all components clamped between the components of min and max, by running @GlobalScope.clamp() on each component.

### clampi {#clampi}

```gdscript
Vector4i clampi(min: int, max: int) const
```

Returns a new vector with all components clamped between min and max, by running @GlobalScope.clamp() on each component.

### distance_squared_to {#distance_squared_to}

```gdscript
int distance_squared_to(to: Vector4i) const
```

Returns the squared Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

This method runs faster than distance_to(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### distance_to {#distance_to}

```gdscript
float distance_to(to: Vector4i) const
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
Vector4i max(with: Vector4i) const
```

Returns the component-wise maximum of this and with, equivalent to Vector4i(maxi(x, with.x), maxi(y, with.y), maxi(z, with.z), maxi(w, with.w)).

### max_axis_index {#max_axis_index}

```gdscript
int max_axis_index() const
```

Returns the axis of the vector's highest value. See AXIS_* constants. If all components are equal, this method returns AXIS_X.

### maxi {#maxi}

```gdscript
Vector4i maxi(with: int) const
```

Returns the component-wise maximum of this and with, equivalent to Vector4i(maxi(x, with), maxi(y, with), maxi(z, with), maxi(w, with)).

### min {#min}

```gdscript
Vector4i min(with: Vector4i) const
```

Returns the component-wise minimum of this and with, equivalent to Vector4i(mini(x, with.x), mini(y, with.y), mini(z, with.z), mini(w, with.w)).

### min_axis_index {#min_axis_index}

```gdscript
int min_axis_index() const
```

Returns the axis of the vector's lowest value. See AXIS_* constants. If all components are equal, this method returns AXIS_W.

### mini {#mini}

```gdscript
Vector4i mini(with: int) const
```

Returns the component-wise minimum of this and with, equivalent to Vector4i(mini(x, with), mini(y, with), mini(z, with), mini(w, with)).

### sign {#sign}

```gdscript
Vector4i sign() const
```

Returns a new vector with each component set to 1 if it's positive, -1 if it's negative, and 0 if it's zero. The result is identical to calling @GlobalScope.sign() on each component.

### snapped {#snapped}

```gdscript
Vector4i snapped(step: Vector4i) const
```

Returns a new vector with each component snapped to the closest multiple of the corresponding component in step.

### snappedi {#snappedi}

```gdscript
Vector4i snappedi(step: int) const
```

Returns a new vector with each component snapped to the closest multiple of step.

## 运算符

```gdscript
operator !=(right: Vector4i)
```

Returns true if the vectors are not equal.

```gdscript
operator %(right: Vector4i)
```

Gets the remainder of each component of the Vector4i with the components of the given Vector4i. This operation uses truncated division, which is often not desired as it does not work well with negative numbers. Consider using @GlobalScope.posmod() instead if you want to handle negative numbers.

```gdscript
print(Vector4i(10, -20, 30, -40) % Vector4i(7, 8, 9, 10)) # Prints (3, -4, 3, 0)
```

```gdscript
operator %(right: int)
```

Gets the remainder of each component of the Vector4i with the given int. This operation uses truncated division, which is often not desired as it does not work well with negative numbers. Consider using @GlobalScope.posmod() instead if you want to handle negative numbers.

```gdscript
print(Vector4i(10, -20, 30, -40) % 7) # Prints (3, -6, 2, -5)
```

```gdscript
operator *(right: Vector4i)
```

Multiplies each component of the Vector4i by the components of the given Vector4i.

```gdscript
print(Vector4i(10, 20, 30, 40) * Vector4i(3, 4, 5, 6)) # Prints (30, 80, 150, 240)
```

```gdscript
operator *(right: float)
```

Multiplies each component of the Vector4i by the given float.

Returns a Vector4 value due to floating-point operations.

```gdscript
print(Vector4i(10, 20, 30, 40) * 2) # Prints (20.0, 40.0, 60.0, 80.0)
```

```gdscript
operator *(right: int)
```

Multiplies each component of the Vector4i by the given int.

```gdscript
operator +(right: Vector4i)
```

Adds each component of the Vector4i by the components of the given Vector4i.

```gdscript
print(Vector4i(10, 20, 30, 40) + Vector4i(3, 4, 5, 6)) # Prints (13, 24, 35, 46)
```

```gdscript
operator -(right: Vector4i)
```

Subtracts each component of the Vector4i by the components of the given Vector4i.

```gdscript
print(Vector4i(10, 20, 30, 40) - Vector4i(3, 4, 5, 6)) # Prints (7, 16, 25, 34)
```

```gdscript
operator /(right: Vector4i)
```

Divides each component of the Vector4i by the components of the given Vector4i.

```gdscript
print(Vector4i(10, 20, 30, 40) / Vector4i(2, 5, 3, 4)) # Prints (5, 4, 10, 10)
```

```gdscript
operator /(right: float)
```

Divides each component of the Vector4i by the given float.

Returns a Vector4 value due to floating-point operations.

```gdscript
print(Vector4i(1, 2, 3, 4) / 2.5) # Prints (0.4, 0.8, 1.2, 1.6)
```

```gdscript
operator /(right: int)
```

Divides each component of the Vector4i by the given int.

```gdscript
operator <(right: Vector4i)
```

Compares two Vector4i vectors by first checking if the X value of the left vector is less than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

```gdscript
operator <=(right: Vector4i)
```

Compares two Vector4i vectors by first checking if the X value of the left vector is less than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

```gdscript
operator ==(right: Vector4i)
```

Returns true if the vectors are exactly equal.

```gdscript
operator >(right: Vector4i)
```

Compares two Vector4i vectors by first checking if the X value of the left vector is greater than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

```gdscript
operator >=(right: Vector4i)
```

Compares two Vector4i vectors by first checking if the X value of the left vector is greater than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

```gdscript
operator [](index: int)
```

Access vector components using their index. v[0] is equivalent to v.x, v[1] is equivalent to v.y, v[2] is equivalent to v.z, and v[3] is equivalent to v.w.

```gdscript
operator unary+()
```

Returns the same value as if the + was not there. Unary + does nothing, but sometimes it can make your code more readable.

```gdscript
operator unary-()
```

Returns the negative value of the Vector4i. This is the same as writing Vector4i(-v.x, -v.y, -v.z, -v.w). This operation flips the direction of the vector while keeping the same magnitude.

## 属性

### x {#x}

```gdscript
x: int
```

The vector's X component. Also accessible by using the index position [0].

### y {#y}

```gdscript
y: int
```

The vector's Y component. Also accessible by using the index position [1].

### z {#z}

```gdscript
z: int
```

The vector's Z component. Also accessible by using the index position [2].

### w {#w}

```gdscript
w: int
```

The vector's W component. Also accessible by using the index position [3].

## 枚举

### Axis {#enum-Axis}

```gdscript
enum Axis { AXIS_X = 0, AXIS_Y = 1, AXIS_Z = 2, AXIS_W = 3 }
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

#### AXIS_W {#AXIS_W}

```gdscript
AXIS_W = 3
```

Enumerated value for the W axis. Returned by max_axis_index() and min_axis_index().

---

## 常量

### ZERO {#ZERO}

```gdscript
const ZERO = Vector4i(0, 0, 0, 0)
```

Zero vector, a vector with all components set to 0.

ONE = Vector4i(1, 1, 1, 1) 

One vector, a vector with all components set to 1.

MIN = Vector4i(-2147483648, -2147483648, -2147483648, -2147483648) 

Min vector, a vector with all components equal to INT32_MIN. Can be used as a negative integer equivalent of Vector4.INF.

MAX = Vector4i(2147483647, 2147483647, 2147483647, 2147483647) 

Max vector, a vector with all components equal to INT32_MAX. Can be used as an integer equivalent of Vector4.INF.

### ONE {#ONE}

```gdscript
const ONE = Vector4i(1, 1, 1, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MIN {#MIN}

```gdscript
const MIN = Vector4i(-2147483648, -2147483648, -2147483648, -2147483648)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MAX {#MAX}

```gdscript
const MAX = Vector4i(2147483647, 2147483647, 2147483647, 2147483647)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
