# GDScriptTextDocument

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **GDScriptTextDocument**

> Provides language server functionality related to documents.

<details><summary>官方英文描述</summary>

Provides language server functionality related to documents.

</details>

## 方法

### codeLens {#codeLens}

```gdscript
Array codeLens(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### colorPresentation {#colorPresentation}

```gdscript
Array colorPresentation(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### completion {#completion}

```gdscript
Array completion(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### declaration {#declaration}

```gdscript
Variant declaration(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### definition {#definition}

```gdscript
Array definition(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### didChange {#didChange}

```gdscript
void didChange(params: Variant)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### didClose {#didClose}

```gdscript
void didClose(params: Variant)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### didOpen {#didOpen}

```gdscript
void didOpen(params: Variant)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### didSave {#didSave}

```gdscript
void didSave(params: Variant)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### documentLink {#documentLink}

```gdscript
Array documentLink(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### documentSymbol {#documentSymbol}

```gdscript
Array documentSymbol(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### foldingRange {#foldingRange}

```gdscript
Array foldingRange(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### hover {#hover}

```gdscript
Variant hover(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### nativeSymbol {#nativeSymbol}

```gdscript
Variant nativeSymbol(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### prepareRename {#prepareRename}

```gdscript
Variant prepareRename(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### references {#references}

```gdscript
Array references(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### rename {#rename}

```gdscript
Dictionary rename(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### resolve {#resolve}

```gdscript
Dictionary resolve(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### show_native_symbol_in_editor {#show_native_symbol_in_editor}

```gdscript
void show_native_symbol_in_editor(symbol_id: String)
```

Deprecated: Use ScriptEditor.goto_help() instead.

### signatureHelp {#signatureHelp}

```gdscript
Variant signatureHelp(params: Dictionary)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.

### willSaveWaitUntil {#willSaveWaitUntil}

```gdscript
void willSaveWaitUntil(params: Variant)
```

Deprecated: Accessing LSP endpoints directly might lead to unwanted side effects. Connect to the server via TCP, like a regular language server client.
