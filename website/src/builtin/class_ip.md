# IP

**继承**：[Object](class_object.md) → **IP**

> IP contains support functions for the Internet Protocol (IP). TCP/IP support is in different classes (see StreamPeerTCP and TCPServer). IP provides DNS hostname resolution support, both blocking and threaded.

<details><summary>官方英文描述</summary>

IP contains support functions for the Internet Protocol (IP). TCP/IP support is in different classes (see StreamPeerTCP and TCPServer). IP provides DNS hostname resolution support, both blocking and threaded.

</details>

## 方法

### clear_cache {#clear_cache}

```gdscript
void clear_cache(hostname: String = "")
```

Removes all of a hostname's cached references. If no hostname is given, all cached IP addresses are removed.

### erase_resolve_item {#erase_resolve_item}

```gdscript
void erase_resolve_item(id: int)
```

Removes a given item id from the queue. This should be used to free a queue after it has completed to enable more queries to happen.

### get_local_addresses {#get_local_addresses}

```gdscript
PackedStringArray get_local_addresses() const
```

Returns all the user's current IPv4 and IPv6 addresses as an array.

### get_local_interfaces {#get_local_interfaces}

```gdscript
typedarray::Dictionary get_local_interfaces() const
```

Returns all network adapters as an array.

Each adapter is a dictionary of the form:

```gdscript
{
    "index": "1", # Interface index.
    "name": "eth0", # Interface name.
    "friendly": "Ethernet One", # A friendly name (might be empty).
    "addresses": ["192.168.1.101"], # An array of IP addresses associated to this interface.
}
```

### get_resolve_item_address {#get_resolve_item_address}

```gdscript
String get_resolve_item_address(id: int) const
```

Returns a queued hostname's IP address, given its queue id. Returns an empty string on error or if resolution hasn't happened yet (see get_resolve_item_status()).

### get_resolve_item_addresses {#get_resolve_item_addresses}

```gdscript
Array get_resolve_item_addresses(id: int) const
```

Returns resolved addresses, or an empty array if an error happened or resolution didn't happen yet (see get_resolve_item_status()).

### get_resolve_item_status {#get_resolve_item_status}

```gdscript
IP.ResolverStatus get_resolve_item_status(id: int) const
```

Returns a queued hostname's status as a ResolverStatus constant, given its queue id.

### resolve_hostname {#resolve_hostname}

```gdscript
String resolve_hostname(host: String, ip_type: IP.Type = 3)
```

Returns a given hostname's IPv4 or IPv6 address when resolved (blocking-type method). The address type returned depends on the Type constant given as ip_type.

### resolve_hostname_addresses {#resolve_hostname_addresses}

```gdscript
PackedStringArray resolve_hostname_addresses(host: String, ip_type: IP.Type = 3)
```

Resolves a given hostname in a blocking way. Addresses are returned as an Array of IPv4 or IPv6 addresses depending on ip_type.

### resolve_hostname_queue_item {#resolve_hostname_queue_item}

```gdscript
int resolve_hostname_queue_item(host: String, ip_type: IP.Type = 3)
```

Creates a queue item to resolve a hostname to an IPv4 or IPv6 address depending on the Type constant given as ip_type. Returns the queue ID if successful, or RESOLVER_INVALID_ID on error.

## 枚举

### ResolverStatus {#enum-ResolverStatus}

```gdscript
enum ResolverStatus { RESOLVER_STATUS_NONE = 0, RESOLVER_STATUS_WAITING = 1, RESOLVER_STATUS_DONE = 2, RESOLVER_STATUS_ERROR = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### RESOLVER_STATUS_NONE {#RESOLVER_STATUS_NONE}

```gdscript
RESOLVER_STATUS_NONE = 0
```

DNS hostname resolver status: No status.

#### RESOLVER_STATUS_WAITING {#RESOLVER_STATUS_WAITING}

```gdscript
RESOLVER_STATUS_WAITING = 1
```

DNS hostname resolver status: Waiting.

#### RESOLVER_STATUS_DONE {#RESOLVER_STATUS_DONE}

```gdscript
RESOLVER_STATUS_DONE = 2
```

DNS hostname resolver status: Done.

#### RESOLVER_STATUS_ERROR {#RESOLVER_STATUS_ERROR}

```gdscript
RESOLVER_STATUS_ERROR = 3
```

DNS hostname resolver status: Error.

---

### Type {#enum-Type}

```gdscript
enum Type { TYPE_NONE = 0, TYPE_IPV4 = 1, TYPE_IPV6 = 2, TYPE_ANY = 3 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### TYPE_NONE {#TYPE_NONE}

```gdscript
TYPE_NONE = 0
```

Address type: None.

#### TYPE_IPV4 {#TYPE_IPV4}

```gdscript
TYPE_IPV4 = 1
```

Address type: Internet protocol version 4 (IPv4).

#### TYPE_IPV6 {#TYPE_IPV6}

```gdscript
TYPE_IPV6 = 2
```

Address type: Internet protocol version 6 (IPv6).

#### TYPE_ANY {#TYPE_ANY}

```gdscript
TYPE_ANY = 3
```

Address type: Any.

---

## 常量

### RESOLVER_MAX_QUERIES {#RESOLVER_MAX_QUERIES}

```gdscript
const RESOLVER_MAX_QUERIES = 256
```

Maximum number of concurrent DNS resolver queries allowed, RESOLVER_INVALID_ID is returned if exceeded.

RESOLVER_INVALID_ID = -1 

Invalid ID constant. Returned if RESOLVER_MAX_QUERIES is exceeded.

### RESOLVER_INVALID_ID {#RESOLVER_INVALID_ID}

```gdscript
const RESOLVER_INVALID_ID = -1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
