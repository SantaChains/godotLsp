# VoxelGI

**继承**：[VisualInstance3D](class_visualinstance3d.md) → [Node3D](class_node3d.md) → [Node](class_node.md) → [Object](class_object.md) → **VoxelGI**

> VoxelGIs are used to provide high-quality real-time indirect light and reflections to scenes. They precompute the effect of objects that emit light and the effect of static geometry to simulate the behavior of complex light in real-time. VoxelGIs need to be baked before having a visible effect. However, once baked, dynamic objects will receive light from them. Furthermore, lights can be fully dynamic or baked.

<details><summary>官方英文描述（中文精译进行中）</summary>

VoxelGIs are used to provide high-quality real-time indirect light and reflections to scenes. They precompute the effect of objects that emit light and the effect of static geometry to simulate the behavior of complex light in real-time. VoxelGIs need to be baked before having a visible effect. However, once baked, dynamic objects will receive light from them. Furthermore, lights can be fully dynamic or baked.

Note: VoxelGI is only supported in the Forward+ rendering method, not Mobile or Compatibility.

Procedural generation: VoxelGI can be baked in an exported project, which makes it suitable for procedurally generated or user-built levels as long as all the geometry is generated in advance. For games where geometry is generated at any time during gameplay, SDFGI is more suitable (see Environment.sdfgi_enabled).

Performance: VoxelGI is relatively demanding on the GPU and is not suited to low-end hardware such as integrated graphics (consider LightmapGI instead). To improve performance, adjust ProjectSettings.rendering/global_illumination/voxel_gi/quality and enable ProjectSettings.rendering/global_illumination/gi/use_half_resolution in the Project Settings. To provide a fallback for low-end hardware, consider adding an option to disable VoxelGI in your project's options menus. A VoxelGI node can be disabled by hiding it.

Note: Meshes should have sufficiently thick walls to avoid light leaks (avoid one-sided walls). For interior levels, enclose your level geometry in a sufficiently large box and bridge the loops to close the mesh. To further prevent light leaks, you can also strategically place temporary MeshInstance3D nodes with their GeometryInstance3D.gi_mode set to GeometryInstance3D.GI_MODE_STATIC. These temporary nodes can then be hidden after baking the VoxelGI node.

</details>

## 方法

### bake {#bake}

```gdscript
void bake(from_node: Node = null, create_visual_debug: bool = false)
```

Bakes the effect from all GeometryInstance3Ds marked with GeometryInstance3D.GI_MODE_STATIC and Light3Ds marked with either Light3D.BAKE_STATIC or Light3D.BAKE_DYNAMIC. If create_visual_debug is true, after baking the light, this will generate a MultiMesh that has a cube representing each solid cell with each cube colored to the cell's albedo color. This can be used to visualize the VoxelGI's data and debug any issues that may be occurring.

Note: bake() works from the editor and in exported projects. This makes it suitable for procedurally generated or user-built levels. Baking a VoxelGI node generally takes from 5 to 20 seconds in most scenes. Reducing subdiv can speed up baking.

Note: GeometryInstance3Ds and Light3Ds must be fully ready before bake() is called. If you are procedurally creating those and some meshes or lights are missing from your baked VoxelGI, use call_deferred("bake") instead of calling bake() directly.

### debug_bake {#debug_bake}

```gdscript
void debug_bake()
```

Calls bake() with create_visual_debug enabled.

### get_camera_attributes {#get_camera_attributes}

```gdscript
CameraAttributes get_camera_attributes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_probe_data {#get_probe_data}

```gdscript
VoxelGIData get_probe_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_size {#get_size}

```gdscript
Vector3 get_size() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_subdiv {#get_subdiv}

```gdscript
VoxelGI.Subdiv get_subdiv() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_camera_attributes {#set_camera_attributes}

```gdscript
void set_camera_attributes(camera_attributes: CameraAttributes)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_probe_data {#set_probe_data}

```gdscript
void set_probe_data(data: VoxelGIData)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_size {#set_size}

```gdscript
void set_size(size: Vector3)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_subdiv {#set_subdiv}

```gdscript
void set_subdiv(subdiv: VoxelGI.Subdiv)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### subdiv {#subdiv}

```gdscript
subdiv: int
```

- void set_subdiv(value: Subdiv)
- Subdiv get_subdiv()

Number of times to subdivide the grid that the VoxelGI operates on. A higher number results in finer detail and thus higher visual quality, while lower numbers result in better performance.

### size {#size}

```gdscript
size: Vector3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### camera_attributes {#camera_attributes}

```gdscript
camera_attributes: CameraAttributesPractical,CameraAttributesPhysical
```

- void set_camera_attributes(value: CameraAttributes)
- CameraAttributes get_camera_attributes()

The CameraAttributes resource that specifies exposure levels to bake at. Auto-exposure and non exposure properties will be ignored. Exposure settings should be used to reduce the dynamic range present when baking. If exposure is too high, the VoxelGI will have banding artifacts or may have over-exposure artifacts.

### data {#data}

```gdscript
data: VoxelGIData
```

- void set_probe_data(value: VoxelGIData)
- VoxelGIData get_probe_data()

The VoxelGIData resource that holds the data for this VoxelGI.

## 枚举

### Subdiv {#enum-Subdiv}

```gdscript
enum Subdiv { SUBDIV_64 = 0, SUBDIV_128 = 1, SUBDIV_256 = 2, SUBDIV_512 = 3, SUBDIV_MAX = 4 }
```

Subdiv SUBDIV_64 = 0

Use 64 subdivisions. This is the lowest quality setting, but the fastest. Use it if you can, but especially use it on lower-end hardware.

Subdiv SUBDIV_128 = 1

Use 128 subdivisions. This is the default quality setting.

Subdiv SUBDIV_256 = 2

Use 256 subdivisions.

Subdiv SUBDIV_512 = 3

Use 512 subdivisions. This is the highest quality setting, but the slowest. On lower-end hardware, this could cause the GPU to stall.

Subdiv SUBDIV_MAX = 4

Represents the size of the Subdiv enum.

#### SUBDIV_64 {#SUBDIV_64}

```gdscript
SUBDIV_64 = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SUBDIV_128 {#SUBDIV_128}

```gdscript
SUBDIV_128 = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SUBDIV_256 {#SUBDIV_256}

```gdscript
SUBDIV_256 = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SUBDIV_512 {#SUBDIV_512}

```gdscript
SUBDIV_512 = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### SUBDIV_MAX {#SUBDIV_MAX}

```gdscript
SUBDIV_MAX = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
