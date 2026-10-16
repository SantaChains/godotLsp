# OpenXRBindingModifier

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **OpenXRBindingModifier**

> Binding modifier base class. Subclasses implement various modifiers that alter how an OpenXR runtime processes inputs.

<details><summary>官方英文描述</summary>

Binding modifier base class. Subclasses implement various modifiers that alter how an OpenXR runtime processes inputs.

</details>

## 方法

### _get_description {#_get_description}

```gdscript
String _get_description() virtual const
```

Return the description of this class that is used for the title bar of the binding modifier editor.

### _get_ip_modification {#_get_ip_modification}

```gdscript
PackedByteArray _get_ip_modification() virtual
```

Returns the data that is sent to OpenXR when submitting the suggested interacting bindings this modifier is a part of.

Note: This must be data compatible with an XrBindingModificationBaseHeaderKHR structure.
