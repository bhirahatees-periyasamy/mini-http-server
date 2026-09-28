# Spec 005 — HTTP Response Serialization

## 1. Objective

Implement HTTP response serialization.

The server currently has a `Response` type, but the response is constructed and written to the `TcpStream` manually.

The goal of this specification is to create a dedicated serializer that converts a `Response` into the exact sequence of bytes required by HTTP/1.x.

The serializer must not perform network I/O.

### Desired flow

```text
Response
   │
   ▼
ResponseSerializer
   │
   ▼
Vec<u8>
   │
   ▼
TcpStream::write_all()
```

The serializer is responsible only for:

```text
Response → HTTP bytes
```

---

# 2. Scope

### In scope

The implementation must support:

* HTTP/1.0 responses
* HTTP/1.1 responses
* status lines
* response headers
* response bodies
* `Content-Length`
* empty response bodies
* binary response bodies
* CRLF formatting
* deterministic serialization
* unit tests

### Out of scope

Do NOT implement these yet:

* chunked transfer encoding
* HTTP/2
* compression
* TLS
* keep-alive management
* streaming responses
* trailers
* connection management

Those will be separate specifications.

---

# 3. HTTP Response Format

A serialized HTTP response has this structure:

```text
STATUS-LINE\r\n
HEADER\r\n
HEADER\r\n
...
\r\n
BODY
```

For example:

```text
HTTP/1.1 200 OK\r\n
Content-Type: text/plain\r\n
Content-Length: 5\r\n
\r\n
Hello
```

The actual bytes must contain `\r\n`, not just `\n`.

---

# 4. Status Line

The first line must contain:

```text
HTTP-VERSION STATUS-CODE REASON-PHRASE
```

followed by:

```text
\r\n
```

Example:

```text
HTTP/1.1 200 OK\r\n
```

Another example:

```text
HTTP/1.1 404 Not Found\r\n
```

The serializer must obtain the HTTP version from the `Response`.

For example:

```rust
Response {
    version: HttpVersion::Http11,
    ...
}
```

must produce:

```text
HTTP/1.1
```

---

# 5. Headers

Every response header must be serialized as:

```text
Header-Name: Header-Value\r\n
```

Example:

```text
Content-Type: text/plain\r\n
```

There must be exactly one empty line after the final header:

```text
\r\n
```

Therefore:

```text
Content-Length: 5\r\n
\r\n
```

separates the headers from the body.

---

# 6. Content-Length

The serializer must ensure that the response contains a correct `Content-Length`.

The value must represent the number of bytes in the response body.

For example:

```rust
body = b"Hello"
```

has:

```text
Content-Length: 5
```

Do not use character count.

The following must also work correctly:

```rust
body = "こんにちは".as_bytes()
```

because HTTP bodies are byte sequences.

### Important

If the response already contains a `Content-Length` header, the serializer must not silently produce a conflicting value.

Choose one of the following designs:

```text
Option A:
Serializer overwrites Content-Length with body.len()

Option B:
Serializer returns an error when the existing value is incorrect

Option C:
Response construction guarantees Content-Length and serializer validates it
```

For this project, use:

```text
Option A
```

The serializer owns the final wire representation.

Therefore:

```text
Content-Length = body.len()
```

must always be true in the serialized response.

---

# 7. Response Body

The response body is arbitrary bytes.

Do not assume that the body is UTF-8 text.

For example, this must work:

```rust
vec![
    0x89, 0x50, 0x4E, 0x47
]
```

Therefore the serializer should work with:

```rust
Vec<u8>
```

or:

```rust
&[u8]
```

rather than:

```rust
String
```

The serializer must not convert the body into UTF-8.

---

# 8. Empty Response

The serializer must support responses without a body.

Example:

```text
HTTP/1.1 204 No Content\r\n
Content-Length: 0\r\n
\r\n
```

The serialized result must end immediately after the final `\r\n`.

There must be no additional body bytes.

---

# 9. Serializer API

Design an API similar to:

```rust
pub fn serialize(response: &Response) -> Result<Vec<u8>, SerializeError>
```

The exact API is up to you.

The important requirement is:

```text
Response → Result<Vec<u8>, SerializeError>
```

The serializer must not accept a `TcpStream`.

It must not write directly to the network.

---

# 10. Serialization Errors

Create an appropriate error type.

For example:

```rust
#[derive(Debug, PartialEq)]
pub enum SerializeError {
    InvalidHeader,
    InvalidHeaderName,
    InvalidHeaderValue,
}
```

You may add other errors if your design requires them.

Do not create errors that cannot actually occur.

Think about what could make a response invalid before writing it to the wire.

---

# 11. Header Validation

At minimum, prevent a header from injecting additional HTTP lines.

For example, this must not be accepted blindly:

```text
Header-Name: X\r\n
Injected: true
```

Header names and values must not contain illegal CR (`\r`) or LF (`\n`) characters.

The serializer should reject invalid header data.

For example:

```rust
Header: "hello\r\nworld"
```

should result in:

```rust
Err(SerializeError::InvalidHeaderValue)
```

rather than generating malformed HTTP.

---

# 12. Determinism

Given the same `Response`, the serializer should produce the same bytes.

For example:

