# TLSOptions

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **TLSOptions**

> TLSOptions abstracts the configuration options for the StreamPeerTLS and PacketPeerDTLS classes.

<details><summary>官方英文描述（中文精译进行中）</summary>

TLSOptions abstracts the configuration options for the StreamPeerTLS and PacketPeerDTLS classes.

Objects of this class cannot be instantiated directly, and one of the static methods client(), client_unsafe(), or server() should be used instead.

```
# Create a TLS client configuration which uses our custom trusted CA chain.
var client_trusted_cas = load("res://my_trusted_cas.crt")
var client_tls_options = TLSOptions.client(client_trusted_cas)
# Create a TLS server configuration.
var server_certs = load("res://my_server_cas.crt")
var server_key = load("res://my_server_key.key")
var server_tls_options = TLSOptions.server(server_key, server_certs)
```

</details>

## 方法

### client {#client}

```gdscript
static TLSOptions client(trusted_chain: X509Certificate = null, common_name_override: String = "")
```

Creates a TLS client configuration which validates certificates and their common names (fully qualified domain names).

You can specify a custom trusted_chain of certification authorities (the default CA list will be used if null), and optionally provide a common_name_override if you expect the certificate to have a common name other than the server FQDN.

Note: On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature.

### client_unsafe {#client_unsafe}

```gdscript
static TLSOptions client_unsafe(trusted_chain: X509Certificate = null)
```

Creates an unsafe TLS client configuration where certificate validation is optional. You can optionally provide a valid trusted_chain, but the common name of the certificates will never be checked. Using this configuration for purposes other than testing is not recommended.

Note: On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature.

### get_common_name_override {#get_common_name_override}

```gdscript
String get_common_name_override() const
```

Returns the common name (domain name) override specified when creating with client().

### get_own_certificate {#get_own_certificate}

```gdscript
X509Certificate get_own_certificate() const
```

Returns the X509Certificate specified when creating with server().

### get_private_key {#get_private_key}

```gdscript
CryptoKey get_private_key() const
```

Returns the CryptoKey specified when creating with server().

### get_trusted_ca_chain {#get_trusted_ca_chain}

```gdscript
X509Certificate get_trusted_ca_chain() const
```

Returns the CA X509Certificate chain specified when creating with client() or client_unsafe().

### is_server {#is_server}

```gdscript
bool is_server() const
```

Returns true if created with server(), false otherwise.

### is_unsafe_client {#is_unsafe_client}

```gdscript
bool is_unsafe_client() const
```

Returns true if created with client_unsafe(), false otherwise.

### server {#server}

```gdscript
static TLSOptions server(key: CryptoKey, certificate: X509Certificate)
```

Creates a TLS server configuration using the provided key and certificate.

Note: The certificate should include the full certificate chain up to the signing CA (certificates file can be concatenated using a general purpose text editor).
