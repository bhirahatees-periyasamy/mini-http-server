# mini-http

An HTTP/1.1 server built from scratch in Rust using only the standard library.

This is a learning project. The goal is to understand how web servers work internally — TCP connections, HTTP parsing, routing, and response generation — without relying on external networking crates like Tokio or Hyper.

## Goals

- Learn Rust through systems programming
- Understand TCP networking at the socket level
- Implement the HTTP/1.1 protocol by hand
- Keep the architecture modular and easy to follow

## Non-Goals

This is not a production server. The following are intentionally out of scope for v1:

- HTTPS / TLS
- HTTP/2 or HTTP/3
- Async runtime (Tokio)
- WebSockets
- Middleware framework
- Reverse proxy

## Architecture

Requests flow through a strict pipeline where each layer has a single responsibility:

```
Client (Browser / curl)
        │
        ▼
TCP Listener
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

Dependencies always flow downward:

```
main → server → connection → http → router → handler → filesystem
```

Lower-level modules never import from higher-level ones. Circular dependencies are not allowed.

## Planned Modules

| Module       | Responsibility                                          |
|--------------|---------------------------------------------------------|
| `server`     | TCP listener, accepting connections, lifecycle          |
| `connection` | Reading and writing bytes on the socket                 |
| `http`       | Request model, response model, parsing, serialization  |
| `router`     | Route matching, selecting handlers                      |
| `handler`    | Application logic, producing responses                  |
| `filesystem` | Loading static files, MIME types                        |
| `error`      | Shared error types and conversions                      |
| `util`       | Miscellaneous helpers                                   |

## Development Approach

This project follows **Specification-Driven Development (SDD)**. Every feature starts with a written spec before any code is written.

Order for each feature:

1. Write the specification (`docs/specs/`)
2. Write tests
3. Implement
4. Refactor if needed
5. Update documentation

## Milestones

| Milestone | Scope                                               | Status      |
|-----------|-----------------------------------------------------|-------------|
| 1         | TCP listener, accept connections, read/write stream | Not started |
| 2         | HTTP request parser, response builder, headers      | Not started |
| 3         | Router, route handlers, 404 handling                | Not started |
| 4         | Static file serving, MIME types                     | Not started |
| 5         | Multi-threaded connection handling                  | Not started |
| 6         | Logging, configuration, error handling              | Not started |

## Getting Started

```bash
git clone <repo>
cd mini-http
cargo build
cargo run
```

Requires Rust edition 2024. No external dependencies.

## Documentation

- [`docs/architecture.md`](docs/architecture.md) — Design principles, module boundaries, dependency graph, coding standards
- [`docs/module-design.md`](docs/module-design.md) — Module-level design detail
- [`docs/specs/`](docs/specs/) — Feature specifications

## Coding Standards

- One responsibility per module
- Return `Result<T, E>` instead of panicking
- Keep functions small and focused
- Document all public APIs
- Write tests for all public functionality
- Avoid unnecessary cloning
- No global mutable state
