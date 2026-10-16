# Compositor

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Compositor**

> The compositor resource stores attributes used to customize how a Viewport is rendered.

<details><summary>官方英文描述（中文精译进行中）</summary>

The compositor resource stores attributes used to customize how a Viewport is rendered.

</details>

## 方法

### get_compositor_effects {#get_compositor_effects}

```gdscript
typedarray::CompositorEffect get_compositor_effects() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_compositor_effects {#set_compositor_effects}

```gdscript
void set_compositor_effects(compositor_effects: typedarray::CompositorEffect)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### compositor_effects {#compositor_effects}

```gdscript
compositor_effects: typedarray::24/17:CompositorEffect
```

- void set_compositor_effects(value: Array[CompositorEffect])
- Array[CompositorEffect] get_compositor_effects()

The custom CompositorEffects that are applied during rendering of viewports using this compositor.
