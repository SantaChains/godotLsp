# EncodedObjectAsID

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **EncodedObjectAsID**

> Utility class which holds a reference to the internal identifier of an Object instance, as given by Object.get_instance_id(). This ID can then be used to retrieve the object instance with @GlobalScope.instance_from_id().

<details><summary>官方英文描述（中文精译进行中）</summary>

Utility class which holds a reference to the internal identifier of an Object instance, as given by Object.get_instance_id(). This ID can then be used to retrieve the object instance with @GlobalScope.instance_from_id().

This class is used internally by the editor inspector and script debugger, but can also be used in plugins to pass and display objects as their IDs.

</details>

## 方法

### get_object_id {#get_object_id}

```gdscript
int get_object_id() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_object_id {#set_object_id}

```gdscript
void set_object_id(id: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### object_id {#object_id}

```gdscript
object_id: int
```

- void set_object_id(value: int)
- int get_object_id()

The Object identifier stored in this EncodedObjectAsID instance. The object instance can be retrieved with @GlobalScope.instance_from_id().
