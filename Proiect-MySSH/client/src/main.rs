use std::io::{Read, Write, self};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8081").expect("Failed to connect to server");

    loop {
        println!("Enter comand: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Faileed to read user input");

        //trim whitespace and remove newline
        let command = input.trim();

        if command.to_lowercase() == "exit" {
            break;
        }
        
        send_command(&stream, command.to_string());
    }
}

fn send_command(mut stream: &TcpStream, command: String) {
    // XOR encryption before sending
    let key: u8 = 42;
    let mut encrypted_command = command.into_bytes();
    for byte in encrypted_command.iter_mut() {
        *byte ^= key;
    }

    stream.write_all(&encrypted_command).expect("Failed to write to stream");

    let mut response = Vec::new();
    stream.read_to_end(&mut response).expect("Failed to read from stream");

    // XOR decryption after receiving
    for byte in response.iter_mut() {
        *byte ^= key;
    }

    println!("Server Response: {:?}", String::from_utf8_lossy(&response));
}
