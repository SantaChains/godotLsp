# Vector4

> A 4-element structure that can be used to represent 4D coordinates or any other quadruplet of numeric values.

<details><summary>官方英文描述</summary>

A 4-element structure that can be used to represent 4D coordinates or any other quadruplet of numeric values.

It uses floating-point coordinates. By default, these floating-point values use 32-bit precision, unlike float which is always 64-bit. If double precision is needed, compile the engine with the option precision=double.

See Vector4i for its integer counterpart.

Note: In a boolean context, a Vector4 will evaluate to false if it's equal to Vector4(0, 0, 0, 0). Otherwise, a Vector4 will always evaluate to true.

</details>

## 构造函数

```gdscript
Vector4()
```

Constructs a default-initialized Vector4 with all components set to 0.

```gdscript
Vector4(from: Vector4)
```

Constructs a Vector4 as a copy of the given Vector4.

```gdscript
Vector4(from: Vector4i)
```

Constructs a new Vector4 from the given Vector4i.

```gdscript
Vector4(x: float, y: float, z: float, w: float)
```

Returns a Vector4 with the given components.

## 方法

### abs {#abs}

```gdscript
Vector4 abs() const
```

Returns a new vector with all components in absolute values (i.e. positive).

### ceil {#ceil}

```gdscript
Vector4 ceil() const
```

Returns a new vector with all components rounded up (towards positive infinity).

### clamp {#clamp}

```gdscript
Vector4 clamp(min: Vector4, max: Vector4) const
```

Returns a new vector with all components clamped between the components of min and max, by running @GlobalScope.clamp() on each component.

### clampf {#clampf}

```gdscript
Vector4 clampf(min: float, max: float) const
```

Returns a new vector with all components clamped between min and max, by running @GlobalScope.clamp() on each component.

### cubic_interpolate {#cubic_interpolate}

```gdscript
Vector4 cubic_interpolate(b: Vector4, pre_a: Vector4, post_b: Vector4, weight: float) const
```

Performs a cubic interpolation between this vector and b using pre_a and post_b as handles, and returns the result at position weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

### cubic_interpolate_in_time {#cubic_interpolate_in_time}

```gdscript
Vector4 cubic_interpolate_in_time(b: Vector4, pre_a: Vector4, post_b: Vector4, weight: float, b_t: float, pre_a_t: float, post_b_t: float) const
```

Performs a cubic interpolation between this vector and b using pre_a and post_b as handles, and returns the result at position weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

It can perform smoother interpolation than cubic_interpolate() by the time values.

### direction_to {#direction_to}

```gdscript
Vector4 direction_to(to: Vector4) const
```

Returns the normalized vector pointing from this vector to to. This is equivalent to using (b - a).normalized().

### distance_squared_to {#distance_squared_to}

```gdscript
float distance_squared_to(to: Vector4) const
```

Returns the squared Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

