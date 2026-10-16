# JavaObject

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **JavaObject**

> Represents an object from the Java Native Interface. It can be returned from Java methods called on JavaClass or other JavaObjects. See JavaClassWrapper for an example.

<details><summary>官方英文描述（中文精译进行中）</summary>

Represents an object from the Java Native Interface. It can be returned from Java methods called on JavaClass or other JavaObjects. See JavaClassWrapper for an example.

Note: This class only works on Android. On any other platform, this class does nothing.

Note: This class is not to be confused with JavaScriptObject.

</details>

## 方法

### get_java_class {#get_java_class}

```gdscript
JavaClass get_java_class() const
```

Returns the JavaClass that this object is an instance of.

### has_java_method {#has_java_method}

```gdscript
bool has_java_method(method: StringName) const
```

Returns true if the given method name exists in the object's Java methods.
