# OptimizedTranslation

**继承**：[Translation](class_translation.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OptimizedTranslation**

> An optimized translation. Uses real-time compressed translations, which results in very small dictionaries.

<details><summary>官方英文描述</summary>

An optimized translation. Uses real-time compressed translations, which results in very small dictionaries.

This class does not store the untranslated strings for optimization purposes. Therefore, Translation.get_message_list() always returns an empty array, and Translation.get_message_count() always returns 0.

</details>

## 方法

### generate {#generate}

```gdscript
bool generate(from: Translation)
```

Generates and sets an optimized translation from the given Translation resource. Returns true if successful.

Note: Messages in from should not use context or plural forms.

Note: This method is intended to be used in the editor. It does nothing when called from an exported project.
