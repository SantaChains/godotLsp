# StreamPeerGZIP

**继承**：[StreamPeer](class_streampeer.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **StreamPeerGZIP**

> This class allows to compress or decompress data using GZIP/deflate in a streaming fashion. This is particularly useful when compressing or decompressing files that have to be sent through the network without needing to allocate them all in memory.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class allows to compress or decompress data using GZIP/deflate in a streaming fashion. This is particularly useful when compressing or decompressing files that have to be sent through the network without needing to allocate them all in memory.

After starting the stream via start_compression() (or start_decompression()), calling StreamPeer.put_partial_data() on this stream will compress (or decompress) the data, writing it to the internal buffer. Calling StreamPeer.get_available_bytes() will return the pending bytes in the internal buffer, and StreamPeer.get_partial_data() will retrieve the compressed (or decompressed) bytes from it. When the stream is over, you must call finish() to ensure the internal buffer is properly flushed (make sure to call StreamPeer.get_available_bytes() on last time to check if more data needs to be read after that).

</details>

## 方法

### clear {#clear}

```gdscript
void clear()
```

Clears this stream, resetting the internal state.

### finish {#finish}

```gdscript
Error finish()
```

Finalizes the stream, compressing any buffered chunk left.

You must call it only when you are compressing.

### start_compression {#start_compression}

```gdscript
Error start_compression(use_deflate: bool = false, buffer_size: int = 65535)
```

Start the stream in compression mode with the given buffer_size, if use_deflate is true uses deflate instead of GZIP.

### start_decompression {#start_decompression}

```gdscript
Error start_decompression(use_deflate: bool = false, buffer_size: int = 65535)
```

Start the stream in decompression mode with the given buffer_size, if use_deflate is true uses deflate instead of GZIP.
