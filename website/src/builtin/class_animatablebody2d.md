# AnimatableBody2D

**继承**：[StaticBody2D](class_staticbody2d.md) → [PhysicsBody2D](class_physicsbody2d.md) → [CollisionObject2D](class_collisionobject2d.md) → [Node2D](class_node2d.md) → [CanvasItem](class_canvasitem.md) → [Node](class_node.md) → [Object](class_object.md) → **AnimatableBody2D**

> An animatable 2D physics body. It can't be moved by external forces or contacts, but can be moved manually by other means such as code, AnimationMixers (with AnimationMixer.callback_mode_process set to AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS), and RemoteTransform2D.

<details><summary>官方英文描述（中文精译进行中）</summary>

An animatable 2D physics body. It can't be moved by external forces or contacts, but can be moved manually by other means such as code, AnimationMixers (with AnimationMixer.callback_mode_process set to AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS), and RemoteTransform2D.

When AnimatableBody2D is moved, its linear and angular velocity are estimated and used to affect other physics bodies in its path. This makes it useful for moving platforms, doors, and other moving objects.

</details>

## 方法

### is_sync_to_physics_enabled {#is_sync_to_physics_enabled}

```gdscript
bool is_sync_to_physics_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_sync_to_physics {#set_sync_to_physics}

```gdscript
void set_sync_to_physics(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### sync_to_physics {#sync_to_physics}

```gdscript
sync_to_physics: bool
```

- void set_sync_to_physics(value: bool)
- bool is_sync_to_physics_enabled()

If true, the body's movement will be synchronized to the physics frame. This is useful when animating movement via AnimationPlayer, for example on moving platforms. Do not use together with PhysicsBody2D.move_and_collide().
