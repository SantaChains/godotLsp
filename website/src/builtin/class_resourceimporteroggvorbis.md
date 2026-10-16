# ResourceImporterOggVorbis

**继承**：[ResourceImporter](class_resourceimporter.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ResourceImporterOggVorbis**

> Ogg Vorbis is a lossy audio format, with better audio quality compared to ResourceImporterMP3 at a given bitrate.

<details><summary>官方英文描述</summary>

Ogg Vorbis is a lossy audio format, with better audio quality compared to ResourceImporterMP3 at a given bitrate.

In most cases, it's recommended to use Ogg Vorbis over MP3. However, if you're using an MP3 sound source with no higher quality source available, then it's recommended to use the MP3 file directly to avoid double lossy compression.

Ogg Vorbis requires more CPU to decode than ResourceImporterWAV. If you need to play a lot of simultaneous sounds, it's recommended to use WAV for those sounds instead, especially if targeting low-end devices.

</details>

## 方法

### load_from_buffer {#load_from_buffer}

```gdscript
static AudioStreamOggVorbis load_from_buffer(stream_data: PackedByteArray)
```

Deprecated: Use AudioStreamOggVorbis.load_from_buffer() instead.

Creates a new AudioStreamOggVorbis instance from the given buffer. The buffer must contain Ogg Vorbis data.

### load_from_file {#load_from_file}

```gdscript
static AudioStreamOggVorbis load_from_file(path: String)
```

Deprecated: Use AudioStreamOggVorbis.load_from_file() instead.

Creates a new AudioStreamOggVorbis instance from the given file path. The file must be in Ogg Vorbis format.
