# World3D

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **World3D**

> Class that has everything pertaining to a world: A physics space, a visual scenario, and a sound space. 3D nodes register their resources into the current 3D world.

<details><summary>官方英文描述</summary>

Class that has everything pertaining to a world: A physics space, a visual scenario, and a sound space. 3D nodes register their resources into the current 3D world.

</details>

## 方法

### get_camera_attributes {#get_camera_attributes}

```gdscript
CameraAttributes get_camera_attributes() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_direct_space_state {#get_direct_space_state}

```gdscript
PhysicsDirectSpaceState3D get_direct_space_state()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_environment {#get_environment}

```gdscript
Environment get_environment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_fallback_environment {#get_fallback_environment}

```gdscript
Environment get_fallback_environment() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_navigation_map {#get_navigation_map}

```gdscript
RID get_navigation_map() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_scenario {#get_scenario}

```gdscript
RID get_scenario() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_space {#get_space}

```gdscript
RID get_space() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_camera_attributes {#set_camera_attributes}

```gdscript
void set_camera_attributes(attributes: CameraAttributes)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_environment {#set_environment}

```gdscript
void set_environment(env: Environment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_fallback_environment {#set_fallback_environment}

```gdscript
void set_fallback_environment(env: Environment)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### environment {#environment}

```gdscript
environment: Environment
```

- void set_environment(value: Environment)
- Environment get_environment()

The World3D's Environment.

### fallback_environment {#fallback_environment}

```gdscript
fallback_environment: Environment
```

- void set_fallback_environment(value: Environment)
- Environment get_fallback_environment()

The World3D's fallback environment will be used if environment fails or is missing.

### camera_attributes {#camera_attributes}

```gdscript
camera_attributes: CameraAttributesPractical,CameraAttributesPhysical
```

- void set_camera_attributes(value: CameraAttributes)
- CameraAttributes get_camera_attributes()

The default CameraAttributes resource to use if none set on the Camera3D.

### space {#space}

```gdscript
space: RID
```

- RID get_space()

The World3D's physics space.

### navigation_map {#navigation_map}

```gdscript
navigation_map: RID
```

- RID get_navigation_map()

The RID of this world's navigation map. Used by the NavigationServer3D.

### scenario {#scenario}

```gdscript
scenario: RID
```

- RID get_scenario()

The World3D's visual scenario.

### direct_space_state {#direct_space_state}

```gdscript
direct_space_state: PhysicsDirectSpaceState3D
```

- PhysicsDirectSpaceState3D get_direct_space_state()

Direct access to the world's physics 3D space state. Used for querying current and potential collisions. When using multi-threaded physics, access is limited to Node._physics_process() in the main thread.
