# TextServerManager

**继承**：[Object](class_object.md) → **TextServerManager**

> TextServerManager is the API backend for loading, enumerating, and switching TextServers.

<details><summary>官方英文描述（中文精译进行中）</summary>

TextServerManager is the API backend for loading, enumerating, and switching TextServers.

Note: Switching text server at runtime is possible, but will invalidate all fonts and text buffers. Make sure to unload all controls, fonts, and themes before doing so.

</details>

## 方法

### add_interface {#add_interface}

```gdscript
void add_interface(interface: TextServer)
```

Registers a TextServer interface.

### find_interface {#find_interface}

```gdscript
TextServer find_interface(name: String) const
```

Finds an interface by its name.

### get_interface {#get_interface}

```gdscript
TextServer get_interface(idx: int) const
```

Returns the interface registered at a given index.

### get_interface_count {#get_interface_count}

```gdscript
int get_interface_count() const
```

Returns the number of interfaces currently registered.

### get_interfaces {#get_interfaces}

```gdscript
typedarray::Dictionary get_interfaces() const
```

Returns a list of available interfaces, with the index and name of each interface.

### get_primary_interface {#get_primary_interface}

```gdscript
TextServer get_primary_interface() const
```

Returns the primary TextServer interface currently in use.

### remove_interface {#remove_interface}

```gdscript
void remove_interface(interface: TextServer)
```

Removes an interface. All fonts and shaped text caches should be freed before removing an interface.

### set_primary_interface {#set_primary_interface}

```gdscript
void set_primary_interface(index: TextServer)
```

Sets the primary TextServer interface.

## 信号

### interface_added {#interface_added}

```gdscript
signal interface_added(interface_name: StringName)
```

Emitted when a new interface has been added.

### interface_removed {#interface_removed}

```gdscript
signal interface_removed(interface_name: StringName)
```

Emitted when an interface is removed.
