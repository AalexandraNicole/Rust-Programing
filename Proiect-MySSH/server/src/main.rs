use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Read data from the client
    match stream.read(&mut buffer) {
        Ok(size) => {
            // Print received message
            let received_message = String::from_utf8_lossy(&buffer[..size]);
            println!("Received message from client: {}", received_message);

            // Respond to the client
            let response = "Hello, client!";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");

    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
