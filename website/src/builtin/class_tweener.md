# Tweener

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **Tweener**

> Tweeners are objects that perform a specific animating task, e.g. interpolating a property or calling a method at a given time. A Tweener can't be created manually, you need to use a dedicated method from Tween.

<details><summary>官方英文描述（中文精译进行中）</summary>

Tweeners are objects that perform a specific animating task, e.g. interpolating a property or calling a method at a given time. A Tweener can't be created manually, you need to use a dedicated method from Tween.

---

</details>

## 信号

### finished {#finished}

```gdscript
signal finished()
```

Emitted when the Tweener has just finished its job or became invalid (e.g. due to a freed object).
