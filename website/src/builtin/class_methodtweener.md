# MethodTweener

**继承**：[Tweener](class_tweener.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **MethodTweener**

> MethodTweener is similar to a combination of CallbackTweener and PropertyTweener. It calls a method providing an interpolated value as a parameter. See Tween.tween_method() for more usage information.

<details><summary>官方英文描述</summary>

MethodTweener is similar to a combination of CallbackTweener and PropertyTweener. It calls a method providing an interpolated value as a parameter. See Tween.tween_method() for more usage information.

The tweener will finish automatically if the callback's target object is freed.

Note: Tween.tween_method() is the only correct way to create MethodTweener. Any MethodTweener created manually will not function correctly.

</details>

## 方法

### set_delay {#set_delay}

```gdscript
MethodTweener set_delay(delay: float)
```

Sets the time in seconds after which the MethodTweener will start interpolating. By default there's no delay.

### set_ease {#set_ease}

```gdscript
MethodTweener set_ease(ease: Tween.EaseType)
```

Sets the type of used easing from EaseType. If not set, the default easing is used from the Tween that contains this Tweener.

### set_trans {#set_trans}

```gdscript
MethodTweener set_trans(trans: Tween.TransitionType)
```

Sets the type of used transition from TransitionType. If not set, the default transition is used from the Tween that contains this Tweener.
