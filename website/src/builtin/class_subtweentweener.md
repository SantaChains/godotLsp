# SubtweenTweener

**继承**：[Tweener](class_tweener.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **SubtweenTweener**

> SubtweenTweener is used to execute a Tween as one step in a sequence defined by another Tween. See Tween.tween_subtween() for more usage information.

<details><summary>官方英文描述（中文精译进行中）</summary>

SubtweenTweener is used to execute a Tween as one step in a sequence defined by another Tween. See Tween.tween_subtween() for more usage information.

Note: Tween.tween_subtween() is the only correct way to create SubtweenTweener. Any SubtweenTweener created manually will not function correctly.

</details>

## 方法

### set_delay {#set_delay}

```gdscript
SubtweenTweener set_delay(delay: float)
```

Sets the time in seconds after which the SubtweenTweener will start running the subtween. By default there's no delay.
