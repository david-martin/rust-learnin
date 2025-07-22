use std::net::TcpListener;
// Brings the Write trait into scope so we can call write_all() on TcpStream
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // start listening in a loop
    for stream in listener.incoming() {
        let mut unwrapped_stream = stream.unwrap();
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
        unwrapped_stream.write_all(response.as_bytes()).unwrap();
    }
}
