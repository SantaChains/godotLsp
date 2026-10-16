# GDScriptLanguageProtocol

**继承**：[JSONRPC](class_jsonrpc.md) → [Object](class_object.md) → **GDScriptLanguageProtocol**

> Provides access to certain features that are implemented in the language server.

<details><summary>官方英文描述（中文精译进行中）</summary>

Provides access to certain features that are implemented in the language server.

Note: This class is not a language server client that can be used to access LSP functionality. It only provides access to a limited set of features that is implemented using the same technical foundation as the language server.

</details>

## 方法

### get_text_document {#get_text_document}

```gdscript
GDScriptTextDocument get_text_document()
```

Deprecated: GDScriptTextDocument is deprecated.

Returns the language server's GDScriptTextDocument instance.

### get_workspace {#get_workspace}

```gdscript
GDScriptWorkspace get_workspace()
```

Returns the language server's GDScriptWorkspace instance.

### initialize {#initialize}

```gdscript
Variant initialize(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### initialized {#initialized}

```gdscript
void initialized(params: Variant)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### is_initialized {#is_initialized}

```gdscript
bool is_initialized() const
```

Returns true if the language server was initialized by a language server client, false otherwise.

### is_smart_resolve_enabled {#is_smart_resolve_enabled}

```gdscript
bool is_smart_resolve_enabled() const
```

Returns true if the language server is providing the smart resolve feature, false otherwise. The feature can be configured through the editor settings.

### notify_client {#notify_client}

```gdscript
void notify_client(method: String, params: Variant = null, client_id: int = -1)
```

Deprecated: Might result in unwanted side effects for connected clients.

### on_client_connected {#on_client_connected}

```gdscript
Error on_client_connected()
```

Deprecated: Might result in unwanted side effects for connected clients.

### on_client_disconnected {#on_client_disconnected}

```gdscript
void on_client_disconnected(client_id: int)
```

Deprecated: Might result in unwanted side effects for connected clients.
