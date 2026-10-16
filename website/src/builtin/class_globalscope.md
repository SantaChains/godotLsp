# @GlobalScope

> A list of global scope enumerated constants and built-in functions. This is all that resides in the globals, constants regarding error codes, keycodes, property hints, etc.

## 工具函数

### sin {#sin}

```gdscript
float sin(angle_rad: float)
```

Returns the sine of angle angle_rad in radians.

```gdscript
sin(0.523599)       # Returns 0.5
sin(deg_to_rad(90)) # Returns 1.0
```

### cos {#cos}

```gdscript
float cos(angle_rad: float)
```

Returns the cosine of angle angle_rad in radians.

```gdscript
cos(PI * 2)         # Returns 1.0
cos(PI)             # Returns -1.0
cos(deg_to_rad(90)) # Returns 0.0
```

### tan {#tan}

```gdscript
float tan(angle_rad: float)
```

Returns the tangent of angle angle_rad in radians.

```gdscript
tan(deg_to_rad(45)) # Returns 1
```

### sinh {#sinh}

```gdscript
float sinh(x: float)
```

Returns the hyperbolic sine of x.

```gdscript
var a = log(2.0) # Returns 0.693147
sinh(a) # Returns 0.75
```

### cosh {#cosh}

```gdscript
float cosh(x: float)
```

Returns the hyperbolic cosine of x in radians.

```gdscript
print(cosh(1)) # Prints 1.543081
```

### tanh {#tanh}

```gdscript
float tanh(x: float)
```

Returns the hyperbolic tangent of x.

```gdscript
var a = log(2.0) # Returns 0.693147
tanh(a)          # Returns 0.6
```

### asin {#asin}

```gdscript
float asin(x: float)
```

Returns the arc sine of x in radians. Use to get the angle of sine x. x will be clamped between -1.0 and 1.0 (inclusive), in order to prevent asin() from returning @GDScript.NAN.

```gdscript
# s is 0.523599 or 30 degrees if converted with rad_to_deg(s)
var s = asin(0.5)
```

### acos {#acos}

```gdscript
float acos(x: float)
```

Returns the arc cosine of x in radians. Use to get the angle of cosine x. x will be clamped between -1.0 and 1.0 (inclusive), in order to prevent acos() from returning @GDScript.NAN.

```gdscript
# c is 0.523599 or 30 degrees if converted with rad_to_deg(c)
var c = acos(0.866025)
```

### atan {#atan}

```gdscript
float atan(x: float)
```

Returns the arc tangent of x in radians. Use it to get the angle from an angle's tangent in trigonometry.

The method cannot know in which quadrant the angle should fall. See atan2() if you have both y and x.

```gdscript
var a = atan(0.5) # a is 0.463648
```

If x is between -PI / 2 and PI / 2 (inclusive), atan(tan(x)) is equal to x.

### atan2 {#atan2}

```gdscript
float atan2(y: float, x: float)
```

Returns the arc tangent of y/x in radians. Use to get the angle of tangent y/x. To compute the value, the method takes into account the sign of both arguments in order to determine the quadrant.

Important note: The Y coordinate comes first, by convention.

```gdscript
var a = atan2(0, -1) # a is 3.141593
```

### asinh {#asinh}

```gdscript
float asinh(x: float)
```

Returns the hyperbolic arc (also called inverse) sine of x, returning a value in radians. Use it to get the angle from an angle's sine in hyperbolic space.

```gdscript
var a = asinh(0.9) # Returns 0.8088669356527824
sinh(a) # Returns 0.9
```

### acosh {#acosh}

```gdscript
float acosh(x: float)
```

Returns the hyperbolic arc (also called inverse) cosine of x, returning a value in radians. Use it to get the angle from an angle's cosine in hyperbolic space if x is larger or equal to 1. For values of x lower than 1, it will return 0, in order to prevent acosh() from returning @GDScript.NAN.

```gdscript
var a = acosh(2) # Returns 1.31695789692482
cosh(a) # Returns 2
var b = acosh(-1) # Returns 0
```

### atanh {#atanh}

```gdscript
float atanh(x: float)
```

Returns the hyperbolic arc (also called inverse) tangent of x, returning a value in radians. Use it to get the angle from an angle's tangent in hyperbolic space if x is between -1 and 1 (non-inclusive).

In mathematics, the inverse hyperbolic tangent is only defined for -1 < x < 1 in the real set, so values equal or lower to -1 for x return negative @GDScript.INF and values equal or higher than 1 return positive @GDScript.INF in order to prevent atanh() from returning @GDScript.NAN.

```gdscript
var a = atanh(0.9) # Returns 1.47221948958322
tanh(a) # Returns 0.9
var b = atanh(-2) # Returns -inf
tanh(b) # Returns -1
```

### sqrt {#sqrt}

```gdscript
float sqrt(x: float)
```

Returns the square root of x, where x is a non-negative number.

```gdscript
sqrt(9)     # Returns 3
sqrt(10.24) # Returns 3.2
sqrt(-1)    # Returns NaN
```

Note: Negative values of x return NaN ("Not a Number"). In C#, if you need negative inputs, use System.Numerics.Complex.

### fmod {#fmod}

```gdscript
float fmod(x: float, y: float)
```

Returns the floating-point remainder of x divided by y, keeping the sign of x.

```gdscript
var remainder = fmod(7, 5.5) # remainder is 1.5
```

For the integer remainder operation, use the % operator.

### fposmod {#fposmod}

```gdscript
float fposmod(x: float, y: float)
```

Returns the floating-point modulus of x divided by y, wrapping equally in positive and negative.

```gdscript
print(" (x)  (fmod(x, 1.5))   (fposmod(x, 1.5))")
for i in 7:
    var x = i * 0.5 - 1.5
    print("%4.1f           %4.1f  | %4.1f" % [x, fmod(x, 1.5), fposmod(x, 1.5)])
```

Prints:

```
 (x)  (fmod(x, 1.5))   (fposmod(x, 1.5))
-1.5           -0.0  |  0.0
-1.0           -1.0  |  0.5
-0.5           -0.5  |  1.0
 0.0            0.0  |  0.0
 0.5            0.5  |  0.5
 1.0            1.0  |  1.0
 1.5            0.0  |  0.0
```

### posmod {#posmod}

```gdscript
int posmod(x: int, y: int)
```

Returns the integer modulus of x divided by y that wraps equally in positive and negative.

```gdscript
print("#(i)  (i % 3)   (posmod(i, 3))")
for i in range(-3, 4):
    print("%2d       %2d  | %2d" % [i, i % 3, posmod(i, 3)])
```

Prints:

```
(i)  (i % 3)   (posmod(i, 3))
-3        0  |  0
-2       -2  |  1
-1       -1  |  2
 0        0  |  0
 1        1  |  1
 2        2  |  2
 3        0  |  0
```

### floor {#floor}

```gdscript
Variant floor(x: Variant)
```

Rounds x downward (towards negative infinity), returning the largest whole number that is not more than x. Supported types: int, float, Vector2, Vector2i, Vector3, Vector3i, Vector4, Vector4i.

```gdscript
var a = floor(2.99) # a is 2.0
a = floor(-2.99)    # a is -3.0
```

See also ceil(), round(), and snapped().

Note: For better type safety, use floorf(), floori(), Vector2.floor(), Vector3.floor(), or Vector4.floor().

### floorf {#floorf}

```gdscript
float floorf(x: float)
```

Rounds x downward (towards negative infinity), returning the largest whole number that is not more than x.

A type-safe version of floor(), returning a float.

### floori {#floori}

```gdscript
int floori(x: float)
```

Rounds x downward (towards negative infinity), returning the largest whole number that is not more than x.

A type-safe version of floor(), returning an int.

Note: This function is not the same as int(x), which rounds towards 0.

### ceil {#ceil}

```gdscript
Variant ceil(x: Variant)
```

Rounds x upward (towards positive infinity), returning the smallest whole number that is not less than x. Supported types: int, float, Vector2, Vector2i, Vector3, Vector3i, Vector4, Vector4i.

```gdscript
var i = ceil(1.45) # i is 2.0
i = ceil(1.001)    # i is 2.0
```

See also floor(), round(), and snapped().

Note: For better type safety, use ceilf(), ceili(), Vector2.ceil(), Vector3.ceil(), or Vector4.ceil().

### ceilf {#ceilf}

```gdscript
float ceilf(x: float)
```

Rounds x upward (towards positive infinity), returning the smallest whole number that is not less than x.

A type-safe version of ceil(), returning a float.

### ceili {#ceili}

```gdscript
int ceili(x: float)
```

Rounds x upward (towards positive infinity), returning the smallest whole number that is not less than x.

A type-safe version of ceil(), returning an int.

### round {#round}

```gdscript
Variant round(x: Variant)
```

Rounds x to the nearest whole number, with halfway cases rounded away from 0. Supported types: int, float, Vector2, Vector2i, Vector3, Vector3i, Vector4, Vector4i.

```gdscript
round(2.4) # Returns 2
round(2.5) # Returns 3
round(2.6) # Returns 3
```

See also floor(), ceil(), and snapped().

Note: For better type safety, use roundf(), roundi(), Vector2.round(), Vector3.round(), or Vector4.round().

### roundf {#roundf}

```gdscript
float roundf(x: float)
```

Rounds x to the nearest whole number, with halfway cases rounded away from 0.

A type-safe version of round(), returning a float.

### roundi {#roundi}

```gdscript
int roundi(x: float)
```

Rounds x to the nearest whole number, with halfway cases rounded away from 0.

A type-safe version of round(), returning an int.

### abs {#abs}

```gdscript
Variant abs(x: Variant)
```

Returns the absolute value of a Variant parameter x (i.e. non-negative value). Supported types: int, float, Vector2, Vector2i, Vector3, Vector3i, Vector4, Vector4i.

```gdscript
var a = abs(-1)
# a is 1
var b = abs(-1.2)
# b is 1.2
var c = abs(Vector2(-3.5, -4))
# c is (3.5, 4)
var d = abs(Vector2i(-5, -6))
# d is (5, 6)
var e = abs(Vector3(-7, 8.5, -3.8))
# e is (7, 8.5, 3.8)
var f = abs(Vector3i(-7, -8, -9))
# f is (7, 8, 9)
```

Note: For better type safety, use absf(), absi(), Vector2.abs(), Vector2i.abs(), Vector3.abs(), Vector3i.abs(), Vector4.abs(), or Vector4i.abs().

### absf {#absf}

```gdscript
float absf(x: float)
```

Returns the absolute value of float parameter x (i.e. positive value).

```gdscript
# a is 1.2
var a = absf(-1.2)
```

### absi {#absi}

```gdscript
int absi(x: int)
```

Returns the absolute value of int parameter x (i.e. positive value).

```gdscript
# a is 1
var a = absi(-1)
```

### sign {#sign}

```gdscript
Variant sign(x: Variant)
```

Returns the same type of Variant as x, with -1 for negative values, 1 for positive values, and 0 for zeros. For nan values it returns 0.

Supported types: int, float, Vector2, Vector2i, Vector3, Vector3i, Vector4, Vector4i.

```gdscript
sign(-6.0) # Returns -1
sign(0.0)  # Returns 0
sign(6.0)  # Returns 1
sign(NAN)  # Returns 0
sign(Vector3(-6.0, 0.0, 6.0)) # Returns (-1, 0, 1)
```

Note: For better type safety, use signf(), signi(), Vector2.sign(), Vector2i.sign(), Vector3.sign(), Vector3i.sign(), Vector4.sign(), or Vector4i.sign().

### signf {#signf}

```gdscript
float signf(x: float)
```

Returns -1.0 if x is negative, 1.0 if x is positive, and 0.0 if x is zero. For nan values of x it returns 0.0.

```gdscript
signf(-6.5) # Returns -1.0
signf(0.0)  # Returns 0.0
signf(6.5)  # Returns 1.0
signf(NAN)  # Returns 0.0
```

### signi {#signi}

```gdscript
int signi(x: int)
```

Returns -1 if x is negative, 1 if x is positive, and 0 if x is zero.

```gdscript
signi(-6) # Returns -1
signi(0)  # Returns 0
signi(6)  # Returns 1
```

### snapped {#snapped}

```gdscript
Variant snapped(x: Variant, step: Variant)
```

Returns the multiple of step that is the closest to x. This can also be used to round a floating-point number to an arbitrary number of decimals.

The returned value is the same type of Variant as step. Supported types: int, float, Vector2, Vector2i, Vector3, Vector3i, Vector4, Vector4i.

```gdscript
snapped(100, 32)  # Returns 96
snapped(3.14159, 0.01)  # Returns 3.14
snapped(Vector2(34, 70), Vector2(8, 8))  # Returns (32, 72)
```

See also ceil(), floor(), and round().

Note: For better type safety, use snappedf(), snappedi(), Vector2.snapped(), Vector2i.snapped(), Vector3.snapped(), Vector3i.snapped(), Vector4.snapped(), or Vector4i.snapped().

### snappedf {#snappedf}

```gdscript
float snappedf(x: float, step: float)
```

Returns the multiple of step that is the closest to x. This can also be used to round a floating-point number to an arbitrary number of decimals.

A type-safe version of snapped(), returning a float.

```gdscript
snappedf(32.0, 2.5)  # Returns 32.5
snappedf(3.14159, 0.01)  # Returns 3.14
```

### snappedi {#snappedi}

```gdscript
int snappedi(x: float, step: int)
```

Returns the multiple of step that is the closest to x.

A type-safe version of snapped(), returning an int.

```gdscript
snappedi(53, 16)  # Returns 48
snappedi(4096, 100)  # Returns 4100
```

### pow {#pow}

```gdscript
float pow(base: float, exp: float)
```

Returns the result of base raised to the power of exp.

In GDScript, this is the equivalent of the ** operator.

```gdscript
pow(2, 5)   # Returns 32.0
pow(4, 1.5) # Returns 8.0
```

### log {#log}

```gdscript
float log(x: float)
```

