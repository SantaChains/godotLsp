# VisualInstance3D

**继承**：[Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **VisualInstance3D**

> The VisualInstance3D is used to connect a resource to a visual representation. All visual 3D nodes inherit from the VisualInstance3D. In general, you should not access the VisualInstance3D properties directly as they are accessed and managed by the nodes that inherit from VisualInstance3D. VisualInstance3D is the node representation of the RenderingServer instance.

<details><summary>官方英文描述</summary>

The VisualInstance3D is used to connect a resource to a visual representation. All visual 3D nodes inherit from the VisualInstance3D. In general, you should not access the VisualInstance3D properties directly as they are accessed and managed by the nodes that inherit from VisualInstance3D. VisualInstance3D is the node representation of the RenderingServer instance.

</details>

## 方法

### _get_aabb {#_get_aabb}

```gdscript
AABB _get_aabb() virtual const
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_aabb {#get_aabb}

```gdscript
AABB get_aabb() const
```

Returns the AABB (also known as the bounding box) for this VisualInstance3D.

### get_base {#get_base}

```gdscript
RID get_base() const
```

Returns the RID of the resource associated with this VisualInstance3D. For example, if the Node is a MeshInstance3D, this will return the RID of the associated Mesh.

### get_instance {#get_instance}

```gdscript
RID get_instance() const
```

Returns the RID of this instance. This RID is the same as the RID returned by RenderingServer.instance_create(). This RID is needed if you want to call RenderingServer functions directly on this VisualInstance3D.

### get_layer_mask {#get_layer_mask}

```gdscript
int get_layer_mask() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_layer_mask_value {#get_layer_mask_value}

```gdscript
bool get_layer_mask_value(layer_number: int) const
```

Returns whether or not the specified layer of the layers is enabled, given a layer_number between 1 and 20.

### get_sorting_offset {#get_sorting_offset}

```gdscript
float get_sorting_offset() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_sorting_use_aabb_center {#is_sorting_use_aabb_center}

```gdscript
bool is_sorting_use_aabb_center() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_base {#set_base}

```gdscript
void set_base(base: RID)
```

Sets the resource that is instantiated by this VisualInstance3D, which changes how the engine handles the VisualInstance3D under the hood. Equivalent to RenderingServer.instance_set_base().

### set_layer_mask {#set_layer_mask}

```gdscript
void set_layer_mask(mask: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_layer_mask_value {#set_layer_mask_value}

```gdscript
void set_layer_mask_value(layer_number: int, value: bool)
```

Based on value, enables or disables the specified layer in the layers, given a layer_number between 1 and 20.

### set_sorting_offset {#set_sorting_offset}

```gdscript
void set_sorting_offset(offset: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sorting_use_aabb_center {#set_sorting_use_aabb_center}

```gdscript
void set_sorting_use_aabb_center(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### layers {#layers}

```gdscript
layers: int
```

- void set_layer_mask(value: int)
- int get_layer_mask()

The render layer(s) this VisualInstance3D is drawn on.

This object will only be visible for Camera3Ds whose cull mask includes any of the render layers this VisualInstance3D is set to.

For Light3Ds, this can be used to control which VisualInstance3Ds are affected by a specific light. For GPUParticles3D, this can be used to control which particles are effected by a specific attractor. For Decals, this can be used to control which VisualInstance3Ds are affected by a specific decal.

To adjust layers more easily using a script, use get_layer_mask_value() and set_layer_mask_value().

Note: VoxelGI, SDFGI and LightmapGI will always take all layers into account to determine what contributes to global illumination. If this is an issue, set GeometryInstance3D.gi_mode to GeometryInstance3D.GI_MODE_DISABLED for meshes and Light3D.light_bake_mode to Light3D.BAKE_DISABLED for lights to exclude them from global illumination.

### sorting_offset {#sorting_offset}

```gdscript
sorting_offset: float
```

- void set_sorting_offset(value: float)
- float get_sorting_offset()

The amount by which the depth of this VisualInstance3D will be adjusted when sorting by depth. Uses the same units as the engine (which are typically meters). Adjusting it to a higher value will make the VisualInstance3D reliably draw on top of other VisualInstance3Ds that are otherwise positioned at the same spot. To ensure it always draws on top of other objects around it (not positioned at the same spot), set the value to be greater than the distance between this VisualInstance3D and the other nearby VisualInstance3Ds.

### sorting_use_aabb_center {#sorting_use_aabb_center}

```gdscript
sorting_use_aabb_center: bool
```

- void set_sorting_use_aabb_center(value: bool)
- bool is_sorting_use_aabb_center()

If true, the object is sorted based on the AABB center. The object will be sorted based on the global position otherwise.

The AABB center based sorting is generally more accurate for 3D models. The position based sorting instead allows to better control the drawing order when working with GPUParticles3D and CPUParticles3D.
