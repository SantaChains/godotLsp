# JSONRPC

**继承**：[Object](class_object.md) → **JSONRPC**

> JSON-RPC [https://www.jsonrpc.org/] is a standard which wraps a method call in a JSON object. The object has a particular structure and identifies which method is called, the parameters to that function, and carries an ID to keep track of responses. This class implements that standard on top of Dictionary; you will have to convert between a Dictionary and JSON with other functions.

<details><summary>官方英文描述（中文精译进行中）</summary>

JSON-RPC [https://www.jsonrpc.org/] is a standard which wraps a method call in a JSON object. The object has a particular structure and identifies which method is called, the parameters to that function, and carries an ID to keep track of responses. This class implements that standard on top of Dictionary; you will have to convert between a Dictionary and JSON with other functions.

</details>

## 方法

### make_notification {#make_notification}

```gdscript
Dictionary make_notification(method: String, params: Variant)
```

Returns a dictionary in the form of a JSON-RPC notification. Notifications are one-shot messages which do not expect a response.

- method: Name of the method being called.
- params: An array or dictionary of parameters being passed to the method.

### make_request {#make_request}

```gdscript
Dictionary make_request(method: String, params: Variant, id: Variant)
```

Returns a dictionary in the form of a JSON-RPC request. Requests are sent to a server with the expectation of a response. The ID field is used for the server to specify which exact request it is responding to.

- method: Name of the method being called.
- params: An array or dictionary of parameters being passed to the method.
- id: Uniquely identifies this request. The server is expected to send a response with the same ID.

### make_response {#make_response}

```gdscript
Dictionary make_response(result: Variant, id: Variant)
```

When a server has received and processed a request, it is expected to send a response. If you did not want a response then you need to have sent a Notification instead.

- result: The return value of the function which was called.
- id: The ID of the request this response is targeted to.

### make_response_error {#make_response_error}

```gdscript
Dictionary make_response_error(code: int, message: String, id: Variant = null) const
```

Creates a response which indicates a previous reply has failed in some way.

- code: The error code corresponding to what kind of error this is. See the ErrorCode constants.
- message: A custom message about this error.
- id: The request this error is a response to.

### process_action {#process_action}

```gdscript
Variant process_action(action: Variant, recurse: bool = false)
```

Given a Dictionary which takes the form of a JSON-RPC request: unpack the request and run it. Methods are resolved by looking at the field called "method" and looking for an equivalently named function in the JSONRPC object. If one is found that method is called.

To add new supported methods extend the JSONRPC class and call process_action() on your subclass.

action: The action to be run, as a Dictionary in the form of a JSON-RPC request or notification.

### process_string {#process_string}

```gdscript
String process_string(action: String)
```

There is currently no description for this method. Please help us by contributing one [https://contributing.godotengine.org/en/latest/documentation/class_reference.html]!

### set_method {#set_method}

```gdscript
void set_method(name: String, callback: Callable)
```

Registers a callback for the given method name.

- name: The name that clients can use to access the callback.
- callback: The callback which will handle the specified method.

## 枚举

### ErrorCode {#enum-ErrorCode}

```gdscript
enum ErrorCode { PARSE_ERROR = -32700, INVALID_REQUEST = -32600, METHOD_NOT_FOUND = -32601, INVALID_PARAMS = -32602, INTERNAL_ERROR = -32603 }
```

> 🚧 描述待补（P0 精译）。参见官方 [godot-docs](https://docs.godotengine.org/en/stable/)。

#### PARSE_ERROR {#PARSE_ERROR}

```gdscript
PARSE_ERROR = -32700
```

The request could not be parsed as it was not valid by JSON standard (JSON.parse() failed).

#### INVALID_REQUEST {#INVALID_REQUEST}

```gdscript
INVALID_REQUEST = -32600
```

A method call was requested but the request's format is not valid.

#### METHOD_NOT_FOUND {#METHOD_NOT_FOUND}

```gdscript
METHOD_NOT_FOUND = -32601
```

A method call was requested but no function of that name existed in the JSONRPC subclass.

#### INVALID_PARAMS {#INVALID_PARAMS}

```gdscript
INVALID_PARAMS = -32602
```

A method call was requested but the given method parameters are not valid. Not used by the built-in JSONRPC.

#### INTERNAL_ERROR {#INTERNAL_ERROR}

```gdscript
INTERNAL_ERROR = -32603
```

An internal error occurred while processing the request. Not used by the built-in JSONRPC.

---
