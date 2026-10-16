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

平面法线 X 分量。

### y {#y}

```gdscript
y: float
```

平面法线 Y 分量。

### z {#z}

```gdscript
z: float
```

平面法线 Z 分量。

### d {#d}

```gdscript
d: float
```

平面到原点的有符号距离。

### normal {#normal}

```gdscript
normal: Vector3
```

平面单位法线向量。

## 常量

### PLANE_YZ {#PLANE_YZ}

```gdscript
const PLANE_YZ = Plane(1, 0, 0, 0)
```

YZ 平面（法线沿 X）。

### PLANE_XZ {#PLANE_XZ}

```gdscript
const PLANE_XZ = Plane(0, 1, 0, 0)
```

XZ 平面（法线沿 Y）。

### PLANE_XY {#PLANE_XY}

```gdscript
const PLANE_XY = Plane(0, 0, 1, 0)
```

XY 平面（法线沿 Z）。
