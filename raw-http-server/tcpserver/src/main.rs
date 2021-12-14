use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3001").unwrap();
    println!("Listening on http://0.0.0.0:3001");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
        // handle_connection(stream);
    }
}