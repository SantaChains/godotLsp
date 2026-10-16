# Vector2i

> A 2-element structure that can be used to represent 2D grid coordinates or any other pair of integers.

<details><summary>官方英文描述（中文精译进行中）</summary>

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

The vector's X component. Also accessible by using the index position [0].

### y {#y}

```gdscript
y: int
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
const ZERO = Vector2i(0, 0)
```

Zero vector, a vector with all components set to 0.

ONE = Vector2i(1, 1) 

One vector, a vector with all components set to 1.

MIN = Vector2i(-2147483648, -2147483648) 

Min vector, a vector with all components equal to INT32_MIN. Can be used as a negative integer equivalent of Vector2.INF.

MAX = Vector2i(2147483647, 2147483647) 

Max vector, a vector with all components equal to INT32_MAX. Can be used as an integer equivalent of Vector2.INF.

LEFT = Vector2i(-1, 0) 

Left unit vector. Represents the direction of left.

RIGHT = Vector2i(1, 0) 

Right unit vector. Represents the direction of right.

UP = Vector2i(0, -1) 

Up unit vector. Y is down in 2D, so this vector points -Y.

DOWN = Vector2i(0, 1) 

Down unit vector. Y is down in 2D, so this vector points +Y.

### ONE {#ONE}

```gdscript
const ONE = Vector2i(1, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MIN {#MIN}

```gdscript
const MIN = Vector2i(-2147483648, -2147483648)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### MAX {#MAX}

```gdscript
const MAX = Vector2i(2147483647, 2147483647)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### LEFT {#LEFT}

```gdscript
const LEFT = Vector2i(-1, 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### RIGHT {#RIGHT}

```gdscript
const RIGHT = Vector2i(1, 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### UP {#UP}

```gdscript
const UP = Vector2i(0, -1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### DOWN {#DOWN}

```gdscript
const DOWN = Vector2i(0, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
