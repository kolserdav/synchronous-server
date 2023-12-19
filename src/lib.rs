pub use proxy_server;
pub use proxy_server::http;
use proxy_server::http::{
    headers::{Header, Headers},
    request::{Request, Socket},
    status::Status,
    Http,
};
use std::{
    io::{Result, Write},
    net::TcpListener,
    str,
};

macro_rules! trace {
    ($($args: expr),*) => {
        format!("TRACE: file: {}, line: {}", file!(), line!())
    };
}

pub fn listen<T>(addr: &str, callback: T) -> Result<()>
where
    T: Fn(Request) -> Result<(String, u16, Headers)>,
{
    let listener = TcpListener::bind(addr)?;
    println!("listening on: {}", addr);
    for stream in listener.incoming() {
        let st = stream;
        if let Err(err) = st {
            println!(
                "Failed to create connection {:?}: {}: {:?}",
                err,
                &addr,
                trace!()
            );
            continue;
        }
        let st = st.unwrap();

        let mut client = Http::from(st);

        let h = client.read_headers();
        if let Err(err) = h {
            println!(
                "Failed to read headers {:?}: {:?}: {:?}",
                err,
                &client.socket,
                trace!()
            );
            continue;
        }
        let h = h.unwrap();

        let error = client.socket.take_error().unwrap();
        let error = match error {
            None => "".to_string(),
            Some(val) => val.to_string(),
        };

        let req = Request::new(
            Socket {
                host: client.socket.local_addr().unwrap().to_string(),
                peer_addr: client.socket.peer_addr().unwrap().to_string(),
                ttl: client.socket.ttl().unwrap(),
                error,
            },
            h,
        );
        if let Err(err) = req {
            println!(
                "Failed to create request {:?}: {:?}: {:?}",
                err,
                &client.socket,
                trace!()
            );
            continue;
        }
        let mut req = req.unwrap();

        let body = client.read_body(&req);
        if let Err(err) = body {
            println!("Failed to read body {:?}: {:?}: {:?}", err, &req, trace!());
            continue;
        }
        let body = body.unwrap();

        let body = client.body_to_string(body);
        if let Err(err) = body {
            println!(
                "Failed to stringify body {:?}: {:?}: {:?}",
                err,
                &req,
                trace!()
            );
            continue;
        }
        req.body = body.unwrap();

        let cb_res = callback(req.clone());
        if let Err(err) = cb_res {
            println!(
                "Failed to handle request {:?}: {:?}: {:?}",
                err,
                &req,
                trace!()
            );
            continue;
        }
        let (result, code, headers) = cb_res.unwrap();

        let length = result.len();
        let status = Status::new(code);
        let mut res_heads = Headers::new_response(
            &status,
            vec![
                Header {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                },
                Header {
                    name: "Content-Length".to_string(),
                    value: length.to_string(),
                },
                Header {
                    name: "Server".to_string(),
                    value: "synchronous-server".to_string(),
                },
            ],
        );

        for h in headers.list {
            let added_header = res_heads.set_header(h.name.as_str(), h.value.as_str());
            if let Err(err) = &added_header {
                println!("Failed to add header {:?}: {:?}: {:?}", err, &h, trace!());
            }
            res_heads = added_header.unwrap();
        }

        let res = client.write(res_heads.raw.as_bytes());
        if let Err(err) = res {
            println!(
                "Failed to write headers {:?}: {:?}: {:?}",
                err,
                &req,
                trace!()
            );
            continue;
        }

        let res = client.write(result.as_bytes());
        if let Err(err) = res {
            println!(
                "Failed to write response {:?}: {:?}: {:?}",
                err,
                &req,
                trace!()
            );
            continue;
        }

        let res = client.set_zero_byte();
        if let Err(err) = res {
            println!(
                "Failed to set zero byte {:?}: {:?}: {:?}",
                err,
                &req,
                trace!()
            );
            continue;
        }

        let res = client.flush();
        if let Err(err) = res {
            println!(
                "Failed to flush client {:?}: {:?}: {:?}",
                err,
                &req,
                trace!()
            );
            continue;
        }
    }
    Ok(())
}
