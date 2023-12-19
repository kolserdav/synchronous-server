# synchronous-server

![License](https://img.shields.io/github/license/kolserdav/synchronous-server)

This is a Rust crate called synchronous-server, designed to provide reliable and efficient access services for your well-needed use cases. Our crate is built with synchronous communication in mind, ensuring that all requests are handled sequentially and without any asynchronous overhead.

## Features

- Synchronous communication for reliable and predictable behavior
- Customizable request and response handling logic

## Getting Started

To use this crate, you'll need Rust and Cargo installed. Then, follow these steps:

1. Install `synchronous-server`:

```sh
cargo add synchronous-server
```

2. Import the crate in your Rust module:

```rust
use synchronous_server::listen;
```

3. Create a new `Server` instance with the request handler:

```rust
use std::io::Result;
use synchronous_server::{
    http::{
        headers::{Header, Headers},
        status::Status,
}, listen
};

pub fn main() -> Result<()> {
    let res = listen("0.0.0.0:4001", |d| {
        println!("{:?}", d);

        let result = "hello world".to_string();
        let code = 200;
        let headers = Headers::new_request(
            Status::new(code).to_full_string().as_str(),
            vec![
                Header {
                    name: "Content-Type".to_string(),
                    value: "text/plain".to_string(),
                },
                Header {
                    name: "Custom-Header".to_string(),
                    value: "Hello World!".to_string(),
                },
            ],
        );

        Ok((result, code, headers))
    });
    if let Err(err) = res {
        println!("Failed to listen server: {:?}", err);
    }

    Ok(())
}
```

## Versioning

This package follows the Semantic Versioning (SemVer) scheme. See the [CHANGELOG](CHANGELOG.md) for release history.
