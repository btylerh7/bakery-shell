#[allow(unused_imports)]
mod parser;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, path::PathBuf};

enum CommandError {
    NotFound,
}
enum ShellCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}
impl ShellCommand {
    // fn to_str(&self) -> &str {
    //     match self {
    //         ShellCommand::Exit => "exit",
    //         ShellCommand::Echo => "echo",
    //         ShellCommand::Type => "type",
    //     }
    // }
    fn from_str(check: &str) -> Result<Self, CommandError> {
        match check.trim() {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo),
            "type" => Ok(ShellCommand::Type),
            "pwd" => Ok(ShellCommand::Pwd),
            "cd" => Ok(ShellCommand::Cd),
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
    fn handle_pwd() {
        let current_directory = std::env::current_dir().unwrap();
        print_string(current_directory.into_os_string().to_str().unwrap());
        print_string("\r\n");
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
    fn handle_process(command: &str, args: Vec<String>) {
        let arg0 = &args[0];
        Command::new(command)
            .arg0(arg0)
            .args(args[1..].iter().map(|arg| return arg.trim()))
            .status()
            .unwrap();
    }
    fn handle_not_found(command: &str) {
        let message = format!("{}: command not found", command.trim());
        print_string(&message);
        print_string("\r\n");
    }
    fn handle_cd(directory: &str) {
        let mut new_dir = directory.to_owned();
        if new_dir.starts_with("~") {
            if let Some(home_dir) = std::env::var_os("HOME")
                && let Ok(home_dir_string) = home_dir.to_os_string().into_string()
            {
                new_dir = new_dir.replace("~", &home_dir_string);
            }
        }
        let check_path = std::path::Path::new(&new_dir);
        if check_path.exists() {
            if let Err(error) = std::env::set_current_dir(check_path) {
                println!("Error changing directory to {:?}, {:?}", check_path, error)
            }
        } else {
            let message = format!("cd: {}: No such file or directory \r\n", new_dir);
            print_string(&message);
        }
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
        let mut arg_parser = parser::Parser::new();
        let args = arg_parser.parse_arg_string(&input);
        let command = ShellCommand::from_str(&args[0]);
        match command {
            Ok(ShellCommand::Exit) => ShellCommand::handle_exit(),
            Ok(ShellCommand::Echo) => ShellCommand::handle_echo(args[1..].join(" ").trim()),
            Ok(ShellCommand::Type) => ShellCommand::handle_type(&args[1].trim(), &paths),
            Ok(ShellCommand::Pwd) => ShellCommand::handle_pwd(),
            Ok(ShellCommand::Cd) => ShellCommand::handle_cd(&args[1].trim()),
            _ => {
                if let Some(execute_path) = check_in_path(&args[0].trim(), &paths) {
                    ShellCommand::handle_process(&execute_path, args)
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

// #[cfg(windows)]
// fn is_executable(file: &PathBuf) -> bool {
//     if let Ok(metadata) = file.as_path().metadata() {
//         use std::os::windows::fs::PermissionsExt;
//
//         let permissions = metadata.permissions();
//         return permissions.mode() & 0x21 != 0;
//     }
//     false
// }
