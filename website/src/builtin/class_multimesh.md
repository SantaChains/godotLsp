# MultiMesh

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MultiMesh**

> MultiMesh provides low-level mesh instancing. Drawing thousands of MeshInstance3D nodes can be slow, since each object is submitted to the GPU then drawn individually.

<details><summary>官方英文描述</summary>

MultiMesh provides low-level mesh instancing. Drawing thousands of MeshInstance3D nodes can be slow, since each object is submitted to the GPU then drawn individually.

MultiMesh is much faster as it can draw thousands of instances with a single draw call, resulting in less API overhead.

As a drawback, if the instances are too far away from each other, performance may be reduced as every single instance will always render (they are spatially indexed as one, for the whole object).

Since instances may have any behavior, the AABB used for visibility must be provided by the user.

Note: A MultiMesh is a single object, therefore the same maximum lights per object restriction applies. This means, that once the maximum lights are consumed by one or more instances, the rest of the MultiMesh instances will not receive any lighting.

Note: Blend Shapes will be ignored if used in a MultiMesh.

</details>

## 方法

### get_aabb {#get_aabb}

```gdscript
AABB get_aabb() const
```

Returns the visibility axis-aligned bounding box in local space.

### get_buffer {#get_buffer}

```gdscript
PackedFloat32Array get_buffer() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_custom_aabb {#get_custom_aabb}

```gdscript
AABB get_custom_aabb() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_instance_color {#get_instance_color}

```gdscript
Color get_instance_color(instance: int) const
```

Gets a specific instance's color multiplier.

### get_instance_count {#get_instance_count}

```gdscript
int get_instance_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_instance_custom_data {#get_instance_custom_data}

```gdscript
Color get_instance_custom_data(instance: int) const
```

Returns the custom data that has been set for a specific instance.

### get_instance_transform {#get_instance_transform}

```gdscript
Transform3D get_instance_transform(instance: int) const
```

Returns the Transform3D of a specific instance.

### get_instance_transform_2d {#get_instance_transform_2d}

```gdscript
Transform2D get_instance_transform_2d(instance: int) const
```

Returns the Transform2D of a specific instance.

### get_mesh {#get_mesh}

```gdscript
Mesh get_mesh() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_physics_interpolation_quality {#get_physics_interpolation_quality}

```gdscript
MultiMesh.PhysicsInterpolationQuality get_physics_interpolation_quality() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transform_format {#get_transform_format}

```gdscript
MultiMesh.TransformFormat get_transform_format() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_visible_instance_count {#get_visible_instance_count}

```gdscript
int get_visible_instance_count() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_colors {#is_using_colors}

```gdscript
bool is_using_colors() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_using_custom_data {#is_using_custom_data}

```gdscript
bool is_using_custom_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### reset_instance_physics_interpolation {#reset_instance_physics_interpolation}

```gdscript
void reset_instance_physics_interpolation(instance: int)
```

When using physics interpolation, this function allows you to prevent interpolation on an instance in the current physics tick.

This allows you to move instances instantaneously, and should usually be used when initially placing an instance such as a bullet to prevent graphical glitches.

### reset_instances_physics_interpolation {#reset_instances_physics_interpolation}

```gdscript
void reset_instances_physics_interpolation()
```

When using physics interpolation, this function allows you to prevent interpolation for all instances in the current physics tick.

This allows you to move all instances instantaneously, and should usually be used when initially placing instances to prevent graphical glitches.

### set_buffer {#set_buffer}

```gdscript
void set_buffer(buffer: PackedFloat32Array)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_buffer_interpolated {#set_buffer_interpolated}

```gdscript
void set_buffer_interpolated(buffer_curr: PackedFloat32Array, buffer_prev: PackedFloat32Array)
```

An alternative to setting the buffer property, which can be used with physics interpolation. This method takes two arrays, and can set the data for the current and previous tick in one go. The renderer will automatically interpolate the data at each frame.

This is useful for situations where the order of instances may change from physics tick to tick, such as particle systems.

When the order of instances is coherent, the simpler alternative of setting buffer can still be used with interpolation.

### set_custom_aabb {#set_custom_aabb}

```gdscript
void set_custom_aabb(aabb: AABB)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_instance_color {#set_instance_color}

```gdscript
void set_instance_color(instance: int, color: Color)
```

Sets the color of a specific instance by multiplying the mesh's existing vertex colors. This allows for different color tinting per instance.

Note: Each component is stored in 32 bits in the Forward+ and Mobile rendering methods, but is packed into 16 bits in the Compatibility rendering method.

For the color to take effect, ensure that use_colors is true on the MultiMesh and BaseMaterial3D.vertex_color_use_as_albedo is true on the material. If you intend to set an absolute color instead of tinting, make sure the material's albedo color is set to pure white (Color(1, 1, 1)).

### set_instance_count {#set_instance_count}

```gdscript
void set_instance_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_instance_custom_data {#set_instance_custom_data}

```gdscript
void set_instance_custom_data(instance: int, custom_data: Color)
```

Sets custom data for a specific instance. custom_data is a Color type only to contain 4 floating-point numbers.

Note: Each number is stored in 32 bits in the Forward+ and Mobile rendering methods, but is packed into 16 bits in the Compatibility rendering method.

For the custom data to be used, ensure that use_custom_data is true.

This custom instance data has to be manually accessed in your custom shader using INSTANCE_CUSTOM.

### set_instance_transform {#set_instance_transform}

```gdscript
void set_instance_transform(instance: int, transform: Transform3D)
```

Sets the Transform3D for a specific instance.

### set_instance_transform_2d {#set_instance_transform_2d}

```gdscript
void set_instance_transform_2d(instance: int, transform: Transform2D)
```

Sets the Transform2D for a specific instance.

### set_mesh {#set_mesh}

```gdscript
void set_mesh(mesh: Mesh)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_physics_interpolation_quality {#set_physics_interpolation_quality}

```gdscript
void set_physics_interpolation_quality(quality: MultiMesh.PhysicsInterpolationQuality)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transform_format {#set_transform_format}

```gdscript
void set_transform_format(format: MultiMesh.TransformFormat)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_colors {#set_use_colors}

```gdscript
void set_use_colors(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_use_custom_data {#set_use_custom_data}

```gdscript
void set_use_custom_data(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_visible_instance_count {#set_visible_instance_count}

```gdscript
void set_visible_instance_count(count: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### transform_format {#transform_format}

```gdscript
transform_format: int
```

- void set_transform_format(value: TransformFormat)
- TransformFormat get_transform_format()

Format of transform used to transform mesh, either 2D or 3D.

### use_colors {#use_colors}

```gdscript
use_colors: bool
```

- void set_use_colors(value: bool)
- bool is_using_colors()

If true, the MultiMesh will use color data (see set_instance_color()). Can only be set when instance_count is 0 or less. This means that you need to call this method before setting the instance count, or temporarily reset it to 0.

### use_custom_data {#use_custom_data}

```gdscript
use_custom_data: bool
```

- void set_use_custom_data(value: bool)
- bool is_using_custom_data()

If true, the MultiMesh will use custom data (see set_instance_custom_data()). Can only be set when instance_count is 0 or less. This means that you need to call this method before setting the instance count, or temporarily reset it to 0.

### custom_aabb {#custom_aabb}

```gdscript
custom_aabb: AABB
```

- void set_custom_aabb(value: AABB)
- AABB get_custom_aabb()

Custom AABB for this MultiMesh resource. Setting this manually prevents costly runtime AABB recalculations.

### instance_count {#instance_count}

```gdscript
instance_count: int
```

- void set_instance_count(value: int)
- int get_instance_count()

Number of instances that will get drawn. This clears and (re)sizes the buffers. Setting data format or flags afterwards will have no effect.

By default, all instances are drawn but you can limit this with visible_instance_count.

### visible_instance_count {#visible_instance_count}

```gdscript
visible_instance_count: int
```

- void set_visible_instance_count(value: int)
- int get_visible_instance_count()

Limits the number of instances drawn, -1 draws all instances. Changing this does not change the sizes of the buffers.

### mesh {#mesh}

```gdscript
mesh: Mesh
```

- void set_mesh(value: Mesh)
- Mesh get_mesh()

Mesh resource to be instanced.

The looks of the individual instances can be modified using set_instance_color() and set_instance_custom_data().

### buffer {#buffer}

```gdscript
buffer: PackedFloat32Array
```

- void set_buffer(value: PackedFloat32Array)
- PackedFloat32Array get_buffer()

There is currently no description for this property. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

Note: The returned array is copied and any changes to it will not update the original property value. See PackedFloat32Array for more details.

### transform_array {#transform_array}

```gdscript
transform_array: PackedVector3Array
```

Deprecated: Accessing this property is very slow. Use set_instance_transform() and get_instance_transform() instead.

Array containing each Transform3D value used by all instances of this mesh, as a PackedVector3Array. Each transform is divided into 4 Vector3 values corresponding to the transforms' x, y, z, and origin.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector3Array for more details.

### transform_2d_array {#transform_2d_array}

```gdscript
transform_2d_array: PackedVector2Array
```

Deprecated: Accessing this property is very slow. Use set_instance_transform_2d() and get_instance_transform_2d() instead.

Array containing each Transform2D value used by all instances of this mesh, as a PackedVector2Array. Each transform is divided into 3 Vector2 values corresponding to the transforms' x, y, and origin.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedVector2Array for more details.

### color_array {#color_array}

```gdscript
color_array: PackedColorArray
```

Deprecated: Accessing this property is very slow. Use set_instance_color() and get_instance_color() instead.

Array containing each Color used by all instances of this mesh.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedColorArray for more details.

### custom_data_array {#custom_data_array}

```gdscript
custom_data_array: PackedColorArray
```

Deprecated: Accessing this property is very slow. Use set_instance_custom_data() and get_instance_custom_data() instead.

Array containing each custom data value used by all instances of this mesh, as a PackedColorArray.

Note: The returned array is copied and any changes to it will not update the original property value. See PackedColorArray for more details.

### physics_interpolation_quality {#physics_interpolation_quality}

```gdscript
physics_interpolation_quality: int
```

- void set_physics_interpolation_quality(value: PhysicsInterpolationQuality)
- PhysicsInterpolationQuality get_physics_interpolation_quality()

Choose whether to use an interpolation method that favors speed or quality.

When using low physics tick rates (typically below 20) or high rates of object rotation, you may get better results from the high quality setting.

Note: Fast quality does not equate to low quality. Except in the special cases mentioned above, the quality should be comparable to high quality.

## 枚举

### TransformFormat {#enum-TransformFormat}

```gdscript
enum TransformFormat { TRANSFORM_2D = 0, TRANSFORM_3D = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TRANSFORM_2D {#TRANSFORM_2D}

```gdscript
TRANSFORM_2D = 0
```

Use this when using 2D transforms.

#### TRANSFORM_3D {#TRANSFORM_3D}

```gdscript
TRANSFORM_3D = 1
```

Use this when using 3D transforms.

---

### PhysicsInterpolationQuality {#enum-PhysicsInterpolationQuality}

```gdscript
enum PhysicsInterpolationQuality { INTERP_QUALITY_FAST = 0, INTERP_QUALITY_HIGH = 1 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### INTERP_QUALITY_FAST {#INTERP_QUALITY_FAST}

```gdscript
INTERP_QUALITY_FAST = 0
```

Always interpolate using Basis lerping, which can produce warping artifacts in some situations.

#### INTERP_QUALITY_HIGH {#INTERP_QUALITY_HIGH}

```gdscript
INTERP_QUALITY_HIGH = 1
```

Attempt to interpolate using Basis slerping (spherical linear interpolation) where possible, otherwise fall back to lerping.

---
