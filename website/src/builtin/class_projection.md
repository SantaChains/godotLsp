# Projection

> 4x4 投影矩阵。用于相机投影。

<details><summary>官方英文描述</summary>

A 4×4 matrix used for 3D projective transformations. It can represent transformations such as translation, rotation, scaling, shearing, and perspective division. It consists of four Vector4 columns.

For purely linear transformations (translation, rotation, and scale), it is recommended to use Transform3D, as it is more performant and requires less memory.

Used internally as Camera3D's projection matrix.

Note: In a boolean context, a projection will evaluate to false if it's equal to IDENTITY. Otherwise, a projection will always evaluate to true.

Note

There are notable differences when using this API with C#. See C# API differences to GDScript for more information.

</details>

## 构造函数

```gdscript
Projection()
```

Constructs a default-initialized Projection identical to IDENTITY.

Note: In C#, this constructs a Projection identical to ZERO.

```gdscript
Projection(from: Projection)
```

Constructs a Projection as a copy of the given Projection.

```gdscript
Projection(from: Transform3D)
```

Constructs a Projection as a copy of the given Transform3D.

```gdscript
Projection(x_axis: Vector4, y_axis: Vector4, z_axis: Vector4, w_axis: Vector4)
```

Constructs a Projection from four Vector4 values (matrix columns).

## 方法

### create_depth_correction {#create_depth_correction}

```gdscript
static Projection create_depth_correction(flip_y: bool)
```

Creates a new Projection that projects positions from a depth range of -1 to 1 to one that ranges from 0 to 1, and flips the projected positions vertically, according to flip_y.

### create_fit_aabb {#create_fit_aabb}

```gdscript
static Projection create_fit_aabb(aabb: AABB)
```

Creates a new Projection that scales a given projection to fit around a given AABB in projection space.

### create_for_hmd {#create_for_hmd}

```gdscript
static Projection create_for_hmd(eye: int, aspect: float, intraocular_dist: float, display_width: float, display_to_lens: float, oversample: float, z_near: float, z_far: float)
```

Creates a new Projection for projecting positions onto a head-mounted display with the given X:Y aspect ratio, distance between eyes, display width, distance to lens, oversampling factor, and depth clipping planes.

eye creates the projection for the left eye when set to 1, or the right eye when set to 2.

### create_frustum {#create_frustum}

```gdscript
static Projection create_frustum(left: float, right: float, bottom: float, top: float, z_near: float, z_far: float)
```

Creates a new Projection that projects positions in a frustum with the given clipping planes.

### create_frustum_aspect {#create_frustum_aspect}

```gdscript
static Projection create_frustum_aspect(size: float, aspect: float, offset: Vector2, z_near: float, z_far: float, flip_fov: bool = false)
```

Creates a new Projection that projects positions in a frustum with the given size, X:Y aspect ratio, offset, and clipping planes.

flip_fov determines whether the projection's field of view is flipped over its diagonal.

### create_light_atlas_rect {#create_light_atlas_rect}

```gdscript
static Projection create_light_atlas_rect(rect: Rect2)
```

Creates a new Projection that projects positions into the given Rect2.

### create_orthogonal {#create_orthogonal}

```gdscript
static Projection create_orthogonal(left: float, right: float, bottom: float, top: float, z_near: float, z_far: float)
```

Creates a new Projection that projects positions using an orthogonal projection with the given clipping planes.

### create_orthogonal_aspect {#create_orthogonal_aspect}

```gdscript
static Projection create_orthogonal_aspect(size: float, aspect: float, z_near: float, z_far: float, flip_fov: bool = false)
```

Creates a new Projection that projects positions using an orthogonal projection with the given size, X:Y aspect ratio, and clipping planes.

flip_fov determines whether the projection's field of view is flipped over its diagonal.

### create_perspective {#create_perspective}

```gdscript
static Projection create_perspective(fovy: float, aspect: float, z_near: float, z_far: float, flip_fov: bool = false)
```

Creates a new Projection that projects positions using a perspective projection with the given Y-axis field of view (in degrees), X:Y aspect ratio, and clipping planes.

flip_fov determines whether the projection's field of view is flipped over its diagonal.

### create_perspective_hmd {#create_perspective_hmd}

```gdscript
static Projection create_perspective_hmd(fovy: float, aspect: float, z_near: float, z_far: float, flip_fov: bool, eye: int, intraocular_dist: float, convergence_dist: float)
```

Creates a new Projection that projects positions using a perspective projection with the given Y-axis field of view (in degrees), X:Y aspect ratio, and clipping distances. The projection is adjusted for a head-mounted display with the given distance between eyes and distance to a point that can be focused on.

eye creates the projection for the left eye when set to 1, or the right eye when set to 2.

flip_fov determines whether the projection's field of view is flipped over its diagonal.

### determinant {#determinant}

```gdscript
float determinant() const
```

Returns a scalar value that is the signed factor by which areas are scaled by this matrix. If the sign is negative, the matrix flips the orientation of the area.

The determinant can be used to calculate the invertibility of a matrix or solve linear systems of equations involving the matrix, among other applications.

### flipped_y {#flipped_y}

```gdscript
Projection flipped_y() const
```

Returns a copy of this Projection with the signs of the values of the Y column flipped.

### get_aspect {#get_aspect}

```gdscript
float get_aspect() const
```

Returns the X:Y aspect ratio of this Projection's viewport.

