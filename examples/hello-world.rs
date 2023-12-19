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
