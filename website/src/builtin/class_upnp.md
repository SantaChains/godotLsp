# UPNP

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **UPNP**

> This class can be used to discover compatible UPNPDevices on the local network and execute commands on them, like managing port mappings (for port forwarding/NAT traversal) and querying the local and remote network IP address. Note that methods on this class are synchronous and block the calling thread.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class can be used to discover compatible UPNPDevices on the local network and execute commands on them, like managing port mappings (for port forwarding/NAT traversal) and querying the local and remote network IP address. Note that methods on this class are synchronous and block the calling thread.

To forward a specific port (here 7777, note both discover() and add_port_mapping() can return errors that should be checked):

```gdscript
var upnp = UPNP.new()
upnp.discover()
upnp.add_port_mapping(7777)
```

To close a specific port (e.g. after you have finished using it):

```gdscript
upnp.delete_port_mapping(port)
```

Note: UPnP discovery blocks the current thread. To perform discovery without blocking the main thread, use Threads like this:

```gdscript
# Emitted when UPnP port mapping setup is completed (regardless of success or failure).
signal upnp_completed(error)
# Replace this with your own server port number between 1024 and 65535.
const SERVER_PORT = 3928
var thread = null
func _upnp_setup(server_port):
    # UPNP queries take some time.
    var upnp = UPNP.new()
    var err = upnp.discover()
    if err != UPNP.UPNP_RESULT_SUCCESS:
        push_error(str(err))
        upnp_completed.emit(err)
        return
    if upnp.get_gateway() and upnp.get_gateway().is_valid_gateway():
        upnp.add_port_mapping(server_port, server_port, ProjectSettings.get_setting("application/config/name"), "UDP")
        upnp.add_port_mapping(server_port, server_port, ProjectSettings.get_setting("application/config/name"), "TCP")
        upnp_completed.emit(UPNP.UPNP_RESULT_SUCCESS)
func _ready():
    thread = Thread.new()
    thread.start(_upnp_setup.bind(SERVER_PORT))
func _exit_tree():
    # Wait for thread finish here to handle game exit while the thread is running.
    thread.wait_to_finish()
```

Terminology: In the context of UPnP networking, "gateway" (or "internet gateway device", short IGD) refers to network devices that allow computers in the local network to access the internet ("wide area network", WAN). These gateways are often also called "routers".

Pitfalls:

- As explained above, these calls are blocking and shouldn't be run on the main thread, especially as they can block for multiple seconds at a time. Use threading!
- Networking is physical and messy. Packets get lost in transit or get filtered, addresses, free ports and assigned mappings change, and devices may leave or join the network at any time. Be mindful of this, be diligent when checking and handling errors, and handle these gracefully if you can: add clear error UI, timeouts and re-try handling.
- Port mappings may change (and be removed) at any time, and the remote/external IP address of the gateway can change likewise. You should consider re-querying the external IP and try to update/refresh the port mapping periodically (for example, every 5 minutes and on networking failures).
- Not all devices support UPnP, and some users disable UPnP support. You need to handle this (e.g. documenting and requiring the user to manually forward ports, or adding alternative methods of NAT traversal, like a relay/mirror server, or NAT hole punching, STUN/TURN, etc.).
- Consider what happens on mapping conflicts. Maybe multiple users on the same network would like to play your game at the same time, or maybe another application uses the same port. Make the port configurable, and optimally choose a port automatically (re-trying with a different port on failure).

