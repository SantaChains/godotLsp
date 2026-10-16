# CryptoKey

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **CryptoKey**

> The CryptoKey class represents a cryptographic key. Keys can be loaded and saved like any other Resource.

<details><summary>官方英文描述</summary>

The CryptoKey class represents a cryptographic key. Keys can be loaded and saved like any other Resource.

They can be used to generate a self-signed X509Certificate via Crypto.generate_self_signed_certificate() and as private key in StreamPeerTLS.accept_stream() along with the appropriate certificate.

</details>

## 方法

### is_public_only {#is_public_only}

```gdscript
bool is_public_only() const
```

Returns true if this CryptoKey only has the public part, and not the private one.

### load {#load}

```gdscript
Error load(path: String, public_only: bool = false)
```

Loads a key from path. If public_only is true, only the public key will be loaded.

Note: path should be a "*.pub" file if public_only is true, a "*.key" file otherwise.

### load_from_string {#load_from_string}

```gdscript
Error load_from_string(string_key: String, public_only: bool = false)
```

Loads a key from the given string_key. If public_only is true, only the public key will be loaded.

### save {#save}

```gdscript
Error save(path: String, public_only: bool = false)
```

Saves a key to the given path. If public_only is true, only the public key will be saved.

Note: path should be a "*.pub" file if public_only is true, a "*.key" file otherwise.

### save_to_string {#save_to_string}

```gdscript
String save_to_string(public_only: bool = false)
```

Returns a string containing the key in PEM format. If public_only is true, only the public key will be included.
