use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    let message = "Hello, server!";
    println!("Sending message to server: {}", message);

    // Send data to the server
    stream.write_all(message.as_bytes()).unwrap();

    // Receive and print the server's response
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("Received response from server: {}", response);
        }
        Err(e) => {
            eprintln!("Error reading from server: {}", e);
        }
    }
}
