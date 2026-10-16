# Noise

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Noise**

> This class defines the interface for noise generation libraries to inherit from.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class defines the interface for noise generation libraries to inherit from.

A default get_seamless_image() implementation is provided for libraries that do not provide seamless noise. This function requests a larger image from the get_image() method, reverses the quadrants of the image, then uses the strips of extra width to blend over the seams.

Inheriting noise classes can optionally override this function to provide a more optimal algorithm.

</details>

## 方法

### get_image {#get_image}

```gdscript
Image get_image(width: int, height: int, invert: bool = false, in_3d_space: bool = false, normalize: bool = true) const
```

Returns an Image containing 2D noise values.

Note: With normalize set to false, the default implementation expects the noise generator to return values in the range -1.0 to 1.0.

### get_image_3d {#get_image_3d}

```gdscript
typedarray::Image get_image_3d(width: int, height: int, depth: int, invert: bool = false, normalize: bool = true) const
```

Returns an Array of Images containing 3D noise values for use with ImageTexture3D.create().

Note: With normalize set to false, the default implementation expects the noise generator to return values in the range -1.0 to 1.0.

### get_noise_1d {#get_noise_1d}

```gdscript
float get_noise_1d(x: float) const
```

Returns the 1D noise value at the given (x) coordinate.

### get_noise_2d {#get_noise_2d}

```gdscript
float get_noise_2d(x: float, y: float) const
```

Returns the 2D noise value at the given position.

### get_noise_2dv {#get_noise_2dv}

```gdscript
float get_noise_2dv(v: Vector2) const
```

Returns the 2D noise value at the given position.

### get_noise_3d {#get_noise_3d}

```gdscript
float get_noise_3d(x: float, y: float, z: float) const
```

Returns the 3D noise value at the given position.

### get_noise_3dv {#get_noise_3dv}

```gdscript
float get_noise_3dv(v: Vector3) const
```

Returns the 3D noise value at the given position.

### get_seamless_image {#get_seamless_image}

```gdscript
Image get_seamless_image(width: int, height: int, invert: bool = false, in_3d_space: bool = false, skirt: float = 0.1, normalize: bool = true) const
```

Returns an Image containing seamless 2D noise values.

Note: With normalize set to false, the default implementation expects the noise generator to return values in the range -1.0 to 1.0.

### get_seamless_image_3d {#get_seamless_image_3d}

```gdscript
typedarray::Image get_seamless_image_3d(width: int, height: int, depth: int, invert: bool = false, skirt: float = 0.1, normalize: bool = true) const
```

Returns an Array of Images containing seamless 3D noise values for use with ImageTexture3D.create().

Note: With normalize set to false, the default implementation expects the noise generator to return values in the range -1.0 to 1.0.
