use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client conected!");
                std::thread::spawn(move || {
                    handle_client(&mut stream);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 4096];

    loop{
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        // XOR encryption and decryption
        let key: u8 = 42;
        for byte in buffer.iter_mut().take(bytes_read) {
            *byte ^= key;
        }

        let from_client = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        let command = from_client.trim();
        println!("Comanda de la client: {}", command);

        // Exit if the client sends "exit" command
        if command.trim().to_lowercase() == "exit" {
            println!("Client requested exit. Closing connection.");
            return;
        }

        // Execute the command using std::process::Command
        let output = execute_command(command);

        // Encrypt the output before sending it back
        let mut encrypted_output = output.into_bytes();

        for byte in encrypted_output.iter_mut() {
            *byte ^= key;
        }
        stream.write_all(&encrypted_output).expect("Failed to write to stream");
    }
}

fn execute_command(command: &str) -> String {
    let command_list: Vec<&str> = command.trim().split("&&").collect();

    let mut result = String::new();
    for cmd in command_list {
        let or_commands: Vec<&str> = cmd.trim().split("||").collect();

        if let Some(or_cmd) = or_commands.first() {
            let mut parts = or_cmd.split_whitespace();
            let program = parts.next().unwrap_or("FAILD");
            let args: Vec<&str> = parts.collect();

            let output = Command::new(program)
                .args(&args)
                .env("PATH", "/bin:/usr/bin")
                .output();

            match output {
                Ok(output) => {
                    let result_str = String::from_utf8_lossy(&output.stdout).to_string();
                    let status = output.status;
                    let error_output = String::from_utf8_lossy(&output.stderr).to_string();

                    if !status.success() {
                        result.push_str(&result_str);
                    }else if status.success() && cmd.contains("||") {
                        println!("Command failed1: {}\n Error output {}", or_cmd,error_output);
                        break;
                    } else if status.success() {
                        println!("Command failed2: {}\nError output {}",or_cmd,error_output);
                        break;
                    }
                    return result_str;
                }
                Err(e) => {
                    eprintln!("Command not found: {}", e);
                    return String::from("Command not found");
                }
            }
        }

        if cmd.contains("&&") {
            // All commands with "&&" succeeded
            println!("All commands succeeded: {}", cmd);
        }else if cmd.contains("&&") && !result.is_empty() {
            println!("Error executing && comants");
            return String::from("Error executing && commands;");
        }
    }

    String::new()
}