Further reading: If you want to know more about UPnP (and the Internet Gateway Device (IGD) and Port Control Protocol (PCP) specifically), Wikipedia [https://en.wikipedia.org/wiki/Universal_Plug_and_Play] is a good first stop, the specification can be found at the Open Connectivity Foundation [https://openconnectivity.org/developer/specifications/upnp-resources/upnp/] and Godot's implementation is based on the MiniUPnP client [https://github.com/miniupnp/miniupnp].

</details>

## 方法

### add_device {#add_device}

```gdscript
void add_device(device: UPNPDevice)
```

Adds the given UPNPDevice to the list of discovered devices.

### add_port_mapping {#add_port_mapping}

```gdscript
int add_port_mapping(port: int, port_internal: int = 0, desc: String = "", proto: String = "UDP", duration: int = 0) const
```

Adds a mapping to forward the external port (between 1 and 65535, although recommended to use port 1024 or above) on the default gateway (see get_gateway()) to the port_internal on the local machine for the given protocol proto (either "TCP" or "UDP", with UDP being the default). If a port mapping for the given port and protocol combination already exists on that gateway device, this method tries to overwrite it. If that is not desired, you can retrieve the gateway manually with get_gateway() and call add_port_mapping() on it, if any. Note that forwarding a well-known port (below 1024) with UPnP may fail depending on the device.

Depending on the gateway device, if a mapping for that port already exists, it will either be updated or it will refuse this command due to that conflict, especially if the existing mapping for that port wasn't created via UPnP or points to a different network address (or device) than this one.

If port_internal is 0 (the default), the same port number is used for both the external and the internal port (the port value).

The description (desc) is shown in some routers management UIs and can be used to point out which application added the mapping.

The mapping's lease duration can be limited by specifying a duration in seconds. The default of 0 means no duration, i.e. a permanent lease and notably some devices only support these permanent leases. Note that whether permanent or not, this is only a request and the gateway may still decide at any point to remove the mapping (which usually happens on a reboot of the gateway, when its external IP address changes, or on some models when it detects a port mapping has become inactive, i.e. had no traffic for multiple minutes). If not 0 (permanent), the allowed range according to spec is between 120 (2 minutes) and 86400 seconds (24 hours).

See UPNPResult for possible return values.

### clear_devices {#clear_devices}

```gdscript
void clear_devices()
```

Clears the list of discovered devices.

### delete_port_mapping {#delete_port_mapping}

```gdscript
int delete_port_mapping(port: int, proto: String = "UDP") const
```

Deletes the port mapping for the given port and protocol combination on the default gateway (see get_gateway()) if one exists. port must be a valid port between 1 and 65535, proto can be either "TCP" or "UDP". May be refused for mappings pointing to addresses other than this one, for well-known ports (below 1024), or for mappings not added via UPnP. See UPNPResult for possible return values.

### discover {#discover}

```gdscript
int discover(timeout: int = 2000, ttl: int = 2, device_filter: String = "InternetGatewayDevice")
```

Discovers local UPNPDevices. Clears the list of previously discovered devices.

Filters for IGD (InternetGatewayDevice) type devices by default, as those manage port forwarding. timeout is the time to wait for responses in milliseconds. ttl is the time-to-live; only touch this if you know what you're doing.

See UPNPResult for possible return values.

### get_device {#get_device}

```gdscript
UPNPDevice get_device(index: int) const
```

Returns the UPNPDevice at the given index.

### get_device_count {#get_device_count}

```gdscript
int get_device_count() const
```

Returns the number of discovered UPNPDevices.

### get_discover_local_port {#get_discover_local_port}

```gdscript
int get_discover_local_port() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_discover_multicast_if {#get_discover_multicast_if}

```gdscript
String get_discover_multicast_if() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_gateway {#get_gateway}

```gdscript
UPNPDevice get_gateway() const
```

Returns the default gateway. That is the first discovered UPNPDevice that is also a valid IGD (InternetGatewayDevice).

### is_discover_ipv6 {#is_discover_ipv6}

```gdscript
bool is_discover_ipv6() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### query_external_address {#query_external_address}

```gdscript
String query_external_address() const
```

Returns the external IP address of the default gateway (see get_gateway()) as string. Returns an empty string on error.

### remove_device {#remove_device}

```gdscript
void remove_device(index: int)
```

Removes the device at index from the list of discovered devices.

### set_device {#set_device}

```gdscript
void set_device(index: int, device: UPNPDevice)
```

Sets the device at index from the list of discovered devices to device.

### set_discover_ipv6 {#set_discover_ipv6}

```gdscript
void set_discover_ipv6(ipv6: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_discover_local_port {#set_discover_local_port}

```gdscript
void set_discover_local_port(port: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_discover_multicast_if {#set_discover_multicast_if}

```gdscript
void set_discover_multicast_if(m_if: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### discover_multicast_if {#discover_multicast_if}

```gdscript
discover_multicast_if: String
```

- void set_discover_multicast_if(value: String)
- String get_discover_multicast_if()

Multicast interface to use for discovery. Uses the default multicast interface if empty.

### discover_local_port {#discover_local_port}

```gdscript
discover_local_port: int
```

- void set_discover_local_port(value: int)
- int get_discover_local_port()

If 0, the local port to use for discovery is chosen automatically by the system. If 1, discovery will be done from the source port 1900 (same as destination port). Otherwise, the value will be used as the port.

### discover_ipv6 {#discover_ipv6}

```gdscript
discover_ipv6: bool
```

- void set_discover_ipv6(value: bool)
- bool is_discover_ipv6()

If true, IPv6 is used for UPNPDevice discovery.

## 枚举

### UPNPResult {#enum-UPNPResult}

```gdscript
enum UPNPResult { UPNP_RESULT_SUCCESS = 0, UPNP_RESULT_NOT_AUTHORIZED = 1, UPNP_RESULT_PORT_MAPPING_NOT_FOUND = 2, UPNP_RESULT_INCONSISTENT_PARAMETERS = 3, UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY = 4, UPNP_RESULT_ACTION_FAILED = 5, UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED = 6, UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED = 7, UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED = 8, UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD = 9, UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD = 10, UPNP_RESULT_NO_PORT_MAPS_AVAILABLE = 11, UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM = 12, UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING = 13, UPNP_RESULT_SAME_PORT_VALUES_REQUIRED = 14, UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED = 15, UPNP_RESULT_INVALID_GATEWAY = 16, UPNP_RESULT_INVALID_PORT = 17, UPNP_RESULT_INVALID_PROTOCOL = 18, UPNP_RESULT_INVALID_DURATION = 19, UPNP_RESULT_INVALID_ARGS = 20, UPNP_RESULT_INVALID_RESPONSE = 21, UPNP_RESULT_INVALID_PARAM = 22, UPNP_RESULT_HTTP_ERROR = 23, UPNP_RESULT_SOCKET_ERROR = 24, UPNP_RESULT_MEM_ALLOC_ERROR = 25, UPNP_RESULT_NO_GATEWAY = 26, UPNP_RESULT_NO_DEVICES = 27, UPNP_RESULT_UNKNOWN_ERROR = 28 }
```

UPNPResult UPNP_RESULT_SUCCESS = 0

UPNP command or discovery was successful.

UPNPResult UPNP_RESULT_NOT_AUTHORIZED = 1

Not authorized to use the command on the UPNPDevice. May be returned when the user disabled UPNP on their router.

UPNPResult UPNP_RESULT_PORT_MAPPING_NOT_FOUND = 2

No port mapping was found for the given port, protocol combination on the given UPNPDevice.

UPNPResult UPNP_RESULT_INCONSISTENT_PARAMETERS = 3

Inconsistent parameters.

UPNPResult UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY = 4

No such entry in array. May be returned if a given port, protocol combination is not found on a UPNPDevice.

UPNPResult UPNP_RESULT_ACTION_FAILED = 5

The action failed.

UPNPResult UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED = 6

The UPNPDevice does not allow wildcard values for the source IP address.

UPNPResult UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED = 7

The UPNPDevice does not allow wildcard values for the external port.

UPNPResult UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED = 8

The UPNPDevice does not allow wildcard values for the internal port.

UPNPResult UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD = 9

The remote host value must be a wildcard.

UPNPResult UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD = 10

The external port value must be a wildcard.

UPNPResult UPNP_RESULT_NO_PORT_MAPS_AVAILABLE = 11

No port maps are available. May also be returned if port mapping functionality is not available.

UPNPResult UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM = 12

Conflict with other mechanism. May be returned instead of UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING if a port mapping conflicts with an existing one.

UPNPResult UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING = 13

Conflict with an existing port mapping.

UPNPResult UPNP_RESULT_SAME_PORT_VALUES_REQUIRED = 14

External and internal port values must be the same.

UPNPResult UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED = 15

Only permanent leases are supported. Do not use the duration parameter when adding port mappings.

UPNPResult UPNP_RESULT_INVALID_GATEWAY = 16

Invalid gateway.

UPNPResult UPNP_RESULT_INVALID_PORT = 17

Invalid port.

UPNPResult UPNP_RESULT_INVALID_PROTOCOL = 18

Invalid protocol.

UPNPResult UPNP_RESULT_INVALID_DURATION = 19

Invalid duration.

UPNPResult UPNP_RESULT_INVALID_ARGS = 20

Invalid arguments.

UPNPResult UPNP_RESULT_INVALID_RESPONSE = 21

Invalid response.

UPNPResult UPNP_RESULT_INVALID_PARAM = 22

Invalid parameter.

UPNPResult UPNP_RESULT_HTTP_ERROR = 23

HTTP error.

UPNPResult UPNP_RESULT_SOCKET_ERROR = 24

Socket error.

UPNPResult UPNP_RESULT_MEM_ALLOC_ERROR = 25

Error allocating memory.

UPNPResult UPNP_RESULT_NO_GATEWAY = 26

No gateway available. You may need to call discover() first, or discovery didn't detect any valid IGDs (InternetGatewayDevices).

UPNPResult UPNP_RESULT_NO_DEVICES = 27

No devices available. You may need to call discover() first, or discovery didn't detect any valid UPNPDevices.

UPNPResult UPNP_RESULT_UNKNOWN_ERROR = 28

Unknown error.

#### UPNP_RESULT_SUCCESS {#UPNP_RESULT_SUCCESS}

```gdscript
UPNP_RESULT_SUCCESS = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_NOT_AUTHORIZED {#UPNP_RESULT_NOT_AUTHORIZED}

```gdscript
UPNP_RESULT_NOT_AUTHORIZED = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_PORT_MAPPING_NOT_FOUND {#UPNP_RESULT_PORT_MAPPING_NOT_FOUND}

```gdscript
UPNP_RESULT_PORT_MAPPING_NOT_FOUND = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INCONSISTENT_PARAMETERS {#UPNP_RESULT_INCONSISTENT_PARAMETERS}

```gdscript
UPNP_RESULT_INCONSISTENT_PARAMETERS = 3
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY {#UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY}

```gdscript
UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY = 4
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_ACTION_FAILED {#UPNP_RESULT_ACTION_FAILED}

```gdscript
UPNP_RESULT_ACTION_FAILED = 5
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED {#UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED}

```gdscript
UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED = 6
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED {#UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED}

```gdscript
UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED = 7
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED {#UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED}

```gdscript
UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED = 8
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD {#UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD}

```gdscript
UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD {#UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD}

```gdscript
UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD = 10
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_NO_PORT_MAPS_AVAILABLE {#UPNP_RESULT_NO_PORT_MAPS_AVAILABLE}

```gdscript
UPNP_RESULT_NO_PORT_MAPS_AVAILABLE = 11
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM {#UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM}

```gdscript
UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM = 12
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING {#UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING}

```gdscript
UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING = 13
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_SAME_PORT_VALUES_REQUIRED {#UPNP_RESULT_SAME_PORT_VALUES_REQUIRED}

```gdscript
UPNP_RESULT_SAME_PORT_VALUES_REQUIRED = 14
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED {#UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED}

```gdscript
UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED = 15
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INVALID_GATEWAY {#UPNP_RESULT_INVALID_GATEWAY}

```gdscript
UPNP_RESULT_INVALID_GATEWAY = 16
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INVALID_PORT {#UPNP_RESULT_INVALID_PORT}

```gdscript
UPNP_RESULT_INVALID_PORT = 17
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INVALID_PROTOCOL {#UPNP_RESULT_INVALID_PROTOCOL}

```gdscript
UPNP_RESULT_INVALID_PROTOCOL = 18
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INVALID_DURATION {#UPNP_RESULT_INVALID_DURATION}

```gdscript
UPNP_RESULT_INVALID_DURATION = 19
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INVALID_ARGS {#UPNP_RESULT_INVALID_ARGS}

```gdscript
UPNP_RESULT_INVALID_ARGS = 20
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INVALID_RESPONSE {#UPNP_RESULT_INVALID_RESPONSE}

```gdscript
UPNP_RESULT_INVALID_RESPONSE = 21
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_INVALID_PARAM {#UPNP_RESULT_INVALID_PARAM}

```gdscript
UPNP_RESULT_INVALID_PARAM = 22
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_HTTP_ERROR {#UPNP_RESULT_HTTP_ERROR}

```gdscript
UPNP_RESULT_HTTP_ERROR = 23
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_SOCKET_ERROR {#UPNP_RESULT_SOCKET_ERROR}

```gdscript
UPNP_RESULT_SOCKET_ERROR = 24
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_MEM_ALLOC_ERROR {#UPNP_RESULT_MEM_ALLOC_ERROR}

```gdscript
UPNP_RESULT_MEM_ALLOC_ERROR = 25
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_NO_GATEWAY {#UPNP_RESULT_NO_GATEWAY}

```gdscript
UPNP_RESULT_NO_GATEWAY = 26
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_NO_DEVICES {#UPNP_RESULT_NO_DEVICES}

```gdscript
UPNP_RESULT_NO_DEVICES = 27
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### UPNP_RESULT_UNKNOWN_ERROR {#UPNP_RESULT_UNKNOWN_ERROR}

```gdscript
UPNP_RESULT_UNKNOWN_ERROR = 28
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
