# synchronous-server

![License](https://img.shields.io/github/license/kolserdav/synchronous-server)

This is a Rust crate called synchronous-server, designed to provide reliable and efficient access services for your well-needed use cases. Our crate is built with synchronous communication in mind, ensuring that all requests are handled sequentially and without any asynchronous overhead.

## Features

- Synchronous communication for reliable and predictable behavior
- Built-in support for common HTTP methods (GET, POST, PUT, DELETE)
- Customizable request and response handling logic
- Middleware support for adding functionality to requests and responses
- Integration with popular web frameworks like Rocket and Actix-Web

## Getting Started

To use this crate, you'll need Rust and Cargo installed. Then, follow these steps:

1. Add the `synchronous-server` dependency to your `Cargo.toml` file:

```toml
[dependencies]
synchronous-server = "0.1"
```

2. Import the crate in your Rust module:

```rust
extern crate synchronous_server;
```

3. Create a new `Server` instance with the desired middleware and request/response handlers:
