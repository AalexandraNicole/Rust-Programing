use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::fs::File;
use regex::Regex;

fn is_special_char(s: &str) -> bool {
    s == "&&" || s == "||" || s=="<" || s==">" || s=="|"
}

// Execute a sequence of commands based on special characters
fn execute_command(command_str: &str, current_directory: &std::path::PathBuf) -> String {
    let mut output = String::new();

    // Split the command string based on special characters: |, ;, <, >, &
    //let commands: Vec<&str> = command_str.split(|c| c == '|' || c == ';' || c == '<' || c == '>' || c == '&').collect();
    // Use a regular expression to capture separators and split the command string
    let re = Regex::new(r"(&&|\|\||[;<>])").unwrap();
    let mut commands: Vec<&str> = vec![];

    // Push the first part before any separator
    if let Some(first_part) = re.split(command_str).next() {
        commands.push(first_part);
    }

    // Iterate over captures and parts alternately
    let iter = re.captures_iter(command_str).peekable();
    for cap in iter{
            // Push the part after the separator
            if let Some(next_part) = cap.get(1) {
                commands.push(next_part.as_str());
            }

        // Push the last part (if any)
        if let Some(last_part) = re.split(command_str).last() {
            commands.push(last_part);
        }
    }

    let separator_indices: Vec<usize> = commands.iter().enumerate()
        .filter(|&(_, &x)| is_special_char(x))
        .map(|(i, _)| i)
        .collect();

    // Move each separator one position to the left
    for &separator_index in &separator_indices {
        if separator_index > 0 {
            commands.swap(separator_index, separator_index - 1);
        }
    }

    println!("COMMANDS: {:?}",commands);

    let mut iter = commands.into_iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            "&&" => {
                // Handle logical AND (&&)
                if let Some(next_command) = iter.next() {
                    let next_output = execute_command(next_command, current_directory);
                    let failed = &next_output[..14];
                    if failed != "Command failed" {
                        if let Some(next_next_command) = iter.next() {
                            let next_next_output = execute_command(next_next_command, current_directory);
                            let failed = &next_next_output[..14];
                            if failed != "Command failed" {
                                output = next_output + &next_next_output;
                            }
                        }
                    }else{
                        output = String::from("First command failed");
                    }
                }
                continue;
            }
            "||" => {
                // Handle logical OR (||)
                if let Some(next_command) = iter.next() {
                    let next_output = execute_command(next_command, current_directory);
                    let failed = &next_output[..14];
                    if failed == "Command failed" {
                        if let Some(next_command) = iter.next() {
                            let next_next_output = execute_command(next_command, current_directory);
                            let failed = &next_next_output[..14];
                            if failed == "Command failed" {
                                output = next_next_output;
                            }
                        }
                    }else{
                        output = String::from("First command succed");
                    }
                }
                continue;
            }
            ">" => {
                // Handle output redirection (>)
                if let Some(command) = iter.next() {
                    // Redirect output to a file
                    if let Some(output_file) = iter.next(){
                            let output_result = Command::new("sh")
                                .current_dir(current_directory)
                                .arg("-c")
                                .arg(command)
                                .stdout(Stdio::piped())
                                .output();
            
                            match output_result {
                                Ok(output) => {
                                    if let Ok(mut file) = File::create(output_file.trim()) {
                                        file.write_all(&output.stdout).expect("Failed to write to file");
                                        println!("Output redirected to file: {}", output_file);
                                    } else {
                                        eprintln!("Failed to create or write to file: {}", output_file);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to execute command: {}", e);
                                }
                            }
                    } else {
                        eprintln!("Missing output file after `>`");
                    }
                }
                output = String::from("DONE");
            }
            "<" => {
                // Handle input redirection (<)
                if let Some(command) = iter.next() {
                    if let Some(input_file) = iter.next() {
                        // Input redirection
                        let mut input = String::new();
                        if let Ok(mut file) = File::open(input_file.trim()) {
                            file.read_to_string(&mut input).expect("Failed to read from file");
                            println!("Input redirected from file: {}", input_file);
                        } else {
                            eprintln!("Failed to open or read from file: {}", input_file);
                        }
                        let com = command.to_string() + " " + &input.to_string();
                        output = execute_single_command(&com, current_directory);
                        
                    }
                }
            }
            "|" => {
                    // Handle pipe (|)
                    let mut pipe_child = Command::new("sh")
                    .arg("-c")
                    .arg(token.trim())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to execute command");

                let mut child_stdout = pipe_child.stdout.take().unwrap();
                let mut child_output = String::new();
                child_stdout.read_to_string(&mut child_output).unwrap();
                output = child_output;
                continue;
            }
            ";" => {
                // Handle command sequencing (;)
                let _ = execute_single_command(token, current_directory);
            }
            _ => {
                // Handle a single command
                println!("Execute: {}", token);
                let single_command_output = execute_single_command(token, current_directory);
                output.push_str(&single_command_output);
            }
        }
    }

    output
}