Returns the natural logarithm [https://en.wikipedia.org/wiki/Natural_logarithm] of x (base e [https://en.wikipedia.org/wiki/E_(mathematical_constant)], with e being approximately 2.71828). This is the amount of time needed to reach a certain level of continuous growth.

Note: This is not the same as the "log" function on most calculators, which uses a base 10 logarithm. To use base 10 logarithm, use log(x) / log(10).

```gdscript
log(10) # Returns 2.302585
```

Note: The logarithm of 0 returns -inf, while negative values return -nan.

### exp {#exp}

```gdscript
float exp(x: float)
```

The natural exponential function. It raises the mathematical constant e to the power of x and returns it.

e has an approximate value of 2.71828, and can be obtained with exp(1).

For exponents to other bases use the method pow().

```gdscript
var a = exp(2) # Approximately 7.39
```

### is_nan {#is_nan}

```gdscript
bool is_nan(x: float)
```

Returns true if x is a NaN ("Not a Number" or invalid) value. This method is needed as @GDScript.NAN is not equal to itself, which means x == NAN can't be used to check whether a value is a NaN.

### is_inf {#is_inf}

```gdscript
bool is_inf(x: float)
```

Returns true if x is either positive infinity or negative infinity. See also is_finite() and is_nan().

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(a: float, b: float)
```

Returns true if a and b are approximately equal to each other.

Here, "approximately equal" means that a and b are within a small internal epsilon of each other, which scales with the magnitude of the numbers.

Infinity values of the same sign are considered equal.

### is_zero_approx {#is_zero_approx}

```gdscript
bool is_zero_approx(x: float)
```

Returns true if x is zero or almost zero. The comparison is done using a tolerance calculation with a small internal epsilon.

This function is faster than using is_equal_approx() with one value as zero.

### is_finite {#is_finite}

```gdscript
bool is_finite(x: float)
```

Returns whether x is a finite value, i.e. it is not @GDScript.NAN, positive infinity, or negative infinity. See also is_inf() and is_nan().

### ease {#ease}

```gdscript
float ease(x: float, curve: float)
```

Returns an "eased" value of x based on an easing function defined with curve. This easing function is based on an exponent. The curve can be any floating-point number, with specific values leading to the following behaviors:

```
- Lower than -1.0 (exclusive): Ease in-out
- -1.0: Linear
- Between -1.0 and 0.0 (exclusive): Ease out-in
- 0.0: Constant
- Between 0.0 to 1.0 (exclusive): Ease out
- 1.0: Linear
- Greater than 1.0 (exclusive): Ease in
```

ease() curve values cheatsheet [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/ease_cheatsheet.png]

See also smoothstep(). If you need to perform more advanced transitions, use Tween.interpolate_value().

### step_decimals {#step_decimals}

```gdscript
int step_decimals(x: float)
```

Returns the position of the first non-zero digit, after the decimal point. Note that the maximum return value is 10, which is a design decision in the implementation.

```gdscript
var n = step_decimals(5)       # n is 0
n = step_decimals(1.0005)      # n is 4
n = step_decimals(0.000000005) # n is 9
```

### lerp {#lerp}

```gdscript
Variant lerp(from: Variant, to: Variant, weight: Variant)
```

Linearly interpolates between two values by the factor defined in weight. To perform interpolation, weight should be between 0.0 and 1.0 (inclusive). However, values outside this range are allowed and can be used to perform extrapolation. If this is not desired, use clampf() to limit weight.

Both from and to must be the same type. Supported types: int, float, Vector2, Vector3, Vector4, Color, Quaternion, Basis, Transform2D, Transform3D.

```gdscript
lerp(0, 4, 0.75) # Returns 3.0
```

See also inverse_lerp() which performs the reverse of this operation. To perform eased interpolation with lerp(), combine it with ease() or smoothstep(). See also remap() to map a continuous series of values to another.

Note: For better type safety, use lerpf(), Vector2.lerp(), Vector3.lerp(), Vector4.lerp(), Color.lerp(), Quaternion.slerp(), Basis.slerp(), Transform2D.interpolate_with(), or Transform3D.interpolate_with().

### lerpf {#lerpf}

```gdscript
float lerpf(from: float, to: float, weight: float)
```

Linearly interpolates between two values by the factor defined in weight. To perform interpolation, weight should be between 0.0 and 1.0 (inclusive). However, values outside this range are allowed and can be used to perform extrapolation. If this is not desired, use clampf() on the result of this function.

```gdscript
lerpf(0, 4, 0.75) # Returns 3.0
```

See also inverse_lerp() which performs the reverse of this operation. To perform eased interpolation with lerp(), combine it with ease() or smoothstep().

### cubic_interpolate {#cubic_interpolate}

```gdscript
float cubic_interpolate(from: float, to: float, pre: float, post: float, weight: float)
```

Cubic interpolates between two values by the factor defined in weight with pre and post values.

### cubic_interpolate_angle {#cubic_interpolate_angle}

```gdscript
float cubic_interpolate_angle(from: float, to: float, pre: float, post: float, weight: float)
```

Cubic interpolates between two rotation values with shortest path by the factor defined in weight with pre and post values. See also lerp_angle().

### cubic_interpolate_in_time {#cubic_interpolate_in_time}

```gdscript
float cubic_interpolate_in_time(from: float, to: float, pre: float, post: float, weight: float, to_t: float, pre_t: float, post_t: float)
```

Cubic interpolates between two values by the factor defined in weight with pre and post values.

It can perform smoother interpolation than cubic_interpolate() by the time values.

### cubic_interpolate_angle_in_time {#cubic_interpolate_angle_in_time}

```gdscript
float cubic_interpolate_angle_in_time(from: float, to: float, pre: float, post: float, weight: float, to_t: float, pre_t: float, post_t: float)
```

Cubic interpolates between two rotation values with shortest path by the factor defined in weight with pre and post values. See also lerp_angle().

It can perform smoother interpolation than cubic_interpolate() by the time values.

### bezier_interpolate {#bezier_interpolate}

```gdscript
float bezier_interpolate(start: float, control_1: float, control_2: float, end: float, t: float)
```

Returns the point at the given t on a one-dimensional Bézier curve [https://en.wikipedia.org/wiki/B%C3%A9zier_curve] defined by the given control_1, control_2, and end points.

### bezier_derivative {#bezier_derivative}

```gdscript
float bezier_derivative(start: float, control_1: float, control_2: float, end: float, t: float)
```

Returns the derivative at the given t on a one-dimensional Bézier curve [https://en.wikipedia.org/wiki/B%C3%A9zier_curve] defined by the given control_1, control_2, and end points.

### angle_difference {#angle_difference}

```gdscript
float angle_difference(from: float, to: float)
```

Returns the difference between the two angles (in radians), in the range of [-PI, +PI]. When from and to are opposite, returns -PI if from is smaller than to, or PI otherwise.

### lerp_angle {#lerp_angle}

```gdscript
float lerp_angle(from: float, to: float, weight: float)
```

Linearly interpolates between two angles (in radians) by a weight value between 0.0 and 1.0.

Similar to lerp(), but interpolates correctly when the angles wrap around @GDScript.TAU. To perform eased interpolation with lerp_angle(), combine it with ease() or smoothstep().

```gdscript
extends Sprite
var elapsed = 0.0
func _process(delta):
    var min_angle = deg_to_rad(0.0)
    var max_angle = deg_to_rad(90.0)
    rotation = lerp_angle(min_angle, max_angle, elapsed)
    elapsed += delta
```

Note: This function lerps through the shortest path between from and to. However, when these two angles are approximately PI + k * TAU apart for any integer k, it's not obvious which way they lerp due to floating-point precision errors. For example, lerp_angle(0, PI, weight) lerps counter-clockwise, while lerp_angle(0, PI + 5 * TAU, weight) lerps clockwise.

### inverse_lerp {#inverse_lerp}

```gdscript
float inverse_lerp(from: float, to: float, weight: float)
```

Returns an interpolation or extrapolation factor considering the range specified in from and to, and the interpolated value specified in weight. The returned value will be between 0.0 and 1.0 if weight is between from and to (inclusive). If weight is located outside this range, then an extrapolation factor will be returned (return value lower than 0.0 or greater than 1.0). Use clamp() on the result of inverse_lerp() if this is not desired.

```gdscript
# The interpolation ratio in the `lerp()` call below is 0.75.
var middle = lerp(20, 30, 0.75)
# middle is now 27.5.
# Now, we pretend to have forgotten the original ratio and want to get it back.
var ratio = inverse_lerp(20, 30, 27.5)
# ratio is now 0.75.
```

See also lerp(), which performs the reverse of this operation, and remap() to map a continuous series of values to another.

### remap {#remap}

```gdscript
float remap(value: float, istart: float, istop: float, ostart: float, ostop: float)
```

Maps a value from range [istart, istop] to [ostart, ostop]. See also lerp() and inverse_lerp(). If value is outside [istart, istop], then the resulting value will also be outside [ostart, ostop]. If this is not desired, use clamp() on the result of this function.

```gdscript
remap(75, 0, 100, -1, 1) # Returns 0.5
```

For complex use cases where multiple ranges are needed, consider using Curve or Gradient instead.

Note: If istart == istop, the return value is undefined (most likely NaN, INF, or -INF).

### smoothstep {#smoothstep}

```gdscript
float smoothstep(from: float, to: float, x: float)
```

Returns a smooth cubic Hermite interpolation between 0 and 1.

For positive ranges (when from <= to) the return value is 0 when x <= from, and 1 when x >= to. If x lies between from and to, the return value follows an S-shaped curve that smoothly transitions from 0 to 1.

For negative ranges (when from > to) the function is mirrored and returns 1 when x <= to and 0 when x >= from.

This S-shaped curve is the cubic Hermite interpolator, given by f(y) = 3*y^2 - 2*y^3 where y = (x-from) / (to-from).

```gdscript
smoothstep(0, 2, -5.0) # Returns 0.0
smoothstep(0, 2, 0.5) # Returns 0.15625
smoothstep(0, 2, 1.0) # Returns 0.5
smoothstep(0, 2, 2.0) # Returns 1.0
```

Compared to ease() with a curve value of -1.6521, smoothstep() returns the smoothest possible curve with no sudden changes in the derivative. If you need to perform more advanced transitions, use Tween or AnimationPlayer.

Comparison between smoothstep() and ease(x, -1.6521) return values [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/smoothstep_ease_comparison.png]

Smoothstep() return values with positive, zero, and negative ranges [https://raw.githubusercontent.com/godotengine/godot-docs/master/img/smoothstep_range.webp]

### move_toward {#move_toward}

```gdscript
float move_toward(from: float, to: float, delta: float)
```

Moves from toward to by the delta amount. Will not go past to.

Use a negative delta value to move away.

```gdscript
move_toward(5, 10, 4)    # Returns 9
move_toward(10, 5, 4)    # Returns 6
move_toward(5, 10, 9)    # Returns 10
move_toward(10, 5, -1.5) # Returns 11.5
```

### rotate_toward {#rotate_toward}

```gdscript
float rotate_toward(from: float, to: float, delta: float)
```

Rotates from toward to by the delta amount. Will not go past to.

Similar to move_toward(), but interpolates correctly when the angles wrap around @GDScript.TAU.

If delta is negative, this function will rotate away from to, toward the opposite angle, and will not go past the opposite angle.

### deg_to_rad {#deg_to_rad}

```gdscript
float deg_to_rad(deg: float)
```

Converts an angle expressed in degrees to radians.

```gdscript
var r = deg_to_rad(180) # r is 3.141593
```

### rad_to_deg {#rad_to_deg}

```gdscript
float rad_to_deg(rad: float)
```

Converts an angle expressed in radians to degrees.

```gdscript
rad_to_deg(0.523599) # Returns 30
rad_to_deg(PI)       # Returns 180
rad_to_deg(PI * 2)   # Returns 360
```

### linear_to_db {#linear_to_db}

```gdscript
float linear_to_db(lin: float)
```

Converts from linear energy to decibels (audio). Since volume is not normally linear, this can be used to implement volume sliders that behave as expected.

Example: Change the Master bus's volume through a Slider node, which ranges from 0.0 to 1.0:

```gdscript
AudioServer.set_bus_volume_db(AudioServer.get_bus_index("Master"), linear_to_db($Slider.value))
```

### db_to_linear {#db_to_linear}

```gdscript
float db_to_linear(db: float)
```

Converts from decibels to linear energy (audio).

### wrap {#wrap}

```gdscript
Variant wrap(value: Variant, min: Variant, max: Variant)
```

Wraps the Variant value between min and max. min is inclusive while max is exclusive. This can be used for creating loop-like behavior or infinite surfaces.

Variant types int and float are supported. If any of the arguments is float, this function returns a float, otherwise it returns an int.

```gdscript
var a = wrap(4, 5, 10)
# a is 9 (int)
var a = wrap(7, 5, 10)
# a is 7 (int)
var a = wrap(10.5, 5, 10)
# a is 5.5 (float)
```

### wrapi {#wrapi}

```gdscript
int wrapi(value: int, min: int, max: int)
```

Wraps the integer value between min and max. min is inclusive while max is exclusive. This can be used for creating loop-like behavior or infinite surfaces.

```gdscript
# Infinite loop between 5 and 9
frame = wrapi(frame + 1, 5, 10)
```

```gdscript
# result is -2
var result = wrapi(-6, -5, -1)
```

### wrapf {#wrapf}

```gdscript
float wrapf(value: float, min: float, max: float)
```

Wraps the float value between min and max. min is inclusive while max is exclusive. This can be used for creating loop-like behavior or infinite surfaces.

```gdscript
# Infinite loop between 5.0 and 9.9
value = wrapf(value + 0.1, 5.0, 10.0)
```

```gdscript
# Infinite rotation (in radians)
angle = wrapf(angle + 0.1, 0.0, TAU)
```

```gdscript
# Infinite rotation (in radians)
angle = wrapf(angle + 0.1, -PI, PI)
```

Note: If min is 0, this is equivalent to fposmod(), so prefer using that instead. wrapf() is more flexible than using the fposmod() approach by giving the user control over the minimum value.

### max {#max}

```gdscript
Variant max(arg1: Variant, arg2: Variant, ...)
```

Returns the maximum of the given numeric values. This function can take any number of arguments.

```gdscript
max(1, 7, 3, -6, 5) # Returns 7
```

Note: When using this on vectors it will not perform component-wise maximum, and will pick the largest value when compared using x < y. To perform component-wise maximum, use Vector2.max(), Vector2i.max(), Vector3.max(), Vector3i.max(), Vector4.max(), and Vector4i.max().

### maxi {#maxi}

```gdscript
int maxi(a: int, b: int)
```

Returns the maximum of two int values.

```gdscript
maxi(1, 2)   # Returns 2
maxi(-3, -4) # Returns -3
```

### maxf {#maxf}

```gdscript
float maxf(a: float, b: float)
```

Returns the maximum of two float values.

```gdscript
maxf(3.6, 24)   # Returns 24.0
maxf(-3.99, -4) # Returns -3.99
```

### min {#min}

```gdscript
Variant min(arg1: Variant, arg2: Variant, ...)
```

Returns the minimum of the given numeric values. This function can take any number of arguments.

```gdscript
min(1, 7, 3, -6, 5) # Returns -6
```

Note: When using this on vectors it will not perform component-wise minimum, and will pick the smallest value when compared using x < y. To perform component-wise minimum, use Vector2.min(), Vector2i.min(), Vector3.min(), Vector3i.min(), Vector4.min(), and Vector4i.min().

### mini {#mini}

```gdscript
int mini(a: int, b: int)
```

Returns the minimum of two int values.

```gdscript
mini(1, 2)   # Returns 1
mini(-3, -4) # Returns -4
```

### minf {#minf}

```gdscript
float minf(a: float, b: float)
```

Returns the minimum of two float values.

```gdscript
minf(3.6, 24)   # Returns 3.6
minf(-3.99, -4) # Returns -4.0
```

### clamp {#clamp}

```gdscript
Variant clamp(value: Variant, min: Variant, max: Variant)
```

Clamps the value, returning a Variant not less than min and not more than max. Any values that can be compared with the less than and greater than operators will work.

```gdscript
var a = clamp(-10, -1, 5)
# a is -1
var b = clamp(8.1, 0.9, 5.5)
# b is 5.5
```

Note: For better type safety, use clampf(), clampi(), Vector2.clamp(), Vector2i.clamp(), Vector3.clamp(), Vector3i.clamp(), Vector4.clamp(), Vector4i.clamp(), or Color.clamp() (not currently supported by this method).

Note: When using this on vectors it will not perform component-wise clamping, and will pick min if value < min or max if value > max. To perform component-wise clamping use the methods listed above.

### clampi {#clampi}

```gdscript
int clampi(value: int, min: int, max: int)
```

Clamps the value, returning an int not less than min and not more than max.

```gdscript
var speed = 42
var a = clampi(speed, 1, 20) # a is 20
speed = -10
var b = clampi(speed, -1, 1) # b is -1
```

### clampf {#clampf}

```gdscript
float clampf(value: float, min: float, max: float)
```

Clamps the value, returning a float not less than min and not more than max.

```gdscript
var speed = 42.1
var a = clampf(speed, 1.0, 20.5) # a is 20.5
speed = -10.0
var b = clampf(speed, -1.0, 1.0) # b is -1.0
```

### nearest_po2 {#nearest_po2}

```gdscript
int nearest_po2(value: int)
```

Returns the smallest integer power of 2 that is greater than or equal to value.

```gdscript
nearest_po2(3) # Returns 4
nearest_po2(4) # Returns 4
nearest_po2(5) # Returns 8
nearest_po2(0)  # Returns 0 (this may not be expected)
nearest_po2(-1) # Returns 0 (this may not be expected)
```

Warning: Due to its implementation, this method returns 0 rather than 1 for values less than or equal to 0, with an exception for value being the smallest negative 64-bit integer (-9223372036854775808) in which case the value is returned unchanged.

### pingpong {#pingpong}

```gdscript
float pingpong(value: float, length: float)
```

Wraps value between 0 and the length. If the limit is reached, the next value the function returns is decreased to the 0 side or increased to the length side (like a triangle wave). If length is less than zero, it becomes positive.

```gdscript
pingpong(-3.0, 3.0) # Returns 3.0
pingpong(-2.0, 3.0) # Returns 2.0
pingpong(-1.0, 3.0) # Returns 1.0
pingpong(0.0, 3.0)  # Returns 0.0
pingpong(1.0, 3.0)  # Returns 1.0
pingpong(2.0, 3.0)  # Returns 2.0
pingpong(3.0, 3.0)  # Returns 3.0
pingpong(4.0, 3.0)  # Returns 2.0
pingpong(5.0, 3.0)  # Returns 1.0
pingpong(6.0, 3.0)  # Returns 0.0
```

### randomize {#randomize}

```gdscript
void randomize()
```

Randomizes the seed (or the internal state) of the random number generator. The current implementation uses a number based on the device's time.

Note: This function is called automatically when the project is run. If you need to fix the seed to have consistent, reproducible results, use seed() to initialize the random number generator.

### randi {#randi}

```gdscript
int randi()
```

Returns a random unsigned 32-bit integer. Use remainder to obtain a random value in the interval [0, N - 1] (where N is smaller than 2^32).

```
randi()           # Returns random integer between 0 and 2^32 - 1
randi() % 20      # Returns random integer between 0 and 19
randi() % 100     # Returns random integer between 0 and 99
randi() % 100 + 1 # Returns random integer between 1 and 100
```

```
GD.Randi();           // Returns random integer between 0 and 2^32 - 1
GD.Randi() % 20;      // Returns random integer between 0 and 19
GD.Randi() % 100;     // Returns random integer between 0 and 99
GD.Randi() % 100 + 1; // Returns random integer between 1 and 100
```

### randf {#randf}

```gdscript
float randf()
```

Returns a random floating-point value between 0.0 and 1.0 (inclusive).

```
randf() # Returns e.g. 0.375671
```

```
GD.Randf(); // Returns e.g. 0.375671
```

### randi_range {#randi_range}

```gdscript
int randi_range(from: int, to: int)
```

Returns a random signed 32-bit integer between from and to (inclusive). If to is lesser than from, they are swapped.

```
randi_range(0, 1)      # Returns either 0 or 1
randi_range(-10, 1000) # Returns random integer between -10 and 1000
```

```
GD.RandRange(0, 1);      // Returns either 0 or 1
GD.RandRange(-10, 1000); // Returns random integer between -10 and 1000
```

### randf_range {#randf_range}

```gdscript
float randf_range(from: float, to: float)
```

Returns a random floating-point value between from and to (inclusive).

```
randf_range(0, 20.5) # Returns e.g. 7.45315
randf_range(-10, 10) # Returns e.g. -3.844535
```

```
GD.RandRange(0.0, 20.5);   // Returns e.g. 7.45315
GD.RandRange(-10.0, 10.0); // Returns e.g. -3.844535
```

### randfn {#randfn}

```gdscript
float randfn(mean: float, deviation: float)
```

Returns a normally-distributed [https://en.wikipedia.org/wiki/Normal_distribution], pseudo-random floating-point value from the specified mean and a standard deviation. This is also known as a Gaussian distribution.

Note: This method uses the Box-Muller transform [https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform] algorithm.

### seed {#seed}

```gdscript
void seed(base: int)
```

Sets the seed for the random number generator to base. Setting the seed manually can ensure consistent, repeatable results for most random functions.

```
var my_seed = "Godot Rocks".hash()
seed(my_seed)
var a = randf() + randi()
seed(my_seed)
var b = randf() + randi()
# a and b are now identical
```

```
ulong mySeed = (ulong)GD.Hash("Godot Rocks");
GD.Seed(mySeed);
var a = GD.Randf() + GD.Randi();
GD.Seed(mySeed);
var b = GD.Randf() + GD.Randi();
// a and b are now identical
```

### rand_from_seed {#rand_from_seed}

```gdscript
PackedInt64Array rand_from_seed(seed: int)
```

Given a seed, returns a PackedInt64Array of size 2, where its first element is the randomized int value, and the second element is the same as seed. Passing the same seed consistently returns the same array.

Note: "Seed" here refers to the internal state of the pseudo random number generator, currently implemented as a 64 bit integer.

```gdscript
var a = rand_from_seed(4)
print(a[0]) # Prints 2879024997
print(a[1]) # Prints 4
```

### weakref {#weakref}

```gdscript
Variant weakref(obj: Variant)
```

Returns a WeakRef instance holding a weak reference to obj. Returns an empty WeakRef instance if obj is null. Prints an error and returns null if obj is neither Object-derived nor null.

A weak reference to an object is not enough to keep the object alive: when the only remaining references to a referent are weak references, garbage collection is free to destroy the referent and reuse its memory for something else. However, until the object is actually destroyed the weak reference may return the object even if there are no strong references to it.

### typeof {#typeof}

```gdscript
int typeof(variable: Variant)
```

Returns the internal type of the given variable, using the Variant.Type values.

```gdscript
var json = JSON.new()
json.parse('["a", "b", "c"]')
var result = json.get_data()
if typeof(result) == TYPE_ARRAY:
    print(result[0]) # Prints "a"
else:
    print("Unexpected result!")
```

See also type_string().

### type_convert {#type_convert}

```gdscript
Variant type_convert(variant: Variant, type: int)
```

Converts the given variant to the given type, using the Variant.Type values. This method is generous with how it handles types, it can automatically convert between array types, convert numeric Strings to int, and converting most things to String.

If the type conversion cannot be done, this method will return the default value for that type, for example converting Rect2 to Vector2 will always return Vector2.ZERO. This method will never show error messages as long as type is a valid Variant type.

The returned value is a Variant, but the data inside and its type will be the same as the requested type.

```gdscript
type_convert("Hi!", TYPE_INT) # Returns 0
type_convert("123", TYPE_INT) # Returns 123
type_convert(123.4, TYPE_INT) # Returns 123
type_convert(5, TYPE_VECTOR2) # Returns (0, 0)
type_convert("Hi!", TYPE_NIL) # Returns null
```

### str {#str}

```gdscript
String str(arg1: Variant, ...)
```

Converts one or more arguments of any Variant type to a String in the best way possible.

```gdscript
var a = [10, 20, 30]
var b = str(a)
print(len(a)) # Prints 3 (the number of elements in the array).
print(len(b)) # Prints 12 (the length of the string "[10, 20, 30]").
```

### error_string {#error_string}

```gdscript
String error_string(error: int)
```

Returns a human-readable name for the given Error code.

```gdscript
print(OK)                              # Prints 0
print(error_string(OK))                # Prints "OK"
print(error_string(ERR_BUSY))          # Prints "Busy"
print(error_string(ERR_OUT_OF_MEMORY)) # Prints "Out of memory"
```

### type_string {#type_string}

```gdscript
String type_string(type: int)
```

Returns a human-readable name of the given type, using the Variant.Type values.

```gdscript
print(TYPE_INT) # Prints 2
print(type_string(TYPE_INT)) # Prints "int"
print(type_string(TYPE_STRING)) # Prints "String"
```

See also typeof().

### print {#print}

```gdscript
void print(arg1: Variant, ...)
```

Converts one or more arguments of any type to string in the best way possible and prints them to the console.

```
var a = [1, 2, 3]
print("a", "b", a) # Prints "ab[1, 2, 3]"
```

```
Godot.Collections.Array a = [1, 2, 3];
GD.Print("a", "b", a); // Prints "ab[1, 2, 3]"
```

Note: Consider using push_error() and push_warning() to print error and warning messages instead of print() or print_rich(). This distinguishes them from print messages used for debugging purposes, while also displaying a stack trace when an error or warning is printed. See also Engine.print_to_stdout and ProjectSettings.application/run/disable_stdout.

### print_rich {#print_rich}

```gdscript
void print_rich(arg1: Variant, ...)
```

Converts one or more arguments of any type to string in the best way possible and prints them to the console.

The following BBCode tags are supported: b, i, u, s, indent, code, url, center, right, color, bgcolor, fgcolor.

URL tags only support URLs wrapped by a URL tag, not URLs with a different title.

When printing to standard output, the supported subset of BBCode is converted to ANSI escape codes for the terminal emulator to display. Support for ANSI escape codes varies across terminal emulators, especially for italic and strikethrough. In standard output, code is represented with faint text but without any font change. Unsupported tags are left as-is in standard output.

```
print_rich("[color=green][b]Hello world![/b][/color]") # Prints "Hello world!", in green with a bold font.
```

```
GD.PrintRich("[color=green][b]Hello world![/b][/color]"); // Prints "Hello world!", in green with a bold font.
```

Note: Consider using push_error() and push_warning() to print error and warning messages instead of print() or print_rich(). This distinguishes them from print messages used for debugging purposes, while also displaying a stack trace when an error or warning is printed.

Note: Output displayed in the editor supports clickable text tags. The [url] tag's address value is handled by OS.shell_open() when clicked.

### printerr {#printerr}

```gdscript
void printerr(arg1: Variant, ...)
```

Prints one or more arguments to strings in the best way possible to standard error line.

```
printerr("prints to stderr")
```

```
GD.PrintErr("prints to stderr");
```

### printt {#printt}

```gdscript
void printt(arg1: Variant, ...)
```

Prints one or more arguments to the console with a tab between each argument.

```
printt("A", "B", "C") # Prints "A       B       C"
```

```
GD.PrintT("A", "B", "C"); // Prints "A       B       C"
```

### prints {#prints}

```gdscript
void prints(arg1: Variant, ...)
```

Prints one or more arguments to the console with a space between each argument.

```
prints("A", "B", "C") # Prints "A B C"
```

```
GD.PrintS("A", "B", "C"); // Prints "A B C"
```

### printraw {#printraw}

```gdscript
void printraw(arg1: Variant, ...)
```

Prints one or more arguments to strings in the best way possible to the OS terminal. Unlike print(), no newline is automatically added at the end.

Note: The OS terminal is not the same as the editor's Output dock. The output sent to the OS terminal can be seen when running Godot from a terminal. On Windows, this requires using the console.exe executable.

```
# Prints "ABC" to terminal.
printraw("A")
printraw("B")
printraw("C")
```

```
// Prints "ABC" to terminal.
GD.PrintRaw("A");
GD.PrintRaw("B");
GD.PrintRaw("C");
```

### print_verbose {#print_verbose}

```gdscript
void print_verbose(arg1: Variant, ...)
```

If verbose mode is enabled (OS.is_stdout_verbose() returning true), converts one or more arguments of any type to string in the best way possible and prints them to the console.

### push_error {#push_error}

```gdscript
void push_error(arg1: Variant, ...)
```

Pushes an error message to Godot's built-in debugger and to the OS terminal.

```
push_error("test error") # Prints "test error" to debugger and terminal as an error.
```

```
GD.PushError("test error"); // Prints "test error" to debugger and terminal as an error.
```

Note: This function does not pause project execution. To print an error message and pause project execution in debug builds, use assert(false, "test error") instead.

### push_warning {#push_warning}

```gdscript
void push_warning(arg1: Variant, ...)
```

Pushes a warning message to Godot's built-in debugger and to the OS terminal.

```
push_warning("test warning") # Prints "test warning" to debugger and terminal as a warning.
```

```
GD.PushWarning("test warning"); // Prints "test warning" to debugger and terminal as a warning.
```

### var_to_str {#var_to_str}

```gdscript
String var_to_str(variable: Variant)
```

Converts a Variant variable to a formatted String that can then be parsed using str_to_var().

```
var a = { "a": 1, "b": 2 }
print(var_to_str(a))
```

```
var a = new Godot.Collections.Dictionary { ["a"] = 1, ["b"] = 2 };
GD.Print(GD.VarToStr(a));
```

Prints:

```
{
    "a": 1,
    "b": 2
}
```

Note: Converting Signal or Callable is not supported and will result in an empty value for these types, regardless of their data.

### str_to_var {#str_to_var}

```gdscript
Variant str_to_var(string: String)
```

Converts a formatted string that was returned by var_to_str() to the original Variant.

```
var data = '{ "a": 1, "b": 2 }' # data is a String
var dict = str_to_var(data)     # dict is a Dictionary
print(dict["a"])                # Prints 1
```

```
string data = "{ \"a\": 1, \"b\": 2 }";           // data is a string
var dict = GD.StrToVar(data).AsGodotDictionary(); // dict is a Dictionary
GD.Print(dict["a"]);                              // Prints 1
```

### var_to_bytes {#var_to_bytes}

```gdscript
PackedByteArray var_to_bytes(variable: Variant)
```

Encodes a Variant value to a byte array, without encoding objects. Deserialization can be done with bytes_to_var().

Note: If you need object serialization, see var_to_bytes_with_objects().

Note: Encoding Callable is not supported and will result in an empty value, regardless of the data.

### bytes_to_var {#bytes_to_var}

```gdscript
Variant bytes_to_var(bytes: PackedByteArray)
```

Decodes a byte array back to a Variant value, without decoding objects.

Note: If you need object deserialization, see bytes_to_var_with_objects().

### var_to_bytes_with_objects {#var_to_bytes_with_objects}

```gdscript
PackedByteArray var_to_bytes_with_objects(variable: Variant)
```

Encodes a Variant value to a byte array. Encoding objects is allowed (and can potentially include executable code). Deserialization can be done with bytes_to_var_with_objects().

Note: Encoding Callable is not supported and will result in an empty value, regardless of the data.

### bytes_to_var_with_objects {#bytes_to_var_with_objects}

```gdscript
Variant bytes_to_var_with_objects(bytes: PackedByteArray)
```

Decodes a byte array back to a Variant value. Decoding objects is allowed.

Warning: Deserialized object can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats (remote code execution).

### hash {#hash}

```gdscript
int hash(variable: Variant)
```

Returns the integer hash of the passed variable.

```
print(hash("a")) # Prints 177670
```

```
GD.Print(GD.Hash("a")); // Prints 177670
```

### instance_from_id {#instance_from_id}

```gdscript
Object instance_from_id(instance_id: int)
```

Returns the Object that corresponds to instance_id. All Objects have a unique instance ID. See also Object.get_instance_id().

```
var drink = "water"
func _ready():
    var id = get_instance_id()
    var instance = instance_from_id(id)
    print(instance.drink) # Prints "water"
```

```
public partial class MyNode : Node
{
    public string Drink { get; set; } = "water";
    public override void _Ready()
    {
        ulong id = GetInstanceId();
        var instance = (MyNode)InstanceFromId(Id);
        GD.Print(instance.Drink); // Prints "water"
    }
}
```

### is_instance_id_valid {#is_instance_id_valid}

```gdscript
bool is_instance_id_valid(id: int)
```

Returns true if the Object that corresponds to id is a valid object (e.g. has not been deleted from memory). All Objects have a unique instance ID.

### is_instance_valid {#is_instance_valid}

```gdscript
bool is_instance_valid(instance: Variant)
```

Returns true if instance is a valid Object (e.g. has not been deleted from memory).

### rid_allocate_id {#rid_allocate_id}

```gdscript
int rid_allocate_id()
```

Allocates a unique ID which can be used by the implementation to construct an RID. This is used mainly from native extensions to implement servers.

### rid_from_int64 {#rid_from_int64}

```gdscript
RID rid_from_int64(base: int)
```

Creates an RID from a base. This is used mainly from native extensions to build servers.

### is_same {#is_same}

```gdscript
bool is_same(a: Variant, b: Variant)
```

Returns true, for value types, if a and b share the same value. Returns true, for reference types, if the references of a and b are the same.

```gdscript
# Vector2 is a value type
var vec2_a = Vector2(0, 0)
var vec2_b = Vector2(0, 0)
var vec2_c = Vector2(1, 1)
is_same(vec2_a, vec2_a)  # true
is_same(vec2_a, vec2_b)  # true
is_same(vec2_a, vec2_c)  # false
# Array is a reference type
var arr_a = []
var arr_b = []
is_same(arr_a, arr_a)  # true
is_same(arr_a, arr_b)  # false
```

These are Variant value types: null, bool, int, float, String, StringName, Vector2, Vector2i, Vector3, Vector3i, Vector4, Vector4i, Rect2, Rect2i, Transform2D, Transform3D, Plane, Quaternion, AABB, Basis, Projection, Color, NodePath, RID, Callable and Signal.

These are Variant reference types: Object, Dictionary, Array, PackedByteArray, PackedInt32Array, PackedInt64Array, PackedFloat32Array, PackedFloat64Array, PackedStringArray, PackedVector2Array, PackedVector3Array, PackedVector4Array, and PackedColorArray.

## 全局枚举

### Side {#enum-Side}

```gdscript
enum Side { SIDE_LEFT = 0, SIDE_TOP = 1, SIDE_RIGHT = 2, SIDE_BOTTOM = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SIDE_LEFT {#SIDE_LEFT}

```gdscript
SIDE_LEFT = 0
```

Left side, usually used for Control or StyleBox-derived classes.

#### SIDE_TOP {#SIDE_TOP}

```gdscript
SIDE_TOP = 1
```

Top side, usually used for Control or StyleBox-derived classes.

#### SIDE_RIGHT {#SIDE_RIGHT}

```gdscript
SIDE_RIGHT = 2
```

Right side, usually used for Control or StyleBox-derived classes.

#### SIDE_BOTTOM {#SIDE_BOTTOM}

```gdscript
SIDE_BOTTOM = 3
```

Bottom side, usually used for Control or StyleBox-derived classes.

---

### Corner {#enum-Corner}

```gdscript
enum Corner { CORNER_TOP_LEFT = 0, CORNER_TOP_RIGHT = 1, CORNER_BOTTOM_RIGHT = 2, CORNER_BOTTOM_LEFT = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CORNER_TOP_LEFT {#CORNER_TOP_LEFT}

```gdscript
CORNER_TOP_LEFT = 0
```

Top-left corner.

#### CORNER_TOP_RIGHT {#CORNER_TOP_RIGHT}

```gdscript
CORNER_TOP_RIGHT = 1
```

Top-right corner.

#### CORNER_BOTTOM_RIGHT {#CORNER_BOTTOM_RIGHT}

```gdscript
CORNER_BOTTOM_RIGHT = 2
```

Bottom-right corner.

#### CORNER_BOTTOM_LEFT {#CORNER_BOTTOM_LEFT}

```gdscript
CORNER_BOTTOM_LEFT = 3
```

Bottom-left corner.

---

### Orientation {#enum-Orientation}

```gdscript
enum Orientation { VERTICAL = 1, HORIZONTAL = 0 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VERTICAL {#VERTICAL}

```gdscript
VERTICAL = 1
```

General vertical alignment, usually used for Separator, ScrollBar, Slider, etc.

#### HORIZONTAL {#HORIZONTAL}

```gdscript
HORIZONTAL = 0
```

General horizontal alignment, usually used for Separator, ScrollBar, Slider, etc.

---

### ClockDirection {#enum-ClockDirection}

```gdscript
enum ClockDirection { CLOCKWISE = 0, COUNTERCLOCKWISE = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### CLOCKWISE {#CLOCKWISE}

```gdscript
CLOCKWISE = 0
```

Clockwise rotation. Used by some methods (e.g. Image.rotate_90()).

#### COUNTERCLOCKWISE {#COUNTERCLOCKWISE}

```gdscript
COUNTERCLOCKWISE = 1
```

Counter-clockwise rotation. Used by some methods (e.g. Image.rotate_90()).

---

### HorizontalAlignment {#enum-HorizontalAlignment}

```gdscript
enum HorizontalAlignment { HORIZONTAL_ALIGNMENT_LEFT = 0, HORIZONTAL_ALIGNMENT_CENTER = 1, HORIZONTAL_ALIGNMENT_RIGHT = 2, HORIZONTAL_ALIGNMENT_FILL = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HORIZONTAL_ALIGNMENT_LEFT {#HORIZONTAL_ALIGNMENT_LEFT}

```gdscript
HORIZONTAL_ALIGNMENT_LEFT = 0
```

Horizontal left alignment, usually for text-derived classes.

#### HORIZONTAL_ALIGNMENT_CENTER {#HORIZONTAL_ALIGNMENT_CENTER}

```gdscript
HORIZONTAL_ALIGNMENT_CENTER = 1
```

Horizontal center alignment, usually for text-derived classes.

#### HORIZONTAL_ALIGNMENT_RIGHT {#HORIZONTAL_ALIGNMENT_RIGHT}

```gdscript
HORIZONTAL_ALIGNMENT_RIGHT = 2
```

Horizontal right alignment, usually for text-derived classes.

#### HORIZONTAL_ALIGNMENT_FILL {#HORIZONTAL_ALIGNMENT_FILL}

```gdscript
HORIZONTAL_ALIGNMENT_FILL = 3
```

Expand row to fit width, usually for text-derived classes.

---

### VerticalAlignment {#enum-VerticalAlignment}

```gdscript
enum VerticalAlignment { VERTICAL_ALIGNMENT_TOP = 0, VERTICAL_ALIGNMENT_CENTER = 1, VERTICAL_ALIGNMENT_BOTTOM = 2, VERTICAL_ALIGNMENT_FILL = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### VERTICAL_ALIGNMENT_TOP {#VERTICAL_ALIGNMENT_TOP}

```gdscript
VERTICAL_ALIGNMENT_TOP = 0
```

Vertical top alignment, usually for text-derived classes.

#### VERTICAL_ALIGNMENT_CENTER {#VERTICAL_ALIGNMENT_CENTER}

```gdscript
VERTICAL_ALIGNMENT_CENTER = 1
```

Vertical center alignment, usually for text-derived classes.

#### VERTICAL_ALIGNMENT_BOTTOM {#VERTICAL_ALIGNMENT_BOTTOM}

```gdscript
VERTICAL_ALIGNMENT_BOTTOM = 2
```

Vertical bottom alignment, usually for text-derived classes.

#### VERTICAL_ALIGNMENT_FILL {#VERTICAL_ALIGNMENT_FILL}

```gdscript
VERTICAL_ALIGNMENT_FILL = 3
```

Expand rows to fit height, usually for text-derived classes.

---

### InlineAlignment {#enum-InlineAlignment}

```gdscript
enum InlineAlignment { INLINE_ALIGNMENT_TOP_TO = 0, INLINE_ALIGNMENT_CENTER_TO = 1, INLINE_ALIGNMENT_BASELINE_TO = 3, INLINE_ALIGNMENT_BOTTOM_TO = 2, INLINE_ALIGNMENT_TO_TOP = 0, INLINE_ALIGNMENT_TO_CENTER = 4, INLINE_ALIGNMENT_TO_BASELINE = 8, INLINE_ALIGNMENT_TO_BOTTOM = 12, INLINE_ALIGNMENT_TOP = 0, INLINE_ALIGNMENT_CENTER = 5, INLINE_ALIGNMENT_BOTTOM = 14, INLINE_ALIGNMENT_IMAGE_MASK = 3, INLINE_ALIGNMENT_TEXT_MASK = 12 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INLINE_ALIGNMENT_TOP_TO {#INLINE_ALIGNMENT_TOP_TO}

```gdscript
INLINE_ALIGNMENT_TOP_TO = 0
```

Aligns the top of the inline object (e.g. image, table) to the position of the text specified by INLINE_ALIGNMENT_TO_* constant.

#### INLINE_ALIGNMENT_CENTER_TO {#INLINE_ALIGNMENT_CENTER_TO}

```gdscript
INLINE_ALIGNMENT_CENTER_TO = 1
```

Aligns the center of the inline object (e.g. image, table) to the position of the text specified by INLINE_ALIGNMENT_TO_* constant.

#### INLINE_ALIGNMENT_BASELINE_TO {#INLINE_ALIGNMENT_BASELINE_TO}

```gdscript
INLINE_ALIGNMENT_BASELINE_TO = 3
```

Aligns the baseline (user defined) of the inline object (e.g. image, table) to the position of the text specified by INLINE_ALIGNMENT_TO_* constant.

#### INLINE_ALIGNMENT_BOTTOM_TO {#INLINE_ALIGNMENT_BOTTOM_TO}

```gdscript
INLINE_ALIGNMENT_BOTTOM_TO = 2
```

Aligns the bottom of the inline object (e.g. image, table) to the position of the text specified by INLINE_ALIGNMENT_TO_* constant.

#### INLINE_ALIGNMENT_TO_TOP {#INLINE_ALIGNMENT_TO_TOP}

```gdscript
INLINE_ALIGNMENT_TO_TOP = 0
```

Aligns the position of the inline object (e.g. image, table) specified by INLINE_ALIGNMENT_*_TO constant to the top of the text.

#### INLINE_ALIGNMENT_TO_CENTER {#INLINE_ALIGNMENT_TO_CENTER}

```gdscript
INLINE_ALIGNMENT_TO_CENTER = 4
```

Aligns the position of the inline object (e.g. image, table) specified by INLINE_ALIGNMENT_*_TO constant to the center of the text.

#### INLINE_ALIGNMENT_TO_BASELINE {#INLINE_ALIGNMENT_TO_BASELINE}

```gdscript
INLINE_ALIGNMENT_TO_BASELINE = 8
```

Aligns the position of the inline object (e.g. image, table) specified by INLINE_ALIGNMENT_*_TO constant to the baseline of the text.

#### INLINE_ALIGNMENT_TO_BOTTOM {#INLINE_ALIGNMENT_TO_BOTTOM}

```gdscript
INLINE_ALIGNMENT_TO_BOTTOM = 12
```

Aligns inline object (e.g. image, table) to the bottom of the text.

#### INLINE_ALIGNMENT_TOP {#INLINE_ALIGNMENT_TOP}

```gdscript
INLINE_ALIGNMENT_TOP = 0
```

Aligns top of the inline object (e.g. image, table) to the top of the text. Equivalent to INLINE_ALIGNMENT_TOP_TO | INLINE_ALIGNMENT_TO_TOP.

#### INLINE_ALIGNMENT_CENTER {#INLINE_ALIGNMENT_CENTER}

```gdscript
INLINE_ALIGNMENT_CENTER = 5
```

Aligns center of the inline object (e.g. image, table) to the center of the text. Equivalent to INLINE_ALIGNMENT_CENTER_TO | INLINE_ALIGNMENT_TO_CENTER.

#### INLINE_ALIGNMENT_BOTTOM {#INLINE_ALIGNMENT_BOTTOM}

```gdscript
INLINE_ALIGNMENT_BOTTOM = 14
```

Aligns bottom of the inline object (e.g. image, table) to the bottom of the text. Equivalent to INLINE_ALIGNMENT_BOTTOM_TO | INLINE_ALIGNMENT_TO_BOTTOM.

#### INLINE_ALIGNMENT_IMAGE_MASK {#INLINE_ALIGNMENT_IMAGE_MASK}

```gdscript
INLINE_ALIGNMENT_IMAGE_MASK = 3
```

A bit mask for INLINE_ALIGNMENT_*_TO alignment constants.

#### INLINE_ALIGNMENT_TEXT_MASK {#INLINE_ALIGNMENT_TEXT_MASK}

```gdscript
INLINE_ALIGNMENT_TEXT_MASK = 12
```

A bit mask for INLINE_ALIGNMENT_TO_* alignment constants.

---

### EulerOrder {#enum-EulerOrder}

```gdscript
enum EulerOrder { EULER_ORDER_XYZ = 0, EULER_ORDER_XZY = 1, EULER_ORDER_YXZ = 2, EULER_ORDER_YZX = 3, EULER_ORDER_ZXY = 4, EULER_ORDER_ZYX = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### EULER_ORDER_XYZ {#EULER_ORDER_XYZ}

```gdscript
EULER_ORDER_XYZ = 0
```

Specifies that Euler angles should be in intrinsic XYZ order. When composing, the rotations happen around the local X, Y, and Z axes, in that order. When decomposing, the order is reversed, first Z, then Y, and X last.

#### EULER_ORDER_XZY {#EULER_ORDER_XZY}

```gdscript
EULER_ORDER_XZY = 1
```

Specifies that Euler angles should be in intrinsic XZY order. When composing, the rotations happen around the local X, Z, and Y axes, in that order. When decomposing, the order is reversed, first Y, then Z, and X last.

#### EULER_ORDER_YXZ {#EULER_ORDER_YXZ}

```gdscript
EULER_ORDER_YXZ = 2
```

Specifies that Euler angles should be in intrinsic YXZ order. When composing, the rotations happen around the local Y, X, and Z axes, in that order. When decomposing, the order is reversed, first Z, then X, and Y last.

#### EULER_ORDER_YZX {#EULER_ORDER_YZX}

```gdscript
EULER_ORDER_YZX = 3
```

Specifies that Euler angles should be in intrinsic YZX order. When composing, the rotations happen around the local Y, Z, and X axes, in that order. When decomposing, the order is reversed, first X, then Z, and Y last.

#### EULER_ORDER_ZXY {#EULER_ORDER_ZXY}

```gdscript
EULER_ORDER_ZXY = 4
```

Specifies that Euler angles should be in intrinsic ZXY order. When composing, the rotations happen around the local Z, X, and Y axes, in that order. When decomposing, the order is reversed, first Y, then X, and Z last.

#### EULER_ORDER_ZYX {#EULER_ORDER_ZYX}

```gdscript
EULER_ORDER_ZYX = 5
```

Specifies that Euler angles should be in intrinsic ZYX order. When composing, the rotations happen around the local Z, Y, and X axes, in that order. When decomposing, the order is reversed, first X, then Y, and Z last.

---

### Key {#enum-Key}

```gdscript
enum Key { KEY_NONE = 0, KEY_SPECIAL = 4194304, KEY_ESCAPE = 4194305, KEY_TAB = 4194306, KEY_BACKTAB = 4194307, KEY_BACKSPACE = 4194308, KEY_ENTER = 4194309, KEY_KP_ENTER = 4194310, KEY_INSERT = 4194311, KEY_DELETE = 4194312, KEY_PAUSE = 4194313, KEY_PRINT = 4194314, KEY_SYSREQ = 4194315, KEY_CLEAR = 4194316, KEY_HOME = 4194317, KEY_END = 4194318, KEY_LEFT = 4194319, KEY_UP = 4194320, KEY_RIGHT = 4194321, KEY_DOWN = 4194322, KEY_PAGEUP = 4194323, KEY_PAGEDOWN = 4194324, KEY_SHIFT = 4194325, KEY_CTRL = 4194326, KEY_META = 4194327, KEY_ALT = 4194328, KEY_CAPSLOCK = 4194329, KEY_NUMLOCK = 4194330, KEY_SCROLLLOCK = 4194331, KEY_F1 = 4194332, KEY_F2 = 4194333, KEY_F3 = 4194334, KEY_F4 = 4194335, KEY_F5 = 4194336, KEY_F6 = 4194337, KEY_F7 = 4194338, KEY_F8 = 4194339, KEY_F9 = 4194340, KEY_F10 = 4194341, KEY_F11 = 4194342, KEY_F12 = 4194343, KEY_F13 = 4194344, KEY_F14 = 4194345, KEY_F15 = 4194346, KEY_F16 = 4194347, KEY_F17 = 4194348, KEY_F18 = 4194349, KEY_F19 = 4194350, KEY_F20 = 4194351, KEY_F21 = 4194352, KEY_F22 = 4194353, KEY_F23 = 4194354, KEY_F24 = 4194355, KEY_F25 = 4194356, KEY_F26 = 4194357, KEY_F27 = 4194358, KEY_F28 = 4194359, KEY_F29 = 4194360, KEY_F30 = 4194361, KEY_F31 = 4194362, KEY_F32 = 4194363, KEY_F33 = 4194364, KEY_F34 = 4194365, KEY_F35 = 4194366, KEY_KP_MULTIPLY = 4194433, KEY_KP_DIVIDE = 4194434, KEY_KP_SUBTRACT = 4194435, KEY_KP_PERIOD = 4194436, KEY_KP_ADD = 4194437, KEY_KP_0 = 4194438, KEY_KP_1 = 4194439, KEY_KP_2 = 4194440, KEY_KP_3 = 4194441, KEY_KP_4 = 4194442, KEY_KP_5 = 4194443, KEY_KP_6 = 4194444, KEY_KP_7 = 4194445, KEY_KP_8 = 4194446, KEY_KP_9 = 4194447, KEY_MENU = 4194370, KEY_HYPER = 4194371, KEY_HELP = 4194373, KEY_BACK = 4194376, KEY_FORWARD = 4194377, KEY_STOP = 4194378, KEY_REFRESH = 4194379, KEY_VOLUMEDOWN = 4194380, KEY_VOLUMEMUTE = 4194381, KEY_VOLUMEUP = 4194382, KEY_MEDIAPLAY = 4194388, KEY_MEDIASTOP = 4194389, KEY_MEDIAPREVIOUS = 4194390, KEY_MEDIANEXT = 4194391, KEY_MEDIARECORD = 4194392, KEY_HOMEPAGE = 4194393, KEY_FAVORITES = 4194394, KEY_SEARCH = 4194395, KEY_STANDBY = 4194396, KEY_OPENURL = 4194397, KEY_LAUNCHMAIL = 4194398, KEY_LAUNCHMEDIA = 4194399, KEY_LAUNCH0 = 4194400, KEY_LAUNCH1 = 4194401, KEY_LAUNCH2 = 4194402, KEY_LAUNCH3 = 4194403, KEY_LAUNCH4 = 4194404, KEY_LAUNCH5 = 4194405, KEY_LAUNCH6 = 4194406, KEY_LAUNCH7 = 4194407, KEY_LAUNCH8 = 4194408, KEY_LAUNCH9 = 4194409, KEY_LAUNCHA = 4194410, KEY_LAUNCHB = 4194411, KEY_LAUNCHC = 4194412, KEY_LAUNCHD = 4194413, KEY_LAUNCHE = 4194414, KEY_LAUNCHF = 4194415, KEY_GLOBE = 4194416, KEY_KEYBOARD = 4194417, KEY_JIS_EISU = 4194418, KEY_JIS_KANA = 4194419, KEY_UNKNOWN = 8388607, KEY_SPACE = 32, KEY_EXCLAM = 33, KEY_QUOTEDBL = 34, KEY_NUMBERSIGN = 35, KEY_DOLLAR = 36, KEY_PERCENT = 37, KEY_AMPERSAND = 38, KEY_APOSTROPHE = 39, KEY_PARENLEFT = 40, KEY_PARENRIGHT = 41, KEY_ASTERISK = 42, KEY_PLUS = 43, KEY_COMMA = 44, KEY_MINUS = 45, KEY_PERIOD = 46, KEY_SLASH = 47, KEY_0 = 48, KEY_1 = 49, KEY_2 = 50, KEY_3 = 51, KEY_4 = 52, KEY_5 = 53, KEY_6 = 54, KEY_7 = 55, KEY_8 = 56, KEY_9 = 57, KEY_COLON = 58, KEY_SEMICOLON = 59, KEY_LESS = 60, KEY_EQUAL = 61, KEY_GREATER = 62, KEY_QUESTION = 63, KEY_AT = 64, KEY_A = 65, KEY_B = 66, KEY_C = 67, KEY_D = 68, KEY_E = 69, KEY_F = 70, KEY_G = 71, KEY_H = 72, KEY_I = 73, KEY_J = 74, KEY_K = 75, KEY_L = 76, KEY_M = 77, KEY_N = 78, KEY_O = 79, KEY_P = 80, KEY_Q = 81, KEY_R = 82, KEY_S = 83, KEY_T = 84, KEY_U = 85, KEY_V = 86, KEY_W = 87, KEY_X = 88, KEY_Y = 89, KEY_Z = 90, KEY_BRACKETLEFT = 91, KEY_BACKSLASH = 92, KEY_BRACKETRIGHT = 93, KEY_ASCIICIRCUM = 94, KEY_UNDERSCORE = 95, KEY_QUOTELEFT = 96, KEY_BRACELEFT = 123, KEY_BAR = 124, KEY_BRACERIGHT = 125, KEY_ASCIITILDE = 126, KEY_YEN = 165, KEY_SECTION = 167 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KEY_NONE {#KEY_NONE}

```gdscript
KEY_NONE = 0
```

Enum value which doesn't correspond to any key. This is used to initialize Key properties with a generic state.

#### KEY_SPECIAL {#KEY_SPECIAL}

```gdscript
KEY_SPECIAL = 4194304
```

Keycodes with this bit applied are non-printable.

#### KEY_ESCAPE {#KEY_ESCAPE}

```gdscript
KEY_ESCAPE = 4194305
```

Escape key.

#### KEY_TAB {#KEY_TAB}

```gdscript
KEY_TAB = 4194306
```

Tab key.

#### KEY_BACKTAB {#KEY_BACKTAB}

```gdscript
KEY_BACKTAB = 4194307
```

Shift + Tab key.

#### KEY_BACKSPACE {#KEY_BACKSPACE}

```gdscript
KEY_BACKSPACE = 4194308
```

Backspace key.

#### KEY_ENTER {#KEY_ENTER}

```gdscript
KEY_ENTER = 4194309
```

Return key (on the main keyboard).

#### KEY_KP_ENTER {#KEY_KP_ENTER}

```gdscript
KEY_KP_ENTER = 4194310
```

Enter key on the numeric keypad.

#### KEY_INSERT {#KEY_INSERT}

```gdscript
KEY_INSERT = 4194311
```

Insert key.

#### KEY_DELETE {#KEY_DELETE}

```gdscript
KEY_DELETE = 4194312
```

Delete key.

#### KEY_PAUSE {#KEY_PAUSE}

```gdscript
KEY_PAUSE = 4194313
```

Pause key.

#### KEY_PRINT {#KEY_PRINT}

```gdscript
KEY_PRINT = 4194314
```

Print Screen key.

#### KEY_SYSREQ {#KEY_SYSREQ}

```gdscript
KEY_SYSREQ = 4194315
```

System Request key.

#### KEY_CLEAR {#KEY_CLEAR}

```gdscript
KEY_CLEAR = 4194316
```

Clear key.

#### KEY_HOME {#KEY_HOME}

```gdscript
KEY_HOME = 4194317
```

Home key.

#### KEY_END {#KEY_END}

```gdscript
KEY_END = 4194318
```

End key.

#### KEY_LEFT {#KEY_LEFT}

```gdscript
KEY_LEFT = 4194319
```

Left arrow key.

#### KEY_UP {#KEY_UP}

```gdscript
KEY_UP = 4194320
```

Up arrow key.

#### KEY_RIGHT {#KEY_RIGHT}

```gdscript
KEY_RIGHT = 4194321
```

Right arrow key.

#### KEY_DOWN {#KEY_DOWN}

```gdscript
KEY_DOWN = 4194322
```

Down arrow key.

#### KEY_PAGEUP {#KEY_PAGEUP}

```gdscript
KEY_PAGEUP = 4194323
```

Page Up key.

#### KEY_PAGEDOWN {#KEY_PAGEDOWN}

```gdscript
KEY_PAGEDOWN = 4194324
```

Page Down key.

#### KEY_SHIFT {#KEY_SHIFT}

```gdscript
KEY_SHIFT = 4194325
```

Shift key.

#### KEY_CTRL {#KEY_CTRL}

```gdscript
KEY_CTRL = 4194326
```

Control key.

#### KEY_META {#KEY_META}

```gdscript
KEY_META = 4194327
```

Meta key.

#### KEY_ALT {#KEY_ALT}

```gdscript
KEY_ALT = 4194328
```

Alt key.

#### KEY_CAPSLOCK {#KEY_CAPSLOCK}

```gdscript
KEY_CAPSLOCK = 4194329
```

Caps Lock key.

#### KEY_NUMLOCK {#KEY_NUMLOCK}

```gdscript
KEY_NUMLOCK = 4194330
```

Num Lock key.

#### KEY_SCROLLLOCK {#KEY_SCROLLLOCK}

```gdscript
KEY_SCROLLLOCK = 4194331
```

Scroll Lock key.

#### KEY_F1 {#KEY_F1}

```gdscript
KEY_F1 = 4194332
```

F1 key.

#### KEY_F2 {#KEY_F2}

```gdscript
KEY_F2 = 4194333
```

F2 key.

#### KEY_F3 {#KEY_F3}

```gdscript
KEY_F3 = 4194334
```

F3 key.

#### KEY_F4 {#KEY_F4}

```gdscript
KEY_F4 = 4194335
```

F4 key.

#### KEY_F5 {#KEY_F5}

```gdscript
KEY_F5 = 4194336
```

F5 key.

#### KEY_F6 {#KEY_F6}

```gdscript
KEY_F6 = 4194337
```

F6 key.

#### KEY_F7 {#KEY_F7}

```gdscript
KEY_F7 = 4194338
```

F7 key.

#### KEY_F8 {#KEY_F8}

```gdscript
KEY_F8 = 4194339
```

F8 key.

#### KEY_F9 {#KEY_F9}

```gdscript
KEY_F9 = 4194340
```

F9 key.

#### KEY_F10 {#KEY_F10}

```gdscript
KEY_F10 = 4194341
```

F10 key.

#### KEY_F11 {#KEY_F11}

```gdscript
KEY_F11 = 4194342
```

F11 key.

#### KEY_F12 {#KEY_F12}

```gdscript
KEY_F12 = 4194343
```

F12 key.

#### KEY_F13 {#KEY_F13}

```gdscript
KEY_F13 = 4194344
```

F13 key.

#### KEY_F14 {#KEY_F14}

```gdscript
KEY_F14 = 4194345
```

F14 key.

#### KEY_F15 {#KEY_F15}

```gdscript
KEY_F15 = 4194346
```

F15 key.

#### KEY_F16 {#KEY_F16}

```gdscript
KEY_F16 = 4194347
```

F16 key.

#### KEY_F17 {#KEY_F17}

```gdscript
KEY_F17 = 4194348
```

F17 key.

#### KEY_F18 {#KEY_F18}

```gdscript
KEY_F18 = 4194349
```

F18 key.

#### KEY_F19 {#KEY_F19}

```gdscript
KEY_F19 = 4194350
```

F19 key.

#### KEY_F20 {#KEY_F20}

```gdscript
KEY_F20 = 4194351
```

F20 key.

#### KEY_F21 {#KEY_F21}

```gdscript
KEY_F21 = 4194352
```

F21 key.

#### KEY_F22 {#KEY_F22}

```gdscript
KEY_F22 = 4194353
```

F22 key.

#### KEY_F23 {#KEY_F23}

```gdscript
KEY_F23 = 4194354
```

F23 key.

#### KEY_F24 {#KEY_F24}

```gdscript
KEY_F24 = 4194355
```

F24 key.

#### KEY_F25 {#KEY_F25}

```gdscript
KEY_F25 = 4194356
```

F25 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F26 {#KEY_F26}

```gdscript
KEY_F26 = 4194357
```

F26 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F27 {#KEY_F27}

```gdscript
KEY_F27 = 4194358
```

F27 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F28 {#KEY_F28}

```gdscript
KEY_F28 = 4194359
```

F28 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F29 {#KEY_F29}

```gdscript
KEY_F29 = 4194360
```

F29 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F30 {#KEY_F30}

```gdscript
KEY_F30 = 4194361
```

F30 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F31 {#KEY_F31}

```gdscript
KEY_F31 = 4194362
```

F31 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F32 {#KEY_F32}

```gdscript
KEY_F32 = 4194363
```

F32 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F33 {#KEY_F33}

```gdscript
KEY_F33 = 4194364
```

F33 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F34 {#KEY_F34}

```gdscript
KEY_F34 = 4194365
```

F34 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_F35 {#KEY_F35}

```gdscript
KEY_F35 = 4194366
```

F35 key. Only supported on macOS and Linux due to a Windows limitation.

#### KEY_KP_MULTIPLY {#KEY_KP_MULTIPLY}

```gdscript
KEY_KP_MULTIPLY = 4194433
```

Multiply (*) key on the numeric keypad.

#### KEY_KP_DIVIDE {#KEY_KP_DIVIDE}

```gdscript
KEY_KP_DIVIDE = 4194434
```

Divide (/) key on the numeric keypad.

#### KEY_KP_SUBTRACT {#KEY_KP_SUBTRACT}

```gdscript
KEY_KP_SUBTRACT = 4194435
```

Subtract (-) key on the numeric keypad.

#### KEY_KP_PERIOD {#KEY_KP_PERIOD}

```gdscript
KEY_KP_PERIOD = 4194436
```

Period (.) key on the numeric keypad.

#### KEY_KP_ADD {#KEY_KP_ADD}

```gdscript
KEY_KP_ADD = 4194437
```

Add (+) key on the numeric keypad.

#### KEY_KP_0 {#KEY_KP_0}

```gdscript
KEY_KP_0 = 4194438
```

Number 0 on the numeric keypad.

#### KEY_KP_1 {#KEY_KP_1}

```gdscript
KEY_KP_1 = 4194439
```

Number 1 on the numeric keypad.

#### KEY_KP_2 {#KEY_KP_2}

```gdscript
KEY_KP_2 = 4194440
```

Number 2 on the numeric keypad.

#### KEY_KP_3 {#KEY_KP_3}

```gdscript
KEY_KP_3 = 4194441
```

Number 3 on the numeric keypad.

#### KEY_KP_4 {#KEY_KP_4}

```gdscript
KEY_KP_4 = 4194442
```

Number 4 on the numeric keypad.

#### KEY_KP_5 {#KEY_KP_5}

```gdscript
KEY_KP_5 = 4194443
```

Number 5 on the numeric keypad.

#### KEY_KP_6 {#KEY_KP_6}

```gdscript
KEY_KP_6 = 4194444
```

Number 6 on the numeric keypad.

#### KEY_KP_7 {#KEY_KP_7}

```gdscript
KEY_KP_7 = 4194445
```

Number 7 on the numeric keypad.

#### KEY_KP_8 {#KEY_KP_8}

```gdscript
KEY_KP_8 = 4194446
```

Number 8 on the numeric keypad.

#### KEY_KP_9 {#KEY_KP_9}

```gdscript
KEY_KP_9 = 4194447
```

Number 9 on the numeric keypad.

#### KEY_MENU {#KEY_MENU}

```gdscript
KEY_MENU = 4194370
```

Context menu key.

#### KEY_HYPER {#KEY_HYPER}

```gdscript
KEY_HYPER = 4194371
```

Hyper key. (On Linux/X11 only).

#### KEY_HELP {#KEY_HELP}

```gdscript
KEY_HELP = 4194373
```

Help key.

#### KEY_BACK {#KEY_BACK}

```gdscript
KEY_BACK = 4194376
```

Back key.

#### KEY_FORWARD {#KEY_FORWARD}

```gdscript
KEY_FORWARD = 4194377
```

Forward key.

#### KEY_STOP {#KEY_STOP}

```gdscript
KEY_STOP = 4194378
```

Media stop key.

#### KEY_REFRESH {#KEY_REFRESH}

```gdscript
KEY_REFRESH = 4194379
```

Refresh key.

#### KEY_VOLUMEDOWN {#KEY_VOLUMEDOWN}

```gdscript
KEY_VOLUMEDOWN = 4194380
```

Volume down key.

#### KEY_VOLUMEMUTE {#KEY_VOLUMEMUTE}

```gdscript
KEY_VOLUMEMUTE = 4194381
```

Mute volume key.

#### KEY_VOLUMEUP {#KEY_VOLUMEUP}

```gdscript
KEY_VOLUMEUP = 4194382
```

Volume up key.

#### KEY_MEDIAPLAY {#KEY_MEDIAPLAY}

```gdscript
KEY_MEDIAPLAY = 4194388
```

Media play key.

#### KEY_MEDIASTOP {#KEY_MEDIASTOP}

```gdscript
KEY_MEDIASTOP = 4194389
```

Media stop key.

#### KEY_MEDIAPREVIOUS {#KEY_MEDIAPREVIOUS}

```gdscript
KEY_MEDIAPREVIOUS = 4194390
```

Previous song key.

#### KEY_MEDIANEXT {#KEY_MEDIANEXT}

```gdscript
KEY_MEDIANEXT = 4194391
```

Next song key.

#### KEY_MEDIARECORD {#KEY_MEDIARECORD}

```gdscript
KEY_MEDIARECORD = 4194392
```

Media record key.

#### KEY_HOMEPAGE {#KEY_HOMEPAGE}

```gdscript
KEY_HOMEPAGE = 4194393
```

Home page key.

#### KEY_FAVORITES {#KEY_FAVORITES}

```gdscript
KEY_FAVORITES = 4194394
```

Favorites key.

#### KEY_SEARCH {#KEY_SEARCH}

```gdscript
KEY_SEARCH = 4194395
```

Search key.

#### KEY_STANDBY {#KEY_STANDBY}

```gdscript
KEY_STANDBY = 4194396
```

Standby key.

#### KEY_OPENURL {#KEY_OPENURL}

```gdscript
KEY_OPENURL = 4194397
```

Open URL / Launch Browser key.

#### KEY_LAUNCHMAIL {#KEY_LAUNCHMAIL}

```gdscript
KEY_LAUNCHMAIL = 4194398
```

Launch Mail key.

#### KEY_LAUNCHMEDIA {#KEY_LAUNCHMEDIA}

```gdscript
KEY_LAUNCHMEDIA = 4194399
```

Launch Media key.

#### KEY_LAUNCH0 {#KEY_LAUNCH0}

```gdscript
KEY_LAUNCH0 = 4194400
```

Launch Shortcut 0 key.

#### KEY_LAUNCH1 {#KEY_LAUNCH1}

```gdscript
KEY_LAUNCH1 = 4194401
```

Launch Shortcut 1 key.

#### KEY_LAUNCH2 {#KEY_LAUNCH2}

```gdscript
KEY_LAUNCH2 = 4194402
```

Launch Shortcut 2 key.

#### KEY_LAUNCH3 {#KEY_LAUNCH3}

```gdscript
KEY_LAUNCH3 = 4194403
```

Launch Shortcut 3 key.

#### KEY_LAUNCH4 {#KEY_LAUNCH4}

```gdscript
KEY_LAUNCH4 = 4194404
```

Launch Shortcut 4 key.

#### KEY_LAUNCH5 {#KEY_LAUNCH5}

```gdscript
KEY_LAUNCH5 = 4194405
```

Launch Shortcut 5 key.

#### KEY_LAUNCH6 {#KEY_LAUNCH6}

```gdscript
KEY_LAUNCH6 = 4194406
```

Launch Shortcut 6 key.

#### KEY_LAUNCH7 {#KEY_LAUNCH7}

```gdscript
KEY_LAUNCH7 = 4194407
```

Launch Shortcut 7 key.

#### KEY_LAUNCH8 {#KEY_LAUNCH8}

```gdscript
KEY_LAUNCH8 = 4194408
```

Launch Shortcut 8 key.

#### KEY_LAUNCH9 {#KEY_LAUNCH9}

```gdscript
KEY_LAUNCH9 = 4194409
```

Launch Shortcut 9 key.

#### KEY_LAUNCHA {#KEY_LAUNCHA}

```gdscript
KEY_LAUNCHA = 4194410
```

Launch Shortcut A key.

#### KEY_LAUNCHB {#KEY_LAUNCHB}

```gdscript
KEY_LAUNCHB = 4194411
```

Launch Shortcut B key.

#### KEY_LAUNCHC {#KEY_LAUNCHC}

```gdscript
KEY_LAUNCHC = 4194412
```

Launch Shortcut C key.

#### KEY_LAUNCHD {#KEY_LAUNCHD}

```gdscript
KEY_LAUNCHD = 4194413
```

Launch Shortcut D key.

#### KEY_LAUNCHE {#KEY_LAUNCHE}

```gdscript
KEY_LAUNCHE = 4194414
```

Launch Shortcut E key.

#### KEY_LAUNCHF {#KEY_LAUNCHF}

```gdscript
KEY_LAUNCHF = 4194415
```

Launch Shortcut F key.

#### KEY_GLOBE {#KEY_GLOBE}

```gdscript
KEY_GLOBE = 4194416
```

"Globe" key on Mac / iPad keyboard.

#### KEY_KEYBOARD {#KEY_KEYBOARD}

```gdscript
KEY_KEYBOARD = 4194417
```

"On-screen keyboard" key on iPad keyboard.

#### KEY_JIS_EISU {#KEY_JIS_EISU}

```gdscript
KEY_JIS_EISU = 4194418
```

英数 key on Mac keyboard.

#### KEY_JIS_KANA {#KEY_JIS_KANA}

```gdscript
KEY_JIS_KANA = 4194419
```

かな key on Mac keyboard.

#### KEY_UNKNOWN {#KEY_UNKNOWN}

```gdscript
KEY_UNKNOWN = 8388607
```

Unknown key.

#### KEY_SPACE {#KEY_SPACE}

```gdscript
KEY_SPACE = 32
```

Space key.

#### KEY_EXCLAM {#KEY_EXCLAM}

```gdscript
KEY_EXCLAM = 33
```

Exclamation mark (!) key.

#### KEY_QUOTEDBL {#KEY_QUOTEDBL}

```gdscript
KEY_QUOTEDBL = 34
```

Double quotation mark (") key.

#### KEY_NUMBERSIGN {#KEY_NUMBERSIGN}

```gdscript
KEY_NUMBERSIGN = 35
```

Number sign or hash (#) key.

#### KEY_DOLLAR {#KEY_DOLLAR}

```gdscript
KEY_DOLLAR = 36
```

Dollar sign ($) key.

#### KEY_PERCENT {#KEY_PERCENT}

```gdscript
KEY_PERCENT = 37
```

Percent sign (%) key.

#### KEY_AMPERSAND {#KEY_AMPERSAND}

```gdscript
KEY_AMPERSAND = 38
```

Ampersand (&) key.

#### KEY_APOSTROPHE {#KEY_APOSTROPHE}

```gdscript
KEY_APOSTROPHE = 39
```

Apostrophe (') key.

#### KEY_PARENLEFT {#KEY_PARENLEFT}

```gdscript
KEY_PARENLEFT = 40
```

Left parenthesis (() key.

#### KEY_PARENRIGHT {#KEY_PARENRIGHT}

```gdscript
KEY_PARENRIGHT = 41
```

Right parenthesis (``)``) key.

#### KEY_ASTERISK {#KEY_ASTERISK}

```gdscript
KEY_ASTERISK = 42
```

Asterisk (*) key.

#### KEY_PLUS {#KEY_PLUS}

```gdscript
KEY_PLUS = 43
```

Plus (+) key.

#### KEY_COMMA {#KEY_COMMA}

```gdscript
KEY_COMMA = 44
```

Comma (,) key.

#### KEY_MINUS {#KEY_MINUS}

```gdscript
KEY_MINUS = 45
```

Minus (-) key.

#### KEY_PERIOD {#KEY_PERIOD}

```gdscript
KEY_PERIOD = 46
```

Period (.) key.

#### KEY_SLASH {#KEY_SLASH}

```gdscript
KEY_SLASH = 47
```

Slash (/) key.

#### KEY_0 {#KEY_0}

```gdscript
KEY_0 = 48
```

Number 0 key.

#### KEY_1 {#KEY_1}

```gdscript
KEY_1 = 49
```

Number 1 key.

#### KEY_2 {#KEY_2}

```gdscript
KEY_2 = 50
```

Number 2 key.

#### KEY_3 {#KEY_3}

```gdscript
KEY_3 = 51
```

Number 3 key.

#### KEY_4 {#KEY_4}

```gdscript
KEY_4 = 52
```

Number 4 key.

#### KEY_5 {#KEY_5}

```gdscript
KEY_5 = 53
```

Number 5 key.

#### KEY_6 {#KEY_6}

```gdscript
KEY_6 = 54
```

Number 6 key.

#### KEY_7 {#KEY_7}

```gdscript
KEY_7 = 55
```

Number 7 key.

#### KEY_8 {#KEY_8}

```gdscript
KEY_8 = 56
```

Number 8 key.

#### KEY_9 {#KEY_9}

```gdscript
KEY_9 = 57
```

Number 9 key.

#### KEY_COLON {#KEY_COLON}

```gdscript
KEY_COLON = 58
```

Colon (:) key.

#### KEY_SEMICOLON {#KEY_SEMICOLON}

```gdscript
KEY_SEMICOLON = 59
```

Semicolon (;) key.

#### KEY_LESS {#KEY_LESS}

```gdscript
KEY_LESS = 60
```

Less-than sign (<) key.

#### KEY_EQUAL {#KEY_EQUAL}

```gdscript
KEY_EQUAL = 61
```

Equal sign (=) key.

#### KEY_GREATER {#KEY_GREATER}

```gdscript
KEY_GREATER = 62
```

Greater-than sign (>) key.

#### KEY_QUESTION {#KEY_QUESTION}

```gdscript
KEY_QUESTION = 63
```

Question mark (?) key.

#### KEY_AT {#KEY_AT}

```gdscript
KEY_AT = 64
```

At sign (@) key.

#### KEY_A {#KEY_A}

```gdscript
KEY_A = 65
```

A key.

#### KEY_B {#KEY_B}

```gdscript
KEY_B = 66
```

B key.

#### KEY_C {#KEY_C}

```gdscript
KEY_C = 67
```

C key.

#### KEY_D {#KEY_D}

```gdscript
KEY_D = 68
```

D key.

#### KEY_E {#KEY_E}

```gdscript
KEY_E = 69
```

E key.

#### KEY_F {#KEY_F}

```gdscript
KEY_F = 70
```

F key.

#### KEY_G {#KEY_G}

```gdscript
KEY_G = 71
```

G key.

#### KEY_H {#KEY_H}

```gdscript
KEY_H = 72
```

H key.

#### KEY_I {#KEY_I}

```gdscript
KEY_I = 73
```

I key.

#### KEY_J {#KEY_J}

```gdscript
KEY_J = 74
```

J key.

#### KEY_K {#KEY_K}

```gdscript
KEY_K = 75
```

K key.

#### KEY_L {#KEY_L}

```gdscript
KEY_L = 76
```

L key.

#### KEY_M {#KEY_M}

```gdscript
KEY_M = 77
```

M key.

#### KEY_N {#KEY_N}

```gdscript
KEY_N = 78
```

N key.

#### KEY_O {#KEY_O}

```gdscript
KEY_O = 79
```

O key.

#### KEY_P {#KEY_P}

```gdscript
KEY_P = 80
```

P key.

#### KEY_Q {#KEY_Q}

```gdscript
KEY_Q = 81
```

Q key.

#### KEY_R {#KEY_R}

```gdscript
KEY_R = 82
```

R key.

#### KEY_S {#KEY_S}

```gdscript
KEY_S = 83
```

S key.

#### KEY_T {#KEY_T}

```gdscript
KEY_T = 84
```

T key.

#### KEY_U {#KEY_U}

```gdscript
KEY_U = 85
```

U key.

#### KEY_V {#KEY_V}

```gdscript
KEY_V = 86
```

V key.

#### KEY_W {#KEY_W}

```gdscript
KEY_W = 87
```

W key.

#### KEY_X {#KEY_X}

```gdscript
KEY_X = 88
```

X key.

#### KEY_Y {#KEY_Y}

```gdscript
KEY_Y = 89
```

Y key.

#### KEY_Z {#KEY_Z}

```gdscript
KEY_Z = 90
```

Z key.

#### KEY_BRACKETLEFT {#KEY_BRACKETLEFT}

```gdscript
KEY_BRACKETLEFT = 91
```

Left bracket ([lb]) key.

#### KEY_BACKSLASH {#KEY_BACKSLASH}

```gdscript
KEY_BACKSLASH = 92
```

Backslash (\) key.

#### KEY_BRACKETRIGHT {#KEY_BRACKETRIGHT}

```gdscript
KEY_BRACKETRIGHT = 93
```

Right bracket ([rb]) key.

#### KEY_ASCIICIRCUM {#KEY_ASCIICIRCUM}

```gdscript
KEY_ASCIICIRCUM = 94
```

Caret (^) key.

#### KEY_UNDERSCORE {#KEY_UNDERSCORE}

```gdscript
KEY_UNDERSCORE = 95
```

Underscore (_) key.

#### KEY_QUOTELEFT {#KEY_QUOTELEFT}

```gdscript
KEY_QUOTELEFT = 96
```

Backtick (`) key.

#### KEY_BRACELEFT {#KEY_BRACELEFT}

```gdscript
KEY_BRACELEFT = 123
```

Left brace ({) key.

#### KEY_BAR {#KEY_BAR}

```gdscript
KEY_BAR = 124
```

Vertical bar or pipe (|) key.

#### KEY_BRACERIGHT {#KEY_BRACERIGHT}

```gdscript
KEY_BRACERIGHT = 125
```

Right brace (}) key.

#### KEY_ASCIITILDE {#KEY_ASCIITILDE}

```gdscript
KEY_ASCIITILDE = 126
```

Tilde (~) key.

#### KEY_YEN {#KEY_YEN}

```gdscript
KEY_YEN = 165
```

Yen symbol (¥) key.

#### KEY_SECTION {#KEY_SECTION}

```gdscript
KEY_SECTION = 167
```

Section sign (§) key.

---

flags KeyModifierMask:

### KeyModifierMask {#enum-KeyModifierMask}

```gdscript
enum KeyModifierMask（位标志） { KEY_CODE_MASK = 8388607, KEY_MODIFIER_MASK = 2130706432, KEY_MASK_CMD_OR_CTRL = 16777216, KEY_MASK_SHIFT = 33554432, KEY_MASK_ALT = 67108864, KEY_MASK_META = 134217728, KEY_MASK_CTRL = 268435456, KEY_MASK_KPAD = 536870912, KEY_MASK_GROUP_SWITCH = 1073741824 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KEY_CODE_MASK {#KEY_CODE_MASK}

```gdscript
KEY_CODE_MASK = 8388607
```

Bit mask with all bits enabled except for modifier keys. Apply it to remove modifiers.

```gdscript
var keycode = KEY_A | KEY_MASK_SHIFT
keycode = keycode & KEY_CODE_MASK
print(keycode) # KEY_A
```

#### KEY_MODIFIER_MASK {#KEY_MODIFIER_MASK}

```gdscript
KEY_MODIFIER_MASK = 2130706432
```

Bit mask with all modifier bits enabled. Apply it to isolate modifiers.

```gdscript
var keycode = KEY_A | KEY_MASK_SHIFT
keycode = keycode & KEY_MODIFIER_MASK
print(keycode) # KEY_MASK_SHIFT
```

#### KEY_MASK_CMD_OR_CTRL {#KEY_MASK_CMD_OR_CTRL}

```gdscript
KEY_MASK_CMD_OR_CTRL = 16777216
```

Automatically remapped to KEY_META on macOS and KEY_CTRL on other platforms, this mask is never set in the actual events, and should be used for key mapping only.

#### KEY_MASK_SHIFT {#KEY_MASK_SHIFT}

```gdscript
KEY_MASK_SHIFT = 33554432
```

Shift key mask.

#### KEY_MASK_ALT {#KEY_MASK_ALT}

```gdscript
KEY_MASK_ALT = 67108864
```

Alt or Option (on macOS) key mask.

#### KEY_MASK_META {#KEY_MASK_META}

```gdscript
KEY_MASK_META = 134217728
```

Command (on macOS) or Meta/Windows key mask.

#### KEY_MASK_CTRL {#KEY_MASK_CTRL}

```gdscript
KEY_MASK_CTRL = 268435456
```

Control key mask.

#### KEY_MASK_KPAD {#KEY_MASK_KPAD}

```gdscript
KEY_MASK_KPAD = 536870912
```

Keypad key mask.

#### KEY_MASK_GROUP_SWITCH {#KEY_MASK_GROUP_SWITCH}

```gdscript
KEY_MASK_GROUP_SWITCH = 1073741824
```

Group Switch key mask.

---

### KeyLocation {#enum-KeyLocation}

```gdscript
enum KeyLocation { KEY_LOCATION_UNSPECIFIED = 0, KEY_LOCATION_LEFT = 1, KEY_LOCATION_RIGHT = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### KEY_LOCATION_UNSPECIFIED {#KEY_LOCATION_UNSPECIFIED}

```gdscript
KEY_LOCATION_UNSPECIFIED = 0
```

Used for keys which only appear once, or when a comparison doesn't need to differentiate the LEFT and RIGHT versions.

For example, when using InputEvent.is_match(), an event which has KEY_LOCATION_UNSPECIFIED will match any KeyLocation on the passed event.

#### KEY_LOCATION_LEFT {#KEY_LOCATION_LEFT}

```gdscript
KEY_LOCATION_LEFT = 1
```

A key which is to the left of its twin.

#### KEY_LOCATION_RIGHT {#KEY_LOCATION_RIGHT}

```gdscript
KEY_LOCATION_RIGHT = 2
```

A key which is to the right of its twin.

---

### MouseButton {#enum-MouseButton}

```gdscript
enum MouseButton { MOUSE_BUTTON_NONE = 0, MOUSE_BUTTON_LEFT = 1, MOUSE_BUTTON_RIGHT = 2, MOUSE_BUTTON_MIDDLE = 3, MOUSE_BUTTON_WHEEL_UP = 4, MOUSE_BUTTON_WHEEL_DOWN = 5, MOUSE_BUTTON_WHEEL_LEFT = 6, MOUSE_BUTTON_WHEEL_RIGHT = 7, MOUSE_BUTTON_XBUTTON1 = 8, MOUSE_BUTTON_XBUTTON2 = 9 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MOUSE_BUTTON_NONE {#MOUSE_BUTTON_NONE}

```gdscript
MOUSE_BUTTON_NONE = 0
```

Enum value which doesn't correspond to any mouse button. This is used to initialize MouseButton properties with a generic state.

#### MOUSE_BUTTON_LEFT {#MOUSE_BUTTON_LEFT}

```gdscript
MOUSE_BUTTON_LEFT = 1
```

Primary mouse button, usually assigned to the left button.

#### MOUSE_BUTTON_RIGHT {#MOUSE_BUTTON_RIGHT}

```gdscript
MOUSE_BUTTON_RIGHT = 2
```

Secondary mouse button, usually assigned to the right button.

#### MOUSE_BUTTON_MIDDLE {#MOUSE_BUTTON_MIDDLE}

```gdscript
MOUSE_BUTTON_MIDDLE = 3
```

Middle mouse button.

#### MOUSE_BUTTON_WHEEL_UP {#MOUSE_BUTTON_WHEEL_UP}

```gdscript
MOUSE_BUTTON_WHEEL_UP = 4
```

Mouse wheel scrolling up.

#### MOUSE_BUTTON_WHEEL_DOWN {#MOUSE_BUTTON_WHEEL_DOWN}

```gdscript
MOUSE_BUTTON_WHEEL_DOWN = 5
```

Mouse wheel scrolling down.

#### MOUSE_BUTTON_WHEEL_LEFT {#MOUSE_BUTTON_WHEEL_LEFT}

```gdscript
MOUSE_BUTTON_WHEEL_LEFT = 6
```

Mouse wheel left button (only present on some mice).

#### MOUSE_BUTTON_WHEEL_RIGHT {#MOUSE_BUTTON_WHEEL_RIGHT}

```gdscript
MOUSE_BUTTON_WHEEL_RIGHT = 7
```

Mouse wheel right button (only present on some mice).

#### MOUSE_BUTTON_XBUTTON1 {#MOUSE_BUTTON_XBUTTON1}

```gdscript
MOUSE_BUTTON_XBUTTON1 = 8
```

Extra mouse button 1. This is sometimes present, usually to the sides of the mouse.

#### MOUSE_BUTTON_XBUTTON2 {#MOUSE_BUTTON_XBUTTON2}

```gdscript
MOUSE_BUTTON_XBUTTON2 = 9
```

Extra mouse button 2. This is sometimes present, usually to the sides of the mouse.

---

flags MouseButtonMask:

### MouseButtonMask {#enum-MouseButtonMask}

```gdscript
enum MouseButtonMask（位标志） { MOUSE_BUTTON_MASK_LEFT = 1, MOUSE_BUTTON_MASK_RIGHT = 2, MOUSE_BUTTON_MASK_MIDDLE = 4, MOUSE_BUTTON_MASK_MB_XBUTTON1 = 128, MOUSE_BUTTON_MASK_MB_XBUTTON2 = 256 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MOUSE_BUTTON_MASK_LEFT {#MOUSE_BUTTON_MASK_LEFT}

```gdscript
MOUSE_BUTTON_MASK_LEFT = 1
```

Primary mouse button mask, usually for the left button.

#### MOUSE_BUTTON_MASK_RIGHT {#MOUSE_BUTTON_MASK_RIGHT}

```gdscript
MOUSE_BUTTON_MASK_RIGHT = 2
```

Secondary mouse button mask, usually for the right button.

#### MOUSE_BUTTON_MASK_MIDDLE {#MOUSE_BUTTON_MASK_MIDDLE}

```gdscript
MOUSE_BUTTON_MASK_MIDDLE = 4
```

Middle mouse button mask.

#### MOUSE_BUTTON_MASK_MB_XBUTTON1 {#MOUSE_BUTTON_MASK_MB_XBUTTON1}

```gdscript
MOUSE_BUTTON_MASK_MB_XBUTTON1 = 128
```

Extra mouse button 1 mask.

#### MOUSE_BUTTON_MASK_MB_XBUTTON2 {#MOUSE_BUTTON_MASK_MB_XBUTTON2}

```gdscript
MOUSE_BUTTON_MASK_MB_XBUTTON2 = 256
```

Extra mouse button 2 mask.

---

### JoyButton {#enum-JoyButton}

```gdscript
enum JoyButton { JOY_BUTTON_INVALID = -1, JOY_BUTTON_A = 0, JOY_BUTTON_B = 1, JOY_BUTTON_X = 2, JOY_BUTTON_Y = 3, JOY_BUTTON_BACK = 4, JOY_BUTTON_GUIDE = 5, JOY_BUTTON_START = 6, JOY_BUTTON_LEFT_STICK = 7, JOY_BUTTON_RIGHT_STICK = 8, JOY_BUTTON_LEFT_SHOULDER = 9, JOY_BUTTON_RIGHT_SHOULDER = 10, JOY_BUTTON_DPAD_UP = 11, JOY_BUTTON_DPAD_DOWN = 12, JOY_BUTTON_DPAD_LEFT = 13, JOY_BUTTON_DPAD_RIGHT = 14, JOY_BUTTON_MISC1 = 15, JOY_BUTTON_PADDLE1 = 16, JOY_BUTTON_PADDLE2 = 17, JOY_BUTTON_PADDLE3 = 18, JOY_BUTTON_PADDLE4 = 19, JOY_BUTTON_TOUCHPAD = 20, JOY_BUTTON_MISC2 = 21, JOY_BUTTON_MISC3 = 22, JOY_BUTTON_MISC4 = 23, JOY_BUTTON_MISC5 = 24, JOY_BUTTON_MISC6 = 25, JOY_BUTTON_SDL_MAX = 26, JOY_BUTTON_MAX = 128 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOY_BUTTON_INVALID {#JOY_BUTTON_INVALID}

```gdscript
JOY_BUTTON_INVALID = -1
```

An invalid game controller button.

#### JOY_BUTTON_A {#JOY_BUTTON_A}

```gdscript
JOY_BUTTON_A = 0
```

Game controller SDL button A. Corresponds to the bottom action button: Sony Cross, Xbox A, Nintendo B.

#### JOY_BUTTON_B {#JOY_BUTTON_B}

```gdscript
JOY_BUTTON_B = 1
```

Game controller SDL button B. Corresponds to the right action button: Sony Circle, Xbox B, Nintendo A.

#### JOY_BUTTON_X {#JOY_BUTTON_X}

```gdscript
JOY_BUTTON_X = 2
```

Game controller SDL button X. Corresponds to the left action button: Sony Square, Xbox X, Nintendo Y.

#### JOY_BUTTON_Y {#JOY_BUTTON_Y}

```gdscript
JOY_BUTTON_Y = 3
```

Game controller SDL button Y. Corresponds to the top action button: Sony Triangle, Xbox Y, Nintendo X.

#### JOY_BUTTON_BACK {#JOY_BUTTON_BACK}

```gdscript
JOY_BUTTON_BACK = 4
```

Game controller SDL back button. Corresponds to the Sony Select, Xbox Back, Nintendo - button.

#### JOY_BUTTON_GUIDE {#JOY_BUTTON_GUIDE}

```gdscript
JOY_BUTTON_GUIDE = 5
```

Game controller SDL guide button. Corresponds to the Sony PS, Xbox Home button.

#### JOY_BUTTON_START {#JOY_BUTTON_START}

```gdscript
JOY_BUTTON_START = 6
```

Game controller SDL start button. Corresponds to the Sony Options, Xbox Menu, Nintendo + button.

#### JOY_BUTTON_LEFT_STICK {#JOY_BUTTON_LEFT_STICK}

```gdscript
JOY_BUTTON_LEFT_STICK = 7
```

Game controller SDL left stick button. Corresponds to the Sony L3, Xbox L/LS button.

#### JOY_BUTTON_RIGHT_STICK {#JOY_BUTTON_RIGHT_STICK}

```gdscript
JOY_BUTTON_RIGHT_STICK = 8
```

Game controller SDL right stick button. Corresponds to the Sony R3, Xbox R/RS button.

#### JOY_BUTTON_LEFT_SHOULDER {#JOY_BUTTON_LEFT_SHOULDER}

```gdscript
JOY_BUTTON_LEFT_SHOULDER = 9
```

Game controller SDL left shoulder button. Corresponds to the Sony L1, Xbox LB button.

#### JOY_BUTTON_RIGHT_SHOULDER {#JOY_BUTTON_RIGHT_SHOULDER}

```gdscript
JOY_BUTTON_RIGHT_SHOULDER = 10
```

Game controller SDL right shoulder button. Corresponds to the Sony R1, Xbox RB button.

#### JOY_BUTTON_DPAD_UP {#JOY_BUTTON_DPAD_UP}

```gdscript
JOY_BUTTON_DPAD_UP = 11
```

Game controller D-pad up button.

#### JOY_BUTTON_DPAD_DOWN {#JOY_BUTTON_DPAD_DOWN}

```gdscript
JOY_BUTTON_DPAD_DOWN = 12
```

Game controller D-pad down button.

#### JOY_BUTTON_DPAD_LEFT {#JOY_BUTTON_DPAD_LEFT}

```gdscript
JOY_BUTTON_DPAD_LEFT = 13
```

Game controller D-pad left button.

#### JOY_BUTTON_DPAD_RIGHT {#JOY_BUTTON_DPAD_RIGHT}

```gdscript
JOY_BUTTON_DPAD_RIGHT = 14
```

Game controller D-pad right button.

#### JOY_BUTTON_MISC1 {#JOY_BUTTON_MISC1}

```gdscript
JOY_BUTTON_MISC1 = 15
```

Game controller SDL miscellaneous button. Corresponds to Xbox share button, PS5 microphone button, Nintendo Switch capture button.

#### JOY_BUTTON_PADDLE1 {#JOY_BUTTON_PADDLE1}

```gdscript
JOY_BUTTON_PADDLE1 = 16
```

Game controller SDL paddle 1 button.

#### JOY_BUTTON_PADDLE2 {#JOY_BUTTON_PADDLE2}

```gdscript
JOY_BUTTON_PADDLE2 = 17
```

Game controller SDL paddle 2 button.

#### JOY_BUTTON_PADDLE3 {#JOY_BUTTON_PADDLE3}

```gdscript
JOY_BUTTON_PADDLE3 = 18
```

Game controller SDL paddle 3 button.

#### JOY_BUTTON_PADDLE4 {#JOY_BUTTON_PADDLE4}

```gdscript
JOY_BUTTON_PADDLE4 = 19
```

Game controller SDL paddle 4 button.

#### JOY_BUTTON_TOUCHPAD {#JOY_BUTTON_TOUCHPAD}

```gdscript
JOY_BUTTON_TOUCHPAD = 20
```

Game controller SDL touchpad button.

#### JOY_BUTTON_MISC2 {#JOY_BUTTON_MISC2}

```gdscript
JOY_BUTTON_MISC2 = 21
```

Game controller SDL miscellaneous button. Used by Nintendo Switch 2 Pro Controller and Horipad Steam controllers.

#### JOY_BUTTON_MISC3 {#JOY_BUTTON_MISC3}

```gdscript
JOY_BUTTON_MISC3 = 22
```

Game controller SDL miscellaneous button.

#### JOY_BUTTON_MISC4 {#JOY_BUTTON_MISC4}

```gdscript
JOY_BUTTON_MISC4 = 23
```

Game controller SDL miscellaneous button.

#### JOY_BUTTON_MISC5 {#JOY_BUTTON_MISC5}

```gdscript
JOY_BUTTON_MISC5 = 24
```

Game controller SDL miscellaneous button.

#### JOY_BUTTON_MISC6 {#JOY_BUTTON_MISC6}

```gdscript
JOY_BUTTON_MISC6 = 25
```

Game controller SDL miscellaneous button.

#### JOY_BUTTON_SDL_MAX {#JOY_BUTTON_SDL_MAX}

```gdscript
JOY_BUTTON_SDL_MAX = 26
```

The number of SDL game controller buttons.

#### JOY_BUTTON_MAX {#JOY_BUTTON_MAX}

```gdscript
JOY_BUTTON_MAX = 128
```

The maximum number of game controller buttons supported by the engine. The actual limit may be lower on specific platforms:

- Android: Up to 36 buttons.
- Linux: Up to 80 buttons.
- Windows and macOS: Up to 128 buttons.

---

### JoyAxis {#enum-JoyAxis}

```gdscript
enum JoyAxis { JOY_AXIS_INVALID = -1, JOY_AXIS_LEFT_X = 0, JOY_AXIS_LEFT_Y = 1, JOY_AXIS_RIGHT_X = 2, JOY_AXIS_RIGHT_Y = 3, JOY_AXIS_TRIGGER_LEFT = 4, JOY_AXIS_TRIGGER_RIGHT = 5, JOY_AXIS_SDL_MAX = 6, JOY_AXIS_MAX = 10 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### JOY_AXIS_INVALID {#JOY_AXIS_INVALID}

```gdscript
JOY_AXIS_INVALID = -1
```

An invalid game controller axis.

#### JOY_AXIS_LEFT_X {#JOY_AXIS_LEFT_X}

```gdscript
JOY_AXIS_LEFT_X = 0
```

Game controller left joystick x-axis.

#### JOY_AXIS_LEFT_Y {#JOY_AXIS_LEFT_Y}

```gdscript
JOY_AXIS_LEFT_Y = 1
```

Game controller left joystick y-axis.

#### JOY_AXIS_RIGHT_X {#JOY_AXIS_RIGHT_X}

```gdscript
JOY_AXIS_RIGHT_X = 2
```

Game controller right joystick x-axis.

#### JOY_AXIS_RIGHT_Y {#JOY_AXIS_RIGHT_Y}

```gdscript
JOY_AXIS_RIGHT_Y = 3
```

Game controller right joystick y-axis.

#### JOY_AXIS_TRIGGER_LEFT {#JOY_AXIS_TRIGGER_LEFT}

```gdscript
JOY_AXIS_TRIGGER_LEFT = 4
```

Game controller left trigger axis.

#### JOY_AXIS_TRIGGER_RIGHT {#JOY_AXIS_TRIGGER_RIGHT}

```gdscript
JOY_AXIS_TRIGGER_RIGHT = 5
```

Game controller right trigger axis.

#### JOY_AXIS_SDL_MAX {#JOY_AXIS_SDL_MAX}

```gdscript
JOY_AXIS_SDL_MAX = 6
```

The number of SDL game controller axes.

#### JOY_AXIS_MAX {#JOY_AXIS_MAX}

```gdscript
JOY_AXIS_MAX = 10
```

The maximum number of game controller axes: OpenVR supports up to 5 Joysticks making a total of 10 axes.

---

### MIDIMessage {#enum-MIDIMessage}

```gdscript
enum MIDIMessage { MIDI_MESSAGE_NONE = 0, MIDI_MESSAGE_NOTE_OFF = 8, MIDI_MESSAGE_NOTE_ON = 9, MIDI_MESSAGE_AFTERTOUCH = 10, MIDI_MESSAGE_CONTROL_CHANGE = 11, MIDI_MESSAGE_PROGRAM_CHANGE = 12, MIDI_MESSAGE_CHANNEL_PRESSURE = 13, MIDI_MESSAGE_PITCH_BEND = 14, MIDI_MESSAGE_SYSTEM_EXCLUSIVE = 240, MIDI_MESSAGE_QUARTER_FRAME = 241, MIDI_MESSAGE_SONG_POSITION_POINTER = 242, MIDI_MESSAGE_SONG_SELECT = 243, MIDI_MESSAGE_TUNE_REQUEST = 246, MIDI_MESSAGE_TIMING_CLOCK = 248, MIDI_MESSAGE_START = 250, MIDI_MESSAGE_CONTINUE = 251, MIDI_MESSAGE_STOP = 252, MIDI_MESSAGE_ACTIVE_SENSING = 254, MIDI_MESSAGE_SYSTEM_RESET = 255 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### MIDI_MESSAGE_NONE {#MIDI_MESSAGE_NONE}

```gdscript
MIDI_MESSAGE_NONE = 0
```

Does not correspond to any MIDI message. This is the default value of InputEventMIDI.message.

#### MIDI_MESSAGE_NOTE_OFF {#MIDI_MESSAGE_NOTE_OFF}

```gdscript
MIDI_MESSAGE_NOTE_OFF = 8
```

MIDI message sent when a note is released.

Note: Not all MIDI devices send this message; some may send MIDI_MESSAGE_NOTE_ON with InputEventMIDI.velocity set to 0.

#### MIDI_MESSAGE_NOTE_ON {#MIDI_MESSAGE_NOTE_ON}

```gdscript
MIDI_MESSAGE_NOTE_ON = 9
```

MIDI message sent when a note is pressed.

#### MIDI_MESSAGE_AFTERTOUCH {#MIDI_MESSAGE_AFTERTOUCH}

```gdscript
MIDI_MESSAGE_AFTERTOUCH = 10
```

MIDI message sent to indicate a change in pressure while a note is being pressed down, also called aftertouch.

#### MIDI_MESSAGE_CONTROL_CHANGE {#MIDI_MESSAGE_CONTROL_CHANGE}

```gdscript
MIDI_MESSAGE_CONTROL_CHANGE = 11
```

MIDI message sent when a controller value changes. In a MIDI device, a controller is any input that doesn't play notes. These may include sliders for volume, balance, and panning, as well as switches and pedals. See the General MIDI specification [https://en.wikipedia.org/wiki/General_MIDI#Controller_events] for a small list.

#### MIDI_MESSAGE_PROGRAM_CHANGE {#MIDI_MESSAGE_PROGRAM_CHANGE}

```gdscript
MIDI_MESSAGE_PROGRAM_CHANGE = 12
```

MIDI message sent when the MIDI device changes its current instrument (also called program or preset).

#### MIDI_MESSAGE_CHANNEL_PRESSURE {#MIDI_MESSAGE_CHANNEL_PRESSURE}

```gdscript
MIDI_MESSAGE_CHANNEL_PRESSURE = 13
```

MIDI message sent to indicate a change in pressure for the whole channel. Some MIDI devices may send this instead of MIDI_MESSAGE_AFTERTOUCH.

#### MIDI_MESSAGE_PITCH_BEND {#MIDI_MESSAGE_PITCH_BEND}

```gdscript
MIDI_MESSAGE_PITCH_BEND = 14
```

MIDI message sent when the value of the pitch bender changes, usually a wheel on the MIDI device.

#### MIDI_MESSAGE_SYSTEM_EXCLUSIVE {#MIDI_MESSAGE_SYSTEM_EXCLUSIVE}

```gdscript
MIDI_MESSAGE_SYSTEM_EXCLUSIVE = 240
```

MIDI system exclusive (SysEx) message. This type of message is not standardized and it's highly dependent on the MIDI device sending it.

Note: Getting this message's data from InputEventMIDI is not implemented.

#### MIDI_MESSAGE_QUARTER_FRAME {#MIDI_MESSAGE_QUARTER_FRAME}

```gdscript
MIDI_MESSAGE_QUARTER_FRAME = 241
```

MIDI message sent every quarter frame to keep connected MIDI devices synchronized. Related to MIDI_MESSAGE_TIMING_CLOCK.

Note: Getting this message's data from InputEventMIDI is not implemented.

#### MIDI_MESSAGE_SONG_POSITION_POINTER {#MIDI_MESSAGE_SONG_POSITION_POINTER}

```gdscript
MIDI_MESSAGE_SONG_POSITION_POINTER = 242
```

MIDI message sent to jump onto a new position in the current sequence or song.

Note: Getting this message's data from InputEventMIDI is not implemented.

#### MIDI_MESSAGE_SONG_SELECT {#MIDI_MESSAGE_SONG_SELECT}

```gdscript
MIDI_MESSAGE_SONG_SELECT = 243
```

MIDI message sent to select a sequence or song to play.

Note: Getting this message's data from InputEventMIDI is not implemented.

#### MIDI_MESSAGE_TUNE_REQUEST {#MIDI_MESSAGE_TUNE_REQUEST}

```gdscript
MIDI_MESSAGE_TUNE_REQUEST = 246
```

MIDI message sent to request a tuning calibration. Used on analog synthesizers. Most modern MIDI devices do not need this message.

#### MIDI_MESSAGE_TIMING_CLOCK {#MIDI_MESSAGE_TIMING_CLOCK}

```gdscript
MIDI_MESSAGE_TIMING_CLOCK = 248
```

MIDI message sent 24 times after MIDI_MESSAGE_QUARTER_FRAME, to keep connected MIDI devices synchronized.

#### MIDI_MESSAGE_START {#MIDI_MESSAGE_START}

```gdscript
MIDI_MESSAGE_START = 250
```

MIDI message sent to start the current sequence or song from the beginning.

#### MIDI_MESSAGE_CONTINUE {#MIDI_MESSAGE_CONTINUE}

```gdscript
MIDI_MESSAGE_CONTINUE = 251
```

MIDI message sent to resume from the point the current sequence or song was paused.

#### MIDI_MESSAGE_STOP {#MIDI_MESSAGE_STOP}

```gdscript
MIDI_MESSAGE_STOP = 252
```

MIDI message sent to pause the current sequence or song.

#### MIDI_MESSAGE_ACTIVE_SENSING {#MIDI_MESSAGE_ACTIVE_SENSING}

```gdscript
MIDI_MESSAGE_ACTIVE_SENSING = 254
```

MIDI message sent repeatedly while the MIDI device is idle, to tell the receiver that the connection is alive. Most MIDI devices do not send this message.

#### MIDI_MESSAGE_SYSTEM_RESET {#MIDI_MESSAGE_SYSTEM_RESET}

```gdscript
MIDI_MESSAGE_SYSTEM_RESET = 255
```

MIDI message sent to reset a MIDI device to its default state, as if it was just turned on. It should not be sent when the MIDI device is being turned on.

---

### Error {#enum-Error}

```gdscript
enum Error { OK = 0, FAILED = 1, ERR_UNAVAILABLE = 2, ERR_UNCONFIGURED = 3, ERR_UNAUTHORIZED = 4, ERR_PARAMETER_RANGE_ERROR = 5, ERR_OUT_OF_MEMORY = 6, ERR_FILE_NOT_FOUND = 7, ERR_FILE_BAD_DRIVE = 8, ERR_FILE_BAD_PATH = 9, ERR_FILE_NO_PERMISSION = 10, ERR_FILE_ALREADY_IN_USE = 11, ERR_FILE_CANT_OPEN = 12, ERR_FILE_CANT_WRITE = 13, ERR_FILE_CANT_READ = 14, ERR_FILE_UNRECOGNIZED = 15, ERR_FILE_CORRUPT = 16, ERR_FILE_MISSING_DEPENDENCIES = 17, ERR_FILE_EOF = 18, ERR_CANT_OPEN = 19, ERR_CANT_CREATE = 20, ERR_QUERY_FAILED = 21, ERR_ALREADY_IN_USE = 22, ERR_LOCKED = 23, ERR_TIMEOUT = 24, ERR_CANT_CONNECT = 25, ERR_CANT_RESOLVE = 26, ERR_CONNECTION_ERROR = 27, ERR_CANT_ACQUIRE_RESOURCE = 28, ERR_CANT_FORK = 29, ERR_INVALID_DATA = 30, ERR_INVALID_PARAMETER = 31, ERR_ALREADY_EXISTS = 32, ERR_DOES_NOT_EXIST = 33, ERR_DATABASE_CANT_READ = 34, ERR_DATABASE_CANT_WRITE = 35, ERR_COMPILATION_FAILED = 36, ERR_METHOD_NOT_FOUND = 37, ERR_LINK_FAILED = 38, ERR_SCRIPT_FAILED = 39, ERR_CYCLIC_LINK = 40, ERR_INVALID_DECLARATION = 41, ERR_DUPLICATE_SYMBOL = 42, ERR_PARSE_ERROR = 43, ERR_BUSY = 44, ERR_SKIP = 45, ERR_HELP = 46, ERR_BUG = 47, ERR_PRINTER_ON_FIRE = 48 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OK {#OK}

```gdscript
OK = 0
```

Methods that return Error return OK when no error occurred.

Since OK has value 0, and all other error constants are positive integers, it can also be used in boolean checks.

```gdscript
var error = method_that_returns_error()
if error != OK:
    printerr("Failure!")
# Or, alternatively:
if error:
    printerr("Still failing!")
```

Note: Many functions do not return an error code, but will print error messages to standard output.

#### FAILED {#FAILED}

```gdscript
FAILED = 1
```

Generic error.

#### ERR_UNAVAILABLE {#ERR_UNAVAILABLE}

```gdscript
ERR_UNAVAILABLE = 2
```

Unavailable error.

#### ERR_UNCONFIGURED {#ERR_UNCONFIGURED}

```gdscript
ERR_UNCONFIGURED = 3
```

Unconfigured error.

#### ERR_UNAUTHORIZED {#ERR_UNAUTHORIZED}

```gdscript
ERR_UNAUTHORIZED = 4
```

Unauthorized error.

#### ERR_PARAMETER_RANGE_ERROR {#ERR_PARAMETER_RANGE_ERROR}

```gdscript
ERR_PARAMETER_RANGE_ERROR = 5
```

Parameter range error.

#### ERR_OUT_OF_MEMORY {#ERR_OUT_OF_MEMORY}

```gdscript
ERR_OUT_OF_MEMORY = 6
```

Out of memory (OOM) error.

#### ERR_FILE_NOT_FOUND {#ERR_FILE_NOT_FOUND}

```gdscript
ERR_FILE_NOT_FOUND = 7
```

File: Not found error.

#### ERR_FILE_BAD_DRIVE {#ERR_FILE_BAD_DRIVE}

```gdscript
ERR_FILE_BAD_DRIVE = 8
```

File: Bad drive error.

#### ERR_FILE_BAD_PATH {#ERR_FILE_BAD_PATH}

```gdscript
ERR_FILE_BAD_PATH = 9
```

File: Bad path error.

#### ERR_FILE_NO_PERMISSION {#ERR_FILE_NO_PERMISSION}

```gdscript
ERR_FILE_NO_PERMISSION = 10
```

File: No permission error.

#### ERR_FILE_ALREADY_IN_USE {#ERR_FILE_ALREADY_IN_USE}

```gdscript
ERR_FILE_ALREADY_IN_USE = 11
```

File: Already in use error.

#### ERR_FILE_CANT_OPEN {#ERR_FILE_CANT_OPEN}

```gdscript
ERR_FILE_CANT_OPEN = 12
```

File: Can't open error.

#### ERR_FILE_CANT_WRITE {#ERR_FILE_CANT_WRITE}

```gdscript
ERR_FILE_CANT_WRITE = 13
```

File: Can't write error.

#### ERR_FILE_CANT_READ {#ERR_FILE_CANT_READ}

```gdscript
ERR_FILE_CANT_READ = 14
```

File: Can't read error.

#### ERR_FILE_UNRECOGNIZED {#ERR_FILE_UNRECOGNIZED}

```gdscript
ERR_FILE_UNRECOGNIZED = 15
```

File: Unrecognized error.

#### ERR_FILE_CORRUPT {#ERR_FILE_CORRUPT}

```gdscript
ERR_FILE_CORRUPT = 16
```

File: Corrupt error.

#### ERR_FILE_MISSING_DEPENDENCIES {#ERR_FILE_MISSING_DEPENDENCIES}

```gdscript
ERR_FILE_MISSING_DEPENDENCIES = 17
```

File: Missing dependencies error.

#### ERR_FILE_EOF {#ERR_FILE_EOF}

```gdscript
ERR_FILE_EOF = 18
```

File: End of file (EOF) error.

#### ERR_CANT_OPEN {#ERR_CANT_OPEN}

```gdscript
ERR_CANT_OPEN = 19
```

Can't open error.

#### ERR_CANT_CREATE {#ERR_CANT_CREATE}

```gdscript
ERR_CANT_CREATE = 20
```

Can't create error.

#### ERR_QUERY_FAILED {#ERR_QUERY_FAILED}

```gdscript
ERR_QUERY_FAILED = 21
```

Query failed error.

#### ERR_ALREADY_IN_USE {#ERR_ALREADY_IN_USE}

```gdscript
ERR_ALREADY_IN_USE = 22
```

Already in use error.

#### ERR_LOCKED {#ERR_LOCKED}

```gdscript
ERR_LOCKED = 23
```

Locked error.

#### ERR_TIMEOUT {#ERR_TIMEOUT}

```gdscript
ERR_TIMEOUT = 24
```

Timeout error.

#### ERR_CANT_CONNECT {#ERR_CANT_CONNECT}

```gdscript
ERR_CANT_CONNECT = 25
```

Can't connect error.

#### ERR_CANT_RESOLVE {#ERR_CANT_RESOLVE}

```gdscript
ERR_CANT_RESOLVE = 26
```

Can't resolve error.

#### ERR_CONNECTION_ERROR {#ERR_CONNECTION_ERROR}

```gdscript
ERR_CONNECTION_ERROR = 27
```

Connection error.

#### ERR_CANT_ACQUIRE_RESOURCE {#ERR_CANT_ACQUIRE_RESOURCE}

```gdscript
ERR_CANT_ACQUIRE_RESOURCE = 28
```

Can't acquire resource error.

#### ERR_CANT_FORK {#ERR_CANT_FORK}

```gdscript
ERR_CANT_FORK = 29
```

Can't fork process error.

#### ERR_INVALID_DATA {#ERR_INVALID_DATA}

```gdscript
ERR_INVALID_DATA = 30
```

Invalid data error.

#### ERR_INVALID_PARAMETER {#ERR_INVALID_PARAMETER}

```gdscript
ERR_INVALID_PARAMETER = 31
```

Invalid parameter error.

#### ERR_ALREADY_EXISTS {#ERR_ALREADY_EXISTS}

```gdscript
ERR_ALREADY_EXISTS = 32
```

Already exists error.

#### ERR_DOES_NOT_EXIST {#ERR_DOES_NOT_EXIST}

```gdscript
ERR_DOES_NOT_EXIST = 33
```

Does not exist error.

#### ERR_DATABASE_CANT_READ {#ERR_DATABASE_CANT_READ}

```gdscript
ERR_DATABASE_CANT_READ = 34
```

Database: Read error.

#### ERR_DATABASE_CANT_WRITE {#ERR_DATABASE_CANT_WRITE}

```gdscript
ERR_DATABASE_CANT_WRITE = 35
```

Database: Write error.

#### ERR_COMPILATION_FAILED {#ERR_COMPILATION_FAILED}

```gdscript
ERR_COMPILATION_FAILED = 36
```

Compilation failed error.

#### ERR_METHOD_NOT_FOUND {#ERR_METHOD_NOT_FOUND}

```gdscript
ERR_METHOD_NOT_FOUND = 37
```

Method not found error.

#### ERR_LINK_FAILED {#ERR_LINK_FAILED}

```gdscript
ERR_LINK_FAILED = 38
```

Linking failed error.

#### ERR_SCRIPT_FAILED {#ERR_SCRIPT_FAILED}

```gdscript
ERR_SCRIPT_FAILED = 39
```

Script failed error.

#### ERR_CYCLIC_LINK {#ERR_CYCLIC_LINK}

```gdscript
ERR_CYCLIC_LINK = 40
```

Cycling link (import cycle) error.

#### ERR_INVALID_DECLARATION {#ERR_INVALID_DECLARATION}

```gdscript
ERR_INVALID_DECLARATION = 41
```

Invalid declaration error.

#### ERR_DUPLICATE_SYMBOL {#ERR_DUPLICATE_SYMBOL}

```gdscript
ERR_DUPLICATE_SYMBOL = 42
```

Duplicate symbol error.

#### ERR_PARSE_ERROR {#ERR_PARSE_ERROR}

```gdscript
ERR_PARSE_ERROR = 43
```

Parse error.

#### ERR_BUSY {#ERR_BUSY}

```gdscript
ERR_BUSY = 44
```

Busy error.

#### ERR_SKIP {#ERR_SKIP}

```gdscript
ERR_SKIP = 45
```

Skip error.

#### ERR_HELP {#ERR_HELP}

```gdscript
ERR_HELP = 46
```

Help error. Used internally when passing --version or --help as executable options.

#### ERR_BUG {#ERR_BUG}

```gdscript
ERR_BUG = 47
```

Bug error, caused by an implementation issue in the method.

Note: If a built-in method returns this code, please open an issue on the GitHub Issue Tracker [https://github.com/godotengine/godot/issues].

#### ERR_PRINTER_ON_FIRE {#ERR_PRINTER_ON_FIRE}

```gdscript
ERR_PRINTER_ON_FIRE = 48
```

Printer on fire error (this is an easter egg, no built-in methods return this error code).

---

### PropertyHint {#enum-PropertyHint}

```gdscript
enum PropertyHint { PROPERTY_HINT_NONE = 0, PROPERTY_HINT_RANGE = 1, PROPERTY_HINT_ENUM = 2, PROPERTY_HINT_ENUM_SUGGESTION = 3, PROPERTY_HINT_EXP_EASING = 4, PROPERTY_HINT_LINK = 5, PROPERTY_HINT_FLAGS = 6, PROPERTY_HINT_LAYERS_2D_RENDER = 7, PROPERTY_HINT_LAYERS_2D_PHYSICS = 8, PROPERTY_HINT_LAYERS_2D_NAVIGATION = 9, PROPERTY_HINT_LAYERS_3D_RENDER = 10, PROPERTY_HINT_LAYERS_3D_PHYSICS = 11, PROPERTY_HINT_LAYERS_3D_NAVIGATION = 12, PROPERTY_HINT_LAYERS_AVOIDANCE = 37, PROPERTY_HINT_FILE = 13, PROPERTY_HINT_DIR = 14, PROPERTY_HINT_GLOBAL_FILE = 15, PROPERTY_HINT_GLOBAL_DIR = 16, PROPERTY_HINT_RESOURCE_TYPE = 17, PROPERTY_HINT_MULTILINE_TEXT = 18, PROPERTY_HINT_EXPRESSION = 19, PROPERTY_HINT_PLACEHOLDER_TEXT = 20, PROPERTY_HINT_COLOR_NO_ALPHA = 21, PROPERTY_HINT_OBJECT_ID = 22, PROPERTY_HINT_TYPE_STRING = 23, PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE = 24, PROPERTY_HINT_OBJECT_TOO_BIG = 25, PROPERTY_HINT_NODE_PATH_VALID_TYPES = 26, PROPERTY_HINT_SAVE_FILE = 27, PROPERTY_HINT_GLOBAL_SAVE_FILE = 28, PROPERTY_HINT_INT_IS_OBJECTID = 29, PROPERTY_HINT_INT_IS_POINTER = 30, PROPERTY_HINT_ARRAY_TYPE = 31, PROPERTY_HINT_DICTIONARY_TYPE = 38, PROPERTY_HINT_LOCALE_ID = 32, PROPERTY_HINT_LOCALIZABLE_STRING = 33, PROPERTY_HINT_NODE_TYPE = 34, PROPERTY_HINT_HIDE_QUATERNION_EDIT = 35, PROPERTY_HINT_PASSWORD = 36, PROPERTY_HINT_TOOL_BUTTON = 39, PROPERTY_HINT_ONESHOT = 40, PROPERTY_HINT_GROUP_ENABLE = 42, PROPERTY_HINT_INPUT_NAME = 43, PROPERTY_HINT_FILE_PATH = 44, PROPERTY_HINT_MAX = 45 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROPERTY_HINT_NONE {#PROPERTY_HINT_NONE}

```gdscript
PROPERTY_HINT_NONE = 0
```

The property has no hint for the editor. However, the hint string is still read, which can be used to specify a suffix for a property that has no range limit (see PROPERTY_HINT_RANGE's description).

#### PROPERTY_HINT_RANGE {#PROPERTY_HINT_RANGE}

```gdscript
PROPERTY_HINT_RANGE = 1
```

Hints that an int, float, or packed/typed Array property containing int or float types should be within a range specified via the hint string "min,max" or "min,max,step". The hint string can optionally include "or_greater" and/or "or_less" to allow manual input going respectively above the max or below the min values.

Example: "-360,360,1,or_greater,or_less".

Additionally, other keywords can be included: "exp" for exponential range editing, "radians_as_degrees" for editing radian angles in degrees (the range values are also in degrees), "degrees" to hint at an angle, "prefer_slider" to show the slider for integers, "hide_control" to hide the slider or up-down arrows, and "suffix:px/s" to display a suffix indicating the value's unit (e.g. px/s for pixels per second).

#### PROPERTY_HINT_ENUM {#PROPERTY_HINT_ENUM}

```gdscript
PROPERTY_HINT_ENUM = 2
```

Hints that an int, String, or StringName property is an enumerated value to pick in a list specified via a hint string.

The hint string is a comma separated list of names such as "Hello,Something,Else". Whitespace is not removed from either end of a name. For integer properties, the first name in the list has value 0, the next 1, and so on. Explicit values can also be specified by appending :integer to the name, e.g. "Zero,One,Three:3,Four,Six:6".

#### PROPERTY_HINT_ENUM_SUGGESTION {#PROPERTY_HINT_ENUM_SUGGESTION}

```gdscript
PROPERTY_HINT_ENUM_SUGGESTION = 3
```

Hints that a String or StringName property can be an enumerated value to pick in a list specified via a hint string such as "Hello,Something,Else". See PROPERTY_HINT_ENUM for details.

Unlike PROPERTY_HINT_ENUM, a property with this hint still accepts arbitrary values and can be empty. The list of values serves to suggest possible values.

#### PROPERTY_HINT_EXP_EASING {#PROPERTY_HINT_EXP_EASING}

```gdscript
PROPERTY_HINT_EXP_EASING = 4
```

Hints that a float property should be edited using a curve editor showing an exponential easing function. The hint string can include "attenuation" to flip the curve horizontally and/or "positive_only" to exclude in/out easing and limit values to be greater than or equal to zero. This displays differently to a property that uses PROPERTY_HINT_RANGE with the "exp" keyword, as it's edited with a slider instead of a curve editor.

#### PROPERTY_HINT_LINK {#PROPERTY_HINT_LINK}

```gdscript
PROPERTY_HINT_LINK = 5
```

Hints that a vector property should allow its components to be linked. For example, this allows Vector2.x and Vector2.y to be edited together. This hint is supported on Vector2, Vector2i, Vector3, Vector3i, Vector4, and Vector4i. The hint string can be used to specify a suffix indicating each value's unit with the "suffix:px/s" syntax.

#### PROPERTY_HINT_FLAGS {#PROPERTY_HINT_FLAGS}

```gdscript
PROPERTY_HINT_FLAGS = 6
```

Hints that an int property is a bitmask with named bit flags.

The hint string is a comma separated list of names such as "Bit0,Bit1,Bit2,Bit3". Whitespace is not removed from either end of a name. The first name in the list has value 1, the next 2, then 4, 8, 16 and so on. Explicit values can also be specified by appending :integer to the name, e.g. "A:4,B:8,C:16". You can also combine several flags ("A:4,B:8,AB:12,C:16").

Note: A flag value must be at least 1 and at most 2 ** 32 - 1.

Note: Unlike PROPERTY_HINT_ENUM, the previous explicit value is not taken into account. For the hint "A:16,B,C", A is 16, B is 2, C is 4.

#### PROPERTY_HINT_LAYERS_2D_RENDER {#PROPERTY_HINT_LAYERS_2D_RENDER}

```gdscript
PROPERTY_HINT_LAYERS_2D_RENDER = 7
```

Hints that an int property is a bitmask using the optionally named 2D render layers.

#### PROPERTY_HINT_LAYERS_2D_PHYSICS {#PROPERTY_HINT_LAYERS_2D_PHYSICS}

```gdscript
PROPERTY_HINT_LAYERS_2D_PHYSICS = 8
```

Hints that an int property is a bitmask using the optionally named 2D physics layers.

#### PROPERTY_HINT_LAYERS_2D_NAVIGATION {#PROPERTY_HINT_LAYERS_2D_NAVIGATION}

```gdscript
PROPERTY_HINT_LAYERS_2D_NAVIGATION = 9
```

Hints that an int property is a bitmask using the optionally named 2D navigation layers.

#### PROPERTY_HINT_LAYERS_3D_RENDER {#PROPERTY_HINT_LAYERS_3D_RENDER}

```gdscript
PROPERTY_HINT_LAYERS_3D_RENDER = 10
```

Hints that an int property is a bitmask using the optionally named 3D render layers.

#### PROPERTY_HINT_LAYERS_3D_PHYSICS {#PROPERTY_HINT_LAYERS_3D_PHYSICS}

```gdscript
PROPERTY_HINT_LAYERS_3D_PHYSICS = 11
```

Hints that an int property is a bitmask using the optionally named 3D physics layers.

#### PROPERTY_HINT_LAYERS_3D_NAVIGATION {#PROPERTY_HINT_LAYERS_3D_NAVIGATION}

```gdscript
PROPERTY_HINT_LAYERS_3D_NAVIGATION = 12
```

Hints that an int property is a bitmask using the optionally named 3D navigation layers.

#### PROPERTY_HINT_LAYERS_AVOIDANCE {#PROPERTY_HINT_LAYERS_AVOIDANCE}

```gdscript
PROPERTY_HINT_LAYERS_AVOIDANCE = 37
```

Hints that an integer property is a bitmask using the optionally named avoidance layers.

#### PROPERTY_HINT_FILE {#PROPERTY_HINT_FILE}

```gdscript
PROPERTY_HINT_FILE = 13
```

Hints that a String property is a path to a file. Editing it will show a file dialog for picking the path. The hint string can be a set of filters with wildcards like "*.png,*.jpg". By default the file will be stored as UID whenever available. You can use ResourceUID methods to convert it back to path. For storing a raw path, use PROPERTY_HINT_FILE_PATH.

#### PROPERTY_HINT_DIR {#PROPERTY_HINT_DIR}

```gdscript
PROPERTY_HINT_DIR = 14
```

Hints that a String property is a path to a directory. Editing it will show a file dialog for picking the path.

#### PROPERTY_HINT_GLOBAL_FILE {#PROPERTY_HINT_GLOBAL_FILE}

```gdscript
PROPERTY_HINT_GLOBAL_FILE = 15
```

Hints that a String property is an absolute path to a file outside the project folder. Editing it will show a file dialog for picking the path. The hint string can be a set of filters with wildcards, like "*.png,*.jpg".

#### PROPERTY_HINT_GLOBAL_DIR {#PROPERTY_HINT_GLOBAL_DIR}

```gdscript
PROPERTY_HINT_GLOBAL_DIR = 16
```

Hints that a String property is an absolute path to a directory outside the project folder. Editing it will show a file dialog for picking the path.

#### PROPERTY_HINT_RESOURCE_TYPE {#PROPERTY_HINT_RESOURCE_TYPE}

```gdscript
PROPERTY_HINT_RESOURCE_TYPE = 17
```

Hints that a property is an instance of a Resource-derived type, optionally specified via the hint string (e.g. "Texture2D"). Editing it will show a popup menu of valid resource types to instantiate.

#### PROPERTY_HINT_MULTILINE_TEXT {#PROPERTY_HINT_MULTILINE_TEXT}

```gdscript
PROPERTY_HINT_MULTILINE_TEXT = 18
```

Hints that a String property is text with line breaks. Editing it will show a text input field where line breaks can be typed.

The hint string can be set to "monospace" to force the input field to use a monospaced font.

If the hint string "no_wrap" is set, the input field will not wrap lines at boundaries, instead resorting to making the area scrollable.

#### PROPERTY_HINT_EXPRESSION {#PROPERTY_HINT_EXPRESSION}

```gdscript
PROPERTY_HINT_EXPRESSION = 19
```

Hints that a String property is an Expression.

#### PROPERTY_HINT_PLACEHOLDER_TEXT {#PROPERTY_HINT_PLACEHOLDER_TEXT}

```gdscript
PROPERTY_HINT_PLACEHOLDER_TEXT = 20
```

Hints that a String property should show a placeholder text on its input field, if empty. The hint string is the placeholder text to use.

#### PROPERTY_HINT_COLOR_NO_ALPHA {#PROPERTY_HINT_COLOR_NO_ALPHA}

```gdscript
PROPERTY_HINT_COLOR_NO_ALPHA = 21
```

Hints that a Color property should be edited without affecting its transparency (Color.a is not editable).

#### PROPERTY_HINT_OBJECT_ID {#PROPERTY_HINT_OBJECT_ID}

```gdscript
PROPERTY_HINT_OBJECT_ID = 22
```

Hints that the property's value is an object encoded as object ID, with its type specified in the hint string. Used by the debugger.

#### PROPERTY_HINT_TYPE_STRING {#PROPERTY_HINT_TYPE_STRING}

```gdscript
PROPERTY_HINT_TYPE_STRING = 23
```

If a property is String, hints that the property represents a particular type (class). This allows to select a type from the create dialog. The property will store the selected type as a string.

If a property is Array, hints the editor how to show elements. The hint_string must encode nested types using ":" and "/".

If a property is Dictionary, hints the editor how to show elements. The hint_string is the same as Array, with a ";" separating the key and value.

```
# Array of elem_type.
hint_string = "%d:" % [elem_type]
hint_string = "%d/%d:%s" % [elem_type, elem_hint, elem_hint_string]
# Two-dimensional array of elem_type (array of arrays of elem_type).
hint_string = "%d:%d:" % [TYPE_ARRAY, elem_type]
hint_string = "%d:%d/%d:%s" % [TYPE_ARRAY, elem_type, elem_hint, elem_hint_string]
# Three-dimensional array of elem_type (array of arrays of arrays of elem_type).
hint_string = "%d:%d:%d:" % [TYPE_ARRAY, TYPE_ARRAY, elem_type]
hint_string = "%d:%d:%d/%d:%s" % [TYPE_ARRAY, TYPE_ARRAY, elem_type, elem_hint, elem_hint_string]
```

```
// Array of elemType.
hintString = $"{elemType:D}:";
hintString = $"{elemType:D}/{elemHint:D}:{elemHintString}";
// Two-dimensional array of elemType (array of arrays of elemType).
hintString = $"{Variant.Type.Array:D}:{elemType:D}:";
hintString = $"{Variant.Type.Array:D}:{elemType:D}/{elemHint:D}:{elemHintString}";
// Three-dimensional array of elemType (array of arrays of arrays of elemType).
hintString = $"{Variant.Type.Array:D}:{Variant.Type.Array:D}:{elemType:D}:";
hintString = $"{Variant.Type.Array:D}:{Variant.Type.Array:D}:{elemType:D}/{elemHint:D}:{elemHintString}";
```

Examples:

```
hint_string = "%d:" % [TYPE_INT] # Array of integers.
hint_string = "%d/%d:1,10,1" % [TYPE_INT, PROPERTY_HINT_RANGE] # Array of integers (in range from 1 to 10).
hint_string = "%d/%d:Zero,One,Two" % [TYPE_INT, PROPERTY_HINT_ENUM] # Array of integers (an enum).
hint_string = "%d/%d:Zero,One,Three:3,Six:6" % [TYPE_INT, PROPERTY_HINT_ENUM] # Array of integers (an enum).
hint_string = "%d/%d:*.png" % [TYPE_STRING, PROPERTY_HINT_FILE] # Array of strings (file paths).
hint_string = "%d/%d:Texture2D" % [TYPE_OBJECT, PROPERTY_HINT_RESOURCE_TYPE] # Array of textures.
hint_string = "%d:%d:" % [TYPE_ARRAY, TYPE_FLOAT] # Two-dimensional array of floats.
hint_string = "%d:%d/%d:" % [TYPE_ARRAY, TYPE_STRING, PROPERTY_HINT_MULTILINE_TEXT] # Two-dimensional array of multiline strings.
hint_string = "%d:%d/%d:-1,1,0.1" % [TYPE_ARRAY, TYPE_FLOAT, PROPERTY_HINT_RANGE] # Two-dimensional array of floats (in range from -1 to 1).
hint_string = "%d:%d/%d:Texture2D" % [TYPE_ARRAY, TYPE_OBJECT, PROPERTY_HINT_RESOURCE_TYPE] # Two-dimensional array of textures.
```

```
hintString = $"{Variant.Type.Int:D}/{PropertyHint.Range:D}:1,10,1"; // Array of integers (in range from 1 to 10).
hintString = $"{Variant.Type.Int:D}/{PropertyHint.Enum:D}:Zero,One,Two"; // Array of integers (an enum).
hintString = $"{Variant.Type.Int:D}/{PropertyHint.Enum:D}:Zero,One,Three:3,Six:6"; // Array of integers (an enum).
hintString = $"{Variant.Type.String:D}/{PropertyHint.File:D}:*.png"; // Array of strings (file paths).
hintString = $"{Variant.Type.Object:D}/{PropertyHint.ResourceType:D}:Texture2D"; // Array of textures.
hintString = $"{Variant.Type.Array:D}:{Variant.Type.Float:D}:"; // Two-dimensional array of floats.
hintString = $"{Variant.Type.Array:D}:{Variant.Type.String:D}/{PropertyHint.MultilineText:D}:"; // Two-dimensional array of multiline strings.
hintString = $"{Variant.Type.Array:D}:{Variant.Type.Float:D}/{PropertyHint.Range:D}:-1,1,0.1"; // Two-dimensional array of floats (in range from -1 to 1).
hintString = $"{Variant.Type.Array:D}:{Variant.Type.Object:D}/{PropertyHint.ResourceType:D}:Texture2D"; // Two-dimensional array of textures.
```

Note: The trailing colon is required for properly detecting built-in types.

#### PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE {#PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE}

```gdscript
PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE = 24
```

Deprecated: This hint is not used by the engine.

#### PROPERTY_HINT_OBJECT_TOO_BIG {#PROPERTY_HINT_OBJECT_TOO_BIG}

```gdscript
PROPERTY_HINT_OBJECT_TOO_BIG = 25
```

Hints that an object is too big to be sent via the debugger.

#### PROPERTY_HINT_NODE_PATH_VALID_TYPES {#PROPERTY_HINT_NODE_PATH_VALID_TYPES}

```gdscript
PROPERTY_HINT_NODE_PATH_VALID_TYPES = 26
```

Hints that the hint string specifies valid node types for property of type NodePath.

#### PROPERTY_HINT_SAVE_FILE {#PROPERTY_HINT_SAVE_FILE}

```gdscript
PROPERTY_HINT_SAVE_FILE = 27
```

Hints that a String property is a path to a file. Editing it will show a file dialog for picking the path for the file to be saved at. The dialog has access to the project's directory. The hint string can be a set of filters with wildcards like "*.png,*.jpg". See also FileDialog.filters.

#### PROPERTY_HINT_GLOBAL_SAVE_FILE {#PROPERTY_HINT_GLOBAL_SAVE_FILE}

```gdscript
PROPERTY_HINT_GLOBAL_SAVE_FILE = 28
```

Hints that a String property is a path to a file. Editing it will show a file dialog for picking the path for the file to be saved at. The dialog has access to the entire filesystem. The hint string can be a set of filters with wildcards like "*.png,*.jpg". See also FileDialog.filters.

#### PROPERTY_HINT_INT_IS_OBJECTID {#PROPERTY_HINT_INT_IS_OBJECTID}

```gdscript
PROPERTY_HINT_INT_IS_OBJECTID = 29
```

Deprecated: This hint is not used by the engine.

#### PROPERTY_HINT_INT_IS_POINTER {#PROPERTY_HINT_INT_IS_POINTER}

```gdscript
PROPERTY_HINT_INT_IS_POINTER = 30
```

Hints that an int property is a pointer. Used by GDExtension.

#### PROPERTY_HINT_ARRAY_TYPE {#PROPERTY_HINT_ARRAY_TYPE}

```gdscript
PROPERTY_HINT_ARRAY_TYPE = 31
```

Hints that a property is an Array with the stored type specified in the hint string. The hint string contains the type of the array (e.g. "String").

Use the hint string format from PROPERTY_HINT_TYPE_STRING for more control over the stored type.

#### PROPERTY_HINT_DICTIONARY_TYPE {#PROPERTY_HINT_DICTIONARY_TYPE}

```gdscript
PROPERTY_HINT_DICTIONARY_TYPE = 38
```

Hints that a property is a Dictionary with the stored types specified in the hint string. The hint string contains the key and value types separated by a semicolon (e.g. "int;String").

Use the hint string format from PROPERTY_HINT_TYPE_STRING for more control over the stored types.

#### PROPERTY_HINT_LOCALE_ID {#PROPERTY_HINT_LOCALE_ID}

```gdscript
PROPERTY_HINT_LOCALE_ID = 32
```

Hints that a string property is a locale code. Editing it will show a locale dialog for picking language and country.

#### PROPERTY_HINT_LOCALIZABLE_STRING {#PROPERTY_HINT_LOCALIZABLE_STRING}

```gdscript
PROPERTY_HINT_LOCALIZABLE_STRING = 33
```

Hints that a dictionary property is string translation map. Dictionary keys are locale codes and, values are translated strings.

#### PROPERTY_HINT_NODE_TYPE {#PROPERTY_HINT_NODE_TYPE}

```gdscript
PROPERTY_HINT_NODE_TYPE = 34
```

Hints that a property is an instance of a Node-derived type, optionally specified via the hint string (e.g. "Node2D"). Editing it will show a dialog for picking a node from the scene.

#### PROPERTY_HINT_HIDE_QUATERNION_EDIT {#PROPERTY_HINT_HIDE_QUATERNION_EDIT}

```gdscript
PROPERTY_HINT_HIDE_QUATERNION_EDIT = 35
```

Hints that a quaternion property should disable the temporary euler editor.

#### PROPERTY_HINT_PASSWORD {#PROPERTY_HINT_PASSWORD}

```gdscript
PROPERTY_HINT_PASSWORD = 36
```

Hints that a string property is a password, and every character is replaced with the secret character.

#### PROPERTY_HINT_TOOL_BUTTON {#PROPERTY_HINT_TOOL_BUTTON}

```gdscript
PROPERTY_HINT_TOOL_BUTTON = 39
```

Hints that a Callable property should be displayed as a clickable button. When the button is pressed, the callable is called. The hint string specifies the button text and optionally an icon from the "EditorIcons" theme type.

```
"Click me!" - A button with the text "Click me!" and the default "Callable" icon.
"Click me!,ColorRect" - A button with the text "Click me!" and the "ColorRect" icon.
```

Note: A Callable cannot be properly serialized and stored in a file, so it is recommended to use PROPERTY_USAGE_EDITOR instead of PROPERTY_USAGE_DEFAULT.

#### PROPERTY_HINT_ONESHOT {#PROPERTY_HINT_ONESHOT}

```gdscript
PROPERTY_HINT_ONESHOT = 40
```

Hints that a property will be changed on its own after setting, such as AudioStreamPlayer.playing or GPUParticles3D.emitting.

#### PROPERTY_HINT_GROUP_ENABLE {#PROPERTY_HINT_GROUP_ENABLE}

```gdscript
PROPERTY_HINT_GROUP_ENABLE = 42
```

Hints that a boolean property will enable the feature associated with the group that it occurs in. The property will be displayed as a checkbox on the group header. Only works within a group or subgroup.

By default, disabling the property hides all properties in the group. Use the optional hint string "checkbox_only" to disable this behavior.

#### PROPERTY_HINT_INPUT_NAME {#PROPERTY_HINT_INPUT_NAME}

```gdscript
PROPERTY_HINT_INPUT_NAME = 43
```

Hints that a String or StringName property is the name of an input action. This allows the selection of any action name from the Input Map in the Project Settings. The hint string may contain two options separated by commas:

- If it contains "show_builtin", built-in input actions are included in the selection.
- If it contains "loose_mode", loose mode is enabled. This allows inserting any action name even if it's not present in the input map.

#### PROPERTY_HINT_FILE_PATH {#PROPERTY_HINT_FILE_PATH}

```gdscript
PROPERTY_HINT_FILE_PATH = 44
```

Like PROPERTY_HINT_FILE, but the property is stored as a raw path, not UID. That means the reference will be broken if you move the file. Consider using PROPERTY_HINT_FILE when possible.

#### PROPERTY_HINT_MAX {#PROPERTY_HINT_MAX}

```gdscript
PROPERTY_HINT_MAX = 45
```

Represents the size of the PropertyHint enum.

---

flags PropertyUsageFlags:

### PropertyUsageFlags {#enum-PropertyUsageFlags}

```gdscript
enum PropertyUsageFlags（位标志） { PROPERTY_USAGE_NONE = 0, PROPERTY_USAGE_STORAGE = 2, PROPERTY_USAGE_EDITOR = 4, PROPERTY_USAGE_INTERNAL = 8, PROPERTY_USAGE_CHECKABLE = 16, PROPERTY_USAGE_CHECKED = 32, PROPERTY_USAGE_GROUP = 64, PROPERTY_USAGE_CATEGORY = 128, PROPERTY_USAGE_SUBGROUP = 256, PROPERTY_USAGE_CLASS_IS_BITFIELD = 512, PROPERTY_USAGE_NO_INSTANCE_STATE = 1024, PROPERTY_USAGE_RESTART_IF_CHANGED = 2048, PROPERTY_USAGE_SCRIPT_VARIABLE = 4096, PROPERTY_USAGE_STORE_IF_NULL = 8192, PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED = 16384, PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE = 32768, PROPERTY_USAGE_CLASS_IS_ENUM = 65536, PROPERTY_USAGE_NIL_IS_VARIANT = 131072, PROPERTY_USAGE_ARRAY = 262144, PROPERTY_USAGE_ALWAYS_DUPLICATE = 524288, PROPERTY_USAGE_NEVER_DUPLICATE = 1048576, PROPERTY_USAGE_HIGH_END_GFX = 2097152, PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT = 4194304, PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT = 8388608, PROPERTY_USAGE_KEYING_INCREMENTS = 16777216, PROPERTY_USAGE_DEFERRED_SET_RESOURCE = 33554432, PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT = 67108864, PROPERTY_USAGE_EDITOR_BASIC_SETTING = 134217728, PROPERTY_USAGE_READ_ONLY = 268435456, PROPERTY_USAGE_SECRET = 536870912, PROPERTY_USAGE_DEFAULT = 6, PROPERTY_USAGE_NO_EDITOR = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PROPERTY_USAGE_NONE {#PROPERTY_USAGE_NONE}

```gdscript
PROPERTY_USAGE_NONE = 0
```

The property is not stored, and does not display in the editor. This is the default for non-exported properties.

#### PROPERTY_USAGE_STORAGE {#PROPERTY_USAGE_STORAGE}

```gdscript
PROPERTY_USAGE_STORAGE = 2
```

The property is serialized and saved in the scene file (default for exported properties).

#### PROPERTY_USAGE_EDITOR {#PROPERTY_USAGE_EDITOR}

```gdscript
PROPERTY_USAGE_EDITOR = 4
```

The property is shown in the EditorInspector (default for exported properties).

#### PROPERTY_USAGE_INTERNAL {#PROPERTY_USAGE_INTERNAL}

```gdscript
PROPERTY_USAGE_INTERNAL = 8
```

The property is excluded from the class reference.

#### PROPERTY_USAGE_CHECKABLE {#PROPERTY_USAGE_CHECKABLE}

```gdscript
PROPERTY_USAGE_CHECKABLE = 16
```

The property can be checked in the EditorInspector.

#### PROPERTY_USAGE_CHECKED {#PROPERTY_USAGE_CHECKED}

```gdscript
PROPERTY_USAGE_CHECKED = 32
```

The property is checked in the EditorInspector.

#### PROPERTY_USAGE_GROUP {#PROPERTY_USAGE_GROUP}

```gdscript
PROPERTY_USAGE_GROUP = 64
```

Used to group properties together in the editor. See EditorInspector.

#### PROPERTY_USAGE_CATEGORY {#PROPERTY_USAGE_CATEGORY}

```gdscript
PROPERTY_USAGE_CATEGORY = 128
```

Used to categorize properties together in the editor.

#### PROPERTY_USAGE_SUBGROUP {#PROPERTY_USAGE_SUBGROUP}

```gdscript
PROPERTY_USAGE_SUBGROUP = 256
```

Used to group properties together in the editor in a subgroup (under a group). See EditorInspector.

#### PROPERTY_USAGE_CLASS_IS_BITFIELD {#PROPERTY_USAGE_CLASS_IS_BITFIELD}

```gdscript
PROPERTY_USAGE_CLASS_IS_BITFIELD = 512
```

The property is a bitfield, i.e. it contains multiple flags represented as bits.

#### PROPERTY_USAGE_NO_INSTANCE_STATE {#PROPERTY_USAGE_NO_INSTANCE_STATE}

```gdscript
PROPERTY_USAGE_NO_INSTANCE_STATE = 1024
```

The property does not save its state in PackedScene.

#### PROPERTY_USAGE_RESTART_IF_CHANGED {#PROPERTY_USAGE_RESTART_IF_CHANGED}

```gdscript
PROPERTY_USAGE_RESTART_IF_CHANGED = 2048
```

Editing the property prompts the user for restarting the editor.

#### PROPERTY_USAGE_SCRIPT_VARIABLE {#PROPERTY_USAGE_SCRIPT_VARIABLE}

```gdscript
PROPERTY_USAGE_SCRIPT_VARIABLE = 4096
```

The property is a script variable. PROPERTY_USAGE_SCRIPT_VARIABLE can be used to distinguish between exported script variables from built-in variables (which don't have this usage flag). By default, PROPERTY_USAGE_SCRIPT_VARIABLE is not applied to variables that are created by overriding Object._get_property_list() in a script.

#### PROPERTY_USAGE_STORE_IF_NULL {#PROPERTY_USAGE_STORE_IF_NULL}

```gdscript
PROPERTY_USAGE_STORE_IF_NULL = 8192
```

The property value of type Object will be stored even if its value is null.

#### PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED {#PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED}

```gdscript
PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED = 16384
```

If this property is modified, all inspector fields will be refreshed.

#### PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE {#PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE}

```gdscript
PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE = 32768
```

Deprecated: This flag is not used by the engine.

#### PROPERTY_USAGE_CLASS_IS_ENUM {#PROPERTY_USAGE_CLASS_IS_ENUM}

```gdscript
PROPERTY_USAGE_CLASS_IS_ENUM = 65536
```

The property is a variable of enum type, i.e. it only takes named integer constants from its associated enumeration.

#### PROPERTY_USAGE_NIL_IS_VARIANT {#PROPERTY_USAGE_NIL_IS_VARIANT}

```gdscript
PROPERTY_USAGE_NIL_IS_VARIANT = 131072
```

If property has nil as default value, its type will be Variant.

#### PROPERTY_USAGE_ARRAY {#PROPERTY_USAGE_ARRAY}

```gdscript
PROPERTY_USAGE_ARRAY = 262144
```

The property is the element count of a property array, i.e. a list of groups of related properties. Properties defined with this usage also need a specific class_name field in the form of label,prefix. The field may also include additional comma-separated options:

- page_size=N: Overrides EditorSettings.interface/inspector/max_array_dictionary_items_per_page for this array.
- add_button_text=text: The text displayed by the "Add Element" button.
- static: The elements can't be re-arranged.
- const: New elements can't be added.
- numbered: An index will appear next to each element.
- unfoldable: The array can't be folded.
- swap_method=method_name: The method that will be called when two elements switch places. The method should take 2 int parameters, which will be indices of the elements being swapped.

Note that making a full-fledged property array requires boilerplate code involving Object._get_property_list().

#### PROPERTY_USAGE_ALWAYS_DUPLICATE {#PROPERTY_USAGE_ALWAYS_DUPLICATE}

```gdscript
PROPERTY_USAGE_ALWAYS_DUPLICATE = 524288
```

When duplicating a resource with Resource.duplicate(), and this flag is set on a property of that resource, the property should always be duplicated, regardless of the subresources bool parameter.

#### PROPERTY_USAGE_NEVER_DUPLICATE {#PROPERTY_USAGE_NEVER_DUPLICATE}

```gdscript
PROPERTY_USAGE_NEVER_DUPLICATE = 1048576
```

When duplicating a resource with Resource.duplicate(), and this flag is set on a property of that resource, the property should never be duplicated, regardless of the subresources bool parameter.

#### PROPERTY_USAGE_HIGH_END_GFX {#PROPERTY_USAGE_HIGH_END_GFX}

```gdscript
PROPERTY_USAGE_HIGH_END_GFX = 2097152
```

The property is only shown in the editor if modern renderers are supported (the Compatibility rendering method is excluded).

#### PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT {#PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT}

```gdscript
PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT = 4194304
```

The NodePath property will always be relative to the scene's root. Mostly useful for local resources.

#### PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT {#PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT}

```gdscript
PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT = 8388608
```

Use when a resource is created on the fly, i.e. the getter will always return a different instance. ResourceSaver needs this information to properly save such resources.

#### PROPERTY_USAGE_KEYING_INCREMENTS {#PROPERTY_USAGE_KEYING_INCREMENTS}

```gdscript
PROPERTY_USAGE_KEYING_INCREMENTS = 16777216
```

Inserting an animation key frame of this property will automatically increment the value, allowing to easily keyframe multiple values in a row.

#### PROPERTY_USAGE_DEFERRED_SET_RESOURCE {#PROPERTY_USAGE_DEFERRED_SET_RESOURCE}

```gdscript
PROPERTY_USAGE_DEFERRED_SET_RESOURCE = 33554432
```

Deprecated: This flag is not used by the engine.

#### PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT {#PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT}

```gdscript
PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT = 67108864
```

When this property is a Resource and base object is a Node, a resource instance will be automatically created whenever the node is created in the editor.

#### PROPERTY_USAGE_EDITOR_BASIC_SETTING {#PROPERTY_USAGE_EDITOR_BASIC_SETTING}

```gdscript
PROPERTY_USAGE_EDITOR_BASIC_SETTING = 134217728
```

The property is considered a basic setting and will appear even when advanced mode is disabled. Used for project settings.

#### PROPERTY_USAGE_READ_ONLY {#PROPERTY_USAGE_READ_ONLY}

```gdscript
PROPERTY_USAGE_READ_ONLY = 268435456
```

The property is read-only in the EditorInspector.

#### PROPERTY_USAGE_SECRET {#PROPERTY_USAGE_SECRET}

```gdscript
PROPERTY_USAGE_SECRET = 536870912
```

An export preset property with this flag contains confidential information and is stored separately from the rest of the export preset configuration.

#### PROPERTY_USAGE_DEFAULT {#PROPERTY_USAGE_DEFAULT}

```gdscript
PROPERTY_USAGE_DEFAULT = 6
```

Default usage (storage and editor).

#### PROPERTY_USAGE_NO_EDITOR {#PROPERTY_USAGE_NO_EDITOR}

```gdscript
PROPERTY_USAGE_NO_EDITOR = 2
```

Default usage but without showing the property in the editor (storage).

---

flags MethodFlags:

### MethodFlags {#enum-MethodFlags}

```gdscript
enum MethodFlags（位标志） { METHOD_FLAG_NORMAL = 1, METHOD_FLAG_EDITOR = 2, METHOD_FLAG_CONST = 4, METHOD_FLAG_VIRTUAL = 8, METHOD_FLAG_VARARG = 16, METHOD_FLAG_STATIC = 32, METHOD_FLAG_OBJECT_CORE = 64, METHOD_FLAG_VIRTUAL_REQUIRED = 128, METHOD_FLAGS_DEFAULT = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### METHOD_FLAG_NORMAL {#METHOD_FLAG_NORMAL}

```gdscript
METHOD_FLAG_NORMAL = 1
```

Flag for a normal method.

#### METHOD_FLAG_EDITOR {#METHOD_FLAG_EDITOR}

```gdscript
METHOD_FLAG_EDITOR = 2
```

Flag for an editor method.

#### METHOD_FLAG_CONST {#METHOD_FLAG_CONST}

```gdscript
METHOD_FLAG_CONST = 4
```

Flag for a constant method.

#### METHOD_FLAG_VIRTUAL {#METHOD_FLAG_VIRTUAL}

```gdscript
METHOD_FLAG_VIRTUAL = 8
```

Flag for a virtual method.

#### METHOD_FLAG_VARARG {#METHOD_FLAG_VARARG}

```gdscript
METHOD_FLAG_VARARG = 16
```

Flag for a method with a variable number of arguments.

#### METHOD_FLAG_STATIC {#METHOD_FLAG_STATIC}

```gdscript
METHOD_FLAG_STATIC = 32
```

Flag for a static method.

#### METHOD_FLAG_OBJECT_CORE {#METHOD_FLAG_OBJECT_CORE}

```gdscript
METHOD_FLAG_OBJECT_CORE = 64
```

Used internally. Allows to not dump core virtual methods (such as Object._notification()) to the JSON API.

#### METHOD_FLAG_VIRTUAL_REQUIRED {#METHOD_FLAG_VIRTUAL_REQUIRED}

```gdscript
METHOD_FLAG_VIRTUAL_REQUIRED = 128
```

Flag for a virtual method that is required. In GDScript, this flag is set for abstract functions.

#### METHOD_FLAGS_DEFAULT {#METHOD_FLAGS_DEFAULT}

```gdscript
METHOD_FLAGS_DEFAULT = 1
```

Default method flags (normal).

---

enum Variant.Type:

### Variant.Type {#enum-Variant.Type}

```gdscript
enum Variant.Type { TYPE_NIL = 0, TYPE_BOOL = 1, TYPE_INT = 2, TYPE_FLOAT = 3, TYPE_STRING = 4, TYPE_VECTOR2 = 5, TYPE_VECTOR2I = 6, TYPE_RECT2 = 7, TYPE_RECT2I = 8, TYPE_VECTOR3 = 9, TYPE_VECTOR3I = 10, TYPE_TRANSFORM2D = 11, TYPE_VECTOR4 = 12, TYPE_VECTOR4I = 13, TYPE_PLANE = 14, TYPE_QUATERNION = 15, TYPE_AABB = 16, TYPE_BASIS = 17, TYPE_TRANSFORM3D = 18, TYPE_PROJECTION = 19, TYPE_COLOR = 20, TYPE_STRING_NAME = 21, TYPE_NODE_PATH = 22, TYPE_RID = 23, TYPE_OBJECT = 24, TYPE_CALLABLE = 25, TYPE_SIGNAL = 26, TYPE_DICTIONARY = 27, TYPE_ARRAY = 28, TYPE_PACKED_BYTE_ARRAY = 29, TYPE_PACKED_INT32_ARRAY = 30, TYPE_PACKED_INT64_ARRAY = 31, TYPE_PACKED_FLOAT32_ARRAY = 32, TYPE_PACKED_FLOAT64_ARRAY = 33, TYPE_PACKED_STRING_ARRAY = 34, TYPE_PACKED_VECTOR2_ARRAY = 35, TYPE_PACKED_VECTOR3_ARRAY = 36, TYPE_PACKED_COLOR_ARRAY = 37, TYPE_PACKED_VECTOR4_ARRAY = 38, TYPE_MAX = 39 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_NIL {#TYPE_NIL}

```gdscript
TYPE_NIL = 0
```

Variable is null.

#### TYPE_BOOL {#TYPE_BOOL}

```gdscript
TYPE_BOOL = 1
```

Variable is of type bool.

#### TYPE_INT {#TYPE_INT}

```gdscript
TYPE_INT = 2
```

Variable is of type int.

#### TYPE_FLOAT {#TYPE_FLOAT}

```gdscript
TYPE_FLOAT = 3
```

Variable is of type float.

#### TYPE_STRING {#TYPE_STRING}

```gdscript
TYPE_STRING = 4
```

Variable is of type String.

#### TYPE_VECTOR2 {#TYPE_VECTOR2}

```gdscript
TYPE_VECTOR2 = 5
```

Variable is of type Vector2.

#### TYPE_VECTOR2I {#TYPE_VECTOR2I}

```gdscript
TYPE_VECTOR2I = 6
```

Variable is of type Vector2i.

#### TYPE_RECT2 {#TYPE_RECT2}

```gdscript
TYPE_RECT2 = 7
```

Variable is of type Rect2.

#### TYPE_RECT2I {#TYPE_RECT2I}

```gdscript
TYPE_RECT2I = 8
```

Variable is of type Rect2i.

#### TYPE_VECTOR3 {#TYPE_VECTOR3}

```gdscript
TYPE_VECTOR3 = 9
```

Variable is of type Vector3.

#### TYPE_VECTOR3I {#TYPE_VECTOR3I}

```gdscript
TYPE_VECTOR3I = 10
```

Variable is of type Vector3i.

#### TYPE_TRANSFORM2D {#TYPE_TRANSFORM2D}

```gdscript
TYPE_TRANSFORM2D = 11
```

Variable is of type Transform2D.

#### TYPE_VECTOR4 {#TYPE_VECTOR4}

```gdscript
TYPE_VECTOR4 = 12
```

Variable is of type Vector4.

#### TYPE_VECTOR4I {#TYPE_VECTOR4I}

```gdscript
TYPE_VECTOR4I = 13
```

Variable is of type Vector4i.

#### TYPE_PLANE {#TYPE_PLANE}

```gdscript
TYPE_PLANE = 14
```

Variable is of type Plane.

#### TYPE_QUATERNION {#TYPE_QUATERNION}

```gdscript
TYPE_QUATERNION = 15
```

Variable is of type Quaternion.

#### TYPE_AABB {#TYPE_AABB}

```gdscript
TYPE_AABB = 16
```

Variable is of type AABB.

#### TYPE_BASIS {#TYPE_BASIS}

```gdscript
TYPE_BASIS = 17
```

Variable is of type Basis.

#### TYPE_TRANSFORM3D {#TYPE_TRANSFORM3D}

```gdscript
TYPE_TRANSFORM3D = 18
```

Variable is of type Transform3D.

#### TYPE_PROJECTION {#TYPE_PROJECTION}

```gdscript
TYPE_PROJECTION = 19
```

Variable is of type Projection.

#### TYPE_COLOR {#TYPE_COLOR}

```gdscript
TYPE_COLOR = 20
```

Variable is of type Color.

#### TYPE_STRING_NAME {#TYPE_STRING_NAME}

```gdscript
TYPE_STRING_NAME = 21
```

Variable is of type StringName.

#### TYPE_NODE_PATH {#TYPE_NODE_PATH}

```gdscript
TYPE_NODE_PATH = 22
```

Variable is of type NodePath.

#### TYPE_RID {#TYPE_RID}

```gdscript
TYPE_RID = 23
```

Variable is of type RID.

#### TYPE_OBJECT {#TYPE_OBJECT}

```gdscript
TYPE_OBJECT = 24
```

Variable is of type Object.

#### TYPE_CALLABLE {#TYPE_CALLABLE}

```gdscript
TYPE_CALLABLE = 25
```

Variable is of type Callable.

#### TYPE_SIGNAL {#TYPE_SIGNAL}

```gdscript
TYPE_SIGNAL = 26
```

Variable is of type Signal.

#### TYPE_DICTIONARY {#TYPE_DICTIONARY}

```gdscript
TYPE_DICTIONARY = 27
```

Variable is of type Dictionary.

#### TYPE_ARRAY {#TYPE_ARRAY}

```gdscript
TYPE_ARRAY = 28
```

Variable is of type Array.

#### TYPE_PACKED_BYTE_ARRAY {#TYPE_PACKED_BYTE_ARRAY}

```gdscript
TYPE_PACKED_BYTE_ARRAY = 29
```

Variable is of type PackedByteArray.

#### TYPE_PACKED_INT32_ARRAY {#TYPE_PACKED_INT32_ARRAY}

```gdscript
TYPE_PACKED_INT32_ARRAY = 30
```

Variable is of type PackedInt32Array.

#### TYPE_PACKED_INT64_ARRAY {#TYPE_PACKED_INT64_ARRAY}

```gdscript
TYPE_PACKED_INT64_ARRAY = 31
```

Variable is of type PackedInt64Array.

#### TYPE_PACKED_FLOAT32_ARRAY {#TYPE_PACKED_FLOAT32_ARRAY}

```gdscript
TYPE_PACKED_FLOAT32_ARRAY = 32
```

Variable is of type PackedFloat32Array.

#### TYPE_PACKED_FLOAT64_ARRAY {#TYPE_PACKED_FLOAT64_ARRAY}

```gdscript
TYPE_PACKED_FLOAT64_ARRAY = 33
```

Variable is of type PackedFloat64Array.

#### TYPE_PACKED_STRING_ARRAY {#TYPE_PACKED_STRING_ARRAY}

```gdscript
TYPE_PACKED_STRING_ARRAY = 34
```

Variable is of type PackedStringArray.

#### TYPE_PACKED_VECTOR2_ARRAY {#TYPE_PACKED_VECTOR2_ARRAY}

```gdscript
TYPE_PACKED_VECTOR2_ARRAY = 35
```

Variable is of type PackedVector2Array.

#### TYPE_PACKED_VECTOR3_ARRAY {#TYPE_PACKED_VECTOR3_ARRAY}

```gdscript
TYPE_PACKED_VECTOR3_ARRAY = 36
```

Variable is of type PackedVector3Array.

#### TYPE_PACKED_COLOR_ARRAY {#TYPE_PACKED_COLOR_ARRAY}

```gdscript
TYPE_PACKED_COLOR_ARRAY = 37
```

Variable is of type PackedColorArray.

#### TYPE_PACKED_VECTOR4_ARRAY {#TYPE_PACKED_VECTOR4_ARRAY}

```gdscript
TYPE_PACKED_VECTOR4_ARRAY = 38
```

Variable is of type PackedVector4Array.

#### TYPE_MAX {#TYPE_MAX}

```gdscript
TYPE_MAX = 39
```

Represents the size of the Variant.Type enum.

---

enum Variant.Operator:

### Variant.Operator {#enum-Variant.Operator}

```gdscript
enum Variant.Operator { OP_EQUAL = 0, OP_NOT_EQUAL = 1, OP_LESS = 2, OP_LESS_EQUAL = 3, OP_GREATER = 4, OP_GREATER_EQUAL = 5, OP_ADD = 6, OP_SUBTRACT = 7, OP_MULTIPLY = 8, OP_DIVIDE = 9, OP_NEGATE = 10, OP_POSITIVE = 11, OP_MODULE = 12, OP_POWER = 13, OP_SHIFT_LEFT = 14, OP_SHIFT_RIGHT = 15, OP_BIT_AND = 16, OP_BIT_OR = 17, OP_BIT_XOR = 18, OP_BIT_NEGATE = 19, OP_AND = 20, OP_OR = 21, OP_XOR = 22, OP_NOT = 23, OP_IN = 24, OP_MAX = 25 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_EQUAL {#OP_EQUAL}

```gdscript
OP_EQUAL = 0
```

Equality operator (==).

#### OP_NOT_EQUAL {#OP_NOT_EQUAL}

```gdscript
OP_NOT_EQUAL = 1
```

Inequality operator (!=).

#### OP_LESS {#OP_LESS}

```gdscript
OP_LESS = 2
```

Less than operator (<).

#### OP_LESS_EQUAL {#OP_LESS_EQUAL}

```gdscript
OP_LESS_EQUAL = 3
```

Less than or equal operator (<=).

#### OP_GREATER {#OP_GREATER}

```gdscript
OP_GREATER = 4
```

Greater than operator (>).

#### OP_GREATER_EQUAL {#OP_GREATER_EQUAL}

```gdscript
OP_GREATER_EQUAL = 5
```

Greater than or equal operator (>=).

#### OP_ADD {#OP_ADD}

```gdscript
OP_ADD = 6
```

Addition operator (+).

#### OP_SUBTRACT {#OP_SUBTRACT}

```gdscript
OP_SUBTRACT = 7
```

Subtraction operator (-).

#### OP_MULTIPLY {#OP_MULTIPLY}

```gdscript
OP_MULTIPLY = 8
```

Multiplication operator (*).

#### OP_DIVIDE {#OP_DIVIDE}

```gdscript
OP_DIVIDE = 9
```

Division operator (/).

#### OP_NEGATE {#OP_NEGATE}

```gdscript
OP_NEGATE = 10
```

Unary negation operator (-).

#### OP_POSITIVE {#OP_POSITIVE}

```gdscript
OP_POSITIVE = 11
```

Unary plus operator (+).

#### OP_MODULE {#OP_MODULE}

```gdscript
OP_MODULE = 12
```

Remainder/modulo operator (%).

#### OP_POWER {#OP_POWER}

```gdscript
OP_POWER = 13
```

Power operator (**).

#### OP_SHIFT_LEFT {#OP_SHIFT_LEFT}

```gdscript
OP_SHIFT_LEFT = 14
```

Left shift operator (<<).

#### OP_SHIFT_RIGHT {#OP_SHIFT_RIGHT}

```gdscript
OP_SHIFT_RIGHT = 15
```

Right shift operator (>>).

#### OP_BIT_AND {#OP_BIT_AND}

```gdscript
OP_BIT_AND = 16
```

Bitwise AND operator (&).

#### OP_BIT_OR {#OP_BIT_OR}

```gdscript
OP_BIT_OR = 17
```

Bitwise OR operator (|).

#### OP_BIT_XOR {#OP_BIT_XOR}

```gdscript
OP_BIT_XOR = 18
```

Bitwise XOR operator (^).

#### OP_BIT_NEGATE {#OP_BIT_NEGATE}

```gdscript
OP_BIT_NEGATE = 19
```

Bitwise NOT operator (~).

#### OP_AND {#OP_AND}

```gdscript
OP_AND = 20
```

Logical AND operator (and or &&).

#### OP_OR {#OP_OR}

```gdscript
OP_OR = 21
```

Logical OR operator (or or ||).

#### OP_XOR {#OP_XOR}

```gdscript
OP_XOR = 22
```

Logical XOR operator (not implemented in GDScript).

#### OP_NOT {#OP_NOT}

```gdscript
OP_NOT = 23
```

Logical NOT operator (not or !).

#### OP_IN {#OP_IN}

```gdscript
OP_IN = 24
```

Logical IN operator (in).

#### OP_MAX {#OP_MAX}

```gdscript
OP_MAX = 25
```

Represents the size of the Variant.Operator enum.

---

## 全局常量

### UINT8_MAX {#UINT8_MAX}

```gdscript
const UINT8_MAX = 255
```

Maximum value of an 8-bit unsigned integer.

UINT16_MAX = 65535 

Maximum value of a 16-bit unsigned integer.

UINT32_MAX = 4294967295 

Maximum value of a 32-bit unsigned integer.

INT8_MIN = -128 

Minimum value of an 8-bit signed integer.

INT8_MAX = 127 

Maximum value of an 8-bit signed integer.

INT16_MIN = -32768 

Minimum value of a 16-bit signed integer.

INT16_MAX = 32767 

Maximum value of a 16-bit signed integer.

INT32_MIN = -2147483648 

Minimum value of a 32-bit signed integer.

INT32_MAX = 2147483647 

Maximum value of a 32-bit signed integer.

INT64_MIN = -9223372036854775808 

Minimum value of a 64-bit signed integer.

INT64_MAX = 9223372036854775807 

Maximum value of a 64-bit signed integer.

### UINT16_MAX {#UINT16_MAX}

```gdscript
const UINT16_MAX = 65535
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### UINT32_MAX {#UINT32_MAX}

```gdscript
const UINT32_MAX = 4294967295
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT8_MIN {#INT8_MIN}

```gdscript
const INT8_MIN = -128
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT8_MAX {#INT8_MAX}

```gdscript
const INT8_MAX = 127
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT16_MIN {#INT16_MIN}

```gdscript
const INT16_MIN = -32768
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT16_MAX {#INT16_MAX}

```gdscript
const INT16_MAX = 32767
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT32_MIN {#INT32_MIN}

```gdscript
const INT32_MIN = -2147483648
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT32_MAX {#INT32_MAX}

```gdscript
const INT32_MAX = 2147483647
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT64_MIN {#INT64_MIN}

```gdscript
const INT64_MIN = -9223372036854775808
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### INT64_MAX {#INT64_MAX}

```gdscript
const INT64_MAX = 9223372036854775807
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 单例

- `Performance` → [Performance](class_performance.md)
- `Engine` → [Engine](class_engine.md)
- `ProjectSettings` → [ProjectSettings](class_projectsettings.md)
- `OS` → [OS](class_os.md)
- `Time` → [Time](class_time.md)
- `ClassDB` → [ClassDB](class_classdb.md)
- `TextServerManager` → [TextServerManager](class_textservermanager.md)
- `NavigationServer2DManager` → [NavigationServer2DManager](class_navigationserver2dmanager.md)
- `PhysicsServer2DManager` → [PhysicsServer2DManager](class_physicsserver2dmanager.md)
- `NavigationServer3DManager` → [NavigationServer3DManager](class_navigationserver3dmanager.md)
- `PhysicsServer3DManager` → [PhysicsServer3DManager](class_physicsserver3dmanager.md)
- `NavigationMeshGenerator` → [NavigationMeshGenerator](class_navigationmeshgenerator.md)
- `IP` → [IP](class_ip.md)
- `Geometry2D` → [Geometry2D](class_geometry2d.md)
- `Geometry3D` → [Geometry3D](class_geometry3d.md)
- `ResourceLoader` → [ResourceLoader](class_resourceloader.md)
- `ResourceSaver` → [ResourceSaver](class_resourcesaver.md)
- `Marshalls` → [Marshalls](class_marshalls.md)
- `TranslationServer` → [TranslationServer](class_translationserver.md)
- `Input` → [Input](class_input.md)
- `InputMap` → [InputMap](class_inputmap.md)
- `EngineDebugger` → [EngineDebugger](class_enginedebugger.md)
- `GDExtensionManager` → [GDExtensionManager](class_gdextensionmanager.md)
- `ResourceUID` → [ResourceUID](class_resourceuid.md)
- `WorkerThreadPool` → [WorkerThreadPool](class_workerthreadpool.md)
- `ThemeDB` → [ThemeDB](class_themedb.md)
- `EditorInterface` → [EditorInterface](class_editorinterface.md)
- `GDScriptLanguageProtocol` → [GDScriptLanguageProtocol](class_gdscriptlanguageprotocol.md)
- `JavaClassWrapper` → [JavaClassWrapper](class_javaclasswrapper.md)
- `JavaScriptBridge` → [JavaScriptBridge](class_javascriptbridge.md)
- `AccessibilityServer` → [AccessibilityServer](class_accessibilityserver.md)
- `AudioServer` → [AudioServer](class_audioserver.md)
- `CameraServer` → [CameraServer](class_cameraserver.md)
- `DisplayServer` → [DisplayServer](class_displayserver.md)
- `NativeMenu` → [NativeMenu](class_nativemenu.md)
- `RenderingServer` → [RenderingServer](class_renderingserver.md)
- `NavigationServer2D` → [NavigationServer2D](class_navigationserver2d.md)
- `NavigationServer3D` → [NavigationServer3D](class_navigationserver3d.md)
- `PhysicsServer2D` → [PhysicsServer2D](class_physicsserver2d.md)
- `PhysicsServer3D` → [PhysicsServer3D](class_physicsserver3d.md)
- `XRServer` → [XRServer](class_xrserver.md)
