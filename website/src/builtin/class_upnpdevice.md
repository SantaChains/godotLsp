# UPNPDevice

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **UPNPDevice**

> Universal Plug and Play (UPnP) device. See UPNP for UPnP discovery and utility functions. Provides low-level access to UPNP control commands. Allows to manage port mappings (port forwarding) and to query network information of the device (like local and external IP address and status). Note that methods on this class are synchronous and block the calling thread.

<details><summary>官方英文描述（中文精译进行中）</summary>

Universal Plug and Play (UPnP) device. See UPNP for UPnP discovery and utility functions. Provides low-level access to UPNP control commands. Allows to manage port mappings (port forwarding) and to query network information of the device (like local and external IP address and status). Note that methods on this class are synchronous and block the calling thread.

</details>

## 方法

### add_port_mapping {#add_port_mapping}

```gdscript
int add_port_mapping(port: int, port_internal: int = 0, desc: String = "", proto: String = "UDP", duration: int = 0) const
```

Adds a port mapping to forward the given external port on this UPNPDevice for the given protocol to the local machine. See UPNP.add_port_mapping().

### delete_port_mapping {#delete_port_mapping}

```gdscript
int delete_port_mapping(port: int, proto: String = "UDP") const
```

Deletes the port mapping identified by the given port and protocol combination on this device. See UPNP.delete_port_mapping().

### get_description_url {#get_description_url}

```gdscript
String get_description_url() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_igd_control_url {#get_igd_control_url}

```gdscript
String get_igd_control_url() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_igd_our_addr {#get_igd_our_addr}

```gdscript
String get_igd_our_addr() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_igd_service_type {#get_igd_service_type}

```gdscript
String get_igd_service_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_igd_status {#get_igd_status}

```gdscript
UPNPDevice.IGDStatus get_igd_status() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_service_type {#get_service_type}

```gdscript
String get_service_type() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_valid_gateway {#is_valid_gateway}

```gdscript
bool is_valid_gateway() const
```

Returns true if this is a valid IGD (InternetGatewayDevice) which potentially supports port forwarding.

### query_external_address {#query_external_address}

```gdscript
String query_external_address() const
```

Returns the external IP address of this UPNPDevice or an empty string.

### set_description_url {#set_description_url}

```gdscript
void set_description_url(url: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_igd_control_url {#set_igd_control_url}

```gdscript
void set_igd_control_url(url: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_igd_our_addr {#set_igd_our_addr}

```gdscript
void set_igd_our_addr(addr: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_igd_service_type {#set_igd_service_type}

```gdscript
void set_igd_service_type(type: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_igd_status {#set_igd_status}

```gdscript
void set_igd_status(status: UPNPDevice.IGDStatus)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_service_type {#set_service_type}

```gdscript
void set_service_type(type: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### description_url {#description_url}

```gdscript
description_url: String
```

- void set_description_url(value: String)
- String get_description_url()

URL to the device description.

### service_type {#service_type}

```gdscript
service_type: String
```

- void set_service_type(value: String)
- String get_service_type()

Service type.

### igd_control_url {#igd_control_url}

```gdscript
igd_control_url: String
```

- void set_igd_control_url(value: String)
- String get_igd_control_url()

IDG control URL.

### igd_service_type {#igd_service_type}

```gdscript
igd_service_type: String
```

- void set_igd_service_type(value: String)
- String get_igd_service_type()

IGD service type.

### igd_our_addr {#igd_our_addr}

```gdscript
igd_our_addr: String
```

- void set_igd_our_addr(value: String)
- String get_igd_our_addr()

Address of the local machine in the network connecting it to this UPNPDevice.

### igd_status {#igd_status}

```gdscript
igd_status: int
```

- void set_igd_status(value: IGDStatus)
- IGDStatus get_igd_status()

IGD status.

## 枚举

### IGDStatus {#enum-IGDStatus}

```gdscript
enum IGDStatus { IGD_STATUS_OK = 0, IGD_STATUS_HTTP_ERROR = 1, IGD_STATUS_HTTP_EMPTY = 2, IGD_STATUS_NO_URLS = 3, IGD_STATUS_NO_IGD = 4, IGD_STATUS_DISCONNECTED = 5, IGD_STATUS_UNKNOWN_DEVICE = 6, IGD_STATUS_INVALID_CONTROL = 7, IGD_STATUS_MALLOC_ERROR = 8, IGD_STATUS_UNKNOWN_ERROR = 9 }
```

IGDStatus IGD_STATUS_OK = 0

OK.

IGDStatus IGD_STATUS_HTTP_ERROR = 1

HTTP error.

IGDStatus IGD_STATUS_HTTP_EMPTY = 2

Empty HTTP response.

IGDStatus IGD_STATUS_NO_URLS = 3

Deprecated: This value is no longer used.

Returned response contained no URLs.

IGDStatus IGD_STATUS_NO_IGD = 4

Not a valid IGD.

IGDStatus IGD_STATUS_DISCONNECTED = 5

Disconnected.

IGDStatus IGD_STATUS_UNKNOWN_DEVICE = 6

Unknown device.

IGDStatus IGD_STATUS_INVALID_CONTROL = 7

Invalid control.

IGDStatus IGD_STATUS_MALLOC_ERROR = 8

Deprecated: This value is no longer used.

Memory allocation error.

IGDStatus IGD_STATUS_UNKNOWN_ERROR = 9

Unknown error.

#### IGD_STATUS_OK {#IGD_STATUS_OK}

```gdscript
IGD_STATUS_OK = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_HTTP_ERROR {#IGD_STATUS_HTTP_ERROR}

```gdscript
IGD_STATUS_HTTP_ERROR = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_HTTP_EMPTY {#IGD_STATUS_HTTP_EMPTY}

```gdscript
IGD_STATUS_HTTP_EMPTY = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_NO_URLS {#IGD_STATUS_NO_URLS}

```gdscript
IGD_STATUS_NO_URLS = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_NO_IGD {#IGD_STATUS_NO_IGD}

```gdscript
IGD_STATUS_NO_IGD = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_DISCONNECTED {#IGD_STATUS_DISCONNECTED}

```gdscript
IGD_STATUS_DISCONNECTED = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_UNKNOWN_DEVICE {#IGD_STATUS_UNKNOWN_DEVICE}

```gdscript
IGD_STATUS_UNKNOWN_DEVICE = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_INVALID_CONTROL {#IGD_STATUS_INVALID_CONTROL}

```gdscript
IGD_STATUS_INVALID_CONTROL = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_MALLOC_ERROR {#IGD_STATUS_MALLOC_ERROR}

```gdscript
IGD_STATUS_MALLOC_ERROR = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### IGD_STATUS_UNKNOWN_ERROR {#IGD_STATUS_UNKNOWN_ERROR}

```gdscript
IGD_STATUS_UNKNOWN_ERROR = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
