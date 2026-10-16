# AwaitTweener

**继承**：[Tweener](class_tweener.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **AwaitTweener**

> AwaitTweener is used to await a specified signal, allowing asynchronous steps in Tween animation. See Tween.tween_await() for more usage information.

<details><summary>官方英文描述</summary>

AwaitTweener is used to await a specified signal, allowing asynchronous steps in Tween animation. See Tween.tween_await() for more usage information.

The Tweener.finished signal is emitted when either the awaited signal is received, when timeout is reached, or when the target object is freed.

</details>

## 方法

### set_timeout {#set_timeout}

```gdscript
AwaitTweener set_timeout(timeout: float)
```

Sets the maximum time an AwaitTweener can wait for the signal. Can be used as a safeguard for signals that may never be emitted. If not specified, the tweener will wait indefinitely.