### get_far_plane_half_extents {#get_far_plane_half_extents}

```gdscript
Vector2 get_far_plane_half_extents() const
```

Returns the dimensions of the far clipping plane of the projection, divided by two.

### get_fov {#get_fov}

```gdscript
float get_fov() const
```

Returns the horizontal field of view of the projection (in degrees).

### get_fovy {#get_fovy}

```gdscript
static float get_fovy(fovx: float, aspect: float)
```

Returns the vertical field of view of the projection (in degrees) associated with the given horizontal field of view (in degrees) and aspect ratio.

Note: Unlike most methods of Projection, aspect is expected to be 1 divided by the X:Y aspect ratio.

### get_lod_multiplier {#get_lod_multiplier}

```gdscript
float get_lod_multiplier() const
```

Returns the factor by which the visible level of detail is scaled by this Projection.

### get_pixels_per_meter {#get_pixels_per_meter}

```gdscript
int get_pixels_per_meter(for_pixel_width: int) const
```

Returns for_pixel_width divided by the viewport's width measured in meters on the near plane, after this Projection is applied.

### get_projection_plane {#get_projection_plane}

```gdscript
Plane get_projection_plane(plane: int) const
```

Returns the clipping plane of this Projection whose index is given by plane.

plane should be equal to one of PLANE_NEAR, PLANE_FAR, PLANE_LEFT, PLANE_TOP, PLANE_RIGHT, or PLANE_BOTTOM.

### get_viewport_half_extents {#get_viewport_half_extents}

```gdscript
Vector2 get_viewport_half_extents() const
```

Returns the dimensions of the viewport plane that this Projection projects positions onto, divided by two.

### get_z_far {#get_z_far}

```gdscript
float get_z_far() const
```

Returns the distance for this Projection beyond which positions are clipped.

### get_z_near {#get_z_near}

```gdscript
float get_z_near() const
```

Returns the distance for this Projection before which positions are clipped.

### inverse {#inverse}

```gdscript
Projection inverse() const
```

Returns a Projection that performs the inverse of this Projection's projective transformation.

### is_orthogonal {#is_orthogonal}

```gdscript
bool is_orthogonal() const
```

Returns true if this Projection performs an orthogonal projection.

### jitter_offseted {#jitter_offseted}

```gdscript
Projection jitter_offseted(offset: Vector2) const
```

Returns a Projection with the X and Y values from the given Vector2 added to the first and second values of the final column respectively.

### perspective_znear_adjusted {#perspective_znear_adjusted}

```gdscript
Projection perspective_znear_adjusted(new_znear: float) const
```

Returns a Projection with the near clipping distance adjusted to be new_znear.

Note: The original Projection must be a perspective projection.

## 运算符

```gdscript
operator !=(right: Projection)
```

Returns true if the projections are not equal.

Note: Due to floating-point precision errors, this may return true, even if the projections are virtually equal. An is_equal_approx method may be added in a future version of Godot.

```gdscript
operator *(right: Projection)
```

Returns a Projection that applies the combined transformations of this Projection and right.

```gdscript
operator *(right: Vector4)
```

Projects (multiplies) the given Vector4 by this Projection matrix.

```gdscript
operator ==(right: Projection)
```

Returns true if the projections are equal.

Note: Due to floating-point precision errors, this may return false, even if the projections are virtually equal. An is_equal_approx method may be added in a future version of Godot.

```gdscript
operator [](index: int)
```

Returns the column of the Projection with the given index.

Indices are in the following order: x, y, z, w.

## 属性

### x {#x}

```gdscript
x: Vector4
```

投影矩阵 X 行/列向量。

### y {#y}

```gdscript
y: Vector4
```

投影矩阵 Y 行/列向量。

### z {#z}

```gdscript
z: Vector4
```

投影矩阵 Z 行/列向量。

### w {#w}

```gdscript
w: Vector4
```

投影矩阵 W 行/列向量。

## 枚举

### Planes {#enum-Planes}

```gdscript
enum Planes { PLANE_NEAR = 0, PLANE_FAR = 1, PLANE_LEFT = 2, PLANE_TOP = 3, PLANE_RIGHT = 4, PLANE_BOTTOM = 5 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PLANE_NEAR {#PLANE_NEAR}

```gdscript
PLANE_NEAR = 0
```

The index value of the projection's near clipping plane.

#### PLANE_FAR {#PLANE_FAR}

```gdscript
PLANE_FAR = 1
```

The index value of the projection's far clipping plane.

#### PLANE_LEFT {#PLANE_LEFT}

```gdscript
PLANE_LEFT = 2
```

The index value of the projection's left clipping plane.

#### PLANE_TOP {#PLANE_TOP}

```gdscript
PLANE_TOP = 3
```

The index value of the projection's top clipping plane.

#### PLANE_RIGHT {#PLANE_RIGHT}

```gdscript
PLANE_RIGHT = 4
```

The index value of the projection's right clipping plane.

#### PLANE_BOTTOM {#PLANE_BOTTOM}

```gdscript
PLANE_BOTTOM = 5
```

The index value of the projection bottom clipping plane.

---

## 常量

### IDENTITY {#IDENTITY}

```gdscript
const IDENTITY = Projection(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
```

单位投影矩阵。

### ZERO {#ZERO}

```gdscript
const ZERO = Projection(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
```

零投影矩阵。
