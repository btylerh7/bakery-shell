#[allow(unused_imports)]
use std::io::{self, Write};

enum Command { 
    Exit(String)
}

fn main() {
    loop {
        print_string("$ ");
        let command = read_input();
        let message = match command.trim().to_string().as_str() { 
            "exit" => break,
            _ => format!("{}: command not found", command.trim())
        };
        print_string(&message);
        print_string("\n");

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


