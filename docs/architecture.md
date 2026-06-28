# Architecture

## Overview

This project aims to build an **HTTP/1.1 server from scratch** using only the Rust standard library. The primary goal is to understand how web servers work internally rather than to create a production-ready framework.

The project follows a **Specification-Driven Development (SDD)** approach. Every feature begins with a specification, followed by tests, and finally an implementation.

---

# Project Goals

## Goals

* Learn Rust through systems programming.
* Understand TCP networking.
* Understand the HTTP/1.1 protocol.
* Implement an HTTP server without external networking libraries.
* Design a modular and maintainable codebase.
* Follow clean architecture and single responsibility principles.
* Build each feature from its specification.

## Non-Goals

The following features are intentionally excluded from the initial implementation.

* HTTPS
* HTTP/2
* HTTP/3
* Async runtime (Tokio)
* WebSockets
* Reverse Proxy
* Middleware framework
* Production performance optimizations

These may be explored in future versions after the synchronous server is complete.

---

# High-Level Architecture

```
                 Client (Browser / curl)

                         │
                         ▼

                 TCP Listener (Server)

                         │
                         ▼

              Connection Handler

                         │
                         ▼

                 HTTP Request Parser

                         │
                         ▼

                     Router

                         │
                         ▼

                  Route Handler

                         │
                         ▼

               HTTP Response Builder

                         │
                         ▼

                  TCP Stream Writer
```

---

# Design Principles

## Single Responsibility

Each module should have one clearly defined responsibility.

Examples:

* The parser only parses requests.
* The router only determines which handler should execute.
* The response module only constructs HTTP responses.

Modules should not perform unrelated tasks.

---

## Separation of Concerns

Networking, parsing, routing, and response generation are independent components.

```
Network

↓

Parser

↓

Router

↓

Handler

↓

Response
```

Each layer communicates only through well-defined data structures.

---

## Specification First

Every feature begins with a specification located under the `specs/` directory.

Implementation order:

1. Write the specification.
2. Write unit/integration tests.
3. Implement the feature.
4. Refactor if necessary.
5. Update documentation.

---

## Small Public API

Only expose functionality that is required by other modules.

Internal implementation details should remain private.

---

## Error Propagation

Modules should return `Result<T, E>` instead of panicking.

Errors should be propagated to higher layers where they can be converted into appropriate HTTP responses.

---

# Project Structure

```
mini-http/

├── Cargo.toml
├── README.md
├── LICENSE
│
├── docs/
│   ├── architecture.md
│   ├── networking.md
│   ├── roadmap.md
│   ├── coding-guidelines.md
│   └── decisions/
│
├── specs/
│   ├── 001-tcp-listener.md
│   ├── 002-http-request.md
│   ├── 003-http-response.md
│   ├── 004-router.md
│   ├── 005-static-files.md
│   └── ...
│
├── tests/
│
├── examples/
│
├── assets/
│
└── src/
```

---

# Source Layout

```
src/

main.rs
lib.rs

server/
http/
router/
handler/
filesystem/
error/
util/
```

Each directory represents a subsystem.

---

# Planned Modules

## server

Responsible for:

* Creating the TCP listener
* Accepting client connections
* Managing server lifecycle

Does not:

* Parse HTTP
* Route requests

---

## connection

Responsible for:

* Reading bytes from the socket
* Writing bytes to the socket
* Managing connection state

Does not:

* Parse HTTP
* Route requests

---

## http

Responsible for:

* HTTP request model
* HTTP response model
* Request parsing
* Response serialization
* Header parsing
* Status codes

Does not:

* Read sockets
* Serve files

---

## router

Responsible for:

* Route matching
* Selecting request handlers

Does not:

* Parse requests
* Build responses

---

## handler

Responsible for:

* Executing application logic
* Producing responses

---

## filesystem

Responsible for:

* Loading static files
* Determining file metadata

---

## error

Responsible for:

* Common error definitions
* Error conversion

---

## util

Contains reusable helper functions that do not belong to another module.

---

# Dependency Graph

```
main

│

▼

server

│

▼

connection

│

▼

http

│

▼

router

│

▼

handler

│

▼

filesystem
```

Dependencies should always flow downward.

Lower-level modules should never depend on higher-level modules.

Circular dependencies are not allowed.

---

# Development Workflow

Every feature follows the same process.

```
Specification

↓

Design

↓

Tests

↓

Implementation

↓

Refactoring

↓

Documentation
```

No feature should be implemented without a specification.

---

# Milestones

## Milestone 1

* TCP Listener
* Accept Connections
* Read TCP Stream
* Write Basic Response

---

## Milestone 2

* HTTP Request Parser
* HTTP Response Builder
* Header Parsing

---

## Milestone 3

* Router
* Route Handlers
* 404 Handling

---

## Milestone 4

* Static File Serving
* MIME Types

---

## Milestone 5

* Multi-threaded Server

---

## Milestone 6

* Logging
* Configuration
* Error Handling Improvements

---

# Coding Standards

* One responsibility per module.
* Avoid global mutable state.
* Prefer composition over inheritance.
* Return `Result` instead of panicking.
* Keep functions small and focused.
* Document all public APIs.
* Write tests for all public functionality.
* Avoid unnecessary cloning.
* Follow idiomatic Rust ownership patterns.

---

# Future Improvements

After completing the synchronous server, future versions may include:

* Thread Pool
* Async Runtime (Tokio)
* HTTP Keep-Alive
* HTTP/2
* Middleware
* Reverse Proxy
* TLS (HTTPS)
* Request Logging
* Metrics
* Configuration File Support
* Benchmarking

These are intentionally deferred to keep the first implementation focused on learning Rust and networking fundamentals.
