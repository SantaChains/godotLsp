# NavigationServer3DManager

**继承**：[Object](class_object.md) → **NavigationServer3DManager**

> NavigationServer3DManager is the API for registering NavigationServer3D implementations and setting the default implementation.

<details><summary>官方英文描述（中文精译进行中）</summary>

NavigationServer3DManager is the API for registering NavigationServer3D implementations and setting the default implementation.

Note: It is not possible to switch servers at runtime. This class is only used on startup at the server initialization level.

</details>

## 方法

### register_server {#register_server}

```gdscript
void register_server(name: String, create_callback: Callable)
```

Registers a NavigationServer3D implementation by passing a name and a Callable that returns a NavigationServer3D object.

### set_default_server {#set_default_server}

```gdscript
void set_default_server(name: String, priority: int)
```

Sets the default NavigationServer3D implementation to the one identified by name, if priority is greater than the priority of the current default implementation.
