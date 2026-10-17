# Transform2D

> 2D 变换。由旋转矩阵和平移组成。

<details><summary>官方英文描述</summary>

The Transform2D built-in Variant type is a 2×3 matrix [https://en.wikipedia.org/wiki/Matrix_(mathematics)] representing a transformation in 2D space. It contains three Vector2 values: x, y, and origin. Together, they can represent translation, rotation, scale, and skew.

The x and y axes form a 2×2 matrix, known as the transform's basis. The length of each axis (Vector2.length()) influences the transform's scale, while the direction of all axes influence the rotation. Usually, both axes are perpendicular to one another. However, when you rotate one axis individually, the transform becomes skewed. Applying a skewed transform to a 2D sprite will make the sprite appear distorted.

For a general introduction, see the Matrices and transforms tutorial.

Note: Unlike Transform3D, there is no 2D equivalent to the Basis type. All mentions of "basis" refer to the x and y components of Transform2D.

Note: In a boolean context, a Transform2D will evaluate to false if it's equal to IDENTITY. Otherwise, a Transform2D will always evaluate to true.

Note

There are notable differences when using this API with C#. See C# API differences to GDScript for more information.

</details>

## 构造函数

```gdscript
Transform2D()
```

Constructs a Transform2D identical to IDENTITY.

Note: In C#, this constructs a Transform2D with all of its components set to Vector2.ZERO.

```gdscript
Transform2D(from: Transform2D)
```

Constructs a Transform2D as a copy of the given Transform2D.

```gdscript
Transform2D(rotation: float, position: Vector2)
```

Constructs a Transform2D from a given angle (in radians) and position.

```gdscript
Transform2D(rotation: float, scale: Vector2, skew: float, position: Vector2)
```

Constructs a Transform2D from a given angle (in radians), scale, skew (in radians), and position.

```gdscript
Transform2D(x_axis: Vector2, y_axis: Vector2, origin: Vector2)
```

Constructs a Transform2D from 3 Vector2 values representing x, y, and the origin (the three matrix columns).

## 方法

### affine_inverse {#affine_inverse}

```gdscript
Transform2D affine_inverse() const
```

Returns the inverted version of this transform. Unlike inverse(), this method works with almost any basis, including non-uniform ones, but is slower.

Note: For this method to return correctly, the transform's basis needs to have a determinant that is not exactly 0.0 (see determinant()).

### basis_xform {#basis_xform}

```gdscript
Vector2 basis_xform(v: Vector2) const
```

Returns a copy of the v vector, transformed (multiplied) by the transform basis's matrix. Unlike the multiplication operator (*), this method ignores the origin.

### basis_xform_inv {#basis_xform_inv}

```gdscript
Vector2 basis_xform_inv(v: Vector2) const
```

Returns a copy of the v vector, transformed (multiplied) by the inverse transform basis's matrix (see inverse()). This method ignores the origin.

Note: This method assumes that this transform's basis is orthonormal (see orthonormalized()). If the basis is not orthonormal, transform.affine_inverse().basis_xform(vector) should be used instead (see affine_inverse()).

### determinant {#determinant}

```gdscript
float determinant() const
```

Returns the determinant [https://en.wikipedia.org/wiki/Determinant] of this transform basis's matrix. For advanced math, this number can be used to determine a few attributes:

- If the determinant is exactly 0.0, the basis is not invertible (see inverse()).
- If the determinant is a negative number, the basis represents a negative scale.

Note: If the basis's scale is the same for every axis, its determinant is always that scale by the power of 2.

### get_origin {#get_origin}

```gdscript
Vector2 get_origin() const
```

Returns this transform's translation. Equivalent to origin.

### get_rotation {#get_rotation}

```gdscript
float get_rotation() const
```

Returns this transform's rotation (in radians). This is equivalent to x's angle (see Vector2.angle()).

### get_scale {#get_scale}

```gdscript
Vector2 get_scale() const
```

Returns the length of both x and y, as a Vector2. If this transform's basis is not skewed, this value is the scaling factor. It is not affected by rotation.

```
var my_transform = Transform2D(
    Vector2(2, 0),
    Vector2(0, 4),
    Vector2(0, 0)
)
# Rotating the Transform2D in any way preserves its scale.
my_transform = my_transform.rotated(TAU / 2)
print(my_transform.get_scale()) # Prints (2.0, 4.0)
```

```
var myTransform = new Transform2D(
    Vector3(2.0f, 0.0f),
    Vector3(0.0f, 4.0f),
    Vector3(0.0f, 0.0f)
);
// Rotating the Transform2D in any way preserves its scale.
myTransform = myTransform.Rotated(Mathf.Tau / 2.0f);
GD.Print(myTransform.GetScale()); // Prints (2, 4)
```

Note: If the value returned by determinant() is negative, the scale is also negative.

### get_skew {#get_skew}

```gdscript
float get_skew() const
```

Returns this transform's skew (in radians).

### interpolate_with {#interpolate_with}

```gdscript
Transform2D interpolate_with(xform: Transform2D, weight: float) const
```

Returns the result of the linear interpolation between this transform and xform by the given weight.

The weight should be between 0.0 and 1.0 (inclusive). Values outside this range are allowed and can be used to perform extrapolation instead.

### inverse {#inverse}

```gdscript
Transform2D inverse() const
```

Returns the inverted version of this transform [https://en.wikipedia.org/wiki/Invertible_matrix].

Note: For this method to return correctly, the transform's basis needs to be orthonormal (see orthonormalized()). That means the basis should only represent a rotation. If it does not, use affine_inverse() instead.

### is_conformal {#is_conformal}

```gdscript
bool is_conformal() const
```

Returns true if this transform's basis is conformal. A conformal basis is both orthogonal (the axes are perpendicular to each other) and uniform (the axes share the same length). This method can be especially useful during physics calculations.

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(xform: Transform2D) const
```

Returns true if this transform and xform are approximately equal, by running @GlobalScope.is_equal_approx() on each component.

### is_finite {#is_finite}

```gdscript
bool is_finite() const
```

Returns true if this transform is finite, by calling @GlobalScope.is_finite() on each component.

### looking_at {#looking_at}

```gdscript
Transform2D looking_at(target: Vector2 = Vector2(0, 0)) const
```

Returns a copy of the transform rotated such that the rotated X-axis points towards the target position, in global space.

### orthonormalized {#orthonormalized}

```gdscript
Transform2D orthonormalized() const
```

Returns a copy of this transform with its basis orthonormalized. An orthonormal basis is both orthogonal (the axes are perpendicular to each other) and normalized (the axes have a length of 1.0), which also means it can only represent a rotation.

### rotated {#rotated}

```gdscript
Transform2D rotated(angle: float) const
```

Returns a copy of this transform rotated by the given angle (in radians).

If angle is positive, the transform is rotated clockwise.

This method is an optimized version of multiplying the given transform X with a corresponding rotation transform R from the left, i.e., R * X.

This can be seen as transforming with respect to the global/parent frame.

### rotated_local {#rotated_local}

```gdscript
Transform2D rotated_local(angle: float) const
```

Returns a copy of the transform rotated by the given angle (in radians).

This method is an optimized version of multiplying the given transform X with a corresponding rotation transform R from the right, i.e., X * R.

This can be seen as transforming with respect to the local frame.

### scaled {#scaled}

```gdscript
Transform2D scaled(scale: Vector2) const
```

Returns a copy of the transform scaled by the given scale factor.

This method is an optimized version of multiplying the given transform X with a corresponding scaling transform S from the left, i.e., S * X.

This can be seen as transforming with respect to the global/parent frame.

### scaled_local {#scaled_local}

```gdscript
Transform2D scaled_local(scale: Vector2) const
```

Returns a copy of the transform scaled by the given scale factor.

This method is an optimized version of multiplying the given transform X with a corresponding scaling transform S from the right, i.e., X * S.

This can be seen as transforming with respect to the local frame.

### translated {#translated}

```gdscript
Transform2D translated(offset: Vector2) const
```

Returns a copy of the transform translated by the given offset.

This method is an optimized version of multiplying the given transform X with a corresponding translation transform T from the left, i.e., T * X.

This can be seen as transforming with respect to the global/parent frame.

### translated_local {#translated_local}

```gdscript
Transform2D translated_local(offset: Vector2) const
```

Returns a copy of the transform translated by the given offset.

This method is an optimized version of multiplying the given transform X with a corresponding translation transform T from the right, i.e., X * T.

This can be seen as transforming with respect to the local frame.

## 运算符

```gdscript
operator !=(right: Transform2D)
```

Returns true if the components of both transforms are not equal.

Note: Due to floating-point precision errors, consider using is_equal_approx() instead, which is more reliable.

```gdscript
operator *(right: PackedVector2Array)
```

Transforms (multiplies) every Vector2 element of the given PackedVector2Array by this transformation matrix.

On larger arrays, this operation is much faster than transforming each Vector2 individually.

```gdscript
operator *(right: Rect2)
```

Transforms (multiplies) the Rect2 by this transformation matrix.

```gdscript
operator *(right: Transform2D)
```

Transforms (multiplies) this transform by the right transform.

This is the operation performed between parent and child CanvasItem nodes.

Note: If you need to only modify one attribute of this transform, consider using one of the following methods, instead:

- For translation, see translated() or translated_local().
- For rotation, see rotated() or rotated_local().
- For scale, see scaled() or scaled_local().

```gdscript
operator *(right: Vector2)
```

Transforms (multiplies) the Vector2 by this transformation matrix.

```gdscript
operator *(right: float)
```

Multiplies all components of the Transform2D by the given float, including the origin. This affects the transform's scale uniformly.

```gdscript
operator *(right: int)
```

Multiplies all components of the Transform2D by the given int, including the origin. This affects the transform's scale uniformly.

```gdscript
operator /(right: float)
```

Divides all components of the Transform2D by the given float, including the origin. This affects the transform's scale uniformly.

```gdscript
operator /(right: int)
```

Divides all components of the Transform2D by the given int, including the origin. This affects the transform's scale uniformly.

```gdscript
operator ==(right: Transform2D)
```

Returns true if the components of both transforms are exactly equal.

Note: Due to floating-point precision errors, consider using is_equal_approx() instead, which is more reliable.

```gdscript
operator [](index: int)
```

Accesses each axis (column) of this transform by their index. Index 0 is the same as x, index 1 is the same as y, and index 2 is the same as origin.

## 属性

### x {#x}

```gdscript
x: Vector2
```

2D 变换基向量 X 轴（第一行/列）。

### y {#y}

```gdscript
y: Vector2
```

2D 变换基向量 Y 轴。

### origin {#origin}

```gdscript
origin: Vector2
```

2D 平移原点。

## 常量

### IDENTITY {#IDENTITY}

```gdscript
const IDENTITY = Transform2D(1, 0, 0, 1, 0, 0)
```

单位 2D 变换（无旋转缩放、零平移）。

### FLIP_X {#FLIP_X}

```gdscript
const FLIP_X = Transform2D(-1, 0, 0, 1, 0, 0)
```

沿 X 轴翻转的变换。

### FLIP_Y {#FLIP_Y}

```gdscript
const FLIP_Y = Transform2D(1, 0, 0, -1, 0, 0)
```

沿 Y 轴翻转的变换。
