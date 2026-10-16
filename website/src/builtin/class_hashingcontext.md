# HashingContext

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **HashingContext**

> The HashingContext class provides an interface for computing cryptographic hashes over multiple iterations. Useful for computing hashes of big files (so you don't have to load them all in memory), network streams, and data streams in general (so you don't have to hold buffers).

<details><summary>官方英文描述</summary>

The HashingContext class provides an interface for computing cryptographic hashes over multiple iterations. Useful for computing hashes of big files (so you don't have to load them all in memory), network streams, and data streams in general (so you don't have to hold buffers).

The HashType enum shows the supported hashing algorithms.

```
const CHUNK_SIZE = 1024
func hash_file(path):
    # Check that file exists.
    if not FileAccess.file_exists(path):
        return
    # Start an SHA-256 context.
    var ctx = HashingContext.new()
    ctx.start(HashingContext.HASH_SHA256)
    # Open the file to hash.
    var file = FileAccess.open(path, FileAccess.READ)
    # Update the context after reading each chunk.
    while file.get_position() < file.get_length():
        var remaining = file.get_length() - file.get_position()
        ctx.update(file.get_buffer(min(remaining, CHUNK_SIZE)))
    # Get the computed hash.
    var res = ctx.finish()
    # Print the result as hex string and array.
    printt(res.hex_encode(), Array(res))
```

```
public const int ChunkSize = 1024;
public void HashFile(string path)
{
    // Check that file exists.
    if (!FileAccess.FileExists(path))
    {
        return;
    }
    // Start an SHA-256 context.
    var ctx = new HashingContext();
    ctx.Start(HashingContext.HashType.Sha256);
    // Open the file to hash.
    using var file = FileAccess.Open(path, FileAccess.ModeFlags.Read);
    // Update the context after reading each chunk.
    while (file.GetPosition() < file.GetLength())
    {
        int remaining = (int)(file.GetLength() - file.GetPosition());
        ctx.Update(file.GetBuffer(Mathf.Min(remaining, ChunkSize)));
    }
    // Get the computed hash.
    byte[] res = ctx.Finish();
    // Print the result as hex string and array.
    GD.PrintT(res.HexEncode(), (Variant)res);
}
```

</details>

## 方法

### finish {#finish}

```gdscript
PackedByteArray finish()
```

Closes the current context, and return the computed hash.

### start {#start}

```gdscript
Error start(type: HashingContext.HashType)
```

Starts a new hash computation of the given type (e.g. HASH_SHA256 to start computation of an SHA-256).

### update {#update}

```gdscript
Error update(chunk: PackedByteArray)
```

Updates the computation with the given chunk of data.

## 枚举

### HashType {#enum-HashType}

```gdscript
enum HashType { HASH_MD5 = 0, HASH_SHA1 = 1, HASH_SHA256 = 2 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### HASH_MD5 {#HASH_MD5}

```gdscript
HASH_MD5 = 0
```

Hashing algorithm: MD5.

#### HASH_SHA1 {#HASH_SHA1}

```gdscript
HASH_SHA1 = 1
```

Hashing algorithm: SHA-1.

#### HASH_SHA256 {#HASH_SHA256}

```gdscript
HASH_SHA256 = 2
```

Hashing algorithm: SHA-256.

---
