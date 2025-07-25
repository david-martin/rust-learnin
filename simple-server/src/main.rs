use std::net::TcpListener;
// Brings the Write trait into scope so we can call write_all() on TcpStream
use std::io::{Read,Write};

macro_rules! log {
    ($words:expr) => {
        println!("LOG: {}", $words);
    };
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // start listening in a loop
    for stream in listener.incoming() {
        log!("incoming");
        let mut unwrapped_stream = stream.unwrap();

        log!("just read the request, even though we ignore it, to avoid connection being closed or other odd behaviour");
        let mut buf = [0; 4096];
        let _ = unwrapped_stream.read(&mut buf); // just consume request

        log!("unwrapped");
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
        log!("response built and about to write");
        unwrapped_stream.write_all(response.as_bytes()).unwrap();
        log!("response written");

    }
}
