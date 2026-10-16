# TranslationServer

**继承**：[Object](class_object.md) → **TranslationServer**

> The translation server is the API backend that manages all language translations.

<details><summary>官方英文描述</summary>

The translation server is the API backend that manages all language translations.

Translations are stored in TranslationDomains, which can be accessed by name. The most commonly used translation domain is the main translation domain. It always exists and can be accessed using an empty StringName. The translation server provides wrapper methods for accessing the main translation domain directly, without having to fetch the translation domain first. Custom translation domains are mainly for advanced usages like editor plugins. Names starting with godot. are reserved for engine internals.

</details>

## 方法

### add_translation {#add_translation}

```gdscript
void add_translation(translation: Translation)
```

Adds a translation to the main translation domain.

### clear {#clear}

```gdscript
void clear()
```

Removes all translations from the main translation domain.

### compare_locales {#compare_locales}

```gdscript
int compare_locales(locale_a: String, locale_b: String) const
```

Compares two locales and returns a similarity score between 0 (no match) and 10 (full match).

### find_translations {#find_translations}

```gdscript
typedarray::Translation find_translations(locale: String, exact: bool) const
```

Returns the Translation instances in the main translation domain that match locale (see compare_locales()). If exact is true, only instances whose locale exactly equals locale will be returned.

### format_number {#format_number}

```gdscript
String format_number(number: String, locale: String) const
```

Converts a number from Western Arabic (0..9) to the numeral system used in the given locale.

### get_all_countries {#get_all_countries}

```gdscript
PackedStringArray get_all_countries() const
```

Returns an array of known country codes.

### get_all_languages {#get_all_languages}

```gdscript
PackedStringArray get_all_languages() const
```

Returns array of known language codes.

### get_all_scripts {#get_all_scripts}

```gdscript
PackedStringArray get_all_scripts() const
```

Returns an array of known script codes.

### get_country_name {#get_country_name}

```gdscript
String get_country_name(country: String) const
```

Returns a readable country name for the country code.

### get_language_name {#get_language_name}

```gdscript
String get_language_name(language: String) const
```

Returns a readable language name for the language code.

### get_loaded_locales {#get_loaded_locales}

```gdscript
PackedStringArray get_loaded_locales() const
```

Returns an array of all loaded locales of the project.

### get_locale {#get_locale}

```gdscript
String get_locale() const
```

Returns the current locale of the project.

See also OS.get_locale() and OS.get_locale_language() to query the locale of the user system.

### get_locale_name {#get_locale_name}

```gdscript
String get_locale_name(locale: String) const
```

Returns a locale's language and its variant (e.g. "en_US" would return "English (United States)").

### get_or_add_domain {#get_or_add_domain}

```gdscript
TranslationDomain get_or_add_domain(domain: StringName)
```

Returns the translation domain with the specified name. An empty translation domain will be created and added if it does not exist.

### get_percent_sign {#get_percent_sign}

```gdscript
String get_percent_sign(locale: String) const
```

Returns the percent sign used in the given locale.

### get_plural_rules {#get_plural_rules}

```gdscript
String get_plural_rules(locale: String) const
```

Returns the default plural rules for the locale.

### get_script_name {#get_script_name}

```gdscript
String get_script_name(script: String) const
```

Returns a readable script name for the script code.

### get_tool_locale {#get_tool_locale}

```gdscript
String get_tool_locale()
```

Returns the current locale of the editor.

Note: When called from an exported project returns the same value as get_locale().

### get_translation_object {#get_translation_object}

```gdscript
Translation get_translation_object(locale: String)
```

Deprecated: Use find_translations() instead.

Returns the Translation instance that best matches locale in the main translation domain. Returns null if there are no matches.

### get_translations {#get_translations}

```gdscript
typedarray::Translation get_translations() const
```

Returns all available Translation instances in the main translation domain as added by add_translation().

### has_domain {#has_domain}

```gdscript
bool has_domain(domain: StringName) const
```

Returns true if a translation domain with the specified name exists.

### has_translation {#has_translation}

```gdscript
bool has_translation(translation: Translation) const
```

Returns true if the main translation domain contains the given translation.

### has_translation_for_locale {#has_translation_for_locale}

```gdscript
bool has_translation_for_locale(locale: String, exact: bool) const
```

Returns true if there are any Translation instances in the main translation domain that match locale (see compare_locales()). If exact is true, only instances whose locale exactly equals locale are considered.

### is_pseudolocalization_enabled {#is_pseudolocalization_enabled}

```gdscript
bool is_pseudolocalization_enabled() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### parse_number {#parse_number}

```gdscript
String parse_number(number: String, locale: String) const
```

Converts number from the numeral system used in the given locale to Western Arabic (0..9).

### pseudolocalize {#pseudolocalize}

```gdscript
StringName pseudolocalize(message: StringName) const
```

Returns the pseudolocalized string based on the message passed in.

Note: This method always uses the main translation domain.

### reload_pseudolocalization {#reload_pseudolocalization}

```gdscript
void reload_pseudolocalization()
```

Reparses the pseudolocalization options and reloads the translation for the main translation domain.

### remove_domain {#remove_domain}

```gdscript
void remove_domain(domain: StringName)
```

Removes the translation domain with the specified name.

Note: Trying to remove the main translation domain is an error.

### remove_translation {#remove_translation}

```gdscript
void remove_translation(translation: Translation)
```

Removes the given translation from the main translation domain.

### set_locale {#set_locale}

```gdscript
void set_locale(locale: String)
```

Sets the locale of the project. The locale string will be standardized to match known locales (e.g. en-US would be matched to en_US).

If translations have been loaded beforehand for the new locale, they will be applied.

### set_pseudolocalization_enabled {#set_pseudolocalization_enabled}

```gdscript
void set_pseudolocalization_enabled(enabled: bool)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### standardize_locale {#standardize_locale}

```gdscript
String standardize_locale(locale: String, add_defaults: bool = false) const
```

Returns a locale string standardized to match known locales (e.g. en-US would be matched to en_US). If add_defaults is true, the locale may have a default script or country added.

### translate {#translate}

```gdscript
StringName translate(message: StringName, context: StringName = &"") const
```

Returns the current locale's translation for the given message and context.

Note: This method always uses the main translation domain.

### translate_plural {#translate_plural}

```gdscript
StringName translate_plural(message: StringName, plural_message: StringName, n: int, context: StringName = &"") const
```

Returns the current locale's translation for the given message, plural message and context.

The number n is the number or quantity of the plural object. It will be used to guide the translation system to fetch the correct plural form for the selected language.

Note: This method always uses the main translation domain.

## 属性

### pseudolocalization_enabled {#pseudolocalization_enabled}

```gdscript
pseudolocalization_enabled: bool
```

- void set_pseudolocalization_enabled(value: bool)
- bool is_pseudolocalization_enabled()

If true, enables the use of pseudolocalization on the main translation domain. See ProjectSettings.internationalization/pseudolocalization/use_pseudolocalization for details.
