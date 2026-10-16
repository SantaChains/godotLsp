# Vector3

> 三维向量。x、y、z 分量，支持向量运算、法线、交叉积、距离。

<details><summary>官方英文描述</summary>

A 3-element structure that can be used to represent 3D coordinates or any other triplet of numeric values.

It uses floating-point coordinates. By default, these floating-point values use 32-bit precision, unlike float which is always 64-bit. If double precision is needed, compile the engine with the option precision=double.

See Vector3i for its integer counterpart.

Note: In a boolean context, a Vector3 will evaluate to false if it's equal to Vector3(0, 0, 0). Otherwise, a Vector3 will always evaluate to true.

</details>

## 方法

### abs {#abs}

```gdscript
Vector3 abs() const
```

Returns a new vector with all components in absolute values (i.e. positive).

### angle_to {#angle_to}

```gdscript
float angle_to(to: Vector3) const
```

Returns the unsigned minimum angle to the given vector, in radians.

### bezier_derivative {#bezier_derivative}

```gdscript
Vector3 bezier_derivative(control_1: Vector3, control_2: Vector3, end: Vector3, t: float) const
```

Returns the derivative at the given t on the Bézier curve [https://en.wikipedia.org/wiki/B%C3%A9zier_curve] defined by this vector and the given control_1, control_2, and end points.

### bezier_interpolate {#bezier_interpolate}

```gdscript
Vector3 bezier_interpolate(control_1: Vector3, control_2: Vector3, end: Vector3, t: float) const
```

Returns the point at the given t on the Bézier curve [https://en.wikipedia.org/wiki/B%C3%A9zier_curve] defined by this vector and the given control_1, control_2, and end points.

### bounce {#bounce}

```gdscript
Vector3 bounce(n: Vector3) const
```

Returns the vector "bounced off" from a plane defined by the given normal n.

Note: bounce() performs the operation that most engines and frameworks call reflect().

### ceil {#ceil}

```gdscript
Vector3 ceil() const
```

Returns a new vector with all components rounded up (towards positive infinity).

### clamp {#clamp}

```gdscript
Vector3 clamp(min: Vector3, max: Vector3) const
```

Returns a new vector with all components clamped between the components of min and max, by running @GlobalScope.clamp() on each component.

### clampf {#clampf}

```gdscript
Vector3 clampf(min: float, max: float) const
```

Returns a new vector with all components clamped between min and max, by running @GlobalScope.clamp() on each component.

### cross {#cross}

```gdscript
Vector3 cross(with: Vector3) const
```

Returns the cross product of this vector and with.

This returns a vector perpendicular to both this and with, which would be the normal vector of the plane defined by the two vectors. As there are two such vectors, in opposite directions, this method returns the vector defined by a right-handed coordinate system. If the two vectors are parallel this returns an empty vector, making it useful for testing if two vectors are parallel.

### cubic_interpolate {#cubic_interpolate}

```gdscript
Vector3 cubic_interpolate(b: Vector3, pre_a: Vector3, post_b: Vector3, weight: float) const
```

Performs a cubic interpolation between this vector and b using pre_a and post_b as handles, and returns the result at position weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

### cubic_interpolate_in_time {#cubic_interpolate_in_time}

```gdscript
Vector3 cubic_interpolate_in_time(b: Vector3, pre_a: Vector3, post_b: Vector3, weight: float, b_t: float, pre_a_t: float, post_b_t: float) const
```

Performs a cubic interpolation between this vector and b using pre_a and post_b as handles, and returns the result at position weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

It can perform smoother interpolation than cubic_interpolate() by the time values.

### direction_to {#direction_to}

```gdscript
Vector3 direction_to(to: Vector3) const
```

Returns the normalized vector pointing from this vector to to. This is equivalent to using (b - a).normalized().

### distance_squared_to {#distance_squared_to}

```gdscript
float distance_squared_to(to: Vector3) const
```

Returns the squared Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

This method runs faster than distance_to(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### distance_to {#distance_to}

```gdscript
float distance_to(to: Vector3) const
```

Returns the Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

### dot {#dot}

```gdscript
float dot(with: Vector3) const
```

Returns the dot product of this vector and with. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.

The dot product will be 0 for a right angle (90 degrees), greater than 0 for angles narrower than 90 degrees and lower than 0 for angles wider than 90 degrees.

When using unit (normalized) vectors, the result will always be between -1.0 (180 degree angle) when the vectors are facing opposite directions, and 1.0 (0 degree angle) when the vectors are aligned.

Note: a.dot(b) is equivalent to b.dot(a).

### floor {#floor}

```gdscript
Vector3 floor() const
```

Returns a new vector with all components rounded down (towards negative infinity).

### inverse {#inverse}

```gdscript
Vector3 inverse() const
```

Returns the inverse of the vector. This is the same as Vector3(1.0 / v.x, 1.0 / v.y, 1.0 / v.z).

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(to: Vector3) const
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
Vector3 lerp(to: Vector3, weight: float) const
```

Returns the result of the linear interpolation between this vector and to by amount weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

### limit_length {#limit_length}

```gdscript
Vector3 limit_length(length: float = 1.0) const
```

Returns the vector with a maximum length by limiting its length to length. If the vector is non-finite, the result is undefined.

### max {#max}

```gdscript
Vector3 max(with: Vector3) const
```

Returns the component-wise maximum of this and with, equivalent to Vector3(maxf(x, with.x), maxf(y, with.y), maxf(z, with.z)).

### max_axis_index {#max_axis_index}

```gdscript
int max_axis_index() const
```

Returns the axis of the vector's highest value. See AXIS_* constants. If all components are equal, this method returns AXIS_X.

### maxf {#maxf}

```gdscript
Vector3 maxf(with: float) const
```

Returns the component-wise maximum of this and with, equivalent to Vector3(maxf(x, with), maxf(y, with), maxf(z, with)).

### min {#min}

```gdscript
Vector3 min(with: Vector3) const
```

Returns the component-wise minimum of this and with, equivalent to Vector3(minf(x, with.x), minf(y, with.y), minf(z, with.z)).

### min_axis_index {#min_axis_index}

```gdscript
int min_axis_index() const
```

Returns the axis of the vector's lowest value. See AXIS_* constants. If all components are equal, this method returns AXIS_Z.

### minf {#minf}

```gdscript
Vector3 minf(with: float) const
```

Returns the component-wise minimum of this and with, equivalent to Vector3(minf(x, with), minf(y, with), minf(z, with)).

### move_toward {#move_toward}

```gdscript
Vector3 move_toward(to: Vector3, delta: float) const
```

Returns a new vector moved toward to by the fixed delta amount. Will not go past the final value.

### normalized {#normalized}

```gdscript
Vector3 normalized() const
```

Returns the result of scaling the vector to unit length. Equivalent to v / v.length(). Returns (0, 0, 0) if v.length() == 0. See also is_normalized().

Note: This function may return incorrect values if the input vector length is near zero.

### octahedron_decode {#octahedron_decode}

```gdscript
static Vector3 octahedron_decode(uv: Vector2)
```

Returns the Vector3 from an octahedral-compressed form created using octahedron_encode() (stored as a Vector2).

### octahedron_encode {#octahedron_encode}

```gdscript
Vector2 octahedron_encode() const
```

Returns the octahedral-encoded (oct32) form of this Vector3 as a Vector2. Since a Vector2 occupies 1/3 less memory compared to Vector3, this form of compression can be used to pass greater amounts of normalized() Vector3s without increasing storage or memory requirements. See also octahedron_decode().

Note: octahedron_encode() can only be used for normalized() vectors. octahedron_encode() does not check whether this Vector3 is normalized, and will return a value that does not decompress to the original value if the Vector3 is not normalized.

Note: Octahedral compression is lossy, although visual differences are rarely perceptible in real world scenarios.

### outer {#outer}

```gdscript
Basis outer(with: Vector3) const
```

Returns the outer product with with.

### posmod {#posmod}

```gdscript
Vector3 posmod(mod: float) const
```

Returns a vector composed of the @GlobalScope.fposmod() of this vector's components and mod.

### posmodv {#posmodv}

```gdscript
Vector3 posmodv(modv: Vector3) const
```

Returns a vector composed of the @GlobalScope.fposmod() of this vector's components and modv's components.

### project {#project}

```gdscript
Vector3 project(b: Vector3) const
```

Returns a new vector resulting from projecting this vector onto the given vector b. The resulting new vector is parallel to b. See also slide().

Note: If the vector b is a zero vector, the components of the resulting new vector will be @GDScript.NAN.

### reflect {#reflect}

```gdscript
Vector3 reflect(n: Vector3) const
```

Returns the result of reflecting the vector through a plane defined by the given normal vector n.

Note: reflect() differs from what other engines and frameworks call reflect(). In other engines, reflect() returns the result of the vector reflected by the given plane. The reflection thus passes through the given normal. While in Godot the reflection passes through the plane and can be thought of as bouncing off the normal. See also bounce() which does what most engines call reflect().

### rotated {#rotated}

```gdscript
Vector3 rotated(axis: Vector3, angle: float) const
```

Returns the result of rotating this vector around a given axis by angle (in radians). The axis must be a normalized vector. See also @GlobalScope.deg_to_rad().

### round {#round}

```gdscript
Vector3 round() const
```

Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.

### sign {#sign}

```gdscript
Vector3 sign() const
```

Returns a new vector with each component set to 1.0 if it's positive, -1.0 if it's negative, and 0.0 if it's zero. The result is identical to calling @GlobalScope.sign() on each component.

### signed_angle_to {#signed_angle_to}

```gdscript
float signed_angle_to(to: Vector3, axis: Vector3) const
```

Returns the signed angle to the given vector, in radians. The sign of the angle is positive in a counter-clockwise direction and negative in a clockwise direction when viewed from the side specified by the axis.

### slerp {#slerp}

```gdscript
Vector3 slerp(to: Vector3, weight: float) const
```

Returns the result of spherical linear interpolation between this vector and to, by amount weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

This method also handles interpolating the lengths if the input vectors have different lengths. For the special case of one or both input vectors having zero length, this method behaves like lerp().

### slide {#slide}

```gdscript
Vector3 slide(n: Vector3) const
```

Returns a new vector resulting from sliding this vector along a plane with normal n. The resulting new vector is perpendicular to n, and is equivalent to this vector minus its projection on n. See also project().

Note: The vector n must be normalized. See also normalized().

### snapped {#snapped}

```gdscript
Vector3 snapped(step: Vector3) const
```

Returns a new vector with each component snapped to the nearest multiple of the corresponding component in step. This can also be used to round the components to an arbitrary number of decimals.

### snappedf {#snappedf}

```gdscript
Vector3 snappedf(step: float) const
```

Returns a new vector with each component snapped to the nearest multiple of step. This can also be used to round the components to an arbitrary number of decimals.

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
const ZERO = Vector3(0, 0, 0)
```

零向量 (0, 0, 0)。

### ONE {#ONE}

```gdscript
const ONE = Vector3(1, 1, 1)
```

单位向量 (1, 1, 1)。

### INF {#INF}

```gdscript
const INF = Vector3(inf, inf, inf)
```

各分量为无穷大的向量。

### LEFT {#LEFT}

```gdscript
const LEFT = Vector3(-1, 0, 0)
```

左向 (-1, 0, 0)。

### RIGHT {#RIGHT}

```gdscript
const RIGHT = Vector3(1, 0, 0)
```

右向 (1, 0, 0)。

### UP {#UP}

```gdscript
const UP = Vector3(0, 1, 0)
```

上向 (0, 1, 0)。

### DOWN {#DOWN}

```gdscript
const DOWN = Vector3(0, -1, 0)
```

下向 (0, -1, 0)。

### FORWARD {#FORWARD}

```gdscript
const FORWARD = Vector3(0, 0, -1)
```

前向 (0, 0, -1)。

### BACK {#BACK}

```gdscript
const BACK = Vector3(0, 0, 1)
```

后向 (0, 0, 1)。

### MODEL_LEFT {#MODEL_LEFT}

```gdscript
const MODEL_LEFT = Vector3(1, 0, 0)
```

模型空间左向。

### MODEL_RIGHT {#MODEL_RIGHT}

```gdscript
const MODEL_RIGHT = Vector3(-1, 0, 0)
```

模型空间右向。

### MODEL_TOP {#MODEL_TOP}

```gdscript
const MODEL_TOP = Vector3(0, 1, 0)
```

模型空间上向。

### MODEL_BOTTOM {#MODEL_BOTTOM}

```gdscript
const MODEL_BOTTOM = Vector3(0, -1, 0)
```

模型空间下向。

### MODEL_FRONT {#MODEL_FRONT}

```gdscript
const MODEL_FRONT = Vector3(0, 0, 1)
```

模型空间前向。

### MODEL_REAR {#MODEL_REAR}

```gdscript
const MODEL_REAR = Vector3(0, 0, -1)
```

模型空间后向。