```rust
let response = Response {
    version: HttpVersion::Http11,
    status: StatusCode::Ok,
    headers: ...,
    body: b"Hello".to_vec(),
};
```

should always produce the same serialized representation.

If you use a `HashMap` for headers, remember that header ordering is not guaranteed.

Therefore your tests should not depend on arbitrary `HashMap` iteration order.

You may choose to:

* use an ordered collection, or
* make tests check individual properties rather than the complete byte sequence.

Do not introduce unnecessary complexity solely for header ordering.

---

# 13. Required Test Cases

Implement unit tests for at least the following cases.

## Test 1 — 200 OK

Input:

```text
HTTP/1.1
200 OK
body = "Hello"
```

Expected:

```text
HTTP/1.1 200 OK\r\n
Content-Length: 5\r\n
\r\n
Hello
```

---

## Test 2 — 404 Not Found

Input:

```text
HTTP/1.1
404 Not Found
body = "Not Found"
```

Expected status line:

```text
HTTP/1.1 404 Not Found\r\n
```

and:

```text
Content-Length: 9
```

---

## Test 3 — Empty body

Input:

```text
HTTP/1.1
204 No Content
body = []
```

Expected:

```text
HTTP/1.1 204 No Content\r\n
Content-Length: 0\r\n
\r\n
```

---

## Test 4 — Custom header

Input:

```text
Content-Type: text/plain
```

Expected serialized header:

```text
Content-Type: text/plain\r\n
```

---

## Test 5 — Binary body

Use a body containing arbitrary bytes:

```rust
vec![0, 1, 2, 3, 255]
```

Expected:

```text
Content-Length: 5
```

and the exact bytes must appear after the header section.

Do not convert the body into a `String`.

---

## Test 6 — UTF-8 body

Use:

```text
こんにちは
```

The `Content-Length` must equal the number of bytes, not the number of Unicode characters.

---

## Test 7 — Invalid header value

A header containing:

```text
\r
```

or:

```text
\n
```

must be rejected.

---

## Test 8 — Invalid header name

A header name containing CR or LF must be rejected.

---

## Test 9 — HTTP/1.0

Create:

```text
HTTP/1.0 200 OK
```

and verify that the serializer uses:

```text
HTTP/1.0
```

instead of:

```text
HTTP/1.1
```

---

# 14. Integration With The Existing Server

After the serializer works independently, replace the manually constructed response in the server.

Instead of:

```rust
let response = format!(
    "HTTP/1.1 200 OK\r\n..."
);
stream.write_all(response.as_bytes())?;
```

the server should conceptually do:

```rust
let response = Response {
    ...
};

let bytes = serialize(&response)?;

stream.write_all(&bytes)?;
```

The important separation is:

```text
Response
   │
   ▼
serialize()
   │
   ▼
Vec<u8>
   │
   ▼
write_all()
```

The serializer must not know that TCP exists.

---

# 15. Design Constraints

Follow these constraints while implementing the specification.

### Constraint 1

Do not put TCP logic inside the serializer.

### Constraint 2

Do not make `Response` responsible for writing itself to the socket.

Avoid:

```rust
response.write_to(&mut stream)
```

for this specification.

### Constraint 3

Do not convert binary bodies into strings.

### Constraint 4

Do not use `unwrap()` in production serialization code.

### Constraint 5

Do not add routing yet.

### Constraint 6

Do not implement keep-alive yet.

### Constraint 7

Do not implement chunked encoding yet.

---

# 16. Suggested Module Structure

You can create:

```text
src/http/
├── method.rs
├── version.rs
├── status.rs
├── request.rs
├── response.rs
├── parser.rs
└── serializer.rs
```

The new module:

```text
serializer.rs
```

should contain the response serialization logic.

For example:

```rust
pub fn serialize(
    response: &Response,
) -> Result<Vec<u8>, SerializeError> {
    // your implementation
}
```

You are free to choose a different API if you can justify it.

---

# 17. Definition of Done

This specification is complete when:

* [ ] `Response` can be serialized into `Vec<u8>`
* [ ] HTTP/1.0 works
* [ ] HTTP/1.1 works
* [ ] status lines are correctly formatted
* [ ] headers are correctly formatted
* [ ] headers use CRLF
* [ ] header/body separator is correct
* [ ] `Content-Length` is correct
* [ ] empty bodies work
* [ ] binary bodies work
* [ ] UTF-8 bodies work
* [ ] invalid CR/LF in headers is rejected
* [ ] unit tests cover the required cases
* [ ] serializer performs no network I/O
* [ ] existing server uses the serializer
* [ ] manual response string construction has been removed

---

# 18. Success Criteria

Given this request:

```text
GET / HTTP/1.1\r\n
Host: localhost\r\n
\r\n
```

your current parser should produce:

```text
Request
```

Your application can then create:

```text
Response
```

The new serializer must produce:

```text
HTTP/1.1 200 OK\r\n
Content-Length: 13\r\n
\r\n
Hello, World!
```

The TCP layer then writes those bytes.

The complete pipeline should therefore become:

```text
TCP
 │
 │ bytes
 ▼
Parser
 │
 │ Request
 ▼
Application
 │
 │ Response
 ▼
Serializer
 │
 │ bytes
 ▼
TCP
```

This is the architectural boundary this specification is intended to establish.
