use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};

macro_rules! log {
    ($words:expr) => {
        println!("LOG: {}", $words);
    };
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    log!("listening on http://127.0.0.1:7879");

    // start listening in a loop
    for stream in listener.incoming() {
        log!("incoming");
        let mut client_stream = stream.unwrap();
        let mut buffer = [0; 4096];
		log!("read bytes");
        let bytes_read = client_stream.read(&mut buffer).unwrap();

		log!("connect to upstream");
        let mut server_stream = TcpStream::connect("127.0.0.1:7878").unwrap();
		log!("write to upstream");
        server_stream.write_all(&buffer[..bytes_read]).unwrap();

        // Read the response from the upstream server
        let mut response = Vec::new();
		log!("read response from upstream");
        server_stream.read_to_end(&mut response).unwrap();

        // Write the response back to the original client
		log!("write response from upstream to downstream");
        client_stream.write_all(&response).unwrap();
		log!("done");
    }
}
