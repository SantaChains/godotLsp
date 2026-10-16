# Translation

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **Translation**

> Translation maps a collection of strings to their individual translations, and also provides convenience methods for pluralization.

<details><summary>官方英文描述（中文精译进行中）</summary>

Translation maps a collection of strings to their individual translations, and also provides convenience methods for pluralization.

A Translation consists of messages. A message is identified by its context and untranslated string. Unlike gettext [https://www.gnu.org/software/gettext/], using an empty context string in Godot means not using any context.

</details>

## 方法

### _get_message {#_get_message}

```gdscript
StringName _get_message(src_message: StringName, context: StringName) virtual const
```

Virtual method to override get_message().

### _get_plural_message {#_get_plural_message}

```gdscript
StringName _get_plural_message(src_message: StringName, src_plural_message: StringName, n: int, context: StringName) virtual const
```

Virtual method to override get_plural_message().

### add_message {#add_message}

```gdscript
void add_message(src_message: StringName, xlated_message: StringName, context: StringName = &"")
```

Adds a message if nonexistent, followed by its translation.

An additional context could be used to specify the translation context or differentiate polysemic words.

### add_plural_message {#add_plural_message}

```gdscript
void add_plural_message(src_message: StringName, xlated_messages: PackedStringArray, context: StringName = &"")
```

Adds a message involving plural translation if nonexistent, followed by its translation.

An additional context could be used to specify the translation context or differentiate polysemic words.

### erase_message {#erase_message}

```gdscript
void erase_message(src_message: StringName, context: StringName = &"")
```

Erases a message.

### get_locale {#get_locale}

```gdscript
String get_locale() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_message {#get_message}

```gdscript
StringName get_message(src_message: StringName, context: StringName = &"") const
```

Returns a message's translation.

### get_message_count {#get_message_count}

```gdscript
int get_message_count() const
```

Returns the number of existing messages.

### get_message_list {#get_message_list}

```gdscript
PackedStringArray get_message_list() const
```

Returns the keys of all messages, that is, the context and untranslated strings of each message.

Note: If a message does not use a context, the corresponding element is the untranslated string. Otherwise, the corresponding element is the context and untranslated string separated by the EOT character (U+0004). This is done for compatibility purposes.

```gdscript
for key in translation.get_message_list():
    var p = key.find("\u0004")
    if p == -1:
        var untranslated = key
        print("Message %s" % untranslated)
    else:
        var context = key.substr(0, p)
        var untranslated = key.substr(p + 1)
        print("Message %s with context %s" % [untranslated, context])
```

### get_plural_message {#get_plural_message}

```gdscript
StringName get_plural_message(src_message: StringName, src_plural_message: StringName, n: int, context: StringName = &"") const
```

Returns a message's translation involving plurals.

The number n is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.

Note: Plurals are only supported in gettext-based translations (PO), not CSV.

### get_plural_rules_override {#get_plural_rules_override}

```gdscript
String get_plural_rules_override() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_translated_message_list {#get_translated_message_list}

```gdscript
PackedStringArray get_translated_message_list() const
```

Returns all the translated strings.

### set_locale {#set_locale}

```gdscript
void set_locale(locale: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_plural_rules_override {#set_plural_rules_override}

```gdscript
void set_plural_rules_override(rules: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### messages {#messages}

```gdscript
messages: Dictionary
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### locale {#locale}

```gdscript
locale: String
```

- void set_locale(value: String)
- String get_locale()

The locale of the translation.

### plural_rules_override {#plural_rules_override}

```gdscript
plural_rules_override: String
```

- void set_plural_rules_override(value: String)
- String get_plural_rules_override()

The plural rules string to enforce. See GNU gettext [https://www.gnu.org/software/gettext/manual/html_node/Plural-forms.html] for examples and more info.

If empty or invalid, default plural rules from TranslationServer.get_plural_rules() are used. The English plural rules are used as a fallback.
