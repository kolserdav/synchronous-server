use proxy_server::http::headers::{Header, Headers};
use serde_json;
use std::io::Result;
use synchronous_server::listen;

pub fn main() -> Result<()> {
    let res = listen("127.0.0.1:4001", |d| {
        println!("request {:?}", d);
        let result = "hello world".to_string();
        let code = 200;
        let headers = Headers::new(
            "HTTP/1.1 OK 200",
            vec![
                Header {
                    name: "Content-Type".to_string(),
                    value: "text/plain".to_string(),
                },
                Header {
                    name: "Custom-Header".to_string(),
                    value: "value".to_string(),
                },
            ],
        );
        Ok((result, code, serde_json::to_string(&headers).unwrap()))
    });
    Ok(())
}
