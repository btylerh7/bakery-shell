#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{env, path::PathBuf};

enum CommandError {
    NotFound,
}
enum ShellCommand {
    Exit,
    Echo,
    Type,
}
impl ShellCommand {
    fn to_str(&self) -> &str {
        match self {
            ShellCommand::Exit => "exit",
            ShellCommand::Echo => "echo",
            ShellCommand::Type => "type",
        }
    }
    fn from_str(check: &str) -> Result<Self, CommandError> {
        match check.trim() {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo),
            "type" => Ok(ShellCommand::Type),
            _ => Err(CommandError::NotFound),
        }
    }
    fn handle_echo(input: &str) {
        print_string(input);
        print_string("\r\n");
    }
    fn handle_exit() {
        std::process::exit(0)
    }
    fn handle_type(command: &str, paths: &Vec<PathBuf>) {
        let result = match ShellCommand::from_str(command.trim()) {
            Ok(_) => format!("{} is a shell builtin", command),
            Err(_) => {
                let in_path = check_in_path(&command, paths);
                match in_path {
                    Some(exec_path) => format!("{} is {}", command, exec_path),
                    None => format!("{}: not found", command),
                }
            }
        };
        print_string(&result);
        print_string("\r\n");
    }
    fn handle_process(command: &str, args: Vec<&str>) {
        Command::new(command)
            .args(args.iter().map(|arg| return arg.trim()))
            .spawn()
            .unwrap();
        print_string("\r\n");
    }
    fn handle_not_found(command: &str) {
        let message = format!("{}: command not found", command.trim());
        print_string(&message);
        print_string("\r\n");
    }
}

fn main() {
    // Load path environment variable
    let mut paths: Vec<PathBuf> = vec![];
    if let Some(path_list) = std::env::var_os("PATH") {
        paths = env::split_paths(&path_list).collect();
    }

    // Eval loop
    loop {
        print_string("$ ");
        let input = read_input();
        let args: Vec<&str> = input.split(" ").collect();
        let command = ShellCommand::from_str(&args[0]);
        match command {
            Ok(ShellCommand::Exit) => ShellCommand::handle_exit(),
            Ok(ShellCommand::Echo) => ShellCommand::handle_echo(args[1..].join(" ").trim()),
            Ok(ShellCommand::Type) => ShellCommand::handle_type(&args[1].trim(), &paths),
            _ => {
                if let Some(execute_path) = check_in_path(&args[0].trim(), &paths) {
                    ShellCommand::handle_process(&execute_path, args[1..].to_vec())
                } else {
                    ShellCommand::handle_not_found(&args[0].trim())
                }
            }
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

fn check_in_path(command: &str, paths: &Vec<PathBuf>) -> Option<String> {
    let paths_cloned = paths.clone();
    for p in paths_cloned {
        let command_check = p.join(command);
        if command_check.exists() && is_executable(&command_check) {
            return Some(command_check.into_os_string().into_string().unwrap());
        }
        continue;
    }
    None
}

#[cfg(unix)]
fn is_executable(file: &PathBuf) -> bool {
    if let Ok(metadata) = file.as_path().metadata() {
        use std::os::unix::fs::PermissionsExt;

        let permissions = metadata.permissions();
        return permissions.mode() & 0o111 != 0;
    }
    false
}

#[cfg(windows)]
fn is_executable(file: &PathBuf) -> bool {
    if let Ok(metadata) = file.as_path().metadata() {
        use std::os::windows::fs::PermissionsExt;

        let permissions = metadata.permissions();
        return permissions.mode() & 0x21 != 0;
    }
    false
}
