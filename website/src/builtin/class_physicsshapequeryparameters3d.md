# PhysicsShapeQueryParameters3D

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **PhysicsShapeQueryParameters3D**

> By changing various properties of this object, such as the shape, you can configure the parameters for PhysicsDirectSpaceState3D's methods.

<details><summary>官方英文描述（中文精译进行中）</summary>

By changing various properties of this object, such as the shape, you can configure the parameters for PhysicsDirectSpaceState3D's methods.

</details>

## 方法

### get_collision_mask {#get_collision_mask}

```gdscript
int get_collision_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_exclude {#get_exclude}

```gdscript
typedarray::RID get_exclude() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_margin {#get_margin}

```gdscript
float get_margin() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_motion {#get_motion}

```gdscript
Vector3 get_motion() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape {#get_shape}

```gdscript
Resource get_shape() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_shape_rid {#get_shape_rid}

```gdscript
RID get_shape_rid() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_transform {#get_transform}

```gdscript
Transform3D get_transform() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collide_with_areas_enabled {#is_collide_with_areas_enabled}

```gdscript
bool is_collide_with_areas_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_collide_with_bodies_enabled {#is_collide_with_bodies_enabled}

```gdscript
bool is_collide_with_bodies_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collide_with_areas {#set_collide_with_areas}

```gdscript
void set_collide_with_areas(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collide_with_bodies {#set_collide_with_bodies}

```gdscript
void set_collide_with_bodies(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_collision_mask {#set_collision_mask}

```gdscript
void set_collision_mask(collision_mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_exclude {#set_exclude}

```gdscript
void set_exclude(exclude: typedarray::RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_margin {#set_margin}

```gdscript
void set_margin(margin: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_motion {#set_motion}

```gdscript
void set_motion(motion: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape {#set_shape}

```gdscript
void set_shape(shape: Resource)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_shape_rid {#set_shape_rid}

```gdscript
void set_shape_rid(shape: RID)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_transform {#set_transform}

```gdscript
void set_transform(transform: Transform3D)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### collision_mask {#collision_mask}

```gdscript
collision_mask: int
```

- void set_collision_mask(value: int)
- int get_collision_mask()

The physics layers the query will detect (as a bitmask). By default, all collision layers are detected. See Collision layers and masks in the documentation for more information.

### exclude {#exclude}

```gdscript
exclude: typedarray::RID
```

- void set_exclude(value: Array[RID])
- Array[RID] get_exclude()

The list of object RIDs that will be excluded from collisions. Use CollisionObject3D.get_rid() to get the RID associated with a CollisionObject3D-derived node.

Note: The returned array is copied and any changes to it will not update the original property value. To update the value you need to modify the returned array, and then assign it to the property again.

### margin {#margin}

```gdscript
margin: float
```

- void set_margin(value: float)
- float get_margin()

The collision margin for the shape.

### motion {#motion}

```gdscript
motion: Vector3
```

- void set_motion(value: Vector3)
- Vector3 get_motion()

The motion of the shape being queried for.

### shape {#shape}

```gdscript
shape: Shape3D
```

- void set_shape(value: Resource)
- Resource get_shape()

The Shape3D that will be used for collision/intersection queries. This stores the actual reference which avoids the shape to be released while being used for queries, so always prefer using this over shape_rid.

### shape_rid {#shape_rid}

```gdscript
shape_rid: RID
```

- void set_shape_rid(value: RID)
- RID get_shape_rid()

The queried shape's RID that will be used for collision/intersection queries. Use this over shape if you want to optimize for performance using the Servers API:

```
var shape_rid = PhysicsServer3D.sphere_shape_create()
var radius = 2.0
PhysicsServer3D.shape_set_data(shape_rid, radius)
var params = PhysicsShapeQueryParameters3D.new()
params.shape_rid = shape_rid
# Execute physics queries here...
# Release the shape when done with physics queries.
PhysicsServer3D.free_rid(shape_rid)
```

```
RID shapeRid = PhysicsServer3D.SphereShapeCreate();
float radius = 2.0f;
PhysicsServer3D.ShapeSetData(shapeRid, radius);
var params = new PhysicsShapeQueryParameters3D();
params.ShapeRid = shapeRid;
// Execute physics queries here...
// Release the shape when done with physics queries.
PhysicsServer3D.FreeRid(shapeRid);
```

### transform {#transform}

```gdscript
transform: Transform3D
```

- void set_transform(value: Transform3D)
- Transform3D get_transform()

The queried shape's transform matrix.

### collide_with_bodies {#collide_with_bodies}

```gdscript
collide_with_bodies: bool
```

- void set_collide_with_bodies(value: bool)
- bool is_collide_with_bodies_enabled()

If true, the query will take PhysicsBody3Ds into account.

### collide_with_areas {#collide_with_areas}

```gdscript
collide_with_areas: bool
```

- void set_collide_with_areas(value: bool)
- bool is_collide_with_areas_enabled()

If true, the query will take Area3Ds into account.
