# Geometry3D

**继承**：[Object](class_object.md) → **Geometry3D**

> Provides a set of helper functions to create geometric shapes, compute intersections between shapes, and process various other geometric operations in 3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

Provides a set of helper functions to create geometric shapes, compute intersections between shapes, and process various other geometric operations in 3D.

</details>

## 方法

### build_box_planes {#build_box_planes}

```gdscript
typedarray::Plane build_box_planes(extents: Vector3)
```

Returns an array with 6 Planes that describe the sides of a box centered at the origin. The box size is defined by extents, which represents one (positive) corner of the box (i.e. half its actual size).

### build_capsule_planes {#build_capsule_planes}

```gdscript
typedarray::Plane build_capsule_planes(radius: float, height: float, sides: int, lats: int, axis: Vector3.Axis = 2)
```

Returns an array of Planes closely bounding a faceted capsule centered at the origin with radius radius and height height. The parameter sides defines how many planes will be generated for the side part of the capsule, whereas lats gives the number of latitudinal steps at the bottom and top of the capsule. The parameter axis describes the axis along which the capsule is oriented (0 for X, 1 for Y, 2 for Z).

### build_cylinder_planes {#build_cylinder_planes}

```gdscript
typedarray::Plane build_cylinder_planes(radius: float, height: float, sides: int, axis: Vector3.Axis = 2)
```

Returns an array of Planes closely bounding a faceted cylinder centered at the origin with radius radius and height height. The parameter sides defines how many planes will be generated for the round part of the cylinder. The parameter axis describes the axis along which the cylinder is oriented (0 for X, 1 for Y, 2 for Z).

### clip_polygon {#clip_polygon}

```gdscript
PackedVector3Array clip_polygon(points: PackedVector3Array, plane: Plane)
```

Clips the polygon defined by the points in points against the plane and returns the points of the clipped polygon.

### compute_convex_mesh_points {#compute_convex_mesh_points}

```gdscript
PackedVector3Array compute_convex_mesh_points(planes: typedarray::Plane)
```

Calculates and returns all the vertex points of a convex shape defined by an array of planes.

### get_closest_point_to_segment {#get_closest_point_to_segment}

```gdscript
Vector3 get_closest_point_to_segment(point: Vector3, s1: Vector3, s2: Vector3)
```

Returns the 3D point on the 3D segment (s1, s2) that is closest to point. The returned point will always be inside the specified segment.

### get_closest_point_to_segment_uncapped {#get_closest_point_to_segment_uncapped}

```gdscript
Vector3 get_closest_point_to_segment_uncapped(point: Vector3, s1: Vector3, s2: Vector3)
```

Returns the 3D point on the 3D line defined by (s1, s2) that is closest to point. The returned point can be inside the segment (s1, s2) or outside of it, i.e. somewhere on the line extending from the segment.

### get_closest_points_between_segments {#get_closest_points_between_segments}

```gdscript
PackedVector3Array get_closest_points_between_segments(p1: Vector3, p2: Vector3, q1: Vector3, q2: Vector3)
```

Given the two 3D segments (p1, p2) and (q1, q2), finds those two points on the two segments that are closest to each other. Returns a PackedVector3Array that contains this point on (p1, p2) as well the accompanying point on (q1, q2).

### get_triangle_barycentric_coords {#get_triangle_barycentric_coords}

```gdscript
Vector3 get_triangle_barycentric_coords(point: Vector3, a: Vector3, b: Vector3, c: Vector3)
```

Returns a Vector3 containing weights based on how close a 3D position (point) is to a triangle's different vertices (a, b and c). This is useful for interpolating between the data of different vertices in a triangle. One example use case is using this to smoothly rotate over a mesh instead of relying solely on face normals.

Here is a more detailed explanation of barycentric coordinates. [https://en.wikipedia.org/wiki/Barycentric_coordinate_system]

### ray_intersects_triangle {#ray_intersects_triangle}

```gdscript
Variant ray_intersects_triangle(from: Vector3, dir: Vector3, a: Vector3, b: Vector3, c: Vector3)
```

Tests if the 3D ray starting at from with the direction of dir intersects the triangle specified by a, b and c. If yes, returns the point of intersection as Vector3. If no intersection takes place, returns null.

### segment_intersects_convex {#segment_intersects_convex}

```gdscript
PackedVector3Array segment_intersects_convex(from: Vector3, to: Vector3, planes: typedarray::Plane)
```

Given a convex hull defined though the Planes in the array planes, tests if the segment (from, to) intersects with that hull. If an intersection is found, returns a PackedVector3Array containing the point the intersection and the hull's normal. Otherwise, returns an empty array.

### segment_intersects_cylinder {#segment_intersects_cylinder}

```gdscript
PackedVector3Array segment_intersects_cylinder(from: Vector3, to: Vector3, height: float, radius: float)
```

Checks if the segment (from, to) intersects the cylinder with height height that is centered at the origin and has radius radius. If no, returns an empty PackedVector3Array. If an intersection takes place, the returned array contains the point of intersection and the cylinder's normal at the point of intersection.

### segment_intersects_sphere {#segment_intersects_sphere}

```gdscript
PackedVector3Array segment_intersects_sphere(from: Vector3, to: Vector3, sphere_position: Vector3, sphere_radius: float)
```

Checks if the segment (from, to) intersects the sphere that is located at sphere_position and has radius sphere_radius. If no, returns an empty PackedVector3Array. If yes, returns a PackedVector3Array containing the point of intersection and the sphere's normal at the point of intersection.

### segment_intersects_triangle {#segment_intersects_triangle}

```gdscript
Variant segment_intersects_triangle(from: Vector3, to: Vector3, a: Vector3, b: Vector3, c: Vector3)
```

Tests if the segment (from, to) intersects the triangle a, b, c. If yes, returns the point of intersection as Vector3. If no intersection takes place, returns null.

### tetrahedralize_delaunay {#tetrahedralize_delaunay}

```gdscript
PackedInt32Array tetrahedralize_delaunay(points: PackedVector3Array)
```

Tetrahedralizes the volume specified by a discrete set of points in 3D space, ensuring that no point lies within the circumsphere of any resulting tetrahedron. The method returns a PackedInt32Array where each tetrahedron consists of four consecutive point indices into the points array (resulting in an array with n * 4 elements, where n is the number of tetrahedra found). If the tetrahedralization is unsuccessful, an empty PackedInt32Array is returned.
