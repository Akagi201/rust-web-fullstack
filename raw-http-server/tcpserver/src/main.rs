use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3001").unwrap();
    println!("Listening on http://0.0.0.0:3001");

    #[allow(clippy::unused_io_amount)]
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write_all(&buffer).unwrap();
    }
}