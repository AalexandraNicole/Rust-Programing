use std::io::{Read, Write, self};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8081").expect("Failed to connect to server");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Faileed to read user input");
        //trim whitespace and remove newline
        let command = input.trim();
        if command.is_empty() {
            continue;
        }
        
        send_command(&stream, command.to_string());

        if command.to_lowercase() == "exit" {
            break;
        }
    }
}

fn send_command(mut stream: &TcpStream, command: String) {
    // XOR encryption before sending
    let mut buffer = [0; 4096];

    let key: u8 = 42;
    let mut encrypted_command = command.into_bytes();
    for byte in encrypted_command.iter_mut() {
        *byte ^= key;
    }

    stream.write_all(&encrypted_command).expect("Failed to write to stream");

    let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        // XOR decryption
        let key: u8 = 42;
        for byte in buffer.iter_mut().take(bytes_read) {
            *byte ^= key;
        }

        let from_server = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        let response = from_server.trim();
        println!("De la server : {}", response);

}
