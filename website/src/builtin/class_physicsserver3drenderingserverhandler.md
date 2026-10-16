# PhysicsServer3DRenderingServerHandler

**继承**：[Object](class_object.md) → **PhysicsServer3DRenderingServerHandler**

## 方法

### _set_aabb {#_set_aabb}

```gdscript
void _set_aabb(aabb: AABB) virtual
```

Called by the PhysicsServer3D to set the bounding box for the SoftBody3D.

### _set_normal {#_set_normal}

```gdscript
void _set_normal(vertex_id: int, normal: Vector3) virtual
```

Called by the PhysicsServer3D to set the normal for the SoftBody3D vertex at the index specified by vertex_id.

Note: The normal parameter used to be of type const void* prior to Godot 4.2.

### _set_vertex {#_set_vertex}

```gdscript
void _set_vertex(vertex_id: int, vertex: Vector3) virtual
```

Called by the PhysicsServer3D to set the position for the SoftBody3D vertex at the index specified by vertex_id.

Note: The vertex parameter used to be of type const void* prior to Godot 4.2.

### set_aabb {#set_aabb}

```gdscript
void set_aabb(aabb: AABB)
```

Sets the bounding box for the SoftBody3D.

### set_normal {#set_normal}

```gdscript
void set_normal(vertex_id: int, normal: Vector3)
```

Sets the normal for the SoftBody3D vertex at the index specified by vertex_id.

### set_vertex {#set_vertex}

```gdscript
void set_vertex(vertex_id: int, vertex: Vector3)
```

Sets the position for the SoftBody3D vertex at the index specified by vertex_id.
