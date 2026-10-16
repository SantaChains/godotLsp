# Vector2

> A 2-element structure that can be used to represent 2D coordinates or any other pair of numeric values.

<details><summary>官方英文描述（中文精译进行中）</summary>

A 2-element structure that can be used to represent 2D coordinates or any other pair of numeric values.

It uses floating-point coordinates. By default, these floating-point values use 32-bit precision, unlike float which is always 64-bit. If double precision is needed, compile the engine with the option precision=double.

See Vector2i for its integer counterpart.

Note: In a boolean context, a Vector2 will evaluate to false if it's equal to Vector2(0, 0). Otherwise, a Vector2 will always evaluate to true.

</details>

## 方法

### abs {#abs}

```gdscript
Vector2 abs() const
```

Returns a new vector with all components in absolute values (i.e. positive).

### angle {#angle}

```gdscript
float angle() const
```

Returns this vector's angle with respect to the positive X axis, or (1, 0) vector, in radians.

For example, Vector2.RIGHT.angle() will return zero, Vector2.DOWN.angle() will return PI / 2 (a quarter turn, or 90 degrees), and Vector2(1, -1).angle() will return -PI / 4 (a negative eighth turn, or -45 degrees).

This is equivalent to calling @GlobalScope.atan2() with y and x.

Illustration of the returned angle. [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle.png]

### angle_to {#angle_to}

```gdscript
float angle_to(to: Vector2) const
```

Returns the signed angle to the given vector, in radians. The result ranges from -PI to PI (inclusive).

Illustration of the returned angle. [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle_to.png]

### angle_to_point {#angle_to_point}

```gdscript
float angle_to_point(to: Vector2) const
```

Returns the signed angle between the X axis and the line from this vector to point to, in radians. The result ranges from -PI to PI (inclusive).

a.angle_to_point(b) is equivalent to (b - a).angle(). See also angle().

Illustration of the returned angle. [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/vector2_angle_to_point.png]

### aspect {#aspect}

```gdscript
float aspect() const
```

Returns this vector's aspect ratio, which is x divided by y.

### bezier_derivative {#bezier_derivative}

```gdscript
Vector2 bezier_derivative(control_1: Vector2, control_2: Vector2, end: Vector2, t: float) const
```

