# Specification 001 - HTTP Request

## Status

**Draft**

---

# Objective

Design the `Request` data model used throughout the HTTP server.

The `Request` struct represents a parsed HTTP request after it has been received from a client and before it is processed by the router.

This specification defines the structure and invariants of the request model only.

It does **not** define how HTTP messages are parsed. Parsing is covered by a separate specification.

---

# Motivation

Every component of the server depends on a common representation of an HTTP request.

Using a dedicated data model provides:

* Type safety
* Separation between parsing and routing
* Easier testing
* Cleaner APIs

---

# Responsibilities

The `Request` type shall:

* Represent an HTTP request.
* Store the request method.
* Store the request path.
* Store the HTTP version.
* Store request headers.
* Store the request body.

The `Request` type shall **not**:

* Parse raw HTTP messages.
* Read from network sockets.
* Validate routing.
* Generate HTTP responses.

---

# Expected Module

```text
src/http/request.rs
```

---

# Public API

The module shall expose:

```rust
pub struct Request;
```

The module may expose helper methods where appropriate.

---

# Required Fields

The request model shall contain the following fields.

| Field   | Type                    | Description             |
| ------- | ----------------------- | ----------------------- |
| method  | Method                  | HTTP request method     |
| path    | String                  | Requested resource path |
| version | String                  | HTTP version            |
| headers | HashMap<String, String> | Request headers         |
| body    | Vec<u8>                 | Request body            |

---

# Invariants

A valid request shall satisfy the following rules.

* A request always has exactly one HTTP method.
* A request always has one request path.
* A request always has one HTTP version.
* Header names are unique.
* The body may be empty.
* The path is stored exactly as received.
* The request object is immutable after construction unless explicitly modified by the application.

---

# Out of Scope

The following features are not part of this specification.

* Query parameter parsing
* Cookie parsing
* Multipart forms
* Chunked transfer encoding
* URL decoding
* Authentication

These will be specified separately if implemented.

---

# Example

HTTP request:

```http
GET /index.html HTTP/1.1
Host: localhost:8080
User-Agent: curl/8.0

```

Expected model:

```text
Request
├── method  : GET
├── path    : "/index.html"
├── version : "HTTP/1.1"
├── headers
│   ├── Host
│   └── User-Agent
└── body    : empty
```

---

# Error Handling

This module shall not perform validation.

Invalid requests are handled by the parser.

The `Request` type itself cannot fail once constructed.

---

# Dependencies

The module may depend on:

* std::collections::HashMap
* http::method

The module shall not depend on:

* TcpStream
* Router
* Filesystem
* Server

---

# Unit Tests

The following behaviors should be tested.

## Create Request

Verify that a request can be constructed correctly.

---

## Empty Body

Verify that requests without bodies are supported.

---

## Headers

Verify that headers are stored correctly.

---

## Path

Verify that arbitrary request paths can be stored.

Examples:

```
/

/hello

/api/users

/assets/image.png
```

---

## Methods

Verify that every supported HTTP method can be represented.

---

# Acceptance Criteria

This specification is complete when:

* A `Request` struct exists.
* The required fields are implemented.
* The module compiles.
* Unit tests pass.
* No networking code exists inside this module.

---

# Future Improvements

Possible future additions include:

* Query parameter support
* Cookies
* Parsed URI
* Client IP
* Extension storage
* Request metadata

These features are intentionally excluded from Version 1.
