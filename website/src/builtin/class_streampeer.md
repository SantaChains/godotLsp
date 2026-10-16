# StreamPeer

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **StreamPeer**

> StreamPeer is an abstract base class mostly used for stream-based protocols (such as TCP). It provides an API for sending and receiving data through streams as raw data or strings.

<details><summary>官方英文描述（中文精译进行中）</summary>

StreamPeer is an abstract base class mostly used for stream-based protocols (such as TCP). It provides an API for sending and receiving data through streams as raw data or strings.

Note: When exporting to Android, make sure to enable the INTERNET permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android.

</details>

## 方法

### get_16 {#get_16}

```gdscript
int get_16()
```

Gets a signed 16-bit value from the stream.

### get_32 {#get_32}

```gdscript
int get_32()
```

Gets a signed 32-bit value from the stream.

### get_64 {#get_64}

```gdscript
int get_64()
```

Gets a signed 64-bit value from the stream.

### get_8 {#get_8}

```gdscript
int get_8()
```

Gets a signed byte from the stream.

### get_available_bytes {#get_available_bytes}

```gdscript
int get_available_bytes() const
```

Returns the number of bytes this StreamPeer has available.

### get_data {#get_data}

```gdscript
Array get_data(bytes: int)
```

Returns a chunk data with the received bytes, as an Array containing two elements: an Error constant and a PackedByteArray. bytes is the number of bytes to be received. If not enough bytes are available, the function will block until the desired amount is received.

### get_double {#get_double}

```gdscript
float get_double()
```

Gets a double-precision float from the stream.

### get_float {#get_float}

```gdscript
float get_float()
```

Gets a single-precision float from the stream.

### get_half {#get_half}

```gdscript
float get_half()
```

Gets a half-precision float from the stream.

### get_partial_data {#get_partial_data}

```gdscript
Array get_partial_data(bytes: int)
```

Returns a chunk data with the received bytes, as an Array containing two elements: an Error constant and a PackedByteArray. bytes is the number of bytes to be received. If not enough bytes are available, the function will return how many were actually received.

### get_string {#get_string}

```gdscript
String get_string(bytes: int = -1)
```

Gets an ASCII string with byte-length bytes from the stream. If bytes is negative (default) the length will be read from the stream using the reverse process of put_string().

### get_u16 {#get_u16}

```gdscript
int get_u16()
```

Gets an unsigned 16-bit value from the stream.

### get_u32 {#get_u32}

```gdscript
int get_u32()
```

Gets an unsigned 32-bit value from the stream.

### get_u64 {#get_u64}

```gdscript
int get_u64()
```

Gets an unsigned 64-bit value from the stream.

### get_u8 {#get_u8}

```gdscript
int get_u8()
```

Gets an unsigned byte from the stream.

### get_utf8_string {#get_utf8_string}

```gdscript
String get_utf8_string(bytes: int = -1)
```

Gets a UTF-8 string with byte-length bytes from the stream (this decodes the string sent as UTF-8). If bytes is negative (default) the length will be read from the stream using the reverse process of put_utf8_string().

### get_var {#get_var}

```gdscript
Variant get_var(allow_objects: bool = false)
```

Gets a Variant from the stream. If allow_objects is true, decoding objects is allowed.

Internally, this uses the same decoding mechanism as the @GlobalScope.bytes_to_var() method.

Warning: Deserialized objects can contain code which gets executed. Do not use this option if the serialized object comes from untrusted sources to avoid potential security threats such as remote code execution.

### is_big_endian_enabled {#is_big_endian_enabled}

```gdscript
bool is_big_endian_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### put_16 {#put_16}

```gdscript
void put_16(value: int)
```

Puts a signed 16-bit value into the stream.

### put_32 {#put_32}

```gdscript
void put_32(value: int)
```

Puts a signed 32-bit value into the stream.

### put_64 {#put_64}

```gdscript
void put_64(value: int)
```

Puts a signed 64-bit value into the stream.

### put_8 {#put_8}

```gdscript
void put_8(value: int)
```

Puts a signed byte into the stream.

### put_data {#put_data}

```gdscript
Error put_data(data: PackedByteArray)
```

Sends a chunk of data through the connection, blocking if necessary until the data is done sending. This function returns an Error code.

### put_double {#put_double}

```gdscript
void put_double(value: float)
```

Puts a double-precision float into the stream.

### put_float {#put_float}

```gdscript
void put_float(value: float)
```

Puts a single-precision float into the stream.

### put_half {#put_half}

```gdscript
void put_half(value: float)
```

Puts a half-precision float into the stream.

### put_partial_data {#put_partial_data}

```gdscript
Array put_partial_data(data: PackedByteArray)
```

Sends a chunk of data through the connection. If all the data could not be sent at once, only part of it will. This function returns two values, an Error code and an integer, describing how much data was actually sent.

### put_string {#put_string}

```gdscript
void put_string(value: String)
```

Puts a zero-terminated ASCII string into the stream prepended by a 32-bit unsigned integer representing its size.

Note: To put an ASCII string without prepending its size, you can use put_data():

```
put_data("Hello world".to_ascii_buffer())
```

```
PutData("Hello World".ToAsciiBuffer());
```

### put_u16 {#put_u16}

```gdscript
void put_u16(value: int)
```

Puts an unsigned 16-bit value into the stream.

### put_u32 {#put_u32}

```gdscript
void put_u32(value: int)
```

Puts an unsigned 32-bit value into the stream.

### put_u64 {#put_u64}

```gdscript
void put_u64(value: int)
```

Puts an unsigned 64-bit value into the stream.

### put_u8 {#put_u8}

```gdscript
void put_u8(value: int)
```

Puts an unsigned byte into the stream.

### put_utf8_string {#put_utf8_string}

```gdscript
void put_utf8_string(value: String)
```

Puts a zero-terminated UTF-8 string into the stream prepended by a 32 bits unsigned integer representing its size.

Note: To put a UTF-8 string without prepending its size, you can use put_data():

```
put_data("Hello world".to_utf8_buffer())
```

```
PutData("Hello World".ToUtf8Buffer());
```

### put_var {#put_var}

```gdscript
void put_var(value: Variant, full_objects: bool = false)
```

Puts a Variant into the stream. If full_objects is true encoding objects is allowed (and can potentially include code).

Internally, this uses the same encoding mechanism as the @GlobalScope.var_to_bytes() method.

### set_big_endian {#set_big_endian}

```gdscript
void set_big_endian(enable: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### big_endian {#big_endian}

```gdscript
big_endian: bool
```

- void set_big_endian(value: bool)
- bool is_big_endian_enabled()

If true, this StreamPeer will using big-endian format for encoding and decoding.
