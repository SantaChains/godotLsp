# JSON

**继承**：[Resource](class_resource.md) → [RefCounted](class_refcounted.md) → [Object](class_object.md) → **JSON**

> The JSON class enables all data types to be converted to and from a JSON string. This is useful for serializing data, e.g. to save to a file or send over the network.

<details><summary>官方英文描述</summary>

The JSON class enables all data types to be converted to and from a JSON string. This is useful for serializing data, e.g. to save to a file or send over the network.

stringify() is used to convert any data type into a JSON string.

parse() is used to convert any existing JSON data into a Variant that can be used within Godot. If successfully parsed, use data to retrieve the Variant, and use @GlobalScope.typeof() to check if the Variant's type is what you expect. JSON Objects are converted into a Dictionary, but JSON data can be used to store Arrays, numbers, Strings and even just a boolean.

```gdscript
var data_to_send = ["a", "b", "c"]
var json_string = JSON.stringify(data_to_send)
# Save data
# ...
# Retrieve data
var json = JSON.new()
var error = json.parse(json_string)
if error == OK:
    var data_received = json.data
    if typeof(data_received) == TYPE_ARRAY:
        print(data_received) # Prints the array.
    else:
        print("Unexpected data")
else:
    print("JSON Parse Error: ", json.get_error_message(), " in ", json_string, " at line ", json.get_error_line())
```

Alternatively, you can parse strings using the static parse_string() method, but it doesn't handle errors.

```gdscript
var data = JSON.parse_string(json_string) # Returns null if parsing failed.
```

Note: Both parse methods do not fully comply with the JSON specification:

- Trailing commas in arrays or objects are ignored, instead of causing a parser error.
- New line and tab characters are accepted in string literals, and are treated like their corresponding escape sequences \n and \t.
- Numbers are parsed using String.to_float() which is generally more lax than the JSON specification.
- Certain errors, such as invalid Unicode sequences, do not cause a parser error. Instead, the string is cleaned up and an error is logged to the console.

</details>

## 方法

### from_native {#from_native}

```gdscript
static Variant from_native(variant: Variant, full_objects: bool = false)
```

Converts a native engine type to a JSON-compliant value.

By default, objects are ignored for security reasons, unless full_objects is true.

You can convert a native value to a JSON string like this:

```gdscript
func encode_data(value, full_objects = false):
    return JSON.stringify(JSON.from_native(value, full_objects))
```

### get_data {#get_data}

```gdscript
Variant get_data() const
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### get_error_line {#get_error_line}

```gdscript
int get_error_line() const
```

Returns 0 if the last call to parse() was successful, or the line number where the parse failed.

### get_error_message {#get_error_message}

```gdscript
String get_error_message() const
```

Returns an empty string if the last call to parse() was successful, or the error message if it failed.

### get_parsed_text {#get_parsed_text}

```gdscript
String get_parsed_text() const
```

Return the text parsed by parse() (requires passing keep_text to parse()).

### parse {#parse}

```gdscript
Error parse(json_text: String, keep_text: bool = false)
```

Attempts to parse the json_text provided.

Returns an Error. If the parse was successful, it returns @GlobalScope.OK and the result can be retrieved using data. If unsuccessful, use get_error_line() and get_error_message() to identify the source of the failure.

Non-static variant of parse_string(), if you want custom error handling.

The optional keep_text argument instructs the parser to keep a copy of the original text. This text can be obtained later by using the get_parsed_text() function and is used when saving the resource (instead of generating new text from data).

### parse_string {#parse_string}

```gdscript
static Variant parse_string(json_string: String)
```

Attempts to parse the json_string provided and returns the parsed data. Returns null if parse failed.

### set_data {#set_data}

```gdscript
void set_data(data: Variant)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

### stringify {#stringify}

```gdscript
static String stringify(data: Variant, indent: String = "", sort_keys: bool = true, full_precision: bool = false)
```

Converts a Variant var to JSON text and returns the result. Useful for serializing data to store or send over the network.

Note: The JSON specification does not define integer or float types, but only a number type. Therefore, converting a Variant to JSON text will convert all numerical values to float types.

Note: If full_precision is true, when stringifying floats, the unreliable digits are stringified in addition to the reliable digits to guarantee exact decoding.

The indent parameter controls if and how something is indented; its contents will be used where there should be an indent in the output. Even spaces like "   " will work. \t and \n can also be used for a tab indent, or to make a newline for each indent respectively.

Warning: Non-finite numbers are not supported in JSON. Any occurrences of @GDScript.INF will be replaced with 1e99999, and negative @GDScript.INF will be replaced with -1e99999, but they will be interpreted correctly as infinity by most JSON parsers. @GDScript.NAN will be replaced with null, and it will not be interpreted as NaN in JSON parsers. If you expect non-finite numbers, consider passing your data through from_native() first.

Example output:

```gdscript

### to_native {#to_native}

```gdscript
static Variant to_native(json: Variant, allow_objects: bool = false)
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

## 属性

### data {#data}

```gdscript
data: Variant
```

- void set_data(value: Variant)
- Variant get_data()

Contains the parsed JSON data in Variant form.
