# AnimationNodeTimeSeek

**继承**：[AnimationNode](class_animationnode.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationNodeTimeSeek**

> This animation node can be used to cause a seek command to happen to any sub-children of the animation graph. Use to play an Animation from the start or a certain playback position inside the AnimationNodeBlendTree.

<details><summary>官方英文描述</summary>

This animation node can be used to cause a seek command to happen to any sub-children of the animation graph. Use to play an Animation from the start or a certain playback position inside the AnimationNodeBlendTree.

After setting the time and changing the animation playback, the time seek node automatically goes into sleep mode on the next process frame by setting its seek_request value to -1.0.

```
# Play child animation from the start.
animation_tree.set("parameters/TimeSeek/seek_request", 0.0)
# Alternative syntax (same result as above).
animation_tree["parameters/TimeSeek/seek_request"] = 0.0
# Play child animation from 12 second timestamp.
animation_tree.set("parameters/TimeSeek/seek_request", 12.0)
# Alternative syntax (same result as above).
animation_tree["parameters/TimeSeek/seek_request"] = 12.0
```

```
// Play child animation from the start.
animationTree.Set("parameters/TimeSeek/seek_request", 0.0);
// Play child animation from 12 second timestamp.
animationTree.Set("parameters/TimeSeek/seek_request", 12.0);
```

</details>

## 方法

### is_explicit_elapse {#is_explicit_elapse}

```gdscript
bool is_explicit_elapse() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_explicit_elapse {#set_explicit_elapse}

```gdscript
void set_explicit_elapse(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### explicit_elapse {#explicit_elapse}

```gdscript
explicit_elapse: bool
```

- void set_explicit_elapse(value: bool)
- bool is_explicit_elapse()

If true, some processes are executed to handle keys between seeks, such as calculating root motion and finding the nearest discrete key.
