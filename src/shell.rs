use std::os::unix::process::CommandExt;
use std::{path::PathBuf};
use std::process::Command;
use crate::repl::REPL;
pub enum CommandError {
    NotFound,
}
pub enum ShellCommand {
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
    pub fn from_str(check: &str) -> Result<Self, CommandError> {
        match check.trim() {
            "exit" => Ok(ShellCommand::Exit),
            "echo" => Ok(ShellCommand::Echo),
            "type" => Ok(ShellCommand::Type),
            "pwd" => Ok(ShellCommand::Pwd),
            "cd" => Ok(ShellCommand::Cd),
            _ => Err(CommandError::NotFound),
        }
    }
    pub fn handle_echo(input: &str) {
        REPL::print_string(input);
        REPL::print_string("\r\n");
    }
    pub fn handle_exit() {
        std::process::exit(0)
    }
    pub fn handle_pwd() {
        let current_directory = std::env::current_dir().unwrap();
        REPL::print_string(current_directory.into_os_string().to_str().unwrap());
        REPL::print_string("\r\n");
    }
    pub fn handle_type(command: &str, paths: &Vec<PathBuf>) {
        let result = match ShellCommand::from_str(command.trim()) {
            Ok(_) => format!("{} is a shell builtin", command),
            Err(_) => {
                let in_path = REPL::check_in_path(&command, paths);
                match in_path {
                    Some(exec_path) => format!("{} is {}", command, exec_path),
                    None => format!("{}: not found", command),
                }
            }
        };
        REPL::print_string(&result);
        REPL::print_string("\r\n");
    }
    pub fn handle_process(command: &str, args: Vec<String>) {
        let arg0 = &args[0];
        Command::new(command)
            .arg0(arg0)
            .args(args[1..].iter().map(|arg| return arg.trim()))
            .status()
            .unwrap();
    }
    pub fn handle_not_found(command: &str) {
        let message = format!("{}: command not found", command.trim());
        REPL::print_string(&message);
        REPL::print_string("\r\n");
    }
    pub fn handle_cd(directory: &str) {
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
            REPL::print_string(&message);
        }
    }
}
