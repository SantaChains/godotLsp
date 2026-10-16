# OpenXRStructureBase

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRStructureBase**

> Object for storing OpenXR structure data that is passed when calling into OpenXR APIs.

<details><summary>官方英文描述（中文精译进行中）</summary>

Object for storing OpenXR structure data that is passed when calling into OpenXR APIs.

</details>

## 方法

### _get_header {#_get_header}

```gdscript
int _get_header(next: int) virtual
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### get_next {#get_next}

```gdscript
OpenXRStructureBase get_next() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_structure_type {#get_structure_type}

```gdscript
int get_structure_type()
```

Returns the structure type (OpenXR XrStructureType) used for this structure.

### set_next {#set_next}

```gdscript
void set_next(entity: OpenXRStructureBase)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### next {#next}

```gdscript
next: OpenXRStructureBase
```

- void set_next(value: OpenXRStructureBase)
- OpenXRStructureBase get_next()

Setting another structure object here chains these structures together to extend the API functionality. Consult the OpenXR documentation for which structures can be used with a given API call.
