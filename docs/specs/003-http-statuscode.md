# Specification 003 - HTTP Status Code

## Status

**Draft**

---

# Objective

Design the `StatusCode` type used throughout the HTTP server.

The `StatusCode` type represents the outcome of an HTTP request and is used by the `Response` model to communicate the result of request processing.

This specification defines only the status code model.

It does **not** define response serialization or application logic.

---

# Motivation

Every HTTP response contains a status code.

Representing status codes as a dedicated type provides:

* Type safety
* Readable application code
* Easier testing
* Prevention of invalid status values

---

# Responsibilities

The `StatusCode` type shall:

* Represent supported HTTP status codes.
* Provide a numeric status code.
* Provide the standard reason phrase.
* Be reusable across the project.

The `StatusCode` type shall **not**:

* Serialize HTTP responses.
* Generate headers.
* Perform validation.
* Manage networking.

---

# Expected Module

```text
src/http/status.rs
```

---

# Public API

The module shall expose:

```rust
pub enum StatusCode;
```

Helper methods may be provided to retrieve metadata associated with a status code.

---

# Supported Status Codes

Version 1 of the server shall support the following status codes.

| Code | Variant             | Reason Phrase         |
| ---- | ------------------- | --------------------- |
| 200  | Ok                  | OK                    |
| 201  | Created             | Created               |
| 204  | NoContent           | No Content            |
| 400  | BadRequest          | Bad Request           |
| 404  | NotFound            | Not Found             |
| 405  | MethodNotAllowed    | Method Not Allowed    |
| 500  | InternalServerError | Internal Server Error |

Additional status codes may be added in future versions.

---

# Invariants

A valid status code shall satisfy the following rules.

* Every variant has exactly one numeric code.
* Every variant has exactly one reason phrase.
* Numeric values never change.
* Reason phrases follow the HTTP specification.

---

# Out of Scope

The following are intentionally excluded.

* Custom status codes
* HTTP extensions
* Localization of reason phrases
* Status code serialization

---

# Example

Application response:

```text
Response
└── status = StatusCode::Ok
```

Expected metadata:

```text
Numeric Code : 200

Reason Phrase : "OK"
```

---

# Error Handling

The `StatusCode` type itself should never fail.

All variants are valid by definition.

---

# Dependencies

This module shall not depend on any project modules.

It may depend only on the Rust standard library.

---

# Unit Tests

The following behaviors should be tested.

## Numeric Code

Verify that every status code returns the expected numeric value.

Examples:

* Ok → 200
* NotFound → 404
* InternalServerError → 500

---

## Reason Phrase

Verify that every status code returns the correct reason phrase.

Examples:

* OK
* Not Found
* Bad Request

---

## Equality

Verify that status codes can be compared correctly.

---

## Debug Output

Verify that the enum implements useful debugging traits.

---

# Acceptance Criteria

This specification is complete when:

* A `StatusCode` enum exists.
* All required status codes are represented.
* Numeric codes are available.
* Reason phrases are available.
* Unit tests pass.
* No networking or serialization logic exists in this module.

---

# Future Improvements

Future versions may include:

* Full HTTP status code coverage
* Informational (1xx) responses
* Redirection (3xx) responses
* Client error expansion
* Server error expansion
* Custom status codes (if required)

These features are intentionally excluded from Version 1.
