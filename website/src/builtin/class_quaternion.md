# Quaternion

> 四元数。用于旋转，避免万向节死锁。

<details><summary>官方英文描述</summary>

The Quaternion built-in Variant type is a 4D data structure that represents rotation in the form of a Hamilton convention quaternion [https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation]. Compared to the Basis type which can store both rotation and scale, quaternions can only store rotation.

A Quaternion is composed by 4 floating-point components: w, x, y, and z. These components are very compact in memory, and because of this some operations are more efficient and less likely to cause floating-point errors. Methods such as get_angle(), get_axis(), and slerp() are faster than their Basis counterparts.

For a great introduction to quaternions, see this video by 3Blue1Brown [https://www.youtube.com/watch?v=d4EgbgTm0Bg]. You do not need to know the math behind quaternions, as Godot provides several helper methods that handle it for you. These include slerp() and spherical_cubic_interpolate(), as well as the * operator.

Note: Quaternions must be normalized before being used for rotation (see normalized()).

Note: Similarly to Vector2 and Vector3, the components of a quaternion use 32-bit precision by default, unlike float which is always 64-bit. If double precision is needed, compile the engine with the option precision=double.

Note: In a boolean context, a quaternion will evaluate to false if it's equal to IDENTITY. Otherwise, a quaternion will always evaluate to true.

Note

There are notable differences when using this API with C#. See C# API differences to GDScript for more information.

</details>

## 方法

### angle_to {#angle_to}

```gdscript
float angle_to(to: Quaternion) const
```

Returns the angle between this quaternion and to. This is the magnitude of the angle you would need to rotate by to get from one to the other.

Note: The magnitude of the floating-point error for this method is abnormally high, so methods such as is_zero_approx will not work reliably.

### dot {#dot}

```gdscript
float dot(with: Quaternion) const
```

Returns the dot product between this quaternion and with.

This is equivalent to (quat.x * with.x) + (quat.y * with.y) + (quat.z * with.z) + (quat.w * with.w).

### exp {#exp}

```gdscript
Quaternion exp() const
```

Returns the exponential of this quaternion. The rotation axis of the result is the normalized rotation axis of this quaternion, the angle of the result is the length of the vector part of this quaternion.

### from_euler {#from_euler}

```gdscript
static Quaternion from_euler(euler: Vector3)
```

Constructs a new Quaternion from the given Vector3 of Euler angles [https://en.wikipedia.org/wiki/Euler_angles], in radians. In Godot, Euler angles always use intrinsic order. This method always uses the intrinsic YXZ convention (@GlobalScope.EULER_ORDER_YXZ).

### get_angle {#get_angle}

```gdscript
float get_angle() const
```

Returns the angle of the rotation represented by this quaternion.

Note: The quaternion must be normalized.

### get_axis {#get_axis}

```gdscript
Vector3 get_axis() const
```

Returns the rotation axis of the rotation represented by this quaternion.

### get_euler {#get_euler}

```gdscript
Vector3 get_euler(order: int = 2) const
```

Returns this quaternion's rotation as a Vector3 of Euler angles [https://en.wikipedia.org/wiki/Euler_angles], in radians.

The order of each consecutive rotation can be changed with order (see EulerOrder constants). In Godot, Euler angles always use intrinsic order. By default, the intrinsic YXZ convention is used (@GlobalScope.EULER_ORDER_YXZ): since we are decomposing, local Z (roll) is calculated first, then local X (pitch), and lastly local Y (yaw). When using the opposite method from_euler() to compose a rotation, this order is reversed.

### inverse {#inverse}

```gdscript
Quaternion inverse() const
```

Returns the inverse version of this quaternion, inverting the sign of every component except w.

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(to: Quaternion) const
```

Returns true if this quaternion and to are approximately equal, by calling @GlobalScope.is_equal_approx() on each component.

### is_finite {#is_finite}

```gdscript
bool is_finite() const
```

Returns true if this quaternion is finite, by calling @GlobalScope.is_finite() on each component.

### is_normalized {#is_normalized}

```gdscript
bool is_normalized() const
```

Returns true if this quaternion is normalized. See also normalized().

### length {#length}

```gdscript
float length() const
```

Returns this quaternion's length, also called magnitude.

### length_squared {#length_squared}

```gdscript
float length_squared() const
```

Returns this quaternion's length, squared.

Note: This method is faster than length(), so prefer it if you only need to compare quaternion lengths.

### log {#log}

```gdscript
Quaternion log() const
```

Returns the logarithm of this quaternion. Multiplies this quaternion's rotation axis by its rotation angle, and stores the result in the returned quaternion's vector part (x, y, and z). The returned quaternion's real part (w) is always 0.0.

### normalized {#normalized}

```gdscript
Quaternion normalized() const
```

Returns a copy of this quaternion, normalized so that its length is 1.0. See also is_normalized().

### slerp {#slerp}

```gdscript
Quaternion slerp(to: Quaternion, weight: float) const
```

Performs a spherical-linear interpolation with the to quaternion, given a weight and returns the result. Both this quaternion and to must be normalized.

### slerpni {#slerpni}

```gdscript
Quaternion slerpni(to: Quaternion, weight: float) const
```

Performs a spherical-linear interpolation with the to quaternion, given a weight and returns the result. Unlike slerp(), this method does not check if the rotation path is smaller than 90 degrees. Both this quaternion and to must be normalized.

### spherical_cubic_interpolate {#spherical_cubic_interpolate}

```gdscript
Quaternion spherical_cubic_interpolate(b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: float) const
```

Performs a spherical cubic interpolation between quaternions pre_a, this vector, b, and post_b, by the given amount weight.

### spherical_cubic_interpolate_in_time {#spherical_cubic_interpolate_in_time}

```gdscript
Quaternion spherical_cubic_interpolate_in_time(b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: float, b_t: float, pre_a_t: float, post_b_t: float) const
```

Performs a spherical cubic interpolation between quaternions pre_a, this vector, b, and post_b, by the given amount weight.

It can perform smoother interpolation than spherical_cubic_interpolate() by the time values.

## 属性

### x {#x}

```gdscript
x: float
```

X component of the quaternion. This is the value along the "imaginary" i axis.

Note: Quaternion components should usually not be manipulated directly.

### y {#y}

```gdscript
y: float
```

Y component of the quaternion. This is the value along the "imaginary" j axis.

Note: Quaternion components should usually not be manipulated directly.

### z {#z}

```gdscript
z: float
```

Z component of the quaternion. This is the value along the "imaginary" k axis.

Note: Quaternion components should usually not be manipulated directly.

### w {#w}

```gdscript
w: float
```

W component of the quaternion. This is the "real" part.

Note: Quaternion components should usually not be manipulated directly.

## 常量

### IDENTITY {#IDENTITY}

```gdscript
const IDENTITY = Quaternion(0, 0, 0, 1)
```

The identity quaternion, representing no rotation. This has the same rotation as Basis.IDENTITY.

If a Vector3 is rotated (multiplied) by this quaternion, it does not change.

Note: In GDScript, this constant is equivalent to creating a Quaternion without any arguments. It can be used to make your code clearer, and for consistency with C#.
