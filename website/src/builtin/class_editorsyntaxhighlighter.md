# EditorSyntaxHighlighter

**继承**：[SyntaxHighlighter](class_syntaxhighlighter.md) → [Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **EditorSyntaxHighlighter**

> Base class that all SyntaxHighlighters used by the ScriptEditor extend from.

<details><summary>官方英文描述（中文精译进行中）</summary>

Base class that all SyntaxHighlighters used by the ScriptEditor extend from.

Add a syntax highlighter to an individual script by calling ScriptEditorBase.add_syntax_highlighter(). To apply to all scripts on open, call ScriptEditor.register_syntax_highlighter().

</details>

## 方法

### _create {#_create}

```gdscript
EditorSyntaxHighlighter _create() virtual const
```

Virtual method which creates a new instance of the syntax highlighter.

### _get_name {#_get_name}

```gdscript
String _get_name() virtual const
```

Virtual method which can be overridden to return the syntax highlighter name.

### _get_supported_languages {#_get_supported_languages}

```gdscript
PackedStringArray _get_supported_languages() virtual const
```

Virtual method which can be overridden to return the supported language names.
