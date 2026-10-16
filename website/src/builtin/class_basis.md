# Basis

> The Basis built-in Variant type is a 3×3 matrix [https://en.wikipedia.org/wiki/Matrix_(mathematics)] used to represent 3D rotation, scale, and shear. It is frequently used within a Transform3D.

<details><summary>官方英文描述（中文精译进行中）</summary>

The Basis built-in Variant type is a 3×3 matrix [https://en.wikipedia.org/wiki/Matrix_(mathematics)] used to represent 3D rotation, scale, and shear. It is frequently used within a Transform3D.

A Basis is composed by 3 axis vectors, each representing a column of the matrix: x, y, and z. The length of each axis (Vector3.length()) influences the basis's scale, while the direction of all axes influence the rotation. Usually, these axes are perpendicular to one another. However, when you rotate any axis individually, the basis becomes sheared. Applying a sheared basis to a 3D model will make the model appear distorted.

A Basis is:

- Orthogonal if its axes are perpendicular to each other.
- Normalized if the length of every axis is 1.0.
- Uniform if all axes share the same length (see get_scale()).
- Orthonormal if it is both orthogonal and normalized, which allows it to only represent rotations (see orthonormalized()).
- Conformal if it is both orthogonal and uniform, which ensures it is not distorted.

For a general introduction, see the Matrices and transforms tutorial.

Note: Godot uses a right-handed coordinate system [https://en.wikipedia.org/wiki/Right-hand_rule], which is a common standard. For directions, the convention for built-in types like Camera3D is for -Z to point forward (+X is right, +Y is up, and +Z is back). Other objects may use different direction conventions. For more information, see the 3D asset direction conventions tutorial.

Note: The basis matrices are exposed as column-major [https://www.mindcontrol.org/~hplus/graphics/matrix-layout.html] order, which is the same as OpenGL. However, they are stored internally in row-major order, which is the same as DirectX.

Note: In a boolean context, a basis will evaluate to false if it's equal to IDENTITY. Otherwise, a basis will always evaluate to true.

Note

There are notable differences when using this API with C#. See C# API differences to GDScript for more information.

</details>

## 方法

### determinant {#determinant}

```gdscript
float determinant() const
```

Returns the determinant [https://en.wikipedia.org/wiki/Determinant] of this basis's matrix. For advanced math, this number can be used to determine a few attributes:

- If the determinant is exactly 0.0, the basis is not invertible (see inverse()).
- If the determinant is a negative number, the basis represents a negative scale.

Note: If the basis's scale is the same for every axis, its determinant is always that scale by the power of 3.

### from_euler {#from_euler}

```gdscript
static Basis from_euler(euler: Vector3, order: int = 2)
```

Constructs a new Basis that only represents rotation from the given Vector3 of Euler angles [https://en.wikipedia.org/wiki/Euler_angles], in radians.

- The Vector3.x should contain the angle around the x axis (pitch);
- The Vector3.y should contain the angle around the y axis (yaw);
- The Vector3.z should contain the angle around the z axis (roll).

```
# Creates a Basis whose z axis points down.
var my_basis = Basis.from_euler(Vector3(TAU / 4, 0, 0))
print(my_basis.z) # Prints (0.0, -1.0, 0.0)
```

```
// Creates a Basis whose z axis points down.
var myBasis = Basis.FromEuler(new Vector3(Mathf.Tau / 4.0f, 0.0f, 0.0f));
GD.Print(myBasis.Z); // Prints (0, -1, 0)
```

The order of each consecutive rotation can be changed with order (see EulerOrder constants). In Godot, Euler angles always use intrinsic order. By default, the intrinsic YXZ convention is used (@GlobalScope.EULER_ORDER_YXZ): the basis rotates first around the local Y axis (yaw), then local X (pitch), and lastly local Z (roll). When using the opposite method get_euler() to decompose a rotation, this order is reversed.

### from_scale {#from_scale}

```gdscript
static Basis from_scale(scale: Vector3)
```

Constructs a new Basis that only represents scale, with no rotation or shear, from the given scale vector.

```
var my_basis = Basis.from_scale(Vector3(2, 4, 8))
print(my_basis.x) # Prints (2.0, 0.0, 0.0)
print(my_basis.y) # Prints (0.0, 4.0, 0.0)
print(my_basis.z) # Prints (0.0, 0.0, 8.0)
```

```
var myBasis = Basis.FromScale(new Vector3(2.0f, 4.0f, 8.0f));
GD.Print(myBasis.X); // Prints (2, 0, 0)
GD.Print(myBasis.Y); // Prints (0, 4, 0)
GD.Print(myBasis.Z); // Prints (0, 0, 8)
```

Note: In linear algebra, the matrix of this basis is also known as a diagonal matrix [https://en.wikipedia.org/wiki/Diagonal_matrix].

### get_euler {#get_euler}

```gdscript
Vector3 get_euler(order: int = 2) const
```

Returns this basis's rotation as a Vector3 of Euler angles [https://en.wikipedia.org/wiki/Euler_angles], in radians. For the returned value:

- The Vector3.x contains the angle around the x axis (pitch);
- The Vector3.y contains the angle around the y axis (yaw);
- The Vector3.z contains the angle around the z axis (roll).

The order of each consecutive rotation can be changed with order (see EulerOrder constants). In Godot, Euler angles always use intrinsic order. By default, the intrinsic YXZ convention is used (@GlobalScope.EULER_ORDER_YXZ): since we are decomposing, local Z (roll) is calculated first, then local X (pitch), and lastly local Y (yaw). When using the opposite method from_euler() to compose a rotation, this order is reversed.

Note: For this method to return correctly, the basis needs to be orthonormal (see orthonormalized()).

Note: Euler angles are much more intuitive but are not suitable for 3D math. Because of this, consider using the get_rotation_quaternion() method instead, which returns a Quaternion.

Note: In the Inspector dock, a basis's rotation is often displayed in Euler angles (in degrees), as is the case with the Node3D.rotation property.

### get_rotation_quaternion {#get_rotation_quaternion}

```gdscript
Quaternion get_rotation_quaternion() const
```

Returns this basis's rotation as a Quaternion.

Note: Quaternions are much more suitable for 3D math but are less intuitive. For user interfaces, consider using the get_euler() method, which returns Euler angles.

### get_scale {#get_scale}

```gdscript
Vector3 get_scale() const
```

Returns the length of each axis of this basis, as a Vector3. If the basis is not sheared, this value is the scaling factor. It is not affected by rotation.

```
var my_basis = Basis(
    Vector3(2, 0, 0),
    Vector3(0, 4, 0),
    Vector3(0, 0, 8)
)
# Rotating the Basis in any way preserves its scale.
my_basis = my_basis.rotated(Vector3.UP, TAU / 2)
my_basis = my_basis.rotated(Vector3.RIGHT, TAU / 4)
print(my_basis.get_scale()) # Prints (2.0, 4.0, 8.0)
```

```
var myBasis = new Basis(
    Vector3(2.0f, 0.0f, 0.0f),
    Vector3(0.0f, 4.0f, 0.0f),
    Vector3(0.0f, 0.0f, 8.0f)
);
// Rotating the Basis in any way preserves its scale.
myBasis = myBasis.Rotated(Vector3.Up, Mathf.Tau / 2.0f);
myBasis = myBasis.Rotated(Vector3.Right, Mathf.Tau / 4.0f);
GD.Print(myBasis.Scale); // Prints (2, 4, 8)
```

Note: If the value returned by determinant() is negative, the scale is also negative.

### inverse {#inverse}

```gdscript
Basis inverse() const
```

Returns the inverse of this basis's matrix [https://en.wikipedia.org/wiki/Invertible_matrix].

### is_conformal {#is_conformal}

```gdscript
bool is_conformal() const
```

Returns true if this basis is conformal. A conformal basis is both orthogonal (the axes are perpendicular to each other) and uniform (the axes share the same length). This method can be especially useful during physics calculations.

### is_equal_approx {#is_equal_approx}

```gdscript
bool is_equal_approx(b: Basis) const
```

Returns true if this basis and b are approximately equal, by calling @GlobalScope.is_equal_approx() on all vector components.

### is_finite {#is_finite}

```gdscript
bool is_finite() const
```

Returns true if this basis is finite, by calling @GlobalScope.is_finite() on all vector components.

### is_orthonormal {#is_orthonormal}

```gdscript
bool is_orthonormal() const
```

Returns true if this basis is orthonormal. An orthonormal basis is both orthogonal (the axes are perpendicular to each other) and normalized (the length of every axis is 1.0). This method can be especially useful during physics calculations.

### looking_at {#looking_at}

```gdscript
static Basis looking_at(target: Vector3, up: Vector3 = Vector3(0, 1, 0), use_model_front: bool = false)
```

Creates a new Basis with a rotation such that the forward axis (-Z) points towards the target position.

By default, the -Z axis (camera forward) is treated as forward (implies +X is right). If use_model_front is true, the +Z axis (asset front) is treated as forward (implies +X is left) and points toward the target position.

The up axis (+Y) points as close to the up vector as possible while staying perpendicular to the forward axis. The returned basis is orthonormalized (see orthonormalized()).

The target and the up cannot be Vector3.ZERO, and shouldn't be colinear to avoid unintended rotation around local Z axis.

### orthonormalized {#orthonormalized}

```gdscript
Basis orthonormalized() const
```

Returns the orthonormalized version of this basis. An orthonormal basis is both orthogonal (the axes are perpendicular to each other) and normalized (the axes have a length of 1.0), which also means it can only represent a rotation.

It is often useful to call this method to avoid rounding errors on a rotating basis:

```
# Rotate this Node3D every frame.
func _process(delta):
    basis = basis.rotated(Vector3.UP, TAU * delta)
    basis = basis.rotated(Vector3.RIGHT, TAU * delta)
    basis = basis.orthonormalized()
```

```
// Rotate this Node3D every frame.
public override void _Process(double delta)
{
    Basis = Basis.Rotated(Vector3.Up, Mathf.Tau * (float)delta)
            .Rotated(Vector3.Right, Mathf.Tau * (float)delta)
            .Orthonormalized();
}
```

### rotated {#rotated}

```gdscript
Basis rotated(axis: Vector3, angle: float) const
```

Returns a copy of this basis rotated around the given axis by the given angle (in radians).

The axis must be a normalized vector (see Vector3.normalized()). If angle is positive, the basis is rotated counter-clockwise around the axis.

```
var my_basis = Basis.IDENTITY
var angle = TAU / 2
my_basis = my_basis.rotated(Vector3.UP, angle)    # Rotate around the up axis (yaw).
my_basis = my_basis.rotated(Vector3.RIGHT, angle) # Rotate around the right axis (pitch).
my_basis = my_basis.rotated(Vector3.BACK, angle)  # Rotate around the back axis (roll).
```

```
var myBasis = Basis.Identity;
var angle = Mathf.Tau / 2.0f;
myBasis = myBasis.Rotated(Vector3.Up, angle);    // Rotate around the up axis (yaw).
myBasis = myBasis.Rotated(Vector3.Right, angle); // Rotate around the right axis (pitch).
myBasis = myBasis.Rotated(Vector3.Back, angle);  // Rotate around the back axis (roll).
```

### scaled {#scaled}

```gdscript
Basis scaled(scale: Vector3) const
```

Returns this basis with each axis's components scaled by the given scale's components.

The basis matrix's rows are multiplied by scale's components. This operation is a global scale (relative to the parent).

```
var my_basis = Basis(
    Vector3(1, 1, 1),
    Vector3(2, 2, 2),
    Vector3(3, 3, 3)
)
my_basis = my_basis.scaled(Vector3(0, 2, -2))
print(my_basis.x) # Prints (0.0, 2.0, -2.0)
print(my_basis.y) # Prints (0.0, 4.0, -4.0)
print(my_basis.z) # Prints (0.0, 6.0, -6.0)
```

```
var myBasis = new Basis(
    new Vector3(1.0f, 1.0f, 1.0f),
    new Vector3(2.0f, 2.0f, 2.0f),
    new Vector3(3.0f, 3.0f, 3.0f)
);
myBasis = myBasis.Scaled(new Vector3(0.0f, 2.0f, -2.0f));
GD.Print(myBasis.X); // Prints (0, 2, -2)
GD.Print(myBasis.Y); // Prints (0, 4, -4)
GD.Print(myBasis.Z); // Prints (0, 6, -6)
```

### scaled_local {#scaled_local}

```gdscript
Basis scaled_local(scale: Vector3) const
```

Returns this basis with each axis scaled by the corresponding component in the given scale.

The basis matrix's columns are multiplied by scale's components. This operation is a local scale (relative to self).

```
var my_basis = Basis(
    Vector3(1, 1, 1),
    Vector3(2, 2, 2),
    Vector3(3, 3, 3)
)
my_basis = my_basis.scaled_local(Vector3(0, 2, -2))
print(my_basis.x) # Prints (0.0, 0.0, 0.0)
print(my_basis.y) # Prints (4.0, 4.0, 4.0)
print(my_basis.z) # Prints (-6.0, -6.0, -6.0)
```

```
var myBasis = new Basis(
    new Vector3(1.0f, 1.0f, 1.0f),
    new Vector3(2.0f, 2.0f, 2.0f),
    new Vector3(3.0f, 3.0f, 3.0f)
);
myBasis = myBasis.ScaledLocal(new Vector3(0.0f, 2.0f, -2.0f));
GD.Print(myBasis.X); // Prints (0, 0, 0)
GD.Print(myBasis.Y); // Prints (4, 4, 4)
GD.Print(myBasis.Z); // Prints (-6, -6, -6)
```

### slerp {#slerp}

```gdscript
Basis slerp(to: Basis, weight: float) const
```

Performs a spherical-linear interpolation with the to basis, given a weight. Both this basis and to should represent a rotation.

Example: Smoothly rotate a Node3D to the target basis over time, with a Tween:

```gdscript
var start_basis = Basis.IDENTITY
var target_basis = Basis.IDENTITY.rotated(Vector3.UP, TAU / 2)
func _ready():
    create_tween().tween_method(interpolate, 0.0, 1.0, 5.0).set_trans(Tween.TRANS_EXPO)
func interpolate(weight):
    basis = start_basis.slerp(target_basis, weight)
```

### tdotx {#tdotx}

```gdscript
float tdotx(with: Vector3) const
```

Returns the transposed dot product between with and the x axis (see transposed()).

This is equivalent to basis.x.dot(vector).

### tdoty {#tdoty}

```gdscript
float tdoty(with: Vector3) const
```

Returns the transposed dot product between with and the y axis (see transposed()).

This is equivalent to basis.y.dot(vector).

### tdotz {#tdotz}

```gdscript
float tdotz(with: Vector3) const
```

Returns the transposed dot product between with and the z axis (see transposed()).

This is equivalent to basis.z.dot(vector).

### transposed {#transposed}

```gdscript
Basis transposed() const
```

Returns the transposed version of this basis. This turns the basis matrix's columns into rows, and its rows into columns.

```
var my_basis = Basis(
    Vector3(1, 2, 3),
    Vector3(4, 5, 6),
    Vector3(7, 8, 9)
)
my_basis = my_basis.transposed()
print(my_basis.x) # Prints (1.0, 4.0, 7.0)
print(my_basis.y) # Prints (2.0, 5.0, 8.0)
print(my_basis.z) # Prints (3.0, 6.0, 9.0)
```

```
var myBasis = new Basis(
    new Vector3(1.0f, 2.0f, 3.0f),
    new Vector3(4.0f, 5.0f, 6.0f),
    new Vector3(7.0f, 8.0f, 9.0f)
);
myBasis = myBasis.Transposed();
GD.Print(myBasis.X); // Prints (1, 4, 7)
GD.Print(myBasis.Y); // Prints (2, 5, 8)
GD.Print(myBasis.Z); // Prints (3, 6, 9)
```

## 属性

### x {#x}

```gdscript
x: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### y {#y}

```gdscript
y: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### z {#z}

```gdscript
z: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 常量

### IDENTITY {#IDENTITY}

```gdscript
const IDENTITY = Basis(1, 0, 0, 0, 1, 0, 0, 0, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### FLIP_X {#FLIP_X}

```gdscript
const FLIP_X = Basis(-1, 0, 0, 0, 1, 0, 0, 0, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### FLIP_Y {#FLIP_Y}

```gdscript
const FLIP_Y = Basis(1, 0, 0, 0, -1, 0, 0, 0, 1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### FLIP_Z {#FLIP_Z}

```gdscript
const FLIP_Z = Basis(1, 0, 0, 0, 1, 0, 0, 0, -1)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
