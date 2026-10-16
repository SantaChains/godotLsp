# AnimationLibrary

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AnimationLibrary**

> An animation library stores a set of animations accessible through StringName keys, for use with AnimationPlayer nodes.

<details><summary>官方英文描述</summary>

An animation library stores a set of animations accessible through StringName keys, for use with AnimationPlayer nodes.

</details>

## 方法

### add_animation {#add_animation}

```gdscript
Error add_animation(name: StringName, animation: Animation)
```

Adds the animation to the library, accessible by the key name.

### get_animation {#get_animation}

```gdscript
Animation get_animation(name: StringName) const
```

Returns the Animation with the key name. If the animation does not exist, null is returned and an error is logged.

### get_animation_list {#get_animation_list}

```gdscript
typedarray::StringName get_animation_list() const
```

Returns the keys for the Animations stored in the library.

### get_animation_list_size {#get_animation_list_size}

```gdscript
int get_animation_list_size() const
```

Returns the key count for the Animations stored in the library.

### has_animation {#has_animation}

```gdscript
bool has_animation(name: StringName) const
```

Returns true if the library stores an Animation with name as the key.

### remove_animation {#remove_animation}

```gdscript
void remove_animation(name: StringName)
```

Removes the Animation with the key name.

### rename_animation {#rename_animation}

```gdscript
void rename_animation(name: StringName, newname: StringName)
```

Changes the key of the Animation associated with the key name to newname.

## 信号

### animation_added {#animation_added}

```gdscript
signal animation_added(anim_name: StringName)
```

Emitted when an Animation is added, under the key anim_name.

### animation_removed {#animation_removed}

```gdscript
signal animation_removed(anim_name: StringName)
```

Emitted when an Animation stored with the key anim_name is removed.

### animation_renamed {#animation_renamed}

```gdscript
signal animation_renamed(old_name: StringName, new_name: StringName)
```

Emitted when the key for an Animation is changed, from old_name to new_name.

### animation_changed {#animation_changed}

```gdscript
signal animation_changed(anim_name: StringName)
```

Emitted when there's a change in one of the animations, e.g. tracks are added, moved or have changed paths. anim_name is the key of the animation that was changed.

See also Resource.changed, which this acts as a relay for.
