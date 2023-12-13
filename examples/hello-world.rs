use proxy_server::http::headers::{Header, Headers};
use std::io::Result;
use synchronous_server::listen;

pub fn main() -> Result<()> {
    let res = listen("0.0.0.0:4001", |d| {
        println!("{:?}", d);

        let result = "hello world".to_string();
        let code = 200;
        let headers = Headers::new(
            "HTTP/1.1 200 OK",
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

        Ok((result, code, headers))
    });
    if let Err(err) = res {
        println!("Failed to listen server: {:?}", err);
    }

    Ok(())
}