Returns the derivative at the given t on the Bézier curve [https://en.wikipedia.org/wiki/B%C3%A9zier_curve] defined by this vector and the given control_1, control_2, and end points.

### bezier_interpolate {#bezier_interpolate}

```gdscript
Vector2 bezier_interpolate(control_1: Vector2, control_2: Vector2, end: Vector2, t: float) const
```

Returns the point at the given t on the Bézier curve [https://en.wikipedia.org/wiki/B%C3%A9zier_curve] defined by this vector and the given control_1, control_2, and end points.

### bounce {#bounce}

```gdscript
Vector2 bounce(n: Vector2) const
```

Returns the vector "bounced off" from a line defined by the given normal n perpendicular to the line.

Note: bounce() performs the operation that most engines and frameworks call reflect().

### ceil {#ceil}

```gdscript
Vector2 ceil() const
```

Returns a new vector with all components rounded up (towards positive infinity).

### clamp {#clamp}

```gdscript
Vector2 clamp(min: Vector2, max: Vector2) const
```

Returns a new vector with all components clamped between the components of min and max, by running @GlobalScope.clamp() on each component.

### clampf {#clampf}

```gdscript
Vector2 clampf(min: float, max: float) const
```

Returns a new vector with all components clamped between min and max, by running @GlobalScope.clamp() on each component.

### cross {#cross}

```gdscript
float cross(with: Vector2) const
```

Returns the 2D analog of the cross product for this vector and with.

This is the signed area of the parallelogram formed by the two vectors. If the second vector is clockwise from the first vector, then the cross product is the positive area. If counter-clockwise, the cross product is the negative area. If the two vectors are parallel this returns zero, making it useful for testing if two vectors are parallel.

Note: Cross product is not defined in 2D mathematically. This method embeds the 2D vectors in the XY plane of 3D space and uses their cross product's Z component as the analog.

### cubic_interpolate {#cubic_interpolate}

```gdscript
Vector2 cubic_interpolate(b: Vector2, pre_a: Vector2, post_b: Vector2, weight: float) const
```

Performs a cubic interpolation between this vector and b using pre_a and post_b as handles, and returns the result at position weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

### cubic_interpolate_in_time {#cubic_interpolate_in_time}

```gdscript
Vector2 cubic_interpolate_in_time(b: Vector2, pre_a: Vector2, post_b: Vector2, weight: float, b_t: float, pre_a_t: float, post_b_t: float) const
```

Performs a cubic interpolation between this vector and b using pre_a and post_b as handles, and returns the result at position weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

It can perform smoother interpolation than cubic_interpolate() by the time values.

### direction_to {#direction_to}

```gdscript
Vector2 direction_to(to: Vector2) const
```

Returns the normalized vector pointing from this vector to to.

a.direction_to(b) is equivalent to (b - a).normalized(). See also normalized().

### distance_squared_to {#distance_squared_to}

```gdscript
float distance_squared_to(to: Vector2) const
```

Returns the squared Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

This method runs faster than distance_to(), so prefer it if you need to compare vectors or need the squared distance for some formula.

### distance_to {#distance_to}

```gdscript
float distance_to(to: Vector2) const
```

Returns the Euclidean distance [https://en.wikipedia.org/wiki/Euclidean_distance] between this vector and to.

### dot {#dot}

```gdscript
float dot(with: Vector2) const
```

Returns the dot product of this vector and with. This can be used to compare the angle between two vectors. For example, this can be used to determine whether an enemy is facing the player.

The dot product will be 0 for a right angle (90 degrees), greater than 0 for angles narrower than 90 degrees and lower than 0 for angles wider than 90 degrees.

When using unit (normalized) vectors, the result will always be between -1.0 (180 degree angle) when the vectors are facing opposite directions, and 1.0 (0 degree angle) when the vectors are aligned.

Note: a.dot(b) is equivalent to b.dot(a).

### floor {#floor}

```gdscript
Vector2 floor() const
```

Returns a new vector with all components rounded down (towards negative infinity).

### from_angle {#from_angle}

```gdscript
static Vector2 from_angle(angle: float)
```

Creates a Vector2 rotated to the given angle in radians. This is equivalent to doing Vector2(cos(angle), sin(angle)) or Vector2.RIGHT.rotated(angle).

```gdscript
print(Vector2.from_angle(0)) # Prints (1.0, 0.0)
print(Vector2(1, 0).angle()) # Prints 0.0, which is the angle used above.
print(Vector2.from_angle(PI / 2)) # Prints (0.0, 1.0)
```

Note: The length of the returned Vector2 is approximately 1.0, but is is not guaranteed to be exactly 1.0 due to floating-point precision issues. Call normalized() on the returned Vector2 if you require a unit vector.

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(to: Vector2) const
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
Vector2 lerp(to: Vector2, weight: float) const
```

Returns the result of the linear interpolation between this vector and to by amount weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

### limit_length {#limit_length}

```gdscript
Vector2 limit_length(length: float = 1.0) const
```

Returns the vector with a maximum length by limiting its length to length. If the vector is non-finite, the result is undefined.

### max {#max}

```gdscript
Vector2 max(with: Vector2) const
```

Returns the component-wise maximum of this and with, equivalent to Vector2(maxf(x, with.x), maxf(y, with.y)).

### max_axis_index {#max_axis_index}

```gdscript
int max_axis_index() const
```

Returns the axis of the vector's highest value. See AXIS_* constants. If all components are equal, this method returns AXIS_X.

### maxf {#maxf}

```gdscript
Vector2 maxf(with: float) const
```

Returns the component-wise maximum of this and with, equivalent to Vector2(maxf(x, with), maxf(y, with)).

### min {#min}

```gdscript
Vector2 min(with: Vector2) const
```

Returns the component-wise minimum of this and with, equivalent to Vector2(minf(x, with.x), minf(y, with.y)).

### min_axis_index {#min_axis_index}

```gdscript
int min_axis_index() const
```

Returns the axis of the vector's lowest value. See AXIS_* constants. If all components are equal, this method returns AXIS_Y.

### minf {#minf}

```gdscript
Vector2 minf(with: float) const
```

Returns the component-wise minimum of this and with, equivalent to Vector2(minf(x, with), minf(y, with)).

### move_toward {#move_toward}

```gdscript
Vector2 move_toward(to: Vector2, delta: float) const
```

Returns a new vector moved toward to by the fixed delta amount. Will not go past the final value.

### normalized {#normalized}

```gdscript
Vector2 normalized() const
```

Returns the result of scaling the vector to unit length. Equivalent to v / v.length(). Returns (0, 0) if v.length() == 0. See also is_normalized().

Note: This function may return incorrect values if the input vector length is near zero.

### orthogonal {#orthogonal}

```gdscript
Vector2 orthogonal() const
```

Returns a perpendicular vector rotated 90 degrees counter-clockwise compared to the original, with the same length.

### posmod {#posmod}

```gdscript
Vector2 posmod(mod: float) const
```

Returns a vector composed of the @GlobalScope.fposmod() of this vector's components and mod.

### posmodv {#posmodv}

```gdscript
Vector2 posmodv(modv: Vector2) const
```

Returns a vector composed of the @GlobalScope.fposmod() of this vector's components and modv's components.

### project {#project}

```gdscript
Vector2 project(b: Vector2) const
```

Returns a new vector resulting from projecting this vector onto the given vector b. The resulting new vector is parallel to b. See also slide().

Note: If the vector b is a zero vector, the components of the resulting new vector will be @GDScript.NAN.

### reflect {#reflect}

```gdscript
Vector2 reflect(line: Vector2) const
```

Returns the result of reflecting the vector from a line defined by the given direction vector line.

Note: reflect() differs from what other engines and frameworks call reflect(). In other engines, reflect() takes a normal direction which is a direction perpendicular to the line. In Godot, you specify the direction of the line directly. See also bounce() which does what most engines call reflect().

### rotated {#rotated}

```gdscript
Vector2 rotated(angle: float) const
```

Returns the result of rotating this vector by angle (in radians). See also @GlobalScope.deg_to_rad().

### round {#round}

```gdscript
Vector2 round() const
```

Returns a new vector with all components rounded to the nearest integer, with halfway cases rounded away from zero.

### sign {#sign}

```gdscript
Vector2 sign() const
```

Returns a new vector with each component set to 1.0 if it's positive, -1.0 if it's negative, and 0.0 if it's zero. The result is identical to calling @GlobalScope.sign() on each component.

### slerp {#slerp}

```gdscript
Vector2 slerp(to: Vector2, weight: float) const
```

Returns the result of spherical linear interpolation between this vector and to, by amount weight. weight is on the range of 0.0 to 1.0, representing the amount of interpolation.

This method also handles interpolating the lengths if the input vectors have different lengths. For the special case of one or both input vectors having zero length, this method behaves like lerp().

### slide {#slide}

```gdscript
Vector2 slide(n: Vector2) const
```

Returns a new vector resulting from sliding this vector along a line with normal n. The resulting new vector is perpendicular to n, and is equivalent to this vector minus its projection on n. See also project().

Note: The vector n must be normalized. See also normalized().

### snapped {#snapped}

```gdscript
Vector2 snapped(step: Vector2) const
```

Returns a new vector with each component snapped to the nearest multiple of the corresponding component in step. This can also be used to round the components to an arbitrary number of decimals.

### snappedf {#snappedf}

```gdscript
Vector2 snappedf(step: float) const
```

Returns a new vector with each component snapped to the nearest multiple of step. This can also be used to round the components to an arbitrary number of decimals.

## 属性

### x {#x}

```gdscript
x: float
```

The vector's X component. Also accessible by using the index position [0].

### y {#y}

```gdscript
y: float
```

The vector's Y component. Also accessible by using the index position [1].

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
const ZERO = Vector2(0, 0)
```

Zero vector, a vector with all components set to 0.

ONE = Vector2(1, 1) 

One vector, a vector with all components set to 1.

INF = Vector2(inf, inf) 

Infinity vector, a vector with all components set to @GDScript.INF.

LEFT = Vector2(-1, 0) 

Left unit vector. Represents the direction of left.

RIGHT = Vector2(1, 0) 

Right unit vector. Represents the direction of right.

UP = Vector2(0, -1) 

Up unit vector. Y is down in 2D, so this vector points -Y.

DOWN = Vector2(0, 1) 

Down unit vector. Y is down in 2D, so this vector points +Y.

### ONE {#ONE}

```gdscript
const ONE = Vector2(1, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INF {#INF}

```gdscript
const INF = Vector2(inf, inf)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### LEFT {#LEFT}

```gdscript
const LEFT = Vector2(-1, 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### RIGHT {#RIGHT}

```gdscript
const RIGHT = Vector2(1, 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### UP {#UP}

```gdscript
const UP = Vector2(0, -1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DOWN {#DOWN}

```gdscript
const DOWN = Vector2(0, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