// Execute a single command
fn execute_single_command(command: &str, current_directory: &std::path::PathBuf) -> String {
    let mut outputt = String::new();

    // Execute the command using sh -c
    let output_result = Command::new("sh")
        .current_dir(current_directory)
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output();

    match output_result {
        Ok(output) => {
            if !output.status.success() {
                let error_output = String::from_utf8_lossy(&output.stderr);
                outputt.push_str(&format!("Command failed: {}\nError output: {}", command, error_output));
            } else {
                let mut result_str = String::from_utf8_lossy(&output.stdout);
                result_str = (String::from(" Done!\n") + &result_str).into();
                outputt.push_str(&result_str);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            outputt.push_str("Command not found");
        }
    }

    outputt
}


// Handle communication with the client
fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 4096];
    let mut current_directory = std::env::current_dir().unwrap(); // Initial working directory

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
            let a = String::from("BYE! ");
            let output = a;
            // Encrypt the output before sending it back
            let mut encrypted_output = output.into_bytes();

            for byte in encrypted_output.iter_mut() {
                *byte ^= key;
            }
            stream.write_all(&encrypted_output).expect("Failed to write to stream");
            return;
        }
        if command.trim().starts_with("cd ") {
            // Handle 'cd' command to change working directory
            let new_directory = &command[3..].trim();
            if !std::path::Path::new(new_directory).exists(){
                eprintln!("Directory does not exist: {}", new_directory);
                let output = "Directory does not exist ".to_owned() + new_directory;
                let response = output.to_string();

                // Encrypt the output before sending it back
                let mut encrypted_output = response.into_bytes();

                for byte in encrypted_output.iter_mut() {
                    *byte ^= key;
                }
                stream.write_all(&encrypted_output).expect("Failed to write to stream");

                continue;
            }
            if let Ok(path) = std::fs::canonicalize(new_directory) {
                current_directory = path;
                println!("Client changed directory to: {:?}", current_directory);
                let output = "Directory ".to_owned() + new_directory;
                let response = output.to_string();

                // Encrypt the output before sending it back
                let mut encrypted_output = response.into_bytes();

                for byte in encrypted_output.iter_mut() {
                    *byte ^= key;
                }
                stream.write_all(&encrypted_output).expect("Failed to write to stream");

                continue;
            } else {
                eprintln!("Failed to change directory to: {}", new_directory);
            }
        }

        // Execute the command using std::process::Command
        let output = execute_command(command, &current_directory);

        // Send working directory information along with the command output
        let response = output.to_string();

        // Encrypt the output before sending it back
        let mut encrypted_output = response.into_bytes();

        for byte in encrypted_output.iter_mut() {
            *byte ^= key;
        }
        stream.write_all(&encrypted_output).expect("Failed to write to stream");

    }
}

fn main() {
    // Start the TCP listener
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind to address");
    println!("Server running!");
    // Accept incoming connections and spawn threads for handling each client
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
