# CallbackTweener

**继承**：[Tweener](class_tweener.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CallbackTweener**

> CallbackTweener is used to call a method in a tweening sequence. See Tween.tween_callback() for more usage information.

<details><summary>官方英文描述</summary>

CallbackTweener is used to call a method in a tweening sequence. See Tween.tween_callback() for more usage information.

The tweener will finish automatically if the callback's target object is freed.

Note: Tween.tween_callback() is the only correct way to create CallbackTweener. Any CallbackTweener created manually will not function correctly.

</details>

## 方法

### set_delay {#set_delay}

```gdscript
CallbackTweener set_delay(delay: float)
```

Makes the callback call delayed by given time in seconds.

Example: Call Node.queue_free() after 2 seconds:

```gdscript
var tween = get_tree().create_tween()
tween.tween_callback(queue_free).set_delay(2)
```
