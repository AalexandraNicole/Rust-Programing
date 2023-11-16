use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, argv: &[&str]);
}

trait P1Trait {
    fn new() -> Self;
    fn register(&mut self, command: Box<dyn Command>);
    fn run(&mut self, filename: &str);
}

struct Terminal {
    commands: HashMap<String, Box<dyn Command>>,
}
struct TimesCommand {
    count: u32,
}
struct PingCommand;
struct CountCommand;
struct RunCommand;
struct StopCommand;
struct NameCommand;

impl Command for NameCommand {
    fn exec(&mut self, argv: &[&str]) {
        let mut i = 0;
        while i < argv.len() {
            print!("Hello {}\n", argv[i]);
            i += 1;
        }
    }
    fn get_name(&self) -> &str {
        return "name";
    }
}

impl Command for StopCommand {
    fn exec(&mut self, _argv: &[&str]) {
        println!("Stopping execution");
        std::process::exit(0);
    }

    fn get_name(&self) -> &str {
        let nume = "stop";
        return nume;
    }
}

impl Command for CountCommand {
    fn exec(&mut self, argv: &[&str]) {
        println!("counted {} args", argv.len());
    }

    fn get_name(&self) -> &str {
        let nume = "count";
        return nume;
    }
}

impl Command for TimesCommand {
    fn exec(&mut self, _argv: &[&str]) {
        self.count += 1;
        println!("TimesCommand has been called {} times", self.count);
    }

    fn get_name(&self) -> &str {
        let nume = "times";
        return nume;
    }
}

impl Command for PingCommand {
    fn exec(&mut self, _argv: &[&str]) {
        println!("pong!");
    }

    fn get_name(&self) -> &str {
        let nume = "ping";
        return nume;
    }
}

impl Command for RunCommand {
    fn exec(&mut self, argv: &[&str]) {
        println!("RunCommand executed with args: {:?}", argv);
    }

    fn get_name(&self) -> &str {
        let nume = "run";
        return nume;
    }
}

impl P1Trait for Terminal {
    fn new() -> Self {
        Terminal {
            commands: HashMap::new(),
        }
    }

    fn register(&mut self, command: Box<dyn Command>) {
        let command_name = command.get_name().to_string();
        self.commands.insert(command_name, command);
    }

    fn run(&mut self, filename: &str) {
        if let Ok(file) = File::open(filename) {
            for line in io::BufReader::new(file).lines() {
                if let Ok(line) = line {
                    let mut parts = line.split_whitespace();
                    if let Some(command_name) = parts.next() {
                        if command_name == "STOP"
                            || command_name == "stp"
                            || command_name == "sop"
                            || command_name == "Stop"
                            || command_name == "stopp"
                        {
                            println!("Maybe you want to say \'stop\' ?");
                            continue;
                        }
                        let args: Vec<&str> = parts.collect();
                        match self.commands.get_mut(command_name) {
                            Some(command) => {
                                command.exec(&args);
                            }
                            None => eprintln!("Unknown command: \'{}\'", command_name),
                        }
                    }
                }
            }
        } else {
            eprintln!("Error opening commands file");
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(StopCommand {}));
    terminal.register(Box::new(NameCommand {}));

    terminal.run("file");
}
