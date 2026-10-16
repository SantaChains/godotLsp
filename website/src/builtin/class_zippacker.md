# ZIPPacker

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **ZIPPacker**

> This class implements a writer that allows storing the multiple blobs in a ZIP archive. See also ZIPReader and PCKPacker.

<details><summary>官方英文描述（中文精译进行中）</summary>

This class implements a writer that allows storing the multiple blobs in a ZIP archive. See also ZIPReader and PCKPacker.

```gdscript
# Create a ZIP archive with a single file at its root.
func write_zip_file():
    var writer = ZIPPacker.new()
    var err = writer.open("user://archive.zip")
    if err != OK:
        return err
    writer.start_file("hello.txt")
    writer.write_file("Hello World".to_utf8_buffer())
    writer.close_file()
    writer.close()
    return OK
```

</details>

## 方法

### add_directory {#add_directory}

```gdscript
Error add_directory(path: String, permissions: bitfield::FileAccess.UnixPermissionFlags = 493, modified_time: int = 0)
```

Adds directory to the archive. If modified_time is set to 0, current system time is used.

Note: Directories are automatically created when start_file() is called, use this function before adding files to create directories with custom permissions and modification time.

### close {#close}

```gdscript
Error close()
```

Closes the underlying resources used by this instance.

### close_file {#close_file}

```gdscript
Error close_file()
```

Stops writing to a file within the archive.

It will fail if there is no open file.

### get_compression_level {#get_compression_level}

```gdscript
int get_compression_level() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### open {#open}

```gdscript
Error open(path: String, append: ZIPPacker.ZipAppend = 0)
```

Opens a zip file for writing at the given path using the specified write mode.

This must be called before everything else.

### set_compression_level {#set_compression_level}

```gdscript
void set_compression_level(compression_level: int)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### start_file {#start_file}

```gdscript
Error start_file(path: String, permissions: bitfield::FileAccess.UnixPermissionFlags = 420, modified_time: int = 0)
```

Starts writing to a file within the archive. Only one file can be written at the same time. If modified_time is set to 0, current system time is used.

Must be called after open().

### write_file {#write_file}

```gdscript
Error write_file(data: PackedByteArray)
```

Write the given data to the file.

Needs to be called after start_file().

## 属性

### compression_level {#compression_level}

```gdscript
compression_level: int
```

- void set_compression_level(value: int)
- int get_compression_level()

The compression level used when start_file() is called. Use CompressionLevel as a reference.

## 枚举

### ZipAppend {#enum-ZipAppend}

```gdscript
enum ZipAppend { APPEND_CREATE = 0, APPEND_CREATEAFTER = 1, APPEND_ADDINZIP = 2 }
```

ZipAppend APPEND_CREATE = 0

Create a new zip archive at the given path.

ZipAppend APPEND_CREATEAFTER = 1

Append a new zip archive to the end of the already existing file at the given path.

ZipAppend APPEND_ADDINZIP = 2

Add new files to the existing zip archive at the given path.

#### APPEND_CREATE {#APPEND_CREATE}

```gdscript
APPEND_CREATE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### APPEND_CREATEAFTER {#APPEND_CREATEAFTER}

```gdscript
APPEND_CREATEAFTER = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### APPEND_ADDINZIP {#APPEND_ADDINZIP}

```gdscript
APPEND_ADDINZIP = 2
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### CompressionLevel {#enum-CompressionLevel}

```gdscript
enum CompressionLevel { COMPRESSION_DEFAULT = -1, COMPRESSION_NONE = 0, COMPRESSION_FAST = 1, COMPRESSION_BEST = 9 }
```

CompressionLevel COMPRESSION_DEFAULT = -1

Start a file with the default Deflate compression level (6). This is a good compromise between speed and file size.

CompressionLevel COMPRESSION_NONE = 0

Start a file with no compression. This is also known as the "Store" compression mode and is the fastest method of packing files inside a ZIP archive. Consider using this mode for files that are already compressed (such as JPEG, PNG, MP3, or Ogg Vorbis files).

CompressionLevel COMPRESSION_FAST = 1

Start a file with the fastest Deflate compression level (1). This is fast to compress, but results in larger file sizes than COMPRESSION_DEFAULT. Decompression speed is generally unaffected by the chosen compression level.

CompressionLevel COMPRESSION_BEST = 9

Start a file with the best Deflate compression level (9). This is slow to compress, but results in smaller file sizes than COMPRESSION_DEFAULT. Decompression speed is generally unaffected by the chosen compression level.

#### COMPRESSION_DEFAULT {#COMPRESSION_DEFAULT}

```gdscript
COMPRESSION_DEFAULT = -1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COMPRESSION_NONE {#COMPRESSION_NONE}

```gdscript
COMPRESSION_NONE = 0
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COMPRESSION_FAST {#COMPRESSION_FAST}

```gdscript
COMPRESSION_FAST = 1
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### COMPRESSION_BEST {#COMPRESSION_BEST}

```gdscript
COMPRESSION_BEST = 9
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。
