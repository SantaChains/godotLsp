# VisualShaderNodeColorFunc

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeColorFunc**

> Accept a Color to the input port and transform it according to function.

<details><summary>官方英文描述（中文精译进行中）</summary>

Accept a Color to the input port and transform it according to function.

</details>

## 方法

### get_function {#get_function}

```gdscript
VisualShaderNodeColorFunc.Function get_function() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_function {#set_function}

```gdscript
void set_function(func: VisualShaderNodeColorFunc.Function)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### function {#function}

```gdscript
function: int
```

- void set_function(value: Function)
- Function get_function()

A function to be applied to the input color.

## 枚举

### Function {#enum-Function}

```gdscript
enum Function { FUNC_GRAYSCALE = 0, FUNC_HSV2RGB = 1, FUNC_RGB2HSV = 2, FUNC_SEPIA = 3, FUNC_LINEAR_TO_SRGB = 4, FUNC_SRGB_TO_LINEAR = 5, FUNC_MAX = 6 }
```

Function FUNC_GRAYSCALE = 0

Converts the color to grayscale using the following formula:

```gdscript
vec3 c = input;
float max1 = max(c.r, c.g);
float max2 = max(max1, c.b);
float max3 = max(max1, max2);
return vec3(max3, max3, max3);
```

Function FUNC_HSV2RGB = 1

Converts HSV vector to RGB equivalent.

Function FUNC_RGB2HSV = 2

Converts RGB vector to HSV equivalent.

Function FUNC_SEPIA = 3

Applies sepia tone effect using the following formula:

```gdscript
vec3 c = input;
float r = (c.r * 0.393) + (c.g * 0.769) + (c.b * 0.189);
float g = (c.r * 0.349) + (c.g * 0.686) + (c.b * 0.168);
float b = (c.r * 0.272) + (c.g * 0.534) + (c.b * 0.131);
return vec3(r, g, b);
```

Function FUNC_LINEAR_TO_SRGB = 4

Converts color from linear encoding to nonlinear sRGB encoding using the following formula:

```gdscript
const vec3 a = vec3(0.055f);
return mix((vec3(1.0f) + a) * pow(c.rgb, vec3(1.0f / 2.4f)) - a, 12.92f * c.rgb, lessThan(c.rgb, vec3(0.0031308f)));
```

The Compatibility renderer uses a simpler formula that may produce undefined behavior with negative input values:

```gdscript
vec3 c = input;
return max(vec3(1.055) * pow(c, vec3(0.416666667)) - vec3(0.055), vec3(0.0));
```

Function FUNC_SRGB_TO_LINEAR = 5

Converts color from nonlinear sRGB encoding to linear encoding using the following formula:

```gdscript
vec3 c = input;
return mix(pow((c.rgb + vec3(0.055)) * (1.0 / (1.0 + 0.055)), vec3(2.4)), c.rgb * (1.0 / 12.92), lessThan(c.rgb, vec3(0.04045)));
```

The Compatibility renderer uses a simpler formula that behaves poorly with negative input values:

```gdscript
vec3 c = input;
return c * (c * (c * 0.305306011 + 0.682171111) + 0.012522878);
```

Function FUNC_MAX = 6

Represents the size of the Function enum.

#### FUNC_GRAYSCALE {#FUNC_GRAYSCALE}

```gdscript
FUNC_GRAYSCALE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_HSV2RGB {#FUNC_HSV2RGB}

```gdscript
FUNC_HSV2RGB = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_RGB2HSV {#FUNC_RGB2HSV}

```gdscript
FUNC_RGB2HSV = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_SEPIA {#FUNC_SEPIA}

```gdscript
FUNC_SEPIA = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_LINEAR_TO_SRGB {#FUNC_LINEAR_TO_SRGB}

```gdscript
FUNC_LINEAR_TO_SRGB = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_SRGB_TO_LINEAR {#FUNC_SRGB_TO_LINEAR}

```gdscript
FUNC_SRGB_TO_LINEAR = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### FUNC_MAX {#FUNC_MAX}

```gdscript
FUNC_MAX = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
