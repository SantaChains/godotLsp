# TranslationDomain

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **TranslationDomain**

> TranslationDomain is a self-contained collection of Translation resources. Translations can be added to or removed from it.

<details><summary>官方英文描述（中文精译进行中）</summary>

TranslationDomain is a self-contained collection of Translation resources. Translations can be added to or removed from it.

If you're working with the main translation domain, it is more convenient to use the wrap methods on TranslationServer.

</details>

## 方法

### add_translation {#add_translation}

```gdscript
void add_translation(translation: Translation)
```

Adds a translation.

### clear {#clear}

```gdscript
void clear()
```

Removes all translations.

### find_translations {#find_translations}

```gdscript
typedarray::Translation find_translations(locale: String, exact: bool) const
```

Returns the Translation instances that match locale (see TranslationServer.compare_locales()). If exact is true, only instances whose locale exactly equals locale will be returned.

### get_locale_override {#get_locale_override}

```gdscript
String get_locale_override() const
```

Returns the locale override of the domain. Returns an empty string if locale override is disabled.

### get_pseudolocalization_expansion_ratio {#get_pseudolocalization_expansion_ratio}

```gdscript
float get_pseudolocalization_expansion_ratio() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pseudolocalization_prefix {#get_pseudolocalization_prefix}

```gdscript
String get_pseudolocalization_prefix() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_pseudolocalization_suffix {#get_pseudolocalization_suffix}

```gdscript
String get_pseudolocalization_suffix() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_translation_object {#get_translation_object}

```gdscript
Translation get_translation_object(locale: String) const
```

Deprecated: Use find_translations() instead.

Returns the Translation instance that best matches locale. Returns null if there are no matches.

### get_translations {#get_translations}

```gdscript
typedarray::Translation get_translations() const
```

Returns all available Translation instances as added by add_translation().

### has_translation {#has_translation}

```gdscript
bool has_translation(translation: Translation) const
```

Returns true if this translation domain contains the given translation.

### has_translation_for_locale {#has_translation_for_locale}

```gdscript
bool has_translation_for_locale(locale: String, exact: bool) const
```

Returns true if there are any Translation instances that match locale (see TranslationServer.compare_locales()). If exact is true, only instances whose locale exactly equals locale are considered.

### is_enabled {#is_enabled}

```gdscript
bool is_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pseudolocalization_accents_enabled {#is_pseudolocalization_accents_enabled}

```gdscript
bool is_pseudolocalization_accents_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pseudolocalization_double_vowels_enabled {#is_pseudolocalization_double_vowels_enabled}

```gdscript
bool is_pseudolocalization_double_vowels_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pseudolocalization_enabled {#is_pseudolocalization_enabled}

```gdscript
bool is_pseudolocalization_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pseudolocalization_fake_bidi_enabled {#is_pseudolocalization_fake_bidi_enabled}

```gdscript
bool is_pseudolocalization_fake_bidi_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pseudolocalization_override_enabled {#is_pseudolocalization_override_enabled}

```gdscript
bool is_pseudolocalization_override_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### is_pseudolocalization_skip_placeholders_enabled {#is_pseudolocalization_skip_placeholders_enabled}

```gdscript
bool is_pseudolocalization_skip_placeholders_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### pseudolocalize {#pseudolocalize}

```gdscript
StringName pseudolocalize(message: StringName) const
```

Returns the pseudolocalized string based on the message passed in.

### remove_translation {#remove_translation}

```gdscript
void remove_translation(translation: Translation)
```

Removes the given translation.

### set_enabled {#set_enabled}

```gdscript
void set_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_locale_override {#set_locale_override}

```gdscript
void set_locale_override(locale: String)
```

Sets the locale override of the domain.

If locale is an empty string, locale override is disabled. Otherwise, locale will be standardized to match known locales (e.g. en-US would be matched to en_US).

Note: Calling this method does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED signal manually.

### set_pseudolocalization_accents_enabled {#set_pseudolocalization_accents_enabled}

```gdscript
void set_pseudolocalization_accents_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_double_vowels_enabled {#set_pseudolocalization_double_vowels_enabled}

```gdscript
void set_pseudolocalization_double_vowels_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_enabled {#set_pseudolocalization_enabled}

```gdscript
void set_pseudolocalization_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_expansion_ratio {#set_pseudolocalization_expansion_ratio}

```gdscript
void set_pseudolocalization_expansion_ratio(ratio: float)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_fake_bidi_enabled {#set_pseudolocalization_fake_bidi_enabled}

```gdscript
void set_pseudolocalization_fake_bidi_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_override_enabled {#set_pseudolocalization_override_enabled}

```gdscript
void set_pseudolocalization_override_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_prefix {#set_pseudolocalization_prefix}

```gdscript
void set_pseudolocalization_prefix(prefix: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_skip_placeholders_enabled {#set_pseudolocalization_skip_placeholders_enabled}

```gdscript
void set_pseudolocalization_skip_placeholders_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### set_pseudolocalization_suffix {#set_pseudolocalization_suffix}

```gdscript
void set_pseudolocalization_suffix(suffix: String)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### translate {#translate}

```gdscript
StringName translate(message: StringName, context: StringName = &"") const
```

Returns the current locale's translation for the given message and context.

### translate_plural {#translate_plural}

```gdscript
StringName translate_plural(message: StringName, message_plural: StringName, n: int, context: StringName = &"") const
```

Returns the current locale's translation for the given message, plural message and context.

The number n is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.

## 属性

### enabled {#enabled}

```gdscript
enabled: bool
```

- void set_enabled(value: bool)
- bool is_enabled()

If true, translation is enabled. Otherwise, translate() and translate_plural() will return the input message unchanged regardless of the current locale.

### pseudolocalization_enabled {#pseudolocalization_enabled}

```gdscript
pseudolocalization_enabled: bool
```

- void set_pseudolocalization_enabled(value: bool)
- bool is_pseudolocalization_enabled()

If true, enables pseudolocalization for the project. This can be used to spot untranslatable strings or layout issues that may occur once the project is localized to languages that have longer strings than the source language.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_accents_enabled {#pseudolocalization_accents_enabled}

```gdscript
pseudolocalization_accents_enabled: bool
```

- void set_pseudolocalization_accents_enabled(value: bool)
- bool is_pseudolocalization_accents_enabled()

Replace all characters with their accented variants during pseudolocalization.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_double_vowels_enabled {#pseudolocalization_double_vowels_enabled}

```gdscript
pseudolocalization_double_vowels_enabled: bool
```

- void set_pseudolocalization_double_vowels_enabled(value: bool)
- bool is_pseudolocalization_double_vowels_enabled()

Double vowels in strings during pseudolocalization to simulate the lengthening of text due to localization.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_fake_bidi_enabled {#pseudolocalization_fake_bidi_enabled}

```gdscript
pseudolocalization_fake_bidi_enabled: bool
```

- void set_pseudolocalization_fake_bidi_enabled(value: bool)
- bool is_pseudolocalization_fake_bidi_enabled()

If true, emulate bidirectional (right-to-left) text when pseudolocalization is enabled. This can be used to spot issues with RTL layout and UI mirroring that will crop up if the project is localized to RTL languages such as Arabic or Hebrew.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_override_enabled {#pseudolocalization_override_enabled}

```gdscript
pseudolocalization_override_enabled: bool
```

- void set_pseudolocalization_override_enabled(value: bool)
- bool is_pseudolocalization_override_enabled()

Replace all characters in the string with *. Useful for finding non-localizable strings.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_skip_placeholders_enabled {#pseudolocalization_skip_placeholders_enabled}

```gdscript
pseudolocalization_skip_placeholders_enabled: bool
```

- void set_pseudolocalization_skip_placeholders_enabled(value: bool)
- bool is_pseudolocalization_skip_placeholders_enabled()

Skip placeholders for string formatting like %s or %f during pseudolocalization. Useful to identify strings which need additional control characters to display correctly.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_expansion_ratio {#pseudolocalization_expansion_ratio}

```gdscript
pseudolocalization_expansion_ratio: float
```

- void set_pseudolocalization_expansion_ratio(value: float)
- float get_pseudolocalization_expansion_ratio()

The expansion ratio to use during pseudolocalization. A value of 0.3 is sufficient for most practical purposes, and will increase the length of each string by 30%.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_prefix {#pseudolocalization_prefix}

```gdscript
pseudolocalization_prefix: String
```

- void set_pseudolocalization_prefix(value: String)
- String get_pseudolocalization_prefix()

Prefix that will be prepended to the pseudolocalized string.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.

### pseudolocalization_suffix {#pseudolocalization_suffix}

```gdscript
pseudolocalization_suffix: String
```

- void set_pseudolocalization_suffix(value: String)
- String get_pseudolocalization_suffix()

Suffix that will be appended to the pseudolocalized string.

Note: Updating this property does not automatically update texts in the scene tree. Please propagate the MainLoop.NOTIFICATION_TRANSLATION_CHANGED notification manually after you have finished modifying pseudolocalization related options.
