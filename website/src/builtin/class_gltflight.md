# GLTFLight

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **GLTFLight**

> Represents a light as defined by the KHR_lights_punctual glTF extension.

<details><summary>官方英文描述（中文精译进行中）</summary>

Represents a light as defined by the KHR_lights_punctual glTF extension.

</details>

## 方法

### from_dictionary {#from_dictionary}

```gdscript
static GLTFLight from_dictionary(dictionary: Dictionary)
```

Creates a new GLTFLight instance by parsing the given Dictionary.

### from_node {#from_node}

```gdscript
static GLTFLight from_node(light_node: Light3D)
```

Create a new GLTFLight instance from the given Godot Light3D node.

### get_additional_data {#get_additional_data}

```gdscript
Variant get_additional_data(extension_name: StringName)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_color {#get_color}

```gdscript
Color get_color()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_inner_cone_angle {#get_inner_cone_angle}

```gdscript
float get_inner_cone_angle()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_intensity {#get_intensity}

```gdscript
float get_intensity()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_light_type {#get_light_type}

```gdscript
String get_light_type()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_outer_cone_angle {#get_outer_cone_angle}

```gdscript
float get_outer_cone_angle()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_range {#get_range}

```gdscript
float get_range()
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_additional_data {#set_additional_data}

```gdscript
void set_additional_data(extension_name: StringName, additional_data: Variant)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_color {#set_color}

```gdscript
void set_color(color: Color)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_inner_cone_angle {#set_inner_cone_angle}

```gdscript
void set_inner_cone_angle(inner_cone_angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_intensity {#set_intensity}

```gdscript
void set_intensity(intensity: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_light_type {#set_light_type}

```gdscript
void set_light_type(light_type: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_outer_cone_angle {#set_outer_cone_angle}

```gdscript
void set_outer_cone_angle(outer_cone_angle: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_range {#set_range}

```gdscript
void set_range(range: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### to_dictionary {#to_dictionary}

```gdscript
Dictionary to_dictionary() const
```

Serializes this GLTFLight instance into a Dictionary.

### to_node {#to_node}

```gdscript
Light3D to_node() const
```

Converts this GLTFLight instance into a Godot Light3D node.

## 属性

### color {#color}

```gdscript
color: Color
```

- void set_color(value: Color)
- Color get_color()

The Color of the light in linear space. Defaults to white. A black color causes the light to have no effect.

This value is linear to match glTF, but will be converted to nonlinear sRGB when creating a Godot Light3D node upon import, or converted to linear when exporting a Godot Light3D to glTF.

### intensity {#intensity}

```gdscript
intensity: float
```

- void set_intensity(value: float)
- float get_intensity()

The intensity of the light. This is expressed in candelas (lumens per steradian) for point and spot lights, and lux (lumens per m²) for directional lights. When creating a Godot light, this value is converted to a unitless multiplier.

### light_type {#light_type}

```gdscript
light_type: String
```

- void set_light_type(value: String)
- String get_light_type()

The type of the light. The values accepted by Godot are "point", "spot", and "directional", which correspond to Godot's OmniLight3D, SpotLight3D, and DirectionalLight3D respectively.

### range {#range}

```gdscript
range: float
```

- void set_range(value: float)
- float get_range()

The range of the light, beyond which the light has no effect. glTF lights with no range defined behave like physical lights (which have infinite range). When creating a Godot light, the range is clamped to 4096.0.

### inner_cone_angle {#inner_cone_angle}

```gdscript
inner_cone_angle: float
```

- void set_inner_cone_angle(value: float)
- float get_inner_cone_angle()

The inner angle of the cone in a spotlight. Must be less than or equal to the outer cone angle.

Within this angle, the light is at full brightness. Between the inner and outer cone angles, there is a transition from full brightness to zero brightness. When creating a Godot SpotLight3D, the ratio between the inner and outer cone angles is used to calculate the attenuation of the light.

### outer_cone_angle {#outer_cone_angle}

```gdscript
outer_cone_angle: float
```

- void set_outer_cone_angle(value: float)
- float get_outer_cone_angle()

The outer angle of the cone in a spotlight. Must be greater than or equal to the inner angle.

At this angle, the light drops off to zero brightness. Between the inner and outer cone angles, there is a transition from full brightness to zero brightness. If this angle is a half turn, then the spotlight emits in all directions. When creating a Godot SpotLight3D, the outer cone angle is used as the angle of the spotlight.
