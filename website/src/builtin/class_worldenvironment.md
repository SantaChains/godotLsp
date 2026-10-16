# WorldEnvironment

**继承**：[Node](class_node.md) → [Object](class_object.md) → **WorldEnvironment**

> The WorldEnvironment node is used to configure the default Environment for the scene.

<details><summary>官方英文描述</summary>

The WorldEnvironment node is used to configure the default Environment for the scene.

The parameters defined in the WorldEnvironment can be overridden by an Environment node set on the current Camera3D. Additionally, only one WorldEnvironment may be instantiated in a given scene at a time.

The WorldEnvironment allows the user to specify default lighting parameters (e.g. ambient lighting), various post-processing effects (e.g. SSAO, DOF, Tonemapping), and how to draw the background (e.g. solid color, skybox). Usually, these are added in order to improve the realism/color balance of the scene.

</details>

## 方法

### get_camera_attributes {#get_camera_attributes}

```gdscript
CameraAttributes get_camera_attributes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_compositor {#get_compositor}

```gdscript
Compositor get_compositor() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_environment {#get_environment}

```gdscript
Environment get_environment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_camera_attributes {#set_camera_attributes}

```gdscript
void set_camera_attributes(camera_attributes: CameraAttributes)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_compositor {#set_compositor}

```gdscript
void set_compositor(compositor: Compositor)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment {#set_environment}

```gdscript
void set_environment(env: Environment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### environment {#environment}

```gdscript
environment: Environment
```

- void set_environment(value: Environment)
- Environment get_environment()

The Environment resource used by this WorldEnvironment, defining the default properties.

### camera_attributes {#camera_attributes}

```gdscript
camera_attributes: CameraAttributesPractical,CameraAttributesPhysical
```

- void set_camera_attributes(value: CameraAttributes)
- CameraAttributes get_camera_attributes()

The default CameraAttributes resource to use if none set on the Camera3D.

### compositor {#compositor}

```gdscript
compositor: Compositor
```

- void set_compositor(value: Compositor)
- Compositor get_compositor()

The default Compositor resource to use if none set on the Camera3D.
