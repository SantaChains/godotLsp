# JavaClass

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **JavaClass**

> Represents a class from the Java Native Interface. It is returned from JavaClassWrapper.wrap().

<details><summary>官方英文描述</summary>

Represents a class from the Java Native Interface. It is returned from JavaClassWrapper.wrap().

Note: This class only works on Android. On any other platform, this class does nothing.

Note: This class is not to be confused with JavaScriptObject.

</details>

## 方法

### get_java_class_name {#get_java_class_name}

```gdscript
String get_java_class_name() const
```

Returns the Java class name.

### get_java_method_list {#get_java_method_list}

```gdscript
typedarray::Dictionary get_java_method_list() const
```

Returns the object's Java methods and their signatures as an Array of dictionaries, in the same format as Object.get_method_list().

### get_java_parent_class {#get_java_parent_class}

```gdscript
JavaClass get_java_parent_class() const
```

Returns a JavaClass representing the Java parent class of this class.

### has_java_method {#has_java_method}

```gdscript
bool has_java_method(method: StringName) const
```

Returns true if the given method name exists in the object's Java methods.
