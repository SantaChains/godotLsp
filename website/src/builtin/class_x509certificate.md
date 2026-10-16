# X509Certificate

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **X509Certificate**

> The X509Certificate class represents an X509 certificate. Certificates can be loaded and saved like any other Resource.

<details><summary>官方英文描述（中文精译进行中）</summary>

The X509Certificate class represents an X509 certificate. Certificates can be loaded and saved like any other Resource.

They can be used as the server certificate in StreamPeerTLS.accept_stream() (along with the proper CryptoKey), and to specify the only certificate that should be accepted when connecting to a TLS server via StreamPeerTLS.connect_to_stream().

</details>

## 方法

### load {#load}

```gdscript
Error load(path: String)
```

Loads a certificate from path ("*.crt" file).

### load_from_string {#load_from_string}

```gdscript
Error load_from_string(string: String)
```

Loads a certificate from the given string.

### save {#save}

```gdscript
Error save(path: String)
```

Saves a certificate to the given path (should be a "*.crt" file).

### save_to_string {#save_to_string}

```gdscript
String save_to_string()
```

Returns a string representation of the certificate, or an empty string if the certificate is invalid.
