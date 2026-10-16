# Vector4i

> A 4-element structure that can be used to represent 4D grid coordinates or any other quadruplet of integers.

<details><summary>官方英文描述</summary>

A 4-element structure that can be used to represent 4D grid coordinates or any other quadruplet of integers.

It uses integer coordinates and is therefore preferable to Vector4 when exact precision is required. Note that the values are limited to 32 bits, and unlike Vector4 this cannot be configured with an engine build option. Use int or PackedInt64Array if 64-bit values are needed.

Note: In a boolean context, a Vector4i will evaluate to false if it's equal to Vector4i(0, 0, 0, 0). Otherwise, a Vector4i will always evaluate to true.

</details>

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
