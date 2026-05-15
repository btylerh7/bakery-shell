use std::{env, fs::DirEntry, io::Error, path::PathBuf};
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
    fn handle_echo(input: &str) {
        print_string(input);
        print_string("\r\n");
    }
    fn handle_exit() {
        std::process::exit(0)
    }
    fn handle_type(command: &str) {
        let result = match command.trim() {
            "exit" => format!("{} is a shell builtin", &command),
            "type" => format!("{} is a shell builtin", &command),
            "echo" => format!("{} is a shell builtin", &command),
            _ => {
                let in_path = check_in_path(&command);
                match in_path {
                    Some(exec_path) => format!("{} is {}", command, exec_path),
                    None => format!("{}: not found", command)
                }
            }
        };
        print_string(&result);
        print_string("\r\n");
    }
    fn handle_not_found(command: &str) {
        let message = format!("{}: command not found", command.trim());
        print_string(&message);
        print_string("\r\n");
    }

}

fn main() {
    loop {
        print_string("$ ");
        let input = read_input();
        let args: Vec<&str> = input.split(" ").collect();
        let command = Command::from_str(&args[0]);
        match command {
            Command::Exit => Command::handle_exit(),
            Command::Echo => Command::handle_echo(args[1..].join(" ").trim()),
            Command::Type => Command::handle_type(&args[1].trim()),
            Command::NotFound(cmd) => Command::handle_not_found(&cmd)
        };

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

fn check_in_path(command: &str) -> Option<String> {
    if let Some(path_list) = std::env::var_os("PATH") {
        let paths: Vec<PathBuf> = env::split_paths(&path_list).collect();
        for p in paths {
            let command_check = p.join(command);
            if command_check.exists() && is_executable(&command_check) {
                return Some(command_check.into_os_string().into_string().unwrap());
            }
            continue
        }
    }
    None
}

#[cfg(unix)]
fn is_executable(file: &PathBuf) -> bool {
    if let Ok(metadata) = file.as_path().metadata() {
        use std::os::unix::fs::PermissionsExt;

        let permissions = metadata.permissions();
        return permissions.mode() & 0o111 != 0
    }
    false
}

#[cfg(windows)]
fn is_executable(file: &PathBuf) -> bool {
    if let Ok(metadata) = file.as_path().metadata() {
        use std::os::windows::fs::PermissionsExt;

        let permissions = metadata.permissions();
        return permissions.mode() & 0x21 != 0
    }
    false
}



