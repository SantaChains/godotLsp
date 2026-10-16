# ImmediateMesh

**继承**：[Mesh](class_mesh.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ImmediateMesh**

> A mesh type optimized for creating geometry manually, similar to OpenGL 1.x immediate mode.

<details><summary>官方英文描述（中文精译进行中）</summary>

A mesh type optimized for creating geometry manually, similar to OpenGL 1.x immediate mode.

Here's a sample on how to generate a triangular face:

```
var mesh = ImmediateMesh.new()
mesh.surface_begin(Mesh.PRIMITIVE_TRIANGLES)
mesh.surface_add_vertex(Vector3.LEFT)
mesh.surface_add_vertex(Vector3.FORWARD)
mesh.surface_add_vertex(Vector3.ZERO)
mesh.surface_end()
```

```
var mesh = new ImmediateMesh();
mesh.SurfaceBegin(Mesh.PrimitiveType.Triangles);
mesh.SurfaceAddVertex(Vector3.Left);
mesh.SurfaceAddVertex(Vector3.Forward);
mesh.SurfaceAddVertex(Vector3.Zero);
mesh.SurfaceEnd();
```

Note: Generating complex geometries with ImmediateMesh is highly inefficient. Instead, it is designed to generate simple geometry that changes often.

</details>

## 方法

### clear_surfaces {#clear_surfaces}

```gdscript
void clear_surfaces()
```

Clear all surfaces.

### surface_add_vertex {#surface_add_vertex}

```gdscript
void surface_add_vertex(vertex: Vector3)
```

Add a 3D vertex using the current attributes previously set.

### surface_add_vertex_2d {#surface_add_vertex_2d}

```gdscript
void surface_add_vertex_2d(vertex: Vector2)
```

Add a 2D vertex using the current attributes previously set.

### surface_begin {#surface_begin}

```gdscript
void surface_begin(primitive: Mesh.PrimitiveType, material: Material = null)
```

Begin a new surface.

### surface_end {#surface_end}

```gdscript
void surface_end()
```

End and commit current surface. Note that surface being created will not be visible until this function is called.

### surface_set_color {#surface_set_color}

```gdscript
void surface_set_color(color: Color)
```

Set the color attribute that will be pushed with the next vertex.

### surface_set_normal {#surface_set_normal}

```gdscript
void surface_set_normal(normal: Vector3)
```

Set the normal attribute that will be pushed with the next vertex.

### surface_set_tangent {#surface_set_tangent}

```gdscript
void surface_set_tangent(tangent: Plane)
```

Set the tangent attribute that will be pushed with the next vertex.

Note: Even though tangent is a Plane, it does not directly represent the tangent plane. Its Plane.x, Plane.y, and Plane.z represent the tangent vector and Plane.d should be either -1 or 1. See also Mesh.ARRAY_TANGENT.

### surface_set_uv {#surface_set_uv}

```gdscript
void surface_set_uv(uv: Vector2)
```

Set the UV attribute that will be pushed with the next vertex.

### surface_set_uv2 {#surface_set_uv2}

```gdscript
void surface_set_uv2(uv2: Vector2)
```

Set the UV2 attribute that will be pushed with the next vertex.
