#[allow(unused_imports)]
use std::io::{self, Write};

enum CommandError {
    NotFound
}
enum Command { 
    Exit,
    Echo,
    Type,
    NotFound(String)
}
impl Command {
    fn to_str(&self) -> &str {
        match self {
            Command::Exit => "exit",
            Command::Echo => "echo",
            Command::Type => "type",
            Command::NotFound(string) => string
        }
    }
    fn from_str(check: &str) -> Self {
        match check.trim() {
            "exit" => Command::Exit,
            "echo" => Command::Echo,
            "type" => Command::Type,
            _ => Command::NotFound(check.to_string())
        }
    }
}

fn main() {
    loop {
        print_string("$ ");
        let input = read_input();
        let args: Vec<&str> = input.split(" ").collect();
        let command = Command::from_str(&args[0]);
        let message = match command {
            Command::Exit => return,
            Command::Echo => format!("{}", args[1..].join(" ").trim()),
            Command::Type => format!("{}", get_type(&args[1].trim())),
            _ => format!("{}: command not found", command.to_str().trim())
        };
        print_string(&message);
        print_string("\r\n");

    }
    
}

fn print_string(text: &str) {
    print!("{}", text);
    io::stdout().flush().unwrap();
}
fn read_input() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command
}

fn get_type(command: &str) -> String {
    let result = match command.trim() {
        "exit" => format!("{} is a shell builtin", &command),
        "type" => format!("{} is a shell builtin", &command),
        "echo" => format!("{} is a shell builtin", &command),
        _ => format!("{}: not found", command)
    };
    result
}


