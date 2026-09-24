# Specification 004 - HTTP Request Parser

## Status

**Draft**

---

# Objective

Design the HTTP request parser responsible for converting raw HTTP request data into a structured `Request` object.

The parser operates on raw bytes received from a client connection and produces a validated `Request` model.

This specification defines parsing behavior only.

Reading from a TCP stream is handled by the Connection module.

---

# Motivation

HTTP requests arrive as raw bytes.

Before the server can perform routing or execute application logic, these bytes must be converted into a structured representation.

Separating parsing from networking provides:

* Better testability
* Cleaner architecture
* Reusable parser
* Separation of concerns

---

# Responsibilities

The parser shall:

* Parse the HTTP request line.
* Parse request headers.
* Parse the request body.
* Validate the HTTP request format.
* Construct a `Request` object.

The parser shall **not**:

* Read from a socket.
* Write responses.
* Route requests.
* Access the filesystem.

---

# Expected Module

```text
src/http/parser.rs
```

---

# Public API

The module shall expose a parser capable of transforming raw request bytes into a `Request`.

Example:

```rust
pub fn parse(input: &[u8]) -> Result<Request, ParseError>;
```

The exact API may evolve during implementation.

---

# Input

The parser receives raw HTTP request bytes.

Example:

```http
GET /hello HTTP/1.1
Host: localhost:8080
User-Agent: curl/8.0
Accept: */*

```

---

# Output

A successfully parsed request.

```text
Request
├── method
├── path
├── version
├── headers
└── body
```

---

# Parsing Requirements

The parser shall perform the following steps.

## Step 1

Read the request line.

Example:

```text
GET /hello HTTP/1.1
```

---

## Step 2

Extract:

* Method
* Path
* HTTP Version

---

## Step 3

Parse every HTTP header.

Example:

```text
Host: localhost

User-Agent: curl
```

Store headers inside the request.

---

## Step 4

Detect the blank line separating headers and body.

```text
Headers

<blank line>

Body
```

---

## Step 5

Store the remaining bytes as the request body.

The parser shall not interpret body contents.

---

# Validation Rules

The parser shall reject malformed requests.

Examples include:

* Missing request line
* Missing HTTP version
* Invalid method
* Malformed header
* Invalid header separator

---

# Out of Scope

The following features are intentionally excluded.

* Chunked Transfer Encoding
* Multipart Forms
* Cookies
* Query Parameter Parsing
* URL Decoding
* Compression
* Keep-Alive

---

# Error Handling

The parser shall return a dedicated error type.

Example errors include:

* InvalidRequestLine
* InvalidMethod
* InvalidVersion
* InvalidHeader
* UnexpectedEndOfInput

The parser must never panic due to malformed input.

---

# Dependencies

The parser may depend on:

* Request
* Method
* Version
* ParseError

The parser shall not depend on:

* TcpStream
* Router
* Filesystem
* Response

---

# Unit Tests

The following scenarios shall be tested.

## Valid GET Request

Verify a simple GET request is parsed successfully.

---

## Request With Headers

Verify multiple headers are parsed correctly.

---

## Request With Body

Verify request bodies are preserved.

---

## Empty Body

Verify requests without a body are supported.

---

## Invalid Method

Verify unsupported methods return an error.

---

## Invalid HTTP Version

Verify malformed versions are rejected.

---

## Missing Header Separator

Verify malformed headers produce an error.

---

## Binary Body

Verify arbitrary bytes are preserved without modification.

---

# Acceptance Criteria

This specification is complete when:

* A parser module exists.
* Valid requests are parsed correctly.
* Invalid requests return structured errors.
* All unit tests pass.
* No networking code exists inside the parser.

---

# Future Improvements

Future versions may include:

* Query parameter parsing
* Cookie parsing
* Chunked transfer decoding
* Multipart form parsing
* Streaming request bodies
* Header normalization

These features are intentionally excluded from Version 1.

---

# Review Checklist

## Design

* [ ] Parser has a single responsibility.
* [ ] Public API is minimal.
* [ ] Parser is independent of networking.

## Implementation

* [ ] Parses request line.
* [ ] Parses headers.
* [ ] Parses body.
* [ ] Returns `Result<Request, ParseError>`.

## Testing

* [ ] Valid request tests pass.
* [ ] Invalid request tests pass.
* [ ] Edge cases are covered.

## Documentation

* [ ] Rust documentation comments added.
* [ ] Module documentation updated.
* [ ] Specification marked as Implemented.
