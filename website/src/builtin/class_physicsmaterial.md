# PhysicsMaterial

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **PhysicsMaterial**

> Holds physics-related properties of a surface, namely its roughness and bounciness. This class is used to apply these properties to a physics body.

<details><summary>官方英文描述</summary>

Holds physics-related properties of a surface, namely its roughness and bounciness. This class is used to apply these properties to a physics body.

</details>

## 方法

### get_bounce {#get_bounce}

```gdscript
float get_bounce() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_friction {#get_friction}

```gdscript
float get_friction() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_absorbent {#is_absorbent}

```gdscript
bool is_absorbent() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_rough {#is_rough}

```gdscript
bool is_rough() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_absorbent {#set_absorbent}

```gdscript
void set_absorbent(absorbent: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_bounce {#set_bounce}

```gdscript
void set_bounce(bounce: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_friction {#set_friction}

```gdscript
void set_friction(friction: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_rough {#set_rough}

```gdscript
void set_rough(rough: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### friction {#friction}

```gdscript
friction: float
```

- void set_friction(value: float)
- float get_friction()

The body's friction. Values range from 0 (frictionless) to 1 (maximum friction).

### rough {#rough}

```gdscript
rough: bool
```

- void set_rough(value: bool)
- bool is_rough()

If true, the physics engine will use the friction of the object marked as "rough" when two objects collide. If false, the physics engine will use the lowest friction of all colliding objects instead. If true for both colliding objects, the physics engine will use the highest friction.

### bounce {#bounce}

```gdscript
bounce: float
```

- void set_bounce(value: float)
- float get_bounce()

The body's bounciness. Values range from 0 (no bounce) to 1 (full bounciness).

Note: Even with bounce set to 1.0, some energy will be lost over time due to linear and angular damping. To have a physics body that preserves all its energy over time, set bounce to 1.0, the body's linear damp mode to Replace (if applicable), its linear damp to 0.0, its angular damp mode to Replace (if applicable), and its angular damp to 0.0.

### absorbent {#absorbent}

```gdscript
absorbent: bool
```

- void set_absorbent(value: bool)
- bool is_absorbent()

If true, subtracts the bounciness from the colliding object's bounciness instead of adding it.
