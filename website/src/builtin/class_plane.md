# Plane

> 三维平面。由法线和距离原点距离定义。

<details><summary>官方英文描述</summary>

Represents a normalized plane equation. normal is the normal of the plane (a, b, c normalized), and d is the distance from the origin to the plane (in the direction of "normal"). "Over" or "Above" the plane is considered the side of the plane towards where the normal is pointing.

Note: In a boolean context, a plane will evaluate to false if all its components equal 0. Otherwise, a plane will always evaluate to true.

</details>

## 方法

### distance_to {#distance_to}

```gdscript
float distance_to(point: Vector3) const
```

Returns the shortest distance from the plane to the position point. If the point is above the plane, the distance will be positive. If below, the distance will be negative.

### get_center {#get_center}

```gdscript
Vector3 get_center() const
```

Returns the center of the plane.

### has_point {#has_point}

```gdscript
bool has_point(point: Vector3, tolerance: float = 1e-05) const
```

Returns true if point is inside the plane. Comparison uses a custom minimum tolerance threshold.

### intersect_3 {#intersect_3}

```gdscript
Variant intersect_3(b: Plane, c: Plane) const
```

Returns the intersection point of the three planes b, c and this plane. If no intersection is found, null is returned.

### intersects_ray {#intersects_ray}

```gdscript
Variant intersects_ray(from: Vector3, dir: Vector3) const
```

Returns the intersection point of a ray consisting of the position from and the direction normal dir with this plane. If no intersection is found, null is returned.

### intersects_segment {#intersects_segment}

```gdscript
Variant intersects_segment(from: Vector3, to: Vector3) const
```

Returns the intersection point of a segment from position from to position to with this plane. If no intersection is found, null is returned.

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(to_plane: Plane) const
```

Returns true if this plane and to_plane are approximately equal, by running @GlobalScope.is_equal_approx() on each component.

### is_finite {#is_finite}

```gdscript
bool is_finite() const
```

Returns true if this plane is finite, by calling @GlobalScope.is_finite() on each component.

### is_point_over {#is_point_over}

```gdscript
bool is_point_over(point: Vector3) const
```

Returns true if point is located above the plane.

### normalized {#normalized}

```gdscript
Plane normalized() const
```

Returns a copy of the plane, with normalized normal (so it's a unit vector). Returns Plane(0, 0, 0, 0) if normal can't be normalized (it has zero length).

### project {#project}

```gdscript
Vector3 project(point: Vector3) const
```

Returns the orthogonal projection of point into a point in the plane.

## 属性

### x {#x}

```gdscript
x: float
```

The X component of the plane's normal vector.

### y {#y}

```gdscript
y: float
```

The Y component of the plane's normal vector.

### z {#z}

```gdscript
z: float
```

The Z component of the plane's normal vector.

### d {#d}

```gdscript
d: float
```

The distance from the origin to the plane, expressed in terms of normal (according to its direction and magnitude). Actual absolute distance from the origin to the plane can be calculated as abs(d) / normal.length() (if normal has zero length then this Plane does not represent a valid plane).

In the scalar equation of the plane ax + by + cz = d, this is d, while the (a, b, c) coordinates are represented by the normal property.

### normal {#normal}

```gdscript
normal: Vector3
```

The normal of the plane, typically a unit vector. Shouldn't be a zero vector as Plane with such normal does not represent a valid plane.

In the scalar equation of the plane ax + by + cz = d, this is the vector (a, b, c), where d is the d property.

## 常量

### PLANE_YZ {#PLANE_YZ}

```gdscript
const PLANE_YZ = Plane(1, 0, 0, 0)
```

A plane that extends in the Y and Z axes (normal vector points +X).

PLANE_XZ = Plane(0, 1, 0, 0) 

A plane that extends in the X and Z axes (normal vector points +Y).

PLANE_XY = Plane(0, 0, 1, 0) 

A plane that extends in the X and Y axes (normal vector points +Z).

### PLANE_XZ {#PLANE_XZ}

```gdscript
const PLANE_XZ = Plane(0, 1, 0, 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### PLANE_XY {#PLANE_XY}

```gdscript
const PLANE_XY = Plane(0, 0, 1, 0)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
