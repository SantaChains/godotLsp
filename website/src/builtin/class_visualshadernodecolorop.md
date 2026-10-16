# VisualShaderNodeColorOp

**继承**：[VisualShaderNode](class_visualshadernode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **VisualShaderNodeColorOp**

> Applies operator to two color inputs.

<details><summary>官方英文描述（中文精译进行中）</summary>

Applies operator to two color inputs.

</details>

## 方法

### get_operator {#get_operator}

```gdscript
VisualShaderNodeColorOp.Operator get_operator() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_operator {#set_operator}

```gdscript
void set_operator(op: VisualShaderNodeColorOp.Operator)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### operator {#operator}

```gdscript
operator: int
```

- void set_operator(value: Operator)
- Operator get_operator()

An operator to be applied to the inputs.

## 枚举

### Operator {#enum-Operator}

```gdscript
enum Operator { OP_SCREEN = 0, OP_DIFFERENCE = 1, OP_DARKEN = 2, OP_LIGHTEN = 3, OP_OVERLAY = 4, OP_DODGE = 5, OP_BURN = 6, OP_SOFT_LIGHT = 7, OP_HARD_LIGHT = 8, OP_MAX = 9 }
```

Operator OP_SCREEN = 0

Produce a screen effect with the following formula:

```gdscript
result = vec3(1.0) - (vec3(1.0) - a) * (vec3(1.0) - b);
```

Operator OP_DIFFERENCE = 1

Produce a difference effect with the following formula:

```gdscript
result = abs(a - b);
```

Operator OP_DARKEN = 2

Produce a darken effect with the following formula:

```gdscript
result = min(a, b);
```

Operator OP_LIGHTEN = 3

Produce a lighten effect with the following formula:

```gdscript
result = max(a, b);
```

Operator OP_OVERLAY = 4

Produce an overlay effect with the following formula:

```gdscript
for (int i = 0; i < 3; i++) {
    float base = a[i];
    float blend = b[i];
    if (base < 0.5) {
        result[i] = 2.0 * base * blend;
    } else {
        result[i] = 1.0 - 2.0 * (1.0 - blend) * (1.0 - base);
    }
}
```

Operator OP_DODGE = 5

Produce a dodge effect with the following formula:

```gdscript
result = a / (vec3(1.0) - b);
```

Operator OP_BURN = 6

Produce a burn effect with the following formula:

```gdscript
result = vec3(1.0) - (vec3(1.0) - a) / b;
```

Operator OP_SOFT_LIGHT = 7

Produce a soft light effect with the following formula:

```gdscript
for (int i = 0; i < 3; i++) {
    float base = a[i];
    float blend = b[i];
    if (base < 0.5) {
        result[i] = base * (blend + 0.5);
    } else {
        result[i] = 1.0 - (1.0 - base) * (1.0 - (blend - 0.5));
    }
}
```

Operator OP_HARD_LIGHT = 8

Produce a hard light effect with the following formula:

```gdscript
for (int i = 0; i < 3; i++) {
    float base = a[i];
    float blend = b[i];
    if (base < 0.5) {
        result[i] = base * (2.0 * blend);
    } else {
        result[i] = 1.0 - (1.0 - base) * (1.0 - 2.0 * (blend - 0.5));
    }
}
```

Operator OP_MAX = 9

Represents the size of the Operator enum.

#### OP_SCREEN {#OP_SCREEN}

```gdscript
OP_SCREEN = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_DIFFERENCE {#OP_DIFFERENCE}

```gdscript
OP_DIFFERENCE = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_DARKEN {#OP_DARKEN}

```gdscript
OP_DARKEN = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_LIGHTEN {#OP_LIGHTEN}

```gdscript
OP_LIGHTEN = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_OVERLAY {#OP_OVERLAY}

```gdscript
OP_OVERLAY = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_DODGE {#OP_DODGE}

```gdscript
OP_DODGE = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_BURN {#OP_BURN}

```gdscript
OP_BURN = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_SOFT_LIGHT {#OP_SOFT_LIGHT}

```gdscript
OP_SOFT_LIGHT = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_HARD_LIGHT {#OP_HARD_LIGHT}

```gdscript
OP_HARD_LIGHT = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### OP_MAX {#OP_MAX}

```gdscript
OP_MAX = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
