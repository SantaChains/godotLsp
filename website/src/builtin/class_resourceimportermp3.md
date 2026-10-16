# ResourceImporterMP3

**继承**：[ResourceImporter](class_resourceimporter.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **ResourceImporterMP3**

> MP3 is a lossy audio format, with worse audio quality compared to ResourceImporterOggVorbis at a given bitrate.

<details><summary>官方英文描述</summary>

MP3 is a lossy audio format, with worse audio quality compared to ResourceImporterOggVorbis at a given bitrate.

In most cases, it's recommended to use Ogg Vorbis over MP3. However, if you're using an MP3 sound source with no higher quality source available, then it's recommended to use the MP3 file directly to avoid double lossy compression.

MP3 requires more CPU to decode than ResourceImporterWAV. If you need to play a lot of simultaneous sounds, it's recommended to use WAV for those sounds instead, especially if targeting low-end devices.

</details>