This method runs faster than distance_to(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### distance_to {#distance_to}

```gdscript
float distance_to(to: Vector4) const
```

Returns the Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

### dot {#dot}

```gdscript
float dot(with: Vector4) const
```

Returns the dot product of this vector and with.

### floor {#floor}

```gdscript
Vector4 floor() const
```

Returns a new vector with all components rounded down (towards negative infinity).

### inverse {#inverse}

```gdscript
Vector4 inverse() const
```

Returns the inverse of the vector. This is the same as Vector4(1.0 / v.x, 1.0 / v.y, 1.0 / v.z, 1.0 / v.w).

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(to: Vector4) const
```

Returns true if this vector and to are approximately equal, by running @GlobalScope.is_equal_approx() on each component.

### is_finite {#is_finite}

```gdscript
bool is_finite() const
```

Returns true if this vector is finite, by calling @GlobalScope.is_finite() on each component.

### is_normalized {#is_normalized}

```gdscript
bool is_normalized() const
```

Returns true if the vector is normalized, i.e. its length is approximately equal to 1.

### is_zero_approx {#is_zero_approx}

```gdscript
bool is_zero_approx() const
```

Returns true if this vector's values are approximately zero, by running @GlobalScope.is_zero_approx() on each component.

This method is faster than using is_equal_approx() with one value as a zero vector.

### length {#length}

```gdscript
float length() const
```

Returns the length (magnitude) of this vector.

### length_squared {#length_squared}

```gdscript
float length_squared() const
```

Returns the squared length (squared magnitude) of this vector.

This method runs faster than length(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### lerp {#lerp}

```gdscript
Vector4 lerp(to: Vector4, weight: float) const
```

Returns the result of the linear interpolation between this vector and to by amount weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

### max {#max}

```gdscript
Vector4 max(with: Vector4) const
```

Returns the component-wise maximum of this and with, equivalent to Vector4(maxf(x, with.x), maxf(y, with.y), maxf(z, with.z), maxf(w, with.w)).

### max_axis_index {#max_axis_index}

```gdscript
int max_axis_index() const
```

Returns the axis of the vector's highest value. See AXIS_* constants. If all components are equal, this method returns AXIS_X.

### maxf {#maxf}

```gdscript
Vector4 maxf(with: float) const
```

Returns the component-wise maximum of this and with, equivalent to Vector4(maxf(x, with), maxf(y, with), maxf(z, with), maxf(w, with)).

### min {#min}

```gdscript
Vector4 min(with: Vector4) const
```

Returns the component-wise minimum of this and with, equivalent to Vector4(minf(x, with.x), minf(y, with.y), minf(z, with.z), minf(w, with.w)).

### min_axis_index {#min_axis_index}

```gdscript
int min_axis_index() const
```

Returns the axis of the vector's lowest value. See AXIS_* constants. If all components are equal, this method returns AXIS_W.

### minf {#minf}

```gdscript
Vector4 minf(with: float) const
```

Returns the component-wise minimum of this and with, equivalent to Vector4(minf(x, with), minf(y, with), minf(z, with), minf(w, with)).

### normalized {#normalized}

```gdscript
Vector4 normalized() const
```

Returns the result of scaling the vector to unit length. Equivalent to v / v.length(). Returns (0, 0, 0, 0) if v.length() == 0. See also is_normalized().

Note: This function may return incorrect values if the input vector length is near zero.

### posmod {#posmod}

```gdscript
Vector4 posmod(mod: float) const
```

Returns a vector composed of the @GlobalScope.fposmod() of this vector's components and mod.

### posmodv {#posmodv}

```gdscript
Vector4 posmodv(modv: Vector4) const
```

Returns a vector composed of the @GlobalScope.fposmod() of this vector's components and modv's components.

### round {#round}

```gdscript
Vector4 round() const
```

Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.

### sign {#sign}

```gdscript
Vector4 sign() const
```

Returns a new vector with each component set to 1.0 if it's positive, -1.0 if it's negative, and 0.0 if it's zero. The result is identical to calling @GlobalScope.sign() on each component.

### snapped {#snapped}

```gdscript
Vector4 snapped(step: Vector4) const
```

Returns a new vector with each component snapped to the nearest multiple of the corresponding component in step. This can also be used to round the components to an arbitrary number of decimals.

### snappedf {#snappedf}

```gdscript
Vector4 snappedf(step: float) const
```

Returns a new vector with each component snapped to the nearest multiple of step. This can also be used to round the components to an arbitrary number of decimals.

## 运算符

```gdscript
operator !=(right: Vector4)
```

Returns true if the vectors are not equal.

Note: Due to floating-point precision errors, consider using is_equal_approx() instead, which is more reliable.

Note: Vectors with @GDScript.NAN elements don't behave the same as other vectors. Therefore, the results from this operator may not be accurate if NaNs are included.

```gdscript
operator *(right: Projection)
```

Transforms (multiplies) the Vector4 by the transpose of the given Projection matrix.

For transforming by inverse of a projection projection.inverse() * vector can be used instead. See Projection.inverse().

```gdscript
operator *(right: Vector4)
```

Multiplies each component of the Vector4 by the components of the given Vector4.

```gdscript
print(Vector4(10, 20, 30, 40) * Vector4(3, 4, 5, 6)) # Prints (30.0, 80.0, 150.0, 240.0)
```

```gdscript
operator *(right: float)
```

Multiplies each component of the Vector4 by the given float.

```gdscript
print(Vector4(10, 20, 30, 40) * 2) # Prints (20.0, 40.0, 60.0, 80.0)
```

```gdscript
operator *(right: int)
```

Multiplies each component of the Vector4 by the given int.

```gdscript
operator +(right: Vector4)
```

Adds each component of the Vector4 by the components of the given Vector4.

```gdscript
print(Vector4(10, 20, 30, 40) + Vector4(3, 4, 5, 6)) # Prints (13.0, 24.0, 35.0, 46.0)
```

```gdscript
operator -(right: Vector4)
```

Subtracts each component of the Vector4 by the components of the given Vector4.

```gdscript
print(Vector4(10, 20, 30, 40) - Vector4(3, 4, 5, 6)) # Prints (7.0, 16.0, 25.0, 34.0)
```

```gdscript
operator /(right: Vector4)
```

Divides each component of the Vector4 by the components of the given Vector4.

```gdscript
print(Vector4(10, 20, 30, 40) / Vector4(2, 5, 3, 4)) # Prints (5.0, 4.0, 10.0, 10.0)
```

```gdscript
operator /(right: float)
```

Divides each component of the Vector4 by the given float.

```gdscript
print(Vector4(10, 20, 30, 40) / 2) # Prints (5.0, 10.0, 15.0, 20.0)
```

```gdscript
operator /(right: int)
```

Divides each component of the Vector4 by the given int.

```gdscript
operator <(right: Vector4)
```

Compares two Vector4 vectors by first checking if the X value of the left vector is less than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

Note: Vectors with @GDScript.NAN elements don't behave the same as other vectors. Therefore, the results from this operator may not be accurate if NaNs are included.

```gdscript
operator <=(right: Vector4)
```

Compares two Vector4 vectors by first checking if the X value of the left vector is less than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

Note: Vectors with @GDScript.NAN elements don't behave the same as other vectors. Therefore, the results from this operator may not be accurate if NaNs are included.

```gdscript
operator ==(right: Vector4)
```

Returns true if the vectors are exactly equal.

Note: Due to floating-point precision errors, consider using is_equal_approx() instead, which is more reliable.

Note: Vectors with @GDScript.NAN elements don't behave the same as other vectors. Therefore, the results from this operator may not be accurate if NaNs are included.

```gdscript
operator >(right: Vector4)
```

Compares two Vector4 vectors by first checking if the X value of the left vector is greater than the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

Note: Vectors with @GDScript.NAN elements don't behave the same as other vectors. Therefore, the results from this operator may not be accurate if NaNs are included.

```gdscript
operator >=(right: Vector4)
```

Compares two Vector4 vectors by first checking if the X value of the left vector is greater than or equal to the X value of the right vector. If the X values are exactly equal, then it repeats this check with the Y values of the two vectors, Z values of the two vectors, and then with the W values. This operator is useful for sorting vectors.

Note: Vectors with @GDScript.NAN elements don't behave the same as other vectors. Therefore, the results from this operator may not be accurate if NaNs are included.

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

Returns the negative value of the Vector4. This is the same as writing Vector4(-v.x, -v.y, -v.z, -v.w). This operation flips the direction of the vector while keeping the same magnitude. With floats, the number zero can be either positive or negative.

## 属性

### x {#x}

```gdscript
x: float
```

X 分量。

### y {#y}

```gdscript
y: float
```

Y 分量。

### z {#z}

```gdscript
z: float
```

Z 分量。

### w {#w}

```gdscript
w: float
```

W 分量（四维向量/四元式的第四分量）。

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
const ZERO = Vector4(0, 0, 0, 0)
```

四维零向量。

### ONE {#ONE}

```gdscript
const ONE = Vector4(1, 1, 1, 1)
```

四维单位向量。

### INF {#INF}

```gdscript
const INF = Vector4(inf, inf, inf, inf)
```

各分量为无穷大。
